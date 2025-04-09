use crate::models::discussion::{Discussion, DiscussionStatus};
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use tracing::{info, error, instrument};

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Data error: {0}")]
    DataError(String),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait DiscussionRepository: Send + Sync {
    async fn get_discussions_by_course(&self, course_id: &str) -> Result<Vec<Discussion>, DbError>;
    async fn get_discussion_by_id(&self, id: &str) -> Result<Option<Discussion>, DbError>;
    async fn get_discussion_by_topic_id(&self, topic_id: &str) -> Result<Option<Discussion>, DbError>;
    async fn create_discussion(&self, discussion: Discussion) -> Result<Discussion, DbError>;
    async fn update_discussion(&self, discussion: Discussion) -> Result<Discussion, DbError>;
    async fn delete_discussion(&self, id: &str) -> Result<bool, DbError>;
}

pub struct SqliteDiscussionRepository {
    pool: Pool<Sqlite>,
}

impl SqliteDiscussionRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DiscussionRepository for SqliteDiscussionRepository {
    #[instrument(skip(self), err)]
    async fn get_discussions_by_course(&self, course_id: &str) -> Result<Vec<Discussion>, DbError> {
        sqlx::query_as::<_, Discussion>("SELECT * FROM discussions WHERE course_id = ? ORDER BY created_at DESC")
            .bind(course_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch discussions for course {}: {}", course_id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), err)]
    async fn get_discussion_by_id(&self, id: &str) -> Result<Option<Discussion>, DbError> {
        sqlx::query_as::<_, Discussion>("SELECT * FROM discussions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch discussion by ID {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), err)]
    async fn get_discussion_by_topic_id(&self, topic_id: &str) -> Result<Option<Discussion>, DbError> {
        sqlx::query_as::<_, Discussion>("SELECT * FROM discussions WHERE topic_id = ?")
            .bind(topic_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch discussion by topic ID {}: {}", topic_id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), fields(discussion_id = %discussion.id), err)]
    async fn create_discussion(&self, discussion: Discussion) -> Result<Discussion, DbError> {
        sqlx::query(
            "INSERT INTO discussions 
             (id, course_id, title, content, topic_id, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&discussion.id)
        .bind(&discussion.course_id)
        .bind(&discussion.title)
        .bind(&discussion.content)
        .bind(&discussion.topic_id)
        .bind(discussion.status.to_string())
        .bind(&discussion.created_at)
        .bind(&discussion.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create discussion: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        info!("Created new discussion with ID: {}", discussion.id);
        
        // Return the created discussion
        self.get_discussion_by_id(&discussion.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve created discussion".to_string()))
    }
    
    #[instrument(skip(self), fields(discussion_id = %discussion.id), err)]
    async fn update_discussion(&self, discussion: Discussion) -> Result<Discussion, DbError> {
        let result = sqlx::query(
            "UPDATE discussions 
             SET course_id = ?, title = ?, content = ?, topic_id = ?, status = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(&discussion.course_id)
        .bind(&discussion.title)
        .bind(&discussion.content)
        .bind(&discussion.topic_id)
        .bind(discussion.status.to_string())
        .bind(&discussion.updated_at)
        .bind(&discussion.id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update discussion: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        if result.rows_affected() == 0 {
            return Err(DbError::DataError(format!("Discussion with ID {} not found", discussion.id)));
        }
        
        info!("Updated discussion with ID: {}", discussion.id);
        
        // Return the updated discussion
        self.get_discussion_by_id(&discussion.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve updated discussion".to_string()))
    }
    
    #[instrument(skip(self), err)]
    async fn delete_discussion(&self, id: &str) -> Result<bool, DbError> {
        let result = sqlx::query("DELETE FROM discussions WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete discussion {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted discussion with ID: {}", id);
        } else {
            info!("No discussion found to delete with ID: {}", id);
        }
        
        Ok(deleted)
    }
}