use async_trait::async_trait;
use reqwest::{Client, Method};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::collections::HashMap;

use super::base_client::{ApiClient, ApiClientConfig, ApiError, Result, PaginationParams, PaginatedResponse};
use crate::models::unified_models::{User, Topic, Group};

/// Discourse API client
#[derive(Debug, Clone)]
pub struct DiscourseApiClient {
    /// Base API client configuration
    config: ApiClientConfig,
    
    /// HTTP client
    client: Client,
    
    /// API username
    api_username: String,
}

/// Discourse notification model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseNotification {
    /// Notification ID
    pub id: String,
    
    /// Creation timestamp
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    
    /// Whether the notification has been read
    pub read: bool,
    
    /// Notification type
    #[serde(rename = "notification_type")]
    pub notification_type: String,
    
    /// Additional fields
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Discourse category model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseCategory {
    /// Category ID
    pub id: i64,
    
    /// Category name
    pub name: String,
    
    /// Category slug
    pub slug: String,
    
    /// Category description
    pub description: Option<String>,
    
    /// Category color
    pub color: Option<String>,
    
    /// Category text color
    pub text_color: Option<String>,
    
    /// Parent category ID
    pub parent_category_id: Option<i64>,
    
    /// Topic count
    pub topic_count: Option<i64>,
    
    /// Post count
    pub post_count: Option<i64>,
    
    /// Position
    pub position: Option<i64>,
}

/// Discourse post model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoursePost {
    /// Post ID
    pub id: i64,
    
    /// Topic ID
    pub topic_id: i64,
    
    /// User ID
    pub user_id: i64,
    
    /// Post number
    pub post_number: i64,
    
    /// Raw content
    pub raw: String,
    
    /// Cooked content (HTML)
    pub cooked: String,
    
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    
    /// Update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Reply count
    pub reply_count: i64,
    
    /// Quote count
    pub quote_count: i64,
    
    /// Like count
    pub like_count: i64,
    
    /// Incoming link count
    pub incoming_link_count: i64,
    
    /// Reads
    pub reads: i64,
    
    /// Score
    pub score: f64,
    
    /// Whether the post is yours
    pub yours: bool,
    
    /// Whether the post is a moderator action
    pub moderator: bool,
    
    /// Whether the post is staff
    pub staff: bool,
    
    /// Username
    pub username: String,
    
    /// Avatar template
    pub avatar_template: String,
}

impl DiscourseApiClient {
    /// Create a new Discourse API client
    pub fn new(base_url: &str, api_key: &str, api_username: &str) -> Result<Self> {
        // Create client builder
        let client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(std::time::Duration::from_secs(60))
            .tcp_keepalive(std::time::Duration::from_secs(60))
            .gzip(true)
            .brotli(true);
            
        // Build the client
        let client = client_builder.build()
            .map_err(ApiError::HttpError)?;
            
        // Create configuration
        let config = ApiClientConfig {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            timeout_seconds: 30,
            max_retries: 3,
            use_exponential_backoff: true,
            enable_circuit_breaker: true,
            max_connections_per_host: 10,
            enable_compression: true,
            additional_headers: Vec::new(),
            api_username: Some(api_username.to_string()),
        };
        
        Ok(Self {
            config,
            client,
            api_username: api_username.to_string(),
        })
    }
    
    /// Add API key and username to query parameters
    fn add_auth_params<'a>(&self, params: Option<&'a [(&'a str, &'a str)]>) -> Vec<(&'a str, &'a str)> {
        let mut auth_params = Vec::new();
        
        // Add API key and username
        auth_params.push(("api_key", self.config.api_key.as_str()));
        auth_params.push(("api_username", self.api_username.as_str()));
        
        // Add existing params
        if let Some(params) = params {
            auth_params.extend_from_slice(params);
        }
        
        auth_params
    }
    
    /// Override the get method to add API key and username
    async fn discourse_get<T>(&self, path: &str, params: Option<&[(&str, &str)]>) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let auth_params = self.add_auth_params(params);
        ApiClient::get(self, path, Some(&auth_params)).await
    }
    
    /// Override the post method to add API key and username
    async fn discourse_post<D, T>(&self, path: &str, data: Option<&D>, params: Option<&[(&str, &str)]>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let auth_params = self.add_auth_params(params);
        ApiClient::post(self, path, data, Some(&auth_params)).await
    }
    
    /// Override the put method to add API key and username
    async fn discourse_put<D, T>(&self, path: &str, data: Option<&D>, params: Option<&[(&str, &str)]>) -> Result<T>
    where
        D: Serialize + Send + Sync + ?Sized,
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let auth_params = self.add_auth_params(params);
        ApiClient::put(self, path, data, Some(&auth_params)).await
    }
    
    /// Override the delete method to add API key and username
    async fn discourse_delete<T>(&self, path: &str, params: Option<&[(&str, &str)]>) -> Result<T>
    where
        T: serde::de::DeserializeOwned + Send + 'static,
    {
        let auth_params = self.add_auth_params(params);
        ApiClient::delete(self, path, Some(&auth_params)).await
    }
    
    /// Get user notifications
    pub async fn get_user_notifications(&self, discourse_user_id: &str) -> Result<Vec<DiscourseNotification>> {
        let path = format!("/users/{}/notifications.json", discourse_user_id);
        
        #[derive(Deserialize)]
        struct NotificationsResponse {
            notifications: Vec<DiscourseNotification>,
        }
        
        let response: NotificationsResponse = self.discourse_get(&path, None).await?;
        Ok(response.notifications)
    }
    
    /// Mark a notification as read
    pub async fn mark_notification_as_read(&self, notification_id: &str) -> Result<DiscourseNotification> {
        let path = format!("/notifications/{}/mark-read.json", notification_id);
        
        #[derive(Serialize)]
        struct MarkReadRequest {}
        
        let request = MarkReadRequest {};
        self.discourse_put(&path, Some(&request), None).await
    }
    
    /// Create a notification
    pub async fn create_notification(&self, notification_data: &serde_json::Value) -> Result<DiscourseNotification> {
        // Discourse doesn't have a direct API for creating notifications
        // This is a stub implementation
        Err(ApiError::ApiError {
            status_code: reqwest::StatusCode::NOT_IMPLEMENTED,
            message: "Discourse doesn't support creating notifications directly".to_string(),
        })
    }
    
    /// Get a user by ID
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        let path = format!("/users/{}.json", user_id);
        
        #[derive(Deserialize)]
        struct UserResponse {
            user: serde_json::Value,
        }
        
        let response: UserResponse = self.discourse_get(&path, None).await?;
        Ok(User::from_discourse_user(&response.user))
    }
    
    /// Get a user by username
    pub async fn get_user_by_username(&self, username: &str) -> Result<User> {
        let path = format!("/users/{}.json", username);
        
        #[derive(Deserialize)]
        struct UserResponse {
            user: serde_json::Value,
        }
        
        let response: UserResponse = self.discourse_get(&path, None).await?;
        Ok(User::from_discourse_user(&response.user))
    }
    
    /// Get a list of categories
    pub async fn get_categories(&self) -> Result<Vec<DiscourseCategory>> {
        let path = "/categories.json";
        
        #[derive(Deserialize)]
        struct CategoriesResponse {
            category_list: CategoryList,
        }
        
        #[derive(Deserialize)]
        struct CategoryList {
            categories: Vec<DiscourseCategory>,
        }
        
        let response: CategoriesResponse = self.discourse_get(path, None).await?;
        Ok(response.category_list.categories)
    }
    
    /// Get a category by ID
    pub async fn get_category(&self, category_id: &str) -> Result<DiscourseCategory> {
        let path = format!("/c/{}.json", category_id);
        
        #[derive(Deserialize)]
        struct CategoryResponse {
            category: DiscourseCategory,
        }
        
        let response: CategoryResponse = self.discourse_get(&path, None).await?;
        Ok(response.category)
    }
    
    /// Get a topic by ID
    pub async fn get_topic(&self, topic_id: &str) -> Result<Topic> {
        let path = format!("/t/{}.json", topic_id);
        
        let discourse_topic: serde_json::Value = self.discourse_get(&path, None).await?;
        Ok(Topic::from_discourse_topic(&discourse_topic))
    }
    
    /// Get a list of topics for a category
    pub async fn get_category_topics(&self, category_id: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<Topic>> {
        let path = format!("/c/{}.json", category_id);
        
        let mut params = Vec::new();
        if let Some(page) = pagination.page {
            params.push(("page", page.to_string().as_str()));
        }
        
        #[derive(Deserialize)]
        struct TopicsResponse {
            topic_list: TopicList,
        }
        
        #[derive(Deserialize)]
        struct TopicList {
            topics: Vec<serde_json::Value>,
            more_topics_url: Option<String>,
        }
        
        let response: TopicsResponse = self.discourse_get(&path, Some(&params)).await?;
        
        // Convert Discourse topics to unified Topic models
        let topics = response.topic_list.topics.iter()
            .map(|topic| Topic::from_discourse_topic(topic))
            .collect();
            
        Ok(PaginatedResponse {
            items: topics,
            total: None,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: None,
            next_cursor: None,
            prev_cursor: None,
            has_next: response.topic_list.more_topics_url.is_some(),
            has_prev: pagination.page.map(|p| p > 1).unwrap_or(false),
        })
    }
    
    /// Get posts for a topic
    pub async fn get_topic_posts(&self, topic_id: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<DiscoursePost>> {
        let path = format!("/t/{}/posts.json", topic_id);
        
        let mut params = Vec::new();
        if let Some(page) = pagination.page {
            params.push(("page", page.to_string().as_str()));
        }
        
        #[derive(Deserialize)]
        struct PostsResponse {
            post_stream: PostStream,
        }
        
        #[derive(Deserialize)]
        struct PostStream {
            posts: Vec<DiscoursePost>,
            stream: Vec<i64>,
        }
        
        let response: PostsResponse = self.discourse_get(&path, Some(&params)).await?;
        
        Ok(PaginatedResponse {
            items: response.post_stream.posts,
            total: Some(response.post_stream.stream.len() as u64),
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages: None,
            next_cursor: None,
            prev_cursor: None,
            has_next: pagination.page.map(|p| (p as usize) * pagination.per_page.unwrap_or(20) as usize < response.post_stream.stream.len()).unwrap_or(false),
            has_prev: pagination.page.map(|p| p > 1).unwrap_or(false),
        })
    }
    
    /// Create a post in a topic
    pub async fn create_post(&self, topic_id: &str, raw_content: &str) -> Result<DiscoursePost> {
        let path = "/posts.json";
        
        #[derive(Serialize)]
        struct CreatePostRequest {
            topic_id: String,
            raw: String,
        }
        
        let request = CreatePostRequest {
            topic_id: topic_id.to_string(),
            raw: raw_content.to_string(),
        };
        
        self.discourse_post(&path, Some(&request), None).await
    }
    
    /// Get a list of groups
    pub async fn get_groups(&self) -> Result<Vec<Group>> {
        let path = "/groups.json";
        
        #[derive(Deserialize)]
        struct GroupsResponse {
            groups: Vec<serde_json::Value>,
        }
        
        let response: GroupsResponse = self.discourse_get(path, None).await?;
        
        // Convert Discourse groups to unified Group models
        let groups = response.groups.iter()
            .map(|group| Group::from_discourse_group(group))
            .collect();
            
        Ok(groups)
    }
    
    /// Get a group by name
    pub async fn get_group(&self, group_name: &str) -> Result<Group> {
        let path = format!("/groups/{}.json", group_name);
        
        #[derive(Deserialize)]
        struct GroupResponse {
            group: serde_json::Value,
        }
        
        let response: GroupResponse = self.discourse_get(&path, None).await?;
        Ok(Group::from_discourse_group(&response.group))
    }
    
    /// Get members of a group
    pub async fn get_group_members(&self, group_name: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<User>> {
        let path = format!("/groups/{}/members.json", group_name);
        
        let mut params = Vec::new();
        if let Some(page) = pagination.page {
            params.push(("page", page.to_string().as_str()));
        }
        
        if let Some(per_page) = pagination.per_page {
            params.push(("limit", per_page.to_string().as_str()));
        }
        
        #[derive(Deserialize)]
        struct MembersResponse {
            members: Vec<serde_json::Value>,
            meta: Meta,
        }
        
        #[derive(Deserialize)]
        struct Meta {
            total: Option<u64>,
            limit: Option<u32>,
            offset: Option<u32>,
        }
        
        let response: MembersResponse = self.discourse_get(&path, Some(&params)).await?;
        
        // Convert Discourse users to unified User models
        let users = response.members.iter()
            .map(|user| User::from_discourse_user(user))
            .collect();
            
        Ok(PaginatedResponse {
            items: users,
            total: response.meta.total,
            page: pagination.page,
            per_page: response.meta.limit,
            total_pages: response.meta.total.zip(response.meta.limit).map(|(total, limit)| ((total + limit as u64 - 1) / limit as u64) as u32),
            next_cursor: None,
            prev_cursor: None,
            has_next: response.meta.offset.zip(response.meta.limit).zip(response.meta.total).map(|((offset, limit), total)| (offset + limit) < total as u32).unwrap_or(false),
            has_prev: response.meta.offset.map(|offset| offset > 0).unwrap_or(false),
        })
    }
}

#[async_trait]
impl ApiClient for DiscourseApiClient {
    fn get_config(&self) -> &ApiClientConfig {
        &self.config
    }
    
    fn get_http_client(&self) -> &Client {
        &self.client
    }
}

/// Create a new Discourse API client
pub fn create_discourse_client(base_url: &str, api_key: &str, api_username: &str) -> Result<Arc<DiscourseApiClient>> {
    let client = DiscourseApiClient::new(base_url, api_key, api_username)?;
    Ok(Arc::new(client))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_get_user() {
        let _m = mock("GET", "/users/testuser.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"user":{"id":123,"username":"testuser","name":"Test User","email":"test@example.com"}}"#)
            .create();
            
        let client = DiscourseApiClient::new(&server_url(), "test_key", "test_admin").unwrap();
        let user = client.get_user_by_username("testuser").await.unwrap();
        
        assert_eq!(user.username, "testuser");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[tokio::test]
    async fn test_get_categories() {
        let _m = mock("GET", "/categories.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"category_list":{"categories":[{"id":1,"name":"Test Category","slug":"test-category","description":"Test description"}]}}"#)
            .create();
            
        let client = DiscourseApiClient::new(&server_url(), "test_key", "test_admin").unwrap();
        let categories = client.get_categories().await.unwrap();
        
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, "Test Category");
        assert_eq!(categories[0].slug, "test-category");
        assert_eq!(categories[0].description, Some("Test description".to_string()));
    }
    
    #[tokio::test]
    async fn test_get_topic() {
        let _m = mock("GET", "/t/123.json")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("api_key".into(), "test_key".into()),
                mockito::Matcher::UrlEncoded("api_username".into(), "test_admin".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":123,"title":"Test Topic","created_at":"2023-01-01T00:00:00Z","category_id":1,"user_id":456,"posts_count":5,"views":100,"like_count":10,"tags":["test","topic"]}"#)
            .create();
            
        let client = DiscourseApiClient::new(&server_url(), "test_key", "test_admin").unwrap();
        let topic = client.get_topic("123").await.unwrap();
        
        assert_eq!(topic.title, "Test Topic");
        assert_eq!(topic.category_id, Some("1".to_string()));
        assert_eq!(topic.author_id, Some("456".to_string()));
        assert_eq!(topic.view_count, Some(100));
        assert_eq!(topic.tags, vec!["test".to_string(), "topic".to_string()]);
    }
}
