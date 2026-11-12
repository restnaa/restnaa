# Contributing to Restnaa

Thank you for helping improve quiet, reliable break reminders. Documentation, bug reports, accessibility fixes and deterministic tests are useful contributions.

## Getting started

Follow the Windows setup in README.md, read AGENTS.md, then inspect the relevant existing code. First-run setup is in Settings. The app is offline and needs no account, secrets or environment file. Use temporary databases for tests; never commit personal history or diagnostic screenshots.

Open a focused issue for bugs with version, Windows version, reproduction steps and expected/actual behavior. Remove private information. For timing bugs include working hours, pause/Quiet mode and category state as text; do not attach your database publicly. Discuss larger changes before implementation. Features outside the documented product boundaries need separate maintainer approval.

## Pull requests

1. Fork and create a focused branch from the latest `main`, using `feat/<description>`, `fix/<description>` or `docs/<description>`.
2. Keep scheduler, validation, storage and OS integration in Rust. Do not introduce JS reminder timers, broad capabilities or network calls.
3. Preserve independent deadlines and all four distinct outcomes. Close/Escape must never imply completion. Preserve nonactivating automatic presentation and paired native hide.
4. Add regression tests for meaningful behavior changes; use injectable clocks instead of wall-clock sleeps.
5. Run the README check commands relevant to your change. For native changes, record installed Windows tests separately from browser tests. State any checks you could not run.
6. Explain the problem, final behavior, tests and limitations in the PR. Keep unrelated formatting or dependencies out.

CI runs on Windows. It is not evidence of clean Windows 10/11, physical sleep/wake, DND, sign-in or screen-reader validation. Dependency downloads are build-time activities, not runtime feature authorization.

## Branch workflow

`main` is the stable integration branch. Keep changes on short-lived branches and open pull requests targeting `main`; there is no permanent `dev` branch. For example:

```powershell
git switch main
git pull --ff-only origin main
git switch -c fix/reminder-dismissal
# Make and validate the focused change, then commit it.
git push -u origin fix/reminder-dismissal
```

Open a pull request from the pushed branch to `main`. Contributors without repository write access should push to their fork instead. Merge after review and passing Windows CI, then delete the merged branch. A passing CI run does not make an unverified build a stable release. Publish releases from reviewed `main` commits using version tags as described below.

Maintainers should protect `main` in GitHub Settings > Branches: require a pull request, require the `release-candidate` status check from the Windows desktop workflow, require branches to be up to date, require resolved conversations, and block force pushes and deletion. These are intended repository settings; this document does not enable them. An additional approval is encouraged when another maintainer is available, but is not required while there is only one active maintainer. Do not require the tag-only release workflow on pull requests.

Use semantic controls, keyboard access, visible focus, sufficient contrast and reduced-motion support. Use com.restnaa.desktop, restnaa.sqlite3 and restnaa.exe consistently. Future public identity changes need an explicit data/installation plan.

## Visual identity

Use the supplied artwork in `public/brand/`. `restnaa-logo.png` is the original transparent logo; `restnaa-lockup.png` places it on a light background for readable documentation. The crescent-and-eye `restnaa-mark.png` is used in the app, favicon and Windows icons. Use `#F0F8EA` as the dominant background, `#260C1A` for primary text/actions and `#432E36` for secondary details in `src/styles.css`. Keep Dashboard, Settings, History and break panels flat and simple, without decorative landscapes, gradients or oversized badges. Preserve visible focus, contrast, reduced motion and text labels for state.

## License and conduct

By contributing, you agree your original contribution is provided under the project's MIT license. Include attribution/license information for third-party work; do not submit content you cannot license. No CLA is currently required. Follow CODE_OF_CONDUCT.md. Maintainers review scope and may request a smaller patch.

## Maintainer beta releases

Update release notes, installation guidance and the audit before publishing. From a reviewed, clean commit on main, push an annotated tag `v<package version>-beta.<number>` (for example `v0.1.4-beta.1`). The Publish Windows beta workflow validates the tag, runs frontend/Rust checks, builds NSIS and uploads the candidate. A separate job with release-write permission verifies its source commit, clean worktree and SHA-256, uploads eight assets to a draft, then publishes it as a pre-release. Build jobs have read-only repository access. Existing tags/releases are never overwritten by the workflow. Keep clean-machine and installed native checks explicit in the release notes.
