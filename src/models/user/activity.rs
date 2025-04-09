use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserActivity {
    pub id: String,
    pub user_id: String,
    pub activity_type: ActivityType,
    pub target_id: String, // The ID of the thing being acted upon
    pub target_type: TargetType,
    pub data: Option<serde_json::Value>, // Additional context data
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityType {
    TopicCreated,
    TopicReplied,
    TopicLiked,
    PostLiked,
    UserFollowed,
    CategoryFollowed,
    BadgeAwarded,
    CourseEnrolled,
    CourseCompleted,
    ModuleCompleted,
    AssignmentSubmitted,
    QuizCompleted,
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