use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserProfile {
    pub user_id: String,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub tag_line: Option<String>,
    pub profile_views: i32,
    pub trust_level: i32,
    pub is_moderator: bool,
    pub is_admin: bool,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub created_topics_count: i32,
    pub posts_count: i32,
    pub likes_given: i32,
    pub likes_received: i32,
    pub featured_topic_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserProfileUpdate {
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
    pub title: Option<String>,
    pub tag_line: Option<String>,
}