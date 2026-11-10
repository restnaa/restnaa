use crate::{clock::Clock, scheduler::model::*};
use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

pub const RECENT_LIMIT: usize = 100;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HistoryRange {
    Today,
    SevenDays,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutcomeCounts {
    pub completed: u64,
    pub skipped: u64,
    pub snoozed: u64,
    pub unanswered: u64,
}
impl OutcomeCounts {
    pub fn record(&mut self, outcome: Outcome) {
        *match outcome {
            Outcome::Completed => &mut self.completed,
            Outcome::Skipped => &mut self.skipped,
            Outcome::Snoozed => &mut self.snoozed,
            Outcome::Unanswered => &mut self.unanswered,
        } += 1;
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCounts {
    pub category: Category,
    pub counts: OutcomeCounts,
}
pub fn empty_counts() -> [CategoryCounts; 2] {
    Category::ALL.map(|category| CategoryCounts {
        category,
        counts: OutcomeCounts::default(),
    })
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DaySummary {
    pub date: NaiveDate,
    pub categories: [CategoryCounts; 2],
}
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub range: HistoryRange,
    pub from_date: NaiveDate,
    pub through_date: NaiveDate,
    pub as_of_utc: DateTime<Utc>,
    pub categories: [CategoryCounts; 2],
    pub days: Vec<DaySummary>,
    pub recent: Vec<BreakEvent>,
    pub total_events: u64,
    pub recent_truncated: bool,
}
impl History {
    pub fn empty(range: HistoryRange, clock: &dyn Clock) -> Self {
        let now = clock.sample().utc;
        let today = clock.local_at(now).date_naive();
        let length = match range {
            HistoryRange::Today => 1,
            HistoryRange::SevenDays => 7,
        };
        let from = today - Duration::days(length - 1);
        Self {
            range,
            from_date: from,
            through_date: today,
            as_of_utc: now,
            categories: empty_counts(),
            days: (0..length)
                .map(|day| DaySummary {
                    date: from + Duration::days(day),
                    categories: empty_counts(),
                })
                .collect(),
            recent: vec![],
            total_events: 0,
            recent_truncated: false,
        }
    }
}
