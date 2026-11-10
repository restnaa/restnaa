> Historical PausePal record. Restna is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](OPEN_SOURCE_PLAN.md).

# M3 native Windows integration and acceptance checks

The native integration uses official Tauri v2 notification, autostart, and single-instance plugins. All mutation APIs are Rust-owned. The main webview gets narrowly scoped custom commands; no direct notification/autostart/shell/filesystem/HTTP plugin grants are exposed. The break webview gets only read state, Start/Done/Snooze/Skip, hide, and event listen/unlisten.

## Behavior

- Single-instance registration is the first plugin, before SQLite or scheduler creation. A repeated manual launch shows/focuses the existing main window; a duplicate `--autostart` invocation stays quiet.
- Closing the main window hides it to the tray. The tray has Show, Open current break, Take eye/movement break, Pause 30m/1h/2h, Resume, Settings, and Quit. Quit exits the entire process. If tray creation fails, the dashboard stays accessible and closing it can exit rather than strand an invisible app.
- Automatic scheduler effects are committed before a toast is attempted. They never show/focus the break window. A user tray action or supported toast-body click can show it. No toast action types/buttons, OS notification schedules, priority/DND overrides, or automatic full-screen overlays are used.
- The official notification plugin version pinned in `Cargo.lock` supports on-screen Windows body clicks. Action Center activation is not guaranteed. A callback must contain the UUID of the currently open Rust reminder; stale/unknown clicks cannot act on another reminder. The tray remains the reliable fallback.
- The predefined, hidden break window implements explicit Start/Done/Snooze/Skip through Rust commands. Start does not complete a break. Esc, Close for now, and the window close button hide it without recording completion; the Rust unanswered timeout remains authoritative. No JS reminder/countdown timer is introduced.
- Notifications default on and autostart off when loading older M2 settings. Native preference changes persist through the worker without resetting reminder deadlines. General scheduler settings cannot bypass the dedicated native commands.
- Autostart registration is changed only by an explicit user command. Boot reads actual registration but never silently enables or repairs it. Startup-launched apps start hidden only when the tray is available. An OS registration change is rolled back if preference persistence fails; a failed rollback is reported. Startup and notification settings can differ from Windows policy and the UI reports those differences.

## Diagnostics and limitations

Settings exposes app notification preference, actual startup registration, this app's Windows `ToastNotifier.Setting`, a read-only `SHQueryUserNotificationState` attention hint, tray readiness, and the last notification attempt.

- Disabled in app: no toast is sent.
- Disabled for app/user/policy/manifest: no toast is sent; review Windows notification settings.
- Away/busy/fullscreen/presentation/quiet-time shell hint: skip delivery and explain suppression; scheduling and tray fallback continue.
- Unknown attention state: let Windows handle its own suppression. Do not infer that Focus Assist is off.
- Plugin success: `submittedUnconfirmed`, never proof of visible delivery. The plugin prepares/dispatches on a background task; later Windows failures may not be reported.
- Preparation failure: show a diagnostic without exposing paths or resetting data.

No API here completely detects Do not disturb/Focus Assist, guarantees banners, or bypasses Windows preferences. Refresh diagnostics and Send test reminder are explicit user actions. Notification permission checks on desktop may report granted while Windows settings suppress display.

## Build and installed verification

```powershell
npm run tauri build -- --bundles nsis -- --locked
```

The NSIS installer targets the current user. Installer output is under `src-tauri/target/release/bundle/nsis`. Building a portable executable or seeing a successful notification API return does not satisfy installed notification verification.

Acceptance checklist (update only with observed evidence):

| Check | Status |
|---|---|
| Rust policy, stale-click, compatibility, rollback, capability and scheduler tests | Passed locally |
| Frontend break/native preference tests and static checks | Passed locally |
| Desktop/compact browser regressions | Six passed serially and all six passed again with four workers (22.6 seconds) |
| NSIS build and per-user installation | Passed on this Windows 11 Pro host; installer exit 0 |
| Installed executable native notification test | Passed: explicit Settings test submitted through installed app |
| Actual Windows notification/banner or own notification history evidence | Passed for own Windows notification history: two entries, test title found, XML has no action buttons. Visible banner remains pending |
| Main close-to-tray / restore / explicit Quit | Passed: close hid main while process remained alive, duplicate launch restored main, Settings Quit exited process |
| Installed duplicate launch preserves one app process | Passed: duplicate exited within five seconds; one PausePal process remained |
| Autostart explicit on/off registration, restored after QA | Passed: checkbox and HKCU Run registration on/off agreed; restored off |
| Startup invocation stays quiet | Passed with installed `--autostart`: enumerated main and break windows both hidden; no sign-out performed |
| Actual login launches quietly | Pending manual test |
| Toast-body click opens matching break | Pending manual test |
| Break actions through installed native window | Start, Esc hide without outcome, reopen and Snooze passed. Done/Skip passed unit tests; installed clicks pending |
| Direct installed tray-menu interactions | Pending manual test; tray creation/readiness and hide lifecycle passed |
| Physical DND/Focus Assist and notification-denied scenarios | Pending manual test; policy paths unit tested |
| Clean Windows 10 and Windows 11 install/uninstall | Pending clean-machine tests |

Local evidence, 2026-10-21: installed from `src-tauri/target/release/bundle/nsis/PausePal_0.1.0_x64-setup.exe` into ignored `.tools/m3-installed`. Screenshots are in `C:/OracleTemp/pausepal-m3-qa`. Test settings and notification preference were restored; startup registration was removed, and the app was explicitly quit. The Snooze test intentionally recorded a snoozed outcome; no existing history was erased. This is implementation with pending acceptance checks, not a claim of complete clean-machine validation.

The read-only probe queries notification history for **this app ID only**, checks the test title and absence of `<actions>`, and returns a nonzero exit when absent. After sending a test from the installed app:

```powershell
cargo run --manifest-path src-tauri/Cargo.toml --example notification_probe --locked
# Observed: historyCount=2, testReminderFound=true, testHasActionButtons=false
```

Local checks: Rust 36 tests, frontend 27 tests, strict Clippy, formatting, TypeScript, ESLint, frontend build, and NSIS build passed. An initial browser run timed out once under concurrent build load; complete serial and final parallel six-test reruns passed. The first Rust compile caught private notification fields; using the plugin's public accessors resolved it. A startup-toggle automation assertion checked before async persistence completed; an eventual checked-state assertion and native/registry checks passed. Quit closes the CDP target, so a final awaiting IPC evaluation reported target-closed after the app exited; this is expected teardown, verified by the absent process.

Next installed acceptance steps:

1. Launch the installed app; right-click its tray icon. Exercise Show, Settings, Take eye/movement break, Open current break, pause/resume, and Quit. Confirm no overlapping windows and sensible disabled controls.
2. Complete and skip separate eligible breaks through the native window; confirm distinct outcomes and future deadlines. Respect the real two-minute delivery guard.
3. Send a test with banners enabled and click an on-screen reminder for the current UUID; verify the matching break opens. Try an old toast and verify it cannot act on a later reminder. Action Center activation may be unsupported.
4. Disable notifications in Windows, refresh diagnostics, then restore the original setting. Repeat with DND/Focus Assist; verify no forced window/focus and tray actions remain available. The attention hint is not a complete DND detector.
5. Explicitly enable startup, sign out/in, verify one hidden app with usable tray, then restore the desired startup preference. Repeat install/uninstall on clean Windows 10 and 11; signing remains a release task.

Official references: [Tauri notifications](https://v2.tauri.app/plugin/notification/), [single instance](https://v2.tauri.app/plugin/single-instance/), [autostart](https://v2.tauri.app/plugin/autostart/), [Windows shell notification hint](https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shqueryusernotificationstate).

## M5 follow-up

See [current release audit](RELEASE_AUDIT.md) for superseding results. M5 verified the installed final identity with a fresh Windows-history test toast and no action buttons, notification-off enforcement, one-process close/restore/Quit, plus isolated installed completion/clear-history/corruption/startup/uninstall tests. This remains existing-host evidence; visible banners/body clicks, actual Windows permission denial/DND, direct tray-menu input, physical power/time/session behavior, actual login and clean Windows 10/11 are still pending. The notification probe now supports --snapshot FILE / --since FILE and counts duplicate identical XML entries; an old snapshot without a new row cannot pass.

## User notification report, 2026-10-22

See [installed notification investigation](NOTIFICATION_TROUBLESHOOTING.md). The actual Windows DND switch was On despite an accepting shell attention hint; a fresh installed test toast reached own-app Windows history. DND was left unchanged. The read-only history probe now reports/skips older unreadable XML payloads rather than aborting before checking new entries. Post-DND banner visibility remains pending user confirmation.

## Automatic break-window change, 0.1.1

See [implementation plan and evidence](AUTO_BREAK_WINDOW_PLAN.md). The user authorized default On and explicitly accepted best-effort Windows DND. Newly due eligible reminders may show the existing break HWND using documented [ShowWindow SW_SHOWNOACTIVATE](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow), with no focus call or always-on-top setting. Explicit user actions still focus normally. Closing/Esc leaves the reminder open until its timeout, without reopening on each scheduler tick. No JavaScript reminder timer was introduced.

The OS-notification test button remains a notification-only test; Dashboard Take eye/movement break tests the break panel immediately. Popup preference changes use the existing Settings save semantics, including fresh schedule intervals. No SQLite migration or new Tauri capability is required.
