use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

/// Group join level enum representing how users can join a group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupJoinLevel {
    /// Anyone can join the group
    Free,
    /// Users must request to join the group
    RequestToJoin,
    /// Users must be invited to join the group
    InvitationOnly,
    /// Users are automatically assigned to the group
    Automatic,
}

impl Default for GroupJoinLevel {
    fn default() -> Self {
        GroupJoinLevel::InvitationOnly
    }
}

impl std::fmt::Display for GroupJoinLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupJoinLevel::Free => write!(f, "free"),
            GroupJoinLevel::RequestToJoin => write!(f, "request"),
            GroupJoinLevel::InvitationOnly => write!(f, "invitation_only"),
            GroupJoinLevel::Automatic => write!(f, "automatic"),
        }
    }
}

impl From<&str> for GroupJoinLevel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "free" => GroupJoinLevel::Free,
            "request" | "request_to_join" => GroupJoinLevel::RequestToJoin,
            "invitation_only" | "invite_only" => GroupJoinLevel::InvitationOnly,
            "automatic" => GroupJoinLevel::Automatic,
            _ => GroupJoinLevel::InvitationOnly,
        }
    }
}

/// Group membership status enum representing the state of a user's membership in a group
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GroupMembershipStatus {
    /// User has accepted the group membership
    Accepted,
    /// User has been invited to the group
    Invited,
    /// User has requested to join the group
    Requested,
    /// User has been rejected from the group
    Rejected,
    /// User's membership has been deleted
    Deleted,
}

impl Default for GroupMembershipStatus {
    fn default() -> Self {
        GroupMembershipStatus::Accepted
    }
}

impl std::fmt::Display for GroupMembershipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GroupMembershipStatus::Accepted => write!(f, "accepted"),
            GroupMembershipStatus::Invited => write!(f, "invited"),
            GroupMembershipStatus::Requested => write!(f, "requested"),
            GroupMembershipStatus::Rejected => write!(f, "rejected"),
            GroupMembershipStatus::Deleted => write!(f, "deleted"),
        }
    }
}

impl From<&str> for GroupMembershipStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "accepted" => GroupMembershipStatus::Accepted,
            "invited" => GroupMembershipStatus::Invited,
            "requested" => GroupMembershipStatus::Requested,
            "rejected" => GroupMembershipStatus::Rejected,
            "deleted" => GroupMembershipStatus::Deleted,
            _ => GroupMembershipStatus::Accepted,
        }
    }
}

/// Group membership model representing a user's membership in a group
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMembership {
    /// Unique identifier
    pub id: String,
    /// Group ID
    pub group_id: String,
    /// User ID
    pub user_id: String,
    /// Membership status
    pub status: GroupMembershipStatus,
    /// Whether the user is a moderator
    pub is_moderator: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl GroupMembership {
    /// Create a new group membership
    pub fn new(
        group_id: String,
        user_id: String,
        status: GroupMembershipStatus,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            group_id,
            user_id,
            status,
            is_moderator: false,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Set moderator status
    pub fn set_moderator(&mut self, is_moderator: bool) {
        self.is_moderator = is_moderator;
        self.updated_at = Utc::now();
    }
    
    /// Update status
    pub fn update_status(&mut self, status: GroupMembershipStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// Check if the membership is active
    pub fn is_active(&self) -> bool {
        self.status == GroupMembershipStatus::Accepted
    }
}

/// Group model that harmonizes all existing group implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub name: String,                         // Group name
    pub description: Option<String>,          // Group description
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp
    
    // Context and category
    pub context_id: Option<String>,           // Context ID (course or account)
    pub context_type: Option<String>,         // Context type (Course or Account)
    pub group_category_id: Option<String>,    // Group category ID
    
    // Membership settings
    pub join_level: GroupJoinLevel,           // How users can join the group
    pub max_membership: Option<i32>,          // Maximum number of members
    pub is_public: bool,                      // Whether the group is public
    
    // External system IDs
    pub canvas_id: Option<String>,            // Canvas group ID
    pub discourse_id: Option<String>,         // Discourse group ID
    
    // Discourse-specific fields
    pub full_name: Option<String>,            // Full name (Discourse)
    pub visibility_level: Option<i32>,        // Visibility level (Discourse)
    pub mentionable_level: Option<i32>,       // Mentionable level (Discourse)
    pub messageable_level: Option<i32>,       // Messageable level (Discourse)
    pub automatic: bool,                      // Whether the group is automatic (Discourse)
    
    // Canvas-specific fields
    pub sis_source_id: Option<String>,        // SIS source ID (Canvas)
    pub storage_quota: Option<i64>,           // Storage quota in bytes (Canvas)
    pub default_view: Option<String>,         // Default view (Canvas)
    
    // Metadata and extensibility
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata
    
    // Memberships
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<GroupMembership>>, // Group memberships
}

impl Group {
    /// Create a new Group with default values
    pub fn new(
        id: Option<String>,
        name: String,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        Self {
            id,
            name,
            description: None,
            created_at: now,
            updated_at: now,
            context_id: None,
            context_type: None,
            group_category_id: None,
            join_level: GroupJoinLevel::InvitationOnly,
            max_membership: None,
            is_public: false,
            canvas_id: None,
            discourse_id: None,
            full_name: None,
            visibility_level: None,
            mentionable_level: None,
            messageable_level: None,
            automatic: false,
            sis_source_id: None,
            storage_quota: None,
            default_view: Some("feed".to_string()),
            source_system: None,
            metadata: HashMap::new(),
            memberships: Some(Vec::new()),
        }
    }
    
    /// Create a Group from a Canvas group JSON
    pub fn from_canvas_group(canvas_group: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let canvas_id = canvas_group["id"].as_str()
            .or_else(|| canvas_group["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let name = canvas_group["name"].as_str().unwrap_or("").to_string();
        let description = canvas_group["description"].as_str().map(|s| s.to_string());
        
        // Parse context
        let context_id = canvas_group["context_id"].as_str()
            .or_else(|| canvas_group["context_id"].as_i64().map(|id| id.to_string()));
        let context_type = canvas_group["context_type"].as_str().map(|s| s.to_string());
        
        // Parse group category
        let group_category_id = canvas_group["group_category_id"].as_str()
            .or_else(|| canvas_group["group_category_id"].as_i64().map(|id| id.to_string()));
        
        // Parse join level
        let join_level_str = canvas_group["join_level"].as_str().unwrap_or("invitation_only");
        let join_level = GroupJoinLevel::from(join_level_str);
        
        // Parse max membership
        let max_membership = canvas_group["max_membership"].as_i64().map(|m| m as i32);
        
        // Parse is_public
        let is_public = canvas_group["is_public"].as_bool().unwrap_or(false);
        
        // Parse SIS source ID
        let sis_source_id = canvas_group["sis_source_id"].as_str().map(|s| s.to_string());
        
        // Parse storage quota
        let storage_quota = canvas_group["storage_quota"].as_i64();
        
        // Parse default view
        let default_view = canvas_group["default_view"].as_str()
            .map(|s| s.to_string())
            .or_else(|| Some("feed".to_string()));
        
        // Convert the canvas_group to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_group).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
            context_id,
            context_type,
            group_category_id,
            join_level,
            max_membership,
            is_public,
            canvas_id: Some(canvas_id),
            discourse_id: None,
            full_name: None,
            visibility_level: None,
            mentionable_level: None,
            messageable_level: None,
            automatic: false,
            sis_source_id,
            storage_quota,
            default_view,
            source_system: Some("canvas".to_string()),
            metadata,
            memberships: Some(Vec::new()),
        }
    }
    
    /// Create a Group from a Discourse group JSON
    pub fn from_discourse_group(discourse_group: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let discourse_id = discourse_group["id"].as_str()
            .or_else(|| discourse_group["id"].as_i64().map(|id| id.to_string()))
            .unwrap_or_default();
        let name = discourse_group["name"].as_str().unwrap_or("").to_string();
        let description = discourse_group["description"].as_str().map(|s| s.to_string());
        let full_name = discourse_group["full_name"].as_str().map(|s| s.to_string());
        
        // Parse visibility level
        let visibility_level = discourse_group["visibility_level"].as_i64().map(|l| l as i32);
        
        // Parse mentionable level
        let mentionable_level = discourse_group["mentionable_level"].as_i64().map(|l| l as i32);
        
        // Parse messageable level
        let messageable_level = discourse_group["messageable_level"].as_i64().map(|l| l as i32);
        
        // Parse automatic
        let automatic = discourse_group["automatic"].as_bool().unwrap_or(false);
        
        // Convert the discourse_group to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_group).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
            context_id: None,
            context_type: None,
            group_category_id: None,
            join_level: GroupJoinLevel::InvitationOnly,
            max_membership: None,
            is_public: true,
            canvas_id: None,
            discourse_id: Some(discourse_id),
            full_name,
            visibility_level,
            mentionable_level,
            messageable_level,
            automatic,
            sis_source_id: None,
            storage_quota: None,
            default_view: Some("feed".to_string()),
            source_system: Some("discourse".to_string()),
            metadata,
            memberships: Some(Vec::new()),
        }
    }
    
    /// Convert Group to Canvas group JSON
    pub fn to_canvas_group(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.canvas_id,
            "name": self.name,
            "description": self.description,
            "context_id": self.context_id,
            "context_type": self.context_type,
            "group_category_id": self.group_category_id,
            "join_level": self.join_level.to_string(),
            "max_membership": self.max_membership,
            "is_public": self.is_public,
            "sis_source_id": self.sis_source_id,
            "storage_quota": self.storage_quota,
            "default_view": self.default_view,
            "members_count": self.memberships.as_ref().map(|m| m.len()).unwrap_or(0)
        })
    }
    
    /// Convert Group to Discourse group JSON
    pub fn to_discourse_group(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "name": self.name,
            "full_name": self.full_name,
            "description": self.description,
            "visibility_level": self.visibility_level,
            "mentionable_level": self.mentionable_level,
            "messageable_level": self.messageable_level,
            "automatic": self.automatic,
            "user_count": self.memberships.as_ref().map(|m| m.len()).unwrap_or(0)
        })
    }
    
    /// Add a member to the group
    pub fn add_member(&mut self, user_id: &str, status: GroupMembershipStatus) -> Option<&GroupMembership> {
        let memberships = self.memberships.get_or_insert(Vec::new());
        
        // Check if the user is already a member
        if let Some(membership) = memberships.iter_mut().find(|m| m.user_id == user_id) {
            membership.update_status(status);
            membership.updated_at = Utc::now();
            return Some(membership);
        }
        
        // Add the new membership
        let membership = GroupMembership::new(
            self.id.clone(),
            user_id.to_string(),
            status,
        );
        
        memberships.push(membership);
        self.updated_at = Utc::now();
        
        memberships.last()
    }
    
    /// Remove a member from the group
    pub fn remove_member(&mut self, user_id: &str) -> bool {
        if let Some(memberships) = self.memberships.as_mut() {
            let initial_len = memberships.len();
            memberships.retain(|m| m.user_id != user_id);
            
            if memberships.len() < initial_len {
                self.updated_at = Utc::now();
                return true;
            }
        }
        
        false
    }
    
    /// Get active members
    pub fn active_members(&self) -> Vec<String> {
        if let Some(memberships) = &self.memberships {
            memberships.iter()
                .filter(|m| m.is_active())
                .map(|m| m.user_id.clone())
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Get the number of active members
    pub fn active_member_count(&self) -> usize {
        if let Some(memberships) = &self.memberships {
            memberships.iter().filter(|m| m.is_active()).count()
        } else {
            0
        }
    }
    
    /// Check if the group is full
    pub fn is_full(&self) -> bool {
        if let Some(max) = self.max_membership {
            self.active_member_count() >= max as usize
        } else {
            false
        }
    }
    
    /// Check if a user is a member of the group
    pub fn is_member(&self, user_id: &str) -> bool {
        if let Some(memberships) = &self.memberships {
            memberships.iter().any(|m| m.user_id == user_id && m.is_active())
        } else {
            false
        }
    }
    
    /// Check if a user is a moderator of the group
    pub fn is_moderator(&self, user_id: &str) -> bool {
        if let Some(memberships) = &self.memberships {
            memberships.iter().any(|m| m.user_id == user_id && m.is_active() && m.is_moderator)
        } else {
            false
        }
    }
    
    /// Set a user as a moderator
    pub fn set_moderator(&mut self, user_id: &str, is_moderator: bool) -> bool {
        if let Some(memberships) = self.memberships.as_mut() {
            if let Some(membership) = memberships.iter_mut().find(|m| m.user_id == user_id && m.is_active()) {
                membership.set_moderator(is_moderator);
                self.updated_at = Utc::now();
                return true;
            }
        }
        
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_group() {
        let group = Group::new(
            None,
            "Test Group".to_string(),
        );
        
        assert_eq!(group.name, "Test Group");
        assert_eq!(group.join_level, GroupJoinLevel::InvitationOnly);
        assert_eq!(group.is_public, false);
        assert_eq!(group.default_view, Some("feed".to_string()));
        assert_eq!(group.active_member_count(), 0);
    }
    
    #[test]
    fn test_from_canvas_group() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "name": "Canvas Group",
            "description": "A group from Canvas",
            "context_id": "67890",
            "context_type": "Course",
            "group_category_id": "54321",
            "join_level": "request",
            "max_membership": 20,
            "is_public": true,
            "sis_source_id": "SIS123",
            "storage_quota": 1073741824,
            "default_view": "wiki"
        });
        
        let group = Group::from_canvas_group(&canvas_json);
        
        assert_eq!(group.name, "Canvas Group");
        assert_eq!(group.description, Some("A group from Canvas".to_string()));
        assert_eq!(group.context_id, Some("67890".to_string()));
        assert_eq!(group.context_type, Some("Course".to_string()));
        assert_eq!(group.group_category_id, Some("54321".to_string()));
        assert_eq!(group.join_level, GroupJoinLevel::RequestToJoin);
        assert_eq!(group.max_membership, Some(20));
        assert_eq!(group.is_public, true);
        assert_eq!(group.canvas_id, Some("12345".to_string()));
        assert_eq!(group.sis_source_id, Some("SIS123".to_string()));
        assert_eq!(group.storage_quota, Some(1073741824));
        assert_eq!(group.default_view, Some("wiki".to_string()));
        assert_eq!(group.source_system, Some("canvas".to_string()));
    }
    
    #[test]
    fn test_from_discourse_group() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "name": "Discourse Group",
            "description": "A group from Discourse",
            "full_name": "The Discourse Group",
            "visibility_level": 1,
            "mentionable_level": 2,
            "messageable_level": 1,
            "automatic": true
        });
        
        let group = Group::from_discourse_group(&discourse_json);
        
        assert_eq!(group.name, "Discourse Group");
        assert_eq!(group.description, Some("A group from Discourse".to_string()));
        assert_eq!(group.full_name, Some("The Discourse Group".to_string()));
        assert_eq!(group.visibility_level, Some(1));
        assert_eq!(group.mentionable_level, Some(2));
        assert_eq!(group.messageable_level, Some(1));
        assert_eq!(group.automatic, true);
        assert_eq!(group.discourse_id, Some("67890".to_string()));
        assert_eq!(group.source_system, Some("discourse".to_string()));
    }
    
    #[test]
    fn test_to_canvas_group() {
        let mut group = Group::new(
            Some("abcd1234".to_string()),
            "Test Canvas Group".to_string(),
        );
        
        group.canvas_id = Some("54321".to_string());
        group.description = Some("A test group for Canvas".to_string());
        group.context_id = Some("67890".to_string());
        group.context_type = Some("Course".to_string());
        group.group_category_id = Some("12345".to_string());
        group.join_level = GroupJoinLevel::Free;
        group.max_membership = Some(15);
        group.is_public = true;
        group.sis_source_id = Some("SIS456".to_string());
        group.storage_quota = Some(2147483648);
        group.default_view = Some("wiki".to_string());
        
        let canvas_group = group.to_canvas_group();
        
        assert_eq!(canvas_group["id"], "54321");
        assert_eq!(canvas_group["name"], "Test Canvas Group");
        assert_eq!(canvas_group["description"], "A test group for Canvas");
        assert_eq!(canvas_group["context_id"], "67890");
        assert_eq!(canvas_group["context_type"], "Course");
        assert_eq!(canvas_group["group_category_id"], "12345");
        assert_eq!(canvas_group["join_level"], "free");
        assert_eq!(canvas_group["max_membership"], 15);
        assert_eq!(canvas_group["is_public"], true);
        assert_eq!(canvas_group["sis_source_id"], "SIS456");
        assert_eq!(canvas_group["storage_quota"], 2147483648);
        assert_eq!(canvas_group["default_view"], "wiki");
        assert_eq!(canvas_group["members_count"], 0);
    }
    
    #[test]
    fn test_to_discourse_group() {
        let mut group = Group::new(
            Some("efgh5678".to_string()),
            "Test Discourse Group".to_string(),
        );
        
        group.discourse_id = Some("98765".to_string());
        group.description = Some("A test group for Discourse".to_string());
        group.full_name = Some("The Test Discourse Group".to_string());
        group.visibility_level = Some(2);
        group.mentionable_level = Some(3);
        group.messageable_level = Some(2);
        group.automatic = true;
        
        let discourse_group = group.to_discourse_group();
        
        assert_eq!(discourse_group["id"], "98765");
        assert_eq!(discourse_group["name"], "Test Discourse Group");
        assert_eq!(discourse_group["description"], "A test group for Discourse");
        assert_eq!(discourse_group["full_name"], "The Test Discourse Group");
        assert_eq!(discourse_group["visibility_level"], 2);
        assert_eq!(discourse_group["mentionable_level"], 3);
        assert_eq!(discourse_group["messageable_level"], 2);
        assert_eq!(discourse_group["automatic"], true);
        assert_eq!(discourse_group["user_count"], 0);
    }
    
    #[test]
    fn test_add_remove_member() {
        let mut group = Group::new(
            None,
            "Test Group".to_string(),
        );
        
        // Add a member
        let user_id = "user123";
        group.add_member(user_id, GroupMembershipStatus::Accepted);
        
        assert_eq!(group.active_member_count(), 1);
        assert!(group.is_member(user_id));
        
        // Remove the member
        let removed = group.remove_member(user_id);
        
        assert!(removed);
        assert_eq!(group.active_member_count(), 0);
        assert!(!group.is_member(user_id));
    }
    
    #[test]
    fn test_moderator() {
        let mut group = Group::new(
            None,
            "Test Group".to_string(),
        );
        
        // Add a member
        let user_id = "user123";
        group.add_member(user_id, GroupMembershipStatus::Accepted);
        
        assert!(!group.is_moderator(user_id));
        
        // Set as moderator
        let set = group.set_moderator(user_id, true);
        
        assert!(set);
        assert!(group.is_moderator(user_id));
        
        // Unset as moderator
        let unset = group.set_moderator(user_id, false);
        
        assert!(unset);
        assert!(!group.is_moderator(user_id));
    }
    
    #[test]
    fn test_is_full() {
        let mut group = Group::new(
            None,
            "Test Group".to_string(),
        );
        
        // Set max membership
        group.max_membership = Some(2);
        
        assert!(!group.is_full());
        
        // Add members
        group.add_member("user1", GroupMembershipStatus::Accepted);
        assert!(!group.is_full());
        
        group.add_member("user2", GroupMembershipStatus::Accepted);
        assert!(group.is_full());
        
        // Add another member (should still be full)
        group.add_member("user3", GroupMembershipStatus::Accepted);
        assert!(group.is_full());
        
        // Remove a member
        group.remove_member("user1");
        assert!(!group.is_full());
    }
    
    #[test]
    fn test_membership_status() {
        let mut group = Group::new(
            None,
            "Test Group".to_string(),
        );
        
        // Add members with different statuses
        group.add_member("user1", GroupMembershipStatus::Accepted);
        group.add_member("user2", GroupMembershipStatus::Invited);
        group.add_member("user3", GroupMembershipStatus::Requested);
        group.add_member("user4", GroupMembershipStatus::Rejected);
        
        assert_eq!(group.active_member_count(), 1);
        assert!(group.is_member("user1"));
        assert!(!group.is_member("user2"));
        assert!(!group.is_member("user3"));
        assert!(!group.is_member("user4"));
        
        // Update status
        if let Some(memberships) = group.memberships.as_mut() {
            if let Some(membership) = memberships.iter_mut().find(|m| m.user_id == "user2") {
                membership.update_status(GroupMembershipStatus::Accepted);
            }
        }
        
        assert_eq!(group.active_member_count(), 2);
        assert!(group.is_member("user2"));
    }
}
