use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumCategory {
    pub id: Option<i64>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub course_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub id: Option<i64>,
    pub category_id: i64,
    pub title: String,
    pub slug: String,
    pub user_id: i64,
    pub pinned: bool,
    pub locked: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_post_at: Option<String>,
    pub view_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: Option<i64>,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub is_solution: bool,
    pub parent_id: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumUserPreferences {
    pub id: Option<i64>,
    pub user_id: i64,
    pub email_on_reply: bool,
    pub email_on_mention: bool,
    pub email_digest: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTrustLevel {
    pub id: Option<i64>,
    pub user_id: i64,
    pub trust_level: i32,
    pub posts_read: i32,
    pub posts_created: i32,
    pub updated_at: Option<String>,
}