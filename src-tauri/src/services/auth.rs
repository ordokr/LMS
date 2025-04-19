use sqlx::{Pool, Sqlite};
use anyhow::{Result, Context, anyhow};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::auth::jwt::JwtService;
use crate::auth::password::{hash_password, verify_password, validate_password_strength};
use crate::models::user::{User, UserRole, NewUser};

/// Authentication service for user authentication and authorization
pub struct AuthService {
    /// Database connection pool
    db_pool: Pool<Sqlite>,
    /// JWT secret key
    jwt_secret: Vec<u8>,
}

impl AuthService {
    /// Create a new authentication service
    ///
    /// # Arguments
    /// * `db_pool` - Database connection pool
    /// * `jwt_secret` - JWT secret key
    pub fn new(db_pool: Pool<Sqlite>, jwt_secret: Vec<u8>) -> Self {
        Self {
            db_pool,
            jwt_secret,
        }
    }
    
    /// Register a new user
    ///
    /// # Arguments
    /// * `username` - Username
    /// * `email` - Email address
    /// * `password` - Password
    /// * `role` - User role (default: "user")
    ///
    /// # Returns
    /// A Result containing the new user or an error
    pub async fn register_user(&self, username: &str, email: &str, password: &str, role: Option<&str>) -> Result<User> {
        // Validate password strength
        validate_password_strength(password)
            .map_err(|e| anyhow!(e))?;
        
        // Check if username or email already exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE username = ? OR email = ?",
            username,
            email
        )
        .fetch_optional(&self.db_pool)
        .await
        .context("Failed to check for existing user")?;
        
        if existing_user.is_some() {
            return Err(anyhow!("Username or email already exists"));
        }
        
        // Hash password
        let password_hash = hash_password(password)?;
        
        // Generate user ID
        let user_id = Uuid::new_v4().to_string();
        
        // Get current timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();
        
        // Determine user role
        let user_role = role.unwrap_or("user");
        
        // Create new user
        let new_user = NewUser {
            id: user_id.clone(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            role: user_role.to_string(),
            created_at: now as i64,
            updated_at: now as i64,
        };
        
        // Insert user into database
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            new_user.id,
            new_user.username,
            new_user.email,
            new_user.password_hash,
            new_user.role,
            new_user.created_at,
            new_user.updated_at
        )
        .execute(&self.db_pool)
        .await
        .context("Failed to insert user")?;
        
        // Return user
        let user = User {
            id: new_user.id,
            username: new_user.username,
            email: new_user.email,
            role: UserRole::from_str(&new_user.role),
            created_at: new_user.created_at,
            updated_at: new_user.updated_at,
        };
        
        Ok(user)
    }
    
    /// Authenticate a user
    ///
    /// # Arguments
    /// * `username_or_email` - Username or email
    /// * `password` - Password
    ///
    /// # Returns
    /// A Result containing the user and tokens or an error
    pub async fn login(&self, username_or_email: &str, password: &str) -> Result<(User, String, String)> {
        // Find user by username or email
        let user_record = sqlx::query!(
            "SELECT id, username, email, password_hash, role, created_at, updated_at
             FROM users
             WHERE username = ? OR email = ?",
            username_or_email,
            username_or_email
        )
        .fetch_optional(&self.db_pool)
        .await
        .context("Failed to find user")?;
        
        let user_record = user_record.ok_or_else(|| anyhow!("Invalid username or password"))?;
        
        // Verify password
        let is_valid = verify_password(password, &user_record.password_hash)?;
        
        if !is_valid {
            return Err(anyhow!("Invalid username or password"));
        }
        
        // Create user object
        let user = User {
            id: user_record.id,
            username: user_record.username,
            email: user_record.email,
            role: UserRole::from_str(&user_record.role),
            created_at: user_record.created_at,
            updated_at: user_record.updated_at,
        };
        
        // Create JWT service
        let jwt_service = JwtService::new(self.jwt_secret.clone(), None);
        
        // Generate access token
        let access_token = jwt_service.generate_token(
            &user.id,
            &user.role.to_string(),
            Some(&user.username),
            Some(&user.email),
        )?;
        
        // Generate refresh token
        let refresh_token = jwt_service.generate_refresh_token(&user.id)?;
        
        // Update last login timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs() as i64;
        
        sqlx::query!(
            "UPDATE users SET last_login = ? WHERE id = ?",
            now,
            user.id
        )
        .execute(&self.db_pool)
        .await
        .context("Failed to update last login")?;
        
        Ok((user, access_token, refresh_token))
    }
    
    /// Refresh an access token
    ///
    /// # Arguments
    /// * `refresh_token` - Refresh token
    ///
    /// # Returns
    /// A Result containing the new access token or an error
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String> {
        // Create JWT service
        let jwt_service = JwtService::new(self.jwt_secret.clone(), None);
        
        // Refresh access token
        let access_token = jwt_service.refresh_access_token(refresh_token)?;
        
        Ok(access_token)
    }
    
    /// Get a user by ID
    ///
    /// # Arguments
    /// * `user_id` - User ID
    ///
    /// # Returns
    /// A Result containing the user or an error
    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        // Find user by ID
        let user_record = sqlx::query!(
            "SELECT id, username, email, role, created_at, updated_at
             FROM users
             WHERE id = ?",
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .context("Failed to find user")?;
        
        let user_record = user_record.ok_or_else(|| anyhow!("User not found"))?;
        
        // Create user object
        let user = User {
            id: user_record.id,
            username: user_record.username,
            email: user_record.email,
            role: UserRole::from_str(&user_record.role),
            created_at: user_record.created_at,
            updated_at: user_record.updated_at,
        };
        
        Ok(user)
    }
    
    /// Change a user's password
    ///
    /// # Arguments
    /// * `user_id` - User ID
    /// * `current_password` - Current password
    /// * `new_password` - New password
    ///
    /// # Returns
    /// A Result containing a boolean indicating success or an error
    pub async fn change_password(&self, user_id: &str, current_password: &str, new_password: &str) -> Result<bool> {
        // Validate new password strength
        validate_password_strength(new_password)
            .map_err(|e| anyhow!(e))?;
        
        // Find user by ID
        let user_record = sqlx::query!(
            "SELECT password_hash FROM users WHERE id = ?",
            user_id
        )
        .fetch_optional(&self.db_pool)
        .await
        .context("Failed to find user")?;
        
        let user_record = user_record.ok_or_else(|| anyhow!("User not found"))?;
        
        // Verify current password
        let is_valid = verify_password(current_password, &user_record.password_hash)?;
        
        if !is_valid {
            return Err(anyhow!("Invalid current password"));
        }
        
        // Hash new password
        let password_hash = hash_password(new_password)?;
        
        // Update password
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs() as i64;
        
        sqlx::query!(
            "UPDATE users SET password_hash = ?, updated_at = ? WHERE id = ?",
            password_hash,
            now,
            user_id
        )
        .execute(&self.db_pool)
        .await
        .context("Failed to update password")?;
        
        Ok(true)
    }
}
