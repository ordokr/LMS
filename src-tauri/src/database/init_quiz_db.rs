use sqlx::{SqlitePool, Sqlite};
use anyhow::{Result, anyhow};
use std::path::Path;
use tracing::{info, error};

/// Initialize the quiz database
pub async fn init_quiz_db(pool: &SqlitePool) -> Result<()> {
    info!("Initializing quiz database...");
    
    // Apply migrations
    apply_quiz_migrations(pool).await?;
    
    info!("Quiz database initialization complete");
    Ok(())
}

/// Apply quiz migrations
async fn apply_quiz_migrations(pool: &SqlitePool) -> Result<()> {
    // Check if migrations directory exists
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        error!("Migrations directory not found");
        return Err(anyhow!("Migrations directory not found"));
    }
    
    // Apply quiz schema migration
    let migration_path = migrations_dir.join("20240421_ordo_quiz_schema.sql");
    if migration_path.exists() {
        info!("Applying quiz schema migration: {:?}", migration_path);
        let sql = std::fs::read_to_string(&migration_path)?;
        sqlx::query(&sql).execute(pool).await?;
    } else {
        error!("Quiz schema migration not found: {:?}", migration_path);
        return Err(anyhow!("Quiz schema migration not found"));
    }
    
    Ok(())
}

/// Check if quiz tables exist
pub async fn check_quiz_tables(pool: &SqlitePool) -> Result<bool> {
    // Check if quizzes table exists
    let result = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name='quizzes'"
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(result.is_some())
}

/// Create test data for the quiz module
pub async fn create_test_data(pool: &SqlitePool) -> Result<()> {
    info!("Creating test data for quiz module...");
    
    // Check if test data already exists
    let quiz_count = sqlx::query!(
        "SELECT COUNT(*) as count FROM quizzes"
    )
    .fetch_one(pool)
    .await?
    .count;
    
    if quiz_count > 0 {
        info!("Test data already exists, skipping creation");
        return Ok(());
    }
    
    // Create a test quiz
    let quiz_id = uuid::Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO quizzes (
            id, title, description, author_id, 
            time_limit, passing_score, shuffle_questions, show_results,
            visibility, tags, study_mode
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        quiz_id,
        "Sample Quiz",
        "A sample quiz for testing",
        "test-user-1",
        60 * 10, // 10 minutes
        70, // 70% passing score
        false,
        true,
        "private",
        "[]", // Empty tags array
        "multiple_choice"
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
    let question1_id = uuid::Uuid::new_v4().to_string();
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
        uuid::Uuid::new_v4().to_string(),
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
        uuid::Uuid::new_v4().to_string(),
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
        uuid::Uuid::new_v4().to_string(),
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
        uuid::Uuid::new_v4().to_string(),
        question1_id,
        "Madrid",
        false
    )
    .execute(pool)
    .await?;
    
    // Create a second question
    let question2_id = uuid::Uuid::new_v4().to_string();
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
        uuid::Uuid::new_v4().to_string(),
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
        uuid::Uuid::new_v4().to_string(),
        question2_id,
        "False",
        true
    )
    .execute(pool)
    .await?;
    
    info!("Test data created successfully");
    Ok(())
}
