# Installed notification investigation — 2026-10-22

User reported Send test reminder and scheduled reminders appearing not to work.

## Confirmed on the user's running installed app

- Running executable: D:/pausepal/.tools/m3-installed/pausepal.exe. Windows Start registration uses com.pausepal.desktop and its shortcut points to this executable. No installation replacement or app restart was performed.
- Both categories enabled, notifications enabled, quiet mode off, no manual/timed pause. Workdays all seven; hours 09:00–22:00 in the system timezone. Existing recent eye/movement unanswered rows and future independent deadlines show the Rust scheduler is progressing. No fake completions or test schedule edits were inserted.
- App native diagnostics: tray available; notification permission allowed; shell attention hint accepting; latest attempt submitted/unconfirmed.
- **Actual Windows Settings → System → Notifications → Do not disturb switch: On**, read using UI Automation TogglePattern after locating the Settings top-level window. No setting was toggled. SHQueryUserNotificationState returning accepting was insufficient to determine real DND state.
- A real Send test reminder click through the installed app's UI produced a fresh readable own-app toast in Windows notification history. Snapshot count 36 → 37; freshCheck=true, testReminderFound=true, testHasActionButtons=false. This proves acceptance into Windows history, not a visible banner. DND can send notifications directly to Notification Center; see [Microsoft guidance](https://support.microsoft.com/en-us/windows/experience/notifications-and-do-not-disturb-in-windows).

## Diagnostic repair

The prior notification_probe stopped at an old unreadable Windows history payload: XML document must have a top level element (0xC00CE558). The new read-only notification_diagnostics example identified 36 own-app entries, 20 readable and 16 unreadable. No global notification history was inspected or cleared. Unreadable older payloads are not evidence that the app's SQLite database or Windows itself is corrupt.

notification_probe now skips unreadable entries and reports unreadableHistoryCount. It only tests readable new entries against a multiset snapshot; unreadable entries cannot establish success. A snapshot followed immediately by --since without sending a new toast returns testReminderFound=false and exit 2. The positive installed test still passes with older unreadable entries present. notification_diagnostics identifies permission, query, size and individual-content errors without dumping toast XML.

No application delivery, scheduler, frontend, permission, migration, DND, startup or notification preference was changed. No new installer is needed for this investigation. Diagnostic examples remain developer tools.

## Commands and results

- cargo run --manifest-path src-tauri/Cargo.toml --example notification_diagnostics --locked --offline — passed, permission NotificationSetting(0), 36 entries, 20 readable / 16 unreadable.
- cargo build --manifest-path src-tauri/Cargo.toml --example notification_probe --locked --offline — passed.
- notification_probe --snapshot FILE; click installed Send test reminder; notification_probe --since FILE — passed fresh positive, no action buttons.
- Immediate unchanged snapshot / --since — expected nonzero 2, proving old entries alone do not satisfy the check.
- cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline — all 48 tests passed.
- cargo fmt --manifest-path src-tauri/Cargo.toml -- --check — passed.
- Frontend tests/build and NSIS were not rerun: no shipped application/UI source or dependencies changed. The existing running installation remains intact.

Tool limitations: the first history probe failed at unreadable older XML; fixed above. Windows Settings MainWindowHandle became zero between calls, so DND was ultimately read by finding its Settings window through the UI Automation root. InvokePattern does not apply to HTML details summary; ExpandCollapsePattern successfully exposed app diagnostics. The initial cargo diagnostic attempt stalled; it was stopped after verifying its command line and rerun with cached dependencies/offline. No claim is made that a banner was visible while DND was on.

## User action and remaining acceptance

If banners are desired, turn off Do not disturb in Windows Settings (or Win+N), then use Restna Settings → Send test reminder. If DND returns, review its automatic rules rather than allowing the app to override them. If DND should remain on, use Win+N to read accepted notifications and the Restna tray → Open current break for an active reminder. Expired reminders are unanswered, and old Action Center clicks may not reopen them.

Banner visibility after the user changes DND and physical click activation remain pending user confirmation. Other M5 clean-machine/manual gates remain pending. No Windows DND bypass or destructive reset was attempted.
