# Restnaa identity reset plan and verification

Authorized 2026-11-10: no public installations exist; use the new name for all product identifiers. Version 0.1.4 distinguishes the fresh identity from internal candidates.

Implementation: Tauri identifier com.restnaa.desktop, main binary restnaa, Cargo default-run/bin restnaa, SQLite restnaa.sqlite3 under the Tauri app-data directory, native tray/worker/diagnostic identifiers, test fixtures and helper paths. Update current docs, packages and identity regression. No data migration or deletion; local development profiles stay untouched. No scheduler, outcome, capability or Windows preference behavior change. Exact earlier evidence remains in Git history; obsolete audit pages become current plans/pending acceptance instead of renamed historical claims.

Validation: scan every tracked text file for old names, frontend unit/lint/type/build/UI, Rust tests/format/Clippy, locked Windows NSIS build and source archive review. Run UI and packaging frontend builds sequentially to avoid replacing assets while tests load them. Commit/push to the existing GitHub remote and verify its hash. Installed notification/startup/single-instance/clean-machine/manual gates remain pending for the new identity until actually performed.


## Preparation checks

Tracked text scan: no old product spelling or identifiers remain. Current README/contributor/security/publication relative links resolve. Earlier exact source/audit records remain in Git history; current audit pages are plans/pending acceptance, not relabeled historical passes. The first offline Rust check rejected an accidental broad lockfile version edit (smappservice-rs); restored all dependency records and changed only the Restnaa root version. No dependency update or network workaround was used.


## Current frontend validation

npm test -- --run --maxWorkers=1: 63 passed across 14 files. npm run lint and npm run typecheck: passed. npm run test:ui -- --workers=2: 30 passed (1.2m), including keyboard/accessibility, navigation, settings, pause, movement outcomes and break-panel states. Browser plugin unavailable; existing Playwright/Edge production-asset suite at 127.0.0.1:1420 used. Desktop/compact, zoom/forced colors, reduced motion and high DPI are covered. No frontend build ran concurrently with this UI suite. Fixture counts are not native/user data. New-identity installed OS behavior is still pending.


## Current Rust validation

cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline: 52 passed. The native identity regression checks product name, identifier, binary name, SQLite filename, Cargo default-run and both diagnostic helpers. cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline -- -D warnings: passed. cargo fmt --manifest-path src-tauri/Cargo.toml -- --check and git diff --check: passed. No dependency version change remains in either lockfile beyond the Restnaa root version.


## Packaging and publication

npm run package:windows passed: production frontend, locked Rust release build, restnaa.exe and NSIS installer. Export: release/restnaa-0.1.4/Restnaa_0.1.4_x64-setup.exe, 3,125,449 bytes, NotSigned. SHA-256: 531e5725fa78a8a133529d2b0fb8ef6aaa12d4bb85a334b5af90c79c13dc2010. Manifest identifier is com.restnaa.desktop. Installed launch/notification/tray/login/single-instance/uninstall and clean-machine validation were not performed in this task and remain pending. Nonfatal MSVC vswhere lookup and Vite plugin-timing/NO_COLOR notices did not prevent compilation.

The source-only archive release/restnaa-source-0.1.4.zip contains the reviewed Git-index files plus a generated checksum inventory. No .git metadata, dependencies, tools, installers, user databases or credentials are included. No automatic profile migration, deletion, OS preference change or previous-artifact rewrite was performed. Exact prior audit evidence remains in Git history. Current tracked text uses Restnaa consistently.
