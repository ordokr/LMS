use super::QuizEngine;
use super::ai_generation::{AIGenerationRequest, AIGenerationResult, AISourceType, AIModelType, AIGenerationStatus};
use super::models::{AnswerType, StudyMode, QuizVisibility};
use uuid::Uuid;
use std::error::Error;

impl QuizEngine {
    // AI Generation methods
    
    /// Create a new AI generation request
    pub async fn create_ai_generation_request(
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
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.create_request(
                title,
                description,
                user_id,
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
            ).await
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Process an AI generation request
    pub async fn process_ai_generation_request(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationResult, Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.process_request(request_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Get an AI generation request by ID
    pub async fn get_ai_generation_request(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationRequest, Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.get_request(request_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Get an AI generation result by request ID
    pub async fn get_ai_generation_result(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationResult, Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.get_result_by_request(request_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Get all AI generation requests for a user
    pub async fn get_ai_generation_requests_by_user(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AIGenerationRequest>, Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.get_requests_by_user(user_id, limit, offset).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Cancel an AI generation request
    pub async fn cancel_ai_generation_request(
        &self,
        request_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            ai_generation_service.cancel_request(request_id).await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        } else {
            Err("AI generation service is not available".into())
        }
    }
    
    /// Get available AI model providers
    pub fn get_ai_model_providers(&self) -> Result<Vec<(String, AIModelType)>, Box<dyn Error + Send + Sync>> {
        if let Some(ai_generation_service) = &self.ai_generation_service {
            Ok(ai_generation_service.get_model_providers())
        } else {
            Err("AI generation service is not available".into())
        }
    }
}
