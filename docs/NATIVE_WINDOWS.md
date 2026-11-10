# Restnaa native Windows behavior

Rust owns official Tauri notification/autostart plugins, tray and window lifecycle. Named local windows receive narrow command capabilities. App identifier: com.restnaa.desktop; executable: restnaa.exe.

Notifications have no Windows toast action buttons. Done/Snooze/Skip live in one break panel; the tray is the fallback. Automatic panels are default On with a persisted opt-out, app eligibility controls and best-effort DND. Known suppression/unknown attention states block automatic panels; an accepting hint does not prove DND off. No OS settings are bypassed.

Automatic Windows presentation uses SW_SHOWNOACTIVATE on the UI thread; native SW_HIDE is paired with it for all dismissal paths. No forced focus or always-on-top. Explicit user window actions can focus. Done records completion before hiding; Close/Escape/X do not complete. The Rust unanswered timeout remains authoritative.

Dashboard close hides to tray; explicit Quit exits fully. Startup is off until opted in. A single instance owns one scheduler for this app identity. Other development identities are separate apps and must be quit manually during testing.

## Pending installed acceptance

For the new identity, test native dispatch/banner/click/denial, tray menus, duplicate launch, Quit, login consent/actual sign-in, panel focus/dismissal, install/uninstall and OS DND without changing global settings. Test on clean Windows 10 and 11. No new-identity installed pass is asserted here.

Earlier internal plans and exact test evidence are preserved in Git history before 0.1.4. They do not certify the current identity. Current executed checks are recorded in [identity verification](IDENTITY_RESET.md).
