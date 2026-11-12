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

## Beta 1 packaging and installed smoke (2026-11-12)

Prepared v0.1.4-beta.1 with the simplified light/plum UI. npm run package:windows passed using the x64 BuildTools environment and repository-local Rust. The local unsigned NSIS installer was 4,095,664 bytes; independently verified SHA-256 a732a609120ced3cfd5dabce4432f5b5ef0b4eaed0c796065e01abc7b25ab598. This is the local test artifact; hosted release assets are built independently and carry their own checksum/manifest. Earlier frontend checks remain 63 unit and 30 browser tests passing for the unchanged application code. Official actionlint 1.7.7 (download checksum verified) passed both CI/release workflows; git diff --check passed.

Installed the local NSIS candidate silently to an isolated test installation directory on the existing Microsoft Windows 11 Pro host, version 10.0.26300/build 26300. Installer exit code 0; uninstall registration and installed executable confirmed. This was not a clean machine. The installed app loaded bundled content at tauri.localhost with schema 2 storage Ready and the expected pale background. The real UI opened an eye break; Done added exactly one completed outcome and cleared the current reminder. Pause/Resume changed committed Rust state. Exact original settings were restored; no history was deleted. A second launch exited with only the original installed process remaining. Quit from Settings stopped the process. Saved and registered autostart were both false. Tray availability was reported by the backend, but menu interaction was not manually observed.

The installed Send test reminder returned submittedUnconfirmed: Windows accepted submission, but visible banner delivery was not established. Initial Windows permission diagnostics were unknown and the attention hint accepting. No permission or DND settings were changed. Break-panel HTML visibility did not provide proof of native window dismissal. The desktop capture helper failed to initialize (setup refresh error), so visible banner/click/DND and tray/close-window acceptance remain pending. Clean Windows 10/11, sign-in startup, physical sleep/wake, screen reader and prolonged-use gates also remain pending.

Publication uses a tag-only workflow. A read-only Windows job validates the tag, runs frontend/Rust checks and builds NSIS. A separate release-write job verifies the source commit, clean worktree, file size and SHA-256; uploads eight assets to a draft; then publishes it as a pre-release. No stored local credentials are extracted. Hosted CI success and actual public release/asset verification must be observed separately before claiming publication.

## Beta 2 export repair (2026-11-12)

The user supplied the hosted error from Beta 1: NSIS completed, but Get-AuthenticodeSignature could not autoload Microsoft.PowerShell.Security. No release was published. The Beta 1 tag remains unchanged.

Signature inspection now imports the security module explicitly from the running host's PSHOME and invokes the module-qualified command. Hosted CI/release packaging runs directly in pwsh instead of nesting Windows PowerShell. Signature failures are not converted into fabricated success. The new PowerShell regression test reproduces an incompatible shadow module, verifies successful inspection through the explicit host module, rejects a missing signature input, and checks SHA-256 against the abc known vector. powershell -NoProfile -File scripts/test-release.ps1 and package-release.ps1 -SkipBuild passed locally; Actionlint passed both workflows. The local installer bytes and installed smoke evidence above remain unchanged. v0.1.4-beta.2 is the publication retry; the application version remains 0.1.4.

## Published Beta 2 verification

Published https://github.com/restnaa/restnaa/releases/tag/v0.1.4-beta.2 from source e38dd583258e34cdb5bd7a0074e99811d934191d. Hosted workflow 43324283994 completed both build and publish jobs successfully, including the PowerShell regression and all frontend/Rust checks. Public API verification confirmed draft=false, prerelease=true and eight uploaded assets: installer, checksum, manifest, installation guide, known limitations, privacy, license and third-party notices.

Downloaded the public installer, manifest and checksum. Verified clean-source provenance against the release commit, file size 4,095,420 bytes, and SHA-256 01ca0cb2dc0fb2889bcd30939e30125f7e11b0790b7dc6ad932ed09b211ff07b. Explicit local Authenticode inspection confirmed NotSigned. This published artifact differs from the earlier local build and was checked independently.

Installed that exact downloaded asset successfully on the existing Windows 11 Pro host. Repeated bundled-frontend/palette, eye-break Done/history increment, pause/resume, exact settings restoration, single-instance and Quit smoke checks: passed. Saved and registered autostart remained false. Windows permission now reported allowed and attention accepting; test notification still returned submittedUnconfirmed, so no visible banner claim is made. Native physical window dismissal, full tray/close behavior, DND/click/denial, sign-in, clean Windows 10/11, sleep/wake, screen reader and prolonged use remain pending. No history was deleted or Windows preference changed. The app was quit after testing.
