use crate::{scheduler::model::*, settings::Settings};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use std::{path::Path, time::Duration};
use thiserror::Error;

pub const SCHEMA_VERSION: u32 = 2;
const INITIAL_MIGRATION: &str = include_str!("../../migrations/0001_initial.sql");
const SCHEDULER_MIGRATION: &str = include_str!("../../migrations/0002_scheduler.sql");

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite operation failed")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Database integrity check failed")]
    Integrity,
    #[error("Database schema is newer than this application")]
    NewerSchema,
    #[error("Database schema is incomplete")]
    IncompleteSchema,
    #[error("Persisted scheduler data is invalid or inconsistent")]
    InvalidData,
}

pub struct Storage {
    connection: Connection,
}

impl Storage {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        let mut connection = Connection::open(path)?;
        connection.busy_timeout(Duration::from_secs(5))?;
        let integrity: String = connection.query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        if integrity != "ok" {
            return Err(StorageError::Integrity);
        }
        apply_initial_migration(&mut connection, INITIAL_MIGRATION)?;
        // Fail visibly on missing tables/columns, rather than resetting data.
        connection.prepare("SELECT key, value_json, updated_at_utc FROM settings LIMIT 0")?;
        connection.prepare("SELECT category, next_due_at_utc, open_reminder_id, updated_at_utc FROM reminder_state LIMIT 0")?;
        connection.prepare("SELECT id, category, outcome, triggered_at_utc, acted_at_utc, duration_seconds, created_at_utc FROM break_events LIMIT 0")?;
        Ok(Self { connection })
    }

    pub fn schema_version(&self) -> Result<u32, StorageError> {
        Ok(self
            .connection
            .query_row("PRAGMA user_version", [], |r| r.get(0))?)
    }

    pub fn history(
        &self,
        range: crate::history::HistoryRange,
        clock: &dyn crate::clock::Clock,
    ) -> Result<crate::history::History, StorageError> {
        use crate::history::{History, RECENT_LIMIT};
        let invalid_timestamp: bool = self.connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM break_events WHERE julianday(acted_at_utc) IS NULL)",
            [],
            |row| row.get(0),
        )?;
        if invalid_timestamp {
            return Err(StorageError::InvalidData);
        }
        let mut history = History::empty(range, clock);
        let start = crate::settings::resolve_boundary(
            history.from_date.and_hms_opt(0, 0, 0).unwrap(),
            clock,
        );
        // Use instant comparison rather than string comparison: RFC3339 offsets
        // and fractional precision may differ in older local records.
        let mut statement = self.connection.prepare("SELECT id,category,outcome,triggered_at_utc,acted_at_utc,duration_seconds,created_at_utc FROM break_events WHERE julianday(acted_at_utc)>=julianday(?1) AND julianday(acted_at_utc)<=julianday(?2) ORDER BY julianday(acted_at_utc) DESC,id DESC")?;
        let mut rows =
            statement.query(params![start.to_rfc3339(), history.as_of_utc.to_rfc3339()])?;
        while let Some(row) = rows.next()? {
            let parse_time = |column| -> Result<DateTime<Utc>, StorageError> {
                DateTime::parse_from_rfc3339(&row.get::<_, String>(column)?)
                    .map(|time| time.with_timezone(&Utc))
                    .map_err(|_| StorageError::InvalidData)
            };
            let acted = parse_time(4)?;
            let date = clock.local_at(acted).date_naive();
            let Some(day) = history.days.iter_mut().find(|day| day.date == date) else {
                continue;
            };
            // Explicit precision guard: SQLite's julianday rounds submilliseconds.
            if acted > history.as_of_utc {
                continue;
            }
            let category = match row.get::<_, String>(1)?.as_str() {
                "eye" => Category::Eye,
                "movement" => Category::Movement,
                _ => return Err(StorageError::InvalidData),
            };
            let outcome = match row.get::<_, String>(2)?.as_str() {
                "completed" => Outcome::Completed,
                "skipped" => Outcome::Skipped,
                "snoozed" => Outcome::Snoozed,
                "unanswered" => Outcome::Unanswered,
                _ => return Err(StorageError::InvalidData),
            };
            history.categories[category.index()].counts.record(outcome);
            day.categories[category.index()].counts.record(outcome);
            history.total_events += 1;
            if history.recent.len() < RECENT_LIMIT {
                history.recent.push(BreakEvent {
                    id: row
                        .get::<_, String>(0)?
                        .parse()
                        .map_err(|_| StorageError::InvalidData)?,
                    category,
                    outcome,
                    triggered_at_utc: parse_time(3)?,
                    acted_at_utc: acted,
                    duration_seconds: row.get(5)?,
                    created_at_utc: parse_time(6)?,
                });
            }
        }
        history.recent_truncated = history.total_events > history.recent.len() as u64;
        Ok(history)
    }

    pub fn clear_history(&mut self, confirmed: bool) -> Result<(), StorageError> {
        if !confirmed {
            return Err(StorageError::InvalidData);
        }
        let transaction = self.connection.transaction()?;
        transaction.execute("DELETE FROM break_events", [])?;
        transaction.commit()?;
        Ok(())
    }

    pub fn load_scheduler(&self) -> Result<Option<Persisted>, StorageError> {
        let json: Option<String> = self
            .connection
            .query_row(
                "SELECT value_json FROM settings WHERE key='scheduler_settings'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        let Some(json) = json else {
            let count: u32 = self.connection.query_row("SELECT (SELECT count(*) FROM reminder_state) + (SELECT count(*) FROM scheduler_runtime)", [], |row| row.get(0))?;
            return if count == 0 {
                Ok(None)
            } else {
                Err(StorageError::InvalidData)
            };
        };
        let settings: Settings =
            serde_json::from_str(&json).map_err(|_| StorageError::InvalidData)?;
        let json: String = self.connection.query_row(
            "SELECT value_json FROM scheduler_runtime WHERE id=1",
            [],
            |row| row.get(0),
        )?;
        let runtime: RuntimeMeta =
            serde_json::from_str(&json).map_err(|_| StorageError::InvalidData)?;
        let mut reminders = Vec::new();
        for category in Category::ALL {
            let (due, open): (String, Option<String>) = self.connection.query_row(
                "SELECT next_due_at_utc, open_reminder_id FROM reminder_state WHERE category=?1",
                [category.name()],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )?;
            reminders.push(ReminderState {
                category,
                next_due_at_utc: DateTime::parse_from_rfc3339(&due)
                    .map_err(|_| StorageError::InvalidData)?
                    .with_timezone(&Utc),
                open_reminder_id: open
                    .map(|id| id.parse())
                    .transpose()
                    .map_err(|_| StorageError::InvalidData)?,
            });
        }
        let data = Persisted {
            settings,
            reminders: reminders
                .try_into()
                .map_err(|_| StorageError::InvalidData)?,
            runtime,
        };
        data.validate().map_err(|_| StorageError::InvalidData)?;
        Ok(Some(data))
    }

    pub fn save_scheduler(
        &mut self,
        data: &Persisted,
        events: &[BreakEvent],
        now: DateTime<Utc>,
    ) -> Result<(), StorageError> {
        data.validate().map_err(|_| StorageError::InvalidData)?;
        let settings =
            serde_json::to_string(&data.settings).map_err(|_| StorageError::InvalidData)?;
        let runtime =
            serde_json::to_string(&data.runtime).map_err(|_| StorageError::InvalidData)?;
        let transaction = self.connection.transaction()?;
        transaction.execute("INSERT INTO settings(key,value_json,updated_at_utc) VALUES('scheduler_settings',?1,?2) ON CONFLICT(key) DO UPDATE SET value_json=excluded.value_json,updated_at_utc=excluded.updated_at_utc", params![settings,now.to_rfc3339()])?;
        for state in &data.reminders {
            transaction.execute("INSERT INTO reminder_state(category,next_due_at_utc,open_reminder_id,updated_at_utc) VALUES(?1,?2,?3,?4) ON CONFLICT(category) DO UPDATE SET next_due_at_utc=excluded.next_due_at_utc,open_reminder_id=excluded.open_reminder_id,updated_at_utc=excluded.updated_at_utc", params![state.category.name(),state.next_due_at_utc.to_rfc3339(),state.open_reminder_id.map(|id| id.to_string()),now.to_rfc3339()])?;
        }
        transaction.execute("INSERT INTO scheduler_runtime(id,value_json,updated_at_utc) VALUES(1,?1,?2) ON CONFLICT(id) DO UPDATE SET value_json=excluded.value_json,updated_at_utc=excluded.updated_at_utc", params![runtime,now.to_rfc3339()])?;
        for event in events {
            transaction.execute("INSERT INTO break_events(id,category,outcome,triggered_at_utc,acted_at_utc,duration_seconds,created_at_utc) VALUES(?1,?2,?3,?4,?5,?6,?7)", params![event.id.to_string(),event.category.name(),event.outcome.name(),event.triggered_at_utc.to_rfc3339(),event.acted_at_utc.to_rfc3339(),event.duration_seconds,event.created_at_utc.to_rfc3339()])?;
        }
        transaction.commit()?;
        Ok(())
    }
}

fn apply_initial_migration(connection: &mut Connection, sql: &str) -> Result<(), StorageError> {
    let transaction = connection.transaction()?;
    let version: u32 = transaction.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    match version {
        0 => {
            // A versionless database containing user tables is not a fresh DB.
            let tables: u32 = transaction.query_row("SELECT count(*) FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'", [], |r| r.get(0))?;
            if tables != 0 {
                return Err(StorageError::IncompleteSchema);
            }
            transaction.execute_batch(sql)?;
            transaction.execute_batch(SCHEDULER_MIGRATION)?;
        }
        1 => {
            transaction.execute_batch(SCHEDULER_MIGRATION)?;
        }
        SCHEMA_VERSION => (),
        _ => return Err(StorageError::NewerSchema),
    }
    transaction.pragma_update(None, "user_version", SCHEMA_VERSION)?;
    transaction.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_migration_rolls_back_schema_and_version() {
        let mut connection = Connection::open_in_memory().unwrap();
        assert!(apply_initial_migration(
            &mut connection,
            "CREATE TABLE temporary_data(id TEXT); INVALID SQL;"
        )
        .is_err());
        assert_eq!(
            connection
                .query_row("PRAGMA user_version", [], |r| r.get::<_, u32>(0))
                .unwrap(),
            0
        );
        assert_eq!(
            connection
                .query_row(
                    "SELECT count(*) FROM sqlite_master WHERE name = 'temporary_data'",
                    [],
                    |r| r.get::<_, u32>(0)
                )
                .unwrap(),
            0
        );
    }
}
