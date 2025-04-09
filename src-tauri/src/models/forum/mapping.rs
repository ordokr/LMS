use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Maps between Canvas DiscussionTopic and Discourse Topic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMapping {
    pub id: Uuid,
    pub canvas_topic_id: String,
    pub discourse_topic_id: i64,
    pub last_sync_at: DateTime<Utc>,
    pub sync_status: SyncStatus,
    pub local_topic_id: Option<Uuid>, // Reference to our local Topic entity
}

/// Maps between Canvas DiscussionEntry and Discourse Post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMapping {
    pub id: Uuid,
    pub canvas_entry_id: String,
    pub discourse_post_id: i64,
    pub topic_mapping_id: Uuid,
    pub last_sync_at: DateTime<Utc>,
    pub sync_status: SyncStatus,
    pub local_post_id: Option<Uuid>, // Reference to our local Post entity
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncStatus {
    Synced,
    PendingToCanvas,
    PendingToDiscourse,
    Conflict,
    Error,
}

impl TopicMapping {
    pub fn new(canvas_id: String, discourse_id: i64) -> Self {
        TopicMapping {
            id: Uuid::new_v4(),
            canvas_topic_id: canvas_id,
            discourse_topic_id: discourse_id,
            last_sync_at: Utc::now(),
            sync_status: SyncStatus::Synced,
            local_topic_id: None,
        }
    }

    // Find mapping by Canvas topic ID
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let mapping = sqlx::query_as::<_, Self>(
            "SELECT * FROM topic_mappings WHERE canvas_topic_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(mapping)
    }
    
    // Find mapping by Discourse topic ID
    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let mapping = sqlx::query_as::<_, Self>(
            "SELECT * FROM topic_mappings WHERE discourse_topic_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(mapping)
    }
    
    // Create new mapping
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO topic_mappings 
            (id, canvas_topic_id, discourse_topic_id, last_sync_at, sync_status, local_topic_id)
            VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(&self.canvas_topic_id)
        .bind(self.discourse_topic_id)
        .bind(self.last_sync_at)
        .bind(self.sync_status as i32)
        .bind(self.local_topic_id)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    // Update mapping
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE topic_mappings SET
            canvas_topic_id = ?, discourse_topic_id = ?, last_sync_at = ?,
            sync_status = ?, local_topic_id = ?
            WHERE id = ?"
        )
        .bind(&self.canvas_topic_id)
        .bind(self.discourse_topic_id)
        .bind(self.last_sync_at)
        .bind(self.sync_status as i32)
        .bind(self.local_topic_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    // Delete mapping
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM topic_mappings WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Find all mappings that need sync
    pub async fn find_pending(db: &DB) -> Result<Vec<Self>, Error> {
        let mappings = sqlx::query_as::<_, Self>(
            "SELECT * FROM topic_mappings 
            WHERE sync_status IN (?, ?)"
        )
        .bind(SyncStatus::PendingToCanvas as i32)
        .bind(SyncStatus::PendingToDiscourse as i32)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(mappings)
    }
}

impl PostMapping {
    pub fn new(canvas_id: String, discourse_id: i64, topic_mapping_id: Uuid) -> Self {
        PostMapping {
            id: Uuid::new_v4(),
            canvas_entry_id: canvas_id,
            discourse_post_id: discourse_id,
            topic_mapping_id,
            last_sync_at: Utc::now(),
            sync_status: SyncStatus::Synced,
            local_post_id: None,
        }
    }

    // Find mapping by Canvas entry ID
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let mapping = sqlx::query_as::<_, Self>(
            "SELECT * FROM post_mappings WHERE canvas_entry_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(mapping)
    }
    
    // Find mapping by Discourse post ID
    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let mapping = sqlx::query_as::<_, Self>(
            "SELECT * FROM post_mappings WHERE discourse_post_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(mapping)
    }
    
    // Create new mapping
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO post_mappings 
            (id, canvas_entry_id, discourse_post_id, topic_mapping_id, 
            last_sync_at, sync_status, local_post_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(&self.canvas_entry_id)
        .bind(self.discourse_post_id)
        .bind(self.topic_mapping_id)
        .bind(self.last_sync_at)
        .bind(self.sync_status as i32)
        .bind(self.local_post_id)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    // Update mapping
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE post_mappings SET
            canvas_entry_id = ?, discourse_post_id = ?, topic_mapping_id = ?,
            last_sync_at = ?, sync_status = ?, local_post_id = ?
            WHERE id = ?"
        )
        .bind(&self.canvas_entry_id)
        .bind(self.discourse_post_id)
        .bind(self.topic_mapping_id)
        .bind(self.last_sync_at)
        .bind(self.sync_status as i32)
        .bind(self.local_post_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    // Delete mapping
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM post_mappings WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Find all mappings by topic mapping ID
    pub async fn find_by_topic_mapping_id(db: &DB, topic_mapping_id: Uuid) -> Result<Vec<Self>, Error> {
        let mappings = sqlx::query_as::<_, Self>(
            "SELECT * FROM post_mappings WHERE topic_mapping_id = ?"
        )
        .bind(topic_mapping_id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(mappings)
    }
}