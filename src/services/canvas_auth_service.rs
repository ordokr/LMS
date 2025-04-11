use crate::config::Config;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use log::error;

/// Canvas user data structure
#[derive(Debug, Deserialize)]
struct CanvasUser {
    id: String,
    email: String,
    name: String,
    #[serde(default)]
    roles: Vec<String>,
}

/// Internal user representation
#[derive(Debug, Serialize)]
pub struct InternalUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub roles: Vec<String>,
    pub canvas_id: String,
    pub source: String,
}

/// Authenticates a user through Canvas OAuth
///
/// # Arguments
///
/// * `token` - Canvas OAuth token
///
/// # Returns
///
/// * `Option<InternalUser>` - User object if authentication successful, None otherwise
pub async fn authenticate_canvas_user(token: &str, config: &Config) -> Option<InternalUser> {
    let client = reqwest::Client::new();
    
    let mut headers = HeaderMap::new();
    let auth_header = format!("Bearer {}", token);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header).ok()?
    );
    
    match client.get(&format!("{}/users/self", config.canvas_api_url))
        .headers(headers)
        .send()
        .await {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<CanvasUser>().await {
                        Ok(canvas_user) => Some(map_canvas_user_to_internal(canvas_user)),
                        Err(e) => {
                            error!("Failed to parse Canvas user data: {}", e);
                            None
                        }
                    }
                } else {
                    error!("Canvas API returned error status: {}", response.status());
                    None
                }
            },
            Err(e) => {
                error!("Canvas authentication error: {}", e);
                None
            }
        }
}

/// Maps Canvas user data to internal user model
fn map_canvas_user_to_internal(canvas_user: CanvasUser) -> InternalUser {
    InternalUser {
        id: canvas_user.id.clone(),
        email: canvas_user.email,
        name: canvas_user.name,
        roles: determine_user_roles(&canvas_user),
        canvas_id: canvas_user.id,
        source: "canvas".to_string(),
    }
}

/// Determines user roles based on Canvas user data
fn determine_user_roles(canvas_user: &CanvasUser) -> Vec<String> {
    // In the JavaScript version, this function wasn't implemented
    // This is a placeholder implementation that just returns the roles from Canvas
    // Should be expanded based on business logic
    canvas_user.roles.clone()
}
