use crate::clock::Clock;
use chrono::{
    DateTime, Datelike, Duration, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Settings {
    pub eye_enabled: bool,
    pub movement_enabled: bool,
    pub eye_interval_minutes: u32,
    pub movement_interval_minutes: u32,
    #[serde(default = "notifications_default")]
    pub water_enabled: bool,
    #[serde(default = "water_interval_default")]
    pub water_interval_minutes: u32,
    /// Monday = 0, Sunday = 6.
    pub work_days: Vec<u32>,
    pub work_start_local: NaiveTime,
    pub work_end_local: NaiveTime,
    pub quiet_mode: bool,
    pub paused_until_utc: Option<DateTime<Utc>>,
    pub paused_indefinitely: bool,
    #[serde(default = "notifications_default")]
    pub notification_enabled: bool,
    #[serde(default = "notifications_default")]
    pub show_break_window: bool,
    #[serde(default)]
    pub autostart_enabled: bool,
    #[serde(default)]
    pub onboarding_completed: bool,
}

fn notifications_default() -> bool {
    true
}

fn water_interval_default() -> u32 {
    60
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            eye_enabled: true,
            movement_enabled: true,
            eye_interval_minutes: 20,
            movement_interval_minutes: 45,
            water_enabled: true,
            water_interval_minutes: water_interval_default(),
            work_days: vec![0, 1, 2, 3, 4],
            work_start_local: NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
            work_end_local: NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            quiet_mode: false,
            paused_until_utc: None,
            paused_indefinitely: false,
            notification_enabled: true,
            show_break_window: true,
            autostart_enabled: false,
            onboarding_completed: false,
        }
    }
}

impl Settings {
    pub fn validate(&self) -> Result<(), String> {
        if !(30..=240).contains(&self.water_interval_minutes) {
            return Err("Water reminder interval must be 30–240 minutes. This is an app preference, not a medical recommendation.".into());
        }
        if !(10..=120).contains(&self.eye_interval_minutes)
            || !(15..=180).contains(&self.movement_interval_minutes)
        {
            return Err(
                "Eye interval must be 10–120 minutes and movement interval 15–180 minutes.".into(),
            );
        }
        if self.work_days.is_empty()
            || self.work_days.len() > 7
            || self.work_days.iter().any(|day| *day > 6)
        {
            return Err("Choose at least one valid working day (Monday=0 to Sunday=6).".into());
        }
        let mut days = self.work_days.clone();
        days.sort();
        days.dedup();
        if days.len() != self.work_days.len() {
            return Err("Working days must be unique.".into());
        }
        if self.work_end_local <= self.work_start_local
            || self.work_start_local.second() != 0
            || self.work_end_local.second() != 0
            || self.work_start_local.nanosecond() != 0
            || self.work_end_local.nanosecond() != 0
        {
            return Err("Working hours must use whole minutes and end after they start; overnight windows are not supported.".into());
        }
        if self.paused_indefinitely && self.paused_until_utc.is_some() {
            return Err("Pause must be manual or time-limited, not both.".into());
        }
        Ok(())
    }

    pub fn paused(&self, now: DateTime<Utc>) -> bool {
        self.paused_indefinitely || self.paused_until_utc.is_some_and(|until| until > now)
    }

    pub fn within_work(&self, now: DateTime<Utc>, clock: &dyn Clock) -> bool {
        let local = clock.local_at(now);
        self.work_days
            .contains(&local.weekday().num_days_from_monday())
            && local.time() >= self.work_start_local
            && local.time() < self.work_end_local
    }

    pub fn eligible(&self, now: DateTime<Utc>, clock: &dyn Clock) -> bool {
        !self.paused(now) && !self.quiet_mode && self.within_work(now, clock)
    }

    pub fn next_work(&self, at: DateTime<Utc>, clock: &dyn Clock) -> DateTime<Utc> {
        if self.within_work(at, clock) {
            return at;
        }
        let local = clock.local_at(at);
        for offset in 0..=8 {
            let date = local.date_naive() + Duration::days(offset);
            if !self
                .work_days
                .contains(&date.weekday().num_days_from_monday())
            {
                continue;
            }
            let local_start = date.and_time(self.work_start_local);
            let start = match clock.resolve_local(local_start) {
                LocalResult::Ambiguous(a, b) if a.min(b) < at && a.max(b) >= at => a.max(b),
                _ => resolve_boundary(local_start, clock),
            };
            if start >= at && self.within_work(start, clock) {
                return start;
            }
        }
        // Valid settings always provide a window within the next eight days.
        // A timezone that skips an entire selected window may require another week.
        self.next_work(at + Duration::days(7), clock)
    }
}

/// Earlier occurrence for a fold; advance to the first real minute for a gap.
pub fn resolve_boundary(mut local: NaiveDateTime, clock: &dyn Clock) -> DateTime<Utc> {
    loop {
        match clock.resolve_local(local) {
            LocalResult::Single(time) => return time,
            LocalResult::Ambiguous(a, b) => return a.min(b),
            LocalResult::None => local += Duration::minutes(1),
        }
    }
}

pub fn tomorrow(now: DateTime<Utc>, clock: &dyn Clock) -> DateTime<Utc> {
    let date: NaiveDate = clock.local_at(now).date_naive() + Duration::days(1);
    resolve_boundary(date.and_hms_opt(0, 0, 0).unwrap(), clock)
}
