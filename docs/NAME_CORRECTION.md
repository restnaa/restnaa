# Restnaa name correction

2026-11-10: user clarified that the intended name is Restnaa (two final a characters). Update current UI/native strings, package/library names, Tauri product/window titles, favicon, packaging/CI and current docs/tests. Retain 0.1.3 as an unreleased candidate, com.pausepal.desktop, pausepal.sqlite3 and pausepal.exe. No scheduler/storage/ACL/lifecycle changes. Preserve previous build/audit evidence under its original spelling and mark it historical. Run frontend unit/lint/type/build/UI and Rust tests/format/Clippy, build the renamed installer, then commit and push the correction to the already authorized restnaa/restnaa remote. Installed/clean-machine gates stay pending.


## Verification

- npm test -- --run --maxWorkers=1: 63 passed across 14 files.
- npm run lint and npm run typecheck: passed.
- cargo test --manifest-path src-tauri/Cargo.toml --all-targets --offline: 52 passed, including corrected Restnaa product/legacy-identity regression.
- cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline -- -D warnings: passed.
- cargo fmt --manifest-path src-tauri/Cargo.toml -- --check and git diff --check: passed.

Browser plugin unavailable; existing Playwright/Edge production-asset tests used. Initial UI run passed 28/30; two compact navigation checks failed because simultaneous packaging removed/rebuilt dist assets during requests (ENOENT and HTTP-response failure). This was a verification orchestration error, not evidence of a product regression. Rerun after packaging frontend build ends, without another concurrent frontend build; do not hide the initial failure.

Historical Restna reports/installers retain their actual original spelling and hashes. Corrected current source, license branding, installer prefix and ZIP use Restnaa. Product version remains 0.1.3, an unreleased candidate. Repo URL remains https://github.com/restnaa/restnaa. No database migration, notification permission/OS setting, scheduler policy or native presentation/dismissal behavior changed. Signing, renamed installed upgrade, startup/notification identity and clean Windows gates remain pending.


## Final result

Complete UI rerun: npm run test:ui -- --workers=2 passed all 30 tests (1.4m). Restnaa title/controls, keyboard access, reduced motion, navigation and workflows passed. npm run package:windows passed the production build and locked NSIS compilation. Export: release/restnaa-0.1.3/Restnaa_0.1.3_x64-setup.exe, 3,128,470 bytes, NotSigned; SHA-256 1123399a868fb9a25c751085e44a0edfd2f5f14e6ebd988c5aaf3e0b69c54d83. Not installed in this task; pending OS acceptance is unchanged. Old Restna/PausePal exports are preserved as historical artifacts. The corrected source archive is release/restnaa-source-0.1.3.zip; it includes only reviewed Git-index files and a checksum inventory, not dependencies, credentials, local user data or installers.
