use crate::auth::jwt_service::{JwtService, JwtError};
use crate::auth::canvas_auth_service::{CanvasAuthService, CanvasAuthError};
use crate::auth::discourse_sso_service::{DiscourseSsoService, SsoUser, DiscourseSsoError};
use crate::models::unified::User;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tauri::{command, State, AppHandle};
use log::{info, error};

// Auth state that will be managed by Tauri
pub struct AuthState {
    pub jwt_service: Arc<JwtService>,
    pub canvas_auth_service: Arc<CanvasAuthService>,
    pub discourse_sso_service: Arc<DiscourseSsoService>,
}

// Implement Default for AuthState to allow easy initialization during app setup
impl Default for AuthState {
    fn default() -> Self {
        // These values should be loaded from configuration in a real application
        let jwt_service = Arc::new(JwtService::new("jwt_secret_placeholder", 3600, 86400));
        let canvas_auth_service = Arc::new(CanvasAuthService::new("https://canvas.example.com/api"));
        let discourse_sso_service = Arc::new(DiscourseSsoService::new("discourse_sso_secret_placeholder"));
        
        Self {
            jwt_service,
            canvas_auth_service,
            discourse_sso_service,
        }
    }
}

// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub source: Option<String>,
}

// Login response
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: Option<User>,
    pub error: Option<String>,
}

// SSO request payload
#[derive(Debug, Deserialize)]
pub struct SsoRequest {
    pub sso: String,
    pub sig: String,
    pub canvas_token: String,
}

// SSO response
#[derive(Debug, Serialize)]
pub struct SsoResponse {
    pub redirect_url: Option<String>,
    pub error: Option<String>,
}

// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Login command
#[command]
pub async fn login(
    auth_state: State<'_, AuthState>,
    request: LoginRequest,
) -> Result<LoginResponse, String> {
    info!("Processing login request from source: {:?}", request.source);
    
    // Determine the authentication source (Canvas by default)
    let source = request.source.unwrap_or_else(|| "canvas".to_string());
    
    match source.as_str() {
        "canvas" => {
            // Authenticate through Canvas API
            match auth_state.canvas_auth_service.authenticate_canvas_user(&request.password).await {
                Ok(user) => {
                    // Generate JWT token
                    let token = match auth_state.jwt_service.generate_token(
                        &user.id.clone().unwrap_or_default(),
                        &user.email.clone().unwrap_or_default(),
                        &user.name.clone().unwrap_or_default(),
                        user.roles.clone().unwrap_or_default(),
                        user.canvas_id.clone(),
                        user.discourse_id.clone(),
                    ) {
                        Ok(token) => token,
                        Err(e) => {
                            error!("Failed to generate JWT token: {}", e);
                            return Ok(LoginResponse {
                                token: String::new(),
                                user: None,
                                error: Some("Failed to generate authentication token".to_string()),
                            });
                        }
                    };
                    
                    info!("User successfully authenticated and JWT token generated");
                    
                    Ok(LoginResponse {
                        token,
                        user: Some(user),
                        error: None,
                    })
                },
                Err(e) => {
                    error!("Canvas authentication failed: {}", e);
                    Ok(LoginResponse {
                        token: String::new(),
                        user: None,
                        error: Some("Authentication failed".to_string()),
                    })
                }
            }
        },
        "discourse" => {
            // For now, we'll just return an error as Discourse direct login 
            // isn't implemented in this sample
            error!("Discourse direct login not implemented yet");
            
            Ok(LoginResponse {
                token: String::new(),
                user: None,
                error: Some("Discourse authentication not implemented".to_string()),
            })
        },
        _ => {
            error!("Unknown authentication source: {}", source);
            
            Ok(LoginResponse {
                token: String::new(),
                user: None,
                error: Some(format!("Unknown authentication source: {}", source)),
            })
        }
    }
}

// Discourse SSO command
#[command]
pub async fn handle_discourse_sso(
    auth_state: State<'_, AuthState>,
    app_handle: AppHandle,
    request: SsoRequest,
) -> Result<SsoResponse, String> {
    info!("Processing Discourse SSO request");
    
    // First, authenticate the user with Canvas
    match auth_state.canvas_auth_service.authenticate_canvas_user(&request.canvas_token).await {
        Ok(user) => {
            // Now create an SSO payload for Discourse
            let sso_user = SsoUser {
                id: user.id.unwrap_or_default(),
                email: user.email.unwrap_or_default(),
                name: user.name.unwrap_or_default(),
                roles: user.roles.unwrap_or_default(),
            };
            
            match auth_state.discourse_sso_service.generate_sso_payload(&sso_user, &request.sso, &request.sig) {
                Ok(payload) => {
                    // In a real application, this URL would come from configuration
                    let discourse_url = "https://discourse.example.com";
                    let redirect_url = format!("{}/session/sso_login?{}", discourse_url, payload);
                    
                    info!("Successfully generated SSO payload and redirect URL");
                    
                    // For desktop apps, we can use shell::open to open the browser
                    if let Err(e) = tauri::api::shell::open(&app_handle.shell_scope(), &redirect_url, None) {
                        error!("Failed to open browser for SSO redirect: {}", e);
                        
                        return Ok(SsoResponse {
                            redirect_url: Some(redirect_url),
                            error: Some("Failed to open browser, please use the URL manually".to_string()),
                        });
                    }
                    
                    Ok(SsoResponse {
                        redirect_url: Some(redirect_url),
                        error: None,
                    })
                },
                Err(e) => {
                    error!("Failed to generate SSO payload: {}", e);
                    
                    Ok(SsoResponse {
                        redirect_url: None,
                        error: Some(format!("SSO authentication failed: {}", e)),
                    })
                }
            }
        },
        Err(e) => {
            error!("Canvas authentication failed during SSO process: {}", e);
            
            Ok(SsoResponse {
                redirect_url: None,
                error: Some("Canvas authentication required".to_string()),
            })
        }
    }
}

// Verify token command
#[command]
pub fn verify_token(
    auth_state: State<'_, AuthState>,
    token: String,
) -> Result<bool, String> {
    match auth_state.jwt_service.verify_token(&token) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

// Register these commands with Tauri
pub fn register_commands(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
