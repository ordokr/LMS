-- Quiz Templates Schema

-- Quiz templates table
CREATE TABLE IF NOT EXISTS quiz_templates (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author_id TEXT,
    category TEXT NOT NULL,
    tags TEXT,
    default_study_mode TEXT NOT NULL,
    default_visibility TEXT NOT NULL,
    is_public INTEGER NOT NULL DEFAULT 0,
    usage_count INTEGER NOT NULL DEFAULT 0,
    rating REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Quiz question templates table
CREATE TABLE IF NOT EXISTS quiz_question_templates (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    text TEXT NOT NULL,
    description TEXT,
    answer_type TEXT NOT NULL,
    placeholder_text TEXT,
    example_answers TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES quiz_templates (id) ON DELETE CASCADE
);

-- Quiz template ratings table
CREATE TABLE IF NOT EXISTS quiz_template_ratings (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    rating REAL NOT NULL,
    comment TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (template_id) REFERENCES quiz_templates (id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_templates_author_id ON quiz_templates (author_id);
CREATE INDEX IF NOT EXISTS idx_quiz_templates_category ON quiz_templates (category);
CREATE INDEX IF NOT EXISTS idx_quiz_templates_is_public ON quiz_templates (is_public);
CREATE INDEX IF NOT EXISTS idx_quiz_question_templates_template_id ON quiz_question_templates (template_id);
CREATE INDEX IF NOT EXISTS idx_quiz_template_ratings_template_id ON quiz_template_ratings (template_id);
CREATE INDEX IF NOT EXISTS idx_quiz_template_ratings_user_id ON quiz_template_ratings (user_id);
