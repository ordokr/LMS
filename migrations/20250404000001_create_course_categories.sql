-- Create course_categories table
CREATE TABLE IF NOT EXISTS course_categories (
    id UUID PRIMARY KEY,
    canvas_course_id VARCHAR(255) NOT NULL,
    discourse_category_id BIGINT NOT NULL,
    sync_enabled BOOLEAN NOT NULL DEFAULT true,
    last_synced_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX idx_course_categories_canvas_course_id ON course_categories(canvas_course_id);
CREATE INDEX idx_course_categories_discourse_category_id ON course_categories(discourse_category_id);