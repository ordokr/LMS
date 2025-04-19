-- Ordo Quiz Module Schema

-- Quizzes table
CREATE TABLE IF NOT EXISTS quizzes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    course_id TEXT,
    author_id TEXT NOT NULL,
    time_limit INTEGER,
    passing_score INTEGER,
    shuffle_questions BOOLEAN DEFAULT 0,
    show_results BOOLEAN DEFAULT 1,
    visibility TEXT DEFAULT 'private',
    tags TEXT DEFAULT '[]',
    study_mode TEXT DEFAULT 'multiple_choice',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_quizzes_author_id ON quizzes(author_id);
CREATE INDEX IF NOT EXISTS idx_quizzes_course_id ON quizzes(course_id);
CREATE INDEX IF NOT EXISTS idx_quizzes_visibility ON quizzes(visibility);

-- Questions table
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    question_text TEXT NOT NULL,
    content TEXT,
    question_type TEXT NOT NULL,
    points INTEGER DEFAULT 1,
    position INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
);

CREATE INDEX IF NOT EXISTS idx_questions_quiz_id ON questions(quiz_id);

-- Answer options table
CREATE TABLE IF NOT EXISTS answer_options (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL,
    option_text TEXT NOT NULL,
    content TEXT,
    is_correct BOOLEAN NOT NULL,
    position INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (question_id) REFERENCES questions(id)
);

CREATE INDEX IF NOT EXISTS idx_answer_options_question_id ON answer_options(question_id);

-- Quiz attempts table
CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    status TEXT NOT NULL, -- 'in_progress', 'completed', 'abandoned'
    score REAL,
    start_time TEXT NOT NULL,
    end_time TEXT,
    time_limit INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
);

CREATE INDEX IF NOT EXISTS idx_quiz_attempts_quiz_id ON quiz_attempts(quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_user_id ON quiz_attempts(user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_status ON quiz_attempts(status);

-- User answers table
CREATE TABLE IF NOT EXISTS user_answers (
    id TEXT PRIMARY KEY,
    attempt_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    answer_option_id TEXT,
    answer_text TEXT,
    is_correct BOOLEAN,
    duration_ms INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (answer_option_id) REFERENCES answer_options(id)
);

CREATE INDEX IF NOT EXISTS idx_user_answers_attempt_id ON user_answers(attempt_id);
CREATE INDEX IF NOT EXISTS idx_user_answers_question_id ON user_answers(question_id);

-- Quiz settings table
CREATE TABLE IF NOT EXISTS quiz_settings (
    quiz_id TEXT PRIMARY KEY,
    allow_retakes BOOLEAN DEFAULT 1,
    max_attempts INTEGER,
    show_correct_answers BOOLEAN DEFAULT 1,
    show_correct_answers_after_completion BOOLEAN DEFAULT 1,
    time_limit INTEGER,
    passing_score INTEGER,
    shuffle_questions BOOLEAN DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
);

-- Activity tracking table
CREATE TABLE IF NOT EXISTS quiz_activities (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    quiz_id TEXT,
    question_id TEXT,
    attempt_id TEXT,
    activity_type TEXT NOT NULL,
    data TEXT, -- JSON data
    duration_ms INTEGER,
    timestamp TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    synced INTEGER DEFAULT 0,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id)
);

CREATE INDEX IF NOT EXISTS idx_quiz_activities_user_id ON quiz_activities(user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_quiz_id ON quiz_activities(quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_timestamp ON quiz_activities(timestamp);
CREATE INDEX IF NOT EXISTS idx_quiz_activities_synced ON quiz_activities(synced);

-- Sync items table
CREATE TABLE IF NOT EXISTS quiz_sync_items (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    data TEXT NOT NULL, -- JSON data
    priority TEXT NOT NULL, -- 'critical', 'high', 'medium', 'low'
    status TEXT NOT NULL, -- 'pending', 'in_progress', 'completed', 'failed'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_at TEXT,
    error TEXT,
    retry_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_status ON quiz_sync_items(status);
CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_entity ON quiz_sync_items(entity_type, entity_id);

-- Users table (simplified for standalone mode)
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
