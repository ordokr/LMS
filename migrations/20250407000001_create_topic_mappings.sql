-- Topic Mappings Table
CREATE TABLE IF NOT EXISTS topic_mappings (
    id UUID PRIMARY KEY,
    canvas_topic_id TEXT NOT NULL,
    discourse_topic_id TEXT NOT NULL,
    mapping_id UUID NOT NULL REFERENCES course_category_mappings(id) ON DELETE CASCADE,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_synced_at TIMESTAMPTZ,
    canvas_updated_at TIMESTAMPTZ,
    discourse_updated_at TIMESTAMPTZ,
    UNIQUE(canvas_topic_id),
    UNIQUE(discourse_topic_id)
);

CREATE INDEX idx_topic_mappings_canvas_id ON topic_mappings(canvas_topic_id);
CREATE INDEX idx_topic_mappings_discourse_id ON topic_mappings(discourse_topic_id);
CREATE INDEX idx_topic_mappings_mapping_id ON topic_mappings(mapping_id);

-- Post Mappings Table
CREATE TABLE IF NOT EXISTS post_mappings (
    id UUID PRIMARY KEY,
    canvas_entry_id TEXT NOT NULL,
    discourse_post_id TEXT NOT NULL,
    topic_mapping_id UUID NOT NULL REFERENCES topic_mappings(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(canvas_entry_id),
    UNIQUE(discourse_post_id)
);

CREATE INDEX idx_post_mappings_canvas_id ON post_mappings(canvas_entry_id);
CREATE INDEX idx_post_mappings_discourse_id ON post_mappings(discourse_post_id);
CREATE INDEX idx_post_mappings_topic_id ON post_mappings(topic_mapping_id);