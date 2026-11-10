pub mod clock;
mod commands;
pub mod dto;
pub mod history;
pub mod native;
pub mod scheduler;
pub mod settings;
pub mod storage;

use dto::StorageStatus;
use tauri::Manager;

// The worker owns the connection and serializes every scheduling transition.
// No SQL or filesystem command is registered for the webview.
pub struct FoundationState {
    pub storage_status: StorageStatus,
    pub scheduler: Option<scheduler::service::SchedulerHandle>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _| {
            if !args.iter().any(|arg| arg == "--autostart") { native::windows::show_main(app, false); }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::Builder::new().arg("--autostart").build())
        .setup(|app| {
            app.manage(native::NativeRuntime::default());
            if let Err(error) = native::tray::setup(app.handle()) { native::warn(app.handle(), &error); }
            if let Err(error) = native::notifications::register_click_handler(app.handle()) { native::warn(app.handle(), &error); }
            let opened = app.path().app_data_dir().map_err(|_| ()).and_then(|path| {
                std::fs::create_dir_all(&path).map_err(|_| ())?;
                storage::Storage::open(&path.join("pausepal.sqlite3")).map_err(|_| ())
            });
            let scheduler = opened.and_then(|storage| scheduler::service::spawn(storage, app.handle().clone()).map_err(|_| ()));
            let state = match scheduler {
                Ok(scheduler) => FoundationState {
                    storage_status: StorageStatus::Ready { schema_version: storage::SCHEMA_VERSION },
                    scheduler: Some(scheduler),
                },
                Err(()) => FoundationState {
                    storage_status: StorageStatus::Unavailable {
                        message: "Local storage could not be opened. Existing data has not been erased. Close Restna and check the app data folder and its permissions before restarting.".into(),
                    },
                    scheduler: None,
                },
            };
            app.manage(state);
            if std::env::args().any(|arg| arg == "--autostart") && app.state::<native::NativeRuntime>().tray_available.load(std::sync::atomic::Ordering::SeqCst) {
                if let Some(window) = app.get_webview_window("main") { let _ = window.hide(); }
            } else {
                native::windows::show_main(app.handle(), false);
            }
            Ok(())
        })
        .on_window_event(native::windows::on_window_event)
        .invoke_handler(tauri::generate_handler![commands::get_app_info, commands::get_dashboard_state, commands::get_settings, commands::update_settings, commands::start_break, commands::complete_break, commands::skip_break, commands::snooze_break, commands::pause_reminders, commands::resume_reminders, commands::get_native_status, commands::set_notifications_enabled, commands::set_autostart_enabled, commands::test_notification, commands::open_current_break, commands::hide_break_window, commands::quit_app, commands::get_history, commands::clear_history])
        .build(tauri::generate_context!())
        .expect("Restna failed to start")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                let runtime = app.state::<native::NativeRuntime>();
                if runtime.tray_available.load(std::sync::atomic::Ordering::SeqCst) && !runtime.quitting.load(std::sync::atomic::Ordering::SeqCst) { api.prevent_exit(); }
            }
        });
}
