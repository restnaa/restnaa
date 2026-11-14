# Unreleased security and history improvements

- Added optional water reminders, enabled by default every 60 minutes with an independent toggle/interval, native tray/notification paths and distinct local history outcomes. The interval is a software default, not a medical recommendation. Schema 4 preserves prior records and initializes the new deadline without replaying missed water reminders; back up the stopped profile before upgrading.

- Updated bundled SQLite, Tokio and rustix dependencies to address advisory findings; patched the lint tooling's minimatch dependency using a documented temporary override.
- Pinned CI actions to commit hashes, disabled persisted checkout credentials, bounded job duration and configured weekly Dependabot proposals.
- Added transactional database schema 3 indexes for history ranges, ordering and invalid-timestamp detection. Existing outcomes and scheduler settings are preserved. Back up the stopped profile before trying this source; older Beta 2 cannot open schema 3.

These changes have not been published as a new release. The Beta 2 notes below describe the existing public installer.

# Restnaa 0.1.4 Beta 2

An early Windows 10/11 x64 pre-release for feedback and contributions. This is unsigned beta software; clean-machine and several native Windows acceptance checks remain pending.

## Included

- The new crescent-and-resting-eye logo and simpler light interface: pale background, plum text and buttons.
- Independent eye and movement reminders, working hours, Quiet mode, timed/manual pause and local history.
- Automatic break panels with Done, Snooze and Skip, native notifications and tray fallback.
- Offline operation, local SQLite storage, no account or telemetry implementation, and opt-in startup.
- MIT source licensing and contributor documentation.

## Download and install

Download `Restnaa_0.1.4_x64-setup.exe` and its `.sha256` file from this release. Compare the complete SHA-256 checksum before installation. The checksum detects mismatches; the installer is not signed and does not establish a trusted publisher. Follow the attached INSTALLATION.md. Windows may need to download WebView2 if it is absent.

Quit any running Restnaa build before installing. Back up its stopped data directory if you want a recovery copy. This app uses `%APPDATA%\com.restnaa.desktop\restnaa.sqlite3`. Earlier development identities are not imported or deleted.

## Known limitations and verification

- Windows Do Not Disturb detection is best effort. Use Restnaa Pause or Quiet mode for guaranteed app quiet.
- Windows notifications have no Done/Snooze/Skip toast buttons; these actions live in the break panel. Action Center activation is not guaranteed.
- Overnight schedules are rejected. The process must stay running for reminders.
- Local SQLite is not encrypted by the app. Completion records the user's explicit action and makes no medical or exercise-verification claim.
- Clean Windows 10/11, installed banner/click/denial/DND behavior, actual startup at sign-in, physical sleep/wake, screen reader, prolonged use, signing and complete bundled dependency licensing review remain pending.
- The application implements no network calls; a strict zero-egress claim covering Windows/WebView2 has not been established.

The release workflow runs frontend and Rust checks and verifies the installer checksum and source revision before publication. This does not certify the pending manual checks. See the attached KNOWN_LIMITATIONS.md, PRIVACY.md, LICENSE and THIRD_PARTY_NOTICES.md, and the repository release audit for detailed evidence.

## Local candidate smoke checks

A local NSIS build installed and launched on an existing Windows 11 Pro host. The new UI, eye-break completion, pause/resume, single-instance prevention and Quit passed smoke checks; original settings were restored and startup stayed off. Notification submission returned unconfirmed delivery. The hosted installer attached here is built independently from the release tag and carries its own SHA-256; visible notification delivery, physical window dismissal and clean-machine acceptance remain pending.

## Packaging repair

Beta 1 was not published: its installer built, but signature-metadata export failed because an inherited PowerShell module could not load. Beta 2 explicitly loads the security module from the running PowerShell installation, tests that regression, and runs hosted packaging in the runner's native pwsh shell. Signature inspection still fails the export if it cannot run; no successful signature result is fabricated.
