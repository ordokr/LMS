CREATE TABLE IF NOT EXISTS course_category_mappings (
    id BIGSERIAL PRIMARY KEY,
    course_id BIGINT NOT NULL,
    category_id BIGINT NOT NULL,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    sync_topics BOOLEAN NOT NULL DEFAULT true,
    sync_users BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(course_id),
    UNIQUE(category_id)
);

CREATE INDEX idx_course_category_mappings_course_id ON course_category_mappings(course_id);
CREATE INDEX idx_course_category_mappings_category_id ON course_category_mappings(category_id);