# Restnaa — Product Requirements Document

Version: 1.0 | Platform: Windows-first desktop | Status: Development-ready MVP specification

## 1. Product summary
Restnaa is an offline-first desktop wellness companion for screen-heavy workers. It provides gentle eye-rest and movement reminders, a small break window with completion/snooze/skip actions, quiet hours, tray controls, and minimal local progress history. It is not a medical product and gives no diagnosis or treatment advice.

**Promise:** Rest your eyes. Move your body. Keep your focus.

## 2. Audience and problem
Primary audience: developers, designers, office professionals and students using Windows PCs for extended sessions. Many forget to rest or move; existing reminders can be intrusive, unreliable or require a browser tab.

**Jobs to be done:**
- Remind me to take healthy breaks without derailing important work.
- Give me control over reminders in meetings and deep-work sessions.
- Show me whether I am actually taking breaks.
- Keep running without accounts, a cloud connection, or keeping a browser open.

## 3. Goals, non-goals, launch criteria
### MVP goals
- Reliable timer engine while the installed app process is running, even with main window hidden.
- Native Windows notifications with opening of an in-app break panel.
- Independent eye and movement reminder schedules.
- Settings saved locally; reminders restore coherently after restart.
- System tray with show/pause/resume/quit.
- Basic local history and seven-day summary.

### Non-goals
No medical claims, posture sensing, webcam, screen recording, background employee monitoring, accounts, syncing, payments, AI, team dashboards, social sharing, auto-updates in the initial MVP, or mobile apps.

### Launch gates
- Installed Windows build passes manual notification tests on Windows 10 and 11.
- No duplicate reminders after sleep/wake, resume, restart, or change of settings.
- App does not notify during configured quiet hours or pause periods.
- A single background process stays active when the main window closes to tray.
- User can unambiguously quit completely, disable notifications and erase history.
- Basic accessibility and keyboard navigation are supported.

## 4. Platform and constraints
- Tauri 2; React + TypeScript + Vite + Tailwind CSS; Rust for scheduling/system integrations; SQLite for structured history. No server.
- Target Windows 10/11; verify WebView2 dependency and packaging on clean Windows machines.
- Windows notifications from the official Tauri notification plugin require an installed application; test **installed builds**, not only dev mode.
- Notification-action buttons are not assumed available on Windows through the official plugin. **Done/Snooze/Skip live in the app's small break window**. A system notification may open that window if click events are supported; always retain tray fallback.
- OS Do Not Disturb/Focus Assist may suppress visible notifications. Do not bypass OS preferences.
- Runtime local timezone: use system local time; handle timezone/DST changes and wall-clock jumps.

## 5. User journeys
### 5.1 First launch
1. Welcome screen: promise and privacy explanation.
2. Choose eye interval (default 20 min; 10–120 min) and movement interval (default 45 min; 15–180 min).
3. Choose working hours (default weekdays 09:00–18:00, editable) and whether to start on login (OFF by default).
4. Explain OS notifications and request permission only if required and supported.
5. Start running. Allow 'Test reminder' to preview. Show a clear tray icon.

### 5.2 Eye reminder
1. Eye interval becomes due during scheduled working hours and not paused.
2. Display a native notification: 'Time for an eye break' with 20-20-20 suggestion.
3. Clicking notification, if supported, opens small break window; tray menu can also open current break.
4. In break window: 'Start 20-second break', 'Done', 'Snooze 5 min', 'Skip'. Mark complete only after explicit user action. No claims that exercise was actually performed.
5. Record outcome and plan next reminder according to policy below.

### 5.3 Movement reminder
As above, with gentle movement choices: stand, walk briefly, shoulder rolls. Display a suggested 2-minute timer; user can complete, snooze or skip.

### 5.4 Pause
Choose pause for 30 min, 1 hour, 2 hours, until tomorrow, or manual pause. Persist chosen deadline and show paused state in tray/dashboard. Resume immediately on request.

## 6. Scheduling contract (critical)
- A single Rust scheduler owns schedule state, deadlines, state transitions and event emission. Frontend is a view; it must not independently schedule reminders.
- Each category has its own `next_due_at` and `status`. Use a monotonic runtime clock for elapsed waiting plus persisted UTC timestamps for recovery. Recompute on suspend/resume, process restart, OS time/timezone changes, schedule changes and pause/resume.
- Eligibility: feature enabled AND within configured local working schedule AND not paused AND no open reminder for that category AND not suppressed by quiet mode.
- On **Done**: log `completed` and schedule next reminder from completion time + configured interval (avoid immediate re-notification).
- On **Skip**: log `skipped`, schedule next from action time + configured interval.
- On **Snooze**: log `snoozed`, set next due 5 minutes later, but move it to the next valid work window if snooze ends out of hours. Do not count a snooze as a completed break.
- If an alert is unanswered, auto-dismiss and log `unanswered` after a configurable internal timeout (initially 5 minutes); schedule next from dismissal + interval. Do not create repeated notifications every minute.
- If both categories become due within 2 minutes, show one combined reminder, but retain distinct category outcomes and deadlines; MVP may instead sequence with at least 2 minutes separation if easier to implement safely.
- On waking from sleep or after extended app downtime, NEVER replay missed reminders. Re-evaluate eligibility and show at most one reminder for the most overdue category, only if currently within schedule; otherwise set upcoming eligible deadlines.
- While Windows DND/Focus Assist is on, OS may hide alerts. Do not claim to detect all such cases. App continues scheduling; allow quiet modes within app.
- No automatic full-screen overlays or forced break locks.

## 7. Screens and UX
### 7.1 Dashboard
- Compact friendly header with current state: Active / Paused / Outside work hours.
- Next eye break and next movement break; progress indicators.
- Main buttons: Pause, Resume, Take break now.
- Today summary: completed eye and movement breaks, skipped count.
- Quick settings link.

### 7.2 Break window
- Small centered or corner window; never steals focus forcibly.
- Category icon, friendly one-line prompt, optional countdown.
- Controls: Start, Done, Snooze 5 min, Skip.
- Esc closes panel but does not falsely mark completed; define closing as unanswered after timeout.
- Respect reduced-motion preference and accessibility contrast.

### 7.3 Settings
- Independent toggles and intervals, work days/hours, autostart opt-in, pause defaults, notification test, data deletion, app version.
- Changes apply immediately and persist.

### 7.4 History
- Simple counts for today and previous seven days, broken down by category/outcome.
- Avoid guilt-inducing 'failed' streaks. No health score or diagnosis.

### 7.5 Tray
Show Restnaa; Take eye break; Take movement break; Pause [30m/1h/2h]; Resume; Settings; Quit.
- Closing dashboard hides to tray. Quit terminates app.
- Single-instance enforcement brings existing app forward instead of launching a second scheduler.

## 8. Data model (SQLite)
### settings
- key TEXT PRIMARY KEY
- value_json TEXT NOT NULL
- updated_at_utc TEXT NOT NULL

Suggested keys: eye_enabled, movement_enabled, eye_interval_minutes, movement_interval_minutes, work_days, work_start_local, work_end_local, autostart_enabled, paused_until_utc, paused_indefinitely, onboarding_completed, notification_enabled.

### reminder_state
- category TEXT PRIMARY KEY CHECK(category IN ('eye','movement'))
- next_due_at_utc TEXT NOT NULL
- open_reminder_id TEXT NULL
- updated_at_utc TEXT NOT NULL

### break_events
- id TEXT PRIMARY KEY (UUID)
- category TEXT NOT NULL CHECK(category IN ('eye','movement'))
- outcome TEXT NOT NULL CHECK(outcome IN ('completed','skipped','snoozed','unanswered'))
- triggered_at_utc TEXT NOT NULL
- acted_at_utc TEXT NOT NULL
- duration_seconds INTEGER NULL
- created_at_utc TEXT NOT NULL

Indexes: break_events(created_at_utc), break_events(category, created_at_utc).

Store no name, email, medical conditions or browsing history. Local app data directory; no telemetry by default.

## 9. Architecture and implementation boundaries
### Frontend
React views for onboarding, dashboard, break panel, settings and history. Use typed invoke APIs; UI calls backend commands and subscribes to typed scheduler events. No direct SQLite access from untrusted UI code.

### Rust core
Modules:
- scheduler: central state machine and timer loop
- storage: SQLite migrations and persistence
- settings: validation and constraints
- notifications: native Windows notifications and capability checks
- tray: tray actions and window lifecycle
- startup: autostart management
- clock: injectable time abstraction for deterministic tests
- commands/events: typed interface to frontend

Suggested commands:
`get_dashboard_state`, `get_settings`, `update_settings`, `start_break`, `complete_break`, `skip_break`, `snooze_break`, `pause_reminders`, `resume_reminders`, `get_history`, `test_notification`, `clear_history`.

Events: `scheduler-state-changed`, `reminder-due`, `break-event-recorded`, `settings-updated`.

Apply least-privilege Tauri capabilities: only the app's trusted windows get needed commands/plugins. Validate settings in Rust. Do not enable unrestricted filesystem or shell access.

## 10. Edge cases and verification
- App launched twice -> one scheduler instance.
- Close main window -> tray persists; menu Quit -> process exits.
- Resume after sleep of 1 h -> never burst 3+ missed reminders.
- User changes clock forward/back 2 h -> deadlines are recomputed predictably.
- Work window crosses midnight -> correct local eligibility (either support explicitly or reject invalid windows in MVP; preferred MVP: reject end <= start and explain).
- Notification permission disabled -> dashboard/tray remains usable; clear explanation.
- Clean Windows install -> notifications displayed with proper app identity.
- A second category due during break -> queue/separate and avoid overlapping windows.
- Database missing/corrupt -> recover with user-visible warning, never silently erase history.
- User chooses manual pause, closes app, relaunches -> remains paused.
- Battery/CPU: benchmark idle resource usage; no polling loop tighter than necessary.
- Keyboard-only operation, screen reader labels, reduced-motion support.

## 11. Security and privacy
- Offline-only; no analytics or network calls in MVP (dependencies and installer excluded at build/release time).
- No medical advice, diagnostic claims, or tracking of sensitive health symptoms.
- Optional autostart, never enabled silently.
- Explicit clear-history action with confirmation.
- Avoid collecting window titles, key presses, active application names or screenshots.
- Release builds signed when feasible; review Windows SmartScreen reputation; distribute checksum.

## 12. Milestones and acceptance tests
### M1 — Foundation
Bootstrap Tauri 2 + React/TS + Tailwind; create navigation/layout, Rust command skeleton, SQLite migration, lint/test and CI setup. App launches and builds on Windows.

### M2 — Scheduler engine
Implement Rust scheduler, clock abstraction, persisted settings/state, pause, per-category reminders, edge-case tests. Unit tests exercise scheduling without GUI.

### M3 — OS integration
Native notifications, tray, autostart opt-in, single-instance, window lifecycle and packaged Windows verification. OS notification test works on installed build.

### M4 — Polished workflows
Onboarding, dashboard, break window, controls, settings, history, keyboard UX. Every button has an actual handler.

### M5 — Release validation
Test all cases, package NSIS exe, verify clean Windows installation/uninstall, test 7-day dogfooding, create release notes and privacy statement.

## 13. Core metrics (local beta feedback, consent-based)
- Installed beta testers; weekly active users; percent with reminders enabled at day 7/day 30; snooze/skip/completion distribution; reminder delivery issues; reports of disruptive notifications. No passive transmission; gather feedback voluntarily.

## 14. Open decisions and defaults
- Name Restnaa is provisional; check trademark/domain before public launch.
- Windows 10/11 support must be tested on clean machines, including WebView2 installation path.
- Use notification click-to-open only where supported; tray is reliable fallback.
- Start with fixed 5-minute snooze and configurable work schedule; adaptive/idle-aware scheduling later.
- No remote update service in MVP; when adding signed updates, protect signing keys and follow official updater guide.

## 15. Definition of done
An installed user can start with zero account setup, configure both reminder types, close UI to tray, receive reminders while other apps are foregrounded, choose Done/Snooze/Skip via break panel, pause and resume, view local history, restart/sleep without reminder storms, and uninstall cleanly.

## 16. Authorized automatic break panels (2026-10-22)

The user explicitly requested automatic presentation of the existing break panel and default On. New and legacy profiles default showBreakWindow to true; explicit false persists. A newly due eligible reminder may show the existing single panel without activation or keyboard-focus changes. No always-on-top or forced locks. Notification and tray access remain fallbacks; explicit Take break/Open current break actions retain normal focus behavior. Closing/Esc does not complete a break and does not immediately reopen it; existing unanswered timeout applies.

The user explicitly accepted Windows DND as best effort after observing an accepting shell hint while actual DND was On. Known blocked or unknown attention states suppress automatic panels; the accepting hint cannot prove DND off. Restnaa Pause, work hours and Quiet mode are authoritative; use them for guaranteed app quiet. Native notifications must be enabled, OS-denied notification states suppress panels, and no Windows preference is changed or bypass API used. Windows 10/11 remain the primary targets; non-Windows automatic presentation is not claimed verified.
