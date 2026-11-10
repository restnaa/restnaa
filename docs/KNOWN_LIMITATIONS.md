# Restnaa known limitations

0.1.4 is an unsigned release candidate. See [current verification](IDENTITY_RESET.md) and [release gates](RELEASE_AUDIT.md). No stable binary release approval is claimed.

- Current identity is com.restnaa.desktop, binary restnaa.exe, database restnaa.sqlite3. Earlier development profiles are not imported or deleted.
- New-identity installed notifications, tray/Quit/single instance, startup/sign-in and uninstall behavior need acceptance. Clean Windows 10/11, physical sleep/wake/session/time changes, screen reader and seven-day use remain pending.
- DND is best effort. Known suppression blocks panels, but an accepting attention hint can coexist with DND. Use Restnaa Pause or Quiet mode for guaranteed app quiet.
- Process must remain running. No Windows toast action buttons, exercise verification, full-screen lock or medical claims. Suggested break durations are text; no fabricated countdown.
- Overnight schedules are rejected. Snooze and unanswered timeout are five minutes; deliveries are spaced at least two minutes. History details cap at 100 while totals cover the selected period. Refresh after midnight/timezone changes if needed.
- Clear history logically deletes outcome rows, not preferences, Windows history, backups or forensic remnants. SQLite is not encrypted by the app.
- WebView2 installation may need a prerequisite download. The app implements no network calls, but previous internal runtime-tree HTTPS traffic leaves strict zero-egress unproven. No global security protections are disabled.
- Signing/SmartScreen reputation, trademark review and full binary third-party licensing review remain pending. MIT project licensing does not relicense dependencies.
