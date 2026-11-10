# Restna 0.1.3 open-source preparation report

Date: 2026-11-10. Rename/source preparation authorized; MIT explicitly selected by the user. See OPEN_SOURCE_PLAN.md. Historical results are kept separate from current checks below.

Visible branding, npm/Rust package/lib names, metadata and installer are Restna. Legacy binary pausepal.exe, identifier com.pausepal.desktop and database pausepal.sqlite3 remain intentionally unchanged. Audits/old exports preserve original names/versions. No scheduler timing, SQLite migration, ACL or native lifecycle change intended. Installed upgrade, old startup registration and notification identity remain pending; preserving identifiers alone is not upgrade verification.

Added contributor/security/conduct guidance, license, third-party notices, issue/PR templates, ignore rules and portable setup. Full transitive binary-license review, trademark clearance, configured private reporting and hosted CI remain pending. No GitHub remote exists and no source was pushed.

## Changes and publication review

- 135 staged source/documentation files on a newly initialized main branch. No commits, remote or push created.
- MIT explicitly selected. Runtime npm license texts and third-party attribution included; full transitive binary-license review remains a release task.
- .tools, node_modules, generated builds, local databases/WALs, credentials/signing material, installers and historical generated raster excluded from the publication inventory. No files deleted from the user's profile or historical releases.
- Reviewed source candidates for private-key blocks, GitHub token patterns and AWS access-key patterns: no matches. This is a limited pattern scan, not a comprehensive security audit. Personal user-profile path references were sanitized; local historical evidence paths are explicitly historical.
- Current README/contributor/security/publication guide relative links resolve. git diff --cached --check passed after correcting historical mixed line endings. UTF-8 normalization fixed a Windows default-codepage read failure while writing docs; no application behavior was changed by that repair.

## Current checks

| Command | Result |
| --- | --- |
| npm test -- --run --maxWorkers=1 | 63 tests passed, 14 files |
| npm run lint | Passed |
| npm run typecheck | Passed |
| npm run test:ui -- --workers=2 | 30 tests passed, 2.8 minutes |
| cargo test --manifest-path src-tauri/Cargo.toml --all-targets --offline | 52 tests passed; lockfile updated for renamed root package |
| cargo fmt --manifest-path src-tauri/Cargo.toml -- --check | Passed after formatting the new regression test |
| cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline -- -D warnings | Passed |

Browser plugin not available; existing Playwright/Microsoft Edge fixture used at http://127.0.0.1:1420. Flow: load Restna -> navigate Dashboard/Settings/History -> save settings/pause -> act on a movement break -> verify typed invocation and UI state; also keyboard/accessibility, clear confirmation, native preference controls and reduced motion. Viewports include 1200x800 desktop, 390x844 compact, reference 1448x1086, minimum width, zoom/forced colors and high DPI. Title Restna, meaningful content, no framework overlay, no uncaught page errors/failed bundled assets and no overflow were asserted. Screenshot inspected: local temporary pausepal-redesign/dashboard.png, showing Restna branding; displayed counts are test fixtures, not native/user activity. No design/layout change was requested or made.

Remaining acceptance: renamed installed upgrade/startup registration/notification identity, clean Windows 10/11, signing, physical toast click/DND/power/session testing, screen reader, seven-day use and embedded-runtime egress review. Historical PausePal installed tests are not Restna certification. Hosted CI has not yet run on GitHub. Configure private reporting/contact after creating the repository. The name has not received trademark clearance. No public stable release approval is claimed.

## Packaging and handoff

npm run package:windows passed: production frontend build and locked Windows NSIS compilation. Installer: release/restna-0.1.3/Restna_0.1.3_x64-setup.exe, 3,127,011 bytes, unsigned. SHA-256: 43cf0b709a8b034f8eee31fb61ce9705ad70ee6582b98ffe088b725dde933260. Renamed installer was built, not installed/tested in this task. Old release folders remain unchanged. Nonfatal MSVC vswhere lookup, Vite plugin-timing and NO_COLOR notices occurred; compile/link/package completed.

A source-only ZIP at release/restna-source-0.1.3.zip contains the 135 reviewed Git-index paths plus a generated per-file SHA-256 inventory. It excludes .git metadata, dependencies, tools, databases, installers, generated output and the local concept image. This archive is a handoff artifact, not an uploaded repository. Re-run publication review before adding future files; ignore rules cannot prevent every secret from being committed.

The local main branch has the initial source staged for review, no commit and no remote. Follow docs/OPEN_SOURCE.md: create an empty public restna repository, review staged content, commit, add the real URL and push. Enable private reporting, configure a maintainer contact and run hosted CI after creation. Source can be reviewed/contributed to while stable binary release gates remain explicitly open.
