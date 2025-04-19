use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::{Submission, SubmissionStatus, SubmissionContentType};
use super::repository::Repository;

/// Submission repository interface
#[async_trait]
pub trait SubmissionRepository: Repository<Submission, String> {
    /// Find submissions by assignment ID
    async fn find_by_assignment_id(&self, assignment_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by user ID
    async fn find_by_user_id(&self, user_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by assignment ID and user ID
    async fn find_by_assignment_and_user(&self, assignment_id: &str, user_id: &str) -> Result<Option<Submission>, Error>;
    
    /// Find submissions by course ID
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by course ID and user ID
    async fn find_by_course_and_user(&self, course_id: &str, user_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by status
    async fn find_by_status(&self, status: SubmissionStatus) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by submission type
    async fn find_by_submission_type(&self, submission_type: SubmissionContentType) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Submission>, Error>;
    
    /// Find submissions by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Submission>, Error>;
    
    /// Find late submissions
    async fn find_late_submissions(&self, assignment_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find missing submissions
    async fn find_missing_submissions(&self, assignment_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Find submissions that need grading
    async fn find_needs_grading(&self, assignment_id: &str) -> Result<Vec<Submission>, Error>;
    
    /// Submit a submission
    async fn submit(&self, id: &str) -> Result<Submission, Error>;
    
    /// Grade a submission
    async fn grade(&self, id: &str, grader_id: &str, grade: &str, score: Option<f64>) -> Result<Submission, Error>;
    
    /// Return a submission to the student
    async fn return_to_student(&self, id: &str) -> Result<Submission, Error>;
    
    /// Mark a submission as late
    async fn mark_late(&self, id: &str) -> Result<Submission, Error>;
    
    /// Mark a submission as missing
    async fn mark_missing(&self, id: &str) -> Result<Submission, Error>;
    
    /// Excuse a submission
    async fn excuse(&self, id: &str) -> Result<Submission, Error>;
    
    /// Add a comment to a submission
    async fn add_comment(&self, id: &str, author_id: &str, comment: &str) -> Result<Submission, Error>;
    
    /// Add an attachment to a submission
    async fn add_attachment(&self, id: &str, attachment_id: &str) -> Result<Submission, Error>;
    
    /// Remove an attachment from a submission
    async fn remove_attachment(&self, id: &str, attachment_id: &str) -> Result<Submission, Error>;
}
