-- Quiz AI Generation Schema

-- Quiz AI generation requests table
CREATE TABLE IF NOT EXISTS quiz_ai_generation_requests (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    source_type TEXT NOT NULL,
    source_content TEXT NOT NULL,
    model_type TEXT NOT NULL,
    model_parameters TEXT,
    num_questions INTEGER NOT NULL,
    question_types TEXT NOT NULL,
    difficulty_level INTEGER NOT NULL,
    topic_focus TEXT,
    language TEXT NOT NULL,
    study_mode TEXT NOT NULL,
    visibility TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT
);

-- Quiz AI generation results table
CREATE TABLE IF NOT EXISTS quiz_ai_generation_results (
    id TEXT PRIMARY KEY,
    request_id TEXT NOT NULL,
    quiz_id TEXT,
    raw_response TEXT NOT NULL,
    error_message TEXT,
    processing_time_ms INTEGER NOT NULL,
    token_usage INTEGER,
    created_at TEXT NOT NULL,
    FOREIGN KEY (request_id) REFERENCES quiz_ai_generation_requests (id) ON DELETE CASCADE,
    FOREIGN KEY (quiz_id) REFERENCES quizzes (id) ON DELETE SET NULL
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_ai_generation_requests_user_id ON quiz_ai_generation_requests (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_ai_generation_requests_status ON quiz_ai_generation_requests (status);
CREATE INDEX IF NOT EXISTS idx_quiz_ai_generation_results_request_id ON quiz_ai_generation_results (request_id);
CREATE INDEX IF NOT EXISTS idx_quiz_ai_generation_results_quiz_id ON quiz_ai_generation_results (quiz_id);
