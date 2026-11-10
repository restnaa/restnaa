# Restnaa privacy statement

Applies to Restnaa 0.1.4, 2026-11-10. This candidate uses its own Restnaa data directory; no previous development profile is migrated.

## What stays on this device

Restnaa stores a SQLite database in `%APPDATA%\com.restnaa.desktop\restnaa.sqlite3`, resolved through Windows/Tauri. It contains reminder preferences, local working days/hours, onboarding completion, startup/notification preferences, pause state, future UTC deadlines, open-reminder metadata, and break outcomes. Outcome records contain a random ID, category, completed/skipped/snoozed/unanswered result, timestamps and optional elapsed seconds after explicit Start.

An explicit Done is an action record, not proof of movement or eye rest. Restnaa offers no medical diagnosis or treatment and stores no medical conditions, names, email addresses, browsing history, webcam/screen recordings, key presses or active-app/window activity. It has no login, server, analytics, telemetry, cloud sync, remote updater or automatic beta-feedback transmission.

## Operating-system integrations

Windows receives local notification text and identifiers so it can display reminders. Notification history may remain in Windows after app history is cleared; Windows controls banners, lock-screen visibility, retention and Do not disturb. Restnaa does not override those controls. The tray remains available when banners are suppressed.

Start on login is off by default and changes only after explicit choice. Enabling it writes this user's Windows startup registration. It can be disabled in Restnaa or through Windows. The app reads its own notification permission and a shell attention hint for diagnostics; it does not identify which other app is active.

The packaged application contains no application network calls. Dependency installation and the WebView2 prerequisite download are build/install activities. Windows and Microsoft WebView2 are separate components. Microsoft documents required WebView2 diagnostic collection even when optional diagnostic collection is off. Microsoft Defender SmartScreen is enabled by default and may send information to Microsoft. Restnaa does not disable those security protections or control overall runtime collection. See [Microsoft WebView2 data/privacy](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/data-privacy) and [Microsoft Privacy Statement](https://privacy.microsoft.com/privacystatement). An earlier internal audit observed one outbound HTTPS connection in the app/runtime process tree; its owner and payload were not captured before it closed. A strict zero-egress or zero-component-telemetry guarantee is unverified.

## Retention, deletion and access

Local records remain until you choose to erase them. Settings → Clear history → Permanently clear history transactionally deletes break outcome rows, preserving preferences and active/future reminders. New outcomes may be recorded afterwards. This is logical SQLite deletion, not a guarantee of forensic erasure from database pages, backups, Windows notification history or disk storage.

The database is not encrypted by Restnaa; it relies on your Windows profile/device access controls. Copies or backups you create remain your responsibility. Uninstall normally retains local app data unless you explicitly select the uninstaller's Delete app data option. That option removes this application's data folders. No data is silently erased to repair corruption; a storage error requires deliberate recovery.

## Feedback and support

Any beta feedback or diagnostics are shared only if you choose to send them outside the app. Inspect attachments before sharing; database copies can contain your local break timestamps/preferences. The app does not upload reports automatically.
