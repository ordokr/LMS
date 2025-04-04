use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Represents a notification in Canvas
/// Based on Canvas's Notification model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub notification_type: Option<String>,
    pub read: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub user_id: Option<i64>,
    pub workflow_state: Option<String>,
    pub context_type: Option<String>,
    pub context_id: Option<i64>,
    pub url: Option<String>,
}

impl Notification {
    pub fn new() -> Self {
        Self {
            id: 0,
            subject: None,
            message: None,
            notification_type: None,
            read: None,
            created_at: None,
            user_id: None,
            workflow_state: None,
            context_type: None,
            context_id: None,
            url: None,
        }
    }
    
    /// Mark notification as read
    pub fn mark_as_read(&mut self) -> Result<bool, String> {
        self.read = Some(true);
        // Implementation would connect to backend service to persist
        Ok(true)
    }
    
    /// Mark notification as unread
    pub fn mark_as_unread(&mut self) -> Result<bool, String> {
        self.read = Some(false);
        // Implementation would connect to backend service to persist
        Ok(true)
    }
    
    /// Delete this notification
    pub fn delete(&mut self) -> Result<bool, String> {
        self.workflow_state = Some("deleted".to_string());
        // Implementation would connect to backend service to persist
        Ok(true)
    }
    
    /// Get the target object this notification refers to
    pub fn get_context(&self) -> Result<serde_json::Value, String> {
        // Implementation would fetch the appropriate context based on context_type and context_id
        Err("Not implemented".to_string())
    }
}

/// Notification delivery preferences for users
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    pub id: i64,
    pub user_id: Option<i64>,
    pub notification_type: Option<String>,
    pub frequency: Option<String>, // "immediately", "daily", "weekly", "never"
    pub communication_channel_id: Option<i64>,
}

impl NotificationPreference {
    pub fn new() -> Self {
        Self {
            id: 0,
            user_id: None,
            notification_type: None,
            frequency: None,
            communication_channel_id: None,
        }
    }
    
    /// Update the frequency for this notification preference
    pub fn update_frequency(&mut self, frequency: &str) -> Result<bool, String> {
        if ["immediately", "daily", "weekly", "never"].contains(&frequency) {
            self.frequency = Some(frequency.to_string());
            // Implementation would connect to backend service to persist
            Ok(true)
        } else {
            Err("Invalid frequency value".to_string())
        }
    }
    
    /// Find all preferences for a user
    pub fn find_all_for_user(user_id: i64) -> Vec<Self> {
        // Implementation would connect to backend service
        Vec::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub user_id: i64,
    pub enable_browser_notifications: bool,
    pub enable_email_notifications: bool,
    pub mentions_notification: bool,
    pub replies_notification: bool,
    pub quotes_notification: bool,
    pub likes_notification: bool,
    pub messages_notification: bool,
    pub follows_notification: bool,
    pub group_mentions_notification: bool,
    pub group_messages_notification: bool,
    pub digest_emails: String, // "never", "daily", "weekly"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSummary {
    pub total_count: i32,
    pub unread_count: i32,
    pub mention_count: i32,
    pub message_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationData {
    pub title: Option<String>,
    pub message: String,
    pub topic_id: Option<i64>,
    pub topic_title: Option<String>,
    pub post_id: Option<i64>,
    pub post_number: Option<i32>,
    pub from_user_id: Option<i64>,
    pub from_username: Option<String>,
    pub from_user_avatar: Option<String>,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
    pub tag_name: Option<String>,
    pub badge_id: Option<i64>,
    pub badge_name: Option<String>,
    pub badge_icon: Option<String>,
    // Additional fields that might be specific to certain notification types
    pub reaction_type: Option<String>,
    pub group_id: Option<i64>,
    pub group_name: Option<String>,
    pub old_category_id: Option<i64>,
    pub old_category_name: Option<String>,
    pub user_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub user_id: i64,
    pub email_notifications_enabled: bool,
    pub browser_notifications_enabled: bool,
    pub push_notifications_enabled: bool,
    pub notification_types: Vec<NotificationType>,
    pub quiet_hours_enabled: bool,
    pub quiet_hours_start: Option<String>, // Format: "HH:MM"
    pub quiet_hours_end: Option<String>,   // Format: "HH:MM"
    pub digest_frequency: DigestFrequency,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigestFrequency {
    Never,
    Daily,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Reply,
    Mention,
    Quote,
    PrivateMessage,
    GroupMention,
    Reaction,
    TopicCreated,
    AdminNotification,
    PostEdited,
    TopicMoved,
    BadgeAwarded,
    WelcomeNotification,
    SystemNotification,
    TaggedUser,
}