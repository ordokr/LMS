use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub status: NotificationStatus,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationCreate {
    pub user_id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub reference_id: Option<String>,
    pub reference_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationStatus {
    Unread,
    Read,
}

impl ToString for NotificationStatus {
    fn to_string(&self) -> String {
        match self {
            NotificationStatus::Unread => "unread".to_string(),
            NotificationStatus::Read => "read".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    Discussion,
    Assignment,
    Submission,
    Grade,
    Announcement,
    CourseEnrollment,
    CourseUpdate,
    SystemMessage,
    DiscoursePost,
    DiscourseReply,
    DiscourseMessage,
}

impl ToString for NotificationType {
    fn to_string(&self) -> String {
        match self {
            NotificationType::Discussion => "discussion".to_string(),
            NotificationType::Assignment => "assignment".to_string(),
            NotificationType::Submission => "submission".to_string(),
            NotificationType::Grade => "grade".to_string(),
            NotificationType::Announcement => "announcement".to_string(),
            NotificationType::CourseEnrollment => "course_enrollment".to_string(),
            NotificationType::CourseUpdate => "course_update".to_string(),
            NotificationType::SystemMessage => "system_message".to_string(),
            NotificationType::DiscoursePost => "discourse_post".to_string(),
            NotificationType::DiscourseReply => "discourse_reply".to_string(),
            NotificationType::DiscourseMessage => "discourse_message".to_string(),
        }
    }
}