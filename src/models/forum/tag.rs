use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub topic_count: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_restricted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagWithTopics {
    pub tag: Tag,
    pub recent_topics: Vec<crate::models::forum::Topic>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_restricted: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>, // Option<Option<String>> for null values
    pub color: Option<Option<String>>,
    pub icon: Option<Option<String>>,
    pub is_restricted: Option<bool>,
}