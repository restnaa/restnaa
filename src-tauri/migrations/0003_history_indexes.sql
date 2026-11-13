-- Match history's instant-based range and ordering without changing stored text.
-- Keep equivalent RFC3339 offsets and fractional precision supported.
CREATE INDEX break_events_acted_instant
    ON break_events(julianday(acted_at_utc) DESC, id DESC);

-- Preserve the global invalid-timestamp check without scanning valid history.
CREATE INDEX break_events_invalid_acted
    ON break_events(id) WHERE julianday(acted_at_utc) IS NULL;
