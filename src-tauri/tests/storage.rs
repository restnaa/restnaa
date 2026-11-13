use restnaa_lib::storage::{Storage, StorageError};
use rusqlite::{params, Connection};

#[test]
fn fresh_schema_is_empty_and_reopening_preserves_data() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.sqlite3");
    let storage = Storage::open(&path).unwrap();
    assert_eq!(storage.schema_version().unwrap(), 3);
    drop(storage);
    let connection = Connection::open(&path).unwrap();
    for table in ["settings", "reminder_state", "break_events"] {
        let count: u32 = connection
            .query_row(&format!("SELECT count(*) FROM {table}"), [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }
    let indexes: u32 = connection.query_row("SELECT count(*) FROM sqlite_master WHERE type='index' AND name IN ('break_events_created_at', 'break_events_category_created_at')", [], |r| r.get(0)).unwrap();
    assert_eq!(indexes, 2);
    connection
        .execute(
            "INSERT INTO settings VALUES ('fixture', 'true', '2026-10-20T00:00:00Z')",
            [],
        )
        .unwrap();
    drop(connection);
    drop(Storage::open(&path).unwrap());
    let connection = Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row(
                "SELECT value_json FROM settings WHERE key='fixture'",
                [],
                |r| r.get::<_, String>(0)
            )
            .unwrap(),
        "true"
    );
}

#[test]
fn categories_outcomes_and_duration_are_constrained() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("test.sqlite3");
    drop(Storage::open(&path).unwrap());
    let connection = Connection::open(&path).unwrap();
    for (category, outcome, duration, valid) in [
        ("eye", "completed", Some(20), true),
        ("movement", "skipped", None, true),
        ("eye", "snoozed", None, true),
        ("movement", "unanswered", None, true),
        ("unknown", "completed", None, false),
        ("eye", "failed", None, false),
        ("eye", "completed", Some(-1), false),
    ] {
        let result = connection.execute("INSERT INTO break_events VALUES (?1, ?2, ?3, '2026-10-20T00:00:00Z', '2026-10-20T00:00:20Z', ?4, '2026-10-20T00:00:20Z')", params![format!("{category}-{outcome}-{duration:?}"), category, outcome, duration]);
        assert_eq!(result.is_ok(), valid);
    }
    assert!(connection
        .execute(
            "INSERT INTO reminder_state VALUES ('unknown', 'now', NULL, 'now')",
            []
        )
        .is_err());
}

#[test]
fn version_two_upgrade_preserves_rows_and_indexes_history_transactionally() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("version-two.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute_batch(include_str!("../migrations/0001_initial.sql"))
        .unwrap();
    connection
        .execute_batch(include_str!("../migrations/0002_scheduler.sql"))
        .unwrap();
    connection.execute_batch("PRAGMA user_version=2; INSERT INTO settings VALUES('fixture','true','2026-10-21T09:00:00Z'); INSERT INTO break_events VALUES('old','eye','completed','2026-10-20T09:00:00Z','2026-10-20T09:00:20Z',20,'2026-10-20T09:00:20Z');").unwrap();
    drop(connection);
    let storage = Storage::open(&path).unwrap();
    assert_eq!(storage.schema_version().unwrap(), 3);
    drop(storage);
    let connection = Connection::open(&path).unwrap();
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
    assert_eq!(
        connection
            .query_row(
                "SELECT acted_at_utc FROM break_events WHERE id='old'",
                [],
                |row| row.get::<_, String>(0)
            )
            .unwrap(),
        "2026-10-20T09:00:20Z"
    );
    assert_eq!(connection.query_row("SELECT count(*) FROM sqlite_master WHERE type='index' AND name IN ('break_events_acted_instant','break_events_invalid_acted')", [], |row| row.get::<_, u32>(0)).unwrap(), 2);
}

#[test]
fn failed_history_index_upgrade_rolls_back_indexes_and_version() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("failed-index-upgrade.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute_batch(include_str!("../migrations/0001_initial.sql"))
        .unwrap();
    connection
        .execute_batch(include_str!("../migrations/0002_scheduler.sql"))
        .unwrap();
    connection.execute_batch("PRAGMA user_version=2; CREATE INDEX break_events_invalid_acted ON break_events(category); INSERT INTO settings VALUES('preserve','true','2026-10-21T09:00:00Z');").unwrap();
    drop(connection);
    assert!(Storage::open(&path).is_err());
    let connection = Connection::open(&path).unwrap();
    assert_eq!(
        connection
            .query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        2
    );
    assert_eq!(
        connection
            .query_row(
                "SELECT count(*) FROM sqlite_master WHERE name='break_events_acted_instant'",
                [],
                |row| row.get::<_, u32>(0)
            )
            .unwrap(),
        0
    );
    assert_eq!(
        connection
            .query_row(
                "SELECT value_json FROM settings WHERE key='preserve'",
                [],
                |row| row.get::<_, String>(0)
            )
            .unwrap(),
        "true"
    );
}

#[test]
fn corrupt_database_is_not_replaced() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("corrupt.sqlite3");
    let original = b"This is not a database. Preserve this file.";
    std::fs::write(&path, original).unwrap();
    assert!(Storage::open(&path).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), original);
}

#[test]
fn newer_and_unrecognized_databases_are_not_migrated() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("newer.sqlite3");
    let connection = Connection::open(&path).unwrap();
    connection
        .execute_batch("CREATE TABLE unfamiliar(id TEXT); PRAGMA user_version=4;")
        .unwrap();
    drop(connection);
    let original = std::fs::read(&path).unwrap();
    assert!(matches!(
        Storage::open(&path),
        Err(StorageError::NewerSchema)
    ));
    assert_eq!(std::fs::read(&path).unwrap(), original);
    let connection = Connection::open(&path).unwrap();
    connection.pragma_update(None, "user_version", 0).unwrap();
    drop(connection);
    let original = std::fs::read(&path).unwrap();
    assert!(matches!(
        Storage::open(&path),
        Err(StorageError::IncompleteSchema)
    ));
    assert_eq!(std::fs::read(&path).unwrap(), original);
}
