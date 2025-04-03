use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: Option<i64>,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub content_html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Post {
    pub fn new(topic_id: i64, user_id: i64, content: String) -> Self {
        let now = Utc::now();
        // In a real app, you would convert markdown to HTML here
        // For now we just store the same content
        let content_html = content.clone();
        
        Self {
            id: None,
            topic_id,
            user_id,
            content,
            content_html,
            created_at: now,
            updated_at: now,
            is_deleted: false,
        }
    }
    
    pub fn edit(&mut self, new_content: String) {
        self.content = new_content;
        // Update HTML version as well
        self.content_html = new_content.clone();
        self.updated_at = Utc::now();
    }
}