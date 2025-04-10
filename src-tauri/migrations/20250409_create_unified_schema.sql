-- Create tables for unified models

-- Users table
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    username TEXT NOT NULL,
    avatar TEXT,
    canvas_id TEXT,
    discourse_id TEXT,
    last_login TEXT,
    source_system TEXT,
    roles TEXT NOT NULL, -- JSON array
    metadata TEXT, -- JSON object
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(email),
    UNIQUE(username),
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Courses table
CREATE TABLE IF NOT EXISTS courses (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    canvas_specific_fields TEXT, -- JSON object
    discourse_specific_fields TEXT, -- JSON object
    UNIQUE(title)
);

-- Assignments table
CREATE TABLE IF NOT EXISTS assignments (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    course_id TEXT,
    due_date TEXT,
    points_possible REAL,
    submission_types TEXT, -- JSON array
    canvas_specific_fields TEXT, -- JSON object
    discourse_specific_fields TEXT, -- JSON object
    FOREIGN KEY (course_id) REFERENCES courses(id)
);

-- Discussions table
CREATE TABLE IF NOT EXISTS discussions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    course_id TEXT,
    user_id TEXT,
    canvas_specific_fields TEXT, -- JSON object
    discourse_specific_fields TEXT, -- JSON object
    FOREIGN KEY (course_id) REFERENCES courses(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Notifications table
CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    title TEXT NOT NULL,
    message TEXT,
    notification_type TEXT NOT NULL,
    read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    url TEXT,
    source_system TEXT NOT NULL,
    source_id TEXT,
    metadata TEXT, -- JSON object
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Sync status table
CREATE TABLE IF NOT EXISTS sync_status (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    last_synced_at TEXT NOT NULL,
    canvas_version TEXT,
    discourse_version TEXT,
    sync_status TEXT NOT NULL, -- "success", "error", "pending"
    error_message TEXT,
    UNIQUE(entity_type, entity_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_users_canvas_id ON users(canvas_id);
CREATE INDEX IF NOT EXISTS idx_users_discourse_id ON users(discourse_id);
CREATE INDEX IF NOT EXISTS idx_assignments_course_id ON assignments(course_id);
CREATE INDEX IF NOT EXISTS idx_discussions_course_id ON discussions(course_id);
CREATE INDEX IF NOT EXISTS idx_discussions_user_id ON discussions(user_id);
CREATE INDEX IF NOT EXISTS idx_notifications_user_id ON notifications(user_id);
CREATE INDEX IF NOT EXISTS idx_sync_status_entity ON sync_status(entity_type, entity_id);