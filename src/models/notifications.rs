use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Notification {
    pub id: String,
    pub notification_type: NotificationType,
    pub user_id: String,
    pub actor_id: Option<String>, // Person who triggered notification (if applicable)
    pub target_id: String,        // The ID of the thing being notified about
    pub target_type: TargetType,
    pub data: Option<serde_json::Value>, // Additional context data
    pub read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Mentioned,          // When user is @mentioned
    Replied,            // When someone replies to user's post
    Liked,              // When user's content is liked
    PrivateMessage,     // PM received
    BadgeAwarded,       // User received badge
    TopicUpdated,       // Topic user follows was updated
    Announcement,       // Course/system announcement
    AssignmentGraded,   // Assignment was graded
    DueDateReminder,    // Upcoming due date
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Topic,
    Post,
    User,
    Category,
    Badge,
    Course,
    Module,
    Assignment,
    Quiz,
}

/// A user-friendly notification summary with processed information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotificationSummary {
    pub id: String,
    pub notification_type: NotificationType,
    pub title: String,
    pub body: String,
    pub url: Option<String>,
    pub actor_name: Option<String>,
    pub actor_avatar: Option<String>,
    pub read: bool,
    pub created_at: String,
}