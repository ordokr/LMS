use async_trait::async_trait;
use uuid::Uuid;
use crate::error::Error;
use crate::db::DB;

#[async_trait]
pub trait Repository<T> {
    /// Find an entity by its ID
    async fn find_by_id(&self, id: Uuid) -> Result<T, Error>;
    
    /// Find all entities
    async fn find_all(&self) -> Result<Vec<T>, Error>;
    
    /// Create a new entity
    async fn create(&self, entity: &T) -> Result<Uuid, Error>;
    
    /// Update an existing entity
    async fn update(&self, entity: &T) -> Result<(), Error>;
    
    /// Delete an entity by its ID
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

// Export forum repository
pub mod forum;

pub mod user_repository;
pub mod course_repository;
pub mod topic_repository;
pub mod post_repository;
pub mod assignment_repository;
pub mod submission_repository;
pub mod integration_repository;