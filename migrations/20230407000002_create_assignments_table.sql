CREATE TABLE IF NOT EXISTS assignments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    canvas_id VARCHAR(255) NOT NULL UNIQUE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    points_possible FLOAT NOT NULL DEFAULT 0,
    due_date TIMESTAMPTZ,
    unlock_date TIMESTAMPTZ,
    lock_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    topic_id UUID REFERENCES topics(id) ON DELETE SET NULL
);

CREATE INDEX idx_assignments_course_id ON assignments(course_id);
CREATE INDEX idx_assignments_canvas_id ON assignments(canvas_id);
CREATE INDEX idx_assignments_topic_id ON assignments(topic_id);

-- Add constraint to topics table for assignment_id after assignments table is created
ALTER TABLE topics
ADD CONSTRAINT fk_topics_assignment_id
FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE SET NULL;