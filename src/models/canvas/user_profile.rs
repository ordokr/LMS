use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// UserProfile model - ported from Canvas
/// Represents a user's profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    // Core fields
    pub id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub short_name: Option<String>,
    pub sortable_name: Option<String>,
    pub title: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_state: Option<String>,
    pub pronouns: Option<String>,
    pub locale: Option<String>,
    pub time_zone: Option<String>,
    
    // Canvas-specific fields
    pub canvas_id: Option<String>,
    pub lti_user_id: Option<String>,
    pub integration_id: Option<String>,
    pub login_id: Option<String>,
    pub primary_email: Option<String>,
    pub public: bool,
    
    // Discourse-specific fields
    pub discourse_id: Option<i64>,
    pub username: Option<String>,
    pub trust_level: Option<i32>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub badge_count: Option<i32>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub last_emailed_at: Option<DateTime<Utc>>,
    
    // Common fields
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    /// Create a new user profile
    pub fn new(user_id: Uuid, name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Some(Uuid::new_v4()),
            user_id,
            name,
            short_name: None,
            sortable_name: None,
            title: None,
            bio: None,
            avatar_url: None,
            avatar_state: None,
            pronouns: None,
            locale: None,
            time_zone: None,
            canvas_id: None,
            lti_user_id: None,
            integration_id: None,
            login_id: None,
            primary_email: None,
            public: false,
            discourse_id: None,
            username: None,
            trust_level: None,
            website: None,
            location: None,
            badge_count: None,
            last_seen_at: None,
            last_posted_at: None,
            last_emailed_at: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create a user profile from Canvas data
    pub fn from_canvas(
        user_id: Uuid,
        name: String,
        canvas_id: String,
        login_id: Option<String>,
        primary_email: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        let mut profile = Self::new(user_id, name);
        profile.canvas_id = Some(canvas_id);
        profile.login_id = login_id;
        profile.primary_email = primary_email;
        profile.avatar_url = avatar_url;
        profile
    }
    
    /// Create a user profile from Discourse data
    pub fn from_discourse(
        user_id: Uuid,
        name: String,
        discourse_id: i64,
        username: String,
        avatar_url: Option<String>,
        bio: Option<String>,
        website: Option<String>,
        location: Option<String>,
    ) -> Self {
        let mut profile = Self::new(user_id, name);
        profile.discourse_id = Some(discourse_id);
        profile.username = Some(username);
        profile.avatar_url = avatar_url;
        profile.bio = bio;
        profile.website = website;
        profile.location = location;
        profile
    }
    
    /// Update profile with Canvas data
    pub fn update_from_canvas(
        &mut self,
        name: Option<String>,
        login_id: Option<String>,
        primary_email: Option<String>,
        avatar_url: Option<String>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if login_id.is_some() {
            self.login_id = login_id;
        }
        if primary_email.is_some() {
            self.primary_email = primary_email;
        }
        if avatar_url.is_some() {
            self.avatar_url = avatar_url;
        }
        self.updated_at = Utc::now();
    }
    
    /// Update profile with Discourse data
    pub fn update_from_discourse(
        &mut self,
        username: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        website: Option<String>,
        location: Option<String>,
    ) {
        if username.is_some() {
            self.username = username;
        }
        if avatar_url.is_some() {
            self.avatar_url = avatar_url;
        }
        if bio.is_some() {
            self.bio = bio;
        }
        if website.is_some() {
            self.website = website;
        }
        if location.is_some() {
            self.location = location;
        }
        self.updated_at = Utc::now();
    }
}
