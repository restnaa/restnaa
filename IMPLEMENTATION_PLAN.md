> Historical PausePal record. Restnaa is the current name. Original names, versions and test evidence below are preserved; they do not certify the renamed build. See [open-source preparation](docs/OPEN_SOURCE_PLAN.md).

# M1 foundation implementation plan

Status: M1 implementation authorized by Prompt 1. The original plan below was provided before implementation; verification results are reported separately. The repository began with specification documents and no application scaffold or Git repository.

## 1. Bootstrap the Windows desktop foundation

Create `package.json`, lockfile, `index.html`, `vite.config.ts`, TypeScript configs, `eslint.config.js`, `.gitignore`, `src/main.tsx`, and `src/styles.css`. Use Tauri 2, React/TypeScript, Vite, and Tailwind. At implementation time verify compatible current versions against official documentation, and pin resolved dependencies in lockfiles.

Create `src-tauri/Cargo.toml`, `src-tauri/Cargo.lock`, `src-tauri/build.rs`, `src-tauri/src/main.rs`, `src-tauri/src/lib.rs`, `src-tauri/tauri.conf.json`, and local application icons. Configure application identity, local assets, a restrictive CSP, and only the main window. Configure no runtime network, updater, notification, tray, autostart, or single-instance integration in M1.

Checks: dependency installation, TypeScript checking, frontend production build, `cargo check`, and Windows Tauri development launch. Confirm that the WebView displays the local frontend and surfaces startup errors.

## 2. Build accessible navigation and layout

Create `src/App.tsx`, `src/components/AppLayout.tsx`, `src/pages/Dashboard.tsx`, `src/pages/Onboarding.tsx`, `src/pages/Settings.tsx`, and `src/pages/History.tsx`. Provide a compact, consistent PausePal shell with working navigation. Clearly state that reminders are not yet available. Defer settings forms, onboarding persistence, break controls, countdowns, pause/resume, and history analytics to their milestones. Do not show invented progress or active schedule state.

Create `src/App.test.tsx` and `src/test/setup.ts` with Vitest, Testing Library, and a DOM test environment. Test navigation to each view, accessible control names, current-page indication, and honest foundation status. Manually inspect keyboard navigation, focus visibility, contrast, reduced motion, and layout at small window sizes.

## 3. Establish a narrow typed command boundary

Create `src-tauri/src/commands.rs`, `src-tauri/src/dto.rs`, `src/lib/api.ts`, and `src/lib/types.ts`. Implement only a small read-only foundation command such as `get_app_info`, returning app version and storage readiness without exposing filesystem paths. Render the real response and an understandable failure state in the shell.

Create `src-tauri/permissions/app.toml` and `src-tauri/capabilities/main.json` with only the permissions required by the trusted main window; register custom command permissions through `src-tauri/build.rs`. Review any generated default capability entries and remove unnecessary grants. Document future PRD commands/events without registering behavior stubs.

Checks: Rust DTO serialization test, frontend API response/error rendering tests in `src/lib/api.test.ts` or the app test, and an actual invoke round trip in the desktop app. Review capability scope and CSP for unnecessary privileges.

## 4. Initialize local SQLite safely

Create `src-tauri/src/storage/mod.rs` and `src-tauri/migrations/0001_initial.sql`. Rust opens SQLite in the Tauri application data directory and applies versioned migrations transactionally. Add the PRD's `settings`, `reminder_state`, and `break_events` tables with exact category/outcome checks and history indexes. Migration metadata may use SQLite `user_version` or a dedicated migration table. Do not seed reminder deadlines or implement scheduler recovery in M1.

Create `src-tauri/tests/storage.rs` with isolated temporary databases. Verify fresh schema creation, repeat initialization preserving existing data, constraint rejection, transactional migration rollback, and corrupt-file error handling that preserves the original bytes. Surface initialization failure in the UI; do not replace or erase an unreadable database. Persistence tests may insert fixtures directly through the Rust storage test interface without exposing UI SQL access.

## 5. Add repeatable checks and Windows instructions

Add scripts for `dev`, `build`, `typecheck`, `lint`, `test`, and `tauri` to `package.json`. Add `.github/workflows/ci.yml` using a Windows runner for locked dependency installation, frontend lint/typecheck/tests/build, Rust formatting/Clippy/tests, and a desktop build. Run the workflow's commands locally where tooling permits. CI execution itself remains pending until the repository is hosted and a workflow run is observed.

Update `README.md` with Windows prerequisites (Rust MSVC toolchain, C++ build tools, Node/package-manager version, and WebView2), exact install/run/check/build commands, local data location and privacy boundaries, and the implemented M1 scope. Distinguish development launch, desktop build, installer generation, and future installed notification validation.

Expected checks after bootstrap:

- `npm ci`
- `npm run lint`
- `npm run typecheck`
- `npm run test -- --run`
- `npm run build`
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --locked -- -D warnings`
- `cargo test --manifest-path src-tauri/Cargo.toml --locked`
- `npm run tauri build -- --no-bundle`
- `npm run tauri dev` for manual Windows launch, real command response, navigation, and keyboard checks.

These are planned commands, not executed results. Adjust scripts and paths coherently if the official scaffold requires it. A no-bundle build verifies the desktop executable; NSIS packaging and clean-machine release verification remain later gates.

## M1 acceptance and handoff

M1 is accepted when the Windows app launches and builds, all shell routes work, a typed command returns real Rust state, SQLite initializes and preserves data safely, relevant automated checks pass, and manual accessibility checks have evidence. Report any pending acceptance check explicitly. Stop at M1: no scheduler, reminders, notification plugin, tray lifecycle, autostart, or completed M4 workflows. Provide the M2 plan before implementing M2.

## Implementation notes

- Resolved packages are pinned in `package-lock.json` and `src-tauri/Cargo.lock`. TypeScript 6 is used because the current TypeScript ESLint plugin does not yet support TypeScript 7; ESLint 10 is compatible.
- Storage rollback and DTO serialization tests live beside their Rust modules; file-backed schema, constraint, preservation, and corruption tests live in `src-tauri/tests/storage.rs`.
- `playwright.config.ts` and `tests/ui/shell.spec.ts` add rendered navigation, keyboard, console, and overflow checks through installed Edge. Vite ignores native artifacts and workspace-local tools when watching files.
- The Tauri CLI forwards Cargo's lock flag after a second separator: `npm run tauri build -- --no-bundle -- --locked`.
- Design reference and intentional adaptations are in `docs/design/`. Native OS title chrome is not drawn in HTML. The UI uses real backend readiness and shows an explicit connection warning in browser-only development.
- Future PRD commands (`get_settings`, `update_settings`, break actions, pause/resume, history, notification test, and clear history) and scheduler events remain unregistered until their owning milestones implement them. Only `get_app_info` is available in M1.
