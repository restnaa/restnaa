ALTER TABLE reminder_state RENAME TO reminder_state_before_water;
CREATE TABLE reminder_state (
    category TEXT PRIMARY KEY NOT NULL CHECK(category IN ('eye', 'movement', 'water')),
    next_due_at_utc TEXT NOT NULL,
    open_reminder_id TEXT NULL,
    updated_at_utc TEXT NOT NULL
);
INSERT INTO reminder_state SELECT * FROM reminder_state_before_water;
DROP TABLE reminder_state_before_water;

ALTER TABLE break_events RENAME TO break_events_before_water;
CREATE TABLE break_events (
    id TEXT PRIMARY KEY NOT NULL,
    category TEXT NOT NULL CHECK(category IN ('eye', 'movement', 'water')),
    outcome TEXT NOT NULL CHECK(outcome IN ('completed', 'skipped', 'snoozed', 'unanswered')),
    triggered_at_utc TEXT NOT NULL,
    acted_at_utc TEXT NOT NULL,
    duration_seconds INTEGER NULL CHECK(duration_seconds IS NULL OR duration_seconds >= 0),
    created_at_utc TEXT NOT NULL
);
INSERT INTO break_events SELECT * FROM break_events_before_water;
DROP TABLE break_events_before_water;
CREATE INDEX break_events_created_at ON break_events(created_at_utc);
CREATE INDEX break_events_category_created_at ON break_events(category, created_at_utc);
CREATE INDEX break_events_acted_instant ON break_events(julianday(acted_at_utc) DESC, id DESC);
CREATE INDEX break_events_invalid_acted ON break_events(id) WHERE julianday(acted_at_utc) IS NULL;

-- Rust initializes only the new category using its injected clock. The marker
-- and new deadline are committed together by the scheduler's first save.
INSERT INTO settings(key, value_json, updated_at_utc)
SELECT 'water_upgrade_pending', 'true', updated_at_utc
FROM settings WHERE key='scheduler_settings';
