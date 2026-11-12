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

## Logo and palette update (2026-11-12)

The supplied logo is retained in public/brand; the cropped crescent-and-eye mark is used by the app, favicon and Windows bundle icons. Dashboard, Settings, History and break panels use teal/sage styling. No Rust scheduler or command implementation changed.

Executed: npm run lint; npm run typecheck; npm test -- --run --maxWorkers=1 (63 passed); npm run test:ui -- --workers=2 (30 passed, including a production Vite build, WCAG checks, release-CSP asset loading, keyboard, forced colors, reduced motion, compact and high-DPI views). Browser plugin unavailable; Playwright/Edge used at 127.0.0.1:1420. Original logo and rendered desktop/compact captures were visually compared.

The first cargo invocation could not find cargo on PATH. With the repository-local Rust environment, the initial compile selected an incomplete Community C++ toolchain and failed on missing excpt.h; its vcvarsall.bat was also missing. Selecting the complete x64 BuildTools environment resolved this. cargo build --manifest-path src-tauri/Cargo.toml --release --locked passed. This direct Cargo build uses the local preview URL; desktop captures used the production Vite assets there and the real Rust IPC backend. Original schedule settings were restored exactly after the break-panel capture, and the app was quit. README images show app content without Windows title bars.

No NSIS installer or installed/clean-machine validation was performed for this visual update. Installed tray/taskbar/toast icon appearance and the existing OS acceptance gates remain pending.

## Simplified light identity (2026-11-12)

Supersedes the teal/sage visual direction above. The new supplied logo is preserved byte for byte in public/brand/restnaa-logo.png. Main background: #F0F8EA; primary text/actions: #260C1A; secondary details: #432E36. Removed the landscape asset, gradients, glows and repeated decorative badges. Break cards now group icon, title and backend status together. Settings, History and both break categories retain existing commands and outcomes.

Executed: npm run lint; npm run typecheck; npm test -- --run --maxWorkers=1 (63 passed); npm run test:ui -- --workers=2 (30 passed, including production build, exact palette and release-CSP logo loading, WCAG checks, keyboard, reduced motion, forced colors, compact and high-DPI layouts). Browser plugin unavailable; Playwright/Edge used. Compared the design concept with desktop and compact captures for palette, logo, navigation, flat panels, typography and button hierarchy. Visible copy remains unchanged; status text and counts come from backend state.

Desktop build passed in the x64 BuildTools environment with repository-local Rust: cargo build --manifest-path src-tauri/Cargo.toml --release --locked --features tauri/custom-protocol. Actual app-content screenshots from this build refreshed the README gallery. The real Rust backend handled the temporary work-hours change used to open the eye-break panel; exact original settings were verified after restoration and the app was quit. No history rows were deleted. This visual capture does not certify installed notification delivery or clean-machine behavior. No NSIS rebuild or installed OS acceptance was performed for this update.
