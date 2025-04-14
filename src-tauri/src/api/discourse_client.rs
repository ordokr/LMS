use reqwest::{Client, header, StatusCode};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::Arc;

// Error type for Discourse API operations
#[derive(Debug, thiserror::Error)]
pub enum DiscourseApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("API error: {status_code} - {message}")]
    ApiError {
        status_code: StatusCode,
        message: String,
    },
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

// Type alias for result with DiscourseApiError
pub type Result<T> = std::result::Result<T, DiscourseApiError>;

// Notification model that matches the Discourse API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseNotification {
    pub id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    pub read: bool,
    #[serde(rename = "notificationType")]
    pub notification_type: String,
    // Add any other fields needed
    #[serde(flatten)]
    pub additional_fields: serde_json::Map<String, serde_json::Value>,
}

// Discourse API client
#[derive(Debug, Clone)]
pub struct DiscourseClient {
    base_url: String,
    api_key: String,
    api_username: String,
    client: Client,
}

impl DiscourseClient {
    pub fn new(base_url: &str, api_key: &str, api_username: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");
            
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            api_username: api_username.to_string(),
            client,
        }
    }
    
    pub async fn get_user_notifications(&self, discourse_user_id: &str) -> Result<Vec<DiscourseNotification>> {
        // For now, implementing a stub similar to the JS version
        // In a real implementation, this would make an HTTP request to Discourse
        
        let notification = DiscourseNotification {
            id: "discourse1".to_string(),
            created_at: Utc::now(),
            read: false,
            notification_type: "discussion".to_string(),
            additional_fields: serde_json::Map::new(),
        };
        
        Ok(vec![notification])
    }
    
    pub async fn mark_notification_as_read(&self, notification_id: &str) -> Result<DiscourseNotification> {
        // Stub implementation
        let notification = DiscourseNotification {
            id: notification_id.to_string(),
            created_at: Utc::now(),
            read: true,
            notification_type: "discussion".to_string(),
            additional_fields: serde_json::Map::new(),
        };
        
        Ok(notification)
    }
    
    pub async fn create_notification(&self, notification_data: serde_json::Value) -> Result<DiscourseNotification> {
        // Extract required fields from notification_data
        // In a real implementation, we would send this to the Discourse API
        
        let notification_type = notification_data.get("notificationType")
            .and_then(|v| v.as_str())
            .unwrap_or("general")
            .to_string();
        
        let mut additional_fields = serde_json::Map::new();
        if let Some(obj) = notification_data.as_object() {
            for (key, value) in obj {
                if key != "id" && key != "createdAt" && key != "read" && key != "notificationType" {
                    additional_fields.insert(key.clone(), value.clone());
                }
            }
        }
        
        let notification = DiscourseNotification {
            id: "discourse_created_id".to_string(),
            created_at: Utc::now(),
            read: false,
            notification_type,
            additional_fields,
        };
        
        Ok(notification)
    }
    
    // Additional Discourse-specific methods
    
    // Get categories from Discourse
    pub async fn get_categories(&self) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}/categories.json", self.base_url);
        
        let response = self.client.get(&url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .send()
            .await
            .map_err(DiscourseApiError::HttpError)?;
            
        if !response.status().is_success() {
            return Err(DiscourseApiError::ApiError {
                status_code: response.status(),
                message: response.text().await.unwrap_or_else(|_| "Unknown error".to_string()),
            });
        }
        
        // In a real implementation, we would parse the response
        // For now, just return a stub
        Ok(vec![serde_json::json!({
            "id": 1,
            "name": "Example Category",
            "slug": "example-category",
            "description": "This is an example category",
        })])
    }
    
    // Get a specific topic from Discourse
    pub async fn get_topic(&self, topic_id: u64) -> Result<serde_json::Value> {
        let url = format!("{}/t/{}.json", self.base_url, topic_id);
        
        let response = self.client.get(&url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .send()
            .await
            .map_err(DiscourseApiError::HttpError)?;
            
        if !response.status().is_success() {
            return Err(DiscourseApiError::ApiError {
                status_code: response.status(),
                message: response.text().await.unwrap_or_else(|_| "Unknown error".to_string()),
            });
        }
        
        // In a real implementation, we would parse the response
        // For now, just return a stub
        Ok(serde_json::json!({
            "id": topic_id,
            "title": "Example Topic",
            "posts_count": 5,
            "created_at": Utc::now().to_rfc3339(),
        }))
    }

    // Fetch posts for a specific topic
    pub async fn get_topic_posts(&self, topic_id: u64) -> Result<Vec<serde_json::Value>> {
        let url = format!("{}/t/{}/posts.json", self.base_url, topic_id);

        let response = self.client.get(&url)
            .query(&[
                ("api_key", &self.api_key),
                ("api_username", &self.api_username),
            ])
            .send()
            .await
            .map_err(DiscourseApiError::HttpError)?;

        if !response.status().is_success() {
            return Err(DiscourseApiError::ApiError {
                status_code: response.status(),
                message: response.text().await.unwrap_or_else(|_| "Unknown error".to_string()),
            });
        }

        let posts = response.json::<serde_json::Value>().await
            .map_err(DiscourseApiError::SerializationError)?;

        // Extract posts array from the response
        if let Some(posts_array) = posts.get("post_stream").and_then(|ps| ps.get("posts")).and_then(|p| p.as_array()) {
            return Ok(posts_array.clone());
        }

        Err(DiscourseApiError::ApiError {
            status_code: response.status(),
            message: "Failed to parse posts from response".to_string(),
        })
    }
}

// Factory function to create a Discourse client
pub fn create_discourse_client(base_url: &str, api_key: &str, api_username: &str) -> Arc<DiscourseClient> {
    Arc::new(DiscourseClient::new(base_url, api_key, api_username))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_user_notifications() {
        let client = DiscourseClient::new("https://example.com", "test_key", "test_user");
        let notifications = client.get_user_notifications("user1").await.unwrap();
        assert!(!notifications.is_empty());
        assert_eq!(notifications[0].id, "discourse1");
    }
    
    #[tokio::test]
    async fn test_mark_notification_as_read() {
        let client = DiscourseClient::new("https://example.com", "test_key", "test_user");
        let notification = client.mark_notification_as_read("test_id").await.unwrap();
        assert_eq!(notification.id, "test_id");
        assert!(notification.read);
    }
    
    #[tokio::test]
    async fn test_create_notification() {
        let client = DiscourseClient::new("https://example.com", "test_key", "test_user");
        let data = serde_json::json!({
            "notificationType": "test_type",
            "message": "Test message"
        });
        let notification = client.create_notification(data).await.unwrap();
        assert_eq!(notification.id, "discourse_created_id");
        assert_eq!(notification.notification_type, "test_type");
    }
    
    #[tokio::test]
    async fn test_get_categories() {
        let client = DiscourseClient::new("https://example.com", "test_key", "test_user");
        let categories = client.get_categories().await.unwrap();
        assert!(!categories.is_empty());
        assert_eq!(categories[0]["name"], "Example Category");
    }
    
    #[tokio::test]
    async fn test_get_topic() {
        let client = DiscourseClient::new("https://example.com", "test_key", "test_user");
        let topic = client.get_topic(123).await.unwrap();
        assert_eq!(topic["id"], 123);
        assert_eq!(topic["title"], "Example Topic");
    }
}
