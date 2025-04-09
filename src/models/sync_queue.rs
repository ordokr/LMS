use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncQueueItem {
    pub id: Uuid,
    pub topic_mapping_id: Uuid,
    pub sync_direction: String, // "canvas_to_discourse" or "discourse_to_canvas"
    pub status: String,         // "pending", "processing", "completed", "failed"
    pub attempt_count: i32,
    pub max_attempts: i32,
    pub last_attempt_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SyncQueueItem {
    pub fn new(topic_mapping_id: Uuid, sync_direction: String, max_attempts: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            topic_mapping_id,
            sync_direction,
            status: "pending".to_string(),
            attempt_count: 0,
            max_attempts,
            last_attempt_at: None,
            error_message: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn increment_attempt(&mut self) {
        self.attempt_count += 1;
        self.last_attempt_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn mark_processing(&mut self) {
        self.status = "processing".to_string();
        self.updated_at = Utc::now();
    }
    
    pub fn mark_completed(&mut self) {
        self.status = "completed".to_string();
        self.updated_at = Utc::now();
    }
    
    pub fn mark_failed(&mut self, error_message: &str) {
        self.status = 
            if self.attempt_count >= self.max_attempts {
                "failed"
            } else {
                "pending"
            }.to_string();
        self.error_message = Some(error_message.to_string());
        self.updated_at = Utc::now();
    }
    
    pub fn should_retry(&self) -> bool {
        self.status == "pending" && self.attempt_count < self.max_attempts
    }
}