# Contributing to Restnaa

Thank you for helping improve quiet, reliable break reminders. Documentation, bug reports, accessibility fixes and deterministic tests are useful contributions.

## Getting started

Follow the Windows setup in README.md, read AGENTS.md, then inspect the relevant existing code. First-run setup is in Settings. The app is offline and needs no account, secrets or environment file. Use temporary databases for tests; never commit personal history or diagnostic screenshots.

Open a focused issue for bugs with version, Windows version, reproduction steps and expected/actual behavior. Remove private information. For timing bugs include working hours, pause/Quiet mode and category state as text; do not attach your database publicly. Discuss larger changes before implementation. Features outside the documented product boundaries need separate maintainer approval.

## Pull requests

1. Fork and create a focused branch.
2. Keep scheduler, validation, storage and OS integration in Rust. Do not introduce JS reminder timers, broad capabilities or network calls.
3. Preserve independent deadlines and all four distinct outcomes. Close/Escape must never imply completion. Preserve nonactivating automatic presentation and paired native hide.
4. Add regression tests for meaningful behavior changes; use injectable clocks instead of wall-clock sleeps.
5. Run the README check commands relevant to your change. For native changes, record installed Windows tests separately from browser tests. State any checks you could not run.
6. Explain the problem, final behavior, tests and limitations in the PR. Keep unrelated formatting or dependencies out.

CI runs on Windows. It is not evidence of clean Windows 10/11, physical sleep/wake, DND, sign-in or screen-reader validation. Dependency downloads are build-time activities, not runtime feature authorization.

Use semantic controls, keyboard access, visible focus, sufficient contrast and reduced-motion support. Use com.restnaa.desktop, restnaa.sqlite3 and restnaa.exe consistently. Future public identity changes need an explicit data/installation plan.

## License and conduct

By contributing, you agree your original contribution is provided under the project's MIT license. Include attribution/license information for third-party work; do not submit content you cannot license. No CLA is currently required. Follow CODE_OF_CONDUCT.md. Maintainers review scope and may request a smaller patch.
