-- Create unified courses table
CREATE TABLE IF NOT EXISTS courses (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    status TEXT NOT NULL,
    visibility TEXT NOT NULL,
    is_public INTEGER NOT NULL DEFAULT 0,
    is_published INTEGER NOT NULL DEFAULT 0,
    start_date TEXT,
    end_date TEXT,
    instructor_id TEXT,
    allow_self_enrollment INTEGER NOT NULL DEFAULT 0,
    enrollment_code TEXT,
    enrollment_count INTEGER,
    syllabus_body TEXT,
    homepage_type TEXT NOT NULL,
    default_view TEXT NOT NULL,
    theme_color TEXT,
    banner_image_url TEXT,
    timezone TEXT,
    license TEXT,
    canvas_id TEXT,
    discourse_id TEXT,
    category_id TEXT,
    slug TEXT,
    color TEXT,
    position INTEGER,
    parent_id TEXT,
    last_sync TEXT,
    source_system TEXT,
    metadata TEXT, -- JSON object
    
    -- Constraints
    UNIQUE(code),
    UNIQUE(canvas_id),
    UNIQUE(discourse_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_courses_code ON courses(code);
CREATE INDEX IF NOT EXISTS idx_courses_canvas_id ON courses(canvas_id);
CREATE INDEX IF NOT EXISTS idx_courses_discourse_id ON courses(discourse_id);
CREATE INDEX IF NOT EXISTS idx_courses_instructor_id ON courses(instructor_id);
CREATE INDEX IF NOT EXISTS idx_courses_status ON courses(status);
CREATE INDEX IF NOT EXISTS idx_courses_source_system ON courses(source_system);
