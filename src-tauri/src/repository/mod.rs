mod forum_category_repository;
mod forum_topic_repository;
mod forum_post_repository;
mod course_repository;
mod user_repository;
mod integration_repository;
// ... other repositories

pub use forum_category_repository::ForumCategoryRepository;
pub use forum_topic_repository::ForumTopicRepository;
pub use forum_post_repository::ForumPostRepository;
pub use course_repository::CourseRepository;
pub use user_repository::UserRepository;
pub use integration_repository::IntegrationRepository;
// ... other exports

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Entity not found")]
    NotFound,
    
    #[error("Failed to insert entity")]
    InsertFailed,
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}