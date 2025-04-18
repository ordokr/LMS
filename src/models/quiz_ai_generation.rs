use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::quiz::{AnswerType, StudyMode, QuizVisibility};

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

impl AIGenerationRequest {
    pub fn new(
        title: String,
        source_type: AISourceType,
        source_content: String,
        model_type: AIModelType,
        num_questions: i32,
        question_types: Vec<AnswerType>,
        difficulty_level: i32,
        language: String,
        study_mode: StudyMode,
        visibility: QuizVisibility,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: None,
            title,
            description: None,
            source_type,
            source_content,
            model_type,
            model_parameters: None,
            num_questions,
            question_types,
            difficulty_level,
            topic_focus: None,
            language,
            study_mode,
            visibility,
            status: AIGenerationStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
        }
    }
    
    pub fn with_user(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
    
    pub fn with_model_parameters(mut self, parameters: serde_json::Value) -> Self {
        self.model_parameters = Some(parameters);
        self
    }
    
    pub fn with_topic_focus(mut self, topic: String) -> Self {
        self.topic_focus = Some(topic);
        self
    }
    
    pub fn mark_as_processing(&mut self) {
        self.status = AIGenerationStatus::Processing;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_as_completed(&mut self) {
        self.status = AIGenerationStatus::Completed;
        self.updated_at = Utc::now();
        self.completed_at = Some(Utc::now());
    }
    
    pub fn mark_as_failed(&mut self) {
        self.status = AIGenerationStatus::Failed;
        self.updated_at = Utc::now();
    }
    
    pub fn mark_as_cancelled(&mut self) {
        self.status = AIGenerationStatus::Cancelled;
        self.updated_at = Utc::now();
    }
}

impl AIGenerationResult {
    pub fn new(
        request_id: Uuid,
        raw_response: serde_json::Value,
        processing_time_ms: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            request_id,
            quiz_id: None,
            raw_response,
            error_message: None,
            processing_time_ms,
            token_usage: None,
            created_at: Utc::now(),
        }
    }
    
    pub fn with_quiz(mut self, quiz_id: Uuid) -> Self {
        self.quiz_id = Some(quiz_id);
        self
    }
    
    pub fn with_error(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }
    
    pub fn with_token_usage(mut self, token_usage: i32) -> Self {
        self.token_usage = Some(token_usage);
        self
    }
}
