-- Create courses table
CREATE TABLE IF NOT EXISTS courses (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    instructor_id INTEGER NOT NULL REFERENCES users(id),
    start_date TEXT,
    end_date TEXT,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create course settings table
CREATE TABLE IF NOT EXISTS course_settings (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    allow_student_discussion_topics BOOLEAN NOT NULL DEFAULT TRUE,
    allow_student_discussion_editing BOOLEAN NOT NULL DEFAULT TRUE,
    allow_student_forum_attachments BOOLEAN NOT NULL DEFAULT TRUE,
    restrict_student_past_view BOOLEAN NOT NULL DEFAULT FALSE,
    restrict_student_future_view BOOLEAN NOT NULL DEFAULT FALSE,
    hide_final_grades BOOLEAN NOT NULL DEFAULT FALSE,
    hide_distribution_graphs BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(course_id)
);

-- Create course sections table
CREATE TABLE IF NOT EXISTS course_sections (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    name TEXT NOT NULL,
    start_date TEXT,
    end_date TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create course users (enrollments) table
CREATE TABLE IF NOT EXISTS course_users (
    id INTEGER PRIMARY KEY,
    course_id INTEGER NOT NULL REFERENCES courses(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    role TEXT NOT NULL,
    section_id INTEGER REFERENCES course_sections(id),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(course_id, user_id)
);

-- Add indexes for performance
CREATE INDEX IF NOT EXISTS idx_courses_instructor_id ON courses(instructor_id);
CREATE INDEX IF NOT EXISTS idx_course_sections_course_id ON course_sections(course_id);
CREATE INDEX IF NOT EXISTS idx_course_users_course_id ON course_users(course_id);
CREATE INDEX IF NOT EXISTS idx_course_users_user_id ON course_users(user_id);
CREATE INDEX IF NOT EXISTS idx_course_users_section_id ON course_users(section_id);