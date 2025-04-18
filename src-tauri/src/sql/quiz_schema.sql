-- Quiz module schema

-- Main quiz table
CREATE TABLE IF NOT EXISTS quizzes (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    author_id TEXT,
    visibility TEXT NOT NULL,
    tags TEXT NOT NULL,
    study_mode TEXT NOT NULL
);

-- Quiz settings
CREATE TABLE IF NOT EXISTS quiz_settings (
    quiz_id TEXT PRIMARY KEY,
    shuffle_questions INTEGER NOT NULL DEFAULT 0,
    time_limit INTEGER DEFAULT NULL,
    allow_retries INTEGER NOT NULL DEFAULT 1,
    show_correct_answers INTEGER NOT NULL DEFAULT 1,
    passing_score REAL DEFAULT NULL,
    study_mode TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);

-- Questions
CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    content TEXT NOT NULL,
    answer_type TEXT NOT NULL,
    correct_answer TEXT NOT NULL,
    explanation TEXT,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);

-- Choices for multiple choice questions
CREATE TABLE IF NOT EXISTS choices (
    id TEXT PRIMARY KEY,
    question_id TEXT NOT NULL,
    text TEXT NOT NULL,
    rich_text TEXT,
    image_url TEXT,
    FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
);

-- Flashcard spaced repetition data
CREATE TABLE IF NOT EXISTS flashcard_data (
    question_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    ease_factor REAL NOT NULL DEFAULT 2.5,
    interval INTEGER NOT NULL DEFAULT 0,
    repetitions INTEGER NOT NULL DEFAULT 0,
    due_date TEXT NOT NULL,
    last_reviewed TEXT NOT NULL,
    PRIMARY KEY (question_id, user_id),
    FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
);

-- Quiz attempts
CREATE TABLE IF NOT EXISTS quiz_attempts (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    score REAL,
    time_spent INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);

-- Question answers in attempts
CREATE TABLE IF NOT EXISTS question_answers (
    attempt_id TEXT NOT NULL,
    question_id TEXT NOT NULL,
    answer TEXT NOT NULL,
    is_correct INTEGER,
    time_spent INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (attempt_id, question_id),
    FOREIGN KEY (attempt_id) REFERENCES quiz_attempts (id) ON DELETE CASCADE,
    FOREIGN KEY (question_id) REFERENCES questions (id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quizzes_author_id ON quizzes (author_id);
CREATE INDEX IF NOT EXISTS idx_questions_quiz_id ON questions (quiz_id);
CREATE INDEX IF NOT EXISTS idx_choices_question_id ON choices (question_id);
CREATE INDEX IF NOT EXISTS idx_flashcard_data_user_id ON flashcard_data (user_id);
CREATE INDEX IF NOT EXISTS idx_flashcard_data_due_date ON flashcard_data (due_date);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_user_id ON quiz_attempts (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_attempts_quiz_id ON quiz_attempts (quiz_id);

-- Quiz-Course Integration

-- Quiz-Course mappings
CREATE TABLE IF NOT EXISTS quiz_course_mappings (
    id TEXT PRIMARY KEY,
    quiz_id TEXT NOT NULL,
    course_id TEXT NOT NULL,
    module_id TEXT,
    section_id TEXT,
    position INTEGER NOT NULL DEFAULT 0,
    is_required INTEGER NOT NULL DEFAULT 1,
    passing_score REAL,
    due_date TEXT,
    available_from TEXT,
    available_until TEXT,
    max_attempts INTEGER,
    time_limit INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE CASCADE
);

-- Quiz assignments for students
CREATE TABLE IF NOT EXISTS quiz_assignments (
    id TEXT PRIMARY KEY,
    mapping_id TEXT NOT NULL,
    student_id TEXT NOT NULL,
    status TEXT NOT NULL,
    attempts INTEGER NOT NULL DEFAULT 0,
    best_score REAL,
    last_attempt_at TEXT,
    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (mapping_id) REFERENCES quiz_course_mappings (id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_course_mappings_quiz_id ON quiz_course_mappings (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_course_mappings_course_id ON quiz_course_mappings (course_id);
CREATE INDEX IF NOT EXISTS idx_quiz_course_mappings_module_id ON quiz_course_mappings (module_id);
CREATE INDEX IF NOT EXISTS idx_quiz_course_mappings_section_id ON quiz_course_mappings (section_id);
CREATE INDEX IF NOT EXISTS idx_quiz_assignments_mapping_id ON quiz_assignments (mapping_id);
CREATE INDEX IF NOT EXISTS idx_quiz_assignments_student_id ON quiz_assignments (student_id);

-- Quiz Notifications

-- Quiz notifications table
CREATE TABLE IF NOT EXISTS quiz_notifications (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    quiz_id TEXT,
    course_id TEXT,
    mapping_id TEXT,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    link TEXT,
    read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_notifications_user_id ON quiz_notifications (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_notifications_quiz_id ON quiz_notifications (quiz_id);
CREATE INDEX IF NOT EXISTS idx_quiz_notifications_course_id ON quiz_notifications (course_id);
CREATE INDEX IF NOT EXISTS idx_quiz_notifications_mapping_id ON quiz_notifications (mapping_id);
CREATE INDEX IF NOT EXISTS idx_quiz_notifications_read ON quiz_notifications (read);
