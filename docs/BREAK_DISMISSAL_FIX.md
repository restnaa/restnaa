> Historical PausePal record. Restna is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](OPEN_SOURCE_PLAN.md).



# Break-window dismissal repair — 2026-10-25



Plan: src-tauri/src/commands.rs makes hide_break_window asynchronous on the blocking pool so Windows window operations cannot block a synchronous IPC handler. src/pages/BreakPanel.tsx awaits the outcome commit then explicitly hides; failed commits stay visible, failed hides give an actionable error and Close remains available. src-tauri/src/native/windows.rs schedules automatic ShowWindow on the owning UI thread rather than synchronously cross-thread on the scheduler. Existing native auto-hide, UUID deduplication, outcomes and timing remain unchanged; no permission expansion. Add src/pages/BreakPanel.test.tsx coverage for both categories, dismissal order/failures, Close/Escape, and UI regression coverage. Verify lint, frontend/Rust tests and build, installed disposable Windows movement actions, native visibility. Do not alter real user settings/history or claim clean-machine certification.



Initial investigation: installed QA 0.1.1 manual movement Done committed and native IsWindowVisible returned false. User's exact notification-opened freeze has not yet been reproduced. Repairs must not be described as a reproduced root cause without evidence.



## Automated checks



- npm test -- --run --maxWorkers=1: 63 passed across 14 files. Both categories hide only after acknowledged outcome; failed mutation remains visible; hide failure permits a Close retry without repeating an outcome.

- npm run lint and npm run typecheck: passed.

- npm run test:ui -- --workers=2: first attempt timed out starting the build server (no tests ran); complete rerun passed all 30 tests, including movement Skip explicitly invoking hide. Browser plugin unavailable; existing Playwright/Edge production-asset fixture used.

- cargo fmt --manifest-path src-tauri/Cargo.toml -- --check: passed.



Scope note: no claim of reproduced cache desynchronization. Inspection of Tao 0.37.1 shows hide actually calls SW_HIDE even when its cached visibility flag is false, so that initial hypothesis was rejected. Synchronous hide is moved to a worker as a thread-boundary hardening measure, not a proven root cause. Automatic ShowWindow now runs on its owning main thread so scheduler execution does not synchronously wait inside that Win32 call. Explicit outcome dismissal is independent of event delivery. No new Tauri capability or SQLite migration.



## Candidate



Version metadata is 0.1.2 in package.json/package-lock.json, Cargo.toml/Cargo.lock and tauri.conf.json. Application identity and data location stay com.pausepal.desktop. The new candidate will be exported under release-auto-break/0.1.2, preserving the existing 0.1.1 package. Automatic panels remain default On, no always-on-top/focus calls were added, and Windows DND remains best effort as explicitly authorized.



The original installed executable reports 0.1.1. No original process was stopped by this task; no original settings/history or Windows notification settings were changed. QA uses com.pausepal.autobreakqa only. Native acceptance uses autonomous due movement plus explicit controls and native WM_CLOSE (the same message as the titlebar X), checking IsWindowVisible and Rust state after each action. A physical click on an actual Windows toast remains separate evidence from explicit Open current break.



## Reproduced failure and revised native fix



Installed QA 0.1.2 autonomous movement presentation reproduced the user's bug: Done committed successfully, Rust openReminder became null, the UI showed No break waiting, but native IsWindowVisible remained true. Both the observer hide and explicit hide were insufficient. This supersedes the initial inability to reproduce and the initial rejected visibility hypothesis: although Tao's flags implementation contains SW_HIDE, the actual framework path did not hide the raw-shown HWND in this installed case.



Revised fix: src-tauri/src/native/windows.rs pairs native SW_SHOWNOACTIVATE with native SW_HIDE on the owning UI thread, after also requesting the normal Tauri hide. All native dismissal paths (src-tauri/src/native/mod.rs observer, hide_break_window command, and break CloseRequested/titlebar X) use that shared function. Dashboard close behavior is unchanged. No focus/topmost changes or data migration. Rebuild and rerun installed autonomous and explicit action checks before distributing.



## Final verification — 2026-10-25



- Final Rust source: cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline: 51 passed. cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline -- -D warnings: passed. cargo fmt --manifest-path src-tauri/Cargo.toml -- --check: passed.

- Frontend: 63 unit tests, 30 Playwright tests, lint, typecheck and production build passed. Native-only revisions followed those frontend checks; frontend source did not change afterwards.

- npm run tauri -- build --config .tools/auto-break-qa.config.json --bundles nsis -- --locked: passed for the final native fix. Installed disposable QA upgrade returned 0. The initial startup harness attempted IPC during navigation and failed before actions; retry after load was used, with no mutation retry.

- The initial native run reproduced the failure after automatic movement Done. After the shared SW_HIDE fix, a real autonomous due movement reminder appeared, Done committed (openReminder null), and IsWindowVisible returned false. No events were inserted by the harness. Only disposable QA deadlines/settings were adjusted while stopped to arrange eligibility.

- Re-presented the existing native window via SW_SHOWNOACTIVATE and verified Close for now, Escape and native WM_CLOSE each made IsWindowVisible false. The wider preliminary action loop then correctly rejected a new Start inside the two-minute delivery-spacing window. Narrowed the harness to the reported automatic-dismissal flow and reran from a stopped/rearmed disposable profile: node C:/OracleTemp/pausepal-auto-break/regression.cjs automatic completed with exit 0 and all four dismissal assertions passed. The native re-show subtests exercise visibility only, not fabricated reminders or history.

- A normal-identity build started before the reproduced fix was deliberately cancelled and was not distributed. Final npm run tauri -- build --bundles nsis -- --locked passed. Both installers are 0.1.2; the distributed normal installer is the later final build, not the earlier failed QA candidate.



Installed QA is on this existing Windows machine only. Actual physical Windows toast click-through and clean Windows 10/11 validation remain pending; no claim of those tests passing. Existing M5 privacy/runtime-egress, signing and other manual release gates remain unchanged. The candidate is unsigned. Normal app identity/data location unchanged; no changes to actual user preferences/history or OS DND. Automatic panels still default On with persisted opt-out.



QA uninstall returned 0; its executable was removed and debugging port 9230 was no longer listening. No PausePal process remained at cleanup. Final normal installer SHA-256: e420d0fbcd9284441c81e9af98b3f8c0ef0efab4b1ea8e7e9fcbcb4adcef7453 (3,129,874 bytes, NotSigned). Export: release-auto-break/0.1.2/PausePal_0.1.2_x64-setup.exe. Previous candidate exports preserved.
