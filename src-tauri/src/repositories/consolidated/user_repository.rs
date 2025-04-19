use async_trait::async_trait;
use crate::errors::error::{Error, Result};
use crate::models::unified_models::User;
use super::base_repository::Repository;

/// User repository interface
#[async_trait]
pub trait UserRepository: Repository<User, String> {
    /// Find a user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    
    /// Find a user by username
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    
    /// Find a user by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<User>>;
    
    /// Find a user by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<User>>;
    
    /// Find users by role
    async fn find_by_role(&self, role: &str) -> Result<Vec<User>>;
    
    /// Add a role to a user
    async fn add_role(&self, user_id: &str, role: &str) -> Result<User>;
    
    /// Remove a role from a user
    async fn remove_role(&self, user_id: &str, role: &str) -> Result<User>;
    
    /// Update a user's last seen timestamp
    async fn update_last_seen(&self, user_id: &str) -> Result<()>;
    
    /// Authenticate a user with email/username and password
    async fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Option<User>>;
    
    /// Change a user's password
    async fn change_password(&self, user_id: &str, current_password: &str, new_password: &str) -> Result<()>;
    
    /// Get a user's preferences
    async fn get_preferences(&self, user_id: &str) -> Result<serde_json::Value>;
    
    /// Update a user's preferences
    async fn update_preferences(&self, user_id: &str, preferences: serde_json::Value) -> Result<serde_json::Value>;
}
