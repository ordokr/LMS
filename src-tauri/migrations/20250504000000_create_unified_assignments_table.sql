-- Create unified assignments table
CREATE TABLE IF NOT EXISTS assignments (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    course_id TEXT,
    due_date TEXT,
    unlock_date TEXT,
    lock_date TEXT,
    points_possible REAL,
    grading_type TEXT NOT NULL,
    submission_types TEXT NOT NULL, -- JSON array
    status TEXT NOT NULL,
    is_published INTEGER NOT NULL DEFAULT 0,
    group_category_id TEXT,
    assignment_group_id TEXT,
    peer_reviews INTEGER NOT NULL DEFAULT 0,
    automatic_peer_reviews INTEGER NOT NULL DEFAULT 0,
    peer_review_count INTEGER,
    canvas_id TEXT,
    discourse_id TEXT,
    quiz_id TEXT,
    discussion_topic_id TEXT,
    position INTEGER,
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_assignments_course_id ON assignments(course_id);
CREATE INDEX IF NOT EXISTS idx_assignments_status ON assignments(status);
CREATE INDEX IF NOT EXISTS idx_assignments_due_date ON assignments(due_date);
CREATE INDEX IF NOT EXISTS idx_assignments_canvas_id ON assignments(canvas_id);
CREATE INDEX IF NOT EXISTS idx_assignments_discourse_id ON assignments(discourse_id);
CREATE INDEX IF NOT EXISTS idx_assignments_quiz_id ON assignments(quiz_id);
CREATE INDEX IF NOT EXISTS idx_assignments_discussion_topic_id ON assignments(discussion_topic_id);
CREATE INDEX IF NOT EXISTS idx_assignments_group_category_id ON assignments(group_category_id);
CREATE INDEX IF NOT EXISTS idx_assignments_assignment_group_id ON assignments(assignment_group_id);
