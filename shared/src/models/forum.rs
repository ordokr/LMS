use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumCategory {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub id: Option<i64>,
    pub title: String,
    pub category_id: i64,
    pub user_id: i64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: Option<i64>,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: String,
}
