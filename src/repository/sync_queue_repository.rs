use sqlx::{Pool, Postgres};
use anyhow::{Result, anyhow};
use chrono::Utc;
use uuid::Uuid;
use crate::models::sync_queue::SyncQueueItem;

pub struct SyncQueueRepository {
    pool: Pool<Postgres>,
}

impl SyncQueueRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    pub async fn enqueue(&self, topic_mapping_id: Uuid, sync_direction: &str, max_attempts: i32) -> Result<SyncQueueItem> {
        let item = SyncQueueItem::new(topic_mapping_id, sync_direction.to_string(), max_attempts);
        
        sqlx::query!(
            r#"
            INSERT INTO sync_queue 
                (id, topic_mapping_id, sync_direction, status, attempt_count, max_attempts, created_at, updated_at) 
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            item.id,
            item.topic_mapping_id,
            item.sync_direction,
            item.status,
            item.attempt_count,
            item.max_attempts,
            item.created_at,
            item.updated_at,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(item)
    }
    
    pub async fn get_by_id(&self, id: Uuid) -> Result<SyncQueueItem> {
        sqlx::query_as!(
            SyncQueueItem,
            r#"
            SELECT * FROM sync_queue 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get queue item by ID: {}", e))
    }
    
    pub async fn get_pending_items(&self, limit: i64) -> Result<Vec<SyncQueueItem>> {
        sqlx::query_as!(
            SyncQueueItem,
            r#"
            SELECT * FROM sync_queue 
            WHERE status = 'pending'
            ORDER BY created_at ASC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get pending queue items: {}", e))
    }
    
    pub async fn update_status(&self, id: Uuid, status: &str, error_message: Option<&str>) -> Result<SyncQueueItem> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE sync_queue
            SET 
                status = $1, 
                updated_at = $2,
                error_message = $3
            WHERE id = $4
            "#,
            status,
            now,
            error_message,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id).await
    }
    
    pub async fn increment_attempt(&self, id: Uuid) -> Result<SyncQueueItem> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE sync_queue
            SET 
                attempt_count = attempt_count + 1, 
                last_attempt_at = $1,
                updated_at = $2
            WHERE id = $3
            "#,
            now,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id).await
    }
    
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM sync_queue
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn clear_completed(&self, older_than_hours: i64) -> Result<u64> {
        let threshold = Utc::now() - chrono::Duration::hours(older_than_hours);
        
        let result = sqlx::query!(
            r#"
            DELETE FROM sync_queue
            WHERE status = 'completed' AND updated_at < $1
            "#,
            threshold
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.rows_affected())
    }
}