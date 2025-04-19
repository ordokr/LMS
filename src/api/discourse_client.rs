use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::{Result, Context, anyhow};
use log::{info, error, debug};
use reqwest::{Client, header};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::api::discourse_models::*;
use crate::api::base_client::{ApiClient, ApiClientConfig};

/// Discourse API client interface
#[async_trait]
pub trait DiscourseApi: Send + Sync {
    /// Get a user by ID
    async fn get_user(&self, id: &str) -> Result<Value>;

    /// Get a user by external ID
    async fn get_user_by_external_id(&self, external_id: &str) -> Result<Value>;

    /// Create a new user
    async fn create_user(&self, data: &Value) -> Result<Value>;

    /// Update a user
    async fn update_user(&self, id: &str, data: &Value) -> Result<Value>;

    /// Deactivate a user
    async fn deactivate_user(&self, id: &str) -> Result<()>;

    /// Get a category by ID
    async fn get_category(&self, id: &str) -> Result<Value>;

    /// Get a category by custom field
    async fn get_category_by_custom_field(&self, field: &str, value: &str) -> Result<Value>;

    /// Create a new category
    async fn create_category(&self, data: &Value) -> Result<Value>;

    /// Update a category
    async fn update_category(&self, id: &str, data: &Value) -> Result<Value>;

    /// Archive a category
    async fn archive_category(&self, id: &str) -> Result<()>;

    /// Get a topic by ID
    async fn get_topic(&self, id: &str) -> Result<Value>;

    /// Create a new topic
    async fn create_topic(&self, data: &Value) -> Result<Value>;

    /// Update a topic
    async fn update_topic(&self, id: &str, data: &Value) -> Result<Value>;

    /// Create a new post in a topic
    async fn create_post(&self, data: &Value) -> Result<Value>;

    /// Update a post
    async fn update_post(&self, id: &str, data: &Value) -> Result<Value>;

    /// Get all users
    async fn get_users(&self, params: Option<HashMap<String, String>>) -> Result<Vec<DiscourseUser>>;

    /// Get all categories
    async fn get_categories(&self) -> Result<Vec<DiscourseCategory>>;

    /// Get all topics in a category
    async fn get_topics_by_category(&self, category_id: &str) -> Result<Vec<DiscourseTopic>>;

    /// Get all posts in a topic
    async fn get_posts_by_topic(&self, topic_id: &str) -> Result<Vec<DiscoursePost>>;

    /// Get all tags
    async fn get_tags(&self) -> Result<Vec<DiscourseTag>>;

    /// Get all tag groups
    async fn get_tag_groups(&self) -> Result<Vec<DiscourseTagGroup>>;

    /// Get a user by username as a structured model
    async fn get_user_by_username(&self, username: &str) -> Result<DiscourseUser>;

    /// Get a topic by ID as a structured model
    async fn get_topic_by_id(&self, id: &str) -> Result<DiscourseTopic>;

    /// Create a new topic with structured data
    async fn create_topic_structured(&self, category_id: &str, title: &str, content: &str, tags: Option<Vec<String>>) -> Result<DiscourseTopic>;

    /// Create a post in a topic with structured data
    async fn create_post_structured(&self, topic_id: &str, content: &str) -> Result<DiscoursePost>;
}

/// Configuration for Discourse API client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiscourseApiConfig {
    /// Base URL of the Discourse forum (e.g., "https://forum.example.com")
    pub base_url: String,

    /// API key
    pub api_key: String,

    /// API username (typically an admin account)
    pub api_username: String,

    /// Timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,

    /// Maximum number of retries
    #[serde(default = "default_retries")]
    pub max_retries: usize,
}

fn default_timeout() -> u64 {
    30
}

fn default_retries() -> usize {
    3
}

/// Implementation of Discourse API client
pub struct DiscourseApiClient {
    config: DiscourseApiConfig,
    client: Client,
}

impl DiscourseApiClient {
    /// Create a new Discourse API client
    pub fn new(config: DiscourseApiConfig) -> Result<Self> {
        // Create HTTP client with appropriate configuration
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            config,
            client,
        })
    }

    /// Add common query parameters for authentication
    fn auth_params(&self) -> Vec<(String, String)> {
        vec![
            ("api_key".to_string(), self.config.api_key.clone()),
            ("api_username".to_string(), self.config.api_username.clone())
        ]
    }

    /// Make a GET request to Discourse API
    async fn get(&self, path: &str, params: Option<&[(&str, &str)]>) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Discourse API request: GET {}", url);

        // Build the request with authentication parameters
        let mut request = self.client.get(&url);

        // Add auth params
        for (k, v) in self.auth_params() {
            request = request.query(&[(k, v)]);
        }

        // Add additional params if provided
        if let Some(extra_params) = params {
            request = request.query(extra_params);
        }

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match request.try_clone().unwrap().send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_text = response.text().await?;

                    if status.is_success() {
                        match serde_json::from_str(&response_text) {
                            Ok(json) => return Ok(json),
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("Failed to parse JSON: {}", e));
                                retries += 1;
                                continue;
                            }
                        }
                    } else {
                        last_error = Some(anyhow::anyhow!(
                            "Discourse API error: {} - {}",
                            status,
                            response_text
                        ));

                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }

                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }

        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Discourse API request failed")))
    }

    /// Make a POST request to Discourse API
    async fn post(&self, path: &str, data: &Value) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Discourse API request: POST {}", url);

        // Build the request with authentication parameters
        let mut request = self.client.post(&url);

        // Add auth params
        for (k, v) in self.auth_params() {
            request = request.query(&[(k, v)]);
        }

        // Add JSON body
        request = request.json(data);

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match request.try_clone().unwrap().send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_text = response.text().await?;

                    if status.is_success() {
                        match serde_json::from_str(&response_text) {
                            Ok(json) => return Ok(json),
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("Failed to parse JSON: {}", e));
                                retries += 1;
                                continue;
                            }
                        }
                    } else {
                        last_error = Some(anyhow::anyhow!(
                            "Discourse API error: {} - {}",
                            status,
                            response_text
                        ));

                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }

                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }

        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Discourse API request failed")))
    }

    /// Make a PUT request to Discourse API
    async fn put(&self, path: &str, data: &Value) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Discourse API request: PUT {}", url);

        // Build the request with authentication parameters
        let mut request = self.client.put(&url);

        // Add auth params
        for (k, v) in self.auth_params() {
            request = request.query(&[(k, v)]);
        }

        // Add JSON body
        request = request.json(data);

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match request.try_clone().unwrap().send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_text = response.text().await?;

                    if status.is_success() {
                        match serde_json::from_str(&response_text) {
                            Ok(json) => return Ok(json),
                            Err(e) => {
                                last_error = Some(anyhow::anyhow!("Failed to parse JSON: {}", e));
                                retries += 1;
                                continue;
                            }
                        }
                    } else {
                        last_error = Some(anyhow::anyhow!(
                            "Discourse API error: {} - {}",
                            status,
                            response_text
                        ));

                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }

                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }

        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Discourse API request failed")))
    }

    /// Make a DELETE request to Discourse API
    async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Discourse API request: DELETE {}", url);

        // Build the request with authentication parameters
        let mut request = self.client.delete(&url);

        // Add auth params
        for (k, v) in self.auth_params() {
            request = request.query(&[(k, v)]);
        }

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match request.try_clone().unwrap().send().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        return Ok(());
                    } else {
                        let response_text = response.text().await?;
                        last_error = Some(anyhow::anyhow!(
                            "Discourse API error: {} - {}",
                            status,
                            response_text
                        ));

                        // Don't retry if client error (except rate limiting)
                        if status.is_client_error() && status.as_u16() != 429 {
                            break;
                        }

                        retries += 1;
                    }
                }
                Err(e) => {
                    last_error = Some(anyhow::anyhow!("Request failed: {}", e));
                    retries += 1;
                }
            }
        }

        // If we get here, all retries failed
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Discourse API request failed")))
    }
}

#[async_trait]
impl DiscourseApi for DiscourseApiClient {
    async fn get_user(&self, id: &str) -> Result<Value> {
        self.get(&format!("/admin/users/{}.json", id), None).await
    }

    async fn get_user_by_external_id(&self, external_id: &str) -> Result<Value> {
        let params = [("external_id", external_id)];

        let users = self.get("/admin/users/list/all.json", Some(&params)).await?;

        if let Some(users_array) = users.as_array() {
            if users_array.is_empty() {
                return Err(anyhow!("User with external ID '{}' not found", external_id));
            }

            return Ok(users_array[0].clone());
        }

        Err(anyhow!("Unexpected response format for user lookup"))
    }

    async fn create_user(&self, data: &Value) -> Result<Value> {
        self.post("/users", data).await
    }

    async fn update_user(&self, id: &str, data: &Value) -> Result<Value> {
        self.put(&format!("/admin/users/{}.json", id), data).await
    }

    async fn deactivate_user(&self, id: &str) -> Result<()> {
        let data = serde_json::json!({
            "suspend_until": "3018-01-01",  // Far future date
            "reason": "User deactivated via synchronization"
        });

        self.put(&format!("/admin/users/{}/suspend.json", id), &data).await?;

        Ok(())
    }

    async fn get_category(&self, id: &str) -> Result<Value> {
        self.get(&format!("/c/{}.json", id), None).await
    }

    async fn get_category_by_custom_field(&self, field: &str, value: &str) -> Result<Value> {
        // Discourse doesn't have a direct API for fetching by custom fields
        // So we fetch all categories and filter them
        let categories = self.get("/categories.json", None).await?;

        if let Some(category_list) = categories.get("category_list") {
            if let Some(categories_array) = category_list.get("categories").and_then(|c| c.as_array()) {
                for category in categories_array {
                    if let Some(custom_fields) = category.get("custom_fields") {
                        if let Some(field_value) = custom_fields.get(field) {
                            if let Some(field_str) = field_value.as_str() {
                                if field_str == value {
                                    return Ok(category.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        Err(anyhow!("Category with {}='{}' not found", field, value))
    }

    async fn create_category(&self, data: &Value) -> Result<Value> {
        self.post("/categories", data).await
    }

    async fn update_category(&self, id: &str, data: &Value) -> Result<Value> {
        self.put(&format!("/categories/{}", id), data).await
    }

    async fn archive_category(&self, id: &str) -> Result<()> {
        let data = serde_json::json!({
            "position": "archived"
        });

        self.put(&format!("/categories/{}", id), &data).await?;

        Ok(())
    }

    async fn get_topic(&self, id: &str) -> Result<Value> {
        self.get(&format!("/t/{}.json", id), None).await
    }

    async fn create_topic(&self, data: &Value) -> Result<Value> {
        self.post("/posts", data).await
    }

    async fn update_topic(&self, id: &str, data: &Value) -> Result<Value> {
        // In Discourse, updating a topic is done by updating its first post
        let topic = self.get_topic(id).await?;

        if let Some(post_stream) = topic.get("post_stream") {
            if let Some(posts) = post_stream.get("posts").and_then(|p| p.as_array()) {
                if !posts.is_empty() {
                    if let Some(post_id) = posts[0].get("id").and_then(|id| id.as_u64()) {
                        let post_id_str = post_id.to_string();
                        return self.update_post(&post_id_str, data).await;
                    }
                }
            }
        }

        Err(anyhow!("Failed to find first post of topic {}", id))
    }

    async fn create_post(&self, data: &Value) -> Result<Value> {
        self.post("/posts", data).await
    }

    async fn update_post(&self, id: &str, data: &Value) -> Result<Value> {
        self.put(&format!("/posts/{}", id), data).await
    }

    async fn get_users(&self, params: Option<HashMap<String, String>>) -> Result<Vec<DiscourseUser>> {
        let mut query_params = Vec::new();

        if let Some(params) = params {
            for (k, v) in params {
                query_params.push((k.as_str(), v.as_str()));
            }
        }

        let users_value = self.get("/admin/users/list/active.json", if query_params.is_empty() { None } else { Some(&query_params) }).await?;

        if let Some(users_array) = users_value.as_array() {
            let mut users = Vec::new();

            for user_value in users_array {
                match serde_json::from_value::<DiscourseUser>(user_value.clone()) {
                    Ok(user) => users.push(user),
                    Err(e) => {
                        error!("Failed to parse Discourse user: {}", e);
                        continue;
                    }
                }
            }

            return Ok(users);
        }

        Err(anyhow!("Unexpected response format for users list"))
    }

    async fn get_categories(&self) -> Result<Vec<DiscourseCategory>> {
        let categories_value = self.get("/categories.json", None).await?;

        if let Some(category_list) = categories_value.get("category_list") {
            if let Some(categories_array) = category_list.get("categories").and_then(|c| c.as_array()) {
                let mut categories = Vec::new();

                for category_value in categories_array {
                    match serde_json::from_value::<DiscourseCategory>(category_value.clone()) {
                        Ok(category) => categories.push(category),
                        Err(e) => {
                            error!("Failed to parse Discourse category: {}", e);
                            continue;
                        }
                    }
                }

                return Ok(categories);
            }
        }

        Err(anyhow!("Unexpected response format for categories list"))
    }

    async fn get_topics_by_category(&self, category_id: &str) -> Result<Vec<DiscourseTopic>> {
        let topics_value = self.get(&format!("/c/{}/l/latest.json", category_id), None).await?;

        if let Some(topic_list) = topics_value.get("topic_list") {
            if let Some(topics_array) = topic_list.get("topics").and_then(|t| t.as_array()) {
                let mut topics = Vec::new();

                for topic_value in topics_array {
                    match serde_json::from_value::<DiscourseTopic>(topic_value.clone()) {
                        Ok(topic) => topics.push(topic),
                        Err(e) => {
                            error!("Failed to parse Discourse topic: {}", e);
                            continue;
                        }
                    }
                }

                return Ok(topics);
            }
        }

        Err(anyhow!("Unexpected response format for topics list"))
    }

    async fn get_posts_by_topic(&self, topic_id: &str) -> Result<Vec<DiscoursePost>> {
        let topic_value = self.get(&format!("/t/{}.json", topic_id), None).await?;

        if let Some(post_stream) = topic_value.get("post_stream") {
            if let Some(posts_array) = post_stream.get("posts").and_then(|p| p.as_array()) {
                let mut posts = Vec::new();

                for post_value in posts_array {
                    match serde_json::from_value::<DiscoursePost>(post_value.clone()) {
                        Ok(post) => posts.push(post),
                        Err(e) => {
                            error!("Failed to parse Discourse post: {}", e);
                            continue;
                        }
                    }
                }

                return Ok(posts);
            }
        }

        Err(anyhow!("Unexpected response format for posts list"))
    }

    async fn get_tags(&self) -> Result<Vec<DiscourseTag>> {
        let tags_value = self.get("/tags.json", None).await?;

        if let Some(tags_array) = tags_value.get("tags").and_then(|t| t.as_array()) {
            let mut tags = Vec::new();

            for tag_value in tags_array {
                match serde_json::from_value::<DiscourseTag>(tag_value.clone()) {
                    Ok(tag) => tags.push(tag),
                    Err(e) => {
                        error!("Failed to parse Discourse tag: {}", e);
                        continue;
                    }
                }
            }

            return Ok(tags);
        }

        Err(anyhow!("Unexpected response format for tags list"))
    }

    async fn get_tag_groups(&self) -> Result<Vec<DiscourseTagGroup>> {
        let tag_groups_value = self.get("/tag_groups.json", None).await?;

        if let Some(tag_groups_array) = tag_groups_value.get("tag_groups").and_then(|t| t.as_array()) {
            let mut tag_groups = Vec::new();

            for tag_group_value in tag_groups_array {
                match serde_json::from_value::<DiscourseTagGroup>(tag_group_value.clone()) {
                    Ok(tag_group) => tag_groups.push(tag_group),
                    Err(e) => {
                        error!("Failed to parse Discourse tag group: {}", e);
                        continue;
                    }
                }
            }

            return Ok(tag_groups);
        }

        Err(anyhow!("Unexpected response format for tag groups list"))
    }

    async fn get_user_by_username(&self, username: &str) -> Result<DiscourseUser> {
        let user_value = self.get(&format!("/users/{}.json", username), None).await?;

        if let Some(user_obj) = user_value.get("user") {
            match serde_json::from_value::<DiscourseUser>(user_obj.clone()) {
                Ok(user) => return Ok(user),
                Err(e) => return Err(anyhow!("Failed to parse Discourse user: {}", e)),
            }
        }

        Err(anyhow!("Unexpected response format for user"))
    }

    async fn get_topic_by_id(&self, id: &str) -> Result<DiscourseTopic> {
        let topic_value = self.get(&format!("/t/{}.json", id), None).await?;

        match serde_json::from_value::<DiscourseTopic>(topic_value.clone()) {
            Ok(topic) => Ok(topic),
            Err(e) => Err(anyhow!("Failed to parse Discourse topic: {}", e)),
        }
    }

    async fn create_topic_structured(&self, category_id: &str, title: &str, content: &str, tags: Option<Vec<String>>) -> Result<DiscourseTopic> {
        let mut data = serde_json::json!({
            "title": title,
            "raw": content,
            "category": category_id,
        });

        if let Some(tags) = tags {
            data["tags"] = serde_json::to_value(tags)?;
        }

        let response = self.post("/posts", &data).await?;

        if let Some(topic_id) = response.get("topic_id").and_then(|id| id.as_u64()) {
            let topic_id_str = topic_id.to_string();
            return self.get_topic_by_id(&topic_id_str).await;
        }

        Err(anyhow!("Failed to get topic ID from response"))
    }

    async fn create_post_structured(&self, topic_id: &str, content: &str) -> Result<DiscoursePost> {
        let data = serde_json::json!({
            "topic_id": topic_id,
            "raw": content,
        });

        let response = self.post("/posts", &data).await?;

        if let Some(post_id) = response.get("id").and_then(|id| id.as_u64()) {
            let post_id_str = post_id.to_string();
            let post_value = self.get(&format!("/posts/{}.json", post_id_str), None).await?;

            match serde_json::from_value::<DiscoursePost>(post_value.clone()) {
                Ok(post) => return Ok(post),
                Err(e) => return Err(anyhow!("Failed to parse Discourse post: {}", e)),
            }
        }

        Err(anyhow!("Failed to get post ID from response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_get_user() {
        let mock_server = server_url();

        // Setup mock response
        let _m = mock("GET", "/admin/users/123.json")
            .match_query(mockito::Matcher::UrlEncoded(
                "api_key".into(),
                "test_key".into(),
            ))
            .match_query(mockito::Matcher::UrlEncoded(
                "api_username".into(),
                "admin".into(),
            ))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 123, "username": "test_user", "name": "Test User"}"#)
            .create();

        // Create client with mock server URL
        let config = DiscourseApiConfig {
            base_url: mock_server,
            api_key: "test_key".to_string(),
            api_username: "admin".to_string(),
            timeout_seconds: 5,
            max_retries: 1,
        };

        let client = DiscourseApiClient::new(config).unwrap();

        // Test API call
        let result = client.get_user("123").await.unwrap();

        assert_eq!(result["username"], "test_user");
        assert_eq!(result["name"], "Test User");
    }
}
