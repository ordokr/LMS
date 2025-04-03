-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    avatar_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create user_roles table for role-based authorization
CREATE TABLE IF NOT EXISTS user_roles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    role TEXT NOT NULL, -- 'admin', 'teacher', 'student'
    context_type TEXT, -- 'course', 'forum', 'system'
    context_id INTEGER, -- ID of the specific context
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, role, context_type, context_id)
);

-- Create forum trust levels table
CREATE TABLE IF NOT EXISTS forum_trust_levels (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    trust_level INTEGER NOT NULL DEFAULT 0,
    posts_read INTEGER NOT NULL DEFAULT 0,
    posts_created INTEGER NOT NULL DEFAULT 0,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create courses table
CREATE TABLE IF NOT EXISTS courses (
    id INTEGER PRIMARY KEY,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    instructor_id INTEGER NOT NULL REFERENCES users(id),
    start_date TIMESTAMP,
    end_date TIMESTAMP,
    status TEXT NOT NULL DEFAULT 'draft', -- 'draft', 'active', 'archived'
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create modules table
CREATE TABLE IF NOT EXISTS modules (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    title TEXT NOT NULL,
    description TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create enrollments table
CREATE TABLE IF NOT EXISTS enrollments (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    course_id INTEGER NOT NULL REFERENCES courses(id),
    role TEXT NOT NULL, -- 'student', 'teacher', 'teaching_assistant', 'observer'
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, course_id)
);

-- Create assignments table
CREATE TABLE IF NOT EXISTS assignments (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    due_date TIMESTAMP,
    points INTEGER,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create submissions table
CREATE TABLE IF NOT EXISTS submissions (
    id INTEGER PRIMARY KEY,
    assignment_id INTEGER NOT NULL REFERENCES assignments(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    submitted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    grade REAL,
    feedback TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(assignment_id, user_id)
);

-- Create forum_categories table
CREATE TABLE IF NOT EXISTS forum_categories (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    description TEXT,
    course_id INTEGER REFERENCES courses(id),
    parent_id INTEGER REFERENCES forum_categories(id),
    color TEXT,
    text_color TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create forum_topics table
CREATE TABLE IF NOT EXISTS forum_topics (
    id INTEGER PRIMARY KEY,
    category_id INTEGER NOT NULL REFERENCES forum_categories(id),
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    user_id INTEGER NOT NULL REFERENCES users(id),
    pinned BOOLEAN NOT NULL DEFAULT FALSE,
    locked BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_post_at TIMESTAMP,
    view_count INTEGER NOT NULL DEFAULT 0
);

-- Create forum_posts table
CREATE TABLE IF NOT EXISTS forum_posts (
    id INTEGER PRIMARY KEY,
    topic_id INTEGER NOT NULL REFERENCES forum_topics(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    content TEXT NOT NULL,
    is_solution BOOLEAN NOT NULL DEFAULT FALSE,
    parent_id INTEGER REFERENCES forum_posts(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create course_forum_settings table
CREATE TABLE IF NOT EXISTS course_forum_settings (
    course_id INTEGER PRIMARY KEY REFERENCES courses(id),
    default_category_id INTEGER REFERENCES forum_categories(id),
    auto_create_topics_for_announcements BOOLEAN NOT NULL DEFAULT false,
    auto_create_topics_for_assignments BOOLEAN NOT NULL DEFAULT false,
    student_can_create_topics BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create content_references table for cross-system references
CREATE TABLE IF NOT EXISTS content_references (
    id INTEGER PRIMARY KEY,
    source_type TEXT NOT NULL, -- 'forum_post', 'course_page', 'assignment', etc.
    source_id INTEGER NOT NULL,
    target_type TEXT NOT NULL,
    target_id INTEGER NOT NULL,
    reference_type TEXT NOT NULL, -- 'embed', 'link', 'mention'
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(source_type, source_id, target_type, target_id, reference_type)
);

-- Create sync_operations table for offline sync
CREATE TABLE IF NOT EXISTS sync_operations (
    id INTEGER PRIMARY KEY,
    device_id TEXT NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id),
    operation_type TEXT NOT NULL, -- 'create', 'update', 'delete', 'reference'
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    payload TEXT NOT NULL, -- JSON payload
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    synced BOOLEAN NOT NULL DEFAULT FALSE,
    synced_at TIMESTAMP
);

-- Create an index to support offline sync
CREATE INDEX idx_sync_operations_synced ON sync_operations(synced, created_at);