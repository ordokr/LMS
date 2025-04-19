-- Create unified topics table
CREATE TABLE IF NOT EXISTS topics (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    course_id TEXT,
    category_id TEXT,
    group_id TEXT,
    author_id TEXT,
    assignment_id TEXT,
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    topic_type TEXT NOT NULL,
    is_pinned INTEGER NOT NULL DEFAULT 0,
    is_locked INTEGER NOT NULL DEFAULT 0,
    allow_rating INTEGER NOT NULL DEFAULT 0,
    require_initial_post INTEGER NOT NULL DEFAULT 0,
    posted_at TEXT,
    last_reply_at TEXT,
    delayed_post_at TEXT,
    view_count INTEGER,
    reply_count INTEGER,
    participant_count INTEGER,
    canvas_id TEXT,
    discourse_id TEXT,
    slug TEXT,
    tags TEXT, -- JSON array
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE,
    FOREIGN KEY (assignment_id) REFERENCES assignments(id) ON DELETE SET NULL,
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_topics_course_id ON topics(course_id);
CREATE INDEX IF NOT EXISTS idx_topics_category_id ON topics(category_id);
CREATE INDEX IF NOT EXISTS idx_topics_group_id ON topics(group_id);
CREATE INDEX IF NOT EXISTS idx_topics_author_id ON topics(author_id);
CREATE INDEX IF NOT EXISTS idx_topics_assignment_id ON topics(assignment_id);
CREATE INDEX IF NOT EXISTS idx_topics_status ON topics(status);
CREATE INDEX IF NOT EXISTS idx_topics_topic_type ON topics(topic_type);
CREATE INDEX IF NOT EXISTS idx_topics_is_pinned ON topics(is_pinned);
CREATE INDEX IF NOT EXISTS idx_topics_posted_at ON topics(posted_at);
CREATE INDEX IF NOT EXISTS idx_topics_canvas_id ON topics(canvas_id);
CREATE INDEX IF NOT EXISTS idx_topics_discourse_id ON topics(discourse_id);
