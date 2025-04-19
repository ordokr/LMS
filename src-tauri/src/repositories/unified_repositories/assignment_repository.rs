use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::{Assignment, AssignmentStatus};
use super::repository::Repository;

/// Assignment repository interface
#[async_trait]
pub trait AssignmentRepository: Repository<Assignment, String> {
    /// Find assignments by course ID
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Find assignments by course ID and status
    async fn find_by_course_id_and_status(&self, course_id: &str, status: AssignmentStatus) -> Result<Vec<Assignment>, Error>;
    
    /// Find a assignment by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Assignment>, Error>;
    
    /// Find a assignment by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Assignment>, Error>;
    
    /// Find assignments by group category ID
    async fn find_by_group_category_id(&self, group_category_id: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Find assignments by assignment group ID
    async fn find_by_assignment_group_id(&self, assignment_group_id: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Find assignments by quiz ID
    async fn find_by_quiz_id(&self, quiz_id: &str) -> Result<Option<Assignment>, Error>;
    
    /// Find assignments by discussion topic ID
    async fn find_by_discussion_topic_id(&self, discussion_topic_id: &str) -> Result<Option<Assignment>, Error>;
    
    /// Find assignments due between dates
    async fn find_by_due_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Find overdue assignments for a course
    async fn find_overdue_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Find upcoming assignments for a course
    async fn find_upcoming_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error>;
    
    /// Publish an assignment
    async fn publish(&self, id: &str) -> Result<Assignment, Error>;
    
    /// Unpublish an assignment
    async fn unpublish(&self, id: &str) -> Result<Assignment, Error>;
    
    /// Delete an assignment
    async fn delete_assignment(&self, id: &str) -> Result<(), Error>;
}
