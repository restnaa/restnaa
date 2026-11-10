> Historical PausePal record. Restnaa is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](OPEN_SOURCE_PLAN.md).

# Desktop UI redesign plan

Authorized by the supplied Complete Desktop UI Redesign brief on 2026-10-21. This supersedes the Welcome navigation requirement only; M5 release gates remain pending.

## Boundaries
Use the three user-supplied screenshots as accepted visual specifications, not data. Implement deep navy surfaces, blue/cyan/violet accents, horizontal Dashboard/Settings/History navigation, compact 3-column dashboard, 2+3 settings arrangement, and expandable history days. Bundle a decorative SVG landscape; no remote media or generated replacement concept. Retain OS title bar and controls; no duplicate mock controls or new capabilities. Existing 1200x820 default and 720x600 minimum remain appropriate; support narrower browser/zoom reflow. No scheduler, database, native plugin, migration, or permission changes. No network status inference or absolute runtime privacy claims.

## Concrete files
- src/App.tsx, src/lib/types.ts, src/components/AppLayout.tsx: three routes, first launch into Settings; preserve native navigation and main focus.
- src/components/PageHeader.tsx, IconBadge.tsx, src/assets/night-landscape.svg, src/styles.css: shared visual system and decorative landscape. Segoe UI/system fonts, local SVG icons, 12px card radii, restrained blue/violet gradients, visible focus, reduced motion, forced colors.
- src/lib/deadlines.ts and src/hooks/useDisplayTime.ts: presentation of backend timestamps only, no reminder dispatch or JS scheduling decisions. Hidden windows stop display refresh.
- src/pages/Dashboard.tsx and existing SchedulerStatus, PauseControls, HistorySummary, DeviceStatus: compact six-panel composition. Preserve active/paused/quiet/disabled/error/open reminder states and category-specific actions.
- src/pages/Settings.tsx, src/components/ScheduleForm.tsx, NativePreferences.tsx, ClearHistory.tsx: unified setup/settings, one explicit save across reminders/work hours; merge latest settings before save to preserve native consent/pause changes; keep all diagnostics and data controls.
- src/pages/History.tsx: category completion summaries plus all-outcome totals and local-date grouped real events; explain latest-100 truncation without inventing missing rows. Delete standalone Onboarding.tsx.
- src/pages/BreakPanel.tsx: shared appearance only; preserve Start/Done/Snooze/Skip/Esc.
- src/**/*.test.*, tests/ui/*.spec.ts: update intentional UI expectations and add behavior regressions.
- docs/ui-redesign-report.md: changes, precise results, fidelity comparison, screenshot evidence, remaining/manual verification.

## Acceptance and tests
Run npm run typecheck, npm run lint, npm test -- --run, npm run test:ui, cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked, and npm run tauri -- build --bundles nsis -- --locked using workspace Rust/MSVC tooling. No concurrent builds writing dist. Verify first-run Settings completion only on successful backend save; existing users remain Dashboard; no Welcome nav; live counts and distinct outcomes; empty vs error; no fake countdown during pause/quiet/disabled/open state; draft save preserves latest startup/pause preferences; confirmation/focus handling. Browser plugin unavailable: use existing Playwright/Edge production-asset fixture (test-only IPC), axe, keyboard, reduced motion/forced colors, 200% zoom, compact 390px, native minimum 720px, default 1200px, reference 1448x1086, and high-DPI. Inspect rendered screenshots against all three supplied references. Native hidden/restart checks on isolated test identity if feasible; never claim mocked IPC as native persistence. Preserve original user data and existing release export evidence. No clean-machine claim.
