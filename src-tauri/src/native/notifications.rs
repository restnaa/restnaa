use super::{model::*, NativeRuntime};
use crate::scheduler::model::{Category, DashboardState, OpenReminder};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_notification::NotificationExt;

pub fn windows_status(app: &AppHandle) -> (WindowsPermission, Attention) {
    #[cfg(windows)]
    {
        use ::windows::{
            core::HSTRING,
            Win32::UI::Shell::*,
            UI::Notifications::{NotificationSetting, ToastNotificationManager},
        };
        let permission = ToastNotificationManager::CreateToastNotifierWithId(&HSTRING::from(
            &app.config().identifier,
        ))
        .and_then(|notifier| notifier.Setting())
        .map(|setting| match setting {
            NotificationSetting::Enabled => WindowsPermission::Allowed,
            NotificationSetting::DisabledForApplication => {
                WindowsPermission::DisabledForApplication
            }
            NotificationSetting::DisabledForUser => WindowsPermission::DisabledForUser,
            NotificationSetting::DisabledByGroupPolicy => WindowsPermission::DisabledByPolicy,
            NotificationSetting::DisabledByManifest => WindowsPermission::DisabledByManifest,
            _ => WindowsPermission::Unknown,
        })
        .unwrap_or(WindowsPermission::Unknown);
        // Read-only shell hint. This is not a complete Focus Assist/DND detector.
        let attention = unsafe { SHQueryUserNotificationState() }
            .map(|state| match state {
                QUNS_ACCEPTS_NOTIFICATIONS => Attention::Accepting,
                QUNS_NOT_PRESENT => Attention::Away,
                QUNS_BUSY => Attention::Busy,
                QUNS_RUNNING_D3D_FULL_SCREEN => Attention::Fullscreen,
                QUNS_PRESENTATION_MODE => Attention::Presentation,
                QUNS_QUIET_TIME => Attention::QuietTime,
                QUNS_APP => Attention::App,
                _ => Attention::Unknown,
            })
            .unwrap_or(Attention::Unknown);
        (permission, attention)
    }
    #[cfg(not(windows))]
    {
        let _ = app;
        (WindowsPermission::Unknown, Attention::Unknown)
    }
}

fn record(app: &AppHandle, result: Delivery, message: &str) -> Attempt {
    let attempt = Attempt {
        at_utc: chrono::Utc::now().to_rfc3339(),
        result,
        message: message.into(),
    };
    if let Ok(mut last) = app.state::<NativeRuntime>().last_attempt.lock() {
        *last = Some(attempt.clone());
    }
    let _ = app.emit_to("main", "native-status-changed", ());
    attempt
}

pub fn send(
    app: &AppHandle,
    enabled: bool,
    title: &str,
    body: &str,
    reminder: Option<&OpenReminder>,
) -> Attempt {
    let (permission, attention) = windows_status(app);
    if let Some(result) = delivery_policy(enabled, permission, attention) {
        let message = match result {
            Delivery::DisabledInApp => "Notifications are disabled in Restnaa. The tray can still open the current break.",
            Delivery::DeniedByWindows => "Windows has disabled notifications for this app, user, or policy. Review Windows notification settings; the tray remains available.",
            _ => "Windows reports an attention state that may suppress alerts. This reminder was not sent; use the tray when convenient.",
        };
        return record(app, result, message);
    }
    let mut builder = app
        .notification()
        .builder()
        .id((uuid::Uuid::new_v4().as_u128() as u32 & 0x7fff_ffff) as i32)
        .title(title)
        .body(body);
    if let Some(reminder) = reminder {
        builder = builder.extra("reminderId", reminder.id.to_string());
    } else {
        builder = builder.extra("test", true);
    }
    // No action types/buttons, priority override, OS schedule, or forced window.
    match builder.show() {
        Ok(()) => record(app, Delivery::SubmittedUnconfirmed, "Submitted to Windows. Visible delivery is not confirmed; Do not disturb/Focus Assist or Windows settings may hide it. Use the tray as fallback."),
        Err(_) => record(app, Delivery::PreparationFailed, "The notification could not be prepared. Check Windows app notification settings and use the tray as fallback."),
    }
}

pub fn reminder(app: &AppHandle, state: &DashboardState, reminder: &OpenReminder) {
    if state.open_reminder.as_ref().map(|open| open.id) != Some(reminder.id) {
        return;
    }
    let (title, body) = match reminder.category {
        Category::Eye => ("Time for an eye break", "Try looking about 20 feet away for 20 seconds. Open your break from the Restnaa tray icon."),
        Category::Movement => ("Time for a movement break", "A moment to stand, walk briefly, or roll your shoulders. Open your break from the Restnaa tray icon."),
        Category::Water => ("A moment for water", "Consider a drink of water. Follow your own needs and any fluid advice from your clinician. Open the reminder from the Restnaa tray icon."),
    };
    send(
        app,
        state.settings.notification_enabled,
        title,
        body,
        Some(reminder),
    );
}

pub fn register_click_handler(app: &AppHandle) -> Result<(), String> {
    let handle = app.clone();
    app.notification()
        .on_action(move |action| {
            if action.action_id() != "tap" {
                return;
            }
            let app = handle.clone();
            let clicked = action
                .notification()
                .and_then(|notification| notification.extra().get("reminderId"))
                .and_then(|value| value.as_str())
                .map(str::to_owned);
            let test = action
                .notification()
                .and_then(|notification| notification.extra().get("test"))
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            tauri::async_runtime::spawn_blocking(move || {
                if test {
                    super::windows::show_main(&app, true);
                    return;
                }
                if let Ok(state) = super::scheduler(&app)
                    .and_then(|scheduler| scheduler.request(crate::scheduler::model::Action::Tick))
                {
                    if matches_current_click(clicked.as_deref(), state.open_reminder.as_ref()) {
                        let _ = super::windows::show_break(&app);
                    }
                }
            });
        })
        .map_err(|_| "Notification click handling is unavailable; use the tray fallback.".into())
}
