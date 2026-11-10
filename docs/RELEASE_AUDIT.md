> Historical PausePal record. Restnaa is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](OPEN_SOURCE_PLAN.md).

# PausePal 0.1.0 release audit — 2026-10-21

**Decision: release candidate only; public release NO-GO while mandatory manual/clean-machine gates remain pending.** No clean-machine or seven-day dogfooding claim is made.

## Environment

- Existing Windows 11 Pro, 10.0.26300/build 26300, HP Pavilion Gaming Laptop, eight logical processors. Existing WebView2 and previous PausePal installs/data; not a clean host.
- Tauri 2.12.4; notification 2.7.0, autostart 2.7.0, single-instance 2.5.4; React/TS/Vite/Tailwind; SQLite schema 2. Exact lockfile hashes accompany the package manifest. Node 24 and workspace Rust/MSVC are used.
- No `.git` checkout: no commit/clean-worktree claim. Hosted CI updated but not executed here.
- Hyper-V command exists, but `Get-VM` reported insufficient authorization; Windows Sandbox tooling not found. No accessible clean Windows 10/11 VM. Host clock, suspend/session and global notification/DND settings were not changed.
- Browser plugin unavailable; Playwright/Edge at `http://127.0.0.1:1420` uses production assets, 1200×800 desktop, 390×844 compact/reduced motion, and desktop 200% CSS zoom/forced colors. Manual screen-reader/OS high-contrast remains pending.
- Existing preferences and two history IDs backed up to `%TEMP%\pausepal-m5-qa`. User history is not cleared. Final verification retained both original IDs and exact preferences; the count is now 3 because restart QA recorded one genuine unanswered event, with no test completion in the real user profile. Disposable native QA uses separate identity `com.pausepal.audit`, not a clean OS.

## Findings repaired

1. **Forward wall jump bypassed cooldown:** eye opened, wall advanced two hours without monotonic elapsed, and movement opened immediately. Recovery now retains remaining monotonic spacing, capped at two minutes if monotonic time regressed. A 119+1 simulated-second test verifies delayed delivery.
2. **Backward clock adjustment while closed retained hours of delay:** saving at 13:00 and restarting at 11:00 retained old deadlines/cooldown. Restart now bounds future deadlines by a fresh interval shifted into the current work calendar, retaining earlier valid deadlines and bounding cooldown to two minutes. Both new tests failed before the repair and pass afterwards.
3. **No-tray close could strand the hidden break webview:** the close policy now explicitly quits all webviews. Unit policy test passed; actual forced tray-creation failure is not injected into production.
4. **Malformed history timestamps silently disappeared from totals:** invalid dates now return an error without erasing rows.
5. **Storage readiness lost definition semantics:** the status live region moved inside `dd`, fixing axe's definition-list violation.
6. **Diagnostics/storage failure hid Quit:** Quit/Refresh now remain available without a loaded native status; new regression added.
7. **Old toast could pass a delivery check:** the own-app notification probe now compares a pre-send snapshot to require a fresh test entry. History does not prove banners.

Test-tool findings: development-server runs intermittently had blank pages; later script/style diagnostics captured a concrete host network-buffer error even on production preview, so a dev-optimizer root cause is not established. Release tests now build production assets and capture failed scripts/styles. Because this host repeatedly returned ERR_NO_BUFFER_SPACE for loopback CSS, Playwright fulfills exact built assets from dist/assets (no alternate content); HTML still uses the preview server, and missing files/uncaught errors still fail. This avoids changing browser security or OS networking. Native installed WebView2 asset loading is separately exercised. One later package build overlapped preview and briefly removed the shared index file, causing a 404; the final full run followed packaging. A skip-link test pressed Tab before React rendered; it now waits for meaningful content. An initial compact zoom probe used 195 CSS pixels, below minimum dimensions; desktop zoom and independent compact workflows now cover supported layouts.

## Automated evidence

| Command/check | Result |
|---|---|
| `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked` | 48 passed: 46 core/integration plus 2 notification-probe tests, including both clock fixes |
| `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` | Passed before candidate packaging |
| `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings` | Passed |
| `npm run lint`, `npm run typecheck`, `npm run test -- --run` | Passed; 46 frontend unit tests |
| `npm run test:ui -- --workers=1` | Final run passed all 22 in 42.9 seconds, after packaging completed. Earlier host ERR_NO_BUFFER_SPACE and a shared-output build 404 are documented; exact built assets are fulfilled by the test harness |
| `npm run build` | Passed; NSIS regenerates production assets |
| `npm audit --json` | 0 known vulnerabilities including accessibility tooling |
| `cargo audit --file src-tauri/Cargo.lock --json` | Exit 0; 0 vulnerability entries, 5 informational warnings |

RustSec database: 1,348 advisories, revision `4b587f00996ebe5c05a73eaef105ef241e876414`, updated `2026-10-21T08:20:38+02:00`. Informational findings: proc-macro-error 1.0.4 / RUSTSEC-2024-0370; zbus_names 4.3.5 / RUSTSEC-2026-0371; zvariant and zvariant_derive 5.15.1 / RUSTSEC-2026-0370 and RUSTSEC-2026-0372; glib 0.18.5 / RUSTSEC-2024-0429. Windows-target `cargo tree -i` prints no affected packages; `--target all` shows other-platform/GTK paths. Warnings are not suppressed or claimed fixed; review before any non-Windows release.

The Rust suites cover both categories, all outcomes, no overlap/stale actions, quiet/work hours, all pause modes, both suspend monotonic behaviors, restarts, forward/back/timezone/DST and no storms. Temporary DB tests cover transactions, corruption, newer/unrecognized schema, invalid runtime, history boundaries/deletion and preference persistence. These do not prove physical OS power/time behavior. Axe checks WCAG A/AA tags on main workflows/modal and both break categories; this is not a full accessibility certification.

## Installed/native evidence

| Check | Result |
|---|---|
| Final NSIS install/upgrade on this host | Passed: exact exported candidate installed over the existing user install, installer exit 0 |
| Fresh installed test toast, no action buttons | Passed: snapshot 6 to history 7, freshCheck=true/testReminderFound=true/testHasActionButtons=false; unchanged-snapshot check exits 2 |
| Duplicate launch, close/hide, explicit Quit | Passed on both installed audit build and final normal candidate: duplicate exited, one process, manual launch restored main, close hid main without exit, Settings Quit stopped app |
| Startup opt-in/off, restart and persisted manual pause | Passed: explicit opt-in registered startup, restart retained manual pause, both app windows hidden on --autostart |
| Installed corrupt profile preserves file and offers Quit | Passed on installed disposable identity: unavailable storage/error, Quit enabled, corrupt bytes unchanged |
| Confirmed clear history | Passed: explicit Done recorded completed, confirmed clear returned zero, existing user profile untouched |
| Uninstall startup cleanup/data retention | Passed: uninstall exit 0, startup before=true/after=false, executable removed, DB retained with identical SHA-256. Generated update/data-checkbox policy also reviewed |
| Direct tray-menu interaction | Pending: taskbar exposed no icon descendants and tray-area capture was black; no blind clicks sent |
| Actual OS notification denial | Pending: read-only Windows notification Settings attempts could not reliably access PausePal's row/control; no global settings changed. Policy paths unit tested |
| Physical DND/banner/body clicks | Pending interactive check; no bypass |
| Actual sign-in; physical sleep/wake/time/timezone/session resume | Pending; host not disrupted |
| Clean Windows 10/11 | Not run: no accessible clean environment |
| Seven-day dogfooding | Not run: short audit/fake clocks are not a substitute |

## Idle resources

Measured normal installed release, active between reminders with both app windows hidden via --autostart, startup registration still off, no debug listener and no concurrent compilation. Duration 60.37 seconds, 12 samples, 8 processes. CPU counters accumulated 0.0000 seconds for root and tree: no measurable CPU at this counter resolution, not a claim of literally zero CPU. Peak summed working set 416.89 MiB, peak private bytes 212.95 MiB. `scripts/measure-idle.ps1` samples only PausePal and WebView2 descendants, reports CPU deltas and peak aggregate working/private memory, and records duration/process count. Working-set sums can double-count shared pages; exited-child CPU between samples can be missed. A short host sample is not a battery, leak, long-term or comparative performance claim.

## Package and next release gates

`npm run package:windows` exports `release/`: current-user NSIS, SHA-256, signature/lockfile manifest, notes, privacy, installation, limitations and audit. Explicit WebView2 download-bootstrapper mode may need internet if the runtime is missing. Signing is not configured. Hashes/signature status are generated after final build, not hard-coded here.

Before public release obtain clean Windows 10/11 standard-user/missing-WebView2 install/uninstall evidence; actual login, tray, notification denial/DND/banner-click checks; physical power/time/session tests; manual screen-reader/high-contrast review; seven-day dogfooding; signing/distribution review. Keep unresolved rows pending and preserve user data. A temporary C# window-inspection helper initially used a Main method with an invalid entry-point signature; renaming it FindMain resolved the QA-helper compile error before the successful visibility checks.

Distribution follow-up: no repository LICENSE or THIRD_PARTY_NOTICES.md is present. License/third-party-notice and provisional product-name review have not been performed; no distribution-rights claim is made.

Final artifact: current-user `PausePal_0.1.0_x64-setup.exe`, 3,120,285 bytes, SHA-256 `4107dd3c3823702ca71b7138d554d1d3aa037c0ee471405600671ac07d17e357`, Authenticode `NotSigned`. Installed executable matches the built normal candidate; temporary debug process was explicitly quit and normal hidden app restarted with original preferences and startup off. Both original history IDs remain. Temporary QA evidence is under `%TEMP%/pausepal-m5-qa`; the separate audit executable/startup entry was uninstalled and its test database retained to verify retention.


## Runtime network observation: unresolved blocker

A scoped process-tree TCP snapshot after the normal hidden idle sample found one established outbound IPv6 TCP/443 connection (plus its bound socket entry), within eight app/WebView2 processes. The connection closed before a second query captured OwningProcess/name/role; the first snapshot omitted that field. Its payload/purpose is unknown. Do not describe the package as having proven zero runtime network traffic or infer endpoint purpose solely from its address. No application fetch/HTTP/telemetry code was found, but the embedded runtime is not proven silent. Privacy and known limitations now explicitly disclose vendor runtime diagnostics/security behavior, with Microsoft's primary documentation. Detailed attribution/traffic investigation and resolution of a strict zero-egress expectation are additional public-release blockers. No OS/firewall/security policies or protection flags were changed.


## Reproducing installed evidence

With the Windows Rust toolchain initialized, snapshot before clicking Send test reminder in the installed app:

```powershell
cargo run --manifest-path src-tauri/Cargo.toml --example notification_probe --locked -- --snapshot before.json
# Click Settings -> Send test reminder in the installed app.
cargo run --manifest-path src-tauri/Cargo.toml --example notification_probe --locked -- --since before.json
```

An unchanged snapshot should return exit 2; a new test occurrence returns 0 only when no action markup is present. If Windows retained identical rows at a history limit, failure to prove freshness remains inconclusive rather than a delivery success. For idle measurement on this workspace's normal installed binary:

```powershell
$appProcess = Get-Process pausepal | Where-Object Path -eq 'D:\pausepal\.tools\m3-installed\pausepal.exe'
.\scripts\measure-idle.ps1 -AppProcessId $appProcess.Id -Seconds 60 -OutputPath "$env:TEMP\pausepal-idle.json"
```
