use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashSet;

/// Group model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    /// Unique identifier
    pub id: Uuid,
    /// Group name
    pub name: String,
    /// Group description
    pub description: Option<String>,
    /// Course ID
    pub course_id: Uuid,
    /// Group members
    pub members: HashSet<Uuid>,
    /// Group leader
    pub leader_id: Option<Uuid>,
    /// Created at
    pub created_at: DateTime<Utc>,
    /// Updated at
    pub updated_at: DateTime<Utc>,
    /// Canvas ID
    pub canvas_id: Option<String>,
    /// Discourse group ID
    pub discourse_group_id: Option<String>,
}

impl Group {
    /// Create a new group
    pub fn new(
        name: String,
        course_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            course_id,
            members: HashSet::new(),
            leader_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            canvas_id: None,
            discourse_group_id: None,
        }
    }
    
    /// Set description
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    
    /// Add member
    pub fn add_member(mut self, user_id: Uuid) -> Self {
        self.members.insert(user_id);
        self
    }
    
    /// Remove member
    pub fn remove_member(mut self, user_id: &Uuid) -> Self {
        self.members.remove(user_id);
        self
    }
    
    /// Set leader
    pub fn with_leader(mut self, user_id: Uuid) -> Self {
        self.leader_id = Some(user_id);
        self.members.insert(user_id);
        self
    }
    
    /// Set Canvas ID
    pub fn with_canvas_id(mut self, canvas_id: &str) -> Self {
        self.canvas_id = Some(canvas_id.to_string());
        self
    }
    
    /// Set Discourse group ID
    pub fn with_discourse_group_id(mut self, discourse_group_id: &str) -> Self {
        self.discourse_group_id = Some(discourse_group_id.to_string());
        self
    }
    
    /// Check if a user is a member of the group
    pub fn is_member(&self, user_id: &Uuid) -> bool {
        self.members.contains(user_id)
    }
    
    /// Check if a user is the leader of the group
    pub fn is_leader(&self, user_id: &Uuid) -> bool {
        self.leader_id.as_ref().map_or(false, |id| id == user_id)
    }
    
    /// Get the number of members
    pub fn member_count(&self) -> usize {
        self.members.len()
    }
}
