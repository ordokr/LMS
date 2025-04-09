use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::utils::date_utils::{serialize_optional_date, deserialize_optional_date};
use crate::models::forum::category::Category;
use crate::models::forum::post::Post;
use crate::models::user::User;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Topic {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub category_id: String,
    pub user_id: String,
    pub closed: bool,
    pub pinned: bool,
    pub pinned_globally: bool,
    pub visible: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub views: i32,
    pub posts_count: i32,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub bumped_at: DateTime<Utc>,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub highest_post_number: i32,
    pub excerpt: Option<String>,
    
    // Associated data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<Post>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TopicRequest {
    pub title: String,
    pub category_id: String,
    pub raw: String, // Content of the first post
    pub tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct TopicSummary {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub category_id: String,
    pub user_id: String,
    pub closed: bool,
    pub pinned: bool,
    pub visible: bool,
    pub created_at: DateTime<Utc>,
    pub posts_count: i32,
    pub views: i32,
    pub last_posted_at: Option<DateTime<Utc>>,
    pub excerpt: Option<String>,
    pub user_display_name: String,
    pub category_name: String,
}