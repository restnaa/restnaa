use crate::{dto::AppInfo, FoundationState};
use crate::{scheduler::model::*, settings::Settings};
use uuid::Uuid;

#[tauri::command]
pub fn get_app_info(state: tauri::State<'_, FoundationState>) -> AppInfo {
    AppInfo {
        version: env!("CARGO_PKG_VERSION").into(),
        storage: state.storage_status.clone(),
    }
}

async fn dispatch(
    state: tauri::State<'_, FoundationState>,
    action: Action,
) -> Result<DashboardState, String> {
    let handle = state
        .scheduler
        .clone()
        .ok_or("Scheduler unavailable. Check the local storage warning.")?;
    tauri::async_runtime::spawn_blocking(move || handle.request(action))
        .await
        .map_err(|_| "Scheduler request failed.".to_string())?
}

#[tauri::command]
pub async fn get_dashboard_state(
    state: tauri::State<'_, FoundationState>,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Tick).await
}
#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, FoundationState>) -> Result<Settings, String> {
    Ok(dispatch(state, Action::Tick).await?.settings)
}
#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, FoundationState>,
    settings: Settings,
) -> Result<DashboardState, String> {
    dispatch(state, Action::UpdateSettings(settings)).await
}
#[tauri::command]
pub async fn start_break(
    state: tauri::State<'_, FoundationState>,
    category: Category,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Start(category)).await
}
#[tauri::command]
pub async fn complete_break(
    state: tauri::State<'_, FoundationState>,
    reminder_id: Uuid,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Complete(reminder_id)).await
}
#[tauri::command]
pub async fn skip_break(
    state: tauri::State<'_, FoundationState>,
    reminder_id: Uuid,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Skip(reminder_id)).await
}
#[tauri::command]
pub async fn snooze_break(
    state: tauri::State<'_, FoundationState>,
    reminder_id: Uuid,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Snooze(reminder_id)).await
}
#[tauri::command]
pub async fn pause_reminders(
    state: tauri::State<'_, FoundationState>,
    request: PauseRequest,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Pause(request)).await
}
#[tauri::command]
pub async fn resume_reminders(
    state: tauri::State<'_, FoundationState>,
) -> Result<DashboardState, String> {
    dispatch(state, Action::Resume).await
}

#[tauri::command]
pub async fn get_native_status(
    app: tauri::AppHandle,
) -> Result<crate::native::model::NativeStatus, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = crate::native::scheduler(&app)?.request(Action::Tick)?;
        Ok(crate::native::status(&app, &state))
    })
    .await
    .map_err(|_| "Native status request failed.".to_string())?
}

#[tauri::command]
pub async fn set_notifications_enabled(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<crate::native::model::NativeStatus, String> {
    tauri::async_runtime::spawn_blocking(move || {
        crate::native::startup::set_notifications(&app, enabled)
    })
    .await
    .map_err(|_| "Notification preference request failed.".to_string())?
}

#[tauri::command]
pub async fn set_autostart_enabled(
    app: tauri::AppHandle,
    enabled: bool,
) -> Result<crate::native::model::NativeStatus, String> {
    tauri::async_runtime::spawn_blocking(move || {
        crate::native::startup::set_autostart(&app, enabled)
    })
    .await
    .map_err(|_| "Startup preference request failed.".to_string())?
}

#[tauri::command]
pub async fn test_notification(
    app: tauri::AppHandle,
) -> Result<crate::native::model::Attempt, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state = crate::native::scheduler(&app)?.request(Action::Tick)?;
        Ok(crate::native::notifications::send(
            &app,
            state.settings.notification_enabled,
            "Restnaa test reminder",
            "A gentle reminder from Restnaa. Use the tray icon to open a current break.",
            None,
        ))
    })
    .await
    .map_err(|_| "Notification test failed.".to_string())?
}

#[tauri::command]
pub async fn open_current_break(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || crate::native::windows::open_current(&app))
        .await
        .map_err(|_| "Could not open the break window.".to_string())?
}

#[tauri::command]
pub async fn hide_break_window(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || crate::native::windows::hide_break(&app))
        .await
        .map_err(|_| "Could not hide the break window.".to_string())?
}

#[tauri::command]
pub fn quit_app(app: tauri::AppHandle) {
    crate::native::quit(&app);
}

#[tauri::command]
pub async fn get_history(
    state: tauri::State<'_, FoundationState>,
    range: crate::history::HistoryRange,
) -> Result<crate::history::History, String> {
    let handle = state
        .scheduler
        .clone()
        .ok_or("History unavailable. Check local storage.")?;
    tauri::async_runtime::spawn_blocking(move || handle.history(range))
        .await
        .map_err(|_| "History request failed.".to_string())?
}

#[tauri::command]
pub async fn clear_history(
    state: tauri::State<'_, FoundationState>,
    confirmed: bool,
) -> Result<(), String> {
    let handle = state
        .scheduler
        .clone()
        .ok_or("History unavailable. Check local storage.")?;
    tauri::async_runtime::spawn_blocking(move || handle.clear_history(confirmed))
        .await
        .map_err(|_| "History deletion failed.".to_string())?
}
