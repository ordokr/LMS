#[cfg(test)]
mod tests {
    use crate::core::config::Config;
    use crate::quiz::models::{Quiz, Question, Answer, QuestionContent, AnswerType, StudyMode, QuizVisibility};
    use crate::quiz::QuizEngine;
    use crate::quiz::session::QuizSession;
    use crate::quiz::storage::HybridQuizStore;
    use std::sync::Arc;
    use tokio;
    use uuid::Uuid;
    use std::path::PathBuf;
    use tempfile::TempDir;

    async fn setup_test_environment() -> (QuizEngine, TempDir) {
        // Create a temporary directory for the test
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        
        // Create a test configuration
        let config = Config {
            data_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        
        // Initialize the quiz engine
        let engine = QuizEngine::new(&config).expect("Failed to create quiz engine");
        
        (engine, temp_dir)
    }
    
    #[tokio::test]
    async fn test_quiz_creation_and_retrieval() {
        let (engine, _temp_dir) = setup_test_environment().await;
        
        // Create a quiz
        let quiz_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        let quiz = Quiz {
            id: quiz_id,
            title: "Integration Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            questions: Vec::new(),
            settings: Default::default(),
            author_id: Some(author_id),
            visibility: QuizVisibility::Public,
            tags: vec!["integration".to_string(), "test".to_string()],
            study_mode: StudyMode::MultipleChoice,
        };
        
        // Store the quiz
        let result = engine.create_quiz(quiz.clone()).await;
        assert!(result.is_ok());
        
        // Retrieve the quiz using optimized query
        let filters = crate::quiz::query_optimizer::QuizFilters::new()
            .with_visibility(QuizVisibility::Public);
        
        let quizzes = engine.get_quizzes_optimized(filters).await.expect("Failed to retrieve quizzes");
        
        // Verify the quiz was retrieved
        assert!(!quizzes.is_empty());
        let retrieved_quiz = quizzes.iter().find(|q| q.id == quiz_id);
        assert!(retrieved_quiz.is_some());
        
        let retrieved_quiz = retrieved_quiz.unwrap();
        assert_eq!(retrieved_quiz.title, "Integration Test Quiz");
        assert_eq!(retrieved_quiz.visibility, QuizVisibility::Public);
    }
    
    #[tokio::test]
    async fn test_quiz_session_flow() {
        let (engine, _temp_dir) = setup_test_environment().await;
        
        // Create a quiz with questions
        let quiz_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Create a question
        let question_id = Uuid::new_v4();
        let choice1_id = Uuid::new_v4();
        let choice2_id = Uuid::new_v4();
        let choice3_id = Uuid::new_v4();
        
        let choices = vec![
            crate::quiz::models::Choice {
                id: choice1_id,
                text: "Paris".to_string(),
                rich_text: None,
                image_url: None,
            },
            crate::quiz::models::Choice {
                id: choice2_id,
                text: "London".to_string(),
                rich_text: None,
                image_url: None,
            },
            crate::quiz::models::Choice {
                id: choice3_id,
                text: "Berlin".to_string(),
                rich_text: None,
                image_url: None,
            },
        ];
        
        let question = Question {
            id: question_id,
            quiz_id,
            content: QuestionContent {
                text: "What is the capital of France?".to_string(),
                rich_text: None,
                image_url: None,
                audio_url: None,
            },
            answer_type: AnswerType::MultipleChoice,
            choices: choices.clone(),
            correct_answer: Answer::SingleChoice(choice1_id),
            explanation: Some("Paris is the capital of France.".to_string()),
        };
        
        let quiz = Quiz {
            id: quiz_id,
            title: "Session Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            questions: vec![question],
            settings: Default::default(),
            author_id: None,
            visibility: QuizVisibility::Public,
            tags: vec!["session".to_string(), "test".to_string()],
            study_mode: StudyMode::MultipleChoice,
        };
        
        // Store the quiz
        let result = engine.create_quiz(quiz).await;
        assert!(result.is_ok());
        
        // Start a session
        let session_result = engine.start_session(quiz_id, user_id).await;
        assert!(session_result.is_ok());
        
        let session = session_result.unwrap();
        assert_eq!(session.quiz_id, quiz_id);
        assert_eq!(session.user_id, user_id);
        assert!(!session.completed);
        
        // Submit a correct answer
        let answer = Answer::SingleChoice(choice1_id);
        let submission_result = engine.submit_answer(session.id, question_id, answer).await;
        assert!(submission_result.is_ok());
        assert!(submission_result.unwrap()); // Should be correct
        
        // Complete the session
        let completion_result = engine.complete_session(session.id).await;
        assert!(completion_result.is_ok());
        
        let score = completion_result.unwrap();
        assert_eq!(score, 1.0); // Perfect score
    }
    
    #[tokio::test]
    async fn test_asset_caching() {
        let (engine, _temp_dir) = setup_test_environment().await;
        
        // Create test asset data
        let quiz_id = Uuid::new_v4();
        let question_id = Uuid::new_v4();
        let test_data = b"Test image data".to_vec();
        let filename = "test_image.png";
        
        // Store the asset
        let asset_result = engine.store_asset(
            test_data.clone(),
            filename,
            Some(quiz_id),
            Some(question_id)
        ).await;
        
        assert!(asset_result.is_ok());
        let asset_metadata = asset_result.unwrap();
        
        // Retrieve the asset
        let asset_result = engine.get_asset(&asset_metadata.id).await;
        assert!(asset_result.is_ok());
        
        let (data, asset_type, etag) = asset_result.unwrap();
        assert_eq!(data, test_data);
        assert_eq!(asset_type, crate::quiz::asset_cache::AssetType::Image);
        assert!(!etag.is_empty());
        
        // Get quiz assets
        let quiz_assets_result = engine.get_quiz_assets(quiz_id).await;
        assert!(quiz_assets_result.is_ok());
        
        let quiz_assets = quiz_assets_result.unwrap();
        assert_eq!(quiz_assets.len(), 1);
        assert_eq!(quiz_assets[0].id, asset_metadata.id);
        
        // Delete the asset
        let delete_result = engine.delete_asset(&asset_metadata.id).await;
        assert!(delete_result.is_ok());
        
        // Verify it's gone
        let quiz_assets_result = engine.get_quiz_assets(quiz_id).await;
        assert!(quiz_assets_result.is_ok());
        assert_eq!(quiz_assets_result.unwrap().len(), 0);
    }
    
    #[tokio::test]
    async fn test_query_optimization() {
        let (engine, _temp_dir) = setup_test_environment().await;
        
        // Create multiple quizzes
        for i in 0..5 {
            let quiz = Quiz {
                id: Uuid::new_v4(),
                title: format!("Test Quiz {}", i),
                description: Some(format!("Description {}", i)),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                questions: Vec::new(),
                settings: Default::default(),
                author_id: None,
                visibility: if i % 2 == 0 { QuizVisibility::Public } else { QuizVisibility::Private },
                tags: vec!["test".to_string()],
                study_mode: StudyMode::MultipleChoice,
            };
            
            engine.create_quiz(quiz).await.expect("Failed to create quiz");
        }
        
        // Test filtering by visibility
        let public_filters = crate::quiz::query_optimizer::QuizFilters::new()
            .with_visibility(QuizVisibility::Public);
        
        let public_quizzes = engine.get_quizzes_optimized(public_filters).await.expect("Failed to retrieve quizzes");
        assert!(public_quizzes.len() >= 2); // At least the ones we just created
        
        // Test cache stats
        let (hits, misses, hit_rate) = engine.get_query_cache_stats();
        assert_eq!(hits, 0); // First query is always a miss
        assert_eq!(misses, 1);
        assert_eq!(hit_rate, 0.0);
        
        // Query again to test caching
        let _ = engine.get_quizzes_optimized(public_filters).await.expect("Failed to retrieve quizzes");
        
        // Check cache stats again
        let (hits, misses, hit_rate) = engine.get_query_cache_stats();
        assert_eq!(hits, 1); // Should have one hit now
        assert_eq!(misses, 1);
        assert!(hit_rate > 0.0);
    }
}
