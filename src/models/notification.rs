// src/models/notification.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use uuid::Uuid;

/// Unified Notification model for cross-platform notifications
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: Option<String>,
    pub subject: String,
    pub message: String,
    pub created_at: String,
    pub read: bool,
    pub notification_type: String,
    pub source_system: Option<String>,
    pub canvas_id: Option<String>,
    pub discourse_id: Option<String>,
    pub url: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Notification {
    /// Create a new unified notification
    pub fn new(
        id: Option<String>,
        user_id: Option<String>,
        subject: Option<String>,
        message: Option<String>,
        created_at: Option<String>,
        read: Option<bool>,
        notification_type: Option<String>,
        source_system: Option<String>,
        canvas_id: Option<String>,
        discourse_id: Option<String>,
        url: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Self {
        let now = Utc::now().to_rfc3339();
        
        Notification {
            id: id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            user_id,
            subject: subject.unwrap_or_default(),
            message: message.unwrap_or_default(),
            created_at: created_at.unwrap_or_else(|| now),
            read: read.unwrap_or(false),
            notification_type: notification_type.unwrap_or_else(|| "general".to_string()),
            source_system,
            canvas_id,
            discourse_id,
            url: url.unwrap_or_default(),
            metadata: metadata.unwrap_or_default(),
        }
    }

    /// Convert Canvas notification to unified model
    pub fn from_canvas_notification(canvas_notification: &serde_json::Value) -> Self {
        let id = canvas_notification["id"].as_str().map(String::from);
        let user_id = canvas_notification["user_id"].as_str().map(String::from);
        
        let subject = canvas_notification["subject"].as_str()
            .or_else(|| canvas_notification["title"].as_str())
            .map(String::from);
            
        let message = canvas_notification["message"].as_str()
            .or_else(|| canvas_notification["body"].as_str())
            .map(String::from);
            
        let created_at = canvas_notification["created_at"].as_str().map(String::from);
        
        let read = canvas_notification["read"].as_bool().unwrap_or(false);
        
        let notification_type = canvas_notification["notification_type"].as_str()
            .map(String::from)
            .unwrap_or_else(|| "general".to_string());
            
        let url = canvas_notification["html_url"].as_str().map(String::from);
        
        let mut metadata = HashMap::new();
        metadata.insert("original".to_string(), canvas_notification.clone());
        
        Self::new(
            None,
            user_id,
            subject,
            message,
            created_at,
            Some(read),
            Some(notification_type),
            Some("canvas".to_string()),
            id,
            None,
            url,
            Some(metadata),
        )
    }
    
    /// Convert Discourse notification to unified model
    pub fn from_discourse_notification(discourse_notification: &serde_json::Value) -> Self {
        let id = discourse_notification["id"].as_str().map(String::from);
        let user_id = discourse_notification["user_id"].as_str().map(String::from);
        
        let subject = discourse_notification["data"]["topic_title"].as_str()
            .map(String::from);
            
        let message = discourse_notification["data"]["excerpt"].as_str()
            .map(String::from);
            
        let created_at = discourse_notification["created_at"].as_str().map(String::from);
        
        let read = discourse_notification["read"].as_bool().unwrap_or(false);
        
        let notification_type = discourse_notification["notification_type"].as_str()
            .map(String::from)
            .unwrap_or_else(|| "discussion".to_string());
            
        let url = discourse_notification["data"]["topic_url"].as_str().map(String::from);
        
        let mut metadata = HashMap::new();
        metadata.insert("original".to_string(), discourse_notification.clone());
        
        Self::new(
            None,
            user_id,
            subject,
            message,
            created_at,
            Some(read),
            Some(notification_type),
            Some("discourse".to_string()),
            None,
            id,
            url,
            Some(metadata),
        )
    }
}
