use crate::models::course::{Course, CourseStatus};
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
pub trait CourseRepository: Send + Sync {
    async fn get_courses(&self, status: Option<CourseStatus>) -> Result<Vec<Course>, DbError>;
    async fn get_course_by_id(&self, id: &str) -> Result<Option<Course>, DbError>;
    async fn create_course(&self, course: Course) -> Result<Course, DbError>;
    async fn update_course(&self, course: Course) -> Result<Course, DbError>;
    async fn delete_course(&self, id: &str) -> Result<bool, DbError>;
}

pub struct SqliteCourseRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCourseRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CourseRepository for SqliteCourseRepository {
    #[instrument(skip(self), err)]
    async fn get_courses(&self, status: Option<CourseStatus>) -> Result<Vec<Course>, DbError> {
        let mut query = sqlx::QueryBuilder::new("SELECT * FROM courses");
        
        if let Some(status_filter) = status {
            query.push(" WHERE status = ");
            query.push_bind(status_filter.to_string());
        }
        
        let courses = query.build_query_as::<Course>()
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch courses: {}", e);
                DbError::QueryError(e.to_string())
            })?;
            
        info!("Retrieved {} courses", courses.len());
        Ok(courses)
    }
    
    #[instrument(skip(self), err)]
    async fn get_course_by_id(&self, id: &str) -> Result<Option<Course>, DbError> {
        let course = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                error!("Failed to fetch course by ID {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        if course.is_some() {
            info!("Retrieved course with ID: {}", id);
        } else {
            info!("No course found with ID: {}", id);
        }
        
        Ok(course)
    }
    
    #[instrument(skip(self), fields(course_id = %course.id), err)]
    async fn create_course(&self, course: Course) -> Result<Course, DbError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Insert core course data
        let result = sqlx::query(
            "INSERT INTO courses (id, title, description, status, created_at, updated_at) 
             VALUES (?, ?, ?, ?, datetime('now'), datetime('now'))"
        )
        .bind(&course.id)
        .bind(&course.title)
        .bind(&course.description)
        .bind(course.status.to_string())
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to insert course: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        // Associate modules if they exist
        if let Some(modules) = &course.modules {
            for module_id in modules {
                sqlx::query(
                    "INSERT INTO course_modules (course_id, module_id) VALUES (?, ?)"
                )
                .bind(&course.id)
                .bind(module_id)
                .execute(&mut tx)
                .await
                .map_err(|e| {
                    error!("Failed to associate module {} with course {}: {}", 
                          module_id, course.id, e);
                    DbError::QueryError(e.to_string())
                })?;
            }
        }
        
        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Created new course with ID: {}", course.id);
        
        // Fetch the newly created course to ensure we return complete data
        self.get_course_by_id(&course.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve created course".to_string()))
    }
    
    #[instrument(skip(self), fields(course_id = %course.id), err)]
    async fn update_course(&self, course: Course) -> Result<Course, DbError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Update core course data
        let result = sqlx::query(
            "UPDATE courses 
             SET title = ?, description = ?, status = ?, updated_at = datetime('now')
             WHERE id = ?"
        )
        .bind(&course.title)
        .bind(&course.description)
        .bind(course.status.to_string())
        .bind(&course.id)
        .execute(&mut tx)
        .await
        .map_err(|e| {
            error!("Failed to update course: {}", e);
            DbError::QueryError(e.to_string())
        })?;
        
        if result.rows_affected() == 0 {
            return Err(DbError::DataError(format!("Course with ID {} not found", course.id)));
        }
        
        // Update modules if provided
        if let Some(modules) = &course.modules {
            // Remove existing module associations
            sqlx::query("DELETE FROM course_modules WHERE course_id = ?")
                .bind(&course.id)
                .execute(&mut tx)
                .await
                .map_err(|e| {
                    error!("Failed to remove existing modules for course {}: {}", 
                          course.id, e);
                    DbError::QueryError(e.to_string())
                })?;
            
            // Add new module associations
            for module_id in modules {
                sqlx::query(
                    "INSERT INTO course_modules (course_id, module_id) VALUES (?, ?)"
                )
                .bind(&course.id)
                .bind(module_id)
                .execute(&mut tx)
                .await
                .map_err(|e| {
                    error!("Failed to associate module {} with course {}: {}", 
                          module_id, course.id, e);
                    DbError::QueryError(e.to_string())
                })?;
            }
        }
        
        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        info!("Updated course with ID: {}", course.id);
        
        // Fetch the updated course
        self.get_course_by_id(&course.id)
            .await?
            .ok_or_else(|| DbError::DataError("Failed to retrieve updated course".to_string()))
    }
    
    #[instrument(skip(self), err)]
    async fn delete_course(&self, id: &str) -> Result<bool, DbError> {
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Remove module associations
        sqlx::query("DELETE FROM course_modules WHERE course_id = ?")
            .bind(id)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to remove module associations for course {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Delete the course
        let result = sqlx::query("DELETE FROM courses WHERE id = ?")
            .bind(id)
            .execute(&mut tx)
            .await
            .map_err(|e| {
                error!("Failed to delete course {}: {}", id, e);
                DbError::QueryError(e.to_string())
            })?;
        
        // Commit the transaction
        tx.commit()
            .await
            .map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            info!("Deleted course with ID: {}", id);
        } else {
            info!("No course found to delete with ID: {}", id);
        }
        
        Ok(deleted)
    }
}