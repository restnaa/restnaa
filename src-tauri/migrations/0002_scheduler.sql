CREATE TABLE scheduler_runtime (
    id INTEGER PRIMARY KEY CHECK(id = 1),
    value_json TEXT NOT NULL,
    updated_at_utc TEXT NOT NULL
);
