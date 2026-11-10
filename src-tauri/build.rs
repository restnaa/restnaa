fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_app_info",
            "get_dashboard_state",
            "get_settings",
            "update_settings",
            "start_break",
            "complete_break",
            "skip_break",
            "snooze_break",
            "pause_reminders",
            "resume_reminders",
            "get_native_status",
            "set_notifications_enabled",
            "set_autostart_enabled",
            "test_notification",
            "open_current_break",
            "hide_break_window",
            "quit_app",
            "get_history",
            "clear_history",
        ]),
    ))
    .expect("failed to build PausePal application manifest");
}
