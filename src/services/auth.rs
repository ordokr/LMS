use crate::utils::logger::create_logger;
use crate::api::canvas_api::CanvasClient;
use crate::api::discourse_api::DiscourseClient;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use std::error::Error;
use log::LevelFilter;
use std::time::{SystemTime, UNIX_EPOCH};

/// Claims structure for JWT tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// Canvas user data
#[derive(Debug, Deserialize)]
pub struct CanvasUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

/// SSO payload data
#[derive(Debug, Serialize)]
pub struct SSOPayload {
    pub payload: String,
    pub signature: String,
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

/// Service for handling authentication between Canvas and Discourse
pub struct UserAuthService {
    canvas_client: CanvasClient,
    discourse_client: DiscourseClient,
    logger: log::Logger,
}

impl UserAuthService {
    /// Create a new authentication service
    pub fn new(canvas_client: CanvasClient, discourse_client: DiscourseClient) -> Self {
        let logger = create_logger("auth-service", LevelFilter::Info);
        UserAuthService {
            canvas_client,
            discourse_client,
            logger,
        }
    }

    /// Create an SSO payload for Discourse authentication
    pub fn create_sso_payload(&self, canvas_user: &CanvasUser) -> SSOPayload {
        let sso_data = serde_json::json!({
            "external_id": canvas_user.id,
            "email": canvas_user.email,
            "username": canvas_user.email.split('@').next().unwrap_or("user"),
            "name": canvas_user.name
        });

        // In a real implementation, this would encode and sign the payload
        SSOPayload {
            payload: base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD, 
                serde_json::to_string(&sso_data).unwrap_or_default()
            ),
            signature: "mock-signature".to_string(),
        }
    }

    /// Authenticate a Canvas user with Discourse
    pub async fn authenticate_user(&self, canvas_user: &CanvasUser) -> AuthResult {
        self.logger.info(&format!("Authenticating user {}", canvas_user.email));
        
        let sso_data = self.create_sso_payload(canvas_user);
        
        match self.discourse_client.authenticate_sso(&sso_data).await {
            Ok(result) => {
                // Generate a random token suffix for demo purposes
                use rand::{thread_rng, Rng};
                use rand::distributions::Alphanumeric;
                let suffix: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();
                
                AuthResult {
                    success: true,
                    canvas_user_id: Some(canvas_user.id.clone()),
                    discourse_user_id: Some(result.id),
                    sso_token: Some(format!("sample-token-{}", suffix)),
                    error: None,
                }
            }
            Err(e) => {
                self.logger.error(&format!("Authentication failed: {}", e));
                AuthResult {
                    success: false,
                    canvas_user_id: None,
                    discourse_user_id: None,
                    sso_token: None,
                    error: Some(e.to_string()),
                }
            }
        }
    }
}

/// Generate a JWT token with the provided payload
pub fn generate_jwt(user_id: &str, email: Option<&str>, name: Option<&str>, roles: Option<&[String]>) -> Result<String, Box<dyn Error>> {
    let secret_key = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    // Calculate expiration (1 hour from now)
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let expiration = now + 3600; // 1 hour
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
        email: email.map(|s| s.to_string()),
        name: name.map(|s| s.to_string()),
        roles: roles.map(|r| r.to_vec()),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes())
    )?;
    
    Ok(token)
}

/// Verify a given JWT token
pub fn verify_jwt(token: &str) -> Result<Claims, Box<dyn Error>> {
    let secret_key = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &validation
    )?;
    
    Ok(token_data.claims)
}
