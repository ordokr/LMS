use crate::models::model_factory::ModelFactory;
use crate::api::canvas_api::CanvasApi;
use crate::api::discourse_api::DiscourseApi;
use crate::utils::naming_conventions::convert_object_keys;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use thiserror::Error;
use std::sync::Arc;

/// Error types for the model sync service
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Failed to sync user {0} from {1}: {2}")]
    UserSyncError(String, String, String),
    
    #[error("Failed to sync course {0} from {1}: {2}")]
    CourseSyncError(String, String, String),
    
    #[error("Failed to sync discussion {0} from {1}: {2}")]
    DiscussionSyncError(String, String, String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Model conversion error: {0}")]
    ModelConversionError(String),
}

/// Source system for synchronization
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Source {
    Canvas,
    Discourse,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Canvas => "canvas",
            Source::Discourse => "discourse",
        }
    }
    
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "canvas" => Source::Canvas,
            "discourse" => Source::Discourse,
            _ => Source::Canvas, // Default to Canvas
        }
    }
}

/// Service for synchronizing models between Canvas and Discourse
pub struct ModelSyncService {
    model_factory: Arc<ModelFactory>,
    canvas_api: Arc<CanvasApi>,
    discourse_api: Arc<DiscourseApi>,
}

impl ModelSyncService {
    /// Create a new model sync service
    pub fn new(
        model_factory: Arc<ModelFactory>,
        canvas_api: Arc<CanvasApi>,
        discourse_api: Arc<DiscourseApi>,
    ) -> Self {
        ModelSyncService {
            model_factory,
            canvas_api,
            discourse_api,
        }
    }
    
    /// Synchronize a user between Canvas and Discourse
    pub async fn sync_user(
        &self,
        user_id: &str,
        source: Option<Source>,
    ) -> Result<Value, SyncError> {
        let source = source.unwrap_or(Source::Canvas);
        
        match source {
            Source::Canvas => {
                // Fetch from Canvas
                let user_data = self.canvas_api.get_user(user_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_user = self.model_factory.create("user", &user_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Discourse
                let discourse_data = self.model_factory.convert_to_source(&unified_user, "discourse")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                self.discourse_api.create_or_update_user(&discourse_data).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Update last sync time
                unified_user["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_user)
            },
            Source::Discourse => {
                // Fetch from Discourse
                let user_data = self.discourse_api.get_user(user_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_user = self.model_factory.create("user", &user_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Canvas
                let canvas_data = self.model_factory.convert_to_source(&unified_user, "canvas")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                self.canvas_api.create_or_update_user(&canvas_data).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Update last sync time
                unified_user["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_user)
            }
        }
    }
    
    /// Synchronize a course between Canvas and Discourse
    pub async fn sync_course(
        &self,
        course_id: &str,
        source: Option<Source>,
    ) -> Result<Value, SyncError> {
        let source = source.unwrap_or(Source::Canvas);
        
        match source {
            Source::Canvas => {
                // Fetch from Canvas
                let course_data = self.canvas_api.get_course(course_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_course = self.model_factory.create("course", &course_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Discourse
                let discourse_data = self.model_factory.convert_to_source(&unified_course, "discourse")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                let category_result = self.discourse_api.create_or_update_category(&discourse_data).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Store mapping between Canvas course and Discourse category
                unified_course["discourseId"] = category_result["id"].clone();
                
                // Update last sync time
                unified_course["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_course)
            },
            Source::Discourse => {
                // Fetch from Discourse
                let category_data = self.discourse_api.get_category(course_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_course = self.model_factory.create("course", &category_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Canvas
                let canvas_data = self.model_factory.convert_to_source(&unified_course, "canvas")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                let course_result = self.canvas_api.create_or_update_course(&canvas_data).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Store mapping between Discourse category and Canvas course
                unified_course["canvasId"] = course_result["id"].clone();
                
                // Update last sync time
                unified_course["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_course)
            }
        }
    }
    
    /// Synchronize a discussion between Canvas and Discourse
    pub async fn sync_discussion(
        &self,
        discussion_id: &str,
        course_id: &str,
        source: Option<Source>,
    ) -> Result<Value, SyncError> {
        let source = source.unwrap_or(Source::Canvas);
        
        match source {
            Source::Canvas => {
                // Implementation would fetch discussion data from Canvas,
                // convert to a unified model, then push to Discourse
                // This is a simplified version
                let discussion_data = self.canvas_api.get_discussion(discussion_id, course_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_discussion = self.model_factory.create("discussion", &discussion_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Discourse
                let discourse_data = self.model_factory.convert_to_source(&unified_discussion, "discourse")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                let topic_result = self.discourse_api.create_or_update_topic(&discourse_data).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Store mapping between Canvas discussion and Discourse topic
                unified_discussion["discourseId"] = topic_result["id"].clone();
                
                // Update last sync time
                unified_discussion["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_discussion)
            },
            Source::Discourse => {
                // Implementation would fetch topic data from Discourse,
                // convert to a unified model, then push to Canvas
                // This is a simplified version
                let topic_data = self.discourse_api.get_topic(discussion_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Create unified model
                let mut unified_discussion = self.model_factory.create("discussion", &topic_data, source.as_str())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                // Convert and send to Canvas
                let canvas_data = self.model_factory.convert_to_source(&unified_discussion, "canvas")
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                let discussion_result = self.canvas_api.create_or_update_discussion(&canvas_data, course_id).await
                    .map_err(|e| SyncError::ApiError(e.to_string()))?;
                
                // Store mapping between Discourse topic and Canvas discussion
                unified_discussion["canvasId"] = discussion_result["id"].clone();
                
                // Update last sync time
                unified_discussion["lastSync"] = serde_json::to_value(Utc::now().to_rfc3339())
                    .map_err(|e| SyncError::ModelConversionError(e.to_string()))?;
                
                Ok(unified_discussion)
            }
        }
    }
}
