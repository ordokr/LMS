use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use argon2::{self, Config};
use crate::errors::error::{Error, Result};
use crate::models::unified_models::User;
use super::base_repository::Repository;
use super::user_repository::UserRepository;

/// SQLite implementation of the user repository
#[derive(Debug)]
pub struct SqliteUserRepository {
    /// Database connection pool
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    /// Create a new SQLite user repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a User
    async fn row_to_user(&self, id: &str) -> Result<Option<User>> {
        let user_row = sqlx::query!(
            r#"
            SELECT 
                id, name, email, username, avatar_url, created_at, updated_at, last_seen_at,
                canvas_id, discourse_id, sis_id, lti_id, bio, location, website, timezone,
                sortable_name, short_name, post_count, source_system, trust_level,
                is_admin, is_moderator, roles, metadata
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to get user: {}", e)))?;
        
        if let Some(row) = user_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::parsing(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::parsing(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
                
            let last_seen_at = if let Some(ts) = row.last_seen_at {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .map_err(|e| Error::parsing(format!("Failed to parse last_seen_at: {}", e)))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            // Parse roles
            let roles: Vec<String> = serde_json::from_str(&row.roles)
                .map_err(|e| Error::parsing(format!("Failed to parse roles: {}", e)))?;
                
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::parsing(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Create user
            let user = User {
                id: row.id,
                name: row.name,
                email: row.email,
                username: row.username,
                avatar_url: row.avatar_url,
                created_at,
                updated_at,
                last_seen_at,
                roles,
                trust_level: row.trust_level,
                is_admin: row.is_admin != 0,
                is_moderator: row.is_moderator != 0,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                sis_id: row.sis_id,
                lti_id: row.lti_id,
                bio: row.bio,
                location: row.location,
                website: row.website,
                timezone: row.timezone,
                sortable_name: row.sortable_name,
                short_name: row.short_name,
                post_count: row.post_count,
                source_system: row.source_system,
                metadata,
            };
            
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
    
    /// Hash a password
    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = b"randomsaltvalue"; // In production, use a proper salt
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt, &config)
            .map_err(|e| Error::internal(format!("Password hashing failed: {}", e)))
    }
    
    /// Verify a password
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|e| Error::internal(format!("Password verification failed: {}", e)))
    }
}

#[async_trait]
impl Repository<User, String> for SqliteUserRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<User>> {
        self.row_to_user(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        let user_ids = sqlx::query!(
            "SELECT id FROM users"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to get users: {}", e)))?;
        
        let mut users = Vec::new();
        for row in user_ids {
            if let Some(user) = self.row_to_user(&row.id).await? {
                users.push(user);
            }
        }
        
        Ok(users)
    }
    
    async fn create(&self, user: &User) -> Result<User> {
        // Serialize roles and metadata
        let roles_json = serde_json::to_string(&user.roles)
            .map_err(|e| Error::parsing(format!("Failed to serialize roles: {}", e)))?;
            
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| Error::parsing(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert user
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, name, email, username, avatar_url, created_at, updated_at, last_seen_at,
                canvas_id, discourse_id, sis_id, lti_id, bio, location, website, timezone,
                sortable_name, short_name, post_count, source_system, trust_level,
                is_admin, is_moderator, roles, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            user.id,
            user.name,
            user.email,
            user.username,
            user.avatar_url,
            user.created_at.to_rfc3339(),
            user.updated_at.to_rfc3339(),
            user.last_seen_at.map(|dt| dt.to_rfc3339()),
            user.canvas_id,
            user.discourse_id,
            user.sis_id,
            user.lti_id,
            user.bio,
            user.location,
            user.website,
            user.timezone,
            user.sortable_name,
            user.short_name,
            user.post_count,
            user.source_system,
            user.trust_level,
            if user.is_admin { 1 } else { 0 },
            if user.is_moderator { 1 } else { 0 },
            roles_json,
            metadata_json
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to create user: {}", e)))?;
        
        // Return the created user
        Ok(user.clone())
    }
    
    async fn update(&self, user: &User) -> Result<User> {
        // Serialize roles and metadata
        let roles_json = serde_json::to_string(&user.roles)
            .map_err(|e| Error::parsing(format!("Failed to serialize roles: {}", e)))?;
            
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| Error::parsing(format!("Failed to serialize metadata: {}", e)))?;
        
        // Update user
        sqlx::query!(
            r#"
            UPDATE users SET
                name = ?, email = ?, username = ?, avatar_url = ?, updated_at = ?, last_seen_at = ?,
                canvas_id = ?, discourse_id = ?, sis_id = ?, lti_id = ?, bio = ?, location = ?,
                website = ?, timezone = ?, sortable_name = ?, short_name = ?, post_count = ?,
                source_system = ?, trust_level = ?, is_admin = ?, is_moderator = ?,
                roles = ?, metadata = ?
            WHERE id = ?
            "#,
            user.name,
            user.email,
            user.username,
            user.avatar_url,
            user.updated_at.to_rfc3339(),
            user.last_seen_at.map(|dt| dt.to_rfc3339()),
            user.canvas_id,
            user.discourse_id,
            user.sis_id,
            user.lti_id,
            user.bio,
            user.location,
            user.website,
            user.timezone,
            user.sortable_name,
            user.short_name,
            user.post_count,
            user.source_system,
            user.trust_level,
            if user.is_admin { 1 } else { 0 },
            if user.is_moderator { 1 } else { 0 },
            roles_json,
            metadata_json,
            user.id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to update user: {}", e)))?;
        
        // Return the updated user
        Ok(user.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to delete user: {}", e)))?;
            
        Ok(())
    }
    
    async fn count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to count users: {}", e)))?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE email = ?", email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to find user by email: {}", e)))?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE username = ?", username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to find user by username: {}", e)))?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<User>> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE canvas_id = ?", canvas_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to find user by Canvas ID: {}", e)))?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<User>> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE discourse_id = ?", discourse_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to find user by Discourse ID: {}", e)))?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_role(&self, role: &str) -> Result<Vec<User>> {
        // This is a bit tricky since roles are stored as JSON
        // We'll need to fetch all users and filter in memory
        let users = self.find_all().await?;
        
        let filtered_users = users.into_iter()
            .filter(|user| user.roles.contains(&role.to_string()))
            .collect();
            
        Ok(filtered_users)
    }
    
    async fn add_role(&self, user_id: &str, role: &str) -> Result<User> {
        // Get the user
        let mut user = self.find_by_id(&user_id.to_string()).await?
            .ok_or_else(|| Error::not_found(format!("User with ID {} not found", user_id)))?;
        
        // Add the role if it doesn't already exist
        if !user.roles.contains(&role.to_string()) {
            user.roles.push(role.to_string());
            
            // Update the user
            self.update(&user).await
        } else {
            Ok(user)
        }
    }
    
    async fn remove_role(&self, user_id: &str, role: &str) -> Result<User> {
        // Get the user
        let mut user = self.find_by_id(&user_id.to_string()).await?
            .ok_or_else(|| Error::not_found(format!("User with ID {} not found", user_id)))?;
        
        // Remove the role
        user.roles.retain(|r| r != role);
        
        // Update the user
        self.update(&user).await
    }
    
    async fn update_last_seen(&self, user_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            "UPDATE users SET last_seen_at = ? WHERE id = ?",
            now,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to update last seen: {}", e)))?;
        
        Ok(())
    }
    
    async fn authenticate(&self, username_or_email: &str, password: &str) -> Result<Option<User>> {
        // Find user by username or email
        let user_row = sqlx::query!(
            r#"
            SELECT id, password_hash
            FROM users
            WHERE username = ? OR email = ?
            "#,
            username_or_email,
            username_or_email
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to authenticate user: {}", e)))?;
        
        // If user not found or password doesn't match, return None
        match user_row {
            Some(row) if self.verify_password(&row.password_hash, password)? => {
                // Password matches, get full user details
                self.find_by_id(&row.id.to_string()).await
            },
            _ => Ok(None),
        }
    }
    
    async fn change_password(&self, user_id: &str, current_password: &str, new_password: &str) -> Result<()> {
        // Find user
        let user_row = sqlx::query!(
            r#"
            SELECT password_hash
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to find user: {}", e)))?;
        
        // Verify current password
        match user_row {
            Some(row) if self.verify_password(&row.password_hash, current_password)? => {
                // Password matches, hash new password
                let new_password_hash = self.hash_password(new_password)?;
                
                // Update password
                sqlx::query!(
                    "UPDATE users SET password_hash = ? WHERE id = ?",
                    new_password_hash,
                    user_id
                )
                .execute(&self.pool)
                .await
                .map_err(|e| Error::database(format!("Failed to update password: {}", e)))?;
                
                Ok(())
            },
            Some(_) => Err(Error::authentication("Current password is incorrect")),
            None => Err(Error::not_found(format!("User with ID {} not found", user_id))),
        }
    }
    
    async fn get_preferences(&self, user_id: &str) -> Result<serde_json::Value> {
        // Check if user exists
        let user_exists = sqlx::query!("SELECT id FROM users WHERE id = ?", user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to check if user exists: {}", e)))?
            .is_some();
            
        if !user_exists {
            return Err(Error::not_found(format!("User with ID {} not found", user_id)));
        }
        
        // Get preferences
        let result = sqlx::query!(
            "SELECT preferences FROM user_preferences WHERE user_id = ?",
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::database(format!("Failed to get preferences: {}", e)))?;
        
        match result {
            Some(row) => {
                let preferences: serde_json::Value = serde_json::from_str(&row.preferences)
                    .map_err(|e| Error::parsing(format!("Failed to parse preferences: {}", e)))?;
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
                
                let now = Utc::now().to_rfc3339();
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
                .map_err(|e| Error::database(format!("Failed to create default preferences: {}", e)))?;
                
                Ok(default_prefs)
            }
        }
    }
    
    async fn update_preferences(&self, user_id: &str, preferences: serde_json::Value) -> Result<serde_json::Value> {
        // Check if user exists
        let user_exists = sqlx::query!("SELECT id FROM users WHERE id = ?", user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| Error::database(format!("Failed to check if user exists: {}", e)))?
            .is_some();
            
        if !user_exists {
            return Err(Error::not_found(format!("User with ID {} not found", user_id)));
        }
        
        let now = Utc::now().to_rfc3339();
        let prefs_str = preferences.to_string();
        
        // Update or insert preferences
        sqlx::query(
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
        .map_err(|e| Error::database(format!("Failed to update preferences: {}", e)))?;
        
        Ok(preferences)
    }
}
