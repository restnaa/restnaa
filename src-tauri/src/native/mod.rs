pub mod model;
pub mod notifications;
pub mod startup;
pub mod tray;
pub mod windows;

use crate::{
    scheduler::model::{DashboardState, Effects},
    FoundationState,
};
use model::{Attempt, NativeStatus};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Default)]
pub struct NativeRuntime {
    pub quitting: AtomicBool,
    pub tray_available: AtomicBool,
    pub preference_lock: Mutex<()>,
    pub automatic_break_id: Mutex<Option<uuid::Uuid>>,
    pub last_attempt: Mutex<Option<Attempt>>,
    pub warning: Mutex<Option<String>>,
    pub tray_controls: Mutex<Option<tray::TrayControls>>,
}

pub fn warn(app: &AppHandle, message: &str) {
    if let Some(runtime) = app.try_state::<NativeRuntime>() {
        if let Ok(mut warning) = runtime.warning.lock() {
            *warning = Some(message.into());
        }
    }
    let _ = app.emit_to("main", "native-status-changed", ());
}

pub fn scheduler(app: &AppHandle) -> Result<crate::scheduler::service::SchedulerHandle, String> {
    app.try_state::<FoundationState>()
        .and_then(|state| state.scheduler.clone())
        .ok_or_else(|| "Scheduler unavailable. Check local storage.".into())
}

pub fn status(app: &AppHandle, state: &DashboardState) -> NativeStatus {
    use tauri_plugin_autostart::ManagerExt;
    let runtime = app.state::<NativeRuntime>();
    let (windows_permission, attention) = notifications::windows_status(app);
    NativeStatus {
        notification_enabled: state.settings.notification_enabled,
        autostart_enabled: state.settings.autostart_enabled,
        autostart_registered: app.autolaunch().is_enabled().ok(),
        windows_permission,
        attention,
        tray_available: runtime.tray_available.load(Ordering::SeqCst),
        last_attempt: runtime
            .last_attempt
            .lock()
            .ok()
            .and_then(|last| last.clone()),
        warning: runtime
            .warning
            .lock()
            .ok()
            .and_then(|warning| warning.clone()),
    }
}

pub fn committed(app: &AppHandle, state: &DashboardState, effects: &Effects, automatic: bool) {
    tray::update(app, state);
    if state.open_reminder.is_none() {
        if let Ok(mut last) = app.state::<NativeRuntime>().automatic_break_id.lock() {
            *last = None;
        }
        if let Err(error) = windows::hide_break(app) {
            warn(app, &error);
        }
    }
    if automatic {
        for reminder in &effects.due {
            notifications::reminder(app, state, reminder);
            let (permission, attention) = notifications::windows_status(app);
            if model::automatic_break_allowed(state, reminder, permission, attention) {
                let fresh = app
                    .state::<NativeRuntime>()
                    .automatic_break_id
                    .lock()
                    .map(|mut last| model::claim_automatic_break(&mut last, reminder.id))
                    .unwrap_or(false);
                if fresh {
                    if let Err(error) = windows::show_break_automatically(app) {
                        warn(app, &error);
                    }
                }
            }
        }
    }
    if effects.settings_changed {
        let _ = app.emit_to("main", "native-status-changed", ());
    }
}

pub fn quit(app: &AppHandle) {
    app.state::<NativeRuntime>()
        .quitting
        .store(true, Ordering::SeqCst);
    app.exit(0);
}
