mod support;
use restnaa_lib::{
    clock::Clock,
    history::HistoryRange,
    scheduler::{model::*, service::Session},
    storage::Storage,
};
use rusqlite::{params, Connection};
use support::FakeClock;

fn insert(path: &std::path::Path, category: &str, outcome: &str, at: &str) {
    Connection::open(path)
        .unwrap()
        .execute(
            "INSERT INTO break_events VALUES(?1,?2,?3,?4,?4,NULL,?4)",
            params![uuid::Uuid::new_v4().to_string(), category, outcome, at],
        )
        .unwrap();
}

#[test]
fn invalid_history_timestamps_fail_visibly_instead_of_reporting_incomplete_counts() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bad-history.db");
    let storage = Storage::open(&path).unwrap();
    insert(&path, "eye", "completed", "not-a-timestamp");
    assert!(storage
        .history(HistoryRange::Today, &FakeClock::at("2026-10-21T12:00:00Z"))
        .is_err());
    assert_eq!(
        Connection::open(path)
            .unwrap()
            .query_row("SELECT count(*) FROM break_events", [], |row| row
                .get::<_, u32>(0))
            .unwrap(),
        1
    );
}

#[test]
fn local_today_includes_midnight_and_offsets_but_not_future_or_previous_day() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("history.db");
    let clock = FakeClock::at("2026-10-21T12:00:00Z");
    clock.zone(chrono_tz::Africa::Lagos);
    let storage = Storage::open(&path).unwrap();
    insert(&path, "eye", "completed", "2026-10-20T23:00:00Z");
    insert(&path, "movement", "skipped", "2026-10-21T01:00:00+01:00");
    insert(&path, "eye", "snoozed", "2026-10-21T11:00:00.500Z");
    insert(&path, "movement", "unanswered", "2026-10-21T12:00:00Z");
    insert(&path, "eye", "completed", "2026-10-20T22:59:59Z");
    insert(&path, "eye", "completed", "2026-10-21T12:00:00.000001Z");
    let history = storage.history(HistoryRange::Today, &clock).unwrap();
    assert_eq!(history.from_date.to_string(), "2026-10-21");
    assert_eq!(history.total_events, 4);
    assert_eq!(history.categories[0].counts.completed, 1);
    assert_eq!(history.categories[0].counts.snoozed, 1);
    assert_eq!(history.categories[1].counts.skipped, 1);
    assert_eq!(history.categories[1].counts.unanswered, 1);
    assert_eq!(history.recent[0].outcome, Outcome::Unanswered);
}

#[test]
fn seven_local_dates_and_all_counts_are_independent_of_recent_limit() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("history.db");
    let storage = Storage::open(&path).unwrap();
    let clock = FakeClock::at("2026-10-21T12:00:00Z");
    for _ in 0..105 {
        insert(&path, "eye", "completed", "2026-10-21T10:00:00Z");
    }
    insert(&path, "movement", "snoozed", "2026-10-15T00:00:00Z");
    insert(&path, "movement", "skipped", "2026-10-14T23:59:59Z");
    let history = storage.history(HistoryRange::SevenDays, &clock).unwrap();
    assert_eq!(history.days.len(), 7);
    assert_eq!(history.total_events, 106);
    assert_eq!(history.recent.len(), 100);
    assert!(history.recent_truncated);
    assert_eq!(history.categories[0].counts.completed, 105);
    assert_eq!(history.days[0].categories[1].counts.snoozed, 1);
}

#[test]
fn dst_fold_counts_both_occurrences_and_timezone_change_regroups() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("history.db");
    let storage = Storage::open(&path).unwrap();
    let clock = FakeClock::at("2026-11-01T22:00:00Z");
    clock.zone(chrono_tz::America::New_York);
    insert(&path, "eye", "completed", "2026-11-01T05:30:00Z");
    insert(&path, "eye", "completed", "2026-11-01T06:30:00Z");
    insert(&path, "movement", "skipped", "2026-11-01T02:00:00Z");
    assert_eq!(
        storage
            .history(HistoryRange::Today, &clock)
            .unwrap()
            .total_events,
        2
    );
    clock.zone(chrono_tz::UTC);
    assert_eq!(
        storage
            .history(HistoryRange::Today, &clock)
            .unwrap()
            .total_events,
        3
    );
    // Spring-forward day starts at 05:00 UTC and has only 23 hours.
    let spring = FakeClock::at("2026-03-09T03:59:59Z");
    spring.zone(chrono_tz::America::New_York);
    insert(&path, "eye", "snoozed", "2026-03-08T04:59:59Z");
    insert(&path, "movement", "unanswered", "2026-03-08T05:00:00Z");
    assert_eq!(
        storage
            .history(HistoryRange::Today, &spring)
            .unwrap()
            .total_events,
        1
    );
}

#[test]
fn confirmation_required_and_clear_preserves_open_reminder_settings_and_deadlines() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("history.db");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    session.apply(Action::Start(Category::Eye)).unwrap();
    insert(&path, "movement", "skipped", "2026-10-21T08:00:00Z");
    let before = session.engine.data.clone();
    assert!(session.clear_history(false).is_err());
    assert_eq!(
        session.history(HistoryRange::Today).unwrap().total_events,
        1
    );
    session.clear_history(true).unwrap();
    assert_eq!(session.engine.data, before);
    assert_eq!(
        session.history(HistoryRange::Today).unwrap().total_events,
        0
    );
    // Clearing does not suppress later explicit outcomes or mutate their category.
    session
        .apply(Action::Skip(before.runtime.open.unwrap().id))
        .unwrap();
    assert_eq!(
        session.history(HistoryRange::Today).unwrap().categories[0]
            .counts
            .skipped,
        1
    );
    assert_eq!(clock.sample().utc.to_rfc3339(), "2026-10-21T09:00:00+00:00");
}

#[test]
fn failed_delete_rolls_back_without_erasing_rows_or_changing_scheduler() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("history.db");
    let (mut session, _) = Session::open(
        Storage::open(&path).unwrap(),
        FakeClock::at("2026-10-21T09:00:00Z"),
    )
    .unwrap();
    insert(&path, "eye", "snoozed", "2026-10-21T08:00:00Z");
    Connection::open(&path).unwrap().execute_batch("CREATE TRIGGER block_delete BEFORE DELETE ON break_events BEGIN SELECT RAISE(ABORT,'test refusal'); END;").unwrap();
    let before = session.engine.data.clone();
    assert!(session.clear_history(true).is_err());
    assert_eq!(
        session.history(HistoryRange::Today).unwrap().total_events,
        1
    );
    assert_eq!(session.engine.data, before);
}

#[test]
fn old_settings_default_to_incomplete_and_onboarding_completion_persists() {
    use restnaa_lib::settings::Settings;
    let mut old = serde_json::to_value(Settings::default()).unwrap();
    old.as_object_mut().unwrap().remove("onboardingCompleted");
    assert!(
        !serde_json::from_value::<Settings>(old)
            .unwrap()
            .onboarding_completed
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("onboarding.db");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    let mut settings = session.snapshot().settings;
    settings.onboarding_completed = true;
    session.apply(Action::UpdateSettings(settings)).unwrap();
    drop(session);
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    assert!(session.snapshot().settings.onboarding_completed);
    let cap: serde_json::Value =
        serde_json::from_str(include_str!("../capabilities/break.json")).unwrap();
    assert!(!cap["permissions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|p| p == "allow-local-history"));
}
