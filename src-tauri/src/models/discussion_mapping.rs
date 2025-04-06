use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionMapping {
    pub id: String,
    pub canvas_discussion_id: String,
    pub discourse_topic_id: String,
    pub course_category_id: String,
    pub title: String,
    pub last_sync: DateTime<Utc>,
    pub sync_enabled: bool,
    pub sync_posts: bool,
    pub created_at: DateTime<Utc>,
}

impl DiscussionMapping {
    pub fn new(
        canvas_discussion_id: &str,
        discourse_topic_id: &str,
        course_category_id: &str,
        title: &str,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            canvas_discussion_id: canvas_discussion_id.to_string(),
            discourse_topic_id: discourse_topic_id.to_string(),
            course_category_id: course_category_id.to_string(),
            title: title.to_string(),
            last_sync: now,
            sync_enabled: true,
            sync_posts: true,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDiscussionEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub parent_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseTopic {
    pub id: String,
    pub title: String,
    pub category_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoursePost {
    pub id: String,
    pub topic_id: String,
    pub user_id: Option<String>,
    pub content: String,
    pub external_id: Option<String>, // Used to track Canvas source
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SyncResult {
    pub mapping_id: String,
    pub timestamp: DateTime<Utc>,
    pub canvas_updates: u32,
    pub discourse_updates: u32,
    pub errors: Vec<String>,
    pub status: String,
}

impl SyncResult {
    pub fn new(mapping_id: &str) -> Self {
        Self {
            mapping_id: mapping_id.to_string(),
            timestamp: Utc::now(),
            canvas_updates: 0,
            discourse_updates: 0,
            errors: Vec::new(),
            status: "started".to_string(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        if self.status != "failed" {
            self.status = "partial".to_string();
        }
    }

    pub fn set_failed(&mut self, error: String) {
        self.errors.push(error);
        self.status = "failed".to_string();
    }

    pub fn complete(&mut self) {
        if self.errors.is_empty() {
            self.status = "success".to_string();
        }
    }
}