use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
    pub topic_count: Option<i64>,
    pub post_count: Option<i64>,
    pub solution_count: Option<i64>,
    pub badges: Option<Vec<Badge>>,
    pub is_admin: bool,
    pub is_moderator: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Badge {
    pub name: String,
    pub description: String,
    pub icon: String, // Bootstrap icon name
    pub color: String, // Bootstrap color variant (primary, success, warning, etc.)
    pub earned_at: DateTime<Utc>,
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
    pub allow_private_messages: bool,
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