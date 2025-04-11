use crate::utils::logger::create_logger;
use crate::api::canvas_api::CanvasClient;
use crate::api::discourse_api::DiscourseClient;
use serde::{Serialize, Deserialize};
use log::LevelFilter;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::error::Error;

/// Announcement data from Canvas
#[derive(Debug, Deserialize)]
pub struct CanvasAnnouncement {
    pub id: String,
    pub title: String,
    pub message: String,
    #[serde(rename = "courseId")]
    pub course_id: String,
}

/// Create topic request for Discourse
#[derive(Debug, Serialize)]
pub struct CreateTopicRequest {
    pub title: String,
    pub raw: String,
    pub category: i32,
}

/// Canvas user data
#[derive(Debug, Deserialize)]
pub struct CanvasUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

/// SSO authentication data for Discourse
#[derive(Debug, Serialize)]
pub struct SSOAuthRequest {
    pub email: String,
    pub external_id: String,
    pub username: String,
    pub name: String,
}

/// Synchronization result data
#[derive(Debug, Serialize)]
pub struct SyncResult {
    pub success: bool,
    pub canvas_announcement_id: Option<String>,
    pub discourse_topic_id: Option<String>,
    pub discourse_topic: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Authentication result data
#[derive(Debug, Serialize)]
pub struct AuthResult {
    pub success: bool,
    pub canvas_user_id: Option<String>,
    pub discourse_user_id: Option<String>,
    pub sso_token: Option<String>,
    pub error: Option<String>,
}

/// Service for integrating Canvas and Discourse
pub struct IntegrationService {
    canvas_client: CanvasClient,
    discourse_client: DiscourseClient,
    logger: log::Logger,
}

impl IntegrationService {
    /// Create a new integration service
    pub fn new(canvas_client: CanvasClient, discourse_client: DiscourseClient) -> Self {
        let logger = create_logger("integration-service", LevelFilter::Info);
        IntegrationService {
            canvas_client,
            discourse_client,
            logger,
        }
    }

    /// Synchronize a Canvas announcement to a Discourse forum topic
    pub async fn sync_announcement_to_forum(&self, announcement: serde_json::Value) -> Result<SyncResult, Box<dyn Error>> {
        // Parse the announcement from the JSON value
        let announcement: CanvasAnnouncement = serde_json::from_value(announcement.clone())?;
        
        self.logger.info(&format!("Syncing announcement \"{}\" to Discourse", announcement.title));
        
        // Get the appropriate category for the course
        let category = self.get_discourse_category(&announcement.course_id).await?;
        
        // Create the topic request
        let topic_request = CreateTopicRequest {
            title: announcement.title.clone(),
            raw: announcement.message.clone(),
            category,
        };
        
        // Create the topic in Discourse
        let topic_result = self.discourse_client.create_topic(&topic_request).await?;
        
        let topic_id = topic_result["topic_id"].as_str().unwrap_or("unknown").to_string();
        self.logger.info(&format!("Created Discourse topic {}", topic_id));
        
        Ok(SyncResult {
            success: true,
            canvas_announcement_id: Some(announcement.id),
            discourse_topic_id: Some(topic_id),
            discourse_topic: Some(topic_result),
            error: None,
        })
    }

    /// Get the appropriate Discourse category for a Canvas course
    async fn get_discourse_category(&self, _course_id: &str) -> Result<i32, Box<dyn Error>> {
        // In a real implementation, this would look up the mapping
        // For now, just return a mock ID
        Ok(5) // Mock category ID
    }

    /// Authenticate a Canvas user with Discourse via SSO
    pub async fn authenticate_user_with_discourse(&self, canvas_user: &CanvasUser) -> Result<AuthResult, Box<dyn Error>> {
        self.logger.info(&format!("Authenticating user {} with Discourse", canvas_user.name));
        
        // Create the username from the email
        let username = canvas_user.email.split('@').next().unwrap_or("user").to_string();
        
        // Create the SSO request
        let sso_request = SSOAuthRequest {
            email: canvas_user.email.clone(),
            external_id: canvas_user.id.clone(),
            username,
            name: canvas_user.name.clone(),
        };
        
        // Authenticate with Discourse
        let sso_result = self.discourse_client.authenticate_sso(&sso_request).await?;
        
        // Generate a random token suffix
        let suffix: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        
        Ok(AuthResult {
            success: true,
            canvas_user_id: Some(canvas_user.id.clone()),
            discourse_user_id: Some(sso_result["id"].as_str().unwrap_or("unknown").to_string()),
            sso_token: Some(format!("sample-token-{}", suffix)),
            error: None,
        })
    }
}
