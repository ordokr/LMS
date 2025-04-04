use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Represents a content tag in Canvas
/// Based on Canvas's ContentTag model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub title: Option<String>,
    pub context_type: Option<String>,
    pub context_id: Option<i64>,
    pub tag_type: Option<String>,
    pub url: Option<String>,
    pub workflow_state: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub content_type: Option<String>,
    pub content_id: Option<i64>,
}

impl Tag {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: None,
            context_type: None,
            context_id: None,
            tag_type: None,
            url: None,
            workflow_state: None,
            created_at: None,
            updated_at: None,
            content_type: None,
            content_id: None,
        }
    }
    
    /// Find tags for a specific context
    pub fn find_for_context(context_type: &str, context_id: i64) -> Vec<Self> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Get the content object this tag refers to
    pub fn content(&self) -> Result<serde_json::Value, String> {
        // Implementation would fetch the content based on content_type and content_id
        Err("Not implemented".to_string())
    }
    
    /// Check if the tag is published
    pub fn published(&self) -> bool {
        match &self.workflow_state {
            Some(state) => state == "active",
            None => false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagWithTopics {
    pub tag: Tag,
    pub recent_topics: Vec<crate::models::forum::Topic>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_restricted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Option<Option<String>> for null values
    pub color: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub is_restricted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FollowedTag {
    pub tag: Tag,
    pub notification_level: String, // "muted", "normal", or "high"
    pub followed_at: chrono::DateTime<chrono::Utc>,
}