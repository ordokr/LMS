-- Create modules table
CREATE TABLE IF NOT EXISTS modules (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    name TEXT NOT NULL,
    position INTEGER,
    unlock_at TEXT,
    require_sequential_progress BOOLEAN NOT NULL DEFAULT FALSE,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    items_count INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create module items table
CREATE TABLE IF NOT EXISTS module_items (
    id INTEGER PRIMARY KEY,
    module_id INTEGER NOT NULL REFERENCES modules(id),
    title TEXT NOT NULL,
    position INTEGER,
    indent INTEGER NOT NULL DEFAULT 0,
    item_type TEXT NOT NULL,
    content_id INTEGER,
    page_url TEXT,
    external_url TEXT,
    completion_requirement_type TEXT,
    min_score REAL,
    completed BOOLEAN,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Add indexes for performance
CREATE INDEX IF NOT EXISTS idx_modules_course_id ON modules(course_id);
CREATE INDEX IF NOT EXISTS idx_modules_position ON modules(position);
CREATE INDEX IF NOT EXISTS idx_module_items_module_id ON module_items(module_id);
CREATE INDEX IF NOT EXISTS idx_module_items_position ON module_items(position);
CREATE INDEX IF NOT EXISTS idx_module_items_content_id ON module_items(content_id);