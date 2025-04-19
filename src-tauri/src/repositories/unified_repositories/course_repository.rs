use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::{Course, CourseStatus};
use super::repository::Repository;

/// Course repository interface
#[async_trait]
pub trait CourseRepository: Repository<Course, String> {
    /// Find a course by code
    async fn find_by_code(&self, code: &str) -> Result<Option<Course>, Error>;
    
    /// Find a course by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Course>, Error>;
    
    /// Find a course by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Course>, Error>;
    
    /// Find courses by instructor ID
    async fn find_by_instructor_id(&self, instructor_id: &str) -> Result<Vec<Course>, Error>;
    
    /// Find courses by status
    async fn find_by_status(&self, status: CourseStatus) -> Result<Vec<Course>, Error>;
    
    /// Find active courses
    async fn find_active_courses(&self) -> Result<Vec<Course>, Error>;
    
    /// Find archived courses
    async fn find_archived_courses(&self) -> Result<Vec<Course>, Error>;
    
    /// Activate a course
    async fn activate_course(&self, course_id: &str) -> Result<Course, Error>;
    
    /// Archive a course
    async fn archive_course(&self, course_id: &str) -> Result<Course, Error>;
    
    /// Delete a course
    async fn delete_course(&self, course_id: &str) -> Result<(), Error>;
}
