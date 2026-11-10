CREATE TABLE settings (
    key TEXT PRIMARY KEY NOT NULL,
    value_json TEXT NOT NULL,
    updated_at_utc TEXT NOT NULL
);

CREATE TABLE reminder_state (
    category TEXT PRIMARY KEY NOT NULL CHECK(category IN ('eye', 'movement')),
    next_due_at_utc TEXT NOT NULL,
    open_reminder_id TEXT NULL,
    updated_at_utc TEXT NOT NULL
);

CREATE TABLE break_events (
    id TEXT PRIMARY KEY NOT NULL,
    category TEXT NOT NULL CHECK(category IN ('eye', 'movement')),
    outcome TEXT NOT NULL CHECK(outcome IN ('completed', 'skipped', 'snoozed', 'unanswered')),
    triggered_at_utc TEXT NOT NULL,
    acted_at_utc TEXT NOT NULL,
    duration_seconds INTEGER NULL CHECK(duration_seconds IS NULL OR duration_seconds >= 0),
    created_at_utc TEXT NOT NULL
);

CREATE INDEX break_events_created_at ON break_events(created_at_utc);
CREATE INDEX break_events_category_created_at ON break_events(category, created_at_utc);
