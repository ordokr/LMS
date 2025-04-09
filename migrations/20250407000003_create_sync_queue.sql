CREATE TABLE IF NOT EXISTS sync_queue (
    id UUID PRIMARY KEY,
    topic_mapping_id UUID NOT NULL REFERENCES topic_mappings(id) ON DELETE CASCADE,
    sync_direction TEXT NOT NULL,
    status TEXT NOT NULL,
    attempt_count INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 3,
    last_attempt_at TIMESTAMPTZ,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_sync_queue_status ON sync_queue(status);
CREATE INDEX idx_sync_queue_topic_mapping ON sync_queue(topic_mapping_id);
CREATE INDEX idx_sync_queue_created_at ON sync_queue(created_at);