# Restnaa release audit

## Unreleased water reminders (2026-11-14)

Water is a third category in the single Rust scheduler, enabled by default every 60 minutes with an independent 30–240 minute setting and opt-out. NHS/CDC guidance reviewed for this feature supports regular drinking and individual needs, not a universal minute interval. The default/range are app preferences, not medical advice. No quantities, fluid goals or health scores were added. Source references and scheduling policy are in SCHEDULER.md.

Schema 4 transactionally extends category constraints and preserves existing rows/indexes. A migration marker lets Rust initialize only the new water deadline from the injected clock; the first scheduler save atomically persists it and clears the marker. Tests cover older profiles, preservation of eye/movement deadlines and history, rollback on migration failure, missing-row corruption, independent water outcomes/history, quiet hours, pause persistence, restart/time jumps and all three categories due together. Existing window permissions/CSP and automatic nonactivating presentation policies remain unchanged. No personal profile was migrated during tests.

Executed checks: `npm run typecheck`, `npm run lint`, `npm test -- --run --maxWorkers=1` (66 passed); `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked` (63 passed); `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings`; `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`; `git diff --check`: passed. Clippy initially flagged the expanded state-event enum size; boxing the state payload fixed it without changing the serialized contract. Initial frontend checks found a category-metadata naming conflict and an assertion assuming one disabled category; both were corrected. One early Vitest run reported a fork-worker startup timeout; the final default-worker run completed without errors. A new Rust suppression test initially bypassed the pause command; its setup was corrected and the full suite passed.

`npm run test:ui -- --workers=2`: 40 passed in Edge using the production build and IPC fixtures, including ten new water workflow cases across desktop/compact projects, all-category accessibility scans, keyboard outcome actions, Escape and existing layout/forced-color/reduced-motion checks. Browser plugin was unavailable; the repository Playwright path was used. The test flow was dashboard → water reminder → typed Start/Done/Snooze/Skip/Hide calls, plus Settings interval/toggle persistence. No framework errors or failed bundled scripts/styles were reported by the browser fixture. The dashboard capture at `C:/OracleTemp/restnaa-redesign/dashboard.png` was visually inspected; it uses synthetic test state, not an installed user profile. No public gallery screenshots were replaced with test data.

These changes are local and unreleased on `feat/water-reminders`. Installed water notifications, native tray/menu presentation and physical window dismissal have not been verified for this feature. Clean Windows 10/11 and native DND remain separate pending gates. Back up the stopped profile before trying schema 4; older builds require restoration of a pre-upgrade backup to roll back.

`npm run tauri build -- --bundles nsis -- --locked` passed. The local, unsigned installer at `src-tauri/target/release/bundle/nsis/Restnaa_0.1.4_x64-setup.exe` is 4,118,176 bytes with SHA-256 `e4f1b939e8adc9b1cd762e324441832c45e30ff542790d78b33575b2886aee5a`; explicit signature inspection returned `NotSigned`. This dirty-worktree candidate replaces the prior local target bundle, not the exported `release/` folder or published Beta 2 asset. Do not reuse the older exported manifest/checksum. No installation or public release was performed.

## Unreleased security and history changes (2026-11-13)

These local source changes are not in the published Beta 2 installer. `rusqlite` 0.41.1 bundles `libsqlite3-sys` 0.39.1, Tokio is locked to 1.54.1 and rustix to 1.1.6. The yanked non-Windows async-executor 1.14.0 entry was updated to 1.14.1. npm's targeted minimatch override resolves lint tooling to 11.0.8. The refreshed advisory database (1,509 advisories, last updated 2026-11-13) reports zero Rust vulnerabilities; five warnings remain for proc-macro-error, zbus_names, zvariant, zvariant_derive and glib. Windows-target inverse dependency checks find none of those packages. No Linux/macOS security qualification is claimed.

Schema 3 adds history timestamp-expression and invalid-timestamp partial indexes transactionally. Tests verify a schema-2 upgrade preserves records, a failed index creation rolls back the first index and schema version, and a 10,000-old-record history query uses an index without a temporary sort. Existing mixed-offset, fractional timestamp, local-day/DST, corrupt-storage and distinct-outcome tests pass. No personal database was opened or migrated during these checks. Older Beta 2 requires restoration of a stopped pre-upgrade backup to roll back.

Executed checks:

- `npm ci --no-audit`: passed with the updated lockfile.
- `npm audit --json --fetch-retries=0 --fetch-timeout=30000`: zero findings, including development dependencies.
- `cargo-audit audit --file src-tauri/Cargo.lock --json`: refreshed the advisory database and reported zero vulnerability findings; platform/upstream warnings are retained rather than ignored. Earlier `--no-fetch` checks were also performed.
- `npm run lint`, `npm run typecheck`, `npm test -- --run --maxWorkers=1`: passed; 63 frontend tests in 14 files.
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked`: 55 tests passed, including three new history/migration regressions.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings` and `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: passed.
- `npm run build`: passed. `npm run test:ui -- --workers=2`: initial web-server startup exceeded 120 seconds while native compilation was running; a retry after an explicit build passed all 30 Edge tests at desktop/compact sizes, including accessibility checks. Browser plugin was unavailable, so the repository Playwright path was used. These tests use an IPC fixture, not the native scheduler.
- Actionlint 1.7.7 passed both workflows. Actions are full-SHA pinned, checkout does not persist credentials and job timeouts are bounded. Dependabot configuration was reviewed for weekly Actions/npm/Cargo proposals; it has not run on GitHub. Auxiliary YAML parser checks could not run because js-yaml/PyYAML were absent; workflow syntax validation used Actionlint and text assertions verified pins/settings instead.
- `powershell -NoProfile -File scripts/test-release.ps1`: passed. `git diff --check`: passed.
- `npm run tauri build -- --bundles nsis -- --locked`: passed with the final lockfile after the async-executor patch; all 55 Rust tests also passed again. The local installer is `src-tauri/target/release/bundle/nsis/Restnaa_0.1.4_x64-setup.exe`, 4,118,572 bytes, SHA-256 `664fac14dae219abca49a7b22b087f067ec4a61c4d7b32a6c2cc59c849503dae`, Authenticode `NotSigned`. It was built from a dirty local worktree, has not been installed for acceptance testing, and was not exported over the existing `release/` candidate or uploaded to GitHub. Do not pair it with the old exported manifest/checksum.

GitHub's public branch API still reports `main` unprotected. Protection remains a repository-admin setting and was not changed by these source edits. Installed Windows behavior, visible notifications, clean Windows 10/11 and idle resource usage were not retested; previous native evidence does not certify this candidate. No new release was published.

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

## Water candidate and signed-publication gate (2026-11-14)

The user reported that the local water-reminder candidate works. This is user feedback, not independent installed, signature or clean-machine acceptance. Water changes were pushed as 67f9fbf on feat/water-reminders. Merged origin/main at 578e0ca, retaining its dependency updates and loading-copy fixes, in merge commit 00a9012.

Combined-source verification passed: npm run lint; npm run typecheck; npm test -- --run --maxWorkers=1 (68 tests); npm run test:ui -- --workers=2 (40 tests, including production build); cargo fmt --manifest-path src-tauri/Cargo.toml -- --check; cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked (63 tests); cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings. Rust used the repository-local toolchain and x64 Visual Studio BuildTools environment. Browser plugin unavailable; repository Playwright/Edge used. Current Cargo audit reported zero vulnerabilities; cross-platform advisory warnings remain separate from Windows acceptance.

Added a public-release gate requiring valid, timestamped Authenticode signatures on both restnaa.exe and the NSIS installer, plus signature statuses in the release manifest. General CI/local packaging may still export unsigned test candidates. powershell -NoProfile -File scripts/test-release.ps1 passed, covering unsigned, invalid/untrusted and missing-timestamp rejection; the positive fixture is synthetic and does not establish real signing. The actual existing unsigned local installer was rejected by the gate. Actionlint passed both workflows; package-release.ps1 parsed successfully; git diff --check passed.

Successful certificate-store inspection found zero code-signing certificates in CurrentUser/My and LocalMachine/My. No signing provider is configured. Trusted signing requires the maintainer's certificate or service and identity verification; no self-signed substitute or Windows trust changes were made. No new release tag or public release was created. The existing local unsigned installer predates the merged main updates and is not a signed artifact for this combined source. No new installer was built in this signing-preparation step. Signed publication remains blocked on signing-provider setup and the outstanding installed acceptance checks above.
