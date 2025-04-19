use async_trait::async_trait;
use crate::error::Error;
use crate::models::unified_models::{Group, GroupMembership, GroupMembershipStatus};
use super::repository::Repository;

/// Group repository interface
#[async_trait]
pub trait GroupRepository: Repository<Group, String> {
    /// Find a group by name
    async fn find_by_name(&self, name: &str) -> Result<Option<Group>, Error>;
    
    /// Find a group by Canvas ID
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Group>, Error>;
    
    /// Find a group by Discourse ID
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Group>, Error>;
    
    /// Find groups by context
    async fn find_by_context(&self, context_id: &str, context_type: &str) -> Result<Vec<Group>, Error>;
    
    /// Find groups by category
    async fn find_by_category(&self, category_id: &str) -> Result<Vec<Group>, Error>;
    
    /// Find groups by user
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<Group>, Error>;
    
    /// Find groups by user and context
    async fn find_by_user_and_context(&self, user_id: &str, context_id: &str, context_type: &str) -> Result<Vec<Group>, Error>;
    
    /// Add a user to a group
    async fn add_user_to_group(&self, group_id: &str, user_id: &str, status: GroupMembershipStatus) -> Result<GroupMembership, Error>;
    
    /// Remove a user from a group
    async fn remove_user_from_group(&self, group_id: &str, user_id: &str) -> Result<(), Error>;
    
    /// Update a user's membership status
    async fn update_membership_status(&self, group_id: &str, user_id: &str, status: GroupMembershipStatus) -> Result<GroupMembership, Error>;
    
    /// Set a user as a moderator
    async fn set_moderator(&self, group_id: &str, user_id: &str, is_moderator: bool) -> Result<GroupMembership, Error>;
    
    /// Get all memberships for a group
    async fn get_memberships(&self, group_id: &str) -> Result<Vec<GroupMembership>, Error>;
    
    /// Get a specific membership
    async fn get_membership(&self, group_id: &str, user_id: &str) -> Result<Option<GroupMembership>, Error>;
}
