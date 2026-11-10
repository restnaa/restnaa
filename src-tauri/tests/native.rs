mod support;
use restna_lib::{
    native::{model::*, startup::change_with_rollback},
    scheduler::{engine::Engine, model::*},
    settings::Settings,
};
use support::*;

#[test]
fn main_close_without_a_tray_quits_all_hidden_webviews_instead_of_stranding_process() {
    use restna_lib::native::model::{close_behavior, CloseBehavior::*};
    assert_eq!(close_behavior("main", false, false), Quit);
    assert_eq!(close_behavior("main", true, false), Hide);
    assert_eq!(close_behavior("break", false, false), Hide);
    assert_eq!(close_behavior("main", true, true), Allow);
    assert_eq!(close_behavior("break", false, true), Allow);
}

#[test]
fn delivery_policy_respects_disabled_denied_and_attention_states_without_claiming_display() {
    assert_eq!(
        delivery_policy(false, WindowsPermission::Allowed, Attention::Accepting),
        Some(Delivery::DisabledInApp)
    );
    for permission in [
        WindowsPermission::DisabledForApplication,
        WindowsPermission::DisabledForUser,
        WindowsPermission::DisabledByPolicy,
        WindowsPermission::DisabledByManifest,
    ] {
        assert_eq!(
            delivery_policy(true, permission, Attention::Accepting),
            Some(Delivery::DeniedByWindows)
        );
    }
    for attention in [
        Attention::Away,
        Attention::Busy,
        Attention::Fullscreen,
        Attention::Presentation,
        Attention::QuietTime,
    ] {
        assert_eq!(
            delivery_policy(true, WindowsPermission::Allowed, attention),
            Some(Delivery::SuppressedByAttention)
        );
    }
    assert_eq!(
        delivery_policy(true, WindowsPermission::Allowed, Attention::Accepting),
        None
    );
    // Unknown Focus Assist state delegates delivery to Windows, never bypasses it.
    assert_eq!(
        delivery_policy(true, WindowsPermission::Unknown, Attention::Unknown),
        None
    );
}

#[test]
fn toast_click_only_matches_current_valid_uuid() {
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let mut engine = configured(&clock, Settings::default());
    engine.step(Action::Start(Category::Eye), &clock).unwrap();
    let open = engine.data.runtime.open.as_ref().unwrap();
    assert!(matches_current_click(
        Some(&open.id.to_string()),
        Some(open)
    ));
    assert!(!matches_current_click(
        Some(&uuid::Uuid::new_v4().to_string()),
        Some(open)
    ));
    assert!(!matches_current_click(Some("invalid"), Some(open)));
    assert!(!matches_current_click(None, Some(open)));
    assert!(!matches_current_click(Some(&open.id.to_string()), None));
}

#[test]
fn startup_change_rolls_back_on_persistence_failure_and_reports_failed_rollback() {
    let mut changes = vec![];
    let error = change_with_rollback(
        true,
        false,
        |enabled| {
            changes.push(enabled);
            Ok(())
        },
        || Err("disk full".into()),
    )
    .unwrap_err();
    assert_eq!(error, "disk full");
    assert_eq!(changes, vec![true, false]);
    let mut calls = 0;
    let error = change_with_rollback(
        true,
        false,
        |_| {
            calls += 1;
            if calls == 1 {
                Ok(())
            } else {
                Err("rollback failed".into())
            }
        },
        || Err("disk full".into()),
    )
    .unwrap_err();
    assert!(error.contains("restore Windows startup"));
}

#[test]
fn startup_registration_failure_never_persists_consent_and_noop_does_not_touch_os() {
    let mut persisted = false;
    assert!(change_with_rollback(
        true,
        false,
        |_| Err("OS denied".into()),
        || {
            persisted = true;
            Ok(())
        }
    )
    .is_err());
    assert!(!persisted);
    assert!(change_with_rollback(
        false,
        false,
        |_| panic!("must not mutate startup"),
        || Ok(())
    )
    .is_ok());
}

#[test]
fn m2_settings_default_to_notifications_on_and_autostart_off_without_resetting_deadlines() {
    let mut value = serde_json::to_value(Settings::default()).unwrap();
    value.as_object_mut().unwrap().remove("notificationEnabled");
    value.as_object_mut().unwrap().remove("autostartEnabled");
    let settings: Settings = serde_json::from_value(value).unwrap();
    assert!(settings.notification_enabled);
    assert!(!settings.autostart_enabled);
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut engine, _) = Engine::restore(None, &clock).unwrap();
    let deadlines = engine.data.reminders.clone();
    let mut bypass = engine.data.settings.clone();
    bypass.autostart_enabled = true;
    assert!(engine.step(Action::UpdateSettings(bypass), &clock).is_err());
    let effects = engine
        .step(
            Action::NativePreferences {
                notification_enabled: false,
                autostart_enabled: true,
            },
            &clock,
        )
        .unwrap();
    assert!(effects.settings_changed);
    assert!(effects.due.is_empty());
    assert_eq!(engine.data.reminders, deadlines);
}

#[test]
fn break_window_capability_cannot_mutate_preferences_or_use_plugin_apis() {
    let break_cap: serde_json::Value =
        serde_json::from_str(include_str!("../capabilities/break.json")).unwrap();
    assert_eq!(break_cap["windows"], serde_json::json!(["break"]));
    assert_eq!(
        break_cap["permissions"],
        serde_json::json!([
            "allow-break-controls",
            "core:event:allow-listen",
            "core:event:allow-unlisten"
        ])
    );
    let main_cap: serde_json::Value =
        serde_json::from_str(include_str!("../capabilities/main.json")).unwrap();
    for permission in main_cap["permissions"].as_array().unwrap() {
        let value = permission.as_str().unwrap();
        assert!(!value.starts_with("notification:"));
        assert!(!value.starts_with("autostart:"));
    }
}

#[test]
fn native_preferences_persist_and_do_not_register_startup_when_restoring_a_session() {
    use restna_lib::{scheduler::service::Session, storage::Storage};
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("native.sqlite3");
    let clock = FakeClock::at("2026-10-21T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    let deadlines = session.engine.data.reminders.clone();
    session
        .apply(Action::NativePreferences {
            notification_enabled: false,
            autostart_enabled: true,
        })
        .unwrap();
    drop(session);
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    assert!(!session.snapshot().settings.notification_enabled);
    assert!(session.snapshot().settings.autostart_enabled);
    assert_eq!(session.engine.data.reminders, deadlines);
    // Session restoration has no autostart plugin/OS mutation dependency.
}

#[test]
fn automatic_windows_default_on_for_new_and_old_profiles_and_false_persists() {
    use restna_lib::{scheduler::service::Session, storage::Storage};
    let defaults = Settings::default();
    assert!(defaults.show_break_window);
    let mut old = serde_json::to_value(&defaults).unwrap();
    old.as_object_mut().unwrap().remove("showBreakWindow");
    assert!(
        serde_json::from_value::<Settings>(old)
            .unwrap()
            .show_break_window
    );
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("popup.db");
    let clock = FakeClock::at("2026-10-22T09:00:00Z");
    let (mut session, _) = Session::open(Storage::open(&path).unwrap(), clock.clone()).unwrap();
    let mut settings = session.snapshot().settings;
    settings.show_break_window = false;
    settings.eye_interval_minutes = 35;
    session.apply(Action::UpdateSettings(settings)).unwrap();
    drop(session);
    let (session, _) = Session::open(Storage::open(&path).unwrap(), clock).unwrap();
    assert!(!session.snapshot().settings.show_break_window);
    assert_eq!(session.snapshot().settings.eye_interval_minutes, 35);
}

#[test]
fn automatic_panel_policy_covers_both_categories_suppression_and_stale_ids() {
    for category in Category::ALL {
        let clock = FakeClock::at("2026-10-22T09:00:00Z");
        let mut engine = configured(&clock, Settings::default());
        engine.step(Action::Start(category), &clock).unwrap();
        let state = engine.snapshot(&clock);
        let reminder = state.open_reminder.clone().unwrap();
        assert!(automatic_break_allowed(
            &state,
            &reminder,
            WindowsPermission::Allowed,
            Attention::Accepting
        ));
        let mut disabled = state.clone();
        disabled.settings.show_break_window = false;
        assert!(!automatic_break_allowed(
            &disabled,
            &reminder,
            WindowsPermission::Allowed,
            Attention::Accepting
        ));
        disabled = state.clone();
        disabled.settings.notification_enabled = false;
        assert!(!automatic_break_allowed(
            &disabled,
            &reminder,
            WindowsPermission::Allowed,
            Attention::Accepting
        ));
        disabled = state.clone();
        disabled.open_reminder = None;
        assert!(!automatic_break_allowed(
            &disabled,
            &reminder,
            WindowsPermission::Allowed,
            Attention::Accepting
        ));
        for status in [
            RunStatus::Paused,
            RunStatus::Quiet,
            RunStatus::OutsideWorkHours,
            RunStatus::Disabled,
        ] {
            let mut quiet = state.clone();
            quiet.status = status;
            assert!(!automatic_break_allowed(
                &quiet,
                &reminder,
                WindowsPermission::Allowed,
                Attention::Accepting
            ));
        }
        for attention in [
            Attention::Away,
            Attention::Busy,
            Attention::Fullscreen,
            Attention::Presentation,
            Attention::QuietTime,
            Attention::Unknown,
        ] {
            assert!(!automatic_break_allowed(
                &state,
                &reminder,
                WindowsPermission::Allowed,
                attention
            ));
        }
        for permission in [
            WindowsPermission::DisabledForApplication,
            WindowsPermission::DisabledForUser,
            WindowsPermission::DisabledByPolicy,
            WindowsPermission::DisabledByManifest,
        ] {
            assert!(!automatic_break_allowed(
                &state,
                &reminder,
                permission,
                Attention::Accepting
            ));
        }
        let mut stale = reminder.clone();
        stale.id = uuid::Uuid::new_v4();
        assert!(!automatic_break_allowed(
            &state,
            &stale,
            WindowsPermission::Allowed,
            Attention::Accepting
        ));
    }
}

#[test]
fn automatic_panel_claim_is_once_per_reminder_and_dismissal_cannot_reopen_it() {
    let first = uuid::Uuid::new_v4();
    let second = uuid::Uuid::new_v4();
    let mut shown = None;
    assert!(claim_automatic_break(&mut shown, first));
    assert!(!claim_automatic_break(&mut shown, first));
    assert!(claim_automatic_break(&mut shown, second));
    assert!(!claim_automatic_break(&mut shown, second));
}

#[test]
fn restna_branding_preserves_legacy_profile_and_binary_identity() {
    let config: serde_json::Value =
        serde_json::from_str(include_str!("../tauri.conf.json")).unwrap();
    assert_eq!(config["productName"], "Restna");
    assert_eq!(config["identifier"], "com.pausepal.desktop");
    assert_eq!(config["mainBinaryName"], "pausepal");
    assert_eq!(config["app"]["windows"][0]["title"], "Restna");
    assert!(include_str!("../src/lib.rs").contains("pausepal.sqlite3"));
}
