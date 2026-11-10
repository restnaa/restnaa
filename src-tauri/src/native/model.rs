use serde::Serialize;

#[derive(Debug, PartialEq, Eq)]
pub enum CloseBehavior {
    Hide,
    Quit,
    Allow,
}

pub fn close_behavior(label: &str, tray_available: bool, quitting: bool) -> CloseBehavior {
    if quitting {
        return CloseBehavior::Allow;
    }
    match label {
        "main" if !tray_available => CloseBehavior::Quit,
        "main" | "break" => CloseBehavior::Hide,
        _ => CloseBehavior::Allow,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WindowsPermission {
    Allowed,
    DisabledForApplication,
    DisabledForUser,
    DisabledByPolicy,
    DisabledByManifest,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Attention {
    Accepting,
    Away,
    Busy,
    Fullscreen,
    Presentation,
    QuietTime,
    App,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Delivery {
    DisabledInApp,
    DeniedByWindows,
    SuppressedByAttention,
    SubmittedUnconfirmed,
    PreparationFailed,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attempt {
    pub at_utc: String,
    pub result: Delivery,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeStatus {
    pub notification_enabled: bool,
    pub autostart_enabled: bool,
    pub autostart_registered: Option<bool>,
    pub windows_permission: WindowsPermission,
    pub attention: Attention,
    pub tray_available: bool,
    pub last_attempt: Option<Attempt>,
    pub warning: Option<String>,
}

pub fn delivery_policy(
    enabled: bool,
    permission: WindowsPermission,
    attention: Attention,
) -> Option<Delivery> {
    if !enabled {
        return Some(Delivery::DisabledInApp);
    }
    if !matches!(
        permission,
        WindowsPermission::Allowed | WindowsPermission::Unknown
    ) {
        return Some(Delivery::DeniedByWindows);
    }
    if matches!(
        attention,
        Attention::Away
            | Attention::Busy
            | Attention::Fullscreen
            | Attention::Presentation
            | Attention::QuietTime
    ) {
        return Some(Delivery::SuppressedByAttention);
    }
    None
}

pub fn matches_current_click(
    clicked: Option<&str>,
    current: Option<&crate::scheduler::model::OpenReminder>,
) -> bool {
    clicked
        .and_then(|id| uuid::Uuid::parse_str(id).ok())
        .zip(current)
        .is_some_and(|(id, open)| id == open.id)
}

/// An automatic panel is a presentation of one committed reminder, never a timer.
/// Windows attention is a best-effort hint, not a reliable DND detector.
pub fn automatic_break_allowed(
    state: &crate::scheduler::model::DashboardState,
    reminder: &crate::scheduler::model::OpenReminder,
    permission: WindowsPermission,
    attention: Attention,
) -> bool {
    use crate::scheduler::model::RunStatus;
    state.settings.show_break_window
        && state.settings.notification_enabled
        && state.status == RunStatus::Active
        && reminder.category.enabled(&state.settings)
        && state.open_reminder.as_ref().map(|current| current.id) == Some(reminder.id)
        && matches!(attention, Attention::Accepting | Attention::App)
        && delivery_policy(true, permission, attention).is_none()
}

pub fn claim_automatic_break(last: &mut Option<uuid::Uuid>, id: uuid::Uuid) -> bool {
    if *last == Some(id) {
        return false;
    }
    *last = Some(id);
    true
}
