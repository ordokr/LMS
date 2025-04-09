use crate::models::submission::{Submission, SubmissionStatus};
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
pub trait SubmissionRepository: Send + Sync {
    async fn get_submissions_by_assignment(&self, assignment_id: &str) -> Result<Vec<Submission>, DbError>;
    async fn get_submissions_by_user(&self, user_id: &str) -> Result<Vec<Submission>, DbError>;
    async fn get_submission_by_id(&self, id: &str) -> Result<Option<Submission>, DbError>;
    async fn create_submission(&self, submission: Submission) -> Result<Submission, DbError>;
    async fn update_submission(&self, submission: Submission) -> Result<Submission, DbError>;
}

pub struct SqliteSubmissionRepository {
    pool: Pool<Sqlite>,
}

impl SqliteSubmissionRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepository for SqliteSubmissionRepository {
    #[instrument(skip(self), err)]
    async fn get_submissions_by_assignment(&self, assignment_id: &str) -> Result<Vec<Submission>, DbError> {
        sqlx::query_as::<_, Submission>("SELECT * FROM submissions WHERE assignment_id = ? ORDER BY submitted_at DESC")
            .bind(assignment_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch submissions for assignment {}: {}", assignment_id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), err)]
    async fn get_submissions_by_user(&self, user_id: &str) -> Result<Vec<Submission>, DbError> {
        sqlx::query_as::<_, Submission>("SELECT * FROM submissions WHERE user_id = ? ORDER BY submitted_at DESC")
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch submissions for user {}: {}", user_id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), err)]
    async fn get_submission_by_id(&self, id: &str) -> Result<Option<Submission>, DbError> {
        sqlx::query_as::<_, Submission>("SELECT * FROM submissions WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch submission by ID {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), fields(submission_id = %submission.id), err)]
    async fn create_submission(&self, submission: Submission) -> Result<Submission, DbError> {
        sqlx::query(
            "INSERT INTO submissions 
             (id, assignment_id, user_id, content, attachments, status, score, feedback, submitted_at, graded_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&submission.id)
        .bind(&submission.assignment_id)
        .bind(&submission.user_id)
        .bind(&submission.content)
        .bind(&submission.attachments)
        .bind(submission.status.to_string())
        .bind(submission.score)
        .bind(&submission.feedback)
        .bind(&submission.submitted_at)
        .bind(&submission.graded_at)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to create submission: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        info!("Created new submission with ID: {}", submission.id);
        
        // Return the created submission
        self.get_submission_by_id(&submission.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve created submission".to_string()))
    }
    
    #[instrument(skip(self), fields(submission_id = %submission.id), err)]
    async fn update_submission(&self, submission: Submission) -> Result<Submission, DbError> {
        let result = sqlx::query(
            "UPDATE submissions 
             SET content = ?, attachments = ?, status = ?, score = ?, feedback = ?, graded_at = ?
             WHERE id = ?"
        )
        .bind(&submission.content)
        .bind(&submission.attachments)
        .bind(submission.status.to_string())
        .bind(submission.score)
        .bind(&submission.feedback)
        .bind(&submission.graded_at)
        .bind(&submission.id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update submission: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        if result.rows_affected() == 0 {
            return Err(DbError::DataError(format!("Submission with ID {} not found", submission.id)));
        }
        
        info!("Updated submission with ID: {}", submission.id);
        
        // Return the updated submission
        self.get_submission_by_id(&submission.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve updated submission".to_string()))
    }
}