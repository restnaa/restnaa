# Restnaa automatic break-window contract

A committed newly due eligible reminder may show the single break panel. Default On; explicit Off persists. Require enabled notifications/category, active state, current UUID and acceptable known OS attention/permission policy. App pause/work hours/Quiet mode are authoritative. DND is best effort; unknown/known blocked attention states suppress presentation.

Rust schedules and deduplicates per reminder UUID. No JS reminder timer, repeated reopening on refresh, overlapping panel, focus stealing, topmost window or OS preference change. Automatic show uses SW_SHOWNOACTIVATE on the owning thread. Pair with native SW_HIDE for Done/Snooze/Skip, Close/Escape/X and observer dismissal. Explicit window actions retain normal focus behavior.

Test both categories, UUID deduplication, opt-out persistence, pause/quiet/out-of-hours, permission/attention suppression, installed autonomous presentation/focus/dismissal and fallback. Current tests are in IDENTITY_RESET.md; new-identity installed OS checks remain pending.
