use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;

// Repository imports
use crate::database::repositories::quiz_repository::QuizRepository;
use crate::repositories::user_repository::UserRepository;
use crate::repositories::course_repository::CourseRepository;
use crate::repositories::forum_repository::ForumRepository;

// Service imports
use crate::modules::quiz::services::QuizService;
use crate::services::auth::AuthService;
use crate::services::sync::SyncService;
use crate::services::search::SearchService;
use crate::quiz::cmi5::Cmi5Service;
use crate::quiz::scorm::ScormService;
use crate::quiz::ui_controller::UiController;
use crate::quiz::taking_controller::QuizTakingController;

/// Unified application state for the Ordo LMS application
/// This struct holds all repositories, services, and configuration needed by the application
#[derive(Debug)]
pub struct AppState {
    // Database connection pool
    pub db_pool: Pool<Sqlite>,

    // Configuration
    pub jwt_secret: Vec<u8>,
    pub data_dir: PathBuf,
    pub is_online: std::sync::atomic::AtomicBool,

    // Repositories
    pub quiz_repository: Option<Arc<QuizRepository>>,
    pub user_repository: Option<Arc<UserRepository>>,
    pub course_repository: Option<Arc<CourseRepository>>,
    pub forum_repository: Option<Arc<ForumRepository>>,

    // Services
    pub quiz_service: Option<Arc<QuizService>>,
    pub auth_service: Option<Arc<AuthService>>,
    pub sync_service: Option<Arc<SyncService>>,
    pub search_service: Option<Arc<SearchService>>,
    pub cmi5_service: Option<Arc<Cmi5Service>>,
    pub scorm_service: Option<Arc<Mutex<ScormService>>>,
    pub ui_controller: Arc<Mutex<UiController>>,
    pub quiz_taking_controller: Arc<Mutex<QuizTakingController>>,
}

impl AppState {
    /// Create a new AppState instance with minimal initialization
    pub fn new(db_pool: Pool<Sqlite>, jwt_secret: Vec<u8>, data_dir: PathBuf) -> Self {
        Self {
            db_pool,
            jwt_secret,
            data_dir,
            is_online: std::sync::atomic::AtomicBool::new(true),

            // Initialize repositories to None
            quiz_repository: None,
            user_repository: None,
            course_repository: None,
            forum_repository: None,

            // Initialize services to None
            quiz_service: None,
            auth_service: None,
            sync_service: None,
            search_service: None,
            cmi5_service: None,
            scorm_service: None,
            ui_controller: Arc::new(Mutex::new(UiController::new())),
            quiz_taking_controller: Arc::new(Mutex::new(QuizTakingController::new())),
        }
    }

    /// Create a fully initialized AppState with all repositories and services
    pub async fn new_initialized(db_pool: Pool<Sqlite>, jwt_secret: Vec<u8>, data_dir: PathBuf) -> Result<Self> {
        let mut state = Self::new(db_pool, jwt_secret, data_dir);

        // Initialize repositories
        state = state.with_quiz_repository()
                     .with_user_repository()
                     .with_course_repository()
                     .with_forum_repository();

        // Initialize services
        state = state.with_auth_service();
        state = state.with_quiz_service().await?;
        state = state.with_sync_service();
        state = state.with_search_service();
        state = state.with_cmi5_service()?;
        state = state.with_scorm_service()?;
        state = state.with_ui_controller();
        state = state.with_quiz_taking_controller();

        Ok(state)
    }

    pub fn with_quiz_repository(mut self) -> Self {
        let repository = QuizRepository::new(self.db_pool.clone());
        self.quiz_repository = Some(Arc::new(repository));
        self
    }

    pub fn get_quiz_repository(&self) -> Arc<QuizRepository> {
        self.quiz_repository.clone().expect("Quiz repository not initialized")
    }

    pub async fn with_quiz_service(mut self) -> Result<Self> {
        let service = QuizService::new(self.db_pool.clone(), self.data_dir.clone());
        let mut service = service;
        service.initialize().await?;
        self.quiz_service = Some(Arc::new(service));
        Ok(self)
    }

    pub fn get_quiz_service(&self) -> Result<Arc<QuizService>> {
        self.quiz_service.clone().ok_or_else(|| anyhow!("Quiz service not initialized"))
    }

    /// Get the quiz service or create it if it doesn't exist
    pub async fn get_or_create_quiz_service(&mut self) -> Result<Arc<QuizService>> {
        if self.quiz_service.is_none() {
            let service = QuizService::new(self.db_pool.clone(), self.data_dir.clone());
            let mut service = service;
            service.initialize().await?;
            self.quiz_service = Some(Arc::new(service));
        }

        self.get_quiz_service()
    }

    // Additional repository initialization methods

    pub fn with_user_repository(mut self) -> Self {
        let repository = UserRepository::new(self.db_pool.clone());
        self.user_repository = Some(Arc::new(repository));
        self
    }

    pub fn get_user_repository(&self) -> Result<Arc<UserRepository>> {
        self.user_repository.clone().ok_or_else(|| anyhow!("User repository not initialized"))
    }

    pub fn with_course_repository(mut self) -> Self {
        let repository = CourseRepository::new(self.db_pool.clone());
        self.course_repository = Some(Arc::new(repository));
        self
    }

    pub fn get_course_repository(&self) -> Result<Arc<CourseRepository>> {
        self.course_repository.clone().ok_or_else(|| anyhow!("Course repository not initialized"))
    }

    pub fn with_forum_repository(mut self) -> Self {
        let repository = ForumRepository::new(self.db_pool.clone());
        self.forum_repository = Some(Arc::new(repository));
        self
    }

    pub fn get_forum_repository(&self) -> Result<Arc<ForumRepository>> {
        self.forum_repository.clone().ok_or_else(|| anyhow!("Forum repository not initialized"))
    }

    // Additional service initialization methods

    pub fn with_auth_service(mut self) -> Self {
        let service = AuthService::new(self.db_pool.clone(), self.jwt_secret.clone());
        self.auth_service = Some(Arc::new(service));
        self
    }

    pub fn get_auth_service(&self) -> Result<Arc<AuthService>> {
        self.auth_service.clone().ok_or_else(|| anyhow!("Auth service not initialized"))
    }

    pub fn with_sync_service(mut self) -> Self {
        let service = SyncService::new(self.db_pool.clone());
        self.sync_service = Some(Arc::new(service));
        self
    }

    pub fn get_sync_service(&self) -> Result<Arc<SyncService>> {
        self.sync_service.clone().ok_or_else(|| anyhow!("Sync service not initialized"))
    }

    pub fn with_search_service(mut self) -> Self {
        let service = SearchService::new(self.db_pool.clone());
        self.search_service = Some(Arc::new(service));
        self
    }

    pub fn get_search_service(&self) -> Result<Arc<SearchService>> {
        self.search_service.clone().ok_or_else(|| anyhow!("Search service not initialized"))
    }

    pub fn with_cmi5_service(mut self) -> Result<Self> {
        let launch_service = Arc::new(crate::quiz::cmi5::LaunchService::new(
            "https://example.com/lrs",
            "https://example.com/lrs/auth"
        ));

        let service = Cmi5Service::new(
            "https://example.com/lrs",
            Some("test:test"),
            launch_service
        ).map_err(|e| anyhow!("Failed to create cmi5 service: {}", e))?;

        self.cmi5_service = Some(Arc::new(service));
        Ok(self)
    }

    pub fn get_cmi5_service(&self) -> Result<Arc<Cmi5Service>> {
        self.cmi5_service.clone().ok_or_else(|| anyhow!("CMI5 service not initialized"))
    }

    pub fn with_scorm_service(mut self) -> Result<Self> {
        let scorm_package_dir = self.data_dir.join("scorm_packages");
        std::fs::create_dir_all(&scorm_package_dir)
            .map_err(|e| anyhow!("Failed to create SCORM package directory: {}", e))?;

        let service = ScormService::new(scorm_package_dir)
            .map_err(|e| anyhow!("Failed to create SCORM service: {}", e))?;

        self.scorm_service = Some(Arc::new(Mutex::new(service)));
        Ok(self)
    }

    pub fn get_scorm_service(&self) -> Result<Arc<Mutex<ScormService>>> {
        self.scorm_service.clone().ok_or_else(|| anyhow!("SCORM service not initialized"))
    }

    pub fn with_ui_controller(self) -> Self {
        // UI controller is already initialized in new()
        self
    }

    pub fn get_ui_controller(&self) -> Arc<Mutex<UiController>> {
        self.ui_controller.clone()
    }

    pub fn with_quiz_taking_controller(self) -> Self {
        // Quiz taking controller is already initialized in new()
        self
    }

    pub fn get_quiz_taking_controller(&self) -> Arc<Mutex<QuizTakingController>> {
        self.quiz_taking_controller.clone()
    }
}
