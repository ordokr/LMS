use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Topic {
    pub id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub category_id: i64,
    pub user_id: i64,
    pub views: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub is_closed: bool,
    pub is_pinned: bool,
    pub is_deleted: bool,
}

impl Topic {
    pub fn new(title: String, slug: String, category_id: i64, user_id: i64) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            title,
            slug,
            category_id,
            user_id,
            views: 0,
            created_at: now,
            updated_at: now,
            last_posted_at: None,
            is_closed: false,
            is_pinned: false,
            is_deleted: false,
        }
    }
    
    pub fn increment_view(&mut self) {
        self.views += 1;
    }
    
    pub fn mark_active(&mut self) {
        self.updated_at = Utc::now();
    }
}