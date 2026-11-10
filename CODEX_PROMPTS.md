# Restna — Codex execution prompts

Use these prompts **sequentially** in a working repository. Keep `PRD.md` at project root; make Codex read it before each milestone. Never claim a feature works without executing the relevant tests. Build on Windows for installer verification.

## Prompt 0 — Repository instructions
Read `PRD.md` carefully. Create `AGENTS.md` to codify the product boundaries: Windows-first Tauri 2 desktop app, React/TS + Tailwind frontend, Rust-owned scheduler, local SQLite, offline, no login/network/telemetry, no medical claims. Follow least privilege and accessibility. Provide implementation plan with concrete files and tests before coding. Do not implement beyond M1 yet.

## Prompt 1 — Foundation (M1)
Implement milestone M1 from `PRD.md`. Bootstrap a current Tauri 2 project with React, TypeScript, Vite and Tailwind. Configure a clean Restna desktop dashboard shell, navigation to onboarding/settings/history, Tauri command layer, typed DTOs, and SQLite migrations/storage abstraction. Include basic unit/lint/build scripts and Windows development instructions. Use a coherent visually polished design, but do not fake timers or charts. Add meaningful tests. Run all commands, report results, limitations and exact next steps. Do not implement scheduler or notifications in this step.

## Prompt 2 — Scheduler (M2)
Read `PRD.md` and current implementation. Implement the single Rust-owned scheduler as an explicit tested state machine with injectable clock, independent eye/movement deadlines, configured working hours, pause/resume, handling snooze/skip/completed/unanswered, persisted next_due_at timestamps, and robust restart/suspend/time-change policies. No JS interval should be the source of truth. Handle simultaneous reminders without overlap. Test both categories, quiet hours, sleep/wake, clock jumps, restart, pause persistence, settings changes and no backlog storms. Wire typed commands/events. Run cargo tests and frontend tests; report failures precisely.

## Prompt 3 — Native Windows behaviour (M3)
Implement Windows notification plugin integration, tray icon/menu, hide-to-tray on window close, explicit Quit, single-instance prevention and opt-in Windows autostart. Use official Tauri v2 plugin capabilities with minimal privilege. IMPORTANT: official notification actions are mobile-only; do not add fake Windows toast action buttons. Instead notification click may open a small break window where Done/Snooze/Skip are implemented; tray menu must always work as fallback. Verify notifications on an **installed** Windows build, not only development mode. Record any unverified behaviours as pending tests. Add diagnostics for denied/suppressed OS notifications without circumventing Windows DND.

## Prompt 4 — UX and local history (M4)
Implement functional onboarding, dashboard, eye/movement break window, setting toggles/intervals/work schedule, time-limited/manual pause, progress history (today/7 days), test notification and confirmed clear history. Use typed Rust commands. Keyboard accessibility, reduced-motion support, and respectful low-distraction UI are required. Distinguish completed, skipped, snoozed and unanswered events correctly. All buttons must be functional; no mock charts or placeholder counters. Add UI tests and run them.

## Prompt 5 — Testing and packaging (M5)
Read `PRD.md`, inspect implementation, and run a strict release audit: scheduler reliability, session resume, system time changes, single-instance, tray/Quit behaviour, startup consent, native notification from installed Windows build, notification permission denial, storage corruption handling, accessibility and idle resource usage. Repair bugs. Produce a Windows NSIS installer, test on clean Windows 10 and 11 machines where possible, and include release notes, privacy statement, installation instructions, and known limitations. Never claim clean-machine validation unless actually performed. Report test evidence and unresolved risks.

## Prompt 6 — Post-beta improvements (only after evidence)
Use collected and explicitly shared beta feedback to propose the smallest follow-up release. Prioritize annoyance reduction, better timing, and retention. Reject features that add complexity without validation. Do not implement AI, cloud accounts, team tracking, or payment without separate approval.
