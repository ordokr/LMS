use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Unified User model that harmonizes all existing user implementations
/// This model is designed to be compatible with both Canvas and Discourse user models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    // Core fields
    pub id: String,                           // Primary identifier (UUID)
    pub name: String,                         // Full name
    pub email: String,                        // Email address
    pub username: String,                     // Username for login
    pub avatar_url: Option<String>,           // URL to avatar image
    pub created_at: DateTime<Utc>,            // Creation timestamp
    pub updated_at: DateTime<Utc>,            // Last update timestamp
    pub last_seen_at: Option<DateTime<Utc>>,  // Last activity timestamp
    
    // Role and permission fields
    pub roles: Vec<String>,                   // List of roles (student, teacher, admin, etc.)
    pub trust_level: Option<i32>,             // Forum trust level (Discourse concept)
    pub is_admin: bool,                       // Admin flag
    pub is_moderator: bool,                   // Moderator flag
    
    // External system IDs
    pub canvas_id: Option<String>,            // Canvas user ID
    pub discourse_id: Option<String>,         // Discourse user ID
    pub sis_id: Option<String>,               // SIS user ID
    pub lti_id: Option<String>,               // LTI user ID
    
    // Profile fields
    pub bio: Option<String>,                  // User biography
    pub location: Option<String>,             // User location
    pub website: Option<String>,              // User website
    pub timezone: Option<String>,             // User timezone
    
    // Canvas-specific fields
    pub sortable_name: Option<String>,        // Name for sorting
    pub short_name: Option<String>,           // Short/display name
    
    // Discourse-specific fields
    pub post_count: Option<i32>,              // Number of forum posts
    
    // Metadata and extensibility
    pub source_system: Option<String>,        // Source system (canvas, discourse, etc.)
    pub metadata: HashMap<String, serde_json::Value>, // Extensible metadata
}

impl User {
    /// Create a new User with default values
    pub fn new(
        id: Option<String>,
        name: String,
        email: String,
        username: String,
    ) -> Self {
        let now = Utc::now();
        let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
        
        Self {
            id,
            name,
            email,
            username,
            avatar_url: None,
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            roles: vec!["student".to_string()], // Default role
            trust_level: Some(0),
            is_admin: false,
            is_moderator: false,
            canvas_id: None,
            discourse_id: None,
            sis_id: None,
            lti_id: None,
            bio: None,
            location: None,
            website: None,
            timezone: None,
            sortable_name: None,
            short_name: None,
            post_count: None,
            source_system: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a User from a Canvas user JSON
    pub fn from_canvas_user(canvas_user: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let canvas_id = canvas_user["id"].as_str().map(|s| s.to_string());
        let name = canvas_user["name"].as_str().unwrap_or("").to_string();
        let email = canvas_user["email"]
            .as_str()
            .or_else(|| canvas_user["login_id"].as_str())
            .unwrap_or("")
            .to_string();
        let username = canvas_user["login_id"].as_str().unwrap_or("").to_string();
        let avatar_url = canvas_user["avatar_url"].as_str().map(|s| s.to_string());
        
        // Extract roles from enrollments if available
        let mut roles = Vec::new();
        if let Some(enrollments) = canvas_user["enrollments"].as_array() {
            for enrollment in enrollments {
                if let Some(role) = enrollment["type"].as_str() {
                    roles.push(role.to_lowercase());
                }
            }
        }
        
        // If no roles extracted, use default
        if roles.is_empty() {
            roles.push("student".to_string());
        }
        
        // Extract additional fields
        let sortable_name = canvas_user["sortable_name"].as_str().map(|s| s.to_string());
        let short_name = canvas_user["short_name"].as_str().map(|s| s.to_string());
        let sis_id = canvas_user["sis_user_id"].as_str().map(|s| s.to_string());
        let lti_id = canvas_user["lti_user_id"].as_str().map(|s| s.to_string());
        
        // Convert the canvas_user to a HashMap for metadata
        let metadata = serde_json::to_value(canvas_user).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            email,
            username,
            avatar_url,
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            roles,
            trust_level: Some(0),
            is_admin: false,
            is_moderator: false,
            canvas_id,
            discourse_id: None,
            sis_id,
            lti_id,
            bio: None,
            location: None,
            website: None,
            timezone: None,
            sortable_name,
            short_name,
            post_count: None,
            source_system: Some("canvas".to_string()),
            metadata,
        }
    }
    
    /// Create a User from a Discourse user JSON
    pub fn from_discourse_user(discourse_user: &serde_json::Value) -> Self {
        let id = Uuid::new_v4().to_string();
        let discourse_id = discourse_user["id"].as_str().map(|s| s.to_string());
        let name = discourse_user["name"].as_str().unwrap_or("").to_string();
        let email = discourse_user["email"].as_str().unwrap_or("").to_string();
        let username = discourse_user["username"].as_str().unwrap_or("").to_string();
        
        // Handle avatar URL - Discourse uses avatar_template
        let avatar_url = if let Some(avatar_template) = discourse_user["avatar_template"].as_str() {
            Some(avatar_template.replace("{size}", "120"))
        } else {
            None
        };
        
        // Extract last seen timestamp
        let last_seen_at = discourse_user["last_seen_at"]
            .as_str()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        
        // Extract trust level
        let trust_level = discourse_user["trust_level"].as_i64().map(|l| l as i32);
        
        // Extract post count
        let post_count = discourse_user["post_count"].as_i64().map(|c| c as i32);
        
        // Extract admin and moderator status
        let is_admin = discourse_user["admin"].as_bool().unwrap_or(false);
        let is_moderator = discourse_user["moderator"].as_bool().unwrap_or(false);
        
        // Extract roles from groups if available
        let mut roles = Vec::new();
        if let Some(groups) = discourse_user["groups"].as_array() {
            for group in groups {
                if let Some(name) = group["name"].as_str() {
                    roles.push(name.to_lowercase());
                }
            }
        }
        
        // If no roles extracted, use default
        if roles.is_empty() {
            roles.push("member".to_string());
        }
        
        // Extract bio and website
        let bio = discourse_user["bio_raw"].as_str().map(|s| s.to_string());
        let website = discourse_user["website"].as_str().map(|s| s.to_string());
        let location = discourse_user["location"].as_str().map(|s| s.to_string());
        
        // Convert the discourse_user to a HashMap for metadata
        let metadata = serde_json::to_value(discourse_user).ok()
            .and_then(|v| serde_json::from_value::<HashMap<String, serde_json::Value>>(v).ok())
            .unwrap_or_default();
        
        let now = Utc::now();
        
        Self {
            id,
            name,
            email,
            username,
            avatar_url,
            created_at: now,
            updated_at: now,
            last_seen_at,
            roles,
            trust_level,
            is_admin,
            is_moderator,
            canvas_id: None,
            discourse_id,
            sis_id: None,
            lti_id: None,
            bio,
            location,
            website,
            timezone: None,
            sortable_name: None,
            short_name: None,
            post_count,
            source_system: Some("discourse".to_string()),
            metadata,
        }
    }
    
    /// Convert User to Canvas user JSON
    pub fn to_canvas_user(&self) -> serde_json::Value {
        // Determine enrollment type from roles
        let enrollment_type = self.roles.first()
            .map(|r| r.as_str())
            .unwrap_or("student");
            
        serde_json::json!({
            "id": self.canvas_id,
            "name": self.name,
            "email": self.email,
            "login_id": self.username,
            "avatar_url": self.avatar_url,
            "sortable_name": self.sortable_name,
            "short_name": self.short_name,
            "sis_user_id": self.sis_id,
            "lti_user_id": self.lti_id,
            "enrollments": [
                { "type": enrollment_type }
            ]
        })
    }
    
    /// Convert User to Discourse user JSON
    pub fn to_discourse_user(&self) -> serde_json::Value {
        serde_json::json!({
            "id": self.discourse_id,
            "name": self.name,
            "email": self.email,
            "username": self.username,
            "avatar_template": self.avatar_url,
            "last_seen_at": self.last_seen_at,
            "trust_level": self.trust_level,
            "admin": self.is_admin,
            "moderator": self.is_moderator,
            "post_count": self.post_count,
            "bio_raw": self.bio,
            "website": self.website,
            "location": self.location
        })
    }
    
    /// Check if user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
    
    /// Add a role to the user
    pub fn add_role(&mut self, role: &str) {
        if !self.has_role(role) {
            self.roles.push(role.to_string());
        }
    }
    
    /// Remove a role from the user
    pub fn remove_role(&mut self, role: &str) {
        self.roles.retain(|r| r != role);
    }
    
    /// Get full name or username if name is empty
    pub fn display_name(&self) -> &str {
        if self.name.is_empty() {
            &self.username
        } else {
            &self.name
        }
    }
    
    /// Get short name, full name, or username in that order of preference
    pub fn preferred_name(&self) -> String {
        if let Some(short) = &self.short_name {
            if !short.is_empty() {
                return short.clone();
            }
        }
        
        if !self.name.is_empty() {
            return self.name.clone();
        }
        
        self.username.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_user() {
        let user = User::new(
            None,
            "John Doe".to_string(),
            "john@example.com".to_string(),
            "johndoe".to_string(),
        );
        
        assert_eq!(user.name, "John Doe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.roles, vec!["student".to_string()]);
        assert_eq!(user.trust_level, Some(0));
        assert_eq!(user.is_admin, false);
    }
    
    #[test]
    fn test_from_canvas_user() {
        let canvas_json = serde_json::json!({
            "id": "12345",
            "name": "Jane Smith",
            "email": "jane@example.com",
            "login_id": "janesmith",
            "avatar_url": "https://example.com/avatar.jpg",
            "sortable_name": "Smith, Jane",
            "short_name": "Jane",
            "sis_user_id": "SIS123",
            "enrollments": [
                { "type": "Teacher" }
            ]
        });
        
        let user = User::from_canvas_user(&canvas_json);
        
        assert_eq!(user.name, "Jane Smith");
        assert_eq!(user.email, "jane@example.com");
        assert_eq!(user.username, "janesmith");
        assert_eq!(user.avatar_url, Some("https://example.com/avatar.jpg".to_string()));
        assert_eq!(user.canvas_id, Some("12345".to_string()));
        assert_eq!(user.roles, vec!["teacher".to_string()]);
        assert_eq!(user.sortable_name, Some("Smith, Jane".to_string()));
        assert_eq!(user.short_name, Some("Jane".to_string()));
        assert_eq!(user.sis_id, Some("SIS123".to_string()));
        assert_eq!(user.source_system, Some("canvas".to_string()));
    }
    
    #[test]
    fn test_from_discourse_user() {
        let discourse_json = serde_json::json!({
            "id": "67890",
            "name": "Bob Johnson",
            "email": "bob@example.com",
            "username": "bobjohnson",
            "avatar_template": "https://example.com/avatar/{size}.jpg",
            "last_seen_at": "2023-01-01T12:00:00Z",
            "trust_level": 2,
            "admin": true,
            "moderator": false,
            "post_count": 42,
            "bio_raw": "Hello, I'm Bob",
            "website": "https://bob.example.com",
            "location": "New York",
            "groups": [
                { "name": "Moderator" },
                { "name": "Staff" }
            ]
        });
        
        let user = User::from_discourse_user(&discourse_json);
        
        assert_eq!(user.name, "Bob Johnson");
        assert_eq!(user.email, "bob@example.com");
        assert_eq!(user.username, "bobjohnson");
        assert_eq!(user.avatar_url, Some("https://example.com/avatar/120.jpg".to_string()));
        assert_eq!(user.discourse_id, Some("67890".to_string()));
        assert_eq!(user.trust_level, Some(2));
        assert_eq!(user.is_admin, true);
        assert_eq!(user.is_moderator, false);
        assert_eq!(user.post_count, Some(42));
        assert_eq!(user.bio, Some("Hello, I'm Bob".to_string()));
        assert_eq!(user.website, Some("https://bob.example.com".to_string()));
        assert_eq!(user.location, Some("New York".to_string()));
        assert_eq!(user.roles, vec!["moderator".to_string(), "staff".to_string()]);
        assert_eq!(user.source_system, Some("discourse".to_string()));
    }
    
    #[test]
    fn test_to_canvas_user() {
        let mut user = User::new(
            Some("abcd1234".to_string()),
            "Alice Brown".to_string(),
            "alice@example.com".to_string(),
            "alicebrown".to_string(),
        );
        
        user.canvas_id = Some("54321".to_string());
        user.avatar_url = Some("https://example.com/alice.jpg".to_string());
        user.roles = vec!["teacher".to_string()];
        user.sortable_name = Some("Brown, Alice".to_string());
        user.short_name = Some("Alice".to_string());
        user.sis_id = Some("SIS456".to_string());
        
        let canvas_user = user.to_canvas_user();
        
        assert_eq!(canvas_user["id"], "54321");
        assert_eq!(canvas_user["name"], "Alice Brown");
        assert_eq!(canvas_user["email"], "alice@example.com");
        assert_eq!(canvas_user["login_id"], "alicebrown");
        assert_eq!(canvas_user["avatar_url"], "https://example.com/alice.jpg");
        assert_eq!(canvas_user["sortable_name"], "Brown, Alice");
        assert_eq!(canvas_user["short_name"], "Alice");
        assert_eq!(canvas_user["sis_user_id"], "SIS456");
        assert_eq!(canvas_user["enrollments"][0]["type"], "teacher");
    }
    
    #[test]
    fn test_to_discourse_user() {
        let mut user = User::new(
            Some("efgh5678".to_string()),
            "Charlie Davis".to_string(),
            "charlie@example.com".to_string(),
            "charliedavis".to_string(),
        );
        
        user.discourse_id = Some("98765".to_string());
        user.avatar_url = Some("https://example.com/charlie.jpg".to_string());
        user.last_seen_at = Some(DateTime::parse_from_rfc3339("2023-02-01T15:30:00Z").unwrap().with_timezone(&Utc));
        user.trust_level = Some(3);
        user.is_admin = false;
        user.is_moderator = true;
        user.post_count = Some(123);
        user.bio = Some("Hello, I'm Charlie".to_string());
        user.website = Some("https://charlie.example.com".to_string());
        user.location = Some("Los Angeles".to_string());
        
        let discourse_user = user.to_discourse_user();
        
        assert_eq!(discourse_user["id"], "98765");
        assert_eq!(discourse_user["name"], "Charlie Davis");
        assert_eq!(discourse_user["email"], "charlie@example.com");
        assert_eq!(discourse_user["username"], "charliedavis");
        assert_eq!(discourse_user["avatar_template"], "https://example.com/charlie.jpg");
        assert_eq!(discourse_user["trust_level"], 3);
        assert_eq!(discourse_user["admin"], false);
        assert_eq!(discourse_user["moderator"], true);
        assert_eq!(discourse_user["post_count"], 123);
        assert_eq!(discourse_user["bio_raw"], "Hello, I'm Charlie");
        assert_eq!(discourse_user["website"], "https://charlie.example.com");
        assert_eq!(discourse_user["location"], "Los Angeles");
    }
    
    #[test]
    fn test_has_role() {
        let mut user = User::new(
            None,
            "Test User".to_string(),
            "test@example.com".to_string(),
            "testuser".to_string(),
        );
        
        user.roles = vec!["student".to_string(), "ta".to_string()];
        
        assert!(user.has_role("student"));
        assert!(user.has_role("ta"));
        assert!(!user.has_role("teacher"));
        assert!(!user.has_role("admin"));
    }
    
    #[test]
    fn test_add_remove_role() {
        let mut user = User::new(
            None,
            "Test User".to_string(),
            "test@example.com".to_string(),
            "testuser".to_string(),
        );
        
        // Default role is student
        assert!(user.has_role("student"));
        
        // Add a role
        user.add_role("teacher");
        assert!(user.has_role("teacher"));
        assert_eq!(user.roles.len(), 2);
        
        // Add the same role again (should not duplicate)
        user.add_role("teacher");
        assert_eq!(user.roles.len(), 2);
        
        // Remove a role
        user.remove_role("student");
        assert!(!user.has_role("student"));
        assert_eq!(user.roles.len(), 1);
        
        // Remove a non-existent role (should be no-op)
        user.remove_role("admin");
        assert_eq!(user.roles.len(), 1);
    }
    
    #[test]
    fn test_display_name() {
        let mut user = User::new(
            None,
            "Full Name".to_string(),
            "email@example.com".to_string(),
            "username".to_string(),
        );
        
        assert_eq!(user.display_name(), "Full Name");
        
        user.name = "".to_string();
        assert_eq!(user.display_name(), "username");
    }
    
    #[test]
    fn test_preferred_name() {
        let mut user = User::new(
            None,
            "Full Name".to_string(),
            "email@example.com".to_string(),
            "username".to_string(),
        );
        
        assert_eq!(user.preferred_name(), "Full Name");
        
        user.short_name = Some("Short".to_string());
        assert_eq!(user.preferred_name(), "Short");
        
        user.short_name = Some("".to_string());
        assert_eq!(user.preferred_name(), "Full Name");
        
        user.name = "".to_string();
        assert_eq!(user.preferred_name(), "username");
    }
}
