-- Quiz Adaptive Learning Schema

-- Quiz adaptive learning paths table
CREATE TABLE IF NOT EXISTS quiz_adaptive_learning_paths (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    author_id TEXT,
    subject TEXT NOT NULL,
    tags TEXT,
    default_study_mode TEXT NOT NULL,
    default_visibility TEXT NOT NULL,
    is_public INTEGER NOT NULL DEFAULT 0,
    usage_count INTEGER NOT NULL DEFAULT 0,
    rating REAL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    nodes TEXT,
    edges TEXT
);

-- Quiz user learning path progress table
CREATE TABLE IF NOT EXISTS quiz_user_learning_path_progress (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    path_id TEXT NOT NULL,
    current_node_id TEXT NOT NULL,
    completed_nodes TEXT NOT NULL,
    scores TEXT NOT NULL,
    started_at TEXT NOT NULL,
    last_activity_at TEXT NOT NULL,
    completed_at TEXT,
    custom_data TEXT,
    FOREIGN KEY (path_id) REFERENCES quiz_adaptive_learning_paths (id) ON DELETE CASCADE
);

-- Quiz learning path recommendations table
CREATE TABLE IF NOT EXISTS quiz_learning_path_recommendations (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    path_id TEXT NOT NULL,
    score REAL NOT NULL,
    reason TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (path_id) REFERENCES quiz_adaptive_learning_paths (id) ON DELETE CASCADE
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_quiz_adaptive_learning_paths_author_id ON quiz_adaptive_learning_paths (author_id);
CREATE INDEX IF NOT EXISTS idx_quiz_adaptive_learning_paths_is_public ON quiz_adaptive_learning_paths (is_public);
CREATE INDEX IF NOT EXISTS idx_quiz_user_learning_path_progress_user_id ON quiz_user_learning_path_progress (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_user_learning_path_progress_path_id ON quiz_user_learning_path_progress (path_id);
CREATE INDEX IF NOT EXISTS idx_quiz_learning_path_recommendations_user_id ON quiz_learning_path_recommendations (user_id);
CREATE INDEX IF NOT EXISTS idx_quiz_learning_path_recommendations_path_id ON quiz_learning_path_recommendations (path_id);
