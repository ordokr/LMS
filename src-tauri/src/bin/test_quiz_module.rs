use std::sync::Arc;
use sqlx::SqlitePool;
use anyhow::Result;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use std::path::{Path, PathBuf};
use std::fs;
use uuid::Uuid;

use lms_lib::app_state::AppState;
use lms_lib::modules::quiz::services::QuizService;
use lms_lib::models::quiz::{
    CreateQuizRequest, UpdateQuizRequest, 
    CreateQuestionRequest, UpdateQuestionRequest,
    CreateAnswerOptionRequest, QuestionType,
    StartAttemptRequest, CompleteAttemptRequest,
    ActivityType
};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting Ordo Quiz Module Test...");

    // Initialize database connection
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:ordo_quiz_test.db?mode=rwc".to_string());
    
    info!("Connecting to database: {}", db_url);
    let db_pool = SqlitePool::connect(&db_url).await?;

    // Initialize quiz database
    info!("Initializing quiz database...");
    match lms_lib::database::init_quiz_db::init_quiz_db(&db_pool).await {
        Ok(_) => info!("Quiz database initialized successfully"),
        Err(e) => error!("Failed to initialize quiz database: {}", e),
    }

    // Create data directory
    let data_dir = PathBuf::from("ordo_quiz_test_data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }
    
    // Create app state
    let app_state = AppState::new(db_pool.clone(), "ordo_quiz_secret_key".as_bytes().to_vec(), data_dir.clone())
        .with_quiz_repository();
    
    // Initialize quiz service
    let app_state = match app_state.with_quiz_service().await {
        Ok(state) => Arc::new(state),
        Err(e) => {
            error!("Failed to initialize quiz service: {}", e);
            return Err(e);
        }
    };

    // Get the quiz service
    let quiz_service = app_state.get_quiz_service()?;
    
    // Test creating a quiz
    info!("Testing quiz creation...");
    let test_user_id = "test-user-1";
    let quiz_request = CreateQuizRequest {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz for the Ordo Quiz module".to_string()),
        course_id: None,
        time_limit: Some(600), // 10 minutes
        passing_score: Some(70),
        shuffle_questions: Some(false),
        show_results: Some(true),
        visibility: None,
        tags: Some(vec!["test".to_string(), "ordo".to_string()]),
        study_mode: None,
    };
    
    let quiz_id = quiz_service.get_repository().create_quiz(test_user_id, quiz_request).await?;
    info!("Created quiz with ID: {}", quiz_id);
    
    // Test creating questions
    info!("Testing question creation...");
    let question1_request = CreateQuestionRequest {
        quiz_id: quiz_id.clone(),
        question_text: "What is the capital of France?".to_string(),
        content: None,
        question_type: QuestionType::MultipleChoice,
        points: Some(1),
        position: Some(1),
        answer_options: Some(vec![
            CreateAnswerOptionRequest {
                option_text: "Paris".to_string(),
                content: None,
                is_correct: true,
                position: Some(1),
            },
            CreateAnswerOptionRequest {
                option_text: "London".to_string(),
                content: None,
                is_correct: false,
                position: Some(2),
            },
            CreateAnswerOptionRequest {
                option_text: "Berlin".to_string(),
                content: None,
                is_correct: false,
                position: Some(3),
            },
            CreateAnswerOptionRequest {
                option_text: "Madrid".to_string(),
                content: None,
                is_correct: false,
                position: Some(4),
            },
        ]),
    };
    
    let question1_id = quiz_service.get_repository().create_question(question1_request).await?;
    info!("Created question 1 with ID: {}", question1_id);
    
    let question2_request = CreateQuestionRequest {
        quiz_id: quiz_id.clone(),
        question_text: "Is the Earth flat?".to_string(),
        content: None,
        question_type: QuestionType::TrueFalse,
        points: Some(1),
        position: Some(2),
        answer_options: Some(vec![
            CreateAnswerOptionRequest {
                option_text: "True".to_string(),
                content: None,
                is_correct: false,
                position: Some(1),
            },
            CreateAnswerOptionRequest {
                option_text: "False".to_string(),
                content: None,
                is_correct: true,
                position: Some(2),
            },
        ]),
    };
    
    let question2_id = quiz_service.get_repository().create_question(question2_request).await?;
    info!("Created question 2 with ID: {}", question2_id);
    
    // Test starting a quiz attempt
    info!("Testing quiz attempt...");
    let start_attempt_request = StartAttemptRequest {
        quiz_id: quiz_id.clone(),
    };
    
    let attempt = quiz_service.get_repository().start_quiz_attempt(test_user_id, start_attempt_request).await?;
    info!("Started quiz attempt with ID: {}", attempt.id);
    
    // Test tracking activity
    info!("Testing activity tracking...");
    let activity_id = quiz_service.track_quiz_started(test_user_id, &quiz_id).await?;
    info!("Tracked quiz started activity with ID: {}", activity_id);
    
    // Test question answered activity
    let question_activity_id = quiz_service.track_question_answered(
        test_user_id, 
        &quiz_id, 
        &question1_id, 
        true, 
        5000 // 5 seconds
    ).await?;
    info!("Tracked question answered activity with ID: {}", question_activity_id);
    
    // Test completing the quiz attempt
    let complete_attempt_request = CompleteAttemptRequest {
        attempt_id: attempt.id.clone(),
        score: Some(100.0),
    };
    
    let completed_attempt = quiz_service.get_repository().complete_quiz_attempt(complete_attempt_request).await?;
    info!("Completed quiz attempt with score: {}", completed_attempt.score.unwrap_or(0.0));
    
    // Test quiz completed activity
    let completion_activity_id = quiz_service.track_quiz_completed(
        test_user_id, 
        &quiz_id, 
        100.0, 
        15000 // 15 seconds
    ).await?;
    info!("Tracked quiz completed activity with ID: {}", completion_activity_id);
    
    // Test getting activity summary
    info!("Testing activity summary...");
    let user_summary = quiz_service.get_user_activity_summary(test_user_id).await?;
    info!("User activity summary: {} total activities", user_summary.total_activities);
    
    let quiz_summary = quiz_service.get_quiz_activity_summary(&quiz_id).await?;
    info!("Quiz activity summary: {} total activities", quiz_summary.total_activities);
    
    // Test analytics
    info!("Testing analytics...");
    let activity_stats = quiz_service.get_activity_stats(Some(test_user_id)).await?;
    info!("Activity stats: {} quizzes started, {} quizzes completed", 
        activity_stats.total_quizzes_started, 
        activity_stats.total_quizzes_completed
    );
    
    // Test launching the quiz module
    info!("Testing quiz module launch...");
    let session_id = quiz_service.launch_quiz_module(&quiz_id, test_user_id).await?;
    info!("Launched quiz module with session ID: {}", session_id);
    
    // Test sync functionality
    info!("Testing sync functionality...");
    let sync_service = quiz_service.get_sync_service()?;
    let sync_dir = data_dir.join("sync");
    if !sync_dir.exists() {
        fs::create_dir_all(&sync_dir)?;
    }
    
    let export_path = sync_dir.join("test_export.json");
    sync_service.export_sync_data(&export_path).await?;
    info!("Exported sync data to {}", export_path.display());
    
    let processed = sync_service.process_sync_items().await?;
    info!("Processed {} sync items", processed);
    
    info!("Ordo Quiz Module Test completed successfully!");
    Ok(())
}
