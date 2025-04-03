-- Create mapping tables for forum integration

-- Course to Forum Category mapping
CREATE TABLE IF NOT EXISTS course_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    course_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (course_id) REFERENCES courses (id),
    FOREIGN KEY (category_id) REFERENCES forum_categories (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_course_forum_mappings_course_id ON course_forum_mappings (course_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_course_forum_mappings_category_id ON course_forum_mappings (category_id);

-- Module to Forum Topic mapping
CREATE TABLE IF NOT EXISTS module_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    module_id INTEGER NOT NULL,
    topic_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (module_id) REFERENCES modules (id),
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_module_forum_mappings_module_id ON module_forum_mappings (module_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_module_forum_mappings_topic_id ON module_forum_mappings (topic_id);

-- Assignment to Forum Topic mapping
CREATE TABLE IF NOT EXISTS assignment_forum_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    assignment_id INTEGER NOT NULL,
    topic_id INTEGER NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (assignment_id) REFERENCES assignments (id),
    FOREIGN KEY (topic_id) REFERENCES forum_topics (id)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_assignment_forum_mappings_assignment_id ON assignment_forum_mappings (assignment_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_assignment_forum_mappings_topic_id ON assignment_forum_mappings (topic_id);

-- Add course_id column to forum_categories table if it doesn't exist
PRAGMA foreign_keys=off;

-- Check if column exists first (SQLite doesn't support IF NOT EXISTS for ALTER TABLE)
BEGIN TRANSACTION;

-- Only add column if it doesn't exist
INSERT INTO pragma_table_info('forum_categories') VALUES (NULL, 'course_id', 'INTEGER', 0, NULL, 0)
WHERE NOT EXISTS (
    SELECT 1 FROM pragma_table_info('forum_categories') WHERE name = 'course_id'
);

-- Add foreign key in a new create table
CREATE TABLE IF NOT EXISTS forum_categories_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT,
    parent_id INTEGER,
    course_id INTEGER,
    color TEXT,
    text_color TEXT,
    topic_count INTEGER NOT NULL DEFAULT 0,
    post_count INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    FOREIGN KEY (parent_id) REFERENCES forum_categories (id),
    FOREIGN KEY (course_id) REFERENCES courses (id)
);

-- Copy data
INSERT INTO forum_categories_new 
    SELECT id, name, slug, description, parent_id, course_id, color, text_color, 
           topic_count, post_count, created_at, updated_at, deleted_at
    FROM forum_categories;

-- Drop old table
DROP TABLE forum_categories;

-- Rename new table
ALTER TABLE forum_categories_new RENAME TO forum_categories;

-- Recreate indexes
CREATE INDEX IF NOT EXISTS idx_forum_categories_slug ON forum_categories (slug);
CREATE INDEX IF NOT EXISTS idx_forum_categories_parent_id ON forum_categories (parent_id);
CREATE INDEX IF NOT EXISTS idx_forum_categories_course_id ON forum_categories (course_id);

COMMIT;
PRAGMA foreign_keys=on;