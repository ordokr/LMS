use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TopicMapping {
    pub id: Uuid,
    pub canvas_topic_id: String,
    pub discourse_topic_id: String,
    pub mapping_id: Uuid,  // Foreign key to course_category_mapping
    pub sync_enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub canvas_updated_at: Option<DateTime<Utc>>, 
    pub discourse_updated_at: Option<DateTime<Utc>>,
}

impl TopicMapping {
    pub fn new(canvas_topic_id: String, discourse_topic_id: String, mapping_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            canvas_topic_id,
            discourse_topic_id,
            mapping_id,
            sync_enabled: true,
            created_at: now,
            updated_at: now,
            last_synced_at: None,
            canvas_updated_at: None,
            discourse_updated_at: None,
        }
    }
    
    pub fn update_sync_time(&mut self) {
        self.last_synced_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    pub fn update_canvas_time(&mut self, time: DateTime<Utc>) {
        self.canvas_updated_at = Some(time);
        self.updated_at = Utc::now();
    }
    
    pub fn update_discourse_time(&mut self, time: DateTime<Utc>) {
        self.discourse_updated_at = Some(time);
        self.updated_at = Utc::now();
    }
}

// Entry/Post mapping
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PostMapping {
    pub id: Uuid,
    pub canvas_entry_id: String,
    pub discourse_post_id: String,
    pub topic_mapping_id: Uuid, // Foreign key to topic_mapping
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_synced_at: Option<DateTime<Utc>>,
}

impl PostMapping {
    pub fn new(canvas_entry_id: String, discourse_post_id: String, topic_mapping_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            canvas_entry_id,
            discourse_post_id,
            topic_mapping_id,
            created_at: now,
            updated_at: now,
            last_synced_at: None,
        }
    }
    
    pub fn update_sync_time(&mut self) {
        self.last_synced_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}