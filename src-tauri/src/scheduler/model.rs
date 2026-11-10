use crate::settings::Settings;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Eye,
    Movement,
}

impl Category {
    pub const ALL: [Self; 2] = [Self::Eye, Self::Movement];
    pub fn index(self) -> usize {
        match self {
            Self::Eye => 0,
            Self::Movement => 1,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Eye => "eye",
            Self::Movement => "movement",
        }
    }
    pub fn enabled(self, settings: &Settings) -> bool {
        match self {
            Self::Eye => settings.eye_enabled,
            Self::Movement => settings.movement_enabled,
        }
    }
    pub fn interval(self, settings: &Settings) -> i64 {
        i64::from(match self {
            Self::Eye => settings.eye_interval_minutes,
            Self::Movement => settings.movement_interval_minutes,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Outcome {
    Completed,
    Skipped,
    Snoozed,
    Unanswered,
}
impl Outcome {
    pub fn name(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Skipped => "skipped",
            Self::Snoozed => "snoozed",
            Self::Unanswered => "unanswered",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReminderState {
    pub category: Category,
    pub next_due_at_utc: DateTime<Utc>,
    pub open_reminder_id: Option<Uuid>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OpenReminder {
    pub id: Uuid,
    pub category: Category,
    pub triggered_at_utc: DateTime<Utc>,
    pub expires_at_utc: DateTime<Utc>,
    pub started_at_utc: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakEvent {
    pub id: Uuid,
    pub category: Category,
    pub outcome: Outcome,
    pub triggered_at_utc: DateTime<Utc>,
    pub acted_at_utc: DateTime<Utc>,
    pub duration_seconds: Option<i64>,
    pub created_at_utc: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuntimeMeta {
    pub open: Option<OpenReminder>,
    pub next_allowed_at_utc: Option<DateTime<Utc>>,
    pub revision: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Persisted {
    pub settings: Settings,
    pub reminders: [ReminderState; 2],
    pub runtime: RuntimeMeta,
}

impl Persisted {
    pub fn validate(&self) -> Result<(), String> {
        self.settings.validate()?;
        for category in Category::ALL {
            let state = &self.reminders[category.index()];
            if state.category != category
                || state.open_reminder_id
                    != self
                        .runtime
                        .open
                        .as_ref()
                        .filter(|open| open.category == category)
                        .map(|open| open.id)
            {
                return Err("Inconsistent persisted reminder state.".into());
            }
        }
        if let Some(open) = &self.runtime.open {
            if open.expires_at_utc <= open.triggered_at_utc
                || open
                    .started_at_utc
                    .is_some_and(|started| started < open.triggered_at_utc)
            {
                return Err("Invalid persisted open reminder.".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RunStatus {
    Active,
    Paused,
    OutsideWorkHours,
    Quiet,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReminderStatus {
    Scheduled,
    Open,
    Disabled,
    Paused,
    OutsideWorkHours,
    Quiet,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderView {
    pub category: Category,
    pub next_due_at_utc: DateTime<Utc>,
    pub open_reminder_id: Option<Uuid>,
    pub status: ReminderStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardState {
    pub revision: u64,
    pub status: RunStatus,
    pub settings: Settings,
    pub reminders: [ReminderView; 2],
    pub open_reminder: Option<OpenReminder>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PauseRequest {
    Minutes { minutes: u32 },
    UntilTomorrow {},
    Manual {},
}

#[derive(Clone, Debug)]
pub enum Action {
    Tick,
    NativePreferences {
        notification_enabled: bool,
        autostart_enabled: bool,
    },
    UpdateSettings(Settings),
    Pause(PauseRequest),
    Resume,
    Start(Category),
    Complete(Uuid),
    Skip(Uuid),
    Snooze(Uuid),
}

#[derive(Default, Debug)]
pub struct Effects {
    pub due: Vec<OpenReminder>,
    pub recorded: Vec<BreakEvent>,
    pub settings_changed: bool,
}
