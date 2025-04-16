use std::env;
use once_cell::sync::Lazy;

/// Application configuration that loads from environment variables with defaults
pub struct Config {
    /// Secret key for JWT token generation and validation
    pub jwt_secret: String,

    /// JWT token expiration time (e.g. "24h")
    pub jwt_expiration: String,

    /// Base URL for the Discourse API
    pub discourse_url: String,

    /// Base URL for the Canvas API
    pub canvas_api_url: String,

    /// Canvas OAuth client ID
    pub canvas_oauth_client_id: String,

    /// Canvas OAuth client secret
    pub canvas_oauth_client_secret: String,

    /// Canvas OAuth authorize URL
    pub canvas_oauth_authorize_url: String,

    /// Canvas OAuth token URL
    pub canvas_oauth_token_url: String,

    /// Canvas OAuth redirect URL
    pub canvas_oauth_redirect_url: String,
}

impl Config {
    /// Create a new configuration with values from environment variables or defaults
    pub fn new() -> Self {
        Self {
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-development-secret-key".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "24h".to_string()),
            discourse_url: env::var("DISCOURSE_URL")
                .unwrap_or_else(|_| "https://discourse.example.com".to_string()),
            canvas_api_url: env::var("CANVAS_API_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/api".to_string()),
            canvas_oauth_client_id: env::var("CANVAS_OAUTH_CLIENT_ID")
                .unwrap_or_else(|_| "".to_string()),
            canvas_oauth_client_secret: env::var("CANVAS_OAUTH_CLIENT_SECRET")
                .unwrap_or_else(|_| "".to_string()),
            canvas_oauth_authorize_url: env::var("CANVAS_OAUTH_AUTHORIZE_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/login/oauth2/auth".to_string()),
            canvas_oauth_token_url: env::var("CANVAS_OAUTH_TOKEN_URL")
                .unwrap_or_else(|_| "https://canvas.example.com/login/oauth2/token".to_string()),
            canvas_oauth_redirect_url: env::var("CANVAS_OAUTH_REDIRECT_URL")
                .unwrap_or_else(|_| "http://localhost:8080/canvas/callback".to_string()),
        }
    }
}

/// Global static configuration instance
pub static CONFIG: Lazy<Config> = Lazy::new(Config::new);

/// Get the global configuration
pub fn get_config() -> &'static Config {
    &CONFIG
}
