# Restnaa

[Source and issues](https://github.com/restnaa/restnaa)

A quiet, offline Windows desktop app that reminds you to rest your eyes and move. Previously named **PausePal**.

Restnaa uses Tauri 2, Rust, React, TypeScript, Vite, Tailwind CSS and local SQLite. A single Rust scheduler owns reminders; React displays its typed state. No accounts, cloud sync, analytics, telemetry implementation or medical claims.

**Status: 0.1.3 release candidate.** Contributions are welcome; stable binary release approval is pending. Windows 10/11 are the targets. Clean-machine testing, signing, installed rename/upgrade, startup and notification identity checks remain open. Existing platform limits are documented in [known limitations](docs/KNOWN_LIMITATIONS.md). Source publication is not a claim of production readiness.

## What it does

- Independent eye/movement intervals, selected working days/hours and Quiet mode.
- Timed or manual pause, automatic nonactivating break panels and a persistent opt-out.
- Native notifications, tray fallback, close-to-tray, explicit Quit and opt-in startup.
- Done, Snooze, Skip and Unanswered recorded separately in local today/seven-day history.
- First-run configuration in Settings, keyboard controls and reduced-motion support.

Windows DND detection is best effort. Use app Pause or Quiet mode for guaranteed app quiet. Completion records your explicit action; it does not verify exercise or eye rest.

## Run on Windows

Install Node.js >=22.12, npm, stable Rust for x86_64-pc-windows-msvc, Visual Studio Build Tools with Desktop development with C++ and a Windows SDK, plus WebView2 Evergreen. See [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/).

From the repository root, in Developer PowerShell for Visual Studio:

```powershell
npm ci
rustup component add rustfmt clippy
npm run tauri dev
```

Use `npm run dev` for frontend preview only. A normal browser cannot run the native scheduler/commands and will show a desktop connection warning. No API key, .env file, backend server or database service is required.

## Test and build

```powershell
npm run lint
npm run typecheck
npm test -- --run --maxWorkers=1
npm run test:ui -- --workers=2
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run package:windows
```

UI tests use Microsoft Edge through Playwright, build production assets and inject a test-only IPC bridge. Native behavior requires separate installed Windows testing. Packaging exports an NSIS installer, checksum and docs into ignored `release/`. An absent WebView2 runtime may require an installer prerequisite download. No signing certificate is configured.

## Source map

| Path | Responsibility |
| --- | --- |
| `src/` | Accessible React views, typed commands and event subscriptions |
| `src-tauri/src/scheduler/` | Rust state machine, injectable clock and worker |
| `src-tauri/src/native/` | Notifications, tray, startup and window lifecycle |
| `src-tauri/src/storage/`, `src-tauri/migrations/` | SQLite transaction boundary and migrations |
| `src-tauri/tests/`, `tests/ui/` | Deterministic core and browser workflow tests |
| `docs/` | Policies, architecture, historical audits and release gates |

## Data and legacy compatibility

Restnaa retains `%APPDATA%\com.pausepal.desktop\pausepal.sqlite3`, application identifier `com.pausepal.desktop` and binary name `pausepal.exe` to avoid an accidental new profile or executable mismatch for existing installs. Visible branding and installer names are Restnaa. These legacy identifiers are intentional; changing them requires an explicit migration and installed upgrade tests. Back up the stopped profile before upgrading. Never run old and new variants concurrently.

See [privacy](PRIVACY.md), [installation](docs/INSTALLATION.md), [scheduler contract](docs/SCHEDULER.md), and [current naming/verification report](docs/NAME_CORRECTION.md). The app implements no network calls; embedded WebView2/Windows component traffic is a separate unresolved audit boundary. A strict zero-egress guarantee is not established.

## Contribute

Start with [CONTRIBUTING.md](CONTRIBUTING.md) and [repository instructions](AGENTS.md). Bug reports, documentation, accessibility, tests and small fixes are welcome. [PRD.md](PRD.md) defines product boundaries. Please discuss behavior changes before building them; AI, accounts, cloud sync, payments and employee monitoring are outside the current product scope.

See [security reporting](SECURITY.md), [community guidelines](CODE_OF_CONDUCT.md), [MIT license](LICENSE) and [third-party notices](THIRD_PARTY_NOTICES.md). Historical PausePal audits retain original names and versions so evidence is not misrepresented. Maintainers: follow [GitHub publication steps](docs/OPEN_SOURCE.md) before uploading.
