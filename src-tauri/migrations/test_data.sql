-- Test data for Ordo Quiz Module
-- This provides minimal test data for development and testing

-- Create test users if they don't exist
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    avatar_url TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- Insert test users
INSERT OR IGNORE INTO users (id, username, email, password_hash, display_name)
VALUES 
    ('user-1', 'instructor', 'instructor@example.com', '$2a$12$K8GpVzT6.ZSIBsTT.3IyM.vFkxU9KgJH8JuGpBNK9Jjyh3Lvp02hy', 'Test Instructor'),
    ('user-2', 'student', 'student@example.com', '$2a$12$K8GpVzT6.ZSIBsTT.3IyM.vFkxU9KgJH8JuGpBNK9Jjyh3Lvp02hy', 'Test Student');

-- Create test courses if they don't exist
CREATE TABLE IF NOT EXISTS courses (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    code TEXT,
    description TEXT,
    instructor_id TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (instructor_id) REFERENCES users(id)
);

-- Insert test course
INSERT OR IGNORE INTO courses (id, title, code, description, instructor_id)
VALUES ('course-1', 'Introduction to Ordo Quiz', 'QUIZ101', 'Learn how to use the Ordo Quiz module', 'user-1');

-- Insert test quiz
INSERT OR IGNORE INTO quizzes (id, title, description, course_id, author_id, visibility, study_mode)
VALUES (
    'quiz-1', 
    'Sample Quiz', 
    'This is a sample quiz to demonstrate the Ordo Quiz module', 
    'course-1', 
    'user-1', 
    'public',
    'multiple_choice'
);

-- Insert quiz settings
INSERT OR IGNORE INTO quiz_settings (quiz_id, allow_retakes, max_attempts, show_correct_answers)
VALUES ('quiz-1', 1, 3, 1);

-- Insert test questions
INSERT OR IGNORE INTO questions (id, quiz_id, content, question_type, points, position)
VALUES 
    ('question-1', 'quiz-1', '{"text": "What is the primary purpose of the Ordo Quiz module?"}', 'multiple_choice', 1, 1),
    ('question-2', 'quiz-1', '{"text": "Which of the following is a feature of the Ordo Quiz module?"}', 'multiple_choice', 1, 2);

-- Insert answer options for question 1
INSERT OR IGNORE INTO answer_options (id, question_id, content, is_correct, position)
VALUES 
    ('option-1-1', 'question-1', '{"text": "To provide a forum for discussions"}', 0, 1),
    ('option-1-2', 'question-1', '{"text": "To create and take quizzes and assessments"}', 1, 2),
    ('option-1-3', 'question-1', '{"text": "To manage course enrollments"}', 0, 3),
    ('option-1-4', 'question-1', '{"text": "To grade assignments"}', 0, 4);

-- Insert answer options for question 2
INSERT OR IGNORE INTO answer_options (id, question_id, content, is_correct, position)
VALUES 
    ('option-2-1', 'question-2', '{"text": "Offline-first functionality"}', 1, 1),
    ('option-2-2', 'question-2', '{"text": "Virtual reality support"}', 0, 2),
    ('option-2-3', 'question-2', '{"text": "Blockchain-based grading"}', 0, 3),
    ('option-2-4', 'question-2', '{"text": "AI-powered question generation"}', 0, 4);

-- Create sync tables for offline functionality
CREATE TABLE IF NOT EXISTS quiz_sync_items (
    id TEXT PRIMARY KEY,
    entity_type TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    operation TEXT NOT NULL, -- 'create', 'update', 'delete'
    data TEXT NOT NULL, -- JSON data
    priority TEXT NOT NULL, -- 'low', 'medium', 'high', 'critical'
    status TEXT NOT NULL, -- 'pending', 'in_progress', 'completed', 'failed'
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    synced_at TEXT,
    error TEXT,
    retry_count INTEGER DEFAULT 0
);

-- Create index for sync items
CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_status ON quiz_sync_items(status);
CREATE INDEX IF NOT EXISTS idx_quiz_sync_items_entity ON quiz_sync_items(entity_type, entity_id);
