#![allow(dead_code)]
use chrono::{DateTime, Duration, FixedOffset, LocalResult, NaiveDateTime, Offset, TimeZone, Utc};
use chrono_tz::Tz;
use restnaa_lib::{
    clock::{Clock, Sample},
    scheduler::{
        engine::Engine,
        model::{Action, Effects},
    },
    settings::Settings,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration as StdDuration,
};

#[derive(Clone)]
pub struct FakeClock(Arc<Mutex<(DateTime<Utc>, StdDuration, Tz)>>);
impl FakeClock {
    pub fn at(value: &str) -> Self {
        Self(Arc::new(Mutex::new((
            value.parse().unwrap(),
            StdDuration::ZERO,
            chrono_tz::UTC,
        ))))
    }
    pub fn advance(&self, seconds: i64) {
        let mut value = self.0.lock().unwrap();
        value.0 += Duration::seconds(seconds);
        value.1 += StdDuration::from_secs(seconds as u64);
    }
    pub fn jump(&self, seconds: i64) {
        self.0.lock().unwrap().0 += Duration::seconds(seconds);
    }
    pub fn zone(&self, zone: Tz) {
        self.0.lock().unwrap().2 = zone;
    }
    pub fn pump(&self, engine: &mut Engine, seconds: i64) -> Effects {
        let mut effects = Effects::default();
        let mut remaining = seconds;
        while remaining > 0 {
            let step = remaining.min(30);
            self.advance(step);
            remaining -= step;
            let next = engine.step(Action::Tick, self).unwrap();
            effects.due.extend(next.due);
            effects.recorded.extend(next.recorded);
            effects.settings_changed |= next.settings_changed;
        }
        effects
    }
}
impl Clock for FakeClock {
    fn sample(&self) -> Sample {
        let value = self.0.lock().unwrap();
        Sample {
            utc: value.0,
            monotonic: value.1,
            timezone: value.2.name().into(),
            offset_seconds: value
                .0
                .with_timezone(&value.2)
                .offset()
                .fix()
                .local_minus_utc(),
        }
    }
    fn local_at(&self, utc: DateTime<Utc>) -> DateTime<FixedOffset> {
        utc.with_timezone(&self.0.lock().unwrap().2).fixed_offset()
    }
    fn resolve_local(&self, local: NaiveDateTime) -> LocalResult<DateTime<Utc>> {
        self.0
            .lock()
            .unwrap()
            .2
            .from_local_datetime(&local)
            .map(|date| date.with_timezone(&Utc))
    }
}
pub fn configured(clock: &FakeClock, settings: Settings) -> Engine {
    let (mut engine, _) = Engine::restore(None, clock).unwrap();
    engine
        .step(Action::UpdateSettings(settings), clock)
        .unwrap();
    engine
}
