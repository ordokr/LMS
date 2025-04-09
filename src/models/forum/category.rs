use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub position: i32,
    pub parent_category_id: Option<String>,
    pub course_id: Option<String>,
    pub module_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub topic_count: i32,
    pub post_count: i32,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub parent_category_id: Option<String>,
    pub course_id: Option<String>,
    pub module_id: Option<String>,
    pub position: Option<i32>,
}