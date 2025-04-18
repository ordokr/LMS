// Quiz module for Ordo LMS
// Ported from Quenti (https://github.com/quenti-io/quenti)

pub mod models;
pub mod storage;
pub mod commands;
pub mod session;
pub mod sync;
pub mod standalone;
pub mod spaced_repetition;
pub mod analytics;
pub mod export;
pub mod course_integration;
pub mod auth;
pub mod notification;
pub mod collaboration;
pub mod collaboration_comments;
pub mod collaboration_methods;
pub mod templates;
pub mod templates_retrieve;
pub mod templates_rating;
pub mod templates_storage;
pub mod template_methods;
pub mod ai_generation;
pub mod ai_generation_quiz;
pub mod ai_generation_storage;
pub mod ai_generation_retrieve;
pub mod ai_generation_providers;
pub mod ai_generation_methods;
pub mod adaptive_learning;
pub mod adaptive_learning_retrieve;
pub mod adaptive_learning_storage;
pub mod adaptive_learning_progress;
pub mod taking;
pub mod ui_handler;
pub mod ui_controller;
pub mod taking_controller;

// Performance optimization modules
pub mod query_optimizer;
pub mod asset_cache;
pub mod sync_manager;
pub mod lti;
pub mod scorm;
pub mod xapi;
pub mod adaptive_learning_methods;

#[cfg(test)]
mod tests;

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::Seek;

// External system integration modules
pub mod cmi5;  // Primary standard for LMS integration
pub mod scorm; // Backup standard for compatibility
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::collections::HashMap;

use crate::core::config::Config;
use crate::db::HybridStore;
use crate::course::storage::CourseStore;
use crate::auth::AuthService;
use crate::notification::NotificationService;
use session::QuizSession;
use spaced_repetition::{SpacedRepetitionScheduler, FlashcardRating};
use analytics::{AnalyticsEngine, TimePeriod};
use export::{QuizExportEngine, ExportOptions, ExportFormat};
use course_integration::CourseIntegrationService;
use auth::{QuizAuthService, QuizAuthMiddleware};
use notification::QuizNotificationService;
use collaboration::CollaborationService;
use templates::{TemplateService, QuizTemplate, QuestionTemplate, TemplateCategory, TemplateRating};
use ai_generation::{AIGenerationService, AIGenerationRequest, AIGenerationResult, AISourceType, AIModelType, AIGenerationStatus, AIModelProvider};
use ai_generation_providers::{MockAIModelProvider, OpenAIModelProvider, AnthropicModelProvider};
use adaptive_learning::{AdaptiveLearningService, AdaptiveLearningPath, LearningPathNode, LearningPathEdge, LearningPathNodeType, EdgeConditionType, UserLearningPathProgress, LearningPathRecommendation};
use query_optimizer::{QuizQueryOptimizer, QuizFilters};
use asset_cache::{AssetCache, AssetCacheConfig, AssetMetadata, AssetType};
use sync_manager::{SyncManager, SyncManagerConfig, SyncPriority, ConflictStrategy, SyncNotification};
use lti::{LtiService, LtiPlatformConfig, LtiVersion, LtiLaunchRequest, LtiRole};
use scorm::{ScormService, ScormPackageMetadata, ScormSession, ScormActivityState};
use xapi::{XApiClient, XApiClientConfig, XApiStatement, XApiStatementBuilder};

#[derive(Clone)]
pub struct QuizEngine {
    store: Arc<storage::HybridQuizStore>,
    scheduler: Arc<SpacedRepetitionScheduler>,
    analytics: Arc<AnalyticsEngine>,
    export_engine: Arc<QuizExportEngine>,
    session_queue: mpsc::UnboundedSender<QuizSession>,
    course_integration: Option<Arc<CourseIntegrationService>>,
    auth_service: Option<Arc<QuizAuthService>>,
    auth_middleware: Option<Arc<QuizAuthMiddleware>>,
    notification_service: Option<Arc<QuizNotificationService>>,
    collaboration_service: Option<Arc<CollaborationService>>,
    template_service: Option<Arc<TemplateService>>,
    ai_generation_service: Option<Arc<AIGenerationService>>,
    adaptive_learning_service: Option<Arc<AdaptiveLearningService>>,

    // Performance optimization components
    query_optimizer: Arc<QuizQueryOptimizer>,
    asset_cache: Arc<AssetCache>,
    sync_manager: Arc<SyncManager>,

    // External system integration components
    lti_service: Arc<Mutex<LtiService>>,
    scorm_service: Arc<ScormService>,
    xapi_client: Option<Arc<XApiClient>>,
}

impl QuizEngine {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Create a mutable store first
        let mut store = storage::HybridQuizStore::new(config)?;
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Create the scheduler
        let scheduler = Arc::new(SpacedRepetitionScheduler::new(Arc::new(store.clone())));

        // Set the scheduler in the store
        store.set_spaced_repetition_scheduler(scheduler.clone());

        // Create the analytics engine
        let analytics = Arc::new(AnalyticsEngine::new(Arc::new(store.clone())));

        // Create the export engine
        let export_engine = Arc::new(QuizExportEngine::new(Arc::new(store.clone())));

        // Create the immutable store
        let store = Arc::new(store);

        // Process quiz sessions in background
        let store_clone = store.clone();
        tokio::spawn(async move {
            while let Some(session) = rx.recv().await {
                if let Err(e) = store_clone.update_session(&session).await {
                    eprintln!("Failed to update session: {}", e);
                }
            }
        });

        // Try to get the course store if available
        let course_integration = match CourseStore::new(config) {
            Ok(course_store) => {
                let course_store = Arc::new(course_store);
                let integration = CourseIntegrationService::new(store.clone(), course_store);
                Some(Arc::new(integration))
            },
            Err(_) => None,
        };

        // Try to get the auth service if available
        let (auth_service, auth_middleware) = match AuthService::new(config) {
            Ok(auth_service) => {
                let auth_service_arc = Arc::new(auth_service);
                let quiz_auth_service = QuizAuthService::new(auth_service_arc.clone());
                let quiz_auth_service_arc = Arc::new(quiz_auth_service);
                let middleware = QuizAuthMiddleware::new(quiz_auth_service_arc.clone());
                (Some(quiz_auth_service_arc), Some(Arc::new(middleware)))
            },
            Err(_) => (None, None),
        };

        // Try to get the notification service if available
        let notification_service = match NotificationService::new(config) {
            Ok(notification_service) => {
                let notification_service_arc = Arc::new(notification_service);
                let quiz_notification_service = QuizNotificationService::new(
                    store.get_sqlite_pool().clone(),
                    Some(notification_service_arc.clone())
                );
                Some(Arc::new(quiz_notification_service))
            },
            Err(_) => {
                // Create a notification service without the global notification service
                let quiz_notification_service = QuizNotificationService::new(
                    store.get_sqlite_pool().clone(),
                    None
                );
                Some(Arc::new(quiz_notification_service))
            },
        };

        // Create the collaboration service
        let collaboration_service = {
            let collaboration_service = CollaborationService::new(
                store.get_sqlite_pool().clone(),
                store.clone()
            );
            Some(Arc::new(collaboration_service))
        };

        // Create the template service
        let template_service = {
            let template_service = TemplateService::new(
                store.get_sqlite_pool().clone(),
                store.clone()
            );
            Some(Arc::new(template_service))
        };

        // Create the AI generation service
        let ai_generation_service = {
            let mut ai_service = AIGenerationService::new(
                store.get_sqlite_pool().clone(),
                store.clone()
            );

            // Register model providers
            ai_service.register_model_provider(Box::new(MockAIModelProvider));

            // Register OpenAI provider if API key is available
            if let Some(openai_api_key) = std::env::var("OPENAI_API_KEY").ok() {
                ai_service.register_model_provider(Box::new(OpenAIModelProvider::new(
                    openai_api_key,
                    "gpt-4".to_string(),
                )));
            }

            // Register Anthropic provider if API key is available
            if let Some(anthropic_api_key) = std::env::var("ANTHROPIC_API_KEY").ok() {
                ai_service.register_model_provider(Box::new(AnthropicModelProvider::new(
                    anthropic_api_key,
                    "claude-3-opus".to_string(),
                )));
            }

            Some(Arc::new(ai_service))
        };

        // Create the adaptive learning service
        let adaptive_learning_service = {
            let adaptive_learning_service = AdaptiveLearningService::new(
                store.get_sqlite_pool().clone(),
                store.clone()
            );
            Some(Arc::new(adaptive_learning_service))
        };

        // Create the query optimizer
        let query_optimizer = Arc::new(QuizQueryOptimizer::new(store.get_sqlite_pool().clone())
            .with_cache_config(
                std::time::Duration::from_secs(300), // 5 minute cache TTL
                1000 // Maximum cache entries
            ));

        // Create the asset cache
        let asset_cache_config = AssetCacheConfig {
            cache_dir: config.data_dir.join("cache/assets"),
            max_memory_size: 100 * 1024 * 1024, // 100 MB
            ttl: std::time::Duration::from_secs(3600), // 1 hour
            preload_assets: true,
        };

        let asset_cache = match AssetCache::new(asset_cache_config).await {
            Ok(cache) => Arc::new(cache),
            Err(e) => {
                eprintln!("Failed to initialize asset cache: {}", e);
                // Create a fallback cache with default settings
                let fallback_config = AssetCacheConfig {
                    cache_dir: config.data_dir.join("cache/assets"),
                    preload_assets: false,
                    ..Default::default()
                };
                Arc::new(AssetCache::new(fallback_config).await?)
            }
        };

        // Create the sync manager
        let (notification_tx, mut notification_rx) = mpsc::channel(100);

        let sync_config = SyncManagerConfig {
            sync_interval: 60, // 1 minute
            max_retries: 5,
            initial_retry_delay: 5, // 5 seconds
            max_retry_delay: 3600, // 1 hour
            default_conflict_strategy: ConflictStrategy::Merge,
            auto_sync: true,
            show_notifications: true,
            batch_size: 10,
        };

        let sync_manager = Arc::new(SyncManager::new(
            store.clone(),
            sync_config,
            notification_tx,
        ));

        // Start the sync manager
        sync_manager.start().await?;

        // Forward sync notifications to the notification service
        if let Some(notification_service) = &notification_service {
            let notification_service_clone = notification_service.clone();
            tokio::spawn(async move {
                while let Some(sync_notification) = notification_rx.recv().await {
                    let _ = notification_service_clone.send_notification(
                        "Sync".to_string(),
                        sync_notification.title,
                        sync_notification.message,
                        None,
                    ).await;
                }
            });
        }

        // Initialize LTI service
        let lti_service = Arc::new(Mutex::new(LtiService::new()));

        // Initialize SCORM service
        let scorm_package_dir = config.data_dir.join("scorm/packages");
        let scorm_service = match ScormService::new(scorm_package_dir) {
            Ok(service) => Arc::new(service),
            Err(e) => {
                eprintln!("Failed to initialize SCORM service: {}", e);
                return Err(Box::new(e));
            }
        };

        // Initialize xAPI client if configured
        let xapi_client = if let Some(xapi_config) = config.get_table("xapi").ok() {
            if let (Some(endpoint), Some(username), Some(password)) = (
                xapi_config.get("endpoint").and_then(|v| v.as_str()),
                xapi_config.get("username").and_then(|v| v.as_str()),
                xapi_config.get("password").and_then(|v| v.as_str()),
            ) {
                match XApiClient::new(XApiClientConfig {
                    endpoint: endpoint.to_string(),
                    username: username.to_string(),
                    password: password.to_string(),
                    version: xapi_config.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("1.0.3")
                        .to_string(),
                }) {
                    Ok(client) => Some(Arc::new(client)),
                    Err(e) => {
                        eprintln!("Failed to initialize xAPI client: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        // Start a background task to periodically clear expired cache entries
        let query_optimizer_clone = query_optimizer.clone();
        let asset_cache_clone = asset_cache.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // 5 minutes
            loop {
                interval.tick().await;

                // Clear expired query cache entries
                if let Err(e) = query_optimizer_clone.clear_expired_cache().await {
                    eprintln!("Failed to clear expired query cache: {}", e);
                }

                // Clear expired asset cache entries
                if let Err(e) = asset_cache_clone.clear_expired().await {
                    eprintln!("Failed to clear expired asset cache: {}", e);
                }
            }
        });

        Ok(Self {
            store,
            scheduler,
            analytics,
            export_engine,
            session_queue: tx,
            course_integration,
            auth_service,
            auth_middleware,
            notification_service,
            collaboration_service,
            template_service,
            ai_generation_service,
            adaptive_learning_service,
            query_optimizer,
            asset_cache,
            sync_manager,
            lti_service,
            scorm_service,
            xapi_client,
        })
    }

    pub async fn create_quiz(&self, quiz: models::Quiz) -> Result<uuid::Uuid, Box<dyn std::error::Error + Send + Sync>> {
        self.store.store_quiz(&quiz).await?;
        Ok(quiz.id)
    }

    pub async fn start_session(&self, quiz_id: uuid::Uuid, user_id: uuid::Uuid) -> Result<QuizSession, Box<dyn std::error::Error + Send + Sync>> {
        let quiz = self.store.get_quiz(quiz_id).await?;
        let session = QuizSession::new(quiz_id, user_id);
        self.store.store_session(&session).await?;
        Ok(session)
    }

    pub async fn submit_answer(&self, session_id: uuid::Uuid, question_id: uuid::Uuid, answer: models::Answer) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut session = self.store.get_session(session_id).await?;
        let result = session.submit_answer(question_id, answer)?;
        self.session_queue.send(session).map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(result)
    }

    pub async fn complete_session(&self, session_id: uuid::Uuid) -> Result<f32, Box<dyn std::error::Error + Send + Sync>> {
        let mut session = self.store.get_session(session_id).await?;
        let score = session.complete()?;
        self.store.update_session(&session).await?;

        // No need to update spaced repetition data here anymore
        // It's handled by the flashcard rating system

        Ok(score)
    }

    // Flashcard methods

    /// Rate a flashcard and update its spaced repetition data
    pub async fn rate_flashcard(&self, question_id: uuid::Uuid, user_id: uuid::Uuid, rating: i32) -> Result<models::FlashcardData, Box<dyn std::error::Error + Send + Sync>> {
        let rating = FlashcardRating::from(rating);
        self.scheduler.process_rating(question_id, user_id, rating).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Create a flashcard study session with due cards
    pub async fn create_flashcard_session(&self, user_id: uuid::Uuid, limit: usize) -> Result<(QuizSession, Vec<models::Question>), Box<dyn std::error::Error + Send + Sync>> {
        self.scheduler.create_flashcard_session(user_id, limit).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get flashcard statistics for a user
    pub async fn get_flashcard_stats(&self, user_id: uuid::Uuid) -> Result<spaced_repetition::FlashcardStatistics, Box<dyn std::error::Error + Send + Sync>> {
        self.scheduler.get_user_statistics(user_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // Analytics methods

    /// Get study statistics for a user
    pub async fn get_user_stats(&self, user_id: uuid::Uuid, period: TimePeriod) -> Result<analytics::UserStudyStats, Box<dyn std::error::Error + Send + Sync>> {
        self.analytics.get_user_stats(user_id, period).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get analytics for a quiz
    pub async fn get_quiz_analytics(&self, quiz_id: uuid::Uuid, period: TimePeriod) -> Result<analytics::QuizAnalytics, Box<dyn std::error::Error + Send + Sync>> {
        self.analytics.get_quiz_analytics(quiz_id, period).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Generate a PDF report for a user's study statistics
    pub async fn generate_user_report(&self, user_id: uuid::Uuid, period: TimePeriod) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        self.analytics.generate_user_report(user_id, period).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Generate a PDF report for a quiz's analytics
    pub async fn generate_quiz_report(&self, quiz_id: uuid::Uuid, period: TimePeriod) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        self.analytics.generate_quiz_report(quiz_id, period).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // Export/Import methods

    /// Export a quiz to a file
    pub async fn export_quiz_to_file(&self, quiz_id: uuid::Uuid, path: &Path, format: ExportFormat) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let options = ExportOptions {
            format,
            ..Default::default()
        };

        self.export_engine.export_quiz_to_file(quiz_id, path, options).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Export a quiz to a byte array
    pub async fn export_quiz(&self, quiz_id: uuid::Uuid, format: ExportFormat) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let options = ExportOptions {
            format,
            ..Default::default()
        };

        self.export_engine.export_quiz(quiz_id, options).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Export a quiz with custom options
    pub async fn export_quiz_with_options(&self, quiz_id: uuid::Uuid, options: ExportOptions) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        self.export_engine.export_quiz(quiz_id, options).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Import a quiz from a file
    pub async fn import_quiz_from_file(&self, path: &Path) -> Result<uuid::Uuid, Box<dyn std::error::Error + Send + Sync>> {
        self.export_engine.import_quiz_from_file(path).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Import a quiz from a byte array
    pub async fn import_quiz(&self, data: &[u8], format: ExportFormat) -> Result<uuid::Uuid, Box<dyn std::error::Error + Send + Sync>> {
        self.export_engine.import_quiz(data, format).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // Course Integration methods

    /// Add a quiz to a course
    pub async fn add_quiz_to_course(
        &self,
        quiz_id: uuid::Uuid,
        course_id: uuid::Uuid,
        module_id: Option<uuid::Uuid>,
        section_id: Option<uuid::Uuid>,
        position: Option<i32>,
    ) -> Result<course_integration::QuizCourseMapping, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.add_quiz_to_course(quiz_id, course_id, module_id, section_id, position).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Remove a quiz from a course
    pub async fn remove_quiz_from_course(
        &self,
        mapping_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.remove_quiz_from_course(mapping_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Update a quiz-course mapping
    pub async fn update_quiz_course_mapping(
        &self,
        mapping: &course_integration::QuizCourseMapping,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.update_mapping(mapping).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Get all quizzes for a course
    pub async fn get_quizzes_for_course(
        &self,
        course_id: uuid::Uuid,
    ) -> Result<Vec<course_integration::QuizCourseMapping>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.get_quizzes_for_course(course_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Get all courses for a quiz
    pub async fn get_courses_for_quiz(
        &self,
        quiz_id: uuid::Uuid,
    ) -> Result<Vec<(course_integration::QuizCourseMapping, crate::course::models::Course)>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.get_courses_for_quiz(quiz_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Get a quiz with course context
    pub async fn get_quiz_with_context(
        &self,
        mapping_id: uuid::Uuid,
        student_id: Option<uuid::Uuid>,
    ) -> Result<course_integration::QuizWithContext, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.get_quiz_with_context(mapping_id, student_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Get all quizzes for a student in a course
    pub async fn get_student_quizzes(
        &self,
        course_id: uuid::Uuid,
        student_id: uuid::Uuid,
    ) -> Result<Vec<course_integration::QuizWithContext>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.get_student_quizzes(course_id, student_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Create or update a quiz assignment for a student
    pub async fn assign_quiz_to_student(
        &self,
        mapping_id: uuid::Uuid,
        student_id: uuid::Uuid,
    ) -> Result<course_integration::QuizAssignment, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.assign_quiz_to_student(mapping_id, student_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Update a quiz assignment status based on an attempt
    pub async fn update_assignment_from_attempt(
        &self,
        mapping_id: uuid::Uuid,
        student_id: uuid::Uuid,
        attempt: &models::QuizAttempt,
    ) -> Result<course_integration::QuizAssignment, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            integration.update_assignment_from_attempt(mapping_id, student_id, attempt).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    // Authentication methods

    /// Check if a user has permission to view a quiz
    pub async fn can_view_quiz(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::View).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to edit a quiz
    pub async fn can_edit_quiz(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::Edit).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to delete a quiz
    pub async fn can_delete_quiz(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::Delete).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to attempt a quiz
    pub async fn can_attempt_quiz(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::Attempt).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to view quiz results
    pub async fn can_view_quiz_results(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::ViewResults).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to view quiz analytics
    pub async fn can_view_quiz_analytics(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::ViewAnalytics).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Check if a user has permission to manage course integration
    pub async fn can_manage_course_integration(
        &self,
        user_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(auth) = &self.auth_service {
            auth.check_quiz_permission(user_id, quiz_id, auth::QuizPermission::ManageCourseIntegration).await
        } else {
            // If auth service is not available, allow access
            Ok(true)
        }
    }

    /// Get the auth middleware
    pub fn get_auth_middleware(&self) -> Option<Arc<QuizAuthMiddleware>> {
        self.auth_middleware.clone()
    }

    // Notification methods

    /// Get notifications for a user
    pub async fn get_notifications_for_user(
        &self,
        user_id: uuid::Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<notification::QuizNotification>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.get_notifications_for_user(user_id, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(Vec::new())
        }
    }

    /// Get unread notification count for a user
    pub async fn get_unread_count_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<i64, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.get_unread_count_for_user(user_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(0)
        }
    }

    /// Mark a notification as read
    pub async fn mark_notification_as_read(
        &self,
        notification_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.mark_notification_as_read(notification_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Mark all notifications as read for a user
    pub async fn mark_all_notifications_as_read(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.mark_all_notifications_as_read(user_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Delete a notification
    pub async fn delete_notification(
        &self,
        notification_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.delete_notification(notification_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Delete all notifications for a user
    pub async fn delete_all_notifications_for_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.delete_all_notifications_for_user(user_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Check for due soon quizzes and send notifications
    pub async fn check_due_soon_quizzes(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.check_due_soon_quizzes().await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Check for overdue quizzes and send notifications
    pub async fn check_overdue_quizzes(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            notification_service.check_overdue_quizzes().await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Send a notification when a quiz is assigned to a student
    pub async fn notify_quiz_assigned(
        &self,
        student_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
        mapping_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            // Get the quiz
            let quiz = self.get_quiz(quiz_id).await?;

            // Get the mapping
            let mapping = self.get_quiz_course_mapping(mapping_id).await?;

            notification_service.notify_quiz_assigned(student_id, &quiz, &mapping).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    /// Send a notification when a quiz is completed
    pub async fn notify_quiz_completed(
        &self,
        student_id: uuid::Uuid,
        quiz_id: uuid::Uuid,
        mapping_id: uuid::Uuid,
        attempt_id: uuid::Uuid,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(notification_service) = &self.notification_service {
            // Get the quiz
            let quiz = self.get_quiz(quiz_id).await?;

            // Get the mapping
            let mapping = self.get_quiz_course_mapping(mapping_id).await?;

            // Get the attempt
            let attempt = self.get_quiz_attempt(attempt_id).await?;

            notification_service.notify_quiz_completed(student_id, &quiz, &mapping, &attempt).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Ok(())
        }
    }

    // Helper methods for notifications

    /// Get a quiz course mapping
    async fn get_quiz_course_mapping(&self, mapping_id: uuid::Uuid) -> Result<course_integration::QuizCourseMapping, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(integration) = &self.course_integration {
            // Use the course integration service to get the mapping
            integration.get_mapping(mapping_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err("Course integration is not available".into())
        }
    }

    /// Get a quiz attempt
    async fn get_quiz_attempt(&self, attempt_id: uuid::Uuid) -> Result<models::QuizAttempt, Box<dyn std::error::Error + Send + Sync>> {
        self.store.get_quiz_attempt(attempt_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // Performance-optimized methods

    /// Get quizzes with optimized query and caching
    pub async fn get_quizzes_optimized(&self, filters: QuizFilters) -> Result<Vec<models::Quiz>, Box<dyn std::error::Error + Send + Sync>> {
        self.query_optimizer.fetch_quizzes(&filters).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get questions for a quiz with optimized loading and caching
    pub async fn get_quiz_questions_optimized(&self, quiz_id: uuid::Uuid) -> Result<Vec<models::Question>, Box<dyn std::error::Error + Send + Sync>> {
        self.query_optimizer.load_quiz_questions(quiz_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Batch load questions for multiple quizzes with optimized loading and caching
    pub async fn batch_load_questions_optimized(&self, quiz_ids: &[uuid::Uuid]) -> Result<HashMap<uuid::Uuid, Vec<models::Question>>, Box<dyn std::error::Error + Send + Sync>> {
        self.query_optimizer.batch_load_questions(quiz_ids).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get query optimizer cache statistics
    pub async fn get_query_cache_stats(&self) -> (u64, u64, f64) {
        self.query_optimizer.get_cache_stats()
    }

    // Asset cache methods

    /// Store an asset in the cache
    pub async fn store_asset(&self, data: Vec<u8>, filename: &str, quiz_id: Option<uuid::Uuid>, question_id: Option<uuid::Uuid>) -> Result<AssetMetadata, Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.store_asset(data, filename, quiz_id, question_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get an asset by ID
    pub async fn get_asset(&self, asset_id: &str) -> Result<(Vec<u8>, AssetType, String), Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.get_asset(asset_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get asset metadata by ID
    pub async fn get_asset_metadata(&self, asset_id: &str) -> Result<AssetMetadata, Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.get_asset_metadata(asset_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get all assets for a quiz
    pub async fn get_quiz_assets(&self, quiz_id: uuid::Uuid) -> Result<Vec<AssetMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.get_quiz_assets(quiz_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Delete an asset
    pub async fn delete_asset(&self, asset_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.delete_asset(asset_id).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get asset cache statistics
    pub async fn get_asset_cache_stats(&self) -> Result<asset_cache::AssetCacheStats, Box<dyn std::error::Error + Send + Sync>> {
        self.asset_cache.get_stats().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // Sync manager methods

    /// Get the current sync status
    pub async fn get_sync_status(&self) -> crate::models::network::SyncStatus {
        self.sync_manager.get_sync_status().await
    }

    /// Get the current connection status
    pub async fn get_connection_status(&self) -> crate::models::network::ConnectionStatus {
        self.sync_manager.get_connection_status().await
    }

    /// Set the connection status
    pub async fn set_connection_status(&self, status: crate::models::network::ConnectionStatus) {
        self.sync_manager.set_connection_status(status).await;
    }

    /// Trigger an immediate sync
    pub async fn sync_now(&self) {
        self.sync_manager.sync_now().await;
    }

    /// Get the number of pending sync items
    pub async fn get_pending_sync_count(&self) -> usize {
        self.sync_manager.get_pending_count().await
    }

    /// Get all pending sync items
    pub async fn get_pending_sync_items(&self) -> Vec<crate::models::network::SyncItem> {
        self.sync_manager.get_pending_items().await
    }

    /// Clear all pending sync items
    pub async fn clear_pending_sync_items(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.sync_manager.clear_pending_items().await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Queue a quiz for sync
    pub async fn queue_quiz_sync(&self, quiz: &models::Quiz, operation: crate::models::network::SyncOperation, priority: SyncPriority) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_value(quiz)?;
        self.sync_manager.queue_sync_item(
            "quiz",
            &quiz.id.to_string(),
            operation,
            payload,
            priority,
        ).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Queue a question for sync
    pub async fn queue_question_sync(&self, question: &models::Question, operation: crate::models::network::SyncOperation, priority: SyncPriority) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::to_value(question)?;
        self.sync_manager.queue_sync_item(
            "question",
            &question.id.to_string(),
            operation,
            payload,
            priority,
        ).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // LTI integration methods

    /// Add an LTI platform configuration
    pub async fn add_lti_platform(&self, config: LtiPlatformConfig) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut lti_service = self.lti_service.lock().await;
        lti_service.add_platform(config);
        Ok(())
    }

    /// Get an LTI platform configuration
    pub async fn get_lti_platform(&self, id: &Uuid) -> Result<Option<LtiPlatformConfig>, Box<dyn std::error::Error + Send + Sync>> {
        let lti_service = self.lti_service.lock().await;
        Ok(lti_service.get_platform(id).cloned())
    }

    /// Remove an LTI platform configuration
    pub async fn remove_lti_platform(&self, id: &Uuid) -> Result<Option<LtiPlatformConfig>, Box<dyn std::error::Error + Send + Sync>> {
        let mut lti_service = self.lti_service.lock().await;
        Ok(lti_service.remove_platform(id))
    }

    /// Validate an LTI launch request
    pub async fn validate_lti_launch_request(&self, platform_id: &Uuid, params: &HashMap<String, String>) -> Result<LtiLaunchRequest, Box<dyn std::error::Error + Send + Sync>> {
        let lti_service = self.lti_service.lock().await;
        lti_service.validate_launch_request(platform_id, params)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // SCORM integration methods

    /// Import a SCORM package
    pub async fn import_scorm_package(&self, package_path: &Path) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        let mut scorm_service = self.scorm_service.clone();
        scorm_service.import_package(package_path)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get a SCORM package by ID
    pub async fn get_scorm_package(&self, id: &Uuid) -> Result<Option<ScormPackageMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.scorm_service.get_package(id).cloned())
    }

    /// Get all SCORM packages
    pub async fn get_scorm_packages(&self) -> Result<Vec<ScormPackageMetadata>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.scorm_service.get_packages().into_iter().cloned().collect())
    }

    /// Delete a SCORM package
    pub async fn delete_scorm_package(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut scorm_service = self.scorm_service.clone();
        scorm_service.delete_package(id)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Create a new SCORM session
    pub async fn create_scorm_session(&self, package_id: &Uuid, user_id: &Uuid) -> Result<Uuid, Box<dyn std::error::Error + Send + Sync>> {
        let mut scorm_service = self.scorm_service.clone();
        scorm_service.create_session(package_id, user_id)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get a SCORM session by ID
    pub async fn get_scorm_session(&self, id: &Uuid) -> Result<Option<ScormSession>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(self.scorm_service.get_session(id).cloned())
    }

    /// Update a SCORM session
    pub async fn update_scorm_session(&self, session: ScormSession) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut scorm_service = self.scorm_service.clone();
        scorm_service.update_session(session)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    /// Get the launch URL for a SCORM package
    pub async fn get_scorm_launch_url(&self, package_id: &Uuid) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.scorm_service.get_launch_url(package_id)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    // xAPI integration methods

    /// Send an xAPI statement
    pub async fn send_xapi_statement(&self, statement: &XApiStatement) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            client.send_statement(statement).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "xAPI client not configured")) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    /// Get an xAPI statement by ID
    pub async fn get_xapi_statement(&self, statement_id: &str) -> Result<XApiStatement, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            client.get_statement(statement_id).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "xAPI client not configured")) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    /// Query xAPI statements
    pub async fn query_xapi_statements(&self, params: &HashMap<String, String>) -> Result<Vec<XApiStatement>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            client.query_statements(params).await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "xAPI client not configured")) as Box<dyn std::error::Error + Send + Sync>)
        }
    }

    /// Track a quiz start event with xAPI
    pub async fn track_quiz_started(&self, user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            let statement = xapi::create_quiz_started_statement(user_id, user_name, user_email, quiz_id, quiz_name);
            client.send_statement(&statement).await
                .map(|_| ())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            // If xAPI is not configured, just return success
            Ok(())
        }
    }

    /// Track a quiz completion event with xAPI
    pub async fn track_quiz_completed(&self, user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str, score: f32, max_score: f32, success: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            let statement = xapi::create_quiz_completed_statement(user_id, user_name, user_email, quiz_id, quiz_name, score, max_score, success);
            client.send_statement(&statement).await
                .map(|_| ())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            // If xAPI is not configured, just return success
            Ok(())
        }
    }

    /// Track a question answered event with xAPI
    pub async fn track_question_answered(&self, user_id: &str, user_name: &str, user_email: &str, quiz_id: &str, quiz_name: &str, question_id: &str, question_text: &str, response: &str, success: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(client) = &self.xapi_client {
            let statement = xapi::create_question_answered_statement(user_id, user_name, user_email, quiz_id, quiz_name, question_id, question_text, response, success);
            client.send_statement(&statement).await
                .map(|_| ())
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        } else {
            // If xAPI is not configured, just return success
            Ok(())
        }
    }
}
