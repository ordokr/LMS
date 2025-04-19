import os
import sqlite3
import json
import datetime

def main():
    print("Starting Ordo Quiz Module Test...")

    # Create test directories
    data_dir = "ordo_quiz_test_data"
    if not os.path.exists(data_dir):
        os.makedirs(data_dir)
    
    migrations_dir = "migrations"
    if not os.path.exists(migrations_dir):
        os.makedirs(migrations_dir)
    
    # Create migration file
    migration_path = os.path.join(migrations_dir, "20240421_ordo_quiz_schema.sql")
    migration_sql = """
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
"""
    
    with open(migration_path, 'w') as f:
        f.write(migration_sql)
    
    print(f"Created migration file: {migration_path}")
    
    # Create test database using SQLite
    db_path = "ordo_quiz_test.db"
    if os.path.exists(db_path):
        os.remove(db_path)
        print(f"Removed existing database: {db_path}")
    
    # Connect to the database
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    # Apply migration
    cursor.executescript(migration_sql)
    conn.commit()
    
    print(f"Applied migration to database: {db_path}")
    
    # Insert test data
    now = datetime.datetime.now().isoformat()
    
    # Insert test user
    cursor.execute(
        "INSERT INTO users (id, name, email) VALUES (?, ?, ?)",
        ('test-user-1', 'Test User', 'test@example.com')
    )
    
    # Insert test quiz
    cursor.execute(
        """
        INSERT INTO quizzes (
            id, title, description, author_id, 
            time_limit, passing_score, shuffle_questions, show_results,
            visibility, tags, study_mode, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'quiz-1', 'Test Quiz', 'A test quiz for the Ordo Quiz module', 'test-user-1',
            600, 70, 0, 1, 'private', json.dumps(['test', 'ordo']), 'multiple_choice',
            now, now
        )
    )
    
    # Insert quiz settings
    cursor.execute(
        """
        INSERT INTO quiz_settings (
            quiz_id, allow_retakes, max_attempts, 
            show_correct_answers, show_correct_answers_after_completion,
            time_limit, passing_score, shuffle_questions
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'quiz-1', 1, 3, 1, 1, 600, 70, 0
        )
    )
    
    # Insert test question
    cursor.execute(
        """
        INSERT INTO questions (
            id, quiz_id, question_text, question_type, points, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'question-1', 'quiz-1', 'What is the capital of France?', 'multiple_choice', 1,
            now, now
        )
    )
    
    # Insert answer options
    cursor.executemany(
        """
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        """,
        [
            ('option-1', 'question-1', 'Paris', 1, now, now),
            ('option-2', 'question-1', 'London', 0, now, now),
            ('option-3', 'question-1', 'Berlin', 0, now, now),
            ('option-4', 'question-1', 'Madrid', 0, now, now)
        ]
    )
    
    # Insert quiz attempt
    cursor.execute(
        """
        INSERT INTO quiz_attempts (
            id, quiz_id, user_id, status, start_time, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'attempt-1', 'quiz-1', 'test-user-1', 'in_progress', now, now, now
        )
    )
    
    # Insert activity
    cursor.execute(
        """
        INSERT INTO quiz_activities (
            id, user_id, quiz_id, activity_type, timestamp, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        """,
        (
            'activity-1', 'test-user-1', 'quiz-1', 'quiz_started', now, now
        )
    )
    
    # Complete the quiz attempt
    cursor.execute(
        """
        UPDATE quiz_attempts
        SET status = ?, end_time = ?, score = ?, updated_at = ?
        WHERE id = ?
        """,
        (
            'completed', now, 90.0, now, 'attempt-1'
        )
    )
    
    # Insert completion activity
    cursor.execute(
        """
        INSERT INTO quiz_activities (
            id, user_id, quiz_id, activity_type, data, duration_ms, timestamp, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'activity-2', 'test-user-1', 'quiz-1', 'quiz_completed', json.dumps({'score': 90.0}), 15000,
            now, now
        )
    )
    
    conn.commit()
    print(f"Inserted test data into database: {db_path}")
    
    # Query and display test data
    print("\nVerifying test data:")
    
    # Check quizzes
    print("\nQuizzes:")
    cursor.execute("SELECT id, title FROM quizzes")
    for row in cursor.fetchall():
        print(row)
    
    # Check questions
    print("\nQuestions:")
    cursor.execute("SELECT id, question_text FROM questions")
    for row in cursor.fetchall():
        print(row)
    
    # Check quiz attempts
    print("\nQuiz Attempts:")
    cursor.execute("SELECT id, quiz_id, user_id, status, score FROM quiz_attempts")
    for row in cursor.fetchall():
        print(row)
    
    # Check activities
    print("\nActivities:")
    cursor.execute("SELECT id, user_id, quiz_id, activity_type FROM quiz_activities")
    for row in cursor.fetchall():
        print(row)
    
    # Check sync functionality
    print("\nTesting sync functionality:")
    
    # Create a sync item
    cursor.execute(
        """
        INSERT INTO quiz_sync_items (
            id, entity_type, entity_id, operation, data, priority, status, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
        (
            'sync-1', 'quiz', 'quiz-1', 'update', json.dumps({'title': 'Updated Quiz Title'}),
            'high', 'pending', now, now
        )
    )
    
    conn.commit()
    
    # Check sync items
    print("\nSync Items:")
    cursor.execute("SELECT id, entity_type, entity_id, operation, status FROM quiz_sync_items")
    for row in cursor.fetchall():
        print(row)
    
    # Close the connection
    conn.close()
    
    print("\nOrdo Quiz Module Test completed successfully!")

if __name__ == "__main__":
    main()
