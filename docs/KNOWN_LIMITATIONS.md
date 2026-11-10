# Restna current release limitations

Restna 0.1.3 is the renamed open-source candidate. Automated/build checks are recorded in OPEN_SOURCE_REPORT.md. Historical PausePal evidence below does not verify this renamed installer. Installed upgrade/startup/notification identity, clean Windows 10/11, signing, physical toast activation, screen-reader and runtime-egress gates remain pending. Source publication is separate from stable binary release approval.

# Known limitations and remaining release gates

The 0.1.0 audit package and 0.1.1 automatic-window follow-up are reviewable release candidates. **Public-release decision: NO-GO until required manual gates have evidence.** An existing developer Windows host is not a clean Windows 10 or 11 installation.

## Pending platform validation

- Clean Windows 10 and clean Windows 11 install/uninstall, missing-WebView2 prerequisite path, standard-user behavior and packaging identity.
- Actual OS sign-out/sign-in startup. A simulated `--autostart` launch and registry inspection cannot prove login behavior.
- Physical sleep/wake, real OS clock/timezone changes and session lock/unlock. Fake-clock, timezone/DST and restart tests validate policy, not OS power-event behavior. The worker rechecks at most every 30 seconds; native power/time-event hooks are not installed.
- Visible notification banners and body-click activation, stale notification clicks, and actual DND/Focus Assist behavior. Own-app notification history proves dispatch to Windows, not visible banners. Action Center click activation is not guaranteed by the official plugin.
- Direct tray-menu interactions where the desktop/taskbar automation is unavailable, screen-reader/high-contrast OS validation, and seven days of real dogfooding.
- License/third-party-notice and provisional product-name review have not been performed.
- Authenticode signing, publisher identity, SmartScreen reputation and hosted CI execution. The current candidate has no signing certificate configured.

Current attempts/results, including notification-denial availability, are tracked in [RELEASE_AUDIT.md](RELEASE_AUDIT.md); this list must not be silently converted to passed based on mocks or a development build.

## Runtime network/privacy release blocker

One HTTPS connection was observed in the app/WebView2 descendant tree after the idle sample. It closed before its owning subprocess or purpose could be captured. No application HTTP calls or telemetry implementation were found in source, but zero runtime egress is not established. Microsoft documents required runtime diagnostics; see [privacy](../PRIVACY.md). Strict zero-egress expectations need further runtime/network investigation and a product decision before release. No SmartScreen, firewall, global diagnostic or security settings were weakened.

## Product/runtime policies

- The process must be running to schedule. Quit stops reminders. Windows notification policy, shell/Explorer availability and WebView2 affect delivery/UI.
- No Windows toast action buttons, global DND override, full-screen locks, exercise detection or medical claims. Use the break window/tray fallback.
- Overnight work schedules are rejected. One current system timezone is used for scheduling and history; past events may regroup if that timezone changes.
- A five-minute unanswered timeout and five-minute snooze are internal defaults; deliveries are spaced at least two minutes. Durations are suggestions, without an animated countdown.
- Settings apply when saved. A changed schedule dismisses an open reminder as unanswered and schedules fresh intervals. Native settings and pause use separate commands.
- History details are capped at 100 while totals include the whole chosen period. A view left open across midnight/timezone change needs Refresh, focus, or a new outcome to reload.
- Clear history removes outcome rows, not preferences, active reminders, Windows notification history, backups, or forensic remnants. The local SQLite database is not encrypted by the app.
- The installer can need internet for WebView2, despite offline application runtime. No bundled offline WebView2 runtime is included.

## Dependency follow-up

The audited lockfile has no RustSec vulnerability entries, but informational findings remain: proc-macro-error (`RUSTSEC-2024-0370`), zbus_names (`RUSTSEC-2026-0371`), zvariant (`RUSTSEC-2026-0370`), zvariant_derive (`RUSTSEC-2026-0372`) and glib (`RUSTSEC-2024-0429`). These are absent from the Windows-target dependency graph and arise from other-platform transitive paths; don't describe the whole cross-platform graph as warning-free or port it without review. Npm's audit result and RustSec database revision/date are recorded in the audit report, not a future security guarantee.

## 0.1.1 automatic break-window candidate

Automatic break windows are enabled by default, including when upgrading an older local profile. Disable Show break window when a reminder is due in Settings and Save settings to opt out. Explicit off persists. Automatic panels require notifications enabled and respect app Pause, work hours and Quiet mode. They do not take keyboard focus or become always-on-top. Closing/Esc keeps the existing unanswered timeout; an ordinary tick does not reopen the same panel.

Windows DND handling is **best effort, explicitly accepted by the user**. Known suppressed/unknown shell attention states and OS notification denial block the panel; an accepting shell hint can coexist with Windows DND On, so custom panels can still appear in that case. Use PausePal Quiet mode or Pause for guaranteed app suppression. No global OS settings or undocumented DND APIs are used. Automatic panels are implemented for Windows; other platforms retain manual opening and tray/notification fallback.
