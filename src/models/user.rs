use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::services::api::{ApiClient, ApiError};

/// Represents a user in Canvas/Discourse
/// Based on Canvas's User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub sortable_name: Option<String>,
    pub short_name: Option<String>,
    pub sis_user_id: Option<String>,
    pub login_id: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub locale: Option<String>,
    pub effective_locale: Option<String>,
    pub time_zone: Option<String>,
    pub bio: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub roles: Option<Vec<String>>,
    pub preferences: Option<HashMap<String, String>>,
    
    // Discourse-specific fields
    pub username: Option<String>,
    pub trust_level: Option<i32>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub suspended_till: Option<DateTime<Utc>>,
}

impl User {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            sortable_name: None,
            short_name: None,
            sis_user_id: None,
            login_id: None,
            avatar_url: None,
            email: None,
            locale: None,
            effective_locale: None,
            time_zone: None,
            bio: None,
            title: None,
            created_at: None,
            roles: None,
            preferences: None,
            username: None,
            trust_level: None,
            suspended_at: None,
            suspended_till: None,
        }
    }
    
    /// Find a user by ID
    pub async fn find(api: &ApiClient, id: i64) -> Result<Self, ApiError> {
        api.get_user(id).await
    }
    
    /// Get current user (self)
    pub async fn current(api: &ApiClient) -> Result<Self, ApiError> {
        api.get_current_user().await
    }
    
    /// Get courses where this user is enrolled
    pub async fn courses(&self, api: &ApiClient) -> Result<Vec<crate::models::lms::Course>, ApiError> {
        api.get_user_courses(self.id).await
    }
    
    /// Check if the user has a specific role
    pub fn has_role(&self, role: &str) -> bool {
        match &self.roles {
            Some(roles) => roles.iter().any(|r| r == role),
            None => false,
        }
    }
    
    /// Check if the user is suspended
    pub fn is_suspended(&self) -> bool {
        let now = Utc::now();
        
        match (self.suspended_at, self.suspended_till) {
            (Some(_), Some(suspended_till)) => suspended_till > now,
            (Some(_), None) => true, // Suspended indefinitely
            _ => false,
        }
    }
    
    /// Get user's groups
    pub fn groups(&self) -> Vec<crate::models::forum::Group> {
        // Implementation would connect to backend service
        Vec::new()
    }
    
    /// Update user preferences
    pub fn update_preference(&mut self, key: &str, value: &str) -> Result<(), String> {
        match &mut self.preferences {
            Some(prefs) => {
                prefs.insert(key.to_string(), value.to_string());
                Ok(())
            },
            None => {
                let mut prefs = HashMap::new();
                prefs.insert(key.to_string(), value.to_string());
                self.preferences = Some(prefs);
                Ok(())
            }
        }
    }
}

/// Represents a badge that can be awarded to users
/// Based on Discourse's Badge model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub badge_type_id: Option<i32>,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub slug: Option<String>,
    pub multiple_grant: Option<bool>,
    pub enabled: Option<bool>,
    pub allow_title: Option<bool>,
    pub stackable: Option<bool>,
    pub show_posts: Option<bool>,
    pub system: Option<bool>,
    pub long_description: Option<String>,
    pub image: Option<String>,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: None,
            description: None,
            badge_type_id: None,
            icon: None,
            image_url: None,
            slug: None,
            multiple_grant: None,
            enabled: None,
            allow_title: None,
            stackable: None,
            show_posts: None,
            system: None,
            long_description: None,
            image: None,
        }
    }
    
    /// Find a badge by name
    pub fn find_by_name(name: &str) -> Option<Self> {
        // Implementation would connect to backend service
        None
    }
    
    /// Get users who have been awarded this badge
    pub fn user_count(&self) -> i32 {
        // Implementation would connect to backend service
        0
    }
    
    /// Award this badge to a user
    pub fn award_to(&self, user: &User, reason: Option<&str>) -> Result<bool, String> {
        // Implementation would connect to backend service
        Ok(true)
    }
    
    /// Revoke this badge from a user
    pub fn revoke_from(&self, user: &User) -> Result<bool, String> {
        // Implementation would connect to backend service
        Ok(true)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<Option<String>>, // Option<Option<String>> to handle null values
    pub location: Option<Option<String>>, // Option<Option<String>> to handle null values
}

// Add these structs to your user models

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    pub user_id: i64,
    
    // Interface preferences
    pub theme_preference: String, // "system", "light", "dark"
    pub homepage_view: String, // "latest", "top", "unread", "categories"
    pub posts_per_page: i32,
    pub compact_view: bool,
    pub highlight_new_content: bool,
    pub interface_language: String, // language code like "en", "fr", etc.
    
    // Email preferences
    pub enable_email_notifications: bool,
    pub notify_on_reply: bool,
    pub notify_on_mention: bool,
    pub notify_on_message: bool,
    pub digest_emails: String, // "none", "daily", "weekly"
    pub mailing_list_mode: bool,
    
    // Privacy preferences
    pub hide_profile: bool,
    pub hide_online_status: bool,
    pub allow_private_messages: bool,
    pub hide_activity: bool,
    
    // Content preferences
    pub auto_track_topics: bool,
    pub auto_watch_replied: bool,
    pub include_toc: bool,
    pub default_code_lang: String,
    pub link_previews: bool,
    pub embedded_media: bool,
    
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserPreferencesUpdate {
    // Interface preferences
    pub theme_preference: String,
    pub homepage_view: String,
    pub posts_per_page: i32,
    pub compact_view: bool,
    pub highlight_new_content: bool,
    pub interface_language: String,
    
    // Email preferences
    pub enable_email_notifications: bool,
    pub notify_on_reply: bool,
    pub notify_on_mention: bool,
    pub notify_on_message: bool,
    pub digest_emails: String,
    pub mailing_list_mode: bool,
    
    // Privacy preferences
    pub hide_profile: bool,
    pub hide_online_status: bool,
    pub allow_private messages: bool,
    pub hide_activity: bool,
    
    // Content preferences
    pub auto_track_topics: bool,
    pub auto_watch_replied: bool,
    pub include_toc: bool,
    pub default_code_lang: String,
    pub link_previews: bool,
    pub embedded_media: bool,
}

// Add these structs to your user models

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicSubscription {
    pub topic_id: i64,
    pub topic_title: String,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
    pub notification_level: String, // "watching", "tracking", "normal", "muted"
    pub unread_count: Option<i32>,
    pub last_activity_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarkedTopic {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
    pub post_id: Option<i64>,
    pub topic_title: String,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
    pub note: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}