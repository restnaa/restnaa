# Restnaa break-panel dismissal

Automatic native presentation bypasses framework visibility bookkeeping, so use shared native SW_HIDE as well as the framework hide request. Run native window calls on the owning UI thread. Keep hide IPC asynchronous. Outcome actions commit in Rust before requesting hide; failed commits stay visible and hide errors must not imply a failed saved outcome.

Close/Escape/titlebar X only hide; Rust later handles unanswered timeout. Observer auto-hide and explicit controls share the same native hide helper. Never record completion for visibility changes. Installed testing must inspect native visibility, not only return values or WebView focus flags.

Earlier exact reproduction/evidence is in Git history before 0.1.4. See IDENTITY_RESET.md for current automated checks; installed acceptance under the new identity is pending.
