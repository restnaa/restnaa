# Restnaa release audit

Earlier internal plans and exact test evidence are preserved in Git history before 0.1.4. They do not certify the current identity. Current executed checks are recorded in the verification summary below.

## Current candidate

Version 0.1.4; Windows 10/11 x64; Tauri 2; identifier com.restnaa.desktop; restnaa.exe; %APPDATA%/com.restnaa.desktop/restnaa.sqlite3. Unsigned release candidate. Source publication is separate from stable binary approval.

## Required acceptance

- Deterministic scheduler, restart, monotonic/wall-clock changes, working hours, pause persistence, spacing and no backlog storms: covered by deterministic automated tests.
- Installed notifications and denial/suppression diagnostics under the new app identity: pending.
- Installed tray, close-to-tray, explicit Quit, one process/scheduler, opt-in startup and actual sign-in: pending.
- Automatic nonactivating panels and native hide on Done/Close/Escape/X: regression policies tested; current installed acceptance pending.
- Storage corruption must fail visibly without deleting data; fresh data directory and uninstall retention require current installed acceptance.
- Clean Windows 10/11 including missing WebView2, physical sleep/wake/session/timezone changes, screen-reader and seven-day use: pending.
- Idle resource usage and embedded WebView2/Windows egress: current identity measurement pending. Earlier internal runtime-tree HTTPS observation prevents claiming strict zero egress.
- Signing, publisher identity, trademark and complete bundled dependency notices: pending.

Record exact commands, OS/build identity, actual outcomes and failures. Never promote a browser fixture or build result into an installed/clean-machine pass.

## Automated verification summary

Local 0.1.4 validation recorded 63 frontend unit tests, 30 browser workflow tests and 52 Rust tests passing, plus lint, typecheck, formatting, Clippy and an NSIS build. These are automated/local checks, not installed acceptance.

Hosted run 42730323647 passed dependency installation, frontend checks and Rust checks and produced the NSIS installer. Release export then failed because Get-FileHash was unavailable. The packaging script now uses .NET SHA-256; a subsequent hosted run must verify the repair. No clean-machine or installed Windows pass is implied.
