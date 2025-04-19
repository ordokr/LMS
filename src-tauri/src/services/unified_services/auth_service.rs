use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;
use crate::errors::error::{Error, Result};
use crate::auth::jwt::JwtService;
use crate::auth::password::{hash_password, verify_password, validate_password_strength};
use crate::models::unified_models::User;
use super::base_service::{Service, ServiceConfig, BaseService};

/// Authentication service for user authentication and authorization
#[derive(Debug)]
pub struct AuthService {
    /// Base service
    base: BaseService,
    
    /// JWT secret key
    jwt_secret: Vec<u8>,
    
    /// JWT token expiration time in seconds
    token_expiration: u64,
    
    /// JWT refresh token expiration time in seconds
    refresh_token_expiration: u64,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(config: ServiceConfig, jwt_secret: Vec<u8>) -> Self {
        // Get token expiration from config or use default
        let token_expiration = config.get_parameter_as::<u64>("token_expiration").unwrap_or(3600); // 1 hour
        let refresh_token_expiration = config.get_parameter_as::<u64>("refresh_token_expiration").unwrap_or(604800); // 1 week
        
        Self {
            base: BaseService::new(config),
            jwt_secret,
            token_expiration,
            refresh_token_expiration,
        }
    }
    
    /// Register a new user
    pub async fn register_user(
        &self,
        username: &str,
        email: &str,
        password: &str,
        role: Option<&str>,
    ) -> Result<User> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Validate password strength
        validate_password_strength(password)
            .map_err(|e| Error::validation(e))?;
        
        // Check if username or email already exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE username = ? OR email = ?",
            username,
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to check for existing user: {}", e)))?;
        
        if existing_user.is_some() {
            return Err(Error::conflict("Username or email already exists"));
        }
        
        // Hash password
        let password_hash = hash_password(password)
            .map_err(|e| Error::internal(format!("Failed to hash password: {}", e)))?;
        
        // Generate user ID
        let user_id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::internal(format!("Failed to get system time: {}", e)))?
            .as_secs();
        
        // Determine user role
        let user_role = role.unwrap_or("user");
        
        // Insert user into database
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            user_id,
            username,
            email,
            password_hash,
            user_role,
            now as i64,
            now as i64
        )
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to insert user: {}", e)))?;
        
        // Create and return user
        let user = User::new(
            Some(user_id),
            username.to_string(),
            Some(email.to_string()),
            Some(user_role.to_string()),
        );
        
        Ok(user)
    }
    
    /// Authenticate a user
    pub async fn login(
        &self,
        username_or_email: &str,
        password: &str,
    ) -> Result<(User, String, String)> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Find user by username or email
        let user_record = sqlx::query!(
            "SELECT id, username, email, password_hash, role, created_at, updated_at
             FROM users
             WHERE username = ? OR email = ?",
            username_or_email,
            username_or_email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to find user: {}", e)))?;
        
        let user_record = user_record.ok_or_else(|| Error::authentication("Invalid username or password"))?;
        
        // Verify password
        let is_valid = verify_password(password, &user_record.password_hash)
            .map_err(|e| Error::internal(format!("Failed to verify password: {}", e)))?;
        
        if !is_valid {
            return Err(Error::authentication("Invalid username or password"));
        }
        
        // Create user object
        let user = User::new(
            Some(user_record.id),
            user_record.username,
            Some(user_record.email),
            Some(user_record.role),
        );
        
        // Create JWT service
        let jwt_service = JwtService::new(
            self.jwt_secret.clone(),
            Some(self.token_expiration),
            Some(self.refresh_token_expiration),
        );
        
        // Generate access token
        let access_token = jwt_service.generate_token(
            &user.id,
            &user.role.to_string(),
            Some(&user.username),
            user.email.as_deref(),
        )
        .map_err(|e| Error::internal(format!("Failed to generate access token: {}", e)))?;
        
        // Generate refresh token
        let refresh_token = jwt_service.generate_refresh_token(&user.id)
            .map_err(|e| Error::internal(format!("Failed to generate refresh token: {}", e)))?;
        
        // Update last login timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::internal(format!("Failed to get system time: {}", e)))?
            .as_secs() as i64;
        
        sqlx::query!(
            "UPDATE users SET last_login = ? WHERE id = ?",
            now,
            user.id
        )
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to update last login: {}", e)))?;
        
        Ok((user, access_token, refresh_token))
    }
    
    /// Refresh an access token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
        // Create JWT service
        let jwt_service = JwtService::new(
            self.jwt_secret.clone(),
            Some(self.token_expiration),
            Some(self.refresh_token_expiration),
        );
        
        // Refresh access token
        let access_token = jwt_service.refresh_access_token(refresh_token)
            .map_err(|e| Error::authentication(format!("Failed to refresh token: {}", e)))?;
        
        Ok(access_token)
    }
    
    /// Get a user by ID
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Find user by ID
        let user_record = sqlx::query!(
            "SELECT id, username, email, role, created_at, updated_at
             FROM users
             WHERE id = ?",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to find user: {}", e)))?;
        
        let user_record = user_record.ok_or_else(|| Error::not_found("User not found"))?;
        
        // Create user object
        let user = User::new(
            Some(user_record.id),
            user_record.username,
            Some(user_record.email),
            Some(user_record.role),
        );
        
        Ok(user)
    }
    
    /// Change a user's password
    pub async fn change_password(
        &self,
        user_id: &str,
        current_password: &str,
        new_password: &str,
    ) -> Result<bool> {
        // Get database connection pool
        let pool = self.base.config().get_db_pool()?;
        
        // Validate new password strength
        validate_password_strength(new_password)
            .map_err(|e| Error::validation(e))?;
        
        // Find user by ID
        let user_record = sqlx::query!(
            "SELECT password_hash FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to find user: {}", e)))?;
        
        let user_record = user_record.ok_or_else(|| Error::not_found("User not found"))?;
        
        // Verify current password
        let is_valid = verify_password(current_password, &user_record.password_hash)
            .map_err(|e| Error::internal(format!("Failed to verify password: {}", e)))?;
        
        if !is_valid {
            return Err(Error::authentication("Invalid current password"));
        }
        
        // Hash new password
        let password_hash = hash_password(new_password)
            .map_err(|e| Error::internal(format!("Failed to hash password: {}", e)))?;
        
        // Update password
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| Error::internal(format!("Failed to get system time: {}", e)))?
            .as_secs() as i64;
        
        sqlx::query!(
            "UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?",
            password_hash,
            now,
            user_id
        )
        .execute(pool)
        .await
        .map_err(|e| Error::database(format!("Failed to update password: {}", e)))?;
        
        Ok(true)
    }
    
    /// Verify a JWT token
    pub fn verify_token(&self, token: &str) -> Result<JwtService::Claims> {
        // Create JWT service
        let jwt_service = JwtService::new(
            self.jwt_secret.clone(),
            Some(self.token_expiration),
            Some(self.refresh_token_expiration),
        );
        
        // Verify token
        let claims = jwt_service.verify_token(token)
            .map_err(|e| Error::authentication(format!("Invalid token: {}", e)))?;
        
        Ok(claims)
    }
}

#[async_trait]
impl Service for AuthService {
    fn name(&self) -> &str {
        self.base.name()
    }
    
    async fn init(&self) -> Result<()> {
        // Initialize the service
        self.base.init().await
    }
    
    async fn shutdown(&self) -> Result<()> {
        // Shutdown the service
        self.base.shutdown().await
    }
    
    async fn health_check(&self) -> Result<bool> {
        // Check if the service is healthy
        self.base.health_check().await
    }
}
