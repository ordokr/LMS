use sqlx::{Pool, Postgres};
use anyhow::{Result, anyhow};
use chrono::Utc;
use crate::models::mapping::DiscussionTopicMapping;

pub struct TopicMappingRepository {
    pool: Pool<Postgres>,
}

impl TopicMappingRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
    
    /// Creates a new mapping between a Canvas discussion topic and a Discourse topic
    pub async fn create(
        &self, 
        canvas_topic_id: &str, 
        discourse_topic_id: &str,
        title: &str
    ) -> Result<DiscussionTopicMapping> {
        let now = Utc::now();
        
        let id = sqlx::query!(
            r#"
            INSERT INTO discussion_topic_mappings 
                (canvas_topic_id, discourse_topic_id, title, sync_enabled, created_at, updated_at, last_sync) 
            VALUES 
                ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            canvas_topic_id,
            discourse_topic_id,
            title,
            true, // sync_enabled by default
            now,
            now,
            now,  // last_sync initially set to creation time
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        Ok(DiscussionTopicMapping {
            id,
            canvas_topic_id: canvas_topic_id.to_string(),
            discourse_topic_id: discourse_topic_id.to_string(),
            title: title.to_string(),
            last_sync: now,
            sync_enabled: true,
        })
    }
    
    /// Retrieves a mapping by its ID
    pub async fn get_by_id(&self, id: String) -> Result<DiscussionTopicMapping> {
        sqlx::query_as!(
            DiscussionTopicMapping,
            r#"
            SELECT * FROM discussion_topic_mappings 
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get topic mapping by ID: {}", e))
    }
    
    /// Retrieves a mapping by Canvas topic ID
    pub async fn get_by_canvas_topic_id(&self, canvas_topic_id: &str) -> Result<DiscussionTopicMapping> {
        sqlx::query_as!(
            DiscussionTopicMapping,
            r#"
            SELECT * FROM discussion_topic_mappings 
            WHERE canvas_topic_id = $1
            "#,
            canvas_topic_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get mapping for Canvas topic ID {}: {}", canvas_topic_id, e))
    }

    /// Retrieves a mapping by Discourse topic ID
    pub async fn get_by_discourse_topic_id(&self, discourse_topic_id: &str) -> Result<DiscussionTopicMapping> {
        sqlx::query_as!(
            DiscussionTopicMapping,
            r#"
            SELECT * FROM discussion_topic_mappings 
            WHERE discourse_topic_id = $1
            "#,
            discourse_topic_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to get mapping for Discourse topic ID {}: {}", discourse_topic_id, e))
    }
    
    /// Updates the sync timestamp for a mapping
    pub async fn update_sync_time(&self, id: &str) -> Result<DiscussionTopicMapping> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE discussion_topic_mappings
            SET last_sync = $1, updated_at = $2
            WHERE id = $3
            "#,
            now,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id.to_string()).await
    }
    
    /// Lists all topic mappings
    pub async fn list_all(&self) -> Result<Vec<DiscussionTopicMapping>> {
        sqlx::query_as!(
            DiscussionTopicMapping,
            r#"
            SELECT * FROM discussion_topic_mappings 
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to list all topic mappings: {}", e))
    }
    
    /// Lists all topic mappings for a specific course-category mapping
    pub async fn list_by_course_category(&self, course_category_id: i64) -> Result<Vec<DiscussionTopicMapping>> {
        sqlx::query_as!(
            DiscussionTopicMapping,
            r#"
            SELECT dtm.* FROM discussion_topic_mappings dtm
            JOIN topic_course_category_rel tccr ON dtm.id = tccr.topic_mapping_id
            WHERE tccr.course_category_id = $1
            ORDER BY dtm.created_at DESC
            "#,
            course_category_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| anyhow!("Failed to list topic mappings for course-category ID {}: {}", course_category_id, e))
    }
    
    /// Deletes a mapping
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM discussion_topic_mappings
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Updates the sync setting for a mapping
    pub async fn update_sync_setting(&self, id: &str, sync_enabled: bool) -> Result<DiscussionTopicMapping> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE discussion_topic_mappings
            SET sync_enabled = $1, updated_at = $2
            WHERE id = $3
            "#,
            sync_enabled,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id.to_string()).await
    }
    
    /// Updates the title of a mapping
    pub async fn update_title(&self, id: &str, title: &str) -> Result<DiscussionTopicMapping> {
        let now = Utc::now();
        
        sqlx::query!(
            r#"
            UPDATE discussion_topic_mappings
            SET title = $1, updated_at = $2
            WHERE id = $3
            "#,
            title,
            now,
            id
        )
        .execute(&self.pool)
        .await?;
        
        self.get_by_id(id.to_string()).await
    }
    
    /// Associates a topic mapping with a course-category mapping
    pub async fn associate_with_course_category(
        &self, 
        topic_mapping_id: &str, 
        course_category_id: i64
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO topic_course_category_rel
                (topic_mapping_id, course_category_id, created_at)
            VALUES
                ($1, $2, $3)
            "#,
            topic_mapping_id,
            course_category_id,
            Utc::now(),
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
