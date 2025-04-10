use reqwest::{Client, header, StatusCode};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::Arc;

// Error type for Canvas API operations
#[derive(Debug, thiserror::Error)]
pub enum CanvasApiError {
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

// Type alias for result with CanvasApiError
pub type Result<T> = std::result::Result<T, CanvasApiError>;

// Notification model that matches the Canvas API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasNotification {
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

// Canvas API client
#[derive(Debug, Clone)]
pub struct CanvasClient {
    base_url: String,
    api_key: String,
    client: Client,
}

impl CanvasClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .expect("Invalid API key format"),
        );
        
        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");
            
        Self {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            client,
        }
    }
    
    pub async fn get_user_notifications(&self, canvas_user_id: &str) -> Result<Vec<CanvasNotification>> {
        // For now, implementing a stub similar to the JS version
        // In a real implementation, this would make an HTTP request to Canvas
        
        let notification = CanvasNotification {
            id: "canvas1".to_string(),
            created_at: Utc::now(),
            read: false,
            notification_type: "assignment".to_string(),
            additional_fields: serde_json::Map::new(),
        };
        
        Ok(vec![notification])
    }
    
    pub async fn mark_notification_as_read(&self, notification_id: &str) -> Result<CanvasNotification> {
        // Stub implementation
        let notification = CanvasNotification {
            id: notification_id.to_string(),
            created_at: Utc::now(),
            read: true,
            notification_type: "assignment".to_string(),
            additional_fields: serde_json::Map::new(),
        };
        
        Ok(notification)
    }
    
    pub async fn create_notification(&self, notification_data: serde_json::Value) -> Result<CanvasNotification> {
        // Extract required fields from notification_data
        // In a real implementation, we would send this to the Canvas API
        
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
        
        let notification = CanvasNotification {
            id: "canvas_created_id".to_string(),
            created_at: Utc::now(),
            read: false,
            notification_type,
            additional_fields,
        };
        
        Ok(notification)
    }
}

// Factory function to create a Canvas client
pub fn create_canvas_client(base_url: &str, api_key: &str) -> Arc<CanvasClient> {
    Arc::new(CanvasClient::new(base_url, api_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_user_notifications() {
        let client = CanvasClient::new("https://example.com", "test_key");
        let notifications = client.get_user_notifications("user1").await.unwrap();
        assert!(!notifications.is_empty());
        assert_eq!(notifications[0].id, "canvas1");
    }
    
    #[tokio::test]
    async fn test_mark_notification_as_read() {
        let client = CanvasClient::new("https://example.com", "test_key");
        let notification = client.mark_notification_as_read("test_id").await.unwrap();
        assert_eq!(notification.id, "test_id");
        assert!(notification.read);
    }
    
    #[tokio::test]
    async fn test_create_notification() {
        let client = CanvasClient::new("https://example.com", "test_key");
        let data = serde_json::json!({
            "notificationType": "test_type",
            "message": "Test message"
        });
        let notification = client.create_notification(data).await.unwrap();
        assert_eq!(notification.id, "canvas_created_id");
        assert_eq!(notification.notification_type, "test_type");
    }
}
