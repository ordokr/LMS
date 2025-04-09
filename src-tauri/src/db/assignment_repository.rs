use crate::models::course::{Assignment, AssignmentStatus};
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
pub trait AssignmentRepository: Send + Sync {
    async fn get_assignments(&self, course_id: Option<String>) -> Result<Vec<Assignment>, DbError>;
    async fn get_assignment_by_id(&self, id: &str) -> Result<Option<Assignment>, DbError>;
    async fn create_assignment(&self, assignment: Assignment) -> Result<Assignment, DbError>;
    async fn update_assignment(&self, assignment: Assignment) -> Result<Assignment, DbError>;
    async fn delete_assignment(&self, id: &str) -> Result<bool, DbError>;
}

pub struct SqliteAssignmentRepository {
    pool: Pool<Sqlite>,
}

impl SqliteAssignmentRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AssignmentRepository for SqliteAssignmentRepository {
    #[instrument(skip(self), err)]
    async fn get_assignments(&self, course_id: Option<String>) -> Result<Vec<Assignment>, DbError> {
        let mut query = sqlx::QueryBuilder::new("SELECT * FROM assignments");
        
        if let Some(course_id) = course_id {
            query.push(" WHERE course_id = ");
            query.push_bind(course_id);
        }
        
        let assignments = query
            .build_query_as::<Assignment>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch assignments: {}", e);
                DbError::QueryError(e.to_string())
            })?;
            
        info!("Retrieved {} assignments", assignments.len());
        Ok(assignments)
    }
    
    #[instrument(skip(self), err)]
    async fn get_assignment_by_id(&self, id: &str) -> Result<Option<Assignment>, DbError> {
        sqlx::query_as::<_, Assignment>("SELECT * FROM assignments WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch assignment by ID {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })
    }
    
    #[instrument(skip(self), fields(assignment_id = %assignment.id), err)]
    async fn create_assignment(&self, assignment: Assignment) -> Result<Assignment, DbError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Insert assignment
        sqlx::query(
            "INSERT INTO assignments 
             (id, course_id, title, description, due_date, points_possible, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind(&assignment.id)
        .bind(&assignment.course_id)
        .bind(&assignment.title)
        .bind(&assignment.description)
        .bind(&assignment.due_date)
        .bind(assignment.points_possible)
        .bind(assignment.status.to_string())
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to insert assignment: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Created new assignment with ID: {}", assignment.id);
        
        // Fetch the newly created assignment
        self.get_assignment_by_id(&assignment.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve created assignment".to_string()))
    }
    
    #[instrument(skip(self), fields(assignment_id = %assignment.id), err)]
    async fn update_assignment(&self, assignment: Assignment) -> Result<Assignment, DbError> {
        let result = sqlx::query(
            "UPDATE assignments 
             SET course_id = ?, title = ?, description = ?, due_date = ?, 
             points_possible = ?, status = ?, updated_at = datetime('now')
             WHERE id = ?"
        )
        .bind(&assignment.course_id)
        .bind(&assignment.title)
        .bind(&assignment.description)
        .bind(&assignment.due_date)
        .bind(assignment.points_possible)
        .bind(assignment.status.to_string())
        .bind(&assignment.id)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to update assignment: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        if result.rows_affected() == 0 {
            return Err(DbError::DataError(format!("Assignment with ID {} not found", assignment.id)));
        }
        
        info!("Updated assignment with ID: {}", assignment.id);
        
        // Fetch the updated assignment
        self.get_assignment_by_id(&assignment.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve updated assignment".to_string()))
    }
    
    #[instrument(skip(self), err)]
    async fn delete_assignment(&self, id: &str) -> Result<bool, DbError> {
        let result = sqlx::query("DELETE FROM assignments WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to delete assignment {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted assignment with ID: {}", id);
        } else {
            info!("No assignment found to delete with ID: {}", id);
        }
        
        Ok(deleted)
    }
}