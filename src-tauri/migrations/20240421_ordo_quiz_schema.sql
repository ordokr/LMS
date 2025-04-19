-- Ordo Quiz Module Schema
-- This schema harmonizes with the existing models and avoids redundancy

-- Quiz table (if not already exists)
CREATE TABLE IF NOT EXISTS quizzes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    course_id TEXT,
    author_id TEXT NOT NULL,
    time_limit INTEGER, -- in seconds, NULL means no limit
    passing_score INTEGER, -- percentage
    shuffle_questions INTEGER DEFAULT 0,
    show_results INTEGER DEFAULT 1,
    visibility TEXT NOT NULL DEFAULT 'private', -- 'private', 'public', 'course'
    tags TEXT, -- JSON array of tags
    study_mode TEXT NOT NULL DEFAULT 'multiple_choice', -- 'multiple_choice', 'flashcard', 'adaptive'
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT,
    FOREIGN KEY (course_id) REFERENCES courses(id),
    FOREIGN KEY (author_id) REFERENCES users(id)
);

-- Question table (if not already exists)
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    content TEXT NOT NULL, -- JSON content with text, rich_text, and media
    question_type TEXT NOT NULL, -- 'multiple_choice', 'true_false', 'short_answer', 'matching', 'essay'
    points INTEGER DEFAULT 1,
    position INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
);

-- Answer options table (if not already exists)
CREATE TABLE IF NOT EXISTS answer_options (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL,
    content TEXT NOT NULL, -- JSON content with text, rich_text, and media
    is_correct INTEGER DEFAULT 0,
    position INTEGER,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE
);

-- Quiz settings table (if not already exists)
CREATE TABLE IF NOT EXISTS quiz_settings (
    quiz_id TEXT PRIMARY KEY,
    allow_retakes INTEGER DEFAULT 1,
    max_attempts INTEGER,
    show_correct_answers INTEGER DEFAULT 1,
    show_correct_answers_after_completion INTEGER DEFAULT 1,
    time_limit INTEGER, -- in seconds
    passing_score INTEGER, -- percentage
    shuffle_questions INTEGER DEFAULT 0,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
);

-- Quiz attempts table (if not already exists)
CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    start_time TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    end_time TEXT,
    score REAL,
    status TEXT DEFAULT 'in_progress', -- 'in_progress', 'completed', 'abandoned'
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- User answers table (if not already exists)
CREATE TABLE IF NOT EXISTS user_answers (
    id TEXT PRIMARY KEY,
    attempt_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    answer_option_id TEXT,
    text_answer TEXT,
    is_correct INTEGER,
    points_awarded REAL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES questions(id),
    FOREIGN KEY (answer_option_id) REFERENCES answer_options(id)
);

-- CMI5 integration table (if not already exists)
CREATE TABLE IF NOT EXISTS cmi5_sessions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    session_id TEXT NOT NULL UNIQUE,
    registration_id TEXT NOT NULL,
    actor_json TEXT NOT NULL,
    activity_id TEXT NOT NULL,
    return_url TEXT,
    status TEXT DEFAULT 'initialized', -- 'initialized', 'launched', 'in_progress', 'completed', 'passed', 'failed', 'abandoned', 'waived'
    score REAL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (quiz_id) REFERENCES quizzes(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_questions_quiz_id ON questions(quiz_id);
CREATE INDEX IF NOT EXISTS idx_answer_options_question_id ON answer_options(question_id);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_quiz_id ON quiz_attempts(quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_user_id ON quiz_attempts(user_id);
CREATE INDEX IF NOT EXISTS idx_user_answers_attempt_id ON user_answers(attempt_id);
CREATE INDEX IF NOT EXISTS idx_user_answers_question_id ON user_answers(question_id);
CREATE INDEX IF NOT EXISTS idx_cmi5_sessions_quiz_id ON cmi5_sessions(quiz_id);
CREATE INDEX IF NOT EXISTS idx_cmi5_sessions_user_id ON cmi5_sessions(user_id);
