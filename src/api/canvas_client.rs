use std::collections::HashMap;
use async_trait::async_trait;
use anyhow::{Result, Context};
use log::{info, error, debug};
use reqwest::{Client, header};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use crate::api::base_client::{ApiClient, ApiClientConfig};

/// Canvas API client interface
#[async_trait]
pub trait CanvasApi: Send + Sync {
    /// Get a user by ID
    async fn get_user(&self, id: &str) -> Result<Value>;

    /// Get a course by ID
    async fn get_course(&self, id: &str) -> Result<Value>;

    /// Get an assignment by ID
    async fn get_assignment(&self, course_id: &str, assignment_id: &str) -> Result<Value>;

    /// Get a submission by assignment and user ID
    async fn get_submission(&self, course_id: &str, assignment_id: &str, user_id: &str) -> Result<Value>;

    /// Get a discussion topic by ID
    async fn get_discussion_topic(&self, course_id: &str, topic_id: &str) -> Result<Value>;

    /// Create a submission comment
    async fn create_submission_comment(&self, course_id: &str, assignment_id: &str, user_id: &str, comment: &str) -> Result<Value>;

    /// Get enrollment for a user in a course
    async fn get_enrollment(&self, course_id: &str, user_id: &str) -> Result<Value>;

    /// Get all users in a course
    async fn get_course_users(&self, course_id: &str) -> Result<Vec<CanvasUser>>;

    /// Get all courses
    async fn get_courses(&self, params: Option<HashMap<String, String>>) -> Result<Vec<CanvasCourse>>;

    /// Get all discussions in a course
    async fn get_discussions(&self, course_id: &str) -> Result<Vec<CanvasDiscussion>>;

    /// Get discussion entries
    async fn get_discussion_entries(&self, course_id: &str, discussion_id: &str) -> Result<Vec<CanvasDiscussionEntry>>;

    /// Create a new discussion
    async fn create_discussion(&self, course_id: &str, title: &str, message: &str, published: bool) -> Result<CanvasDiscussion>;

    /// Create a discussion entry
    async fn create_discussion_entry(&self, course_id: &str, discussion_id: &str, message: &str) -> Result<CanvasDiscussionEntry>;

    /// Reply to a discussion entry
    async fn reply_to_discussion_entry(&self, course_id: &str, discussion_id: &str, entry_id: &str, message: &str) -> Result<CanvasDiscussionEntry>;
}

/// Canvas user model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasUser {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub sortable_name: Option<String>,
    pub short_name: Option<String>,
    pub sis_user_id: Option<String>,
    pub integration_id: Option<String>,
    pub login_id: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub locale: Option<String>,
    pub effective_locale: Option<String>,
    pub time_zone: Option<String>,
    pub bio: Option<String>,
}

/// Canvas course model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCourse {
    pub id: String,
    pub name: String,
    pub course_code: String,
    pub workflow_state: String,
    pub account_id: Option<String>,
    pub root_account_id: Option<String>,
    pub enrollment_term_id: Option<String>,
    pub grading_standard_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub locale: Option<String>,
    pub enrollments: Option<Vec<CanvasEnrollment>>,
    pub total_students: Option<i32>,
    pub calendar: Option<CanvasCalendar>,
    pub default_view: Option<String>,
    pub syllabus_body: Option<String>,
    pub needs_grading_count: Option<i32>,
    pub term: Option<CanvasTerm>,
    pub course_progress: Option<CanvasCourseProgress>,
    pub apply_assignment_group_weights: Option<bool>,
    pub permissions: Option<HashMap<String, bool>>,
    pub is_public: Option<bool>,
    pub is_public_to_auth_users: Option<bool>,
    pub public_syllabus: Option<bool>,
    pub public_syllabus_to_auth: Option<bool>,
    pub public_description: Option<String>,
    pub storage_quota_mb: Option<i32>,
    pub storage_quota_used_mb: Option<i32>,
    pub hide_final_grades: Option<bool>,
    pub license: Option<String>,
    pub allow_student_assignment_edits: Option<bool>,
    pub allow_wiki_comments: Option<bool>,
    pub allow_student_forum_attachments: Option<bool>,
    pub open_enrollment: Option<bool>,
    pub self_enrollment: Option<bool>,
    pub restrict_enrollments_to_course_dates: Option<bool>,
    pub course_format: Option<String>,
    pub access_restricted_by_date: Option<bool>,
    pub time_zone: Option<String>,
    pub blueprint: Option<bool>,
    pub blueprint_restrictions: Option<HashMap<String, bool>>,
    pub blueprint_restrictions_by_object_type: Option<HashMap<String, HashMap<String, bool>>>,
}

/// Canvas enrollment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasEnrollment {
    pub id: Option<String>,
    pub course_id: Option<String>,
    pub course_section_id: Option<String>,
    pub enrollment_state: Option<String>,
    pub limit_privileges_to_course_section: Option<bool>,
    pub root_account_id: Option<String>,
    #[serde(rename = "type")]
    pub enrollment_type: Option<String>,
    pub user_id: Option<String>,
    pub associated_user_id: Option<String>,
    pub role: Option<String>,
    pub role_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub last_attended_at: Option<DateTime<Utc>>,
    pub total_activity_time: Option<i32>,
    pub html_url: Option<String>,
    pub grades: Option<CanvasGrades>,
    pub user: Option<CanvasUser>,
    pub override_grade: Option<String>,
    pub override_score: Option<f64>,
    pub unposted_current_grade: Option<String>,
    pub unposted_final_grade: Option<String>,
    pub unposted_current_score: Option<f64>,
    pub unposted_final_score: Option<f64>,
    pub has_grading_periods: Option<bool>,
    pub totals_for_all_grading_periods_option: Option<bool>,
    pub current_grading_period_title: Option<String>,
    pub current_grading_period_id: Option<String>,
    pub current_period_override_grade: Option<String>,
    pub current_period_override_score: Option<f64>,
    pub current_period_unposted_current_score: Option<f64>,
    pub current_period_unposted_final_score: Option<f64>,
    pub current_period_unposted_current_grade: Option<String>,
    pub current_period_unposted_final_grade: Option<String>,
}

/// Canvas grades model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasGrades {
    pub html_url: Option<String>,
    pub current_grade: Option<String>,
    pub current_score: Option<f64>,
    pub final_grade: Option<String>,
    pub final_score: Option<f64>,
}

/// Canvas calendar model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCalendar {
    pub ics: Option<String>,
}

/// Canvas term model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasTerm {
    pub id: Option<String>,
    pub name: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
}

/// Canvas course progress model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasCourseProgress {
    pub requirement_count: Option<i32>,
    pub requirement_completed_count: Option<i32>,
    pub next_requirement_url: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Canvas discussion model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDiscussion {
    pub id: String,
    pub title: String,
    pub message: Option<String>,
    pub html_url: Option<String>,
    pub posted_at: Option<DateTime<Utc>>,
    pub last_reply_at: Option<DateTime<Utc>>,
    pub require_initial_post: Option<bool>,
    pub user_can_see_posts: Option<bool>,
    pub discussion_subentry_count: Option<i32>,
    pub read_state: Option<String>,
    pub unread_count: Option<i32>,
    pub subscribed: Option<bool>,
    pub subscription_hold: Option<String>,
    pub assignment_id: Option<String>,
    pub delayed_post_at: Option<DateTime<Utc>>,
    pub published: Option<bool>,
    pub lock_at: Option<DateTime<Utc>>,
    pub locked: Option<bool>,
    pub pinned: Option<bool>,
    pub locked_for_user: Option<bool>,
    pub lock_info: Option<CanvasLockInfo>,
    pub lock_explanation: Option<String>,
    pub user_name: Option<String>,
    pub topic_children: Option<Vec<String>>,
    pub group_topic_children: Option<Vec<CanvasGroupTopicChild>>,
    pub root_topic_id: Option<String>,
    pub podcast_url: Option<String>,
    pub discussion_type: Option<String>,
    pub group_category_id: Option<String>,
    pub attachments: Option<Vec<CanvasAttachment>>,
    pub permissions: Option<HashMap<String, bool>>,
    pub allow_rating: Option<bool>,
    pub only_graders_can_rate: Option<bool>,
    pub sort_by_rating: Option<bool>,
    pub user_id: Option<String>,
    pub course_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Canvas lock info model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasLockInfo {
    pub asset_string: Option<String>,
    pub unlock_at: Option<DateTime<Utc>>,
    pub lock_at: Option<DateTime<Utc>>,
    pub context_module: Option<CanvasContextModule>,
    pub manually_locked: Option<bool>,
}

/// Canvas context module model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasContextModule {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

/// Canvas group topic child model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasGroupTopicChild {
    pub id: Option<String>,
    pub group_id: Option<String>,
}

/// Canvas attachment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasAttachment {
    pub id: Option<String>,
    pub filename: Option<String>,
    pub display_name: Option<String>,
    pub content_type: Option<String>,
    pub url: Option<String>,
    pub size: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub unlock_at: Option<DateTime<Utc>>,
    pub locked: Option<bool>,
    pub hidden: Option<bool>,
    pub lock_at: Option<DateTime<Utc>>,
    pub hidden_for_user: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub modified_at: Option<DateTime<Utc>>,
    pub mime_class: Option<String>,
    pub media_entry_id: Option<String>,
}

/// Canvas discussion entry model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanvasDiscussionEntry {
    pub id: String,
    pub user_id: Option<String>,
    pub parent_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub rating_count: Option<i32>,
    pub rating_sum: Option<i32>,
    pub message: String,
    pub user_name: Option<String>,
    pub read_state: Option<String>,
    pub forced_read_state: Option<bool>,
    pub discussion_topic_id: Option<String>,
    pub attachment: Option<CanvasAttachment>,
    pub attachments: Option<Vec<CanvasAttachment>>,
    pub recent_replies: Option<Vec<CanvasDiscussionEntry>>,
    pub has_more_replies: Option<bool>,
}

/// Configuration for Canvas API client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CanvasApiConfig {
    /// Base URL of the Canvas API (e.g., "https://canvas.example.com/api/v1")
    pub base_url: String,

    /// API access token
    pub access_token: String,

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

/// Implementation of Canvas API client
pub struct CanvasApiClient {
    config: CanvasApiConfig,
    client: Client,
}

impl CanvasApiClient {
    /// Create a new Canvas API client
    pub fn new(config: CanvasApiConfig) -> Result<Self> {
        // Create custom headers with authentication
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.access_token))
                .context("Invalid access token")?
        );

        // Create HTTP client with appropriate configuration
        let client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            config,
            client,
        })
    }

    /// Make a GET request to Canvas API
    async fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Canvas API request: GET {}", url);

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match self.client.get(&url).send().await {
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
                            "Canvas API error: {} - {}",
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
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Canvas API request failed")))
    }

    /// Make a GET request with parameters to Canvas API
    async fn get_with_params(&self, path: &str, params: Option<HashMap<String, String>>) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Canvas API request: GET {} with params {:?}", url, params);

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            let mut request = self.client.get(&url);
            if let Some(params) = &params {
                request = request.query(params);
            }

            match request.send().await {
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
                            "Canvas API error: {} - {}",
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
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Canvas API request failed")))
    }

    /// Make a POST request to Canvas API
    async fn post(&self, path: &str, data: &Value) -> Result<Value> {
        let url = format!("{}{}", self.config.base_url, path);
        debug!("Canvas API request: POST {}", url);

        let mut retries = 0;
        let mut last_error = None;

        // Implement retry logic
        while retries <= self.config.max_retries {
            if retries > 0 {
                let delay = std::time::Duration::from_millis(250 * 2u64.pow(retries as u32));
                tokio::time::sleep(delay).await;
                debug!("Retrying request (attempt {}/{})", retries, self.config.max_retries);
            }

            match self.client.post(&url).json(data).send().await {
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
                            "Canvas API error: {} - {}",
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
        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Canvas API request failed")))
    }
}

#[async_trait]
impl CanvasApi for CanvasApiClient {
    async fn get_user(&self, id: &str) -> Result<Value> {
        self.get(&format!("/users/{}", id)).await
    }

    async fn get_course(&self, id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}", id)).await
    }

    async fn get_assignment(&self, course_id: &str, assignment_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/assignments/{}", course_id, assignment_id)).await
    }

    async fn get_submission(&self, course_id: &str, assignment_id: &str, user_id: &str) -> Result<Value> {
        self.get(&format!(
            "/courses/{}/assignments/{}/submissions/{}",
            course_id, assignment_id, user_id
        )).await
    }

    async fn get_discussion_topic(&self, course_id: &str, topic_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/discussion_topics/{}", course_id, topic_id)).await
    }

    async fn create_submission_comment(&self, course_id: &str, assignment_id: &str, user_id: &str, comment: &str) -> Result<Value> {
        let data = serde_json::json!({
            "comment": {
                "text_comment": comment
            }
        });

        self.post(&format!(
            "/courses/{}/assignments/{}/submissions/{}",
            course_id, assignment_id, user_id
        ), &data).await
    }

    async fn get_enrollment(&self, course_id: &str, user_id: &str) -> Result<Value> {
        self.get(&format!("/courses/{}/enrollments?user_id={}", course_id, user_id)).await
    }

    async fn get_course_users(&self, course_id: &str) -> Result<Vec<CanvasUser>> {
        let value = self.get(&format!("/courses/{}/users", course_id)).await?;

        match serde_json::from_value::<Vec<CanvasUser>>(value) {
            Ok(users) => Ok(users),
            Err(e) => Err(anyhow!("Failed to parse Canvas users: {}", e)),
        }
    }

    async fn get_courses(&self, params: Option<HashMap<String, String>>) -> Result<Vec<CanvasCourse>> {
        let value = if let Some(params) = params {
            self.get_with_params("/courses", Some(params)).await?
        } else {
            self.get("/courses").await?
        };

        match serde_json::from_value::<Vec<CanvasCourse>>(value) {
            Ok(courses) => Ok(courses),
            Err(e) => Err(anyhow!("Failed to parse Canvas courses: {}", e)),
        }
    }

    async fn get_discussions(&self, course_id: &str) -> Result<Vec<CanvasDiscussion>> {
        let value = self.get(&format!("/courses/{}/discussion_topics", course_id)).await?;

        match serde_json::from_value::<Vec<CanvasDiscussion>>(value) {
            Ok(discussions) => Ok(discussions),
            Err(e) => Err(anyhow!("Failed to parse Canvas discussions: {}", e)),
        }
    }

    async fn get_discussion_entries(&self, course_id: &str, discussion_id: &str) -> Result<Vec<CanvasDiscussionEntry>> {
        let value = self.get(&format!("/courses/{}/discussion_topics/{}/entries", course_id, discussion_id)).await?;

        match serde_json::from_value::<Vec<CanvasDiscussionEntry>>(value) {
            Ok(entries) => Ok(entries),
            Err(e) => Err(anyhow!("Failed to parse Canvas discussion entries: {}", e)),
        }
    }

    async fn create_discussion(&self, course_id: &str, title: &str, message: &str, published: bool) -> Result<CanvasDiscussion> {
        let data = serde_json::json!({
            "title": title,
            "message": message,
            "published": published
        });

        let value = self.post(&format!("/courses/{}/discussion_topics", course_id), &data).await?;

        match serde_json::from_value::<CanvasDiscussion>(value) {
            Ok(discussion) => Ok(discussion),
            Err(e) => Err(anyhow!("Failed to parse Canvas discussion: {}", e)),
        }
    }

    async fn create_discussion_entry(&self, course_id: &str, discussion_id: &str, message: &str) -> Result<CanvasDiscussionEntry> {
        let data = serde_json::json!({
            "message": message
        });

        let value = self.post(&format!("/courses/{}/discussion_topics/{}/entries", course_id, discussion_id), &data).await?;

        match serde_json::from_value::<CanvasDiscussionEntry>(value) {
            Ok(entry) => Ok(entry),
            Err(e) => Err(anyhow!("Failed to parse Canvas discussion entry: {}", e)),
        }
    }

    async fn reply_to_discussion_entry(&self, course_id: &str, discussion_id: &str, entry_id: &str, message: &str) -> Result<CanvasDiscussionEntry> {
        let data = serde_json::json!({
            "message": message
        });

        let value = self.post(&format!("/courses/{}/discussion_topics/{}/entries/{}/replies", course_id, discussion_id, entry_id), &data).await?;

        match serde_json::from_value::<CanvasDiscussionEntry>(value) {
            Ok(entry) => Ok(entry),
            Err(e) => Err(anyhow!("Failed to parse Canvas discussion entry: {}", e)),
        }
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
        let _m = mock("GET", "/users/123")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 123, "name": "Test User", "email": "test@example.com"}"#)
            .create();

        // Create client with mock server URL
        let config = CanvasApiConfig {
            base_url: mock_server,
            access_token: "fake-token".to_string(),
            timeout_seconds: 5,
            max_retries: 1,
        };

        let client = CanvasApiClient::new(config).unwrap();

        // Test API call
        let result = client.get_user("123").await.unwrap();

        assert_eq!(result["name"], "Test User");
        assert_eq!(result["email"], "test@example.com");
    }
}
