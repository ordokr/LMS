-- Create unified submissions table
CREATE TABLE IF NOT EXISTS submissions (
    id TEXT PRIMARY KEY,
    assignment_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    submission_type TEXT,
    content TEXT,
    url TEXT,
    attachment_ids TEXT, -- JSON array
    status TEXT NOT NULL,
    submitted_at TEXT,
    attempt INTEGER NOT NULL DEFAULT 1,
    late INTEGER NOT NULL DEFAULT 0,
    missing INTEGER NOT NULL DEFAULT 0,
    excused INTEGER NOT NULL DEFAULT 0,
    grade TEXT,
    score REAL,
    points_deducted REAL,
    graded_at TEXT,
    grader_id TEXT,
    grade_matches_current INTEGER NOT NULL DEFAULT 1,
    posted_at TEXT,
    canvas_id TEXT,
    discourse_id TEXT,
    quiz_submission_id TEXT,
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (grader_id) REFERENCES users(id) ON DELETE SET NULL,
    UNIQUE(canvas_id),
    UNIQUE(discourse_id),
    UNIQUE(assignment_id, user_id)
);

-- Create submission comments table
CREATE TABLE IF NOT EXISTS submission_comments (
    id TEXT PRIMARY KEY,
    submission_id TEXT NOT NULL,
    author_id TEXT NOT NULL,
    comment TEXT NOT NULL,
    created_at TEXT NOT NULL,
    attachment_ids TEXT, -- JSON array
    is_hidden INTEGER NOT NULL DEFAULT 0,
    is_draft INTEGER NOT NULL DEFAULT 0,
    
    -- Constraints
    FOREIGN KEY (submission_id) REFERENCES submissions(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_submissions_assignment_id ON submissions(assignment_id);
CREATE INDEX IF NOT EXISTS idx_submissions_user_id ON submissions(user_id);
CREATE INDEX IF NOT EXISTS idx_submissions_status ON submissions(status);
CREATE INDEX IF NOT EXISTS idx_submissions_submitted_at ON submissions(submitted_at);
CREATE INDEX IF NOT EXISTS idx_submissions_graded_at ON submissions(graded_at);
CREATE INDEX IF NOT EXISTS idx_submissions_canvas_id ON submissions(canvas_id);
CREATE INDEX IF NOT EXISTS idx_submissions_discourse_id ON submissions(discourse_id);
CREATE INDEX IF NOT EXISTS idx_submissions_late ON submissions(late);
CREATE INDEX IF NOT EXISTS idx_submissions_missing ON submissions(missing);

CREATE INDEX IF NOT EXISTS idx_submission_comments_submission_id ON submission_comments(submission_id);
CREATE INDEX IF NOT EXISTS idx_submission_comments_author_id ON submission_comments(author_id);
CREATE INDEX IF NOT EXISTS idx_submission_comments_created_at ON submission_comments(created_at);
