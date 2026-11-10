use super::model::*;
use crate::{
    clock::{Clock, Sample},
    settings::{self, Settings},
};
use chrono::{DateTime, Duration, Utc};
use std::time::Duration as Wait;
use uuid::Uuid;

const SPACING: i64 = 120;
const TIMEOUT: i64 = 300;
const WATCH: u64 = 30;

#[derive(Clone)]
pub struct Engine {
    pub data: Persisted,
    last: Sample,
    due_mono: [Wait; 2],
    open_expires_mono: Option<Wait>,
    started_mono: Option<Wait>,
    allowed_mono: Wait,
}

impl Engine {
    pub fn restore(data: Option<Persisted>, clock: &dyn Clock) -> Result<(Self, Effects), String> {
        let sample = clock.sample();
        let was_saved = data.is_some();
        let data = data.unwrap_or_else(|| Persisted {
            settings: Settings::default(),
            reminders: Category::ALL.map(|category| ReminderState {
                category,
                next_due_at_utc: sample.utc
                    + Duration::minutes(category.interval(&Settings::default())),
                open_reminder_id: None,
            }),
            runtime: RuntimeMeta {
                open: None,
                next_allowed_at_utc: None,
                revision: 0,
            },
        });
        data.validate()?;
        let mut engine = Self {
            data,
            last: sample.clone(),
            due_mono: [Wait::ZERO; 2],
            open_expires_mono: None,
            started_mono: None,
            allowed_mono: Wait::ZERO,
        };
        let mut effects = Effects::default();
        if engine
            .data
            .settings
            .paused_until_utc
            .is_some_and(|until| until <= sample.utc)
        {
            engine.data.settings.paused_until_utc = None;
            effects.settings_changed = true;
            engine.reset_all(&sample, clock);
        }
        if was_saved {
            // A backwards wall-clock adjustment while the process was closed
            // cannot be detected from a new monotonic origin. No legitimate
            // deadline needs to be later than a fresh category interval shifted
            // into the current work calendar; retain earlier valid deadlines.
            for category in Category::ALL {
                let latest = engine.data.settings.next_work(
                    sample.utc + Duration::minutes(category.interval(&engine.data.settings)),
                    clock,
                );
                if engine.data.reminders[category.index()].next_due_at_utc > latest {
                    engine.schedule(category, latest, &sample, clock);
                }
            }
            if let Some(allowed) = engine.data.runtime.next_allowed_at_utc.as_mut() {
                *allowed = (*allowed).min(sample.utc + Duration::seconds(SPACING));
            }
            engine.recover(&sample, clock, false, &mut effects);
        }
        engine.normalize(&sample, clock);
        engine.anchor(&sample);
        engine.tick(&sample, clock, &mut effects);
        engine.data.runtime.revision += 1;
        Ok((engine, effects))
    }

    pub fn snapshot(&self, clock: &dyn Clock) -> DashboardState {
        let now = self.last.utc;
        let settings = &self.data.settings;
        let status = if settings.paused(now) {
            RunStatus::Paused
        } else if settings.quiet_mode {
            RunStatus::Quiet
        } else if !settings.eye_enabled && !settings.movement_enabled {
            RunStatus::Disabled
        } else if !settings.within_work(now, clock) {
            RunStatus::OutsideWorkHours
        } else {
            RunStatus::Active
        };
        DashboardState {
            revision: self.data.runtime.revision,
            status,
            settings: settings.clone(),
            reminders: self.data.reminders.each_ref().map(|reminder| ReminderView {
                category: reminder.category,
                next_due_at_utc: reminder.next_due_at_utc,
                open_reminder_id: reminder.open_reminder_id,
                status: if !reminder.category.enabled(settings) {
                    ReminderStatus::Disabled
                } else if reminder.open_reminder_id.is_some() {
                    ReminderStatus::Open
                } else {
                    match status {
                        RunStatus::Active => ReminderStatus::Scheduled,
                        RunStatus::Paused => ReminderStatus::Paused,
                        RunStatus::OutsideWorkHours => ReminderStatus::OutsideWorkHours,
                        RunStatus::Quiet => ReminderStatus::Quiet,
                        RunStatus::Disabled => ReminderStatus::Disabled,
                    }
                },
            }),
            open_reminder: self.data.runtime.open.clone(),
        }
    }

    pub fn step(&mut self, action: Action, clock: &dyn Clock) -> Result<Effects, String> {
        let sample = clock.sample();
        let mut effects = Effects::default();
        let before = self.data.clone();
        let old_status = self.snapshot(clock).status;
        let mono_delta = sample.monotonic.saturating_sub(self.last.monotonic);
        let wall_delta = (sample.utc - self.last.utc).num_milliseconds();
        let drift = (wall_delta - mono_delta.as_millis().min(i64::MAX as u128) as i64).abs();
        let changed_clock = drift > 2000
            || sample.monotonic < self.last.monotonic
            || sample.timezone != self.last.timezone
            || sample.offset_seconds != self.last.offset_seconds;
        if changed_clock || mono_delta > Wait::from_secs(90) {
            // Mutating suppression/settings commands must never emit a reminder
            // immediately before applying the user's suppression request.
            let allow_delivery = matches!(action, Action::Tick);
            self.recover(&sample, clock, changed_clock, &mut effects);
            self.anchor(&sample);
            if !allow_delivery {
                self.reset_all(&sample, clock);
            }
        }
        if self
            .data
            .settings
            .paused_until_utc
            .is_some_and(|until| until <= sample.utc)
        {
            self.data.settings.paused_until_utc = None;
            effects.settings_changed = true;
            self.reset_all(&sample, clock);
        }
        match action {
            Action::Tick => (),
            Action::NativePreferences {
                notification_enabled,
                autostart_enabled,
            } => {
                if self.data.settings.notification_enabled != notification_enabled
                    || self.data.settings.autostart_enabled != autostart_enabled
                {
                    self.data.settings.notification_enabled = notification_enabled;
                    self.data.settings.autostart_enabled = autostart_enabled;
                    effects.settings_changed = true;
                }
            }
            Action::UpdateSettings(settings) => {
                settings.validate()?;
                if settings.notification_enabled != self.data.settings.notification_enabled
                    || settings.autostart_enabled != self.data.settings.autostart_enabled
                {
                    return Err(
                        "Change native preferences through their dedicated commands.".into(),
                    );
                }
                if settings.paused_until_utc != self.data.settings.paused_until_utc
                    || settings.paused_indefinitely != self.data.settings.paused_indefinitely
                {
                    return Err("Change pause state through pause/resume commands.".into());
                }
                if settings != self.data.settings {
                    self.finish_open(Outcome::Unanswered, &sample, clock, &mut effects);
                    self.data.settings = settings;
                    self.reset_all(&sample, clock);
                    effects.settings_changed = true;
                }
            }
            Action::Pause(request) => {
                let deadline = match request {
                    PauseRequest::Minutes { minutes } if [30, 60, 120].contains(&minutes) => {
                        Some(sample.utc + Duration::minutes(i64::from(minutes)))
                    }
                    PauseRequest::Minutes { .. } => {
                        return Err("Pause must be 30, 60, or 120 minutes.".into())
                    }
                    PauseRequest::UntilTomorrow {} => Some(settings::tomorrow(sample.utc, clock)),
                    PauseRequest::Manual {} => None,
                };
                self.finish_open(Outcome::Unanswered, &sample, clock, &mut effects);
                self.data.settings.paused_indefinitely = deadline.is_none();
                self.data.settings.paused_until_utc = deadline;
                self.reset_all(&sample, clock);
                effects.settings_changed = true;
            }
            Action::Resume => {
                if self.data.settings.paused_indefinitely
                    || self.data.settings.paused_until_utc.is_some()
                {
                    self.data.settings.paused_indefinitely = false;
                    self.data.settings.paused_until_utc = None;
                    self.reset_all(&sample, clock);
                    effects.settings_changed = true;
                }
            }
            Action::Start(category) => {
                if let Some(open) = self.data.runtime.open.as_mut() {
                    if open.category != category {
                        return Err("Another reminder is already open.".into());
                    }
                    if sample.monotonic >= self.open_expires_mono.unwrap_or(Wait::ZERO) {
                        return Err("This reminder has expired.".into());
                    }
                    if open.started_at_utc.is_none() {
                        open.started_at_utc = Some(sample.utc);
                        self.started_mono = Some(sample.monotonic);
                    }
                } else {
                    if !self.data.settings.eligible(sample.utc, clock)
                        || !category.enabled(&self.data.settings)
                        || sample.monotonic < self.allowed_mono
                    {
                        return Err("A break cannot be opened while scheduling is suppressed or within the delivery spacing period.".into());
                    }
                    self.deliver(category, &sample, clock, &mut effects);
                    self.data.runtime.open.as_mut().unwrap().started_at_utc = Some(sample.utc);
                    self.started_mono = Some(sample.monotonic);
                    effects.due[0] = self.data.runtime.open.clone().unwrap();
                }
            }
            Action::Complete(id) | Action::Skip(id) | Action::Snooze(id) => {
                let outcome = match action {
                    Action::Complete(_) => Outcome::Completed,
                    Action::Skip(_) => Outcome::Skipped,
                    _ => Outcome::Snoozed,
                };
                let open = self
                    .data
                    .runtime
                    .open
                    .as_ref()
                    .ok_or("No matching open reminder.")?;
                if open.id != id
                    || sample.monotonic >= self.open_expires_mono.unwrap_or(Wait::ZERO)
                    || !self.data.settings.eligible(sample.utc, clock)
                {
                    return Err("Reminder is stale, expired, or no longer eligible.".into());
                }
                self.finish_open(outcome, &sample, clock, &mut effects);
            }
        }
        self.tick(&sample, clock, &mut effects);
        self.last = sample;
        if self.data != before || self.snapshot(clock).status != old_status {
            self.data.runtime.revision += 1;
        }
        Ok(effects)
    }

    fn reset_all(&mut self, sample: &Sample, clock: &dyn Clock) {
        for category in Category::ALL {
            self.schedule(
                category,
                sample.utc + Duration::minutes(category.interval(&self.data.settings)),
                sample,
                clock,
            );
        }
    }

    fn schedule(
        &mut self,
        category: Category,
        at: DateTime<Utc>,
        sample: &Sample,
        clock: &dyn Clock,
    ) {
        let at = self.data.settings.next_work(at, clock);
        self.data.reminders[category.index()].next_due_at_utc = at;
        self.due_mono[category.index()] = mono_at(at, sample);
    }

    fn normalize(&mut self, sample: &Sample, clock: &dyn Clock) {
        for category in Category::ALL {
            let at = self.data.reminders[category.index()].next_due_at_utc;
            if !self.data.settings.within_work(at, clock)
                || (!self.data.settings.within_work(sample.utc, clock) && at < sample.utc)
            {
                self.schedule(category, at.max(sample.utc), sample, clock);
            }
        }
    }

    fn anchor(&mut self, sample: &Sample) {
        self.due_mono = self
            .data
            .reminders
            .each_ref()
            .map(|state| mono_at(state.next_due_at_utc, sample));
        self.allowed_mono = self
            .data
            .runtime
            .next_allowed_at_utc
            .map(|at| mono_at(at, sample))
            .unwrap_or(sample.monotonic);
        self.open_expires_mono = self
            .data
            .runtime
            .open
            .as_ref()
            .map(|open| mono_at(open.expires_at_utc, sample));
    }

    fn recover(
        &mut self,
        sample: &Sample,
        clock: &dyn Clock,
        reset_future: bool,
        effects: &mut Effects,
    ) {
        let remaining_spacing = self
            .allowed_mono
            .saturating_sub(sample.monotonic)
            .min(Wait::from_secs(SPACING as u64));
        let had_open = self.data.runtime.open.as_ref().map(|open| open.category);
        let overdue = Category::ALL
            .into_iter()
            .filter(|category| {
                category.enabled(&self.data.settings)
                    && Some(*category) != had_open
                    && self.data.reminders[category.index()].next_due_at_utc <= sample.utc
            })
            .min_by_key(|category| self.data.reminders[category.index()].next_due_at_utc);
        self.finish_open(Outcome::Unanswered, sample, clock, effects);
        for category in Category::ALL {
            if reset_future || self.data.reminders[category.index()].next_due_at_utc <= sample.utc {
                self.schedule(
                    category,
                    sample.utc + Duration::minutes(category.interval(&self.data.settings)),
                    sample,
                    clock,
                );
            }
        }
        // Mark at most one candidate due; normal tick still enforces pause,
        // work hours, quiet mode, and persisted global delivery spacing.
        if let Some(category) = overdue.filter(|_| self.data.settings.eligible(sample.utc, clock)) {
            self.schedule(category, sample.utc, sample, clock);
        }
        if reset_future {
            self.data.runtime.next_allowed_at_utc = Some(sample.utc + Duration::seconds(SPACING));
            // A wall-clock jump must not bypass an existing monotonic guard.
            // Cap it at two minutes if the monotonic clock itself regressed.
            if overdue.is_some() {
                self.data.runtime.next_allowed_at_utc =
                    Some(sample.utc + Duration::from_std(remaining_spacing).unwrap());
            }
        }
    }

    fn tick(&mut self, sample: &Sample, clock: &dyn Clock, effects: &mut Effects) {
        let eligible = self.data.settings.eligible(sample.utc, clock);
        if self.data.runtime.open.is_some()
            && (!eligible
                || self
                    .open_expires_mono
                    .is_some_and(|expiry| sample.monotonic >= expiry))
        {
            self.finish_open(Outcome::Unanswered, sample, clock, effects);
        }
        self.normalize(sample, clock);
        if !eligible || self.data.runtime.open.is_some() || sample.monotonic < self.allowed_mono {
            return;
        }
        if let Some(category) = Category::ALL
            .into_iter()
            .filter(|category| {
                category.enabled(&self.data.settings)
                    && self.due_mono[category.index()] <= sample.monotonic
            })
            .min_by_key(|category| self.data.reminders[category.index()].next_due_at_utc)
        {
            self.deliver(category, sample, clock, effects);
        }
    }

    fn deliver(
        &mut self,
        category: Category,
        sample: &Sample,
        clock: &dyn Clock,
        effects: &mut Effects,
    ) {
        let open = OpenReminder {
            id: Uuid::new_v4(),
            category,
            triggered_at_utc: sample.utc,
            expires_at_utc: sample.utc + Duration::seconds(TIMEOUT),
            started_at_utc: None,
        };
        self.data.reminders[category.index()].open_reminder_id = Some(open.id);
        self.data.runtime.open = Some(open.clone());
        self.open_expires_mono = Some(sample.monotonic + Wait::from_secs(TIMEOUT as u64));
        self.started_mono = None;
        self.data.runtime.next_allowed_at_utc = Some(sample.utc + Duration::seconds(SPACING));
        self.allowed_mono = sample.monotonic + Wait::from_secs(SPACING as u64);
        let other = Category::ALL[1 - category.index()];
        if self.data.reminders[other.index()].next_due_at_utc
            < sample.utc + Duration::seconds(SPACING)
        {
            self.schedule(
                other,
                sample.utc + Duration::seconds(SPACING),
                sample,
                clock,
            );
        }
        effects.due.push(open);
    }

    fn finish_open(
        &mut self,
        outcome: Outcome,
        sample: &Sample,
        clock: &dyn Clock,
        effects: &mut Effects,
    ) {
        if let Some(open) = self.data.runtime.open.take() {
            effects.recorded.push(BreakEvent {
                id: Uuid::new_v4(),
                category: open.category,
                outcome,
                triggered_at_utc: open.triggered_at_utc,
                acted_at_utc: sample.utc,
                duration_seconds: if outcome == Outcome::Completed {
                    self.started_mono
                        .map(|start| sample.monotonic.saturating_sub(start).as_secs() as i64)
                } else {
                    None
                },
                created_at_utc: sample.utc,
            });
            self.data.reminders[open.category.index()].open_reminder_id = None;
            let minutes = if outcome == Outcome::Snoozed {
                5
            } else {
                open.category.interval(&self.data.settings)
            };
            self.schedule(
                open.category,
                sample.utc + Duration::minutes(minutes),
                sample,
                clock,
            );
            self.open_expires_mono = None;
            self.started_mono = None;
        }
    }

    pub fn next_wait(&self, clock: &dyn Clock) -> Wait {
        let sample = clock.sample();
        let mut deadlines = vec![sample.monotonic + Wait::from_secs(WATCH)];
        if let Some(until) = self
            .data
            .settings
            .paused_until_utc
            .filter(|until| *until > sample.utc)
        {
            deadlines.push(mono_at(until, &sample));
        }
        if let Some(expiry) = self.open_expires_mono {
            deadlines.push(expiry);
        }
        if self.data.settings.eligible(sample.utc, clock) && self.data.runtime.open.is_none() {
            for category in Category::ALL
                .into_iter()
                .filter(|category| category.enabled(&self.data.settings))
            {
                deadlines.push(self.due_mono[category.index()].max(self.allowed_mono));
            }
        }
        // Work-window transitions matter even without a due reminder.
        let boundary = if self.data.settings.within_work(sample.utc, clock) {
            settings::resolve_boundary(
                clock
                    .local_at(sample.utc)
                    .date_naive()
                    .and_time(self.data.settings.work_end_local),
                clock,
            )
        } else {
            self.data.settings.next_work(sample.utc, clock)
        };
        if boundary > sample.utc {
            deadlines.push(mono_at(boundary, &sample));
        }
        deadlines
            .into_iter()
            .min()
            .unwrap()
            .saturating_sub(sample.monotonic)
            .max(Wait::from_millis(100))
    }
}

fn mono_at(at: DateTime<Utc>, sample: &Sample) -> Wait {
    sample.monotonic + (at - sample.utc).to_std().unwrap_or(Wait::ZERO)
}
