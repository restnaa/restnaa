mod support;
use chrono::{Duration, NaiveTime};
use restnaa_lib::{
    clock::Clock,
    scheduler::{engine::Engine, model::*},
    settings::Settings,
};
use support::*;

fn morning() -> FakeClock {
    FakeClock::at("2026-10-21T09:00:00Z")
}

#[test]
fn forward_clock_jump_does_not_bypass_existing_monotonic_delivery_spacing() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    clock.jump(7200);
    let effects = engine.step(Action::Tick, &clock).unwrap();
    assert!(
        effects.due.is_empty(),
        "clock adjustment must not open movement immediately after eye"
    );
    assert_eq!(effects.recorded.len(), 1);
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert!(clock.pump(&mut engine, 119).due.is_empty());
    assert_eq!(clock.pump(&mut engine, 1).due.len(), 1);
}

#[test]
fn restart_after_backwards_clock_adjustment_bounds_future_deadlines_and_spacing() {
    let clock = FakeClock::at("2026-10-21T13:00:00Z");
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let id = engine.data.runtime.open.as_ref().unwrap().id;
    engine.step(Action::Complete(id), &clock).unwrap();
    clock.jump(-7200);
    let (restarted, effects) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert!(effects.due.is_empty());
    for category in Category::ALL {
        let latest = restarted.data.settings.next_work(
            clock.sample().utc + Duration::minutes(category.interval(&restarted.data.settings)),
            &clock,
        );
        assert!(
            restarted.data.reminders[category.index()].next_due_at_utc <= latest,
            "restart must not retain an extra two-hour wall-clock delay"
        );
    }
    assert!(
        restarted.data.runtime.next_allowed_at_utc.unwrap()
            <= clock.sample().utc + Duration::seconds(120)
    );
}

#[test]
fn independent_categories_complete_skip_and_duplicate_actions() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
    assert_eq!(
        engine.data.reminders[1].next_due_at_utc,
        clock.sample().utc + Duration::minutes(45)
    );
    let effects = clock.pump(&mut engine, 1200);
    assert_eq!(effects.due.len(), 1);
    assert_eq!(effects.due[0].category, Category::Eye);
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    clock.advance(20);
    let id = engine.data.runtime.open.as_ref().unwrap().id;
    let movement = engine.data.reminders[1].next_due_at_utc;
    let event = engine
        .step(Action::Complete(id), &clock)
        .unwrap()
        .recorded
        .remove(0);
    assert_eq!(event.outcome, Outcome::Completed);
    assert_eq!(event.duration_seconds, Some(20));
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
    assert_eq!(engine.data.reminders[1].next_due_at_utc, movement);
    assert!(engine.step(Action::Complete(id), &clock).is_err());
    // The second category is independently actionable after the spacing guard.
    clock.pump(&mut engine, 100);
    engine
        .step(Action::Start(Category::Movement), &clock)
        .unwrap();
    let id = engine.data.runtime.open.as_ref().unwrap().id;
    let result = engine.step(Action::Skip(id), &clock).unwrap();
    assert_eq!(result.recorded[0].outcome, Outcome::Skipped);
    assert_eq!(
        engine.data.reminders[1].next_due_at_utc,
        clock.sample().utc + Duration::minutes(45)
    );
}

#[test]
fn simultaneous_due_reminders_never_overlap_and_are_two_minutes_apart() {
    let clock = morning();
    let settings = Settings {
        movement_interval_minutes: 20,
        ..Settings::default()
    };
    let mut engine = configured(&clock, settings);
    let first = clock.pump(&mut engine, 1200).due.remove(0);
    assert_eq!(first.category, Category::Eye);
    assert!(engine
        .step(Action::Start(Category::Movement), &clock)
        .is_err());
    engine.step(Action::Complete(first.id), &clock).unwrap();
    assert!(clock.pump(&mut engine, 119).due.is_empty());
    let second = clock.pump(&mut engine, 1).due.remove(0);
    assert_eq!(second.category, Category::Movement);
    assert_eq!(
        (second.triggered_at_utc - first.triggered_at_utc).num_seconds(),
        120
    );
    for _ in 0..50 {
        assert!(engine.step(Action::Tick, &clock).unwrap().due.is_empty());
    }
}

#[test]
fn unanswered_is_recorded_once_and_snooze_is_not_completion() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    let first = clock.pump(&mut engine, 1200).due.remove(0);
    let effects = engine.step(Action::Snooze(first.id), &clock).unwrap();
    assert_eq!(effects.recorded[0].outcome, Outcome::Snoozed);
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(5)
    );
    let second = clock.pump(&mut engine, 300).due.remove(0);
    let effects = clock.pump(&mut engine, 300);
    assert_eq!(effects.recorded.len(), 1);
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
    assert!(engine.step(Action::Complete(second.id), &clock).is_err());
    assert!(engine
        .step(Action::Tick, &clock)
        .unwrap()
        .recorded
        .is_empty());
}

#[test]
fn snooze_outside_work_moves_to_next_selected_day_and_end_is_exclusive() {
    let clock = FakeClock::at("2026-10-23T17:57:00Z");
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let id = engine.data.runtime.open.as_ref().unwrap().id;
    engine.step(Action::Snooze(id), &clock).unwrap();
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc.to_rfc3339(),
        "2026-10-26T09:00:00+00:00"
    );
    clock.pump(&mut engine, 180);
    assert_eq!(engine.snapshot(&clock).status, RunStatus::OutsideWorkHours);
    assert!(engine
        .step(Action::Start(Category::Movement), &clock)
        .is_err());
}

#[test]
fn quiet_mode_and_disabled_categories_suppress_delivery() {
    let clock = morning();
    let mut settings = Settings {
        quiet_mode: true,
        ..Settings::default()
    };
    let mut engine = configured(&clock, settings.clone());
    assert!(clock.pump(&mut engine, 7200).due.is_empty());
    assert_eq!(engine.snapshot(&clock).status, RunStatus::Quiet);
    assert!(engine.step(Action::Start(Category::Eye), &clock).is_err());
    settings.quiet_mode = false;
    settings.eye_enabled = false;
    engine
        .step(Action::UpdateSettings(settings.clone()), &clock)
        .unwrap();
    let effects = clock.pump(&mut engine, 2700);
    assert_eq!(effects.due.len(), 1);
    assert_eq!(effects.due[0].category, Category::Movement);
    settings.movement_enabled = false;
    let effects = engine
        .step(Action::UpdateSettings(settings), &clock)
        .unwrap();
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert_eq!(engine.snapshot(&clock).status, RunStatus::Disabled);
}

#[test]
fn timed_manual_and_tomorrow_pause_resume_without_backlog() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    engine
        .step(Action::Pause(PauseRequest::Minutes { minutes: 30 }), &clock)
        .unwrap();
    assert_eq!(engine.snapshot(&clock).status, RunStatus::Paused);
    assert!(clock.pump(&mut engine, 1800).due.is_empty());
    assert_eq!(engine.snapshot(&clock).status, RunStatus::Active);
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
    engine
        .step(Action::Pause(PauseRequest::Manual {}), &clock)
        .unwrap();
    assert!(clock.pump(&mut engine, 3600).due.is_empty());
    engine.step(Action::Resume, &clock).unwrap();
    assert!(engine
        .data
        .reminders
        .iter()
        .all(|reminder| reminder.next_due_at_utc > clock.sample().utc));
    engine
        .step(Action::Pause(PauseRequest::UntilTomorrow {}), &clock)
        .unwrap();
    assert_eq!(
        engine.data.settings.paused_until_utc.unwrap().to_rfc3339(),
        "2026-10-22T00:00:00+00:00"
    );
    assert!(engine
        .step(Action::Pause(PauseRequest::Minutes { minutes: 7 }), &clock)
        .is_err());
}

#[test]
fn settings_changes_validate_reset_deadlines_and_dismiss_open_as_unanswered() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let mut settings = engine.data.settings.clone();
    settings.eye_interval_minutes = 30;
    let effects = engine
        .step(Action::UpdateSettings(settings.clone()), &clock)
        .unwrap();
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert!(effects.settings_changed);
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(30)
    );
    let original = engine.data.clone();
    for invalid in [0, 9, 121] {
        let mut bad = settings.clone();
        bad.eye_interval_minutes = invalid;
        assert!(engine.step(Action::UpdateSettings(bad), &clock).is_err());
        assert_eq!(engine.data, original);
    }
    let mut bad = settings.clone();
    bad.work_end_local = bad.work_start_local;
    assert!(bad.validate().is_err());
    bad = settings.clone();
    bad.work_days = vec![2, 2];
    assert!(bad.validate().is_err());
    bad = settings.clone();
    bad.work_days = vec![];
    assert!(bad.validate().is_err());
    bad = settings;
    bad.movement_interval_minutes = 181;
    assert!(bad.validate().is_err());
}

#[test]
fn restart_preserves_future_deadlines_and_manual_pause() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    clock.pump(&mut engine, 600);
    let due = engine.data.reminders.clone();
    let (mut restarted, effects) = Engine::restore(Some(engine.data.clone()), &clock).unwrap();
    assert_eq!(restarted.data.reminders, due);
    assert!(effects.due.is_empty());
    restarted
        .step(Action::Pause(PauseRequest::Manual {}), &clock)
        .unwrap();
    clock.advance(7200);
    let (restarted, effects) = Engine::restore(Some(restarted.data), &clock).unwrap();
    assert!(restarted.data.settings.paused_indefinitely);
    assert!(effects.due.is_empty());
}

#[test]
fn restart_and_sleep_never_replay_backlog_with_either_monotonic_behavior() {
    for suspended_monotonic in [false, true] {
        let clock = morning();
        let mut engine = configured(&clock, Settings::default());
        if suspended_monotonic {
            clock.jump(7200);
        } else {
            clock.advance(7200);
        }
        let effects = engine.step(Action::Tick, &clock).unwrap();
        assert_eq!(effects.due.len(), 1);
        assert_eq!(effects.due[0].category, Category::Eye);
        assert_eq!(
            engine.data.reminders[1].next_due_at_utc,
            clock.sample().utc + Duration::minutes(45)
        );
        for _ in 0..100 {
            assert!(engine.step(Action::Tick, &clock).unwrap().due.is_empty());
        }
    }
    let clock = morning();
    let engine = configured(&clock, Settings::default());
    clock.advance(7200);
    let (mut restarted, effects) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert_eq!(effects.due.len(), 1);
    assert!(restarted.step(Action::Tick, &clock).unwrap().due.is_empty());
}

#[test]
fn recovered_open_reminder_is_unanswered_once_and_not_reissued() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let old = engine.data.runtime.open.clone().unwrap();
    clock.advance(3600);
    let (restarted, effects) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert_eq!(effects.recorded.len(), 1);
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert!(effects
        .due
        .iter()
        .all(|reminder| reminder.id != old.id && reminder.category != Category::Eye));
    let (_, effects) = Engine::restore(Some(restarted.data), &clock).unwrap();
    // A second recovery may dismiss the newly-open movement reminder, but
    // never records the original eye reminder for a second time.
    assert!(effects
        .recorded
        .iter()
        .all(|event| event.category != Category::Eye));
}

#[test]
fn clock_forward_backward_and_timezone_changes_recompute_predictably() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    clock.jump(7200);
    let effects = engine.step(Action::Tick, &clock).unwrap();
    assert_eq!(effects.due.len(), 1);
    clock.jump(-7200);
    let effects = engine.step(Action::Tick, &clock).unwrap();
    assert!(effects.due.is_empty());
    assert_eq!(effects.recorded[0].outcome, Outcome::Unanswered);
    assert_eq!(
        engine.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
    clock.zone(chrono_tz::Asia::Tokyo);
    let effects = engine.step(Action::Tick, &clock).unwrap();
    assert!(effects.due.is_empty());
    assert_eq!(engine.snapshot(&clock).status, RunStatus::OutsideWorkHours);
    assert_eq!(
        clock
            .local_at(engine.data.reminders[0].next_due_at_utc)
            .time(),
        NaiveTime::from_hms_opt(9, 0, 0).unwrap()
    );
}

#[test]
fn dst_gaps_and_folds_resolve_work_boundaries_without_panics_or_storms() {
    let clock = FakeClock::at("2026-03-08T06:55:00Z");
    clock.zone(chrono_tz::America::New_York);
    let settings = Settings {
        work_days: vec![6],
        work_start_local: NaiveTime::from_hms_opt(2, 30, 0).unwrap(),
        work_end_local: NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
        ..Settings::default()
    };
    assert_eq!(
        settings.next_work(clock.sample().utc, &clock).to_rfc3339(),
        "2026-03-08T07:00:00+00:00"
    );
    let mut engine = configured(&clock, settings);
    let effects = clock.pump(&mut engine, 300);
    assert!(effects.due.len() <= 1);
    let clock = FakeClock::at("2026-11-01T04:00:00Z");
    clock.zone(chrono_tz::America::New_York);
    let settings = Settings {
        work_days: vec![6],
        work_start_local: NaiveTime::from_hms_opt(1, 30, 0).unwrap(),
        work_end_local: NaiveTime::from_hms_opt(3, 0, 0).unwrap(),
        ..Settings::default()
    };
    assert_eq!(
        settings.next_work(clock.sample().utc, &clock).to_rfc3339(),
        "2026-11-01T05:30:00+00:00"
    );
}

#[test]
fn wake_outside_work_and_pause_command_on_wake_do_not_deliver() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    clock.advance(10 * 3600);
    let effects = engine.step(Action::Tick, &clock).unwrap();
    assert!(effects.due.is_empty());
    assert!(engine
        .data
        .reminders
        .iter()
        .all(|state| state.next_due_at_utc > clock.sample().utc));
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    clock.advance(3600);
    let effects = engine
        .step(Action::Pause(PauseRequest::Manual {}), &clock)
        .unwrap();
    assert!(effects.due.is_empty());
}

#[test]
fn wait_is_bounded_and_targets_timeout_spacing_and_work_boundaries() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    assert_eq!(engine.next_wait(&clock).as_secs(), 30);
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    clock.pump(&mut engine, 295);
    assert_eq!(engine.next_wait(&clock).as_secs(), 5);
}

#[test]
fn restart_after_timed_pause_expired_does_not_replay_suppressed_reminders() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    engine
        .step(Action::Pause(PauseRequest::Minutes { minutes: 30 }), &clock)
        .unwrap();
    clock.advance(3600);
    let (restarted, effects) = Engine::restore(Some(engine.data), &clock).unwrap();
    assert!(effects.due.is_empty());
    assert!(effects.settings_changed);
    assert_eq!(restarted.snapshot(&clock).status, RunStatus::Active);
    assert_eq!(
        restarted.data.reminders[0].next_due_at_utc,
        clock.sample().utc + Duration::minutes(20)
    );
}

#[test]
fn work_start_can_use_second_fold_occurrence_when_first_has_passed() {
    let clock = FakeClock::at("2026-11-01T06:20:00Z");
    clock.zone(chrono_tz::America::New_York);
    let settings = Settings {
        work_days: vec![6],
        work_start_local: NaiveTime::from_hms_opt(1, 30, 0).unwrap(),
        work_end_local: NaiveTime::from_hms_opt(3, 0, 0).unwrap(),
        ..Settings::default()
    };
    assert_eq!(
        settings.next_work(clock.sample().utc, &clock).to_rfc3339(),
        "2026-11-01T06:30:00+00:00"
    );
}

#[test]
fn snapshot_exposes_independent_category_status_without_duplicate_persisted_flags() {
    let clock = morning();
    let mut engine = configured(&clock, Settings::default());
    assert!(engine
        .snapshot(&clock)
        .reminders
        .iter()
        .all(|reminder| reminder.status == ReminderStatus::Scheduled));
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let view = engine.snapshot(&clock);
    assert_eq!(view.reminders[0].status, ReminderStatus::Open);
    assert_eq!(view.reminders[1].status, ReminderStatus::Scheduled);
    let mut settings = engine.data.settings.clone();
    settings.eye_enabled = false;
    engine
        .step(Action::UpdateSettings(settings), &clock)
        .unwrap();
    engine
        .step(Action::Pause(PauseRequest::Manual {}), &clock)
        .unwrap();
    let view = engine.snapshot(&clock);
    assert_eq!(view.reminders[0].status, ReminderStatus::Disabled);
    assert_eq!(view.reminders[1].status, ReminderStatus::Paused);
}
