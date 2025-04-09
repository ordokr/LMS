use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::user::User;
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Post {
    pub id: String,
    pub topic_id: String,
    pub user_id: String,
    pub post_number: i32,
    pub raw: String,
    pub html: String,
    pub cooked: String, // Processed content ready for display
    pub reply_to_post_id: Option<String>,
    pub updated_by_id: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub editor_id: Option<String>,
    pub like_count: i32,
    pub reads: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Associated data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PostRequest {
    pub topic_id: String,
    pub raw: String,
    pub reply_to_post_id: Option<String>,
}