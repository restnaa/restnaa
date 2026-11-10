use super::NativeRuntime;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, Manager, WindowEvent};

pub fn show_main(app: &AppHandle, settings: bool) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus();
        if settings {
            let _ = app.emit_to("main", "navigate", "settings");
        }
    }
}

pub fn show_break(app: &AppHandle) -> Result<(), String> {
    let window = app
        .get_webview_window("break")
        .ok_or("Break window unavailable. Use the dashboard to check your schedule.")?;
    window
        .show()
        .map_err(|_| "Could not show the break window.")?;
    window
        .unminimize()
        .map_err(|_| "Could not restore the break window.")?;
    window
        .set_focus()
        .map_err(|_| "Could not focus the break window.")?;
    Ok(())
}

/// Automatic presentation must not activate or focus the break window.
pub fn show_break_automatically(app: &AppHandle) -> Result<(), String> {
    #[cfg(windows)]
    {
        use ::windows::Win32::{
            Foundation::HWND,
            UI::WindowsAndMessaging::{IsWindowVisible, ShowWindow, SW_SHOWNOACTIVATE},
        };
        let handle = app.clone();
        // ShowWindow sends synchronous window messages. Run it on the window's
        // owning thread, never block the scheduler waiting on cross-thread UI.
        app.run_on_main_thread(move || {
            let result = (|| -> Result<(), String> {
                let window = handle
                    .get_webview_window("break")
                    .ok_or("Break window unavailable. Use the tray fallback.")?;
                let hwnd = window
                    .hwnd()
                    .map_err(|_| "Could not access the break window.")?;
                // False means previously hidden, not failure.
                unsafe {
                    let _ = ShowWindow(HWND(hwnd.0), SW_SHOWNOACTIVATE);
                }
                if !unsafe { IsWindowVisible(HWND(hwnd.0)) }.as_bool() {
                    return Err("Could not show the break window. Use the tray fallback.".into());
                }
                Ok(())
            })();
            if let Err(error) = result {
                super::warn(&handle, &error);
            }
        })
        .map_err(|_| "Could not queue the break window. Use the tray fallback.".into())
    }
    #[cfg(not(windows))]
    {
        let _ = app;
        // Do not silently substitute a focus-stealing cross-platform show.
        Err("Automatic break windows are currently supported on Windows only. Use the tray fallback.".into())
    }
}

pub fn open_current(app: &AppHandle) -> Result<(), String> {
    let state = super::scheduler(app)?.request(crate::scheduler::model::Action::Tick)?;
    if state.open_reminder.is_none() {
        return Err("There is no current break. You can start one from the tray.".into());
    }
    show_break(app)
}

pub fn hide_break(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("break") {
        window
            .hide()
            .map_err(|_| "Could not hide the break window.")?;
    }
    #[cfg(windows)]
    {
        // Automatic presentation bypasses Tao's visibility bookkeeping. Its
        // normal hide may therefore be a no-op; pair the native show with a
        // native hide, on the owning thread, for every dismissal path.
        let handle = app.clone();
        app.run_on_main_thread(move || {
            use ::windows::Win32::{
                Foundation::HWND,
                UI::WindowsAndMessaging::{ShowWindow, SW_HIDE},
            };
            if let Some(window) = handle.get_webview_window("break") {
                match window.hwnd() {
                    Ok(hwnd) => unsafe {
                        let _ = ShowWindow(HWND(hwnd.0), SW_HIDE);
                    },
                    Err(_) => super::warn(
                        &handle,
                        "Could not hide the break window. Use the tray to quit Restna.",
                    ),
                }
            }
        })
        .map_err(|_| "Could not queue break window dismissal.".to_string())?;
    }
    Ok(())
}

pub fn on_window_event(window: &tauri::Window, event: &WindowEvent) {
    let app = window.app_handle();
    if let WindowEvent::CloseRequested { api, .. } = event {
        if let Some(runtime) = app.try_state::<NativeRuntime>() {
            match super::model::close_behavior(
                window.label(),
                runtime.tray_available.load(Ordering::SeqCst),
                runtime.quitting.load(Ordering::SeqCst),
            ) {
                super::model::CloseBehavior::Quit => {
                    // A hidden break webview also exists. Destroying only main
                    // would leave an invisible process with no tray fallback.
                    api.prevent_close();
                    super::quit(app);
                }
                super::model::CloseBehavior::Hide => {
                    api.prevent_close();
                    let hidden = if window.label() == "break" {
                        hide_break(app)
                    } else {
                        window.hide().map_err(|error| error.to_string())
                    };
                    if hidden.is_err() {
                        super::warn(app, "Could not hide the window. You can still quit from Settings or the tray.");
                    }
                }
                super::model::CloseBehavior::Allow => (),
            }
        }
    }
}
