mod support;
use restnaa_lib::{
    clock::Clock,
    scheduler::{
        model::*,
        service::{Session, STORAGE_FAILURE},
    },
    storage::Storage,
};
use rusqlite::Connection;
use support::*;

#[test]
fn m1_upgrade_preserves_settings_and_history() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("upgrade.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute_batch(include_str!("../migrations/0001_initial.sql"))
        .unwrap();
    connection.execute_batch("PRAGMA user_version=1; INSERT INTO settings VALUES('fixture','true','2026-10-21T09:00:00Z'); INSERT INTO break_events VALUES('old','eye','completed','2026-10-20T09:00:00Z','2026-10-20T09:00:20Z',20,'2026-10-20T09:00:20Z');").unwrap();
    drop(connection);
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    assert_eq!(session.snapshot().reminders.len(), 3);
    drop(session);
    let connection = Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        4
    );
    assert_eq!(
        connection
            .query_row(
                "SELECT count(*) FROM break_events WHERE id='old'",
                [],
                |row| row.get::<_, u32>(0)
            )
            .unwrap(),
        1
    );
    assert_eq!(
        connection
            .query_row(
                "SELECT value_json FROM settings WHERE key='fixture'",
                [],
                |row| row.get::<_, String>(0)
            )
            .unwrap(),
        "true"
    );
}

#[test]
fn open_state_pause_and_outcomes_round_trip_atomically() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("scheduler.sqlite3");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    session.apply(Action::Start(Category::Eye)).unwrap();
    let id = session.snapshot().open_reminder.unwrap().id;
    session.apply(Action::Snooze(id)).unwrap();
    let deadline = session.snapshot().reminders[0].next_due_at_utc;
    session
        .apply(Action::Pause(PauseRequest::Manual {}))
        .unwrap();
    let paused_deadline = session.snapshot().reminders[0].next_due_at_utc;
    assert!(paused_deadline >= deadline);
    drop(session);
    clock.advance(3600);
    let (mut session, effects) =
        Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    assert!(effects.due.is_empty());
    assert!(session.snapshot().settings.paused_indefinitely);
    session.apply(Action::Resume).unwrap();
    session
        .apply(Action::Pause(PauseRequest::Minutes { minutes: 60 }))
        .unwrap();
    let until = session.snapshot().settings.paused_until_utc;
    drop(session);
    let (session, effects) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    assert_eq!(session.snapshot().settings.paused_until_utc, until);
    assert!(effects.due.is_empty());
    drop(session);
    let connection = Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row(
                "SELECT count(*) FROM break_events WHERE outcome='snoozed'",
                [],
                |row| row.get::<_, u32>(0)
            )
            .unwrap(),
        1
    );
    assert_eq!(
        connection
            .query_row(
                "SELECT count(*) FROM break_events WHERE outcome='completed'",
                [],
                |row| row.get::<_, u32>(0)
            )
            .unwrap(),
        0
    );
}

#[test]
fn crash_recovery_records_unanswered_exactly_once_in_database() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("crash.sqlite3");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    session.apply(Action::Start(Category::Eye)).unwrap();
    drop(session);
    clock.advance(3600);
    let (session, effects) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    assert_eq!(effects.recorded.len(), 1);
    assert!(effects.due.len() <= 1);
    drop(session);
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    drop(session);
    let connection = Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row(
                "SELECT count(*) FROM break_events WHERE category='eye' AND outcome='unanswered'",
                [],
                |row| row.get::<_, u32>(0)
            )
            .unwrap(),
        1
    );
}

#[test]
fn failed_event_insert_rolls_back_all_state_and_stops_session() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("failure.sqlite3");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    session.apply(Action::Start(Category::Eye)).unwrap();
    let before = session.engine.data.clone();
    let id = session.snapshot().open_reminder.unwrap().id;
    let connection = Connection::open(&path).unwrap();
    connection.execute_batch("CREATE TRIGGER reject_event BEFORE INSERT ON break_events BEGIN SELECT RAISE(ABORT,'injected disk failure'); END;").unwrap();
    assert_eq!(
        session.apply(Action::Complete(id)).unwrap_err(),
        STORAGE_FAILURE
    );
    assert_eq!(session.engine.data, before);
    assert_eq!(
        Storage::open(&path)
            .unwrap()
            .load_scheduler(&clock)
            .unwrap()
            .unwrap(),
        before
    );
    assert_eq!(
        connection
            .query_row("SELECT count(*) FROM break_events", [], |row| row
                .get::<_, u32>(0))
            .unwrap(),
        0
    );
    assert_eq!(session.apply(Action::Tick).unwrap_err(), STORAGE_FAILURE);
    assert_eq!(session.apply(Action::Resume).unwrap_err(), STORAGE_FAILURE);
    assert_eq!(clock.sample().utc.to_rfc3339(), "2026-10-21T09:00:00+00:00");
}

#[test]
fn inconsistent_open_ids_and_invalid_settings_fail_without_reset() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("invalid.sqlite3");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    drop(session);
    let connection = Connection::open(&path).unwrap();
    connection.execute("UPDATE reminder_state SET open_reminder_id='00000000-0000-0000-0000-000000000001' WHERE category='eye'",[]).unwrap();
    assert!(Storage::open(&path)
        .unwrap()
        .load_scheduler(&clock)
        .is_err());
    assert!(Session::open(Storage::open(&path).unwrap(), clock.clone()).is_err());
    assert_eq!(
        connection
            .query_row(
                "SELECT open_reminder_id FROM reminder_state WHERE category='eye'",
                [],
                |row| row.get::<_, String>(0)
            )
            .unwrap(),
        "00000000-0000-0000-0000-000000000001"
    );
    connection
        .execute("UPDATE reminder_state SET open_reminder_id=NULL", [])
        .unwrap();
    connection
        .execute(
            "UPDATE settings SET value_json='{}' WHERE key='scheduler_settings'",
            [],
        )
        .unwrap();
    assert!(Session::open(Storage::open(&path).unwrap(), clock).is_err());
    assert_eq!(
        connection
            .query_row(
                "SELECT value_json FROM settings WHERE key='scheduler_settings'",
                [],
                |row| row.get::<_, String>(0)
            )
            .unwrap(),
        "{}"
    );
}
