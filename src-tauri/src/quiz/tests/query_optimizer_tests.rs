use crate::quiz::query_optimizer::{QuizQueryOptimizer, QuizFilters};
use crate::quiz::models::{QuizVisibility, StudyMode};
use sqlx::{SqlitePool, Pool, Sqlite, sqlite::SqlitePoolOptions};
use uuid::Uuid;
use std::time::{Duration, Instant};
use tokio;

async fn setup_test_db() -> Pool<Sqlite> {
    // Create an in-memory SQLite database for testing
    let pool = SqlitePool::connect("sqlite::memory:").await.expect("Failed to create in-memory database");

    // Create the necessary tables
    sqlx::query(
        r#"
        CREATE TABLE quizzes (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            visibility TEXT NOT NULL,
            study_mode TEXT NOT NULL,
            author_id TEXT,
            created_at TIMESTAMP NOT NULL,
            updated_at TIMESTAMP NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create quizzes table");

    sqlx::query(
        r#"
        CREATE TABLE questions (
            id TEXT PRIMARY KEY,
            quiz_id TEXT NOT NULL,
            content TEXT NOT NULL,
            answer_type TEXT NOT NULL,
            explanation TEXT,
            position INTEGER,
            FOREIGN KEY (quiz_id) REFERENCES quizzes(id)
        )
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create questions table");

    // Insert test data
    for i in 0..50 {
        let quiz_id = Uuid::new_v4().to_string();
        let visibility = if i % 3 == 0 { "public" } else if i % 3 == 1 { "private" } else { "unlisted" };
        let study_mode = if i % 2 == 0 { "multiple_choice" } else { "flashcards" };
        let author_id = if i % 5 == 0 { None } else { Some(Uuid::new_v4().to_string()) };

        sqlx::query(
            r#"
            INSERT INTO quizzes (id, title, description, visibility, study_mode, author_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))
            "#
        )
        .bind(&quiz_id)
        .bind(format!("Test Quiz {}", i))
        .bind(format!("Description for quiz {}", i))
        .bind(visibility)
        .bind(study_mode)
        .bind(author_id)
        .execute(&pool)
        .await
        .expect("Failed to insert quiz");

        // Insert questions for this quiz
        for j in 0..5 {
            let question_id = Uuid::new_v4().to_string();

            sqlx::query(
                r#"
                INSERT INTO questions (id, quiz_id, content, answer_type, explanation, position)
                VALUES (?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&question_id)
            .bind(&quiz_id)
            .bind(format!("Question {} for quiz {}", j, i))
            .bind("multiple_choice")
            .bind(format!("Explanation for question {}", j))
            .bind(j)
            .execute(&pool)
            .await
            .expect("Failed to insert question");
        }
    }

    pool
}

#[tokio::test]
async fn test_quiz_query_optimization() {
    // Setup test database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
        .unwrap();

    // Initialize optimizer
    let optimizer = QuizQueryOptimizer::new(pool);

    // Test quiz fetching
    let filters = QuizFilters::new()
        .with_visibility(QuizVisibility::Public)
        .with_limit(10);

    let quizzes = optimizer.fetch_quizzes(&filters).await.unwrap();
    assert!(quizzes.is_empty()); // Empty test DB

    // Test cache hit
    let cached_quizzes = optimizer.fetch_quizzes(&filters).await.unwrap();
    assert_eq!(quizzes.len(), cached_quizzes.len());

    // Test batch loading
    let quiz_ids = vec![Uuid::new_v4(), Uuid::new_v4()];
    let questions_map = optimizer.batch_load_questions(&quiz_ids).await.unwrap();
    assert!(questions_map.is_empty()); // Empty test DB
}

#[tokio::test]
async fn test_cache_expiration() {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(":memory:")
        .await
        .unwrap();

    // Create optimizer with very short TTL for testing
    let optimizer = QuizQueryOptimizer::new(pool)
        .with_cache_config(Duration::from_millis(100), 1000);

    // Add test data to cache
    let filters = QuizFilters::new()
        .with_visibility(QuizVisibility::Public)
        .with_limit(10);

    optimizer.fetch_quizzes(&filters).await.unwrap();

    // Wait for cache to expire
    tokio::time::sleep(Duration::from_millis(150)).await;

    // Clear expired entries
    optimizer.clear_expired_cache().await;

    // Check cache stats
    let (hits, misses, hit_rate) = optimizer.get_cache_stats();
    assert_eq!(hits, 0);
    assert_eq!(misses, 1);
    assert_eq!(hit_rate, 0.0);
}

#[tokio::test]
async fn test_query_optimizer_basic_filtering() {
    let pool = setup_test_db().await;
    let optimizer = QuizQueryOptimizer::new(pool.clone());

    // Test filtering by visibility
    let filters = QuizFilters::new()
        .with_visibility(QuizVisibility::Public)
        .with_limit(10);

    let quizzes = optimizer.fetch_quizzes(&filters).await.expect("Failed to fetch quizzes");

    // Should have found some public quizzes
    assert!(!quizzes.is_empty());
    assert!(quizzes.len() <= 10);

    // All quizzes should be public
    for quiz in &quizzes {
        assert_eq!(quiz.visibility, QuizVisibility::Public);
    }

    // Test filtering by study mode
    let filters = QuizFilters::new()
        .with_study_mode(StudyMode::Flashcards)
        .with_limit(10);

    let quizzes = optimizer.fetch_quizzes(&filters).await.expect("Failed to fetch quizzes");

    // Should have found some flashcard quizzes
    assert!(!quizzes.is_empty());
    assert!(quizzes.len() <= 10);

    // All quizzes should be flashcards
    for quiz in &quizzes {
        assert_eq!(quiz.study_mode, StudyMode::Flashcards);
    }
}

#[tokio::test]
async fn test_query_optimizer_combined_filters() {
    let pool = setup_test_db().await;
    let optimizer = QuizQueryOptimizer::new(pool.clone());

    // Test combined filters
    let filters = QuizFilters::new()
        .with_visibility(QuizVisibility::Public)
        .with_study_mode(StudyMode::MultipleChoice)
        .with_limit(10);

    let quizzes = optimizer.fetch_quizzes(&filters).await.expect("Failed to fetch quizzes");

    // All quizzes should match both filters
    for quiz in &quizzes {
        assert_eq!(quiz.visibility, QuizVisibility::Public);
        assert_eq!(quiz.study_mode, StudyMode::MultipleChoice);
    }
}

#[tokio::test]
async fn test_query_optimizer_caching() {
    let pool = setup_test_db().await;
    let optimizer = QuizQueryOptimizer::new(pool.clone());

    // Define a filter
    let filters = QuizFilters::new()
        .with_visibility(QuizVisibility::Public)
        .with_limit(10);

    // First query - should be a cache miss
    let start = Instant::now();
    let _ = optimizer.fetch_quizzes(&filters).await.expect("Failed to fetch quizzes");
    let first_duration = start.elapsed();

    // Second query with same filter - should be a cache hit
    let start = Instant::now();
    let _ = optimizer.fetch_quizzes(&filters).await.expect("Failed to fetch quizzes");
    let second_duration = start.elapsed();

    // Cache hit should be faster
    assert!(second_duration <= first_duration);

    // Check cache stats
    let (hits, misses, hit_rate) = optimizer.get_cache_stats();
    assert!(hits > 0);
    assert!(misses > 0);
    assert!(hit_rate > 0.0);
}