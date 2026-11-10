use chrono::{DateTime, FixedOffset, Local, LocalResult, NaiveDateTime, Offset, TimeZone, Utc};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Sample {
    pub utc: DateTime<Utc>,
    pub monotonic: Duration,
    pub timezone: String,
    pub offset_seconds: i32,
}

pub trait Clock: Send + 'static {
    fn sample(&self) -> Sample;
    fn local_at(&self, utc: DateTime<Utc>) -> DateTime<FixedOffset>;
    fn resolve_local(&self, local: NaiveDateTime) -> LocalResult<DateTime<Utc>>;
}

pub struct SystemClock {
    origin: Instant,
}

impl Default for SystemClock {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Clock for SystemClock {
    fn sample(&self) -> Sample {
        let utc = Utc::now();
        Sample {
            utc,
            monotonic: self.origin.elapsed(),
            timezone: iana_time_zone::get_timezone().unwrap_or_else(|_| "system-local".into()),
            offset_seconds: utc.with_timezone(&Local).offset().fix().local_minus_utc(),
        }
    }

    fn local_at(&self, utc: DateTime<Utc>) -> DateTime<FixedOffset> {
        utc.with_timezone(&Local).fixed_offset()
    }

    fn resolve_local(&self, local: NaiveDateTime) -> LocalResult<DateTime<Utc>> {
        Local
            .from_local_datetime(&local)
            .map(|time| time.with_timezone(&Utc))
    }
}
