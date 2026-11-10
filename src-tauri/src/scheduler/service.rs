use super::{engine::Engine, model::*};
use crate::{
    clock::{Clock, SystemClock},
    storage::Storage,
};
use serde::Serialize;
use std::{
    sync::mpsc::{self, Receiver, RecvTimeoutError, Sender},
    thread,
};
use tauri::{AppHandle, Emitter};

pub const STORAGE_FAILURE: &str = "Scheduler stopped because local data could not be saved. Existing history has not been erased. Close Restnaa and check local storage before restarting.";

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum StateEvent {
    State { state: DashboardState },
    Error { message: String },
}

/// Transaction boundary shared by production and file-backed recovery tests.
pub struct Session<C: Clock> {
    pub engine: Engine,
    storage: Storage,
    pub clock: C,
    failed: bool,
}

impl<C: Clock> Session<C> {
    pub fn open(mut storage: Storage, clock: C) -> Result<(Self, Effects), String> {
        let data = storage
            .load_scheduler()
            .map_err(|_| STORAGE_FAILURE.to_string())?;
        let (engine, effects) = Engine::restore(data, &clock)?;
        storage
            .save_scheduler(&engine.data, &effects.recorded, clock.sample().utc)
            .map_err(|_| STORAGE_FAILURE.to_string())?;
        Ok((
            Self {
                engine,
                storage,
                clock,
                failed: false,
            },
            effects,
        ))
    }

    pub fn apply(&mut self, action: Action) -> Result<Effects, String> {
        if self.failed {
            return Err(STORAGE_FAILURE.into());
        }
        let mut candidate = self.engine.clone();
        let effects = candidate.step(action, &self.clock)?;
        if candidate.data != self.engine.data
            && self
                .storage
                .save_scheduler(&candidate.data, &effects.recorded, self.clock.sample().utc)
                .is_err()
        {
            self.failed = true;
            return Err(STORAGE_FAILURE.into());
        }
        self.engine = candidate;
        Ok(effects)
    }

    pub fn snapshot(&self) -> DashboardState {
        self.engine.snapshot(&self.clock)
    }

    pub fn history(
        &self,
        range: crate::history::HistoryRange,
    ) -> Result<crate::history::History, String> {
        if self.failed {
            return Err(STORAGE_FAILURE.into());
        }
        self.storage.history(range, &self.clock).map_err(|_| {
            "Local history could not be read. Existing data has not been erased.".into()
        })
    }

    pub fn clear_history(&mut self, confirmed: bool) -> Result<(), String> {
        if !confirmed {
            return Err("Confirm history deletion before continuing.".into());
        }
        if self.failed {
            return Err(STORAGE_FAILURE.into());
        }
        self.storage.clear_history(true).map_err(|_| "History could not be cleared. Please try again; settings and reminders were not changed.".into())
    }
}

enum Request {
    Scheduler {
        action: Action,
        response: Sender<Result<DashboardState, String>>,
    },
    History {
        range: crate::history::HistoryRange,
        response: Sender<Result<crate::history::History, String>>,
    },
    ClearHistory {
        confirmed: bool,
        response: Sender<Result<(), String>>,
    },
}

#[derive(Clone)]
pub struct SchedulerHandle {
    sender: Sender<Request>,
}

impl SchedulerHandle {
    pub fn request(&self, action: Action) -> Result<DashboardState, String> {
        let (response, receiver) = mpsc::channel();
        self.sender
            .send(Request::Scheduler { action, response })
            .map_err(|_| "Scheduler is unavailable.".to_string())?;
        receiver
            .recv()
            .map_err(|_| "Scheduler stopped before replying.".to_string())?
    }

    pub fn history(
        &self,
        range: crate::history::HistoryRange,
    ) -> Result<crate::history::History, String> {
        let (response, receiver) = mpsc::channel();
        self.sender
            .send(Request::History { range, response })
            .map_err(|_| "Scheduler is unavailable.".to_string())?;
        receiver
            .recv()
            .map_err(|_| "History request stopped before replying.".to_string())?
    }

    pub fn clear_history(&self, confirmed: bool) -> Result<(), String> {
        let (response, receiver) = mpsc::channel();
        self.sender
            .send(Request::ClearHistory {
                confirmed,
                response,
            })
            .map_err(|_| "Scheduler is unavailable.".to_string())?;
        receiver
            .recv()
            .map_err(|_| "History deletion stopped before replying.".to_string())?
    }
}

pub fn spawn(storage: Storage, app: AppHandle) -> Result<SchedulerHandle, String> {
    let (sender, receiver) = mpsc::channel();
    let (ready, ready_rx) = mpsc::sync_channel(1);
    thread::Builder::new()
        .name("pausepal-scheduler".into())
        .spawn(move || {
            match Session::open(storage, SystemClock::default()) {
                Ok((session, effects)) => {
                    // Startup recovery is committed before the handle is published.
                    let _ = ready.send(Ok(()));
                    publish(&app, &session.snapshot(), &effects, true);
                    run(session, receiver, app);
                }
                Err(error) => {
                    let _ = ready.send(Err(error));
                }
            }
        })
        .map_err(|_| "Could not start the Rust scheduler worker.".to_string())?;
    ready_rx
        .recv()
        .map_err(|_| "Scheduler initialization failed.".to_string())??;
    Ok(SchedulerHandle { sender })
}

fn run(mut session: Session<SystemClock>, receiver: Receiver<Request>, app: AppHandle) {
    loop {
        let request = match receiver.recv_timeout(if session.failed {
            std::time::Duration::from_secs(30)
        } else {
            session.engine.next_wait(&session.clock)
        }) {
            Ok(request) => Some(request),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => break,
        };
        let (action, response) = match request {
            Some(Request::History { range, response }) => {
                let _ = response.send(session.history(range));
                continue;
            }
            Some(Request::ClearHistory {
                confirmed,
                response,
            }) => {
                let result = session.clear_history(confirmed);
                if result.is_ok() {
                    let _ = app.emit_to("main", "history-changed", ());
                }
                let _ = response.send(result);
                continue;
            }
            Some(Request::Scheduler { action, response }) => (action, Some(response)),
            None => (Action::Tick, None),
        };
        if session.failed {
            if let Some(response) = response {
                let _ = response.send(Err(STORAGE_FAILURE.into()));
            }
            continue;
        }
        let previous_revision = session.engine.data.runtime.revision;
        let automatic = !matches!(action, Action::Start(_));
        match session.apply(action) {
            Ok(effects) => {
                let snapshot = session.snapshot();
                if snapshot.revision != previous_revision {
                    publish(&app, &snapshot, &effects, automatic);
                }
                if let Some(response) = response {
                    let _ = response.send(Ok(snapshot));
                }
            }
            Err(error) => {
                if session.failed {
                    for window in ["main", "break"] {
                        let _ = app.emit_to(
                            window,
                            "scheduler-state-changed",
                            StateEvent::Error {
                                message: error.clone(),
                            },
                        );
                    }
                    crate::native::warn(&app, &error);
                }
                if let Some(response) = response {
                    let _ = response.send(Err(error));
                }
            }
        }
    }
}

fn publish(app: &AppHandle, state: &DashboardState, effects: &Effects, automatic: bool) {
    for window in ["main", "break"] {
        let _ = app.emit_to(
            window,
            "scheduler-state-changed",
            StateEvent::State {
                state: state.clone(),
            },
        );
    }
    crate::native::committed(app, state, effects, automatic);
    if effects.settings_changed {
        let _ = app.emit_to("main", "settings-updated", &state.settings);
    }
    for reminder in &effects.due {
        let _ = app.emit_to("main", "reminder-due", reminder);
    }
    for event in &effects.recorded {
        let _ = app.emit_to("main", "break-event-recorded", event);
    }
}
