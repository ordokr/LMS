use crate::api::canvas_client::{CanvasClient, CanvasApiError};
use crate::api::discourse_client::{DiscourseClient, DiscourseApiError};
use crate::models::unified::{User, Course, Discussion};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use thiserror::Error;

/// Source system enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SourceSystem {
    Canvas,
    Discourse,
}

impl SourceSystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            SourceSystem::Canvas => "canvas",
            SourceSystem::Discourse => "discourse",
        }
    }
}

impl From<&str> for SourceSystem {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "canvas" => SourceSystem::Canvas,
            "discourse" => SourceSystem::Discourse,
            _ => SourceSystem::Canvas, // Default to Canvas
        }
    }
}

/// Error type for model synchronization operations
#[derive(Debug, Error)]
pub enum ModelSyncError {
    #[error("Canvas API error: {0}")]
    CanvasError(#[from] CanvasApiError),
    
    #[error("Discourse API error: {0}")]
    DiscourseError(#[from] DiscourseApiError),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Sync error: {0}")]
    SyncError(String),
}

/// Type alias for result with ModelSyncError
pub type Result<T> = std::result::Result<T, ModelSyncError>;

/// Service for synchronizing models between Canvas and Discourse
#[derive(Debug)]
pub struct ModelSyncService {
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
}

impl ModelSyncService {
    /// Create a new ModelSyncService
    pub fn new(canvas_client: Arc<CanvasClient>, discourse_client: Arc<DiscourseClient>) -> Self {
        Self {
            canvas_client,
            discourse_client,
        }
    }
    
    /// Synchronize a user between Canvas and Discourse
    ///
    /// # Arguments
    /// * `user_id` - User ID in source system
    /// * `source` - Source system (Canvas or Discourse)
    ///
    /// # Returns
    /// Unified user model after synchronization
    pub async fn sync_user(
        &self,
        user_id: &str,
        source: SourceSystem,
    ) -> Result<User> {
        let mut unified_user = match source {
            SourceSystem::Canvas => {
                // In a real implementation, we would:
                // 1. Fetch user data from Canvas
                // 2. Convert to a unified User model
                // 3. Send to Discourse
                // 4. Store the mapping
                
                // For now, create a stub implementation
                let canvas_user = serde_json::json!({
                    "id": user_id,
                    "name": "Test User",
                    "email": "test@example.com",
                });
                
                self.create_user_from_canvas(canvas_user)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create user from Canvas: {}", e)))?
            },
            SourceSystem::Discourse => {
                // Similar process for Discourse
                let discourse_user = serde_json::json!({
                    "id": user_id,
                    "username": "test_user",
                    "name": "Test User",
                    "email": "test@example.com",
                });
                
                self.create_user_from_discourse(discourse_user)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create user from Discourse: {}", e)))?
            },
        };
        
        // Update last sync time
        unified_user.update_sync_time();
        
        Ok(unified_user)
    }
    
    /// Synchronize a course between Canvas and Discourse
    ///
    /// # Arguments
    /// * `course_id` - Course ID in source system
    /// * `source` - Source system (Canvas or Discourse)
    ///
    /// # Returns
    /// Unified course model after synchronization
    pub async fn sync_course(
        &self,
        course_id: &str,
        source: SourceSystem,
    ) -> Result<Course> {
        let mut unified_course = match source {
            SourceSystem::Canvas => {
                // In a real implementation, we would:
                // 1. Fetch course data from Canvas
                // 2. Convert to a unified Course model
                // 3. Create or update a corresponding Discourse category
                // 4. Store the mapping
                
                // For now, create a stub implementation
                let canvas_course = serde_json::json!({
                    "id": course_id,
                    "name": "Test Course",
                    "course_code": "TST101",
                    "description": "Test course description",
                });
                
                self.create_course_from_canvas(canvas_course)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create course from Canvas: {}", e)))?
            },
            SourceSystem::Discourse => {
                // Similar process for Discourse
                let discourse_category = serde_json::json!({
                    "id": course_id,
                    "name": "Test Category",
                    "slug": "test-category",
                    "description": "Test category description",
                });
                
                self.create_course_from_discourse(discourse_category)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create course from Discourse: {}", e)))?
            },
        };
        
        // Update last sync time
        unified_course.update_sync_time();
        
        Ok(unified_course)
    }
    
    /// Synchronize a discussion between Canvas and Discourse
    ///
    /// # Arguments
    /// * `discussion_id` - Discussion ID in source system
    /// * `course_id` - Course/Category ID
    /// * `source` - Source system (Canvas or Discourse)
    ///
    /// # Returns
    /// Unified discussion model after synchronization
    pub async fn sync_discussion(
        &self,
        discussion_id: &str,
        course_id: &str,
        source: SourceSystem,
    ) -> Result<Discussion> {
        let mut unified_discussion = match source {
            SourceSystem::Canvas => {
                // In a real implementation, we would:
                // 1. Fetch discussion data from Canvas
                // 2. Convert to a unified Discussion model
                // 3. Create or update a corresponding Discourse topic
                // 4. Store the mapping
                
                // For now, create a stub implementation
                let canvas_discussion = serde_json::json!({
                    "id": discussion_id,
                    "title": "Test Discussion",
                    "message": "Test discussion content",
                    "course_id": course_id,
                });
                
                self.create_discussion_from_canvas(canvas_discussion)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create discussion from Canvas: {}", e)))?
            },
            SourceSystem::Discourse => {
                // Similar process for Discourse
                let discourse_topic = serde_json::json!({
                    "id": discussion_id,
                    "title": "Test Topic",
                    "raw": "Test topic content",
                    "category_id": course_id,
                });
                
                self.create_discussion_from_discourse(discourse_topic)
                    .map_err(|e| ModelSyncError::SyncError(format!("Failed to create discussion from Discourse: {}", e)))?
            },
        };
        
        // Update last sync time
        unified_discussion.update_sync_time();
        
        Ok(unified_discussion)
    }
    
    // Helper functions to convert data from different sources to unified models
    
    fn create_user_from_canvas(&self, canvas_user: serde_json::Value) -> std::result::Result<User, String> {
        // In a real implementation, we would use the structured data from Canvas
        // to build a unified User model with all required fields
        
        let id = canvas_user["id"].as_str()
            .ok_or_else(|| "Missing ID in Canvas user".to_string())?
            .to_string();
            
        let name = canvas_user["name"].as_str()
            .ok_or_else(|| "Missing name in Canvas user".to_string())?
            .to_string();
            
        let email = canvas_user["email"].as_str()
            .ok_or_else(|| "Missing email in Canvas user".to_string())?
            .to_string();
            
        let user = User::new(
            Some(id),
            Some(name),
            Some(email),
            None, // username
            None, // avatar
            None, // canvas_id (would actually store this in a real implementation)
            None, // discourse_id
            None, // last_login
            Some("canvas".to_string()),
            None, // roles
            None, // metadata
        );
        
        Ok(user)
    }
    
    fn create_user_from_discourse(&self, discourse_user: serde_json::Value) -> std::result::Result<User, String> {
        // Similar to Canvas, but extracting from Discourse JSON
        
        let id = discourse_user["id"].as_str()
            .ok_or_else(|| "Missing ID in Discourse user".to_string())?
            .to_string();
            
        let name = discourse_user["name"].as_str()
            .ok_or_else(|| "Missing name in Discourse user".to_string())?
            .to_string();
            
        let email = discourse_user["email"].as_str()
            .ok_or_else(|| "Missing email in Discourse user".to_string())?
            .to_string();
            
        let username = discourse_user["username"].as_str()
            .ok_or_else(|| "Missing username in Discourse user".to_string())?
            .to_string();
            
        let user = User::new(
            Some(id),
            Some(name),
            Some(email),
            Some(username),
            None, // avatar
            None, // canvas_id
            None, // discourse_id (would actually store this in a real implementation)
            None, // last_login
            Some("discourse".to_string()),
            None, // roles
            None, // metadata
        );
        
        Ok(user)
    }
    
    fn create_course_from_canvas(&self, canvas_course: serde_json::Value) -> std::result::Result<Course, String> {
        // In a real implementation, we would use the structured data from Canvas
        // to build a unified Course model
        
        let id = canvas_course["id"].as_str()
            .ok_or_else(|| "Missing ID in Canvas course".to_string())?
            .to_string();
            
        let title = canvas_course["name"].as_str()
            .ok_or_else(|| "Missing name in Canvas course".to_string())?
            .to_string();
            
        let description = canvas_course["description"].as_str()
            .unwrap_or("")
            .to_string();
            
        let course = Course::new(
            Some(id),
            Some(title),
            Some(description),
            None, // created_at
            None, // updated_at
        );
        
        Ok(course)
    }
    
    fn create_course_from_discourse(&self, discourse_category: serde_json::Value) -> std::result::Result<Course, String> {
        // Similar to Canvas, but extracting from Discourse category JSON
        
        let id = discourse_category["id"].as_str()
            .ok_or_else(|| "Missing ID in Discourse category".to_string())?
            .to_string();
            
        let title = discourse_category["name"].as_str()
            .ok_or_else(|| "Missing name in Discourse category".to_string())?
            .to_string();
            
        let description = discourse_category["description"].as_str()
            .unwrap_or("")
            .to_string();
            
        let course = Course::new(
            Some(id),
            Some(title),
            Some(description),
            None, // created_at
            None, // updated_at
        );
        
        Ok(course)
    }
    
    fn create_discussion_from_canvas(&self, canvas_discussion: serde_json::Value) -> std::result::Result<Discussion, String> {
        // In a real implementation, we would use the structured data from Canvas
        // to build a unified Discussion model
        
        let id = canvas_discussion["id"].as_str()
            .ok_or_else(|| "Missing ID in Canvas discussion".to_string())?
            .to_string();
            
        let title = canvas_discussion["title"].as_str()
            .ok_or_else(|| "Missing title in Canvas discussion".to_string())?
            .to_string();
            
        let message = canvas_discussion["message"].as_str()
            .ok_or_else(|| "Missing message in Canvas discussion".to_string())?
            .to_string();
            
        let course_id = canvas_discussion["course_id"].as_str()
            .ok_or_else(|| "Missing course_id in Canvas discussion".to_string())?
            .to_string();
            
        let discussion = Discussion::new(
            Some(id),
            Some(title),
            Some(message),
            None, // created_at
            None, // updated_at
        );
        
        Ok(discussion)
    }
    
    fn create_discussion_from_discourse(&self, discourse_topic: serde_json::Value) -> std::result::Result<Discussion, String> {
        // Similar to Canvas, but extracting from Discourse topic JSON
        
        let id = discourse_topic["id"].as_str()
            .ok_or_else(|| "Missing ID in Discourse topic".to_string())?
            .to_string();
            
        let title = discourse_topic["title"].as_str()
            .ok_or_else(|| "Missing title in Discourse topic".to_string())?
            .to_string();
            
        let message = discourse_topic["raw"].as_str()
            .ok_or_else(|| "Missing raw content in Discourse topic".to_string())?
            .to_string();
            
        let category_id = discourse_topic["category_id"].as_str()
            .ok_or_else(|| "Missing category_id in Discourse topic".to_string())?
            .to_string();
            
        let discussion = Discussion::new(
            Some(id),
            Some(title),
            Some(message),
            None, // created_at
            None, // updated_at
        );
        
        Ok(discussion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::canvas_client::CanvasClient;
    use crate::api::discourse_client::DiscourseClient;
    
    fn setup_service() -> ModelSyncService {
        let canvas_client = Arc::new(CanvasClient::new("https://example.com", "test_key"));
        let discourse_client = Arc::new(DiscourseClient::new("https://example.com", "test_key", "test_user"));
        ModelSyncService::new(canvas_client, discourse_client)
    }
    
    #[tokio::test]
    async fn test_sync_user_from_canvas() {
        let service = setup_service();
        let user = service.sync_user("user1", SourceSystem::Canvas).await.unwrap();
        assert_eq!(user.id, "user1");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_sync_user_from_discourse() {
        let service = setup_service();
        let user = service.sync_user("user1", SourceSystem::Discourse).await.unwrap();
        assert_eq!(user.id, "user1");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "test_user");
    }
    
    #[tokio::test]
    async fn test_sync_course_from_canvas() {
        let service = setup_service();
        let course = service.sync_course("course1", SourceSystem::Canvas).await.unwrap();
        assert_eq!(course.id, Some("course1".to_string()));
        assert_eq!(course.title, "Test Course");
        assert_eq!(course.description, "Test course description");
    }
    
    #[tokio::test]
    async fn test_sync_course_from_discourse() {
        let service = setup_service();
        let course = service.sync_course("category1", SourceSystem::Discourse).await.unwrap();
        assert_eq!(course.id, Some("category1".to_string()));
        assert_eq!(course.title, "Test Category");
        assert_eq!(course.description, "Test category description");
    }
    
    #[tokio::test]
    async fn test_sync_discussion_from_canvas() {
        let service = setup_service();
        let discussion = service.sync_discussion("discussion1", "course1", SourceSystem::Canvas).await.unwrap();
        assert_eq!(discussion.id, Some("discussion1".to_string()));
        assert_eq!(discussion.title, "Test Discussion");
        assert_eq!(discussion.message, "Test discussion content");
    }
    
    #[tokio::test]
    async fn test_sync_discussion_from_discourse() {
        let service = setup_service();
        let discussion = service.sync_discussion("topic1", "category1", SourceSystem::Discourse).await.unwrap();
        assert_eq!(discussion.id, Some("topic1".to_string()));
        assert_eq!(discussion.title, "Test Topic");
        assert_eq!(discussion.message, "Test topic content");
    }
}
