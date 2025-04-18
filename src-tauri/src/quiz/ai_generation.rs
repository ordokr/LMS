use super::models::{Quiz, Question, Answer, AnswerType, StudyMode, QuizVisibility};
use super::storage::HybridQuizStore;
use uuid::Uuid;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;

/// AI generation source type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AISourceType {
    Text,
    URL,
    PDF,
    Image,
    Video,
    Audio,
    Custom,
}

/// AI generation model type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIModelType {
    OpenAI,
    Anthropic,
    Gemini,
    LocalLLM,
    Custom,
}

/// AI generation request model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationRequest {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub source_type: AISourceType,
    pub source_content: String,
    pub model_type: AIModelType,
    pub model_parameters: Option<serde_json::Value>,
    pub num_questions: i32,
    pub question_types: Vec<AnswerType>,
    pub difficulty_level: i32, // 1-5 scale
    pub topic_focus: Option<String>,
    pub language: String,
    pub study_mode: StudyMode,
    pub visibility: QuizVisibility,
    pub status: AIGenerationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// AI generation status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AIGenerationStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

/// AI generation result model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerationResult {
    pub id: Uuid,
    pub request_id: Uuid,
    pub quiz_id: Option<Uuid>,
    pub raw_response: serde_json::Value,
    pub error_message: Option<String>,
    pub processing_time_ms: i64,
    pub token_usage: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// AI model provider trait
pub trait AIModelProvider: Send + Sync {
    fn generate_quiz(&self, request: &AIGenerationRequest) -> Result<serde_json::Value, Box<dyn Error + Send + Sync>>;
    fn get_name(&self) -> String;
    fn get_type(&self) -> AIModelType;
}

/// AI generation service
pub struct AIGenerationService {
    db_pool: SqlitePool,
    quiz_store: Arc<HybridQuizStore>,
    model_providers: Vec<Box<dyn AIModelProvider>>,
}

impl AIGenerationService {
    pub fn new(db_pool: SqlitePool, quiz_store: Arc<HybridQuizStore>) -> Self {
        Self {
            db_pool,
            quiz_store,
            model_providers: Vec::new(),
        }
    }

    /// Register an AI model provider
    pub fn register_model_provider(&mut self, provider: Box<dyn AIModelProvider>) {
        self.model_providers.push(provider);
    }

    /// Get all registered model providers
    pub fn get_model_providers(&self) -> Vec<(String, AIModelType)> {
        self.model_providers
            .iter()
            .map(|provider| (provider.get_name(), provider.get_type()))
            .collect()
    }

    /// Create a new AI generation request
    pub async fn create_request(
        &self,
        title: String,
        description: Option<String>,
        user_id: Option<Uuid>,
        source_type: AISourceType,
        source_content: String,
        model_type: AIModelType,
        model_parameters: Option<serde_json::Value>,
        num_questions: i32,
        question_types: Vec<AnswerType>,
        difficulty_level: i32,
        topic_focus: Option<String>,
        language: String,
        study_mode: StudyMode,
        visibility: QuizVisibility,
    ) -> Result<AIGenerationRequest, Box<dyn Error + Send + Sync>> {
        // Create a new request
        let request = AIGenerationRequest {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            source_type,
            source_content,
            model_type,
            model_parameters,
            num_questions,
            question_types,
            difficulty_level,
            topic_focus,
            language,
            study_mode,
            visibility,
            status: AIGenerationStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        };

        // Store the request
        self.store_request(&request).await?;

        Ok(request)
    }

    /// Process an AI generation request
    pub async fn process_request(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationResult, Box<dyn Error + Send + Sync>> {
        // Get the request
        let mut request = self.get_request(request_id).await?;

        // Check if the request is already being processed or completed
        if request.status != AIGenerationStatus::Pending {
            return Err(format!("Request is already in {} state", request.status.to_string()).into());
        }

        // Update the request status to Processing
        request.status = AIGenerationStatus::Processing;
        request.updated_at = Utc::now();
        self.store_request(&request).await?;

        // Find the appropriate model provider
        let provider = self.model_providers.iter()
            .find(|p| p.get_type() == request.model_type)
            .ok_or_else(|| format!("No provider found for model type: {:?}", request.model_type))?;

        // Start timing
        let start_time = std::time::Instant::now();

        // Generate the quiz
        let generation_result = match provider.generate_quiz(&request) {
            Ok(response) => {
                // Calculate processing time
                let processing_time = start_time.elapsed().as_millis() as i64;

                // Create the result
                let result = AIGenerationResult {
                    id: Uuid::new_v4(),
                    request_id: request.id,
                    quiz_id: None, // Will be set after creating the quiz
                    raw_response: response.clone(),
                    error_message: None,
                    processing_time_ms: processing_time,
                    token_usage: None, // Could be extracted from the response if available
                    created_at: Utc::now(),
                };

                // Update the request status to Completed
                request.status = AIGenerationStatus::Completed;
                request.updated_at = Utc::now();
                request.completed_at = Some(Utc::now());
                self.store_request(&request).await?;

                // Try to create a quiz from the response
                match self.create_quiz_from_response(&request, &response).await {
                    Ok(quiz_id) => {
                        // Update the result with the quiz ID
                        let mut result_with_quiz = result;
                        result_with_quiz.quiz_id = Some(quiz_id);
                        self.store_result(&result_with_quiz).await?;
                        result_with_quiz
                    },
                    Err(e) => {
                        // Store the result without a quiz ID
                        self.store_result(&result).await?;

                        // Log the error but don't fail the request
                        eprintln!("Failed to create quiz from response: {}", e);
                        result
                    }
                }
            },
            Err(e) => {
                // Calculate processing time
                let processing_time = start_time.elapsed().as_millis() as i64;

                // Create the result with error
                let result = AIGenerationResult {
                    id: Uuid::new_v4(),
                    request_id: request.id,
                    quiz_id: None,
                    raw_response: serde_json::json!({"error": e.to_string()}),
                    error_message: Some(e.to_string()),
                    processing_time_ms: processing_time,
                    token_usage: None,
                    created_at: Utc::now(),
                };

                // Update the request status to Failed
                request.status = AIGenerationStatus::Failed;
                request.updated_at = Utc::now();
                self.store_request(&request).await?;

                // Store the result
                self.store_result(&result).await?;

                result
            }
        };

        Ok(generation_result)
    }
}
