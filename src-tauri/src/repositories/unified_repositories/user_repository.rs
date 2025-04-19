use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::User;
use super::repository::Repository;

/// User repository interface
#[async_trait]
pub trait UserRepository: Repository<User, String> {
    /// Find a user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    
    /// Find a user by username
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error>;
    
    /// Find a user by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<User>, Error>;
    
    /// Find a user by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<User>, Error>;
    
    /// Find users by role
    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, Error>;
    
    /// Add a role to a user
    async fn add_role(&self, user_id: &str, role: &str) -> Result<User, Error>;
    
    /// Remove a role from a user
    async fn remove_role(&self, user_id: &str, role: &str) -> Result<User, Error>;
    
    /// Update a user's last seen timestamp
    async fn update_last_seen(&self, user_id: &str) -> Result<(), Error>;
}
