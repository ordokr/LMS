use std::sync::Arc;
use serde::{Serialize, Deserialize};
use super::storage::HybridQuizStore;
use super::models::{Quiz, Question, Answer};
use super::session::QuizSession;
use crate::core::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandaloneConfig {
    pub storage_path: String,
    pub enable_sync: bool,
    pub offline_mode: bool,
    pub encryption_key: Option<String>,
}

pub struct StandaloneQuizApp {
    pub store: Arc<HybridQuizStore>,
    pub config: StandaloneConfig,
}

impl StandaloneQuizApp {
    pub fn new(standalone_config: StandaloneConfig) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Convert standalone config to app config
        let config = Config {
            database: crate::core::config::DatabaseConfig {
                path: standalone_config.storage_path.clone(),
                max_connections: 10,
                ..Default::default()
            },
            security: crate::core::config::SecurityConfig {
                encryption_key: standalone_config.encryption_key.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        let store = Arc::new(HybridQuizStore::new(&config)?);

        Ok(Self {
            store,
            config: standalone_config,
        })
    }

    pub async fn create_quiz(&self, quiz: Quiz) -> Result<uuid::Uuid, Box<dyn std::error::Error + Send + Sync>> {
        self.store.store_quiz(&quiz).await?;
        Ok(quiz.id)
    }

    pub async fn get_quiz(&self, quiz_id: uuid::Uuid) -> Result<Quiz, Box<dyn std::error::Error + Send + Sync>> {
        self.store.get_quiz(quiz_id).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    pub async fn list_quizzes(&self, limit: usize, offset: usize) -> Result<Vec<Quiz>, Box<dyn std::error::Error + Send + Sync>> {
        self.store.list_quizzes(limit, offset).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }

    pub async fn start_session(&self, quiz_id: uuid::Uuid, user_id: uuid::Uuid) -> Result<QuizSession, Box<dyn std::error::Error + Send + Sync>> {
        let quiz = self.store.get_quiz(quiz_id).await?;
        let session = QuizSession::with_quiz(&quiz, user_id);
        self.store.store_session(&session).await?;
        Ok(session)
    }

    pub async fn submit_answer(&self, session_id: uuid::Uuid, question_id: uuid::Uuid, answer: Answer) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut session = self.store.get_session(session_id).await?;
        let quiz = self.store.get_quiz(session.quiz_id).await?;

        let result = session.submit_answer(question_id, answer, &quiz)?;
        self.store.update_session(&session).await?;

        Ok(result)
    }

    pub async fn complete_session(&self, session_id: uuid::Uuid) -> Result<f32, Box<dyn std::error::Error + Send + Sync>> {
        let mut session = self.store.get_session(session_id).await?;
        let score = session.complete()?;
        self.store.update_session(&session).await?;

        Ok(score)
    }
}