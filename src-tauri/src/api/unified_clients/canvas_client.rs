use async_trait::async_trait;
use reqwest::{Client, header, Method};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::collections::HashMap;

use super::base_client::{ApiClient, ApiClientConfig, ApiError, Result, PaginationParams, PaginatedResponse};
use crate::models::unified_models::{User, Course, Assignment, Submission, Topic};

/// Canvas API client
#[derive(Debug, Clone)]
pub struct CanvasApiClient {
    /// Base API client configuration
    config: ApiClientConfig,
    
    /// HTTP client
    client: Client,
}

/// Canvas notification model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasNotification {
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

impl CanvasApiClient {
    /// Create a new Canvas API client
    pub fn new(base_url: &str, api_key: &str) -> Result<Self> {
        // Create headers with authorization
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Authorization",
            header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                .map_err(|_| ApiError::AuthError("Invalid API key format".to_string()))?,
        );
        
        // Create client builder
        let client_builder = Client::builder()
            .default_headers(headers)
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
            api_username: None,
        };
        
        Ok(Self {
            config,
            client,
        })
    }
    
    /// Get user notifications
    pub async fn get_user_notifications(&self, canvas_user_id: &str) -> Result<Vec<CanvasNotification>> {
        let path = format!("/api/v1/users/{}/notifications", canvas_user_id);
        self.get(&path, None).await
    }
    
    /// Mark a notification as read
    pub async fn mark_notification_as_read(&self, notification_id: &str) -> Result<CanvasNotification> {
        let path = format!("/api/v1/notifications/{}/read", notification_id);
        self.put::<(), CanvasNotification>(&path, None, None).await
    }
    
    /// Create a notification
    pub async fn create_notification(&self, notification_data: &serde_json::Value) -> Result<CanvasNotification> {
        let path = "/api/v1/notifications";
        self.post(&path, Some(notification_data), None).await
    }
    
    /// Get a user by ID
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        let path = format!("/api/v1/users/{}", user_id);
        let canvas_user: serde_json::Value = self.get(&path, None).await?;
        Ok(User::from_canvas_user(&canvas_user))
    }
    
    /// Get a list of users
    pub async fn get_users(&self, pagination: &PaginationParams) -> Result<PaginatedResponse<User>> {
        let path = "/api/v1/users";
        let canvas_users: PaginatedResponse<serde_json::Value> = self.get_paginated(&path, pagination, None).await?;
        
        // Convert Canvas users to unified User models
        let users = canvas_users.items.iter()
            .map(|user| User::from_canvas_user(user))
            .collect();
            
        Ok(PaginatedResponse {
            items: users,
            total: canvas_users.total,
            page: canvas_users.page,
            per_page: canvas_users.per_page,
            total_pages: canvas_users.total_pages,
            next_cursor: canvas_users.next_cursor,
            prev_cursor: canvas_users.prev_cursor,
            has_next: canvas_users.has_next,
            has_prev: canvas_users.has_prev,
        })
    }
    
    /// Get a course by ID
    pub async fn get_course(&self, course_id: &str) -> Result<Course> {
        let path = format!("/api/v1/courses/{}", course_id);
        let canvas_course: serde_json::Value = self.get(&path, None).await?;
        Ok(Course::from_canvas_course(&canvas_course))
    }
    
    /// Get a list of courses
    pub async fn get_courses(&self, pagination: &PaginationParams) -> Result<PaginatedResponse<Course>> {
        let path = "/api/v1/courses";
        let canvas_courses: PaginatedResponse<serde_json::Value> = self.get_paginated(&path, pagination, None).await?;
        
        // Convert Canvas courses to unified Course models
        let courses = canvas_courses.items.iter()
            .map(|course| Course::from_canvas_course(course))
            .collect();
            
        Ok(PaginatedResponse {
            items: courses,
            total: canvas_courses.total,
            page: canvas_courses.page,
            per_page: canvas_courses.per_page,
            total_pages: canvas_courses.total_pages,
            next_cursor: canvas_courses.next_cursor,
            prev_cursor: canvas_courses.prev_cursor,
            has_next: canvas_courses.has_next,
            has_prev: canvas_courses.has_prev,
        })
    }
    
    /// Get an assignment by ID
    pub async fn get_assignment(&self, course_id: &str, assignment_id: &str) -> Result<Assignment> {
        let path = format!("/api/v1/courses/{}/assignments/{}", course_id, assignment_id);
        let canvas_assignment: serde_json::Value = self.get(&path, None).await?;
        Ok(Assignment::from_canvas_assignment(&canvas_assignment))
    }
    
    /// Get a list of assignments for a course
    pub async fn get_course_assignments(&self, course_id: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<Assignment>> {
        let path = format!("/api/v1/courses/{}/assignments", course_id);
        let canvas_assignments: PaginatedResponse<serde_json::Value> = self.get_paginated(&path, pagination, None).await?;
        
        // Convert Canvas assignments to unified Assignment models
        let assignments = canvas_assignments.items.iter()
            .map(|assignment| Assignment::from_canvas_assignment(assignment))
            .collect();
            
        Ok(PaginatedResponse {
            items: assignments,
            total: canvas_assignments.total,
            page: canvas_assignments.page,
            per_page: canvas_assignments.per_page,
            total_pages: canvas_assignments.total_pages,
            next_cursor: canvas_assignments.next_cursor,
            prev_cursor: canvas_assignments.prev_cursor,
            has_next: canvas_assignments.has_next,
            has_prev: canvas_assignments.has_prev,
        })
    }
    
    /// Get a submission by ID
    pub async fn get_submission(&self, course_id: &str, assignment_id: &str, user_id: &str) -> Result<Submission> {
        let path = format!("/api/v1/courses/{}/assignments/{}/submissions/{}", course_id, assignment_id, user_id);
        let canvas_submission: serde_json::Value = self.get(&path, None).await?;
        Ok(Submission::from_canvas_submission(&canvas_submission, assignment_id, user_id))
    }
    
    /// Get a list of submissions for an assignment
    pub async fn get_assignment_submissions(&self, course_id: &str, assignment_id: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<Submission>> {
        let path = format!("/api/v1/courses/{}/assignments/{}/submissions", course_id, assignment_id);
        let canvas_submissions: PaginatedResponse<serde_json::Value> = self.get_paginated(&path, pagination, None).await?;
        
        // Convert Canvas submissions to unified Submission models
        let submissions = canvas_submissions.items.iter()
            .map(|submission| {
                let user_id = submission["user_id"].as_str()
                    .or_else(|| submission["user_id"].as_i64().map(|id| id.to_string().as_str()))
                    .unwrap_or("unknown");
                Submission::from_canvas_submission(submission, assignment_id, user_id)
            })
            .collect();
            
        Ok(PaginatedResponse {
            items: submissions,
            total: canvas_submissions.total,
            page: canvas_submissions.page,
            per_page: canvas_submissions.per_page,
            total_pages: canvas_submissions.total_pages,
            next_cursor: canvas_submissions.next_cursor,
            prev_cursor: canvas_submissions.prev_cursor,
            has_next: canvas_submissions.has_next,
            has_prev: canvas_submissions.has_prev,
        })
    }
    
    /// Get a discussion topic by ID
    pub async fn get_discussion_topic(&self, course_id: &str, topic_id: &str) -> Result<Topic> {
        let path = format!("/api/v1/courses/{}/discussion_topics/{}", course_id, topic_id);
        let canvas_topic: serde_json::Value = self.get(&path, None).await?;
        Ok(Topic::from_canvas_discussion(&canvas_topic))
    }
    
    /// Get a list of discussion topics for a course
    pub async fn get_course_discussion_topics(&self, course_id: &str, pagination: &PaginationParams) -> Result<PaginatedResponse<Topic>> {
        let path = format!("/api/v1/courses/{}/discussion_topics", course_id);
        let canvas_topics: PaginatedResponse<serde_json::Value> = self.get_paginated(&path, pagination, None).await?;
        
        // Convert Canvas topics to unified Topic models
        let topics = canvas_topics.items.iter()
            .map(|topic| Topic::from_canvas_discussion(topic))
            .collect();
            
        Ok(PaginatedResponse {
            items: topics,
            total: canvas_topics.total,
            page: canvas_topics.page,
            per_page: canvas_topics.per_page,
            total_pages: canvas_topics.total_pages,
            next_cursor: canvas_topics.next_cursor,
            prev_cursor: canvas_topics.prev_cursor,
            has_next: canvas_topics.has_next,
            has_prev: canvas_topics.has_prev,
        })
    }
}

#[async_trait]
impl ApiClient for CanvasApiClient {
    fn get_config(&self) -> &ApiClientConfig {
        &self.config
    }
    
    fn get_http_client(&self) -> &Client {
        &self.client
    }
}

/// Create a new Canvas API client
pub fn create_canvas_client(base_url: &str, api_key: &str) -> Result<Arc<CanvasApiClient>> {
    let client = CanvasApiClient::new(base_url, api_key)?;
    Ok(Arc::new(client))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    
    #[tokio::test]
    async fn test_get_user() {
        let _m = mock("GET", "/api/v1/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"123","name":"Test User","email":"test@example.com","login_id":"testuser"}"#)
            .create();
            
        let client = CanvasApiClient::new(&server_url(), "test_key").unwrap();
        let user = client.get_user("123").await.unwrap();
        
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
    }
    
    #[tokio::test]
    async fn test_get_course() {
        let _m = mock("GET", "/api/v1/courses/456")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"456","name":"Test Course","course_code":"TEST101"}"#)
            .create();
            
        let client = CanvasApiClient::new(&server_url(), "test_key").unwrap();
        let course = client.get_course("456").await.unwrap();
        
        assert_eq!(course.title, "Test Course");
        assert_eq!(course.code, Some("TEST101".to_string()));
    }
    
    #[tokio::test]
    async fn test_get_assignment() {
        let _m = mock("GET", "/api/v1/courses/456/assignments/789")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id":"789","name":"Test Assignment","description":"Test description","course_id":"456","points_possible":100}"#)
            .create();
            
        let client = CanvasApiClient::new(&server_url(), "test_key").unwrap();
        let assignment = client.get_assignment("456", "789").await.unwrap();
        
        assert_eq!(assignment.title, "Test Assignment");
        assert_eq!(assignment.description, Some("Test description".to_string()));
        assert_eq!(assignment.course_id, "456");
        assert_eq!(assignment.points_possible, Some(100.0));
    }
}
