#[cfg(test)]
mod tests {
    use crate::core::config::Config;
    use crate::quiz::models::{Quiz, Question, Answer, QuestionContent, AnswerType, StudyMode, QuizVisibility};
    use crate::quiz::QuizEngine;
    use crate::quiz::query_optimizer::QuizFilters;
    use std::sync::Arc;
    use tokio;
    use uuid::Uuid;
    use std::time::{Duration, Instant};
    use tempfile::TempDir;
    use std::collections::HashMap;

    async fn setup_benchmark_environment() -> (QuizEngine, TempDir) {
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
    
    async fn create_test_quizzes(engine: &QuizEngine, count: usize) -> Vec<Uuid> {
        let mut quiz_ids = Vec::with_capacity(count);
        
        for i in 0..count {
            let quiz_id = Uuid::new_v4();
            quiz_ids.push(quiz_id);
            
            // Create questions
            let mut questions = Vec::new();
            for j in 0..10 {
                let question_id = Uuid::new_v4();
                let choice1_id = Uuid::new_v4();
                let choice2_id = Uuid::new_v4();
                let choice3_id = Uuid::new_v4();
                
                let choices = vec![
                    crate::quiz::models::Choice {
                        id: choice1_id,
                        text: format!("Choice 1 for question {}", j),
                        rich_text: None,
                        image_url: None,
                    },
                    crate::quiz::models::Choice {
                        id: choice2_id,
                        text: format!("Choice 2 for question {}", j),
                        rich_text: None,
                        image_url: None,
                    },
                    crate::quiz::models::Choice {
                        id: choice3_id,
                        text: format!("Choice 3 for question {}", j),
                        rich_text: None,
                        image_url: None,
                    },
                ];
                
                let question = Question {
                    id: question_id,
                    quiz_id,
                    content: QuestionContent {
                        text: format!("Question {} for quiz {}", j, i),
                        rich_text: None,
                        image_url: None,
                        audio_url: None,
                    },
                    answer_type: AnswerType::MultipleChoice,
                    choices: choices.clone(),
                    correct_answer: Answer::SingleChoice(choice1_id),
                    explanation: Some(format!("Explanation for question {}", j)),
                };
                
                questions.push(question);
            }
            
            let quiz = Quiz {
                id: quiz_id,
                title: format!("Benchmark Quiz {}", i),
                description: Some(format!("Description for quiz {}", i)),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                questions,
                settings: Default::default(),
                author_id: None,
                visibility: QuizVisibility::Public,
                tags: vec!["benchmark".to_string()],
                study_mode: StudyMode::MultipleChoice,
            };
            
            engine.create_quiz(quiz).await.expect("Failed to create quiz");
        }
        
        quiz_ids
    }
    
    #[tokio::test]
    async fn benchmark_quiz_retrieval() {
        let (engine, _temp_dir) = setup_benchmark_environment().await;
        
        // Create test data - 100 quizzes with 10 questions each
        let quiz_count = 100;
        let quiz_ids = create_test_quizzes(&engine, quiz_count).await;
        
        // Benchmark standard retrieval
        let start = Instant::now();
        for &quiz_id in &quiz_ids[0..10] {
            let _ = engine.get_quiz(quiz_id).await.expect("Failed to retrieve quiz");
        }
        let standard_duration = start.elapsed();
        
        // Benchmark optimized retrieval
        let start = Instant::now();
        let filters = QuizFilters::new().with_limit(10);
        let _ = engine.get_quizzes_optimized(filters).await.expect("Failed to retrieve quizzes");
        let optimized_duration = start.elapsed();
        
        println!("Standard retrieval (10 quizzes): {:?}", standard_duration);
        println!("Optimized retrieval (10 quizzes): {:?}", optimized_duration);
        
        // The optimized retrieval should be faster for multiple quizzes
        assert!(optimized_duration <= standard_duration);
    }
    
    #[tokio::test]
    async fn benchmark_question_batch_loading() {
        let (engine, _temp_dir) = setup_benchmark_environment().await;
        
        // Create test data - 20 quizzes with 10 questions each
        let quiz_count = 20;
        let quiz_ids = create_test_quizzes(&engine, quiz_count).await;
        
        // Benchmark standard question loading
        let start = Instant::now();
        for &quiz_id in &quiz_ids[0..5] {
            let _ = engine.get_quiz_questions(quiz_id).await.expect("Failed to retrieve questions");
        }
        let standard_duration = start.elapsed();
        
        // Benchmark batch loading
        let start = Instant::now();
        let _ = engine.batch_load_questions_optimized(&quiz_ids[0..5]).await.expect("Failed to batch load questions");
        let batch_duration = start.elapsed();
        
        println!("Standard question loading (5 quizzes): {:?}", standard_duration);
        println!("Batch question loading (5 quizzes): {:?}", batch_duration);
        
        // Batch loading should be faster
        assert!(batch_duration <= standard_duration);
    }
    
    #[tokio::test]
    async fn benchmark_asset_caching() {
        let (engine, _temp_dir) = setup_benchmark_environment().await;
        
        // Create test asset data
        let quiz_id = Uuid::new_v4();
        let question_id = Uuid::new_v4();
        let test_data = vec![0u8; 1024 * 1024]; // 1MB of data
        let filename = "benchmark_image.png";
        
        // Store the asset
        let asset_metadata = engine.store_asset(
            test_data.clone(),
            filename,
            Some(quiz_id),
            Some(question_id)
        ).await.expect("Failed to store asset");
        
        // First retrieval (from disk)
        let start = Instant::now();
        let _ = engine.get_asset(&asset_metadata.id).await.expect("Failed to retrieve asset");
        let first_duration = start.elapsed();
        
        // Second retrieval (from memory cache)
        let start = Instant::now();
        let _ = engine.get_asset(&asset_metadata.id).await.expect("Failed to retrieve asset");
        let second_duration = start.elapsed();
        
        println!("Asset retrieval (disk): {:?}", first_duration);
        println!("Asset retrieval (memory cache): {:?}", second_duration);
        
        // Memory cache should be significantly faster
        assert!(second_duration < first_duration / 2);
    }
    
    #[tokio::test]
    async fn benchmark_quiz_session_performance() {
        let (engine, _temp_dir) = setup_benchmark_environment().await;
        
        // Create a quiz with many questions
        let quiz_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Create 100 questions
        let question_count = 100;
        let mut questions = Vec::with_capacity(question_count);
        let mut question_ids = Vec::with_capacity(question_count);
        
        for i in 0..question_count {
            let question_id = Uuid::new_v4();
            question_ids.push(question_id);
            
            let choice1_id = Uuid::new_v4();
            let choice2_id = Uuid::new_v4();
            let choice3_id = Uuid::new_v4();
            
            let choices = vec![
                crate::quiz::models::Choice {
                    id: choice1_id,
                    text: format!("Choice 1 for question {}", i),
                    rich_text: None,
                    image_url: None,
                },
                crate::quiz::models::Choice {
                    id: choice2_id,
                    text: format!("Choice 2 for question {}", i),
                    rich_text: None,
                    image_url: None,
                },
                crate::quiz::models::Choice {
                    id: choice3_id,
                    text: format!("Choice 3 for question {}", i),
                    rich_text: None,
                    image_url: None,
                },
            ];
            
            let question = Question {
                id: question_id,
                quiz_id,
                content: QuestionContent {
                    text: format!("Performance test question {}", i),
                    rich_text: None,
                    image_url: None,
                    audio_url: None,
                },
                answer_type: AnswerType::MultipleChoice,
                choices: choices.clone(),
                correct_answer: Answer::SingleChoice(choice1_id),
                explanation: Some(format!("Explanation for question {}", i)),
            };
            
            questions.push(question);
        }
        
        let quiz = Quiz {
            id: quiz_id,
            title: "Performance Test Quiz".to_string(),
            description: Some("Quiz with many questions for performance testing".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            questions,
            settings: Default::default(),
            author_id: None,
            visibility: QuizVisibility::Public,
            tags: vec!["performance".to_string()],
            study_mode: StudyMode::MultipleChoice,
        };
        
        // Store the quiz
        engine.create_quiz(quiz).await.expect("Failed to create quiz");
        
        // Start a session
        let start = Instant::now();
        let session = engine.start_session(quiz_id, user_id).await.expect("Failed to start session");
        let session_start_duration = start.elapsed();
        
        // Submit answers to all questions
        let start = Instant::now();
        for &question_id in &question_ids {
            let answer = Answer::SingleChoice(Uuid::new_v4()); // Random wrong answer
            let _ = engine.submit_answer(session.id, question_id, answer).await.expect("Failed to submit answer");
        }
        let answer_submission_duration = start.elapsed();
        
        // Complete the session
        let start = Instant::now();
        let _ = engine.complete_session(session.id).await.expect("Failed to complete session");
        let session_completion_duration = start.elapsed();
        
        println!("Session start duration: {:?}", session_start_duration);
        println!("Answer submission duration (100 questions): {:?}", answer_submission_duration);
        println!("Session completion duration: {:?}", session_completion_duration);
        
        // Average time per answer submission
        let avg_submission_time = answer_submission_duration.as_millis() as f64 / question_count as f64;
        println!("Average time per answer submission: {:.2}ms", avg_submission_time);
        
        // Performance assertions
        assert!(session_start_duration.as_millis() < 500, "Session start should be under 500ms");
        assert!(avg_submission_time < 50.0, "Average answer submission should be under 50ms");
        assert!(session_completion_duration.as_millis() < 1000, "Session completion should be under 1000ms");
    }
}
