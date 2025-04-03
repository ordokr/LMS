use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub notification_type: NotificationType,
    pub data: NotificationData,
    pub created_at: DateTime<Utc>,
    pub read: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Reply,          // Someone replied to your topic/post
    Mention,        // Someone mentioned you in a post
    Quote,          // Someone quoted your post
    Like,           // Someone liked your post
    Solution,       // Your answer was marked as a solution
    Welcome,        // Welcome to the forum (new user notification)
    Message,        // Direct message
    System,         // System notification (e.g., maintenance)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationData {
    // Common fields across all notification types
    pub title: String,
    pub message: String,
    pub link: String,
    
    // Fields for specific notification types
    pub sender_id: Option<i64>,
    pub sender_name: Option<String>,
    pub sender_avatar: Option<String>,
    
    // Topic/post-related fields
    pub topic_id: Option<i64>,
    pub topic_title: Option<String>,
    pub post_id: Option<i64>,
    pub post_excerpt: Option<String>,
    
    // For system messages
    pub action_text: Option<String>,
    pub action_link: Option<String>,
    pub icon: Option<String>,          // Bootstrap icon class
    pub color: Option<String>,         // Bootstrap color class (e.g., "primary", "warning")
}