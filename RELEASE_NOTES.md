# Restnaa 0.1.3 ? open-source preparation

Renamed the visible desktop app, windows, notifications, tray, package metadata and installer to Restnaa. Added MIT licensing, contribution/security guidance and GitHub templates. Retained com.pausepal.desktop, pausepal.sqlite3 and pausepal.exe for legacy compatibility. No scheduler, migration or outcome-policy changes. The native automatic-window dismissal repair remains intact.

This is an unsigned candidate, not a stable release. Installed rename/upgrade and clean Windows acceptance remain pending. Historical notes below preserve original names and evidence.

# PausePal 0.1.0 — release candidate

Date: 2026-10-21. Windows x64, per-user NSIS installer. **Not approved for public release:** clean Windows 10/11, seven-day dogfooding, and outstanding manual OS gates remain open. See [release audit](docs/RELEASE_AUDIT.md) and [known limitations](docs/KNOWN_LIMITATIONS.md).

## Included

- Offline Tauri 2 desktop app with a single Rust scheduler and local SQLite.
- Independent eye/movement intervals, local working schedule, quiet mode, timed/manual pause and restart/clock recovery without replaying missed reminders.
- Native Windows notifications, tray fallback, close-to-tray, explicit Quit, single instance, and explicit opt-in startup.
- Onboarding, settings, small break panel with explicit Start/Done/Snooze/Skip, and real today/seven-day history with confirmed deletion.
- Keyboard navigation, reduced-motion styles, compact outcome cards, and automated accessibility checks. No simulated timers, charts, health scores, account, telemetry or remote updater.

## Release-audit repairs

- Clock recovery preserves monotonic delivery spacing, and restart bounds stale deadlines after a backwards clock adjustment.
- Explicit Quit remains available when storage/native diagnostics cannot load.
- Closing the dashboard after tray setup failure explicitly exits all hidden webviews, preventing an inaccessible background process.
- Invalid history timestamps produce an error instead of silently omitting events from counts.
- Storage readiness keeps definition-list semantics while announcing status changes.
- Installed-notification evidence can compare with a pre-test snapshot, rejecting an old toast as proof of a fresh delivery.
- Release browser tests exercise production assets and capture failed script/style requests rather than relying on the development server.

## Installation and data

Use [installation instructions](docs/INSTALLATION.md). Verify the installer against the accompanying SHA-256 file. The candidate is unsigned; do not interpret it as a signed/trusted-publisher release. Existing preferences/history are retained on upgrade. Startup remains off until explicitly enabled. Notifications can be suppressed by Windows, with tray access retained. Toast action buttons are not used.

Core workflows use local data without a server. Zero runtime egress remains unverified because a process-tree HTTPS connection was observed; see privacy/audit. Installation may download Microsoft's WebView2 prerequisite when missing. Review [privacy](PRIVACY.md) for local storage, retention and deletion behavior. Completion records an explicit button action, not verification that an exercise occurred. PausePal makes no medical claims.
