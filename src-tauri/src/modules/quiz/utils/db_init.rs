use sqlx::{SqlitePool, Sqlite};
use anyhow::{Result, anyhow};
use std::path::Path;
use tracing::{info, error};
use uuid::Uuid;
use chrono::Utc;

/// Initialize the quiz database
pub async fn init_quiz_db(pool: &SqlitePool) -> Result<()> {
    info!("Initializing quiz database...");

    // Apply migrations
    apply_quiz_migrations(pool).await?;

    // Check if we need to create test data
    let quiz_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM quizzes"
    )
    .fetch_one(pool)
    .await?
    .count;

    if quiz_count == 0 {
        info!("No quizzes found, creating test data...");
        create_test_data(pool).await?;
    }

    info!("Quiz database initialization complete");
    Ok(())
}

/// Apply quiz migrations
async fn apply_quiz_migrations(pool: &SqlitePool) -> Result<()> {
    // Check if quizzes table exists
    let table_exists = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='quizzes'"
    )
    .fetch_optional(pool)
    .await?
    .is_some();

    if table_exists {
        info!("Quiz tables already exist, skipping migration");
        return Ok(());
    }

    info!("Creating quiz tables...");

    // Create quiz tables
    sqlx::query(
        r#"
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
            deleted_at TEXT
        );

        CREATE TABLE IF NOT EXISTS questions (
            id TEXT PRIMARY KEY,
            quiz_id TEXT NOT NULL,
            question_text TEXT NOT NULL,
            content TEXT, -- JSON content with text, rich_text, and media
            question_type TEXT NOT NULL, -- 'multiple_choice', 'true_false', 'short_answer', 'matching', 'essay'
            points INTEGER DEFAULT 1,
            position INTEGER,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (quiz_id) REFERENCES quizzes(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS answer_options (
            id TEXT PRIMARY KEY,
            question_id TEXT NOT NULL,
            option_text TEXT NOT NULL,
            content TEXT, -- JSON content with text, rich_text, and media
            is_correct INTEGER DEFAULT 0,
            position INTEGER,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (question_id) REFERENCES questions(id) ON DELETE CASCADE
        );

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
            FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
        );

        CREATE TABLE IF NOT EXISTS user_answers (
            id TEXT PRIMARY KEY,
            attempt_id TEXT NOT NULL,
            question_id TEXT NOT NULL,
            answer_option_id TEXT,
            text_answer TEXT,
            content TEXT, -- JSON content with text, rich_text, and media
            is_correct INTEGER,
            points_awarded REAL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (attempt_id) REFERENCES quiz_attempts(id) ON DELETE CASCADE,
            FOREIGN KEY (question_id) REFERENCES questions(id),
            FOREIGN KEY (answer_option_id) REFERENCES answer_options(id)
        );

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
            FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
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
        "#
    )
    .execute(pool)
    .await?;

    info!("Quiz tables created successfully");
    Ok(())
}

/// Create test data for the quiz module
pub async fn create_test_data(pool: &SqlitePool) -> Result<()> {
    info!("Creating test data for quiz module...");

    // Create a test user if it doesn't exist
    let user_id = "test-user-1";

    // Create a test quiz
    let quiz_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query!(
        r#"
        INSERT INTO quizzes (
            id, title, description, author_id,
            time_limit, passing_score, shuffle_questions, show_results,
            visibility, tags, study_mode, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        quiz_id,
        "Sample Quiz",
        "A sample quiz for testing the Ordo Quiz module",
        user_id,
        60 * 10, // 10 minutes
        70, // 70% passing score
        false,
        true,
        "private",
        "[]", // Empty tags array
        "multiple_choice",
        now,
        now
    )
    .execute(pool)
    .await?;

    // Create quiz settings
    sqlx::query!(
        r#"
        INSERT INTO quiz_settings (
            quiz_id, allow_retakes, max_attempts,
            show_correct_answers, show_correct_answers_after_completion,
            time_limit, passing_score, shuffle_questions
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        quiz_id,
        true,
        3, // Max 3 attempts
        true,
        true,
        60 * 10, // 10 minutes
        70, // 70% passing score
        false
    )
    .execute(pool)
    .await?;

    // Create some questions
    let question1_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO questions (
            id, quiz_id, question_text, content, question_type, points
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        question1_id,
        quiz_id,
        "What is the capital of France?",
        null::<String>, // No rich content
        "multiple_choice",
        1
    )
    .execute(pool)
    .await?;

    // Create answer options for question 1
    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question1_id,
        "Paris",
        true
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question1_id,
        "London",
        false
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question1_id,
        "Berlin",
        false
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question1_id,
        "Madrid",
        false
    )
    .execute(pool)
    .await?;

    // Create a second question
    let question2_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO questions (
            id, quiz_id, question_text, content, question_type, points
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        question2_id,
        quiz_id,
        "Is the Earth flat?",
        null::<String>, // No rich content
        "true_false",
        1
    )
    .execute(pool)
    .await?;

    // Create answer options for question 2
    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question2_id,
        "True",
        false
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct
        )
        VALUES (?, ?, ?, ?)
        "#,
        Uuid::new_v4().to_string(),
        question2_id,
        "False",
        true
    )
    .execute(pool)
    .await?;

    // Create a third question (short answer)
    let question3_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO questions (
            id, quiz_id, question_text, content, question_type, points
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        question3_id,
        quiz_id,
        "What is the chemical symbol for water?",
        null::<String>, // No rich content
        "short_answer",
        2 // Worth more points
    )
    .execute(pool)
    .await?;

    info!("Test data created successfully");
    Ok(())
}
