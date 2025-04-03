use leptos::*;
use crate::models::forum::{
    Category, Topic, Post, ForumStats,
    CreateTopicRequest, CreatePostRequest, UpdatePostRequest
};
use crate::utils::errors::ApiError;
use crate::utils::api_client::ApiClient;

#[derive(Clone)]
pub struct ForumService {
    client: ApiClient,
}

impl ForumService {
    pub fn new() -> Self {
        Self {
            client: ApiClient::new(),
        }
    }
    
    // Categories
    pub async fn get_categories(&self) -> Result<Vec<Category>, ApiError> {
        self.client.get::<Vec<Category>>("/forum/categories").await
    }
    
    pub async fn get_category(&self, id: i64) -> Result<Category, ApiError> {
        self.client.get::<Category>(&format!("/forum/categories/{}", id)).await
    }
    
    pub async fn get_categories_by_course(&self, course_id: i64) -> Result<Vec<Category>, ApiError> {
        self.client.get::<Vec<Category>>(&format!("/forum/courses/{}/categories", course_id)).await
    }
    
    // Topics
    pub async fn get_topics(&self) -> Result<Vec<Topic>, ApiError> {
        self.client.get::<Vec<Topic>>("/forum/topics").await
    }
    
    pub async fn get_topics_by_category(&self, category_id: i64) -> Result<Vec<Topic>, ApiError> {
        self.client.get::<Vec<Topic>>(&format!("/forum/categories/{}/topics", category_id)).await
    }
    
    pub async fn get_topic(&self, id: i64) -> Result<Topic, ApiError> {
        self.client.get::<Topic>(&format!("/forum/topics/{}", id)).await
    }
    
    pub async fn create_topic(&self, request: CreateTopicRequest) -> Result<Topic, ApiError> {
        self.client.post::<CreateTopicRequest, Topic>("/forum/topics", request).await
    }
    
    // Posts
    pub async fn get_posts_by_topic(&self, topic_id: i64) -> Result<Vec<Post>, ApiError> {
        self.client.get::<Vec<Post>>(&format!("/forum/topics/{}/posts", topic_id)).await
    }
    
    pub async fn create_post(&self, request: CreatePostRequest) -> Result<Post, ApiError> {
        self.client.post::<CreatePostRequest, Post>(
            &format!("/forum/topics/{}/posts", request.topic_id), 
            request
        ).await
    }
    
    pub async fn update_post(&self, id: i64, content: String) -> Result<Post, ApiError> {
        let request = UpdatePostRequest { content };
        self.client.put::<UpdatePostRequest, Post>(&format!("/forum/posts/{}", id), request).await
    }
    
    pub async fn like_post(&self, id: i64) -> Result<Post, ApiError> {
        self.client.post::<(), Post>(&format!("/forum/posts/{}/like", id), ()).await
    }
    
    pub async fn get_stats(&self) -> Result<ForumStats, ApiError> {
        self.client.get::<ForumStats>("/forum/stats").await
    }
    
    /// Get recent forum activity across all categories
    pub async fn get_recent_activity(&self, limit: usize) -> Result<Vec<Topic>, ApiError> {
        let topics = self.client.get::<Vec<Topic>>("/forum/topics/recent").await?;
        
        // Sort by activity and limit
        let mut topics = topics;
        topics.sort_by(|a, b| {
            let a_date = a.last_post_at.unwrap_or(a.created_at);
            let b_date = b.last_post_at.unwrap_or(b.created_at);
            b_date.cmp(&a_date)
        });
        topics.truncate(limit);
        
        Ok(topics)
    }
}