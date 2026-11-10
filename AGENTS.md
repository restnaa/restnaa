# Restnaa repository instructions

## Source of truth and scope

Read README.md, CONTRIBUTING.md and the relevant architecture docs before changing behavior. Preserve unrelated changes. Discuss product scope changes before implementation. Before writing implementation code, provide a plan naming concrete files, boundaries, tests and acceptance checks.

Restnaa is a release candidate. Keep installed Windows and clean-machine acceptance separate from automated checks; see docs/RELEASE_AUDIT.md and docs/NATIVE_WINDOWS.md. Packaging does not imply release approval.

## Product boundaries

- Windows-first desktop application targeting Windows 10 and 11, built with Tauri 2. Frontend: React, TypeScript, Vite, and Tailwind CSS. Rust owns scheduling, persistence, validation, and system integrations.
- Offline-only runtime. No server, login, accounts, network calls, analytics, telemetry, cloud sync, remote update service, payments, AI, employee monitoring, or social features in the MVP. Build/release dependency downloads and installer prerequisites are distinct from application runtime behavior.
- Store structured data in local SQLite in the application data directory. No direct SQLite access from frontend code. Store no names, email addresses, medical conditions, browsing history, screenshots, active application names, window titles, or key presses.
- Offer gentle eye-rest and movement reminders. Make no medical, diagnostic, treatment, or exercise-verification claims. Completion records explicit user actions only. Avoid health scores, guilt-inducing streaks, forced locks, full-screen overlays, and forced focus stealing.

## Architecture and scheduling contract

- A single Rust scheduler owns state, deadlines, transitions, and event emission. React renders typed backend state and invokes typed commands; JavaScript timers must never become a reminder source of truth.
- Keep independent eye and movement state. Use monotonic elapsed time at runtime and persisted UTC timestamps for recovery; working hours use the system local timezone.
- Preserve reminder eligibility rules: category enabled, inside working schedule, not paused, no open reminder for that category, and not suppressed by app quiet mode.
- Done and Skip schedule from action time plus the category interval; Snooze schedules five minutes later within a valid work window. Unanswered reminders time out after an initial five minutes and reschedule from dismissal. Outcomes remain distinct.
- Recompute after restart, sleep/wake, time/timezone changes, settings changes, and pause/resume. Never replay missed reminders. Recovery may emit at most one currently eligible reminder for the most overdue category. Prevent duplicates and overlapping break windows; sequencing categories at least two minutes apart is an allowed MVP policy.
- Persist manual pause and pause deadlines. Reject end-of-work time less than or equal to start time in the MVP with an understandable explanation. Validate eye intervals of 10–120 minutes and movement intervals of 15–180 minutes in Rust; defaults are 20 and 45 minutes, weekdays 09:00–18:00.
- Use typed command/event contracts. Do not expose success-returning stubs for unavailable behavior.
- Native integrations must respect Windows DND/Focus Assist. Do not assume reliable suppression detection or Windows toast action buttons. Done/Snooze/Skip belong in the break window; tray access remains the fallback. Verify notification delivery in installed builds.
- Window lifecycle must hide the dashboard to tray on close, provide explicit full Quit, enforce one application/scheduler instance, and keep autostart off until the user opts in.

## Security, storage, and accessibility

- Grant only required Tauri capabilities to named trusted local windows. Scope custom command permissions and plugin permissions narrowly. No unrestricted filesystem, shell, process execution, remote content, or arbitrary SQL access from the UI. Use a restrictive content security policy.
- Validate all command inputs in Rust. Keep migrations versioned and transactional. Preserve versioned schema checks and indexes. Missing databases may initialize; corrupt databases must produce a visible error and must never trigger silent deletion of history.
- Require confirmation before erasing history when that feature is implemented. Request OS permissions only when needed and supported; retain usable UI if notifications are denied.
- Use semantic controls, accessible names, visible focus, keyboard navigation, sufficient contrast, and reduced-motion support. Convey state with text as well as color. Esc must not record completion.
- Never fabricate timers, charts, counters, reminders, or test evidence. Foundation screens should clearly describe unavailable features without implying that reminders are running.

## Verification and reporting

Run relevant checks after changes. Report exact commands, results, failures, and checks that could not run. Never equate a development build with installed notification verification, a local machine with clean Windows 10/11 validation, or a frontend build with a working desktop application.

## Authorized automatic break windows (2026-10-22)

The user explicitly authorized automatic break windows, default On, and explicitly accepted Restnaa pause/work-hours/Quiet mode plus best-effort Windows DND handling. Preserve the default-On setting and persistent opt-out. Do not claim complete DND detection; known suppressed/unknown attention states must not trigger automatic windows. No OS preference changes, forced keyboard focus or always-on-top behavior. Existing explicit window actions may focus as before.
