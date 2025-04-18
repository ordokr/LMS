use super::ai_generation::{AIGenerationService, AIGenerationRequest, AIGenerationResult, AIGenerationStatus, AISourceType, AIModelType};
use super::models::AnswerType;
use uuid::Uuid;
use std::error::Error;
use chrono::{DateTime, Utc};

impl AIGenerationService {
    /// Get an AI generation request by ID
    pub async fn get_request(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationRequest, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, title, description, source_type, source_content, model_type,
                   model_parameters, num_questions, question_types, difficulty_level, topic_focus,
                   language, study_mode, visibility, status, created_at, updated_at, completed_at
            FROM quiz_ai_generation_requests
            WHERE id = ?
            "#,
            request_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Parse the source type
            let source_type = match row.source_type.as_str() {
                "Text" => AISourceType::Text,
                "URL" => AISourceType::URL,
                "PDF" => AISourceType::PDF,
                "Image" => AISourceType::Image,
                "Video" => AISourceType::Video,
                "Audio" => AISourceType::Audio,
                _ => AISourceType::Custom,
            };
            
            // Parse the model type
            let model_type = match row.model_type.as_str() {
                "OpenAI" => AIModelType::OpenAI,
                "Anthropic" => AIModelType::Anthropic,
                "Gemini" => AIModelType::Gemini,
                "LocalLLM" => AIModelType::LocalLLM,
                _ => AIModelType::Custom,
            };
            
            // Parse the status
            let status = match row.status.as_str() {
                "Pending" => AIGenerationStatus::Pending,
                "Processing" => AIGenerationStatus::Processing,
                "Completed" => AIGenerationStatus::Completed,
                "Failed" => AIGenerationStatus::Failed,
                "Cancelled" => AIGenerationStatus::Cancelled,
                _ => AIGenerationStatus::Pending,
            };
            
            // Parse the question types
            let question_types: Vec<AnswerType> = serde_json::from_str(&row.question_types)?;
            
            // Parse the model parameters if present
            let model_parameters = if let Some(params_str) = &row.model_parameters {
                Some(serde_json::from_str(params_str)?)
            } else {
                None
            };
            
            // Parse the study mode
            let study_mode = row.study_mode.parse()?;
            
            // Parse the visibility
            let visibility = row.visibility.parse()?;
            
            // Create the request
            let request = AIGenerationRequest {
                id: Uuid::parse_str(&row.id)?,
                user_id: row.user_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                title: row.title,
                description: row.description,
                source_type,
                source_content: row.source_content,
                model_type,
                model_parameters,
                num_questions: row.num_questions,
                question_types,
                difficulty_level: row.difficulty_level,
                topic_focus: row.topic_focus,
                language: row.language,
                study_mode,
                visibility,
                status,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
                completed_at: row.completed_at.map(|dt| dt.parse::<DateTime<Utc>>()).transpose()?,
            };
            
            Ok(request)
        } else {
            Err("Request not found".into())
        }
    }
    
    /// Get an AI generation result by request ID
    pub async fn get_result_by_request(
        &self,
        request_id: Uuid,
    ) -> Result<AIGenerationResult, Box<dyn Error + Send + Sync>> {
        let row = sqlx::query!(
            r#"
            SELECT id, request_id, quiz_id, raw_response, error_message,
                   processing_time_ms, token_usage, created_at
            FROM quiz_ai_generation_results
            WHERE request_id = ?
            "#,
            request_id.to_string()
        )
        .fetch_optional(&self.db_pool)
        .await?;
        
        if let Some(row) = row {
            // Parse the raw response
            let raw_response: serde_json::Value = serde_json::from_str(&row.raw_response)?;
            
            // Create the result
            let result = AIGenerationResult {
                id: Uuid::parse_str(&row.id)?,
                request_id: Uuid::parse_str(&row.request_id)?,
                quiz_id: row.quiz_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                raw_response,
                error_message: row.error_message,
                processing_time_ms: row.processing_time_ms,
                token_usage: row.token_usage,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
            };
            
            Ok(result)
        } else {
            Err("Result not found".into())
        }
    }
    
    /// Get all AI generation requests for a user
    pub async fn get_requests_by_user(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AIGenerationRequest>, Box<dyn Error + Send + Sync>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);
        
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, title, description, source_type, source_content, model_type,
                   model_parameters, num_questions, question_types, difficulty_level, topic_focus,
                   language, study_mode, visibility, status, created_at, updated_at, completed_at
            FROM quiz_ai_generation_requests
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id.to_string(),
            limit,
            offset
        )
        .fetch_all(&self.db_pool)
        .await?;
        
        let mut requests = Vec::new();
        
        for row in rows {
            // Parse the source type
            let source_type = match row.source_type.as_str() {
                "Text" => AISourceType::Text,
                "URL" => AISourceType::URL,
                "PDF" => AISourceType::PDF,
                "Image" => AISourceType::Image,
                "Video" => AISourceType::Video,
                "Audio" => AISourceType::Audio,
                _ => AISourceType::Custom,
            };
            
            // Parse the model type
            let model_type = match row.model_type.as_str() {
                "OpenAI" => AIModelType::OpenAI,
                "Anthropic" => AIModelType::Anthropic,
                "Gemini" => AIModelType::Gemini,
                "LocalLLM" => AIModelType::LocalLLM,
                _ => AIModelType::Custom,
            };
            
            // Parse the status
            let status = match row.status.as_str() {
                "Pending" => AIGenerationStatus::Pending,
                "Processing" => AIGenerationStatus::Processing,
                "Completed" => AIGenerationStatus::Completed,
                "Failed" => AIGenerationStatus::Failed,
                "Cancelled" => AIGenerationStatus::Cancelled,
                _ => AIGenerationStatus::Pending,
            };
            
            // Parse the question types
            let question_types: Vec<AnswerType> = serde_json::from_str(&row.question_types)?;
            
            // Parse the model parameters if present
            let model_parameters = if let Some(params_str) = &row.model_parameters {
                Some(serde_json::from_str(params_str)?)
            } else {
                None
            };
            
            // Parse the study mode
            let study_mode = row.study_mode.parse()?;
            
            // Parse the visibility
            let visibility = row.visibility.parse()?;
            
            // Create the request
            let request = AIGenerationRequest {
                id: Uuid::parse_str(&row.id)?,
                user_id: row.user_id.map(|id| Uuid::parse_str(&id)).transpose()?,
                title: row.title,
                description: row.description,
                source_type,
                source_content: row.source_content,
                model_type,
                model_parameters,
                num_questions: row.num_questions,
                question_types,
                difficulty_level: row.difficulty_level,
                topic_focus: row.topic_focus,
                language: row.language,
                study_mode,
                visibility,
                status,
                created_at: row.created_at.parse::<DateTime<Utc>>()?,
                updated_at: row.updated_at.parse::<DateTime<Utc>>()?,
                completed_at: row.completed_at.map(|dt| dt.parse::<DateTime<Utc>>()).transpose()?,
            };
            
            requests.push(request);
        }
        
        Ok(requests)
    }
    
    /// Cancel an AI generation request
    pub async fn cancel_request(
        &self,
        request_id: Uuid,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get the request
        let mut request = self.get_request(request_id).await?;
        
        // Check if the request can be cancelled
        if request.status != AIGenerationStatus::Pending && request.status != AIGenerationStatus::Processing {
            return Err(format!("Request cannot be cancelled in {} state", request.status.to_string()).into());
        }
        
        // Update the request status to Cancelled
        request.status = AIGenerationStatus::Cancelled;
        request.updated_at = Utc::now();
        self.store_request(&request).await?;
        
        Ok(())
    }
}
