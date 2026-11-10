> Historical PausePal record. Restnaa is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](OPEN_SOURCE_PLAN.md).

# Default-on automatic break-window plan

User authorization, 2026-10-22: add automatic break windows, enabled by default. Existing product boundaries remain; no focus stealing, lock screen overlay, cloud, telemetry, new reminder engine or backlog replay.

## Resolved DND decision

AGENTS.md requires Windows DND/Focus Assist respect. Installed evidence in NOTIFICATION_TROUBLESHOOTING.md showed DND On with an accepting SHQueryUserNotificationState hint. That hint and FocusSessionManager alone cannot guarantee manual DND suppression of a custom app window. Automatic app windows are not Windows toast banners. No registry/WNF reverse engineering, Windows Settings UI scraping in production, or OS preference changes will be introduced.

User decision received: best-effort Windows DND is explicitly accepted. The earlier question was: accept app pause/work-hours/Quiet mode as authoritative controls plus best-effort Windows suppression hints, or keep notification-only until a reliable DND guard is available. Decision resolved; proceed with the default-on preference and honest best-effort wording. The user has authorized default On, not silent disregard of OS preferences.

## Implementation scope

- src-tauri/src/settings.rs: persisted show_break_window boolean, camelCase showBreakWindow, serde default true for old profiles and default true for fresh profiles; explicit false survives restart. Existing SQLite JSON storage needs no schema migration.
- src/lib/types.ts and src/test/schedulerFixtures.ts: typed setting and fixture default.
- src/components/ScheduleForm.tsx: checkbox Show break window when a reminder is due, default from Rust; include field in explicit save while preserving latest native/pause preferences. Brief honest text: no keyboard-focus stealing; app Quiet mode/pause always apply; Windows DND detection best effort.
- src-tauri/src/native/model.rs: pure tested automatic-window policy for new due UUID, preference, active/current reminder, app notification preference/OS denial and attention hint. Never reopen on ordinary state ticks, settings refresh, user closing the panel, or category duplicates. No window overlap; scheduler's existing single open reminder and two-minute sequencing remain authoritative.
- src-tauri/src/native/mod.rs: only automatically due effects call presentation policy after committed state. Keep native notifications/tray fallback; no mutation of scheduler deadlines or outcomes for merely showing a window.
- src-tauri/src/native/windows.rs and Cargo.toml: separate automatic nonactivating show from explicit user-requested show_break (which may focus). Windows uses documented ShowWindow SW_SHOWNOACTIVATE on the existing break HWND, no SetForegroundWindow, forced focus, always-on-top, new windows or capability grant. No automatic focus behavior claimed on untested platforms.
- src-tauri/tests/native.rs, scheduler/storage tests: true defaults from older JSON, explicit false persistence, both categories, pause/quiet/off-hours/OS-denied/suppression states, duplicate effects and no overlap; existing completion/skip/snooze/timeout behavior unchanged.
- src/components/ScheduleForm.test.tsx and tests/ui: checkbox save/reload behavior and preservation of startup/pause fields. Run frontend unit/type/lint/UI and Rust all-target tests, strict lint as appropriate.
- docs/NATIVE_WINDOWS.md, new verification report, release documentation: exact default behavior, DND limitation, tests and pending OS/manual gates.

## Installed Windows acceptance

Build NSIS and test a separate disposable installed identity. Verify a real autonomous due eye and movement reminder opens one break window while dashboard is hidden. Verify foreground HWND remains the same and keyboard focus is not moved; click Done/Snooze/Skip, close/Esc and timeout without phantom completion. Verify disabled toggle survives restart and prevents automatic opening; pause/Quiet mode/work hours and known Windows suppression states prevent showing. Never alter the user's actual schedule/history or OS DND. Record any physical DND/manual checks as pending and do not infer them from mocks. Preserve the normal installed user's app/data; export a separately named candidate with checksum and precise install instructions. Do not call clean-machine testing passed.

Send test reminder remains an OS-notification test; Dashboard Take eye/movement break remains the explicit break-panel test. A fabricated scheduled reminder or fake outcome will not be created for a notification test.

## Candidate identity

Package this change as 0.1.1 so the About version clearly distinguishes it from earlier 0.1.0 candidates. Update package.json/package-lock.json, src-tauri/Cargo.toml/Cargo.lock and tauri.conf.json version metadata only; keep com.pausepal.desktop and the existing SQLite profile. Export separately under release-auto-break/ with checksum, preserving old release exports.

## Installed startup integration repair

The first installed QA launch reproduced the earlier connection-error screen while direct IPC later succeeds. Before shipping, add src/lib/startup.ts and startup.test.ts: a bounded, read-only get_app_info readiness handshake before React App/BreakPanel mount in src/main.tsx. Retry only this idempotent metadata read for at most five seconds; never retry actions/settings/history deletion. Browser preview skips the native handshake; deadline/scheduler ownership stays Rust-only. Render a truthful starting status, then mount the existing error-capable UI even if readiness times out. Verify actual cold installed launch without reload, bounded cleanup and no fake readiness.

## Verification outcome — 2026-10-22

Implemented default On for new/legacy JSON settings, persisted opt-out, typed UI/save integration, native due-effect presentation only, per-UUID duplicate guard, and documented SW_SHOWNOACTIVATE with no focus/foreground/topmost calls in the automatic path. Existing explicit window actions still focus normally. No scheduler-engine deadline policy, migration, command capability or Windows preference was changed. The native hint wording now explicitly says DND may still be on.

| Check | Result |
| --- | --- |
| npm run lint | Passed on final source |
| npm run typecheck / production build | Passed |
| npm test -- --run --maxWorkers=1 | 60 passed across 14 files |
| npm run test:ui -- --workers=2 | 30 passed; final startup-handshake run 1.2 minutes |
| cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline | 51 passed, including three new native/settings regressions |
| cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline -- -D warnings | Passed |
| cargo fmt --manifest-path src-tauri/Cargo.toml | Applied; Rust sources formatted |
| npm run tauri -- build --bundles nsis -- --locked | Passed on final normal 0.1.1 identity |

Browser plugin unavailable: used existing Playwright/Microsoft Edge production-asset fixture at http://127.0.0.1:1420 (desktop 1200x800, compact 390px, reference 1448x1086, minimum 720px, 200% zoom, forced colors/reduced motion, high DPI). The new switch defaults checked, explicit off saves through typed update_settings and remains off after reload. Existing navigation, validation, pause, outcome, confirmation, notification and accessibility regressions pass. Native behavior below uses actual Rust IPC rather than the browser fixture.

### Installed Windows evidence

Installed a separate PausePal Auto Break QA 0.1.1 identity (com.pausepal.autobreakqa), with its own SQLite data, on this existing Windows host. NSIS install and uninstall returned 0. Startup remained off. No clean-machine validation is claimed.

1. Fresh QA settings defaulted Show break window On; actual Settings setup saved both categories, all weekdays, 00:00–23:59 hours, 10/15-minute intervals and notifications On.
2. Only while this disposable QA process was stopped, set its persisted next_due_at 35 seconds ahead and cleared its cooldown. No event rows or user profile settings were fabricated. Startup with --autostart hid the dashboard. Read-only SQLite inspection before any scheduler command found the Rust-created current reminder.
3. Eye and movement both automatically displayed the existing break window with main hidden. Done recorded one actual eye completion. Escape hid movement without falsely recording completion; the open UUID remained. Restart later recorded it unanswered through the existing recovery policy. A separate actual movement popup supported Skip.
4. On the final QA build, a Settings interval input was selected before an autonomous movement popup. The input selection remained; 420 native foreground samples (100ms intervals) never observed the break HWND as foreground. OS foreground changed to another handle during the watch, so **uninterrupted original-foreground identity was not proven**. This is evidence of no observed popup activation, not a claim that user/other-window focus never changed. WebView document.hasFocus reported true for both pages, so an initial QA assertion using that flag failed and was replaced by the authoritative native activation observation. Actual physical typing across the popup remains a useful manual check.
5. Explicitly saved the switch Off through the installed Settings UI, quit and restarted. With a new due QA reminder, Rust still opened its reminder state while both windows stayed hidden. Off persisted; no automatic panel activated. Notification/tray fallback remains available.
6. The first installed build reproduced the historical early-IPC connection-error screen. Added the bounded metadata-only handshake and its success/failure/hung-read tests. The corrected installed build cold-started successfully without reload. Mutation commands are never retried; after five seconds the normal error-capable UI mounts rather than inventing readiness.
7. Uninstalled QA, removed its debugging listener and left no QA startup entry. Original user preferences compared equal and all pre-test user event IDs remained present. The normal user installation was not overwritten or launched by QA. The disposable QA SQLite data and screenshots were retained as evidence.

Earlier verification issues: one typed test assertion and a test Session import were corrected; initial UI server startup consequently failed before tests. One highly concurrent frontend run timed out starting seven workers (20 tests ran, not a passing suite); subsequent full and serial runs passed. Nonfatal MSVC vswhere lookup and Vite plugin-timing/NO_COLOR notices occurred; release linking succeeded. No unresolved application test failure was hidden.

Screenshots inspected with view_image: C:/OracleTemp/pausepal-auto-break/eye-automatic.png, movement-automatic.png, final-automatic.png. These show actual disposable native reminders. Settings screenshot from browser validation remains in C:/OracleTemp/pausepal-redesign/settings.png and uses explicitly test-only fixture values. No application screen-capture feature was added.

### Distribution and remaining limits

Final installer: release-auto-break/PausePal_0.1.1_x64-setup.exe, with .sha256 and manifest.json. Quit older PausePal completely before installing, then launch from Start. Existing identity/profile is retained. Verify About version 0.1.1; the new setting defaults On for old profiles unless explicitly saved false by this version. Notifications must also be enabled. Test the panel immediately with Dashboard Take eye/movement break; Send test reminder remains an OS-notification-only test and does not fabricate a scheduled event.

Back up the stopped local SQLite database before upgrading a candidate you rely on. Older 0.1.0 builds reject unknown settings fields, so downgrading after 0.1.1 writes showBreakWindow requires restoring a pre-upgrade profile backup; do not erase history to force a downgrade.

Windows DND is best effort as explicitly approved. Use app Pause or Quiet mode for guaranteed app suppression. No OS DND, registry toggle, undocumented WNF/COM API, login, network/telemetry, AI, payments or team tracking was added. Automatic display is Windows-only; manual/tray fallback remains elsewhere. Existing M5 clean Windows 10/11, screen-reader, physical power/time/session, signing and runtime-egress review gates remain open. This unsigned 0.1.1 candidate is not public-release approval.
