use serde::{Serialize, Deserialize};
use crate::models::user::UserBasic;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub filter_type: Option<String>, // "topics", "posts", "users", "categories"
    pub filter_categories: Option<Vec<i64>>,
    pub filter_tags: Option<Vec<String>>,
    pub filter_date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub filter_date_to: Option<chrono::DateTime<chrono::Utc>>,
    pub filter_user_id: Option<i64>,
    pub sort_by: Option<String>, // "relevance", "newest", "oldest", "most_replies"
    pub page: usize,
    pub limit: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub result_type: String, // "topic", "post", "user", "category"
    pub id: i64,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub highlight: Option<String>, // HTML highlighted snippet
    pub url: String,
    pub category_id: Option<i64>,
    pub category_name: Option<String>,
    pub category_color: Option<String>,
    pub topic_id: Option<i64>,
    pub topic_title: Option<String>,
    pub author: Option<UserBasic>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reply_count: Option<i32>,
    pub view_count: Option<i32>,
    pub tags: Option<Vec<String>>,
    pub score: Option<f64>, // Search relevance score
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
    pub total: usize,
    pub page: usize,
    pub limit: usize,
    pub query: String,
    pub filters_applied: SearchFilters,
    pub execution_time_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchFilters {
    pub filter_type: Option<String>,
    pub filter_categories: Option<Vec<i64>>,
    pub filter_tags: Option<Vec<String>>,
    pub filter_date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub filter_date_to: Option<chrono::DateTime<chrono::Utc>>,
    pub filter_user_id: Option<i64>,
    pub sort_by: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchSuggestion {
    pub text: String,
    pub type_: String, // "topic", "user", "tag", "category"
    pub id: Option<i64>,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchStats {
    pub post_count: usize,
    pub topic_count: usize,
    pub user_count: usize,
    pub indexed_up_to: chrono::DateTime<chrono::Utc>,
}