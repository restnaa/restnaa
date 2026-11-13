# M2 scheduler contract and integration

`scheduler/engine.rs` is the state machine. `scheduler/service.rs` runs one worker thread and owns both the engine and SQLite connection. Commands enter a channel; frontend code never schedules reminders. M3 registers the official single-instance plugin before storage or scheduler initialization. Native delivery happens only after the scheduler commit; see `NATIVE_WINDOWS.md`.

M4 adds typed `get_history(range: today | sevenDays)` and `clear_history(confirmed: boolean)` requests on the same worker channel, with a main-window-only capability. History uses current local calendar dates and acted-at timestamps, with all-period totals and up to 100 recent events. Successful deletion emits `history-changed` only after its transaction commits. It does not change scheduler revision, deadlines, settings, open reminders, or later outcomes. Failed deletion rolls back and reports an error. `onboardingCompleted` is stored with settings, defaults false for older installs, and becomes true only on a successful Finish setup save. Native preferences and pause state still require their dedicated commands; form saves retain their latest values.

## Clock and eligibility

`Clock` injects UTC, monotonic elapsed time, timezone identity/offset, and local-time conversion. Windows uses the system timezone. Working days are Monday=0 through Sunday=6; work hours are a same-day inclusive start/exclusive end. Reject overnight windows, duplicate/empty day lists, sub-minute work boundaries, and out-of-range intervals. Settings use `HH:mm:ss` JSON time values with seconds zero.

Outside working hours are the scheduled quiet hours. `quietMode` additionally suppresses all reminders until disabled. Manual pause and absolute UTC pause deadlines persist. “Until tomorrow” means next local midnight, with delivery still subject to working hours. Autostart and OS DND detection are not part of this engine.

The runtime waits for the next relevant monotonic deadline, work boundary, pause expiry, or unanswered timeout. A 30-second maximum wait checks for clock/timezone changes; it does not re-notify. A gap greater than 90 seconds, a wall/monotonic discrepancy greater than two seconds, monotonic regression, or timezone/offset change invokes recovery. A physical resume that produces no detectable clock/gap change is rechecked on the next normal wake; native power/time event hooks can be added in M3.

Calendar boundaries handle DST explicitly using Chrono's `LocalResult`: choose the earlier fold occurrence unless it has passed and the second occurrence is still upcoming; advance a nonexistent boundary to the first valid local minute. No frontend timezone is trusted for eligibility.

## Transitions

- Default intervals are eye 20 minutes and movement 45 minutes, weekdays 09:00–18:00. Fresh initialization schedules future eligible deadlines.
- Each category has its own UTC deadline. Only one reminder may be open globally. Deliveries are separated by at least two minutes; a nearby second deadline is moved beyond that guard. An open reminder prevents overlap even after the guard expires.
- Each dashboard category also exposes a derived `status`: scheduled, open, disabled, paused, quiet, or outsideWorkHours. Disabled categories stay disabled even when the other category is paused; these flags are derived from authoritative state rather than stored redundantly.
- Complete and Skip record distinct outcomes and schedule from the action time plus that category's interval. Completion always requires an explicit command. Starting a break alone never completes it.
- Snooze records `snoozed` and schedules five minutes later, shifted into the next working window if necessary.
- An unanswered reminder expires after five minutes and schedules from dismissal plus interval. Suppression or settings changes dismiss an open reminder as `unanswered`, never `completed`. UI closing/Esc must leave completion unchanged; without an explicit action the Rust timeout handles the reminder.
- Settings changes, pause expiration, and explicit resume schedule fresh intervals to avoid an immediate backlog. Valid no-op settings updates do not move deadlines. Use pause/resume commands to change pause fields; `update_settings` cannot bypass that boundary.
- Start opens an eligible manual break, or starts the existing reminder of the same category. Other open categories, suppression, spacing, expired IDs, and duplicate/stale outcome actions are rejected. Durations are optional monotonic elapsed time from explicit Start to Complete, not proof that an exercise happened.

## Recovery and storage

Restart preserves earlier valid future deadlines, bounded by a fresh category interval shifted into the current work calendar, and bounds persisted cooldown to two minutes. This avoids extra delay after a backwards wall-clock adjustment while closed. Overdue deadlines are reset from recovery time, except at most one eligible most-overdue category can become due now. Previously open reminders become unanswered once; their category is not replayed. Ineligible recovery emits nothing. Expired timed pauses clear and schedule fresh intervals without replaying suppressed reminders. Wall/timezone changes rebase deadlines; backwards wall changes never leave reminders waiting an additional clock-shift duration. Clock discontinuities retain remaining monotonic delivery spacing, capped at two minutes if monotonic time regressed. UTC event timestamps reflect the observed wall clock and can move backwards; elapsed durations use monotonic time.

Migration 2 adds `scheduler_runtime` without replacing M1 tables or history. Typed settings live in `settings.scheduler_settings`; category deadlines/open IDs live in `reminder_state`; supplemental open-reminder, cooldown, and revision metadata live in `scheduler_runtime`. Outcomes remain in `break_events`. Invalid or inconsistent persisted data fails visibly without resetting history.

Migration 3 adds an expression index on `julianday(acted_at_utc), id` and a partial index for invalid acted-at timestamps. History range/order queries and the global invalid-timestamp guard use indexes instead of scanning all stored history. Existing text timestamps, mixed offsets, submillisecond precision guards, local-day counts and the latest-100 limit retain their behavior. The migration is transactional and does not rewrite or delete outcomes. A schema-3 database requires this version or newer; older builds must not open it.

Every transition is applied to a candidate engine, then settings, both deadlines, metadata, and new events commit in one transaction. Only committed changes replace live memory and produce events. A write failure stops that worker's scheduling until app restart, emits a visible error, and rejects subsequent commands. No database polling or silent data reset occurs.

## Typed API

Implemented commands: `get_app_info`, `get_dashboard_state`, `get_settings`, `update_settings`, `start_break`, `complete_break`, `skip_break`, `snooze_break`, `pause_reminders`, `resume_reminders`. Outcome commands require `reminderId` (UUID); start requires `category`; pause accepts `{ kind: 'minutes', minutes: 30 | 60 | 120 }`, `{ kind: 'manual' }`, or `{ kind: 'untilTomorrow' }`.

Events target only the trusted main window:

- `scheduler-state-changed`: `{ kind: 'state', state: DashboardState }` or `{ kind: 'error', message }`.
- `reminder-due`: committed `OpenReminder`.
- `break-event-recorded`: committed `BreakEvent`.
- `settings-updated`: committed `SchedulerSettings`.

The main window can listen/unlisten but cannot emit frontend events or invoke arbitrary SQL, filesystem, shell, or HTTP APIs. The React subscription registers before fetching a snapshot, discards older revisions, cleans up late registrations, and keeps a terminal storage error visible. Dates are formatted from backend UTC strings; no JS interval or countdown runs. Runtime data is real; unavailable OS alerts and break controls remain explicitly labeled.

## Verification boundaries

`tests/scheduler.rs` uses a fake UTC/monotonic clock and real timezone rules; `tests/scheduler_storage.rs` uses isolated file-backed SQLite databases, upgrades M1 fixtures, and injects transaction failure. Frontend tests cover event ordering, cleanup, failure states, typed command arguments, and suppressed-deadline rendering. These deterministic cases do not claim physical sleep/wake, live OS timezone changes, or installed Windows notification validation.

References: [Chrono local time ambiguity](https://docs.rs/chrono/latest/chrono/offset/enum.LocalResult.html), [Tauri frontend events](https://v2.tauri.app/develop/calling-frontend/).
