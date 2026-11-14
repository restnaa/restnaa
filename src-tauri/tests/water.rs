mod support;
use chrono::Duration;
use restnaa_lib::{
    clock::Clock,
    scheduler::{engine::Engine, model::*, service::Session},
    settings::Settings,
    storage::Storage,
};
use rusqlite::{params, Connection};
use support::{configured, FakeClock};

fn water_only() -> Settings {
    Settings {
        eye_enabled: false,
        movement_enabled: false,
        ..Settings::default()
    }
}

#[test]
fn water_defaults_and_interval_validation_are_explicit() {
    let settings = Settings::default();
    assert!(settings.water_enabled);
    assert_eq!(settings.water_interval_minutes, 60);
    let mut legacy = serde_json::to_value(&settings).unwrap();
    legacy.as_object_mut().unwrap().remove("waterEnabled");
    legacy
        .as_object_mut()
        .unwrap()
        .remove("waterIntervalMinutes");
    assert_eq!(
        serde_json::from_value::<Settings>(legacy).unwrap(),
        settings
    );
    for minutes in [0, 29, 241, u32::MAX] {
        assert!(Settings {
            water_interval_minutes: minutes,
            ..settings.clone()
        }
        .validate()
        .is_err());
    }
    for minutes in [30, 60, 240] {
        assert!(Settings {
            water_interval_minutes: minutes,
            ..settings.clone()
        }
        .validate()
        .is_ok());
    }
}

#[test]
fn water_outcomes_are_distinct_and_schedule_from_action_time() {
    for outcome in [
        Outcome::Completed,
        Outcome::Skipped,
        Outcome::Snoozed,
        Outcome::Unanswered,
    ] {
        let clock = FakeClock::at("2026-11-16T09:00:00Z");
        let mut engine = configured(&clock, water_only());
        let due = clock.pump(&mut engine, 3600);
        assert_eq!(due.due.len(), 1);
        assert_eq!(due.due[0].category, Category::Water);
        let id = due.due[0].id;
        let effects = match outcome {
            Outcome::Completed => engine.step(Action::Complete(id), &clock).unwrap(),
            Outcome::Skipped => engine.step(Action::Skip(id), &clock).unwrap(),
            Outcome::Snoozed => engine.step(Action::Snooze(id), &clock).unwrap(),
            Outcome::Unanswered => clock.pump(&mut engine, 300),
        };
        assert_eq!(effects.recorded.len(), 1);
        assert_eq!(effects.recorded[0].category, Category::Water);
        assert_eq!(effects.recorded[0].outcome, outcome);
        assert_eq!(
            engine.data.reminders[2].next_due_at_utc,
            clock.sample().utc
                + Duration::minutes(if outcome == Outcome::Snoozed { 5 } else { 60 })
        );
        assert!(engine.step(Action::Complete(id), &clock).is_err());
    }
}

#[test]
fn three_simultaneous_categories_are_sequenced_without_overlap() {
    let clock = FakeClock::at("2026-11-16T09:00:00Z");
    let mut engine = configured(
        &clock,
        Settings {
            eye_interval_minutes: 30,
            movement_interval_minutes: 30,
            water_interval_minutes: 30,
            ..Settings::default()
        },
    );
    for (index, category) in Category::ALL.into_iter().enumerate() {
        let effects = clock.pump(&mut engine, if index == 0 { 1800 } else { 120 });
        assert_eq!(effects.due.len(), 1);
        assert_eq!(effects.due[0].category, category);
        for other in Category::ALL.into_iter().filter(|other| *other != category) {
            assert!(engine.step(Action::Start(other), &clock).is_err());
        }
        engine
            .step(Action::Skip(effects.due[0].id), &clock)
            .unwrap();
        assert!(engine.data.runtime.open.is_none());
    }
}

#[test]
fn water_obeys_quiet_work_pause_and_disabled_controls() {
    for settings in [
        Settings {
            quiet_mode: true,
            ..water_only()
        },
        Settings {
            water_enabled: false,
            ..water_only()
        },
    ] {
        let clock = FakeClock::at("2026-11-16T09:00:00Z");
        let mut engine = configured(&clock, settings);
        assert!(clock.pump(&mut engine, 7200).due.is_empty());
        assert!(engine.step(Action::Start(Category::Water), &clock).is_err());
    }
    let paused_clock = FakeClock::at("2026-11-16T09:00:00Z");
    let mut paused = configured(&paused_clock, water_only());
    paused
        .step(Action::Pause(PauseRequest::Manual {}), &paused_clock)
        .unwrap();
    assert!(paused_clock.pump(&mut paused, 7200).due.is_empty());
    assert!(paused
        .step(Action::Start(Category::Water), &paused_clock)
        .is_err());
    let clock = FakeClock::at("2026-11-16T18:00:00Z");
    let mut engine = configured(&clock, water_only());
    assert!(clock.pump(&mut engine, 7200).due.is_empty());
    assert_eq!(engine.snapshot(&clock).status, RunStatus::OutsideWorkHours);
}

#[test]
fn water_clock_jump_restart_settings_and_pause_do_not_replay_backlogs() {
    let clock = FakeClock::at("2026-11-16T09:00:00Z");
    let mut engine = configured(&clock, water_only());
    clock.jump(7200);
    assert_eq!(engine.step(Action::Tick, &clock).unwrap().due.len(), 1);
    assert!(engine.step(Action::Tick, &clock).unwrap().due.is_empty());
    let (mut engine, effects) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert_eq!(effects.recorded.len(), 1);
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert!(effects.due.is_empty());
    let mut settings = engine.data.settings.clone();
    settings.water_interval_minutes = 90;
    engine
        .step(Action::UpdateSettings(settings), &clock)
        .unwrap();
    assert_eq!(
        engine.data.reminders[2].next_due_at_utc,
        clock.sample().utc + Duration::minutes(90)
    );
    engine
        .step(Action::Pause(PauseRequest::Manual {}), &clock)
        .unwrap();
    let (mut engine, _) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert_eq!(engine.snapshot(&clock).status, RunStatus::Paused);
    assert!(clock.pump(&mut engine, 3600).due.is_empty());
    engine.step(Action::Resume, &clock).unwrap();
    clock.jump(-1800);
    assert!(engine.step(Action::Tick, &clock).unwrap().due.len() <= 1);
    clock.advance(3600);
    assert!(engine.step(Action::Tick, &clock).unwrap().due.len() <= 1);
}

#[test]
fn water_outcomes_persist_in_their_own_history_category_after_restart() {
    for outcome in [
        Outcome::Completed,
        Outcome::Skipped,
        Outcome::Snoozed,
        Outcome::Unanswered,
    ] {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("water-history.db");
        let clock = FakeClock::at("2026-11-16T09:00:00Z");
        let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
        session.apply(Action::UpdateSettings(water_only())).unwrap();
        session.apply(Action::Start(Category::Water)).unwrap();
        let id = session.snapshot().open_reminder.unwrap().id;
        match outcome {
            Outcome::Completed => {
                session.apply(Action::Complete(id)).unwrap();
            }
            Outcome::Skipped => {
                session.apply(Action::Skip(id)).unwrap();
            }
            Outcome::Snoozed => {
                session.apply(Action::Snooze(id)).unwrap();
            }
            Outcome::Unanswered => {
                clock.advance(300);
                session.apply(Action::Tick).unwrap();
            }
        }
        drop(session);
        let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
        let history = session
            .history(restnaa_lib::history::HistoryRange::Today)
            .unwrap();
        assert_eq!(history.total_events, 1);
        assert_eq!(history.recent[0].category, Category::Water);
        assert_eq!(history.recent[0].outcome, outcome);
        assert_eq!(history.categories[0].counts.completed, 0);
        assert_eq!(history.categories[1].counts.completed, 0);
        let counts = &history.categories[2].counts;
        assert_eq!(
            counts.completed + counts.skipped + counts.snoozed + counts.unanswered,
            1
        );
    }
}

fn legacy_profile(path: &std::path::Path) {
    let db = Connection::open(path).unwrap();
    db.execute_batch(include_str!("../migrations/0001_initial.sql"))
        .unwrap();
    db.execute_batch(include_str!("../migrations/0002_scheduler.sql"))
        .unwrap();
    db.execute_batch(include_str!("../migrations/0003_history_indexes.sql"))
        .unwrap();
    db.execute_batch("PRAGMA user_version=3;").unwrap();
    let mut settings = serde_json::to_value(Settings::default()).unwrap();
    settings.as_object_mut().unwrap().remove("waterEnabled");
    settings
        .as_object_mut()
        .unwrap()
        .remove("waterIntervalMinutes");
    db.execute(
        "INSERT INTO settings VALUES('scheduler_settings',?1,'2026-11-16T09:00:00Z')",
        [settings.to_string()],
    )
    .unwrap();
    db.execute(
        "INSERT INTO scheduler_runtime VALUES(1,?1,'2026-11-16T09:00:00Z')",
        [serde_json::to_string(&RuntimeMeta {
            open: None,
            next_allowed_at_utc: None,
            revision: 7,
        })
        .unwrap()],
    )
    .unwrap();
    for (category, due) in [
        ("eye", "2026-11-16T09:20:00Z"),
        ("movement", "2026-11-16T09:45:00Z"),
    ] {
        db.execute(
            "INSERT INTO reminder_state VALUES(?1,?2,NULL,'2026-11-16T09:00:00Z')",
            params![category, due],
        )
        .unwrap();
    }
    db.execute("INSERT INTO break_events VALUES(?1,'eye','completed','2026-11-16T08:00:00Z','2026-11-16T08:00:20Z',20,'2026-11-16T08:00:20Z')", [uuid::Uuid::new_v4().to_string()]).unwrap();
}

#[test]
fn old_profiles_get_one_fresh_water_deadline_without_losing_existing_state() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("upgrade.db");
    legacy_profile(&path);
    let clock = FakeClock::at("2026-11-16T09:00:00Z");
    let (session, effects) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    assert!(effects.due.is_empty());
    assert_eq!(
        session.engine.data.reminders[0]
            .next_due_at_utc
            .to_rfc3339(),
        "2026-11-16T09:20:00+00:00"
    );
    assert_eq!(
        session.engine.data.reminders[1]
            .next_due_at_utc
            .to_rfc3339(),
        "2026-11-16T09:45:00+00:00"
    );
    assert_eq!(
        session.engine.data.reminders[2].next_due_at_utc,
        clock.sample().utc + Duration::minutes(60)
    );
    assert_eq!(
        session
            .history(restnaa_lib::history::HistoryRange::Today)
            .unwrap()
            .total_events,
        1
    );
    drop(session);
    let db = Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row(
            "SELECT count(*) FROM settings WHERE key='water_upgrade_pending'",
            [],
            |row| row.get::<_, u32>(0)
        )
        .unwrap(),
        0
    );
    // A later missing water row is corruption, not permission to reinitialize.
    db.execute("DELETE FROM reminder_state WHERE category='water'", [])
        .unwrap();
    assert!(Storage::open(&path)
        .unwrap()
        .load_scheduler(&clock)
        .is_err());
}

#[test]
fn failed_water_upgrade_rolls_back_tables_history_and_schema_version() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("rollback.db");
    legacy_profile(&path);
    let db = Connection::open(&path).unwrap();
    db.execute(
        "INSERT INTO settings VALUES('water_upgrade_pending','unexpected','2026-11-16T09:00:00Z')",
        [],
    )
    .unwrap();
    drop(db);
    assert!(Storage::open(&path).is_err());
    let db = Connection::open(&path).unwrap();
    assert_eq!(
        db.query_row("PRAGMA user_version", [], |row| row.get::<_, u32>(0))
            .unwrap(),
        3
    );
    assert_eq!(
        db.query_row("SELECT count(*) FROM break_events", [], |row| row
            .get::<_, u32>(0))
            .unwrap(),
        1
    );
    assert!(db.execute("INSERT INTO reminder_state VALUES('water','2026-11-16T10:00:00Z',NULL,'2026-11-16T09:00:00Z')", []).is_err());
}
