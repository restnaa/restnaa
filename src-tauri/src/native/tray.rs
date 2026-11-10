use super::NativeRuntime;
use crate::scheduler::model::{Action, Category, DashboardState, PauseRequest, RunStatus};
use std::sync::atomic::Ordering;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

pub struct TrayControls {
    status: MenuItem<tauri::Wry>,
    open: MenuItem<tauri::Wry>,
    resume: MenuItem<tauri::Wry>,
}

pub fn setup(app: &AppHandle) -> Result<(), String> {
    let build = || -> tauri::Result<()> {
        let status = MenuItem::with_id(app, "status", "Restnaa — starting", false, None::<&str>)?;
        let show = MenuItem::with_id(app, "show", "Show Restnaa", true, None::<&str>)?;
        let open = MenuItem::with_id(app, "open", "Open current break", false, None::<&str>)?;
        let eye = MenuItem::with_id(app, "eye", "Take eye break", true, None::<&str>)?;
        let movement =
            MenuItem::with_id(app, "movement", "Take movement break", true, None::<&str>)?;
        let pause30 = MenuItem::with_id(app, "pause30", "Pause 30 minutes", true, None::<&str>)?;
        let pause60 = MenuItem::with_id(app, "pause60", "Pause 1 hour", true, None::<&str>)?;
        let pause120 = MenuItem::with_id(app, "pause120", "Pause 2 hours", true, None::<&str>)?;
        let resume = MenuItem::with_id(app, "resume", "Resume", false, None::<&str>)?;
        let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
        let quit = MenuItem::with_id(app, "quit", "Quit Restnaa completely", true, None::<&str>)?;
        let separator = PredefinedMenuItem::separator(app)?;
        let menu = Menu::with_items(
            app,
            &[
                &status, &show, &open, &eye, &movement, &separator, &pause30, &pause60, &pause120,
                &resume, &settings, &quit,
            ],
        )?;
        let mut builder = TrayIconBuilder::with_id("pausepal")
            .tooltip("Restnaa")
            .menu(&menu)
            .show_menu_on_left_click(false)
            .on_menu_event(|app, event| dispatch(app, event.id.as_ref()))
            .on_tray_icon_event(|tray, event| {
                if matches!(
                    event,
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    }
                ) {
                    super::windows::show_main(tray.app_handle(), false);
                }
            });
        if let Some(icon) = app.default_window_icon() {
            builder = builder.icon(icon.clone());
        }
        builder.build(app)?;
        if let Ok(mut controls) = app.state::<NativeRuntime>().tray_controls.lock() {
            *controls = Some(TrayControls {
                status,
                open,
                resume,
            });
        }
        Ok(())
    };
    build().map_err(|_| "The Windows tray could not be created. Closing the dashboard will exit; native controls remain in Settings.".to_string())?;
    app.state::<NativeRuntime>()
        .tray_available
        .store(true, Ordering::SeqCst);
    Ok(())
}

pub fn dispatch(app: &AppHandle, id: &str) {
    match id {
        "show" => {
            super::windows::show_main(app, false);
            return;
        }
        "settings" => {
            super::windows::show_main(app, true);
            return;
        }
        "quit" => {
            super::quit(app);
            return;
        }
        _ => (),
    }
    let app = app.clone();
    let id = id.to_owned();
    tauri::async_runtime::spawn_blocking(move || {
        let result = if id == "open" {
            super::windows::open_current(&app)
        } else {
            let action = match id.as_str() {
                "eye" => Action::Start(Category::Eye),
                "movement" => Action::Start(Category::Movement),
                "pause30" => Action::Pause(PauseRequest::Minutes { minutes: 30 }),
                "pause60" => Action::Pause(PauseRequest::Minutes { minutes: 60 }),
                "pause120" => Action::Pause(PauseRequest::Minutes { minutes: 120 }),
                "resume" => Action::Resume,
                _ => return,
            };
            super::scheduler(&app)
                .and_then(|scheduler| scheduler.request(action))
                .and_then(|_| {
                    if id == "eye" || id == "movement" {
                        super::windows::show_break(&app)
                    } else {
                        Ok(())
                    }
                })
        };
        if let Err(error) = result {
            super::warn(&app, &error);
            super::windows::show_main(&app, true);
        }
    });
}

pub fn update(app: &AppHandle, state: &DashboardState) {
    let label = match state.status {
        RunStatus::Active => "Active",
        RunStatus::Paused => "Paused",
        RunStatus::OutsideWorkHours => "Outside work hours",
        RunStatus::Quiet => "Quiet mode",
        RunStatus::Disabled => "Reminders disabled",
    };
    let text = if state.open_reminder.is_some() {
        format!("Restnaa — {label} · break waiting")
    } else {
        format!("Restnaa — {label}")
    };
    if let Some(tray) = app.tray_by_id("pausepal") {
        let _ = tray.set_tooltip(Some(&text));
    }
    if let Ok(controls) = app.state::<NativeRuntime>().tray_controls.lock() {
        if let Some(controls) = controls.as_ref() {
            let _ = controls.status.set_text(&text);
            let _ = controls.open.set_enabled(state.open_reminder.is_some());
            let _ = controls
                .resume
                .set_enabled(state.status == RunStatus::Paused);
        }
    }
}
