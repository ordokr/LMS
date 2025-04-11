use crate::shared::logger::Logger;
use crate::api::canvas_client::CanvasApi;
use crate::api::discourse_client::DiscourseApi;
use crate::services::integration::sync_service::SyncService;
use crate::services::integration::model_mapper::ModelMapper;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Error types for the API integration
#[derive(Error, Debug)]
pub enum IntegrationError {
    #[error("Canvas API error: {0}")]
    CanvasApiError(String),
    
    #[error("Discourse API error: {0}")]
    DiscourseApiError(String),
    
    #[error("Mapping error: {0}")]
    MappingError(String),
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("Transformation error: {0}")]
    TransformationError(String),
    
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
}

/// Integrated entity response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedEntity<T, U> {
    pub canvas: T,
    pub discourse: U,
    pub integrated: bool,
}

/// Canvas-Discourse API Integration
///
/// This module provides unified API operations that bridge Canvas and Discourse systems.
/// It handles the transformation of data models between systems and ensures consistency.
pub struct ApiIntegration {
    logger: Arc<Logger>,
    canvas_api: Arc<CanvasApi>,
    discourse_api: Arc<DiscourseApi>,
    sync_service: Arc<SyncService>,
    model_mapper: Arc<ModelMapper>,
}

impl ApiIntegration {
    /// Create a new API integration instance
    pub fn new(
        logger: Arc<Logger>,
        canvas_api: Arc<CanvasApi>,
        discourse_api: Arc<DiscourseApi>,
        sync_service: Arc<SyncService>,
        model_mapper: Arc<ModelMapper>,
    ) -> Self {
        ApiIntegration {
            logger,
            canvas_api,
            discourse_api,
            sync_service,
            model_mapper,
        }
    }
    
    /// Initialize the API integration
    pub async fn initialize(&self) -> Result<bool, IntegrationError> {
        self.logger.info("Initializing API integration service");
        Ok(true)
    }
    
    /// Create a user in both systems with proper linkage
    ///
    /// # Arguments
    ///
    /// * `user_data` - User data (Canvas format)
    ///
    /// # Returns
    ///
    /// * `Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError>` - Created user with IDs from both systems
    pub async fn create_user(&self, user_data: &serde_json::Value) -> Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError> {
        self.logger.info(&format!("Creating integrated user: {}", user_data["name"].as_str().unwrap_or("unknown")));
        
        // Step 1: Create user in Canvas
        let canvas_user = self.canvas_api.create_user(user_data)
            .await
            .map_err(|e| IntegrationError::CanvasApiError(e.to_string()))?;
        
        // Step 2: Transform to Discourse format
        let discourse_user_data = self.model_mapper.canvas_to_discourse_user(&canvas_user)
            .map_err(|e| IntegrationError::TransformationError(e.to_string()))?;
        
        // Step 3: Create user in Discourse
        let discourse_user = self.discourse_api.create_user(&discourse_user_data)
            .await
            .map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))?;
        
        // Step 4: Record the mapping between the two users
        let canvas_id = canvas_user["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Canvas user ID not found".to_string()))?;
            
        let discourse_id = discourse_user["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Discourse user ID not found".to_string()))?;
            
        self.model_mapper.save_mapping("user", canvas_id, discourse_id, None)
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 5: Return the integrated user object
        Ok(IntegratedEntity {
            canvas: canvas_user,
            discourse: discourse_user,
            integrated: true,
        })
    }
    
    /// Update a user in both systems
    ///
    /// # Arguments
    ///
    /// * `canvas_user_id` - Canvas user ID
    /// * `user_data` - Updated user data
    ///
    /// # Returns
    ///
    /// * `Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError>` - Updated user with data from both systems
    pub async fn update_user(&self, canvas_user_id: &str, user_data: &serde_json::Value) -> Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError> {
        self.logger.info(&format!("Updating integrated user: {}", canvas_user_id));
        
        // Step 1: Get the mapping to find Discourse user ID
        let mapping = self.model_mapper.get_mapping("user", canvas_user_id, Some("canvas"))
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 2: Update user in Canvas
        let canvas_user = self.canvas_api.update_user(canvas_user_id, user_data)
            .await
            .map_err(|e| IntegrationError::CanvasApiError(e.to_string()))?;
        
        // Step 3: Transform to Discourse format
        let discourse_user_data = self.model_mapper.canvas_to_discourse_user(&canvas_user)
            .map_err(|e| IntegrationError::TransformationError(e.to_string()))?;
        
        // Step 4: Update user in Discourse
        let discourse_user = self.discourse_api.update_user(&mapping.target_id, &discourse_user_data)
            .await
            .map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))?;
        
        // Step 5: Return the integrated user object
        Ok(IntegratedEntity {
            canvas: canvas_user,
            discourse: discourse_user,
            integrated: true,
        })
    }
    
    /// Create a course in Canvas and corresponding category in Discourse
    ///
    /// # Arguments
    ///
    /// * `course_data` - Course data (Canvas format)
    ///
    /// # Returns
    ///
    /// * `Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError>` - Created course/category with IDs from both systems
    pub async fn create_course(&self, course_data: &serde_json::Value) -> Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError> {
        self.logger.info(&format!("Creating integrated course: {}", course_data["name"].as_str().unwrap_or("unknown")));
        
        // Step 1: Create course in Canvas
        let canvas_course = self.canvas_api.create_course(course_data)
            .await
            .map_err(|e| IntegrationError::CanvasApiError(e.to_string()))?;
        
        // Step 2: Transform to Discourse category format
        let discourse_category_data = self.model_mapper.canvas_to_discourse_category(&canvas_course)
            .map_err(|e| IntegrationError::TransformationError(e.to_string()))?;
        
        // Step 3: Create category in Discourse
        let discourse_category = self.discourse_api.create_category(&discourse_category_data)
            .await
            .map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))?;
        
        // Step 4: Record the mapping between the course and category
        let canvas_id = canvas_course["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Canvas course ID not found".to_string()))?;
            
        let discourse_id = discourse_category["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Discourse category ID not found".to_string()))?;
            
        self.model_mapper.save_mapping("course", canvas_id, discourse_id, None)
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 5: Return the integrated course object
        Ok(IntegratedEntity {
            canvas: canvas_course,
            discourse: discourse_category,
            integrated: true,
        })
    }
    
    /// Update a course in Canvas and corresponding category in Discourse
    ///
    /// # Arguments
    ///
    /// * `canvas_course_id` - Canvas course ID
    /// * `course_data` - Updated course data
    ///
    /// # Returns
    ///
    /// * `Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError>` - Updated course/category with data from both systems
    pub async fn update_course(&self, canvas_course_id: &str, course_data: &serde_json::Value) -> Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError> {
        self.logger.info(&format!("Updating integrated course: {}", canvas_course_id));
        
        // Step 1: Get the mapping to find Discourse category ID
        let mapping = self.model_mapper.get_mapping("course", canvas_course_id, Some("canvas"))
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 2: Update course in Canvas
        let canvas_course = self.canvas_api.update_course(canvas_course_id, course_data)
            .await
            .map_err(|e| IntegrationError::CanvasApiError(e.to_string()))?;
        
        // Step 3: Transform to Discourse category format
        let discourse_category_data = self.model_mapper.canvas_to_discourse_category(&canvas_course)
            .map_err(|e| IntegrationError::TransformationError(e.to_string()))?;
        
        // Step 4: Update category in Discourse
        let discourse_category = self.discourse_api.update_category(&mapping.target_id, &discourse_category_data)
            .await
            .map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))?;
        
        // Step 5: Return the integrated course object
        Ok(IntegratedEntity {
            canvas: canvas_course,
            discourse: discourse_category,
            integrated: true,
        })
    }
    
    /// Create a discussion in Canvas and corresponding topic in Discourse
    ///
    /// # Arguments
    ///
    /// * `course_id` - Canvas course ID
    /// * `discussion_data` - Discussion data (Canvas format)
    ///
    /// # Returns
    ///
    /// * `Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError>` - Created discussion/topic with IDs from both systems
    pub async fn create_discussion(&self, course_id: &str, discussion_data: &serde_json::Value) -> Result<IntegratedEntity<serde_json::Value, serde_json::Value>, IntegrationError> {
        self.logger.info(&format!("Creating integrated discussion: {}", discussion_data["title"].as_str().unwrap_or("unknown")));
        
        // Step 1: Get the mapping to find Discourse category ID
        let course_mapping = self.model_mapper.get_mapping("course", course_id, Some("canvas"))
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 2: Create discussion in Canvas
        let canvas_discussion = self.canvas_api.create_discussion(course_id, discussion_data)
            .await
            .map_err(|e| IntegrationError::CanvasApiError(e.to_string()))?;
        
        // Step 3: Transform to Discourse topic format
        let mut discourse_topic_data = self.model_mapper.canvas_to_discourse_topic(&canvas_discussion)
            .map_err(|e| IntegrationError::TransformationError(e.to_string()))?;
        
        // Add the category ID to the topic data
        discourse_topic_data["category_id"] = serde_json::Value::String(course_mapping.target_id.clone());
        
        // Step 4: Create topic in Discourse
        let discourse_topic = self.discourse_api.create_topic(&discourse_topic_data)
            .await
            .map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))?;
        
        // Step 5: Record the mapping between the discussion and topic
        let canvas_id = canvas_discussion["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Canvas discussion ID not found".to_string()))?;
            
        let discourse_id = discourse_topic["id"].as_str()
            .ok_or_else(|| IntegrationError::TransformationError("Discourse topic ID not found".to_string()))?;
            
        self.model_mapper.save_mapping("discussion", canvas_id, discourse_id, None)
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 6: Return the integrated discussion object
        Ok(IntegratedEntity {
            canvas: canvas_discussion,
            discourse: discourse_topic,
            integrated: true,
        })
    }
    
    /// Delete an entity from both systems
    ///
    /// # Arguments
    ///
    /// * `entity_type` - Type of entity (user, course, discussion)
    /// * `canvas_id` - Canvas entity ID
    ///
    /// # Returns
    ///
    /// * `Result<bool, IntegrationError>` - True if deletion was successful
    pub async fn delete_entity(&self, entity_type: &str, canvas_id: &str) -> Result<bool, IntegrationError> {
        self.logger.info(&format!("Deleting integrated {}: {}", entity_type, canvas_id));
        
        // Step 1: Get the mapping to find Discourse entity ID
        let mapping = match self.model_mapper.get_mapping(entity_type, canvas_id, Some("canvas")).await {
            Ok(m) => m,
            Err(e) => {
                self.logger.warn(&format!("No mapping found for {} {}: {}", entity_type, canvas_id, e));
                // If no mapping exists, just try to delete from Canvas
                match self.delete_from_canvas(entity_type, canvas_id).await {
                    Ok(_) => return Ok(true),
                    Err(e) => return Err(e),
                }
            }
        };
        
        // Step 2: Delete from both systems
        let canvas_result = self.delete_from_canvas(entity_type, canvas_id).await;
        let discourse_result = self.delete_from_discourse(entity_type, &mapping.target_id).await;
        
        // Step 3: Delete the mapping
        self.model_mapper.delete_mapping(entity_type, canvas_id, Some("canvas"))
            .await
            .map_err(|e| IntegrationError::MappingError(e.to_string()))?;
        
        // Step 4: Return success if at least one deletion succeeded
        match (canvas_result, discourse_result) {
            (Ok(_), Ok(_)) => Ok(true),
            (Ok(_), Err(e)) => {
                self.logger.warn(&format!("Partial deletion: Canvas succeeded but Discourse failed: {}", e));
                Ok(true)
            },
            (Err(e), Ok(_)) => {
                self.logger.warn(&format!("Partial deletion: Discourse succeeded but Canvas failed: {}", e));
                Ok(true)
            },
            (Err(e1), Err(e2)) => {
                Err(IntegrationError::IntegrationError(format!(
                    "Failed to delete from both systems. Canvas: {}. Discourse: {}", 
                    e1, e2
                )))
            }
        }
    }
    
    // Helper method to delete from Canvas
    async fn delete_from_canvas(&self, entity_type: &str, entity_id: &str) -> Result<bool, IntegrationError> {
        match entity_type {
            "user" => self.canvas_api.delete_user(entity_id).await,
            "course" => self.canvas_api.delete_course(entity_id).await,
            "discussion" => self.canvas_api.delete_discussion(entity_id).await,
            _ => return Err(IntegrationError::EntityNotFound(format!("Unknown entity type: {}", entity_type))),
        }.map_err(|e| IntegrationError::CanvasApiError(e.to_string()))
    }
    
    // Helper method to delete from Discourse
    async fn delete_from_discourse(&self, entity_type: &str, entity_id: &str) -> Result<bool, IntegrationError> {
        match entity_type {
            "user" => self.discourse_api.delete_user(entity_id).await,
            "course" => self.discourse_api.delete_category(entity_id).await,
            "discussion" => self.discourse_api.delete_topic(entity_id).await,
            _ => return Err(IntegrationError::EntityNotFound(format!("Unknown entity type: {}", entity_type))),
        }.map_err(|e| IntegrationError::DiscourseApiError(e.to_string()))
    }
}
