use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub course_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub topic_count: i32,
    pub post_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub view_count: Option<i64>,
    pub reply_count: Option<i64>,
    pub last_post_date: Option<DateTime<Utc>>,
    pub last_poster: Option<User>,
    pub pinned: bool,
    pub locked: bool,
    pub tags: Option<Vec<String>>, // Tag names
    pub tag_objects: Option<Vec<Tag>>, // Full tag objects
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub is_solution: bool,
    pub parent_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Derived fields
    pub author_name: String,
    pub author_role: String,
    pub like_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumStats {
    pub total_posts: i64,
    pub total_topics: i64,
    pub total_users: i64,
    pub posts_today: i32,
    pub active_users_today: i32,
}

// Request/response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub category_id: i64,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub topic_id: i64,
    pub content: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub content: String,
}

// Add these new types to your models/forum.rs

/// Search result enum to represent different types of search results
#[derive(Debug, Clone)]
pub enum SearchResult {
    Topic(TopicSearchResult),
    Post(PostSearchResult),
    User(UserSearchResult),
}

#[derive(Debug, Clone)]
pub struct TopicSearchResult {
    pub id: i64,
    pub title: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub category_id: i64,
    pub category_name: Option<String>,
    pub reply_count: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PostSearchResult {
    pub id: i64,
    pub content: String,
    pub excerpt: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub author_id: i64,
    pub author_name: Option<String>,
    pub topic_id: i64,
    pub topic_title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserSearchResult {
    pub id: i64,
    pub name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub topic_count: Option<i64>,
    pub post_count: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicCreationRequest {
    pub title: String,
    pub content: String,
    pub category_id: i64,
    pub pinned: Option<bool>,
    pub locked: Option<bool>,
    pub tags: Option<Vec<String>>, // Tag names
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopicUpdateRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i64>,
    pub pinned: Option<bool>,
    pub locked: Option<bool>,
    pub tags: Option<Vec<String>>, // Tag names
}