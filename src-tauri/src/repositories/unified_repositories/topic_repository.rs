use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::{Topic, TopicStatus, TopicType};
use super::repository::Repository;

/// Topic repository interface
#[async_trait]
pub trait TopicRepository: Repository<Topic, String> {
    /// Find topics by course ID
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by course ID and status
    async fn find_by_course_id_and_status(&self, course_id: &str, status: TopicStatus) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by group ID
    async fn find_by_group_id(&self, group_id: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by category ID
    async fn find_by_category_id(&self, category_id: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by author ID
    async fn find_by_author_id(&self, author_id: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by assignment ID
    async fn find_by_assignment_id(&self, assignment_id: &str) -> Result<Option<Topic>, Error>;
    
    /// Find a topic by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Topic>, Error>;
    
    /// Find a topic by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Topic>, Error>;
    
    /// Find topics by type
    async fn find_by_type(&self, topic_type: TopicType) -> Result<Vec<Topic>, Error>;
    
    /// Find topics by tag
    async fn find_by_tag(&self, tag: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find pinned topics for a course
    async fn find_pinned_by_course_id(&self, course_id: &str) -> Result<Vec<Topic>, Error>;
    
    /// Find recent topics for a course
    async fn find_recent_by_course_id(&self, course_id: &str, limit: i64) -> Result<Vec<Topic>, Error>;
    
    /// Open a topic
    async fn open(&self, id: &str) -> Result<Topic, Error>;
    
    /// Close a topic
    async fn close(&self, id: &str) -> Result<Topic, Error>;
    
    /// Archive a topic
    async fn archive(&self, id: &str) -> Result<Topic, Error>;
    
    /// Delete a topic
    async fn delete_topic(&self, id: &str) -> Result<(), Error>;
    
    /// Pin a topic
    async fn pin(&self, id: &str) -> Result<Topic, Error>;
    
    /// Unpin a topic
    async fn unpin(&self, id: &str) -> Result<Topic, Error>;
    
    /// Add a tag to a topic
    async fn add_tag(&self, id: &str, tag: &str) -> Result<Topic, Error>;
    
    /// Remove a tag from a topic
    async fn remove_tag(&self, id: &str, tag: &str) -> Result<Topic, Error>;
    
    /// Increment view count for a topic
    async fn increment_view_count(&self, id: &str) -> Result<Topic, Error>;
    
    /// Increment reply count for a topic
    async fn increment_reply_count(&self, id: &str) -> Result<Topic, Error>;
}
