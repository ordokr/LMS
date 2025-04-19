use std::sync::Arc;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::fs;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Ordo Quiz Module Test...");

    // Initialize database connection
    let db_url = "sqlite:ordo_quiz_test.db?mode=rwc";
    
    println!("Connecting to database: {}", db_url);
    let db_pool = SqlitePool::connect(db_url).await?;

    // Create test data directory
    let data_dir = PathBuf::from("ordo_quiz_test_data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    // Initialize quiz database
    println!("Initializing quiz database...");
    
    // Apply migrations
    let migrations_dir = PathBuf::from("migrations");
    if migrations_dir.exists() {
        let migration_path = migrations_dir.join("20240421_ordo_quiz_schema.sql");
        if migration_path.exists() {
            println!("Applying quiz schema migration: {:?}", migration_path);
            let sql = std::fs::read_to_string(&migration_path)?;
            sqlx::query(&sql).execute(&db_pool).await?;
        } else {
            println!("Quiz schema migration not found: {:?}", migration_path);
        }
    } else {
        println!("Migrations directory not found");
    }
    
    // Create a test quiz
    println!("Creating test quiz...");
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
        "Test Quiz",
        "A test quiz for the Ordo Quiz module",
        "test-user-1",
        600, // 10 minutes
        70, // 70% passing score
        false,
        true,
        "private",
        "[]", // Empty tags array
        "multiple_choice",
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    println!("Created quiz with ID: {}", quiz_id);
    
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
        600, // 10 minutes
        70, // 70% passing score
        false
    )
    .execute(&db_pool)
    .await?;
    
    // Create a test question
    println!("Creating test question...");
    let question_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO questions (
            id, quiz_id, question_text, question_type, points, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        question_id,
        quiz_id,
        "What is the capital of France?",
        "multiple_choice",
        1,
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    println!("Created question with ID: {}", question_id);
    
    // Create answer options
    println!("Creating answer options...");
    
    // Correct answer
    let option1_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        option1_id,
        question_id,
        "Paris",
        true,
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    // Incorrect answers
    let option2_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        option2_id,
        question_id,
        "London",
        false,
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    let option3_id = Uuid::new_v4().to_string();
    sqlx::query!(
        r#"
        INSERT INTO answer_options (
            id, question_id, option_text, is_correct, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        option3_id,
        question_id,
        "Berlin",
        false,
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    // Create a quiz attempt
    println!("Creating quiz attempt...");
    let attempt_id = Uuid::new_v4().to_string();
    let user_id = "test-user-1";
    
    sqlx::query!(
        r#"
        INSERT INTO quiz_attempts (
            id, quiz_id, user_id, status, start_time, created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        attempt_id,
        quiz_id,
        user_id,
        "in_progress",
        now,
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    println!("Created quiz attempt with ID: {}", attempt_id);
    
    // Track activity
    println!("Tracking activity...");
    let activity_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO quiz_activities (
            id, user_id, quiz_id, activity_type, timestamp, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
        activity_id,
        user_id,
        quiz_id,
        "quiz_started",
        now,
        now
    )
    .execute(&db_pool)
    .await?;
    
    println!("Tracked quiz started activity with ID: {}", activity_id);
    
    // Complete the quiz attempt
    println!("Completing quiz attempt...");
    let end_time = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE quiz_attempts
        SET status = ?, end_time = ?, score = ?, updated_at = ?
        WHERE id = ?
        "#,
        "completed",
        end_time,
        90.0, // 90% score
        end_time,
        attempt_id
    )
    .execute(&db_pool)
    .await?;
    
    println!("Completed quiz attempt with score: 90.0");
    
    // Track completion activity
    let completion_activity_id = Uuid::new_v4().to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO quiz_activities (
            id, user_id, quiz_id, activity_type, data, duration_ms, timestamp, created_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        completion_activity_id,
        user_id,
        quiz_id,
        "quiz_completed",
        "{\"score\": 90.0}",
        15000, // 15 seconds
        end_time,
        end_time
    )
    .execute(&db_pool)
    .await?;
    
    println!("Tracked quiz completed activity with ID: {}", completion_activity_id);
    
    // Get activity summary
    println!("Getting activity summary...");
    
    let activities = sqlx::query!(
        r#"
        SELECT activity_type, COUNT(*) as count
        FROM quiz_activities
        WHERE user_id = ?
        GROUP BY activity_type
        "#,
        user_id
    )
    .fetch_all(&db_pool)
    .await?;
    
    println!("Activity summary:");
    for activity in activities {
        println!("  {}: {}", activity.activity_type, activity.count);
    }
    
    println!("Ordo Quiz Module Test completed successfully!");
    Ok(())
}
