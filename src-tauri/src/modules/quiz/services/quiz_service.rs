use std::sync::Arc;
use anyhow::{Result, anyhow};
use sqlx::SqlitePool;
use uuid::Uuid;
use tracing::{info, error};
use std::path::{Path, PathBuf};

use crate::database::repositories::quiz_repository::QuizRepository;
use crate::models::quiz::{
    Quiz, QuizSummary, CreateQuizRequest, UpdateQuizRequest,
    Question, QuestionWithAnswers, CreateQuestionRequest, UpdateQuestionRequest,
    AnswerOption, CreateAnswerOptionRequest, UpdateAnswerOptionRequest,
    QuizAttempt, StartAttemptRequest, CompleteAttemptRequest, AbandonAttemptRequest, AttemptStatus,
    QuizSettings, CreateQuizSettingsRequest, UpdateQuizSettingsRequest,
    QuizActivity, CreateQuizActivityRequest, QuizActivitySummary, QuizActivityStats, ActivityType,
};
use super::sync_service::{QuizSyncService, SyncOperation, SyncPriority};

/// Service for interacting with the quiz module
pub struct QuizService {
    repository: Arc<QuizRepository>,
    sync_service: Option<Arc<QuizSyncService>>,
    data_dir: PathBuf,
}

impl QuizService {
    /// Create a new QuizService
    pub fn new(db_pool: SqlitePool, data_dir: PathBuf) -> Self {
        let repository = Arc::new(QuizRepository::new(db_pool));
        Self {
            repository,
            sync_service: None,
            data_dir,
        }
    }

    /// Initialize the sync service
    pub async fn init_sync_service(&mut self) -> Result<()> {
        let sync_service = QuizSyncService::new(self.repository.clone(), &self.data_dir)?;
        sync_service.init_sync_db().await?;
        self.sync_service = Some(Arc::new(sync_service));
        Ok(())
    }

    /// Get the sync service
    pub fn get_sync_service(&self) -> Result<Arc<QuizSyncService>> {
        self.sync_service.clone().ok_or_else(|| anyhow!("Sync service not initialized"))
    }

    /// Initialize the quiz module
    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing quiz module...");

        // Check if quiz tables exist
        let tables_exist = crate::database::init_quiz_db::check_quiz_tables(
            &self.repository.get_pool()
        ).await?;

        if !tables_exist {
            info!("Quiz tables do not exist, initializing...");
            crate::database::init_quiz_db::init_quiz_db(
                &self.repository.get_pool()
            ).await?;
        }

        // Create test data if needed
        let quiz_count = sqlx::query!(
            "SELECT COUNT(*) as count FROM quizzes"
        )
        .fetch_one(self.repository.get_pool())
        .await?
        .count;

        if quiz_count == 0 {
            info!("No quizzes found, creating test data...");
            crate::database::init_quiz_db::create_test_data(
                &self.repository.get_pool()
            ).await?;
        }

        // Initialize sync service
        self.init_sync_service().await?;

        info!("Quiz module initialized successfully");
        Ok(())
    }

    /// Get the repository
    pub fn get_repository(&self) -> Arc<QuizRepository> {
        self.repository.clone()
    }

    /// Launch the quiz module
    pub async fn launch(&mut self) -> Result<()> {
        info!("Launching quiz module...");

        // Initialize the module
        self.initialize().await?;

        // Process any pending sync items
        if let Ok(sync_service) = self.get_sync_service() {
            let processed = sync_service.process_sync_items().await?;
            info!("Processed {} pending sync items", processed);
        }

        info!("Quiz module launched successfully");
        Ok(())
    }

    /// Shutdown the quiz module
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down quiz module...");

        // Export any pending sync items
        if let Ok(sync_service) = self.get_sync_service() {
            let export_path = self.data_dir.join("sync").join("pending_sync.json");
            sync_service.export_sync_data(&export_path).await?;
            info!("Exported pending sync items to {}", export_path.display());
        }

        info!("Quiz module shut down successfully");
        Ok(())
    }

    /// Track quiz activity
    pub async fn track_activity(&self, user_id: &str, quiz_id: Option<&str>, activity_type: ActivityType, data: Option<serde_json::Value>, duration_ms: Option<i64>) -> Result<String> {
        let activity_type_str = activity_type.to_string();
        info!("Tracking quiz activity: {} - {:?} - {}", user_id, quiz_id, activity_type_str);

        // Create activity request
        let request = CreateQuizActivityRequest {
            user_id: user_id.to_string(),
            quiz_id: quiz_id.map(|id| id.to_string()),
            question_id: None,
            attempt_id: None,
            activity_type,
            data,
            duration_ms,
        };

        // Track in database
        let activity_id = self.repository.track_activity(request).await?;

        // Queue sync item for activity tracking
        if let Ok(sync_service) = self.get_sync_service() {
            sync_service.queue_sync_item(
                "quiz_activity",
                &activity_id,
                SyncOperation::Create,
                serde_json::json!({
                    "id": activity_id,
                    "user_id": user_id,
                    "quiz_id": quiz_id,
                    "activity_type": activity_type_str,
                    "timestamp": Utc::now().to_rfc3339(),
                    "data": data,
                    "duration_ms": duration_ms
                }),
                SyncPriority::Medium
            ).await?;
        }

        Ok(activity_id)
    }

    /// Track quiz started activity
    pub async fn track_quiz_started(&self, user_id: &str, quiz_id: &str) -> Result<String> {
        self.track_activity(user_id, Some(quiz_id), ActivityType::QuizStarted, None, None).await
    }

    /// Track quiz completed activity
    pub async fn track_quiz_completed(&self, user_id: &str, quiz_id: &str, score: f64, duration_ms: i64) -> Result<String> {
        let data = serde_json::json!({
            "score": score
        });

        self.track_activity(user_id, Some(quiz_id), ActivityType::QuizCompleted, Some(data), Some(duration_ms)).await
    }

    /// Track quiz abandoned activity
    pub async fn track_quiz_abandoned(&self, user_id: &str, quiz_id: &str, duration_ms: i64) -> Result<String> {
        self.track_activity(user_id, Some(quiz_id), ActivityType::QuizAbandoned, None, Some(duration_ms)).await
    }

    /// Track question answered activity
    pub async fn track_question_answered(&self, user_id: &str, quiz_id: &str, question_id: &str, is_correct: bool, duration_ms: i64) -> Result<String> {
        let data = serde_json::json!({
            "question_id": question_id,
            "is_correct": is_correct
        });

        let request = CreateQuizActivityRequest {
            user_id: user_id.to_string(),
            quiz_id: Some(quiz_id.to_string()),
            question_id: Some(question_id.to_string()),
            attempt_id: None,
            activity_type: ActivityType::QuestionAnswered,
            data: Some(data),
            duration_ms: Some(duration_ms),
        };

        // Track in database
        let activity_id = self.repository.track_activity(request).await?;

        // Queue sync item
        if let Ok(sync_service) = self.get_sync_service() {
            sync_service.queue_sync_item(
                "quiz_activity",
                &activity_id,
                SyncOperation::Create,
                serde_json::json!({
                    "id": activity_id,
                    "user_id": user_id,
                    "quiz_id": quiz_id,
                    "question_id": question_id,
                    "activity_type": "question_answered",
                    "is_correct": is_correct,
                    "timestamp": Utc::now().to_rfc3339(),
                    "duration_ms": duration_ms
                }),
                SyncPriority::Medium
            ).await?;
        }

        Ok(activity_id)
    }

    /// Track flashcard activity
    pub async fn track_flashcard_activity(&self, user_id: &str, quiz_id: &str, question_id: &str, activity_type: ActivityType, rating: Option<i32>) -> Result<String> {
        let data = match rating {
            Some(r) => serde_json::json!({ "rating": r }),
            None => serde_json::json!({})
        };

        let request = CreateQuizActivityRequest {
            user_id: user_id.to_string(),
            quiz_id: Some(quiz_id.to_string()),
            question_id: Some(question_id.to_string()),
            attempt_id: None,
            activity_type,
            data: Some(data),
            duration_ms: None,
        };

        // Track in database
        self.repository.track_activity(request).await
    }

    /// Get activity summary for a user
    pub async fn get_user_activity_summary(&self, user_id: &str) -> Result<QuizActivitySummary> {
        self.repository.get_activity_summary_by_user(user_id).await
    }

    /// Get activity summary for a quiz
    pub async fn get_quiz_activity_summary(&self, quiz_id: &str) -> Result<QuizActivitySummary> {
        self.repository.get_activity_summary_by_quiz(quiz_id).await
    }

    /// Get activity stats
    pub async fn get_activity_stats(&self, user_id: Option<&str>) -> Result<QuizActivityStats> {
        self.repository.get_activity_stats(user_id).await
    }

    /// Sync with main app
    pub async fn sync_with_main_app(&self, main_app_sync_path: &Path) -> Result<()> {
        info!("Syncing with main app: {}", main_app_sync_path.display());

        if let Ok(sync_service) = self.get_sync_service() {
            // Export our pending items
            let export_path = self.data_dir.join("sync").join("export_sync.json");
            sync_service.export_sync_data(&export_path).await?;

            // Import items from main app
            if main_app_sync_path.exists() {
                let imported = sync_service.import_sync_data(main_app_sync_path).await?;
                info!("Imported {} sync items from main app", imported);
            }

            // Process pending items
            let processed = sync_service.process_sync_items().await?;
            info!("Processed {} sync items", processed);
        }

        Ok(())
    }

    /// Launch the quiz module for a specific quiz and user
    pub async fn launch_quiz_module(&self, quiz_id: &str, user_id: &str) -> Result<String> {
        info!("Launching quiz module for quiz {} and user {}", quiz_id, user_id);

        // Check if quiz exists
        let quiz = self.repository.get_quiz_by_id(quiz_id).await?;

        // Create launch configuration
        let config = crate::modules::quiz::ui::QuizLaunchConfig {
            quiz_id: quiz_id.to_string(),
            user_id: user_id.to_string(),
            session_id: None,
            return_url: None,
            standalone: false,
            theme: None,
            language: None,
        };

        // Create launch component
        let launch_component = crate::modules::quiz::ui::QuizLaunchComponent::new(
            "http://localhost:8080",
            self.data_dir.to_str().unwrap_or("./data")
        );

        // Launch the quiz module
        let response = launch_component.launch(config).await?;

        // Track the launch activity
        self.track_activity(
            user_id,
            Some(quiz_id),
            ActivityType::QuizStarted,
            Some(serde_json::json!({
                "session_id": response.session_id,
                "standalone": false
            })),
            None
        ).await?;

        // Return the session ID
        Ok(response.session_id.unwrap_or_else(|| Uuid::new_v4().to_string()))
    }
}
