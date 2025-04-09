use crate::models::user::{User, UserProfile, UserProfileUpdate, UserRole};
use sqlx::{Pool, Sqlite};
use async_trait::async_trait;
use tracing::{info, error, instrument};
use argon2::{self, Config};
use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum DbError {
    #[error("Query error: {0}")]
    QueryError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Data error: {0}")]
    DataError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, email: &str, password: &str, first_name: &str, last_name: &str, role: UserRole) -> Result<User, DbError>;
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, DbError>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DbError>;
    async fn authenticate_user(&self, email: &str, password: &str) -> Result<Option<User>, DbError>;
    async fn get_user_profile(&self, user_id: &str) -> Result<Option<UserProfile>, DbError>;
    async fn update_user_profile(&self, user_id: &str, update: UserProfileUpdate) -> Result<UserProfile, DbError>;
    async fn get_user_preferences(&self, user_id: &str) -> Result<Value, DbError>;
    async fn update_user_preferences(&self, user_id: &str, preferences: Value) -> Result<Value, DbError>;
    async fn get_user_integration_settings(&self, user_id: &str) -> Result<Value, DbError>;
    async fn update_user_integration_settings(&self, user_id: &str, settings: Value) -> Result<Value, DbError>;
}

pub struct SqliteUserRepository {
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    fn hash_password(&self, password: &str) -> Result<String, DbError> {
        let salt = b"randomsaltvalue"; // In production, use a proper salt
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt, &config)
            .map_err(|e| DbError::AuthError(format!("Password hashing failed: {}", e)))
    }
    
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, DbError> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|e| DbError::AuthError(format!("Password verification failed: {}", e)))
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    #[instrument(skip(self, password), err)]
    async fn create_user(
        &self, 
        email: &str, 
        password: &str, 
        first_name: &str, 
        last_name: &str, 
        role: UserRole
    ) -> Result<User, DbError> {
        // Check if user with email already exists
        if let Ok(Some(_)) = self.get_user_by_email(email).await {
            return Err(DbError::DataError(format!("User with email {} already exists", email)));
        }
        
        // Hash password
        let password_hash = self.hash_password(password)?;
        
        // Generate UUID for user
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        
        // Create user
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, first_name, last_name, role, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(email)
        .bind(&password_hash)
        .bind(first_name)
        .bind(last_name)
        .bind(&role.to_string())
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        // Create default profile record
        sqlx::query(
            "INSERT INTO user_profiles (user_id, bio, avatar_url, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind("")
        .bind("")
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        // Create default preferences
        let default_prefs = serde_json::json!({
            "notifications": {
                "email": true,
                "push": true,
                "discussion_replies": true,
                "assignment_grades": true
            },
            "display": {
                "theme": "light",
                "font_size": "medium",
                "sidebar_collapsed": false
            }
        });
        
        sqlx::query(
            "INSERT INTO user_preferences (user_id, preferences, created_at, updated_at)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&default_prefs.to_string())
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        // Create default integration settings
        let default_settings = serde_json::json!({
            "discourse": {
                "auto_sync": true,
                "notify_on_replies": true
            },
            "canvas": {
                "auto_sync": true
            }
        });
        
        sqlx::query(
            "INSERT INTO user_integration_settings (user_id, settings, created_at, updated_at)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&default_settings.to_string())
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        // Return created user
        let user = User {
            id,
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            role,
            created_at: now.clone(),
            updated_at: now,
        };
        
        info!("Created new user with ID: {}", user.id);
        Ok(user)
    }
    
    #[instrument(skip(self), err)]
    async fn get_user_by_id(&self, id: &str) -> Result<Option<User>, DbError> {
        // We don't select password_hash for security
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, first_name, last_name, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        Ok(user)
    }
    
    #[instrument(skip(self), err)]
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DbError> {
        // We don't select password_hash for security
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, first_name, last_name, role as "role: UserRole", created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        Ok(user)
    }
    
    #[instrument(skip(self, password), err)]
    async fn authenticate_user(&self, email: &str, password: &str) -> Result<Option<User>, DbError> {
        // First get the user with password_hash
        let result = sqlx::query!(
            r#"
            SELECT id, password_hash
            FROM users
            WHERE email = ?
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        // If user not found or password doesn't match, return None
        match result {
            Some(row) if self.verify_password(&row.password_hash, password)? => {
                // Password matches, get full user details
                self.get_user_by_id(&row.id).await
            },
            _ => Ok(None),
        }
    }
    
    #[instrument(skip(self), err)]
    async fn get_user_profile(&self, user_id: &str) -> Result<Option<UserProfile>, DbError> {
        // First check if user exists
        if let Ok(None) = self.get_user_by_id(user_id).await {
            return Ok(None);
        }
        
        // Get profile data
        let profile = sqlx::query_as!(
            UserProfile,
            r#"
            SELECT u.id as user_id, 
                   u.first_name, 
                   u.last_name, 
                   u.email,
                   p.bio, 
                   p.avatar_url,
                   p.created_at,
                   p.updated_at
            FROM users u
            LEFT JOIN user_profiles p ON u.id = p.user_id
            WHERE u.id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        Ok(profile)
    }
    
    #[instrument(skip(self), err)]
    async fn update_user_profile(&self, user_id: &str, update: UserProfileUpdate) -> Result<UserProfile, DbError> {
        // First check if profile exists
        if let Ok(None) = self.get_user_profile(user_id).await {
            return Err(DbError::DataError(format!("User profile not found for user ID: {}", user_id)));
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        let mut tx = self.pool.begin().await.map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Update user data if provided
        if update.first_name.is_some() || update.last_name.is_some() || update.email.is_some() {
            let mut query = "UPDATE users SET updated_at = ?".to_string();
            let mut params: Vec<String> = Vec::new();
            params.push(now.clone());
            
            if let Some(first_name) = &update.first_name {
                query.push_str(", first_name = ?");
                params.push(first_name.clone());
            }
            
            if let Some(last_name) = &update.last_name {
                query.push_str(", last_name = ?");
                params.push(last_name.clone());
            }
            
            if let Some(email) = &update.email {
                query.push_str(", email = ?");
                params.push(email.clone());
            }
            
            query.push_str(" WHERE id = ?");
            params.push(user_id.to_string());
            
            // Build and execute the dynamic query
            let mut query_builder = sqlx::QueryBuilder::new(query);
            for param in params {
                query_builder.push_bind(param);
            }
            
            query_builder.build()
                .execute(&mut tx)
                .await
                .map_err(|e| DbError::QueryError(e.to_string()))?;
        }
        
        // Update profile data
        if update.bio.is_some() || update.avatar_url.is_some() {
            let mut query = "UPDATE user_profiles SET updated_at = ?".to_string();
            let mut params: Vec<String> = Vec::new();
            params.push(now.clone());
            
            if let Some(bio) = &update.bio {
                query.push_str(", bio = ?");
                params.push(bio.clone());
            }
            
            if let Some(avatar_url) = &update.avatar_url {
                query.push_str(", avatar_url = ?");
                params.push(avatar_url.clone());
            }
            
            query.push_str(" WHERE user_id = ?");
            params.push(user_id.to_string());
            
            // Build and execute the dynamic query
            let mut query_builder = sqlx::QueryBuilder::new(query);
            for param in params {
                query_builder.push_bind(param);
            }
            
            query_builder.build()
                .execute(&mut tx)
                .await
                .map_err(|e| DbError::QueryError(e.to_string()))?;
        }
        
        // Commit transaction
        tx.commit().await.map_err(|e| DbError::ConnectionError(e.to_string()))?;
        
        // Get updated profile
        self.get_user_profile(user_id).await?
            .ok_or_else(|| DbError::DataError(format!("Failed to retrieve updated profile for user ID: {}", user_id)))
    }
    
    #[instrument(skip(self), err)]
    async fn get_user_preferences(&self, user_id: &str) -> Result<Value, DbError> {
        // First check if user exists
        if let Ok(None) = self.get_user_by_id(user_id).await {
            return Err(DbError::DataError(format!("User not found with ID: {}", user_id)));
        }
        
        // Get preferences
        let result = sqlx::query!(
            "SELECT preferences FROM user_preferences WHERE user_id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        match result {
            Some(row) => {
                let preferences: Value = serde_json::from_str(&row.preferences)
                    .map_err(|e| DbError::DataError(format!("Failed to parse preferences: {}", e)))?;
                Ok(preferences)
            },
            None => {
                // Create default preferences if none exist
                let default_prefs = serde_json::json!({
                    "notifications": {
                        "email": true,
                        "push": true,
                        "discussion_replies": true,
                        "assignment_grades": true
                    },
                    "display": {
                        "theme": "light",
                        "font_size": "medium",
                        "sidebar_collapsed": false
                    }
                });
                
                let now = chrono::Utc::now().to_rfc3339();
                sqlx::query(
                    "INSERT INTO user_preferences (user_id, preferences, created_at, updated_at)
                     VALUES (?, ?, ?, ?)"
                )
                .bind(user_id)
                .bind(&default_prefs.to_string())
                .bind(&now)
                .bind(&now)
                .execute(&self.pool)
                .await
                .map_err(|e| DbError::QueryError(e.to_string()))?;
                
                Ok(default_prefs)
            }
        }
    }
    
    #[instrument(skip(self), err)]
    async fn update_user_preferences(&self, user_id: &str, preferences: Value) -> Result<Value, DbError> {
        // First check if user exists
        if let Ok(None) = self.get_user_by_id(user_id).await {
            return Err(DbError::DataError(format!("User not found with ID: {}", user_id)));
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        let prefs_str = preferences.to_string();
        
        // Update or insert preferences
        let result = sqlx::query(
            "INSERT INTO user_preferences (user_id, preferences, created_at, updated_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(user_id) DO UPDATE SET
             preferences = excluded.preferences,
             updated_at = excluded.updated_at"
        )
        .bind(user_id)
        .bind(&prefs_str)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        info!(
            rows_affected = %result.rows_affected(),
            "Updated user preferences for user ID: {}",
            user_id
        );
        
        Ok(preferences)
    }
    
    #[instrument(skip(self), err)]
    async fn get_user_integration_settings(&self, user_id: &str) -> Result<Value, DbError> {
        // First check if user exists
        if let Ok(None) = self.get_user_by_id(user_id).await {
            return Err(DbError::DataError(format!("User not found with ID: {}", user_id)));
        }
        
        // Get integration settings
        let result = sqlx::query!(
            "SELECT settings FROM user_integration_settings WHERE user_id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        match result {
            Some(row) => {
                let settings: Value = serde_json::from_str(&row.settings)
                    .map_err(|e| DbError::DataError(format!("Failed to parse settings: {}", e)))?;
                Ok(settings)
            },
            None => {
                // Create default integration settings if none exist
                let default_settings = serde_json::json!({
                    "discourse": {
                        "auto_sync": true,
                        "notify_on_replies": true
                    },
                    "canvas": {
                        "auto_sync": true
                    }
                });
                
                let now = chrono::Utc::now().to_rfc3339();
                sqlx::query(
                    "INSERT INTO user_integration_settings (user_id, settings, created_at, updated_at)
                     VALUES (?, ?, ?, ?)"
                )
                .bind(user_id)
                .bind(&default_settings.to_string())
                .bind(&now)
                .bind(&now)
                .execute(&self.pool)
                .await
                .map_err(|e| DbError::QueryError(e.to_string()))?;
                
                Ok(default_settings)
            }
        }
    }
    
    #[instrument(skip(self), err)]
    async fn update_user_integration_settings(&self, user_id: &str, settings: Value) -> Result<Value, DbError> {
        // First check if user exists
        if let Ok(None) = self.get_user_by_id(user_id).await {
            return Err(DbError::DataError(format!("User not found with ID: {}", user_id)));
        }
        
        let now = chrono::Utc::now().to_rfc3339();
        let settings_str = settings.to_string();
        
        // Update or insert integration settings
        let result = sqlx::query(
            "INSERT INTO user_integration_settings (user_id, settings, created_at, updated_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(user_id) DO UPDATE SET
             settings = excluded.settings,
             updated_at = excluded.updated_at"
        )
        .bind(user_id)
        .bind(&settings_str)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| DbError::QueryError(e.to_string()))?;
        
        info!(
            rows_affected = %result.rows_affected(),
            "Updated user integration settings for user ID: {}",
            user_id
        );
        
        Ok(settings)
    }
}