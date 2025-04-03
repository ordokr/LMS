use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub parent_id: Option<i64>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Category {
    pub fn new(name: String, slug: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            name,
            slug,
            description,
            color: Some("#3498DB".to_string()),  // Default blue
            text_color: Some("#FFFFFF".to_string()), // Default white
            parent_id: None,
            position: 0,
            created_at: now,
            updated_at: now,
            is_deleted: false,
        }
    }
    
    pub fn is_subcategory(&self) -> bool {
        self.parent_id.is_some()
    }
}