CREATE TABLE IF NOT EXISTS sync_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sync_type TEXT NOT NULL,
    content_id TEXT,
    content_type TEXT,
    sync_time TEXT NOT NULL,
    success BOOLEAN NOT NULL DEFAULT 0,
    error_message TEXT,
    duration_ms INTEGER
);

CREATE INDEX IF NOT EXISTS idx_sync_history_sync_time ON sync_history(sync_time DESC);
CREATE INDEX IF NOT EXISTS idx_sync_history_content ON sync_history(content_type, content_id);