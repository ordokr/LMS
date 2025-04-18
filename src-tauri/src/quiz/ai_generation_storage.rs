use super::ai_generation::{AIGenerationService, AIGenerationRequest, AIGenerationResult, AIGenerationStatus, AISourceType, AIModelType};
use super::models::AnswerType;
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};

impl AIGenerationService {
    /// Store an AI generation request
    pub async fn store_request(
        &self,
        request: &AIGenerationRequest,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert the source type to a string
        let source_type_str = match request.source_type {
            AISourceType::Text => "Text",
            AISourceType::URL => "URL",
            AISourceType::PDF => "PDF",
            AISourceType::Image => "Image",
            AISourceType::Video => "Video",
            AISourceType::Audio => "Audio",
            AISourceType::Custom => "Custom",
        };
        
        // Convert the model type to a string
        let model_type_str = match request.model_type {
            AIModelType::OpenAI => "OpenAI",
            AIModelType::Anthropic => "Anthropic",
            AIModelType::Gemini => "Gemini",
            AIModelType::LocalLLM => "LocalLLM",
            AIModelType::Custom => "Custom",
        };
        
        // Convert the status to a string
        let status_str = match request.status {
            AIGenerationStatus::Pending => "Pending",
            AIGenerationStatus::Processing => "Processing",
            AIGenerationStatus::Completed => "Completed",
            AIGenerationStatus::Failed => "Failed",
            AIGenerationStatus::Cancelled => "Cancelled",
        };
        
        // Convert the question types to a JSON string
        let question_types_json = serde_json::to_string(&request.question_types)?;
        
        // Convert the model parameters to a JSON string if present
        let model_parameters_json = if let Some(params) = &request.model_parameters {
            Some(serde_json::to_string(params)?)
        } else {
            None
        };
        
        // Convert the study mode to a string
        let study_mode_str = request.study_mode.to_string();
        
        // Convert the visibility to a string
        let visibility_str = request.visibility.to_string();
        
        // Check if the request already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_ai_generation_requests
            WHERE id = ?
            "#,
            request.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing request
            sqlx::query!(
                r#"
                UPDATE quiz_ai_generation_requests
                SET user_id = ?, title = ?, description = ?, source_type = ?, source_content = ?,
                    model_type = ?, model_parameters = ?, num_questions = ?, question_types = ?,
                    difficulty_level = ?, topic_focus = ?, language = ?, study_mode = ?,
                    visibility = ?, status = ?, updated_at = ?, completed_at = ?
                WHERE id = ?
                "#,
                request.user_id.map(|id| id.to_string()),
                request.title,
                request.description,
                source_type_str,
                request.source_content,
                model_type_str,
                model_parameters_json,
                request.num_questions,
                question_types_json,
                request.difficulty_level,
                request.topic_focus,
                request.language,
                study_mode_str,
                visibility_str,
                status_str,
                request.updated_at,
                request.completed_at,
                request.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new request
            sqlx::query!(
                r#"
                INSERT INTO quiz_ai_generation_requests
                (id, user_id, title, description, source_type, source_content, model_type,
                 model_parameters, num_questions, question_types, difficulty_level, topic_focus,
                 language, study_mode, visibility, status, created_at, updated_at, completed_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                request.id.to_string(),
                request.user_id.map(|id| id.to_string()),
                request.title,
                request.description,
                source_type_str,
                request.source_content,
                model_type_str,
                model_parameters_json,
                request.num_questions,
                question_types_json,
                request.difficulty_level,
                request.topic_focus,
                request.language,
                study_mode_str,
                visibility_str,
                status_str,
                request.created_at,
                request.updated_at,
                request.completed_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
    
    /// Store an AI generation result
    pub async fn store_result(
        &self,
        result: &AIGenerationResult,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Convert the raw response to a JSON string
        let raw_response_json = serde_json::to_string(&result.raw_response)?;
        
        // Check if the result already exists
        let existing = sqlx::query!(
            r#"
            SELECT id FROM quiz_ai_generation_results
            WHERE id = ?
            "#,
            result.id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if existing.is_some() {
            // Update existing result
            sqlx::query!(
                r#"
                UPDATE quiz_ai_generation_results
                SET quiz_id = ?, raw_response = ?, error_message = ?,
                    processing_time_ms = ?, token_usage = ?
                WHERE id = ?
                "#,
                result.quiz_id.map(|id| id.to_string()),
                raw_response_json,
                result.error_message,
                result.processing_time_ms,
                result.token_usage,
                result.id.to_string()
            )
            .execute(&self.db_pool)
            .await?;
        } else {
            // Insert new result
            sqlx::query!(
                r#"
                INSERT INTO quiz_ai_generation_results
                (id, request_id, quiz_id, raw_response, error_message,
                 processing_time_ms, token_usage, created_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                result.id.to_string(),
                result.request_id.to_string(),
                result.quiz_id.map(|id| id.to_string()),
                raw_response_json,
                result.error_message,
                result.processing_time_ms,
                result.token_usage,
                result.created_at
            )
            .execute(&self.db_pool)
            .await?;
        }
        
        Ok(())
    }
}
