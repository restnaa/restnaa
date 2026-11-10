use super::{model::NativeStatus, NativeRuntime};
use crate::scheduler::model::Action;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

/// OS mutation and its compensating rollback are isolated for deterministic tests.
pub fn change_with_rollback(
    enabled: bool,
    previous: bool,
    mut apply: impl FnMut(bool) -> Result<(), String>,
    persist: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if enabled != previous {
        apply(enabled)?;
    }
    if let Err(error) = persist() {
        if enabled != previous && apply(previous).is_err() {
            return Err("Could not save the preference or restore Windows startup registration. Review the Start on login setting in Windows.".into());
        }
        return Err(error);
    }
    Ok(())
}

pub fn set_autostart(app: &AppHandle, enabled: bool) -> Result<NativeStatus, String> {
    let runtime = app.state::<NativeRuntime>();
    let _guard = runtime
        .preference_lock
        .lock()
        .map_err(|_| "Native settings unavailable.")?;
    let scheduler = super::scheduler(app)?;
    let state = scheduler.request(Action::Tick)?;
    let previous = app
        .autolaunch()
        .is_enabled()
        .map_err(|_| "Could not read Windows startup registration.")?;
    change_with_rollback(
        enabled,
        previous,
        |value| {
            if value {
                app.autolaunch().enable()
            } else {
                app.autolaunch().disable()
            }
            .map_err(|_| "Could not change Windows startup registration.".into())
        },
        || {
            scheduler
                .request(Action::NativePreferences {
                    notification_enabled: state.settings.notification_enabled,
                    autostart_enabled: enabled,
                })
                .map(|_| ())
        },
    )?;
    Ok(super::status(app, &scheduler.request(Action::Tick)?))
}

pub fn set_notifications(app: &AppHandle, enabled: bool) -> Result<NativeStatus, String> {
    use tauri_plugin_notification::NotificationExt;
    let runtime = app.state::<NativeRuntime>();
    let _guard = runtime
        .preference_lock
        .lock()
        .map_err(|_| "Native settings unavailable.")?;
    if enabled {
        app.notification()
            .request_permission()
            .map_err(|_| "Could not check notification permission.")?;
    }
    let scheduler = super::scheduler(app)?;
    let state = scheduler.request(Action::Tick)?;
    let state = scheduler.request(Action::NativePreferences {
        notification_enabled: enabled,
        autostart_enabled: state.settings.autostart_enabled,
    })?;
    Ok(super::status(app, &state))
}
