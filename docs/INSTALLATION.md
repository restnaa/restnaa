# Windows installation and operation

This is a **0.1.3 release candidate**, not a completed clean-machine release. Target: Windows 10/11 x64. Current host verification and remaining gates are in [RELEASE_AUDIT.md](RELEASE_AUDIT.md).

## Verify and install

The generated `release` folder contains `Restna_0.1.3_x64-setup.exe`, its `.sha256`, `manifest.json`, and release/privacy/installation/limitation/audit documents.

```powershell
Get-FileHash .\Restna_0.1.3_x64-setup.exe -Algorithm SHA256
Get-Content .\Restna_0.1.3_x64-setup.exe.sha256
Get-AuthenticodeSignature .\Restna_0.1.3_x64-setup.exe
```

Compare the complete hash with the supplied checksum. The checksum detects a mismatch; it does not establish a trusted publisher. This candidate is unsigned. Public distribution/signing and SmartScreen reputation have not been approved or verified.

Run the installer as your normal Windows user; it targets the current user, with a default location under `%LOCALAPPDATA%\Restna` unless a previous/custom location is retained. Launch **Restna** from the Start menu. Initial setup needs no account. Autostart is off until you opt in. Use **Finish setup** to save onboarding; **Save settings** applies schedule changes without restarting.

WebView2 Evergreen is required. The NSIS download-bootstrapper mode may access Microsoft's servers to install it if absent; internet access may therefore be needed during installation. With an existing suitable runtime, normal app operation is offline. Clean-machine prerequisite installation remains pending. See [official Tauri Windows installer documentation](https://v2.tauri.app/distribute/windows-installer/).

## Reminders and background behavior

- Closing the dashboard hides it to the tray. Use the tray's Show or Settings item to reopen it. **Quit Restna completely** in Settings or the tray stops the process and reminders.
- Windows may suppress banners. Review Settings diagnostics and **Send test reminder**, then use **Open current break** in the tray when one is waiting. There are no Windows toast action buttons. On-screen toast clicks may work; Action Center activation is not guaranteed.
- Eye/movement breaks follow enabled categories, working hours, quiet mode, pause and the two-minute spacing guard. If Take break now is unavailable, check those states. Only one break is open at a time.
- Done, Skip, Snooze and Unanswered remain separate outcomes. Esc/Close hides a break without completion; the Rust timeout handles an unanswered reminder. Suggested durations are text, not a verification/countdown feature.
- Seven-day history means today plus the preceding six local dates. Totals cover the full period; details show up to 100 events. Refresh updates an open view after midnight/timezone changes.
- Pause offers 30 minutes, one/two hours, until local midnight, or until manual Resume. Working hours and quiet mode still apply after Resume.

## Upgrade, uninstall and recovery

Quit the old app before upgrading. Install over the same user installation. Back up `%APPDATA%\com.pausepal.desktop` first if you need a recovery copy, with the app fully quit. Never run two portable/installed variants against the same data intentionally.

To uninstall, Quit and use Windows Settings → Apps → Installed apps → Restna → Uninstall. The generated uninstaller removes startup registration on a normal uninstall, preserves it during an update, and normally retains app data. Select **Delete app data** only if you intend to erase this app's local preferences and history. Clear history in the app deletes outcomes only; Windows notification history is separate.

If storage fails, existing data is not automatically reset. Quit, preserve the data folder, and inspect file permissions/disk health or restore a known-good backup deliberately. Deleting an unfamiliar/corrupt database is not an automatic repair step. Clean Windows install/uninstall and actual sign-in behavior remain manual gates.

## Build from this repository

Follow the toolchain setup in [README](../README.md). With Node/Rust/MSVC configured:

```powershell
npm ci
npm run lint
npm run typecheck
npm run test -- --run
npm run test:ui -- --workers=1
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run package:windows
```

Browser tests use production preview assets, Playwright/Edge and test-only IPC fixtures for workflow coverage. They are not proof of native Windows behaviors. Hosted CI, signed distribution and clean-machine testing require separate evidence.

## Existing PausePal installations

Restna retains com.pausepal.desktop, pausepal.sqlite3 and pausepal.exe for compatibility. Quit PausePal before installing. Back up its stopped profile first. Renamed-installer upgrade, retained install directory, existing startup registration and Windows notification identity require installed acceptance; do not assume those checks passed from an old PausePal build. Do not manually move/delete the database or run both names concurrently.
