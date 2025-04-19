-- Insert test user
INSERT INTO users (id, name, email) VALUES ('test-user-1', 'Test User', 'test@example.com');

-- Insert test quiz
INSERT INTO quizzes (
    id, title, description, author_id, 
    time_limit, passing_score, shuffle_questions, show_results,
    visibility, tags, study_mode, created_at, updated_at
)
VALUES (
    'quiz-1', 'Test Quiz', 'A test quiz for the Ordo Quiz module', 'test-user-1',
    600, 70, 0, 1, 'private', '["test", "ordo"]', 'multiple_choice',
    datetime('now'), datetime('now')
);

-- Insert quiz settings
INSERT INTO quiz_settings (
    quiz_id, allow_retakes, max_attempts, 
    show_correct_answers, show_correct_answers_after_completion,
    time_limit, passing_score, shuffle_questions
)
VALUES (
    'quiz-1', 1, 3, 1, 1, 600, 70, 0
);

-- Insert test question
INSERT INTO questions (
    id, quiz_id, question_text, question_type, points, created_at, updated_at
)
VALUES (
    'question-1', 'quiz-1', 'What is the capital of France?', 'multiple_choice', 1,
    datetime('now'), datetime('now')
);

-- Insert answer options
INSERT INTO answer_options (
    id, question_id, option_text, is_correct, created_at, updated_at
)
VALUES 
    ('option-1', 'question-1', 'Paris', 1, datetime('now'), datetime('now')),
    ('option-2', 'question-1', 'London', 0, datetime('now'), datetime('now')),
    ('option-3', 'question-1', 'Berlin', 0, datetime('now'), datetime('now')),
    ('option-4', 'question-1', 'Madrid', 0, datetime('now'), datetime('now'));

-- Insert quiz attempt
INSERT INTO quiz_attempts (
    id, quiz_id, user_id, status, start_time, created_at, updated_at
)
VALUES (
    'attempt-1', 'quiz-1', 'test-user-1', 'in_progress', datetime('now'),
    datetime('now'), datetime('now')
);

-- Insert activity
INSERT INTO quiz_activities (
    id, user_id, quiz_id, activity_type, timestamp, created_at
)
VALUES (
    'activity-1', 'test-user-1', 'quiz-1', 'quiz_started', datetime('now'), datetime('now')
);

-- Complete the quiz attempt
UPDATE quiz_attempts
SET status = 'completed', end_time = datetime('now'), score = 90.0, updated_at = datetime('now')
WHERE id = 'attempt-1';

-- Insert completion activity
INSERT INTO quiz_activities (
    id, user_id, quiz_id, activity_type, data, duration_ms, timestamp, created_at
)
VALUES (
    'activity-2', 'test-user-1', 'quiz-1', 'quiz_completed', '{"score": 90.0}', 15000,
    datetime('now'), datetime('now')
);
