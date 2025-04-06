CREATE TABLE IF NOT EXISTS assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    points_possible DOUBLE PRECISION,
    due_date TIMESTAMPTZ,
    available_from TIMESTAMPTZ,
    available_until TIMESTAMPTZ,
    submission_types TEXT[] NOT NULL,
    canvas_id VARCHAR(255) NOT NULL UNIQUE,
    topic_id UUID,  -- Will be linked later
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assignments_course_id ON assignments(course_id);
CREATE INDEX idx_assignments_canvas_id ON assignments(canvas_id);