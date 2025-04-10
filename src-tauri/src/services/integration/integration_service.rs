use crate::api::canvas_client::CanvasClient;
use crate::api::discourse_client::DiscourseClient;
use std::sync::Arc;
use log::{info, error};
use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Error type for integration operations
#[derive(Debug, Error)]
pub enum IntegrationError {
    #[error("Canvas API error: {0}")]
    CanvasError(#[from] crate::api::canvas_client::CanvasApiError),
    
    #[error("Discourse API error: {0}")]
    DiscourseError(#[from] crate::api::discourse_client::DiscourseApiError),
    
    #[error("Integration error: {0}")]
    IntegrationError(String),
}

/// Type alias for result with IntegrationError
pub type Result<T> = std::result::Result<T, IntegrationError>;

/// Announcement data from Canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub id: String,
    pub title: String,
    pub message: String,
    #[serde(rename = "courseId")]
    pub course_id: String,
    // Add other necessary fields
}

/// Sync result for announcements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnouncementSyncResult {
    pub success: bool,
    #[serde(rename = "canvasAnnouncementId")]
    pub canvas_announcement_id: Option<String>,
    #[serde(rename = "discourseTopicId")]
    pub discourse_topic_id: Option<String>,
    #[serde(rename = "discourseTopic")]
    pub discourse_topic: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Authentication result for SSO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    #[serde(rename = "canvasUserId")]
    pub canvas_user_id: Option<String>,
    #[serde(rename = "discourseUserId")]
    pub discourse_user_id: Option<String>,
    #[serde(rename = "ssoToken")]
    pub sso_token: Option<String>,
    pub error: Option<String>,
}

/// Canvas user data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasUser {
    pub id: String,
    pub name: String,
    pub email: String,
    // Add other necessary fields
}

/// Service for integrating Canvas and Discourse
#[derive(Debug)]
pub struct IntegrationService {
    canvas_client: Arc<CanvasClient>,
    discourse_client: Arc<DiscourseClient>,
}

impl IntegrationService {
    /// Create a new integration service
    pub fn new(canvas_client: Arc<CanvasClient>, discourse_client: Arc<DiscourseClient>) -> Self {
        Self {
            canvas_client,
            discourse_client,
        }
    }
    
    /// Synchronize a Canvas announcement to a Discourse forum topic
    pub async fn sync_announcement_to_forum(&self, announcement: Announcement) -> AnnouncementSyncResult {
        info!("Syncing announcement \"{}\" to Discourse", announcement.title);
        
        match self.sync_announcement_to_forum_internal(&announcement).await {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to sync announcement: {}", e);
                AnnouncementSyncResult {
                    success: false,
                    canvas_announcement_id: Some(announcement.id),
                    discourse_topic_id: None,
                    discourse_topic: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }
    
    /// Internal implementation for syncing announcements
    async fn sync_announcement_to_forum_internal(&self, announcement: &Announcement) -> Result<AnnouncementSyncResult> {
        // Get the appropriate Discourse category for the Canvas course
        let category_id = self.get_discourse_category(&announcement.course_id).await?;
        
        // Create a topic in Discourse
        let topic_request = serde_json::json!({
            "title": announcement.title,
            "raw": announcement.message,
            "category": category_id,
        });
        
        // In a real implementation, this would use the Discourse client to create a topic
        // For now we'll simulate the response
        let topic_result = serde_json::json!({
            "topic_id": format!("discourse-topic-{}", rand::random::<u32>()),
            "title": announcement.title,
            "created_at": chrono::Utc::now().to_rfc3339(),
        });
        
        info!("Created Discourse topic {}", topic_result["topic_id"]);
        
        Ok(AnnouncementSyncResult {
            success: true,
            canvas_announcement_id: Some(announcement.id.clone()),
            discourse_topic_id: Some(topic_result["topic_id"].as_str().unwrap_or("").to_string()),
            discourse_topic: Some(topic_result),
            error: None,
        })
    }
    
    /// Get the appropriate Discourse category for a Canvas course
    pub async fn get_discourse_category(&self, _course_id: &str) -> Result<u64> {
        // In a real implementation, this would look up the mapping
        // For now, just return a mock ID
        Ok(5) // Mock category ID
    }
    
    /// Authenticate a Canvas user with Discourse via SSO
    pub async fn authenticate_user_with_discourse(&self, canvas_user: CanvasUser) -> AuthenticationResult {
        info!("Authenticating user {} with Discourse", canvas_user.name);
        
        match self.authenticate_user_with_discourse_internal(&canvas_user).await {
            Ok(result) => result,
            Err(e) => {
                error!("Failed to authenticate user: {}", e);
                AuthenticationResult {
                    success: false,
                    canvas_user_id: Some(canvas_user.id),
                    discourse_user_id: None,
                    sso_token: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }
    
    /// Internal implementation for authenticating users
    async fn authenticate_user_with_discourse_internal(&self, canvas_user: &CanvasUser) -> Result<AuthenticationResult> {
        // In a real implementation, this would create an SSO payload and send it to Discourse
        // For now, we'll simulate the response
        
        // Extract username from email
        let username = canvas_user.email.split('@').next().unwrap_or("user");
        
        // Simulate the SSO response
        let sso_result = serde_json::json!({
            "id": format!("discourse-user-{}", rand::random::<u32>()),
            "username": username,
            "name": canvas_user.name,
            "email": canvas_user.email,
        });
        
        // Generate a random token
        let token = format!("sample-token-{}", uuid::Uuid::new_v4());
        
        Ok(AuthenticationResult {
            success: true,
            canvas_user_id: Some(canvas_user.id.clone()),
            discourse_user_id: Some(sso_result["id"].as_str().unwrap_or("").to_string()),
            sso_token: Some(token),
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::canvas_client::CanvasClient;
    use crate::api::discourse_client::DiscourseClient;
    
    fn setup_service() -> IntegrationService {
        let canvas_client = Arc::new(CanvasClient::new("https://example.com", "test_key"));
        let discourse_client = Arc::new(DiscourseClient::new("https://example.com", "test_key", "test_user"));
        IntegrationService::new(canvas_client, discourse_client)
    }
    
    #[tokio::test]
    async fn test_sync_announcement_to_forum() {
        let service = setup_service();
        let announcement = Announcement {
            id: "canvas-announcement-1".to_string(),
            title: "Test Announcement".to_string(),
            message: "This is a test announcement".to_string(),
            course_id: "course-1".to_string(),
        };
        
        let result = service.sync_announcement_to_forum(announcement).await;
        assert!(result.success);
        assert!(result.discourse_topic_id.is_some());
    }
    
    #[tokio::test]
    async fn test_authenticate_user_with_discourse() {
        let service = setup_service();
        let canvas_user = CanvasUser {
            id: "canvas-user-1".to_string(),
            name: "Test User".to_string(),
            email: "test.user@example.com".to_string(),
        };
        
        let result = service.authenticate_user_with_discourse(canvas_user).await;
        assert!(result.success);
        assert!(result.discourse_user_id.is_some());
        assert!(result.sso_token.is_some());
    }
    
    #[tokio::test]
    async fn test_get_discourse_category() {
        let service = setup_service();
        let category_id = service.get_discourse_category("course-1").await.unwrap();
        assert_eq!(category_id, 5);
    }
}
