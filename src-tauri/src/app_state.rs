use sqlx::SqlitePool;
use std::sync::Arc;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use crate::database::repositories::quiz_repository::QuizRepository;
use crate::modules::quiz::services::QuizService;

#[derive(Debug)]
pub struct AppState {
    pub db_pool: SqlitePool,
    pub jwt_secret: Vec<u8>,
    pub quiz_repository: Option<Arc<QuizRepository>>,
    pub quiz_service: Option<Arc<QuizService>>,
    pub data_dir: PathBuf,
}

impl AppState {
    pub fn new(db_pool: SqlitePool, jwt_secret: Vec<u8>, data_dir: PathBuf) -> Self {
        Self {
            db_pool,
            jwt_secret,
            quiz_repository: None,
            quiz_service: None,
            data_dir,
        }
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
}
