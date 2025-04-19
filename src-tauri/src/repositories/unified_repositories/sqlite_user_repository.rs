use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::User;
use super::repository::Repository;
use super::user_repository::UserRepository;

/// SQLite implementation of the user repository
pub struct SqliteUserRepository {
    pool: Pool<Sqlite>,
}

impl SqliteUserRepository {
    /// Create a new SQLite user repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a User
    async fn row_to_user(&self, id: &str) -> Result<Option<User>, Error> {
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
        .await?;
        
        if let Some(row) = user_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
                
            let last_seen_at = if let Some(ts) = row.last_seen_at {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .map_err(|e| Error::ParseError(format!("Failed to parse last_seen_at: {}", e)))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            // Parse roles
            let roles: Vec<String> = serde_json::from_str(&row.roles)
                .map_err(|e| Error::ParseError(format!("Failed to parse roles: {}", e)))?;
                
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
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
}

#[async_trait]
impl Repository<User, String> for SqliteUserRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<User>, Error> {
        self.row_to_user(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let user_ids = sqlx::query!(
            "SELECT id FROM users"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut users = Vec::new();
        for row in user_ids {
            if let Some(user) = self.row_to_user(&row.id).await? {
                users.push(user);
            }
        }
        
        Ok(users)
    }
    
    async fn create(&self, user: &User) -> Result<User, Error> {
        // Serialize roles and metadata
        let roles_json = serde_json::to_string(&user.roles)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize roles: {}", e)))?;
            
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
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
        .await?;
        
        // Return the created user
        Ok(user.clone())
    }
    
    async fn update(&self, user: &User) -> Result<User, Error> {
        // Serialize roles and metadata
        let roles_json = serde_json::to_string(&user.roles)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize roles: {}", e)))?;
            
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
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
        .await?;
        
        // Return the updated user
        Ok(user.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE email = ?", email)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE username = ?", username)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<User>, Error> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE canvas_id = ?", canvas_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<User>, Error> {
        let user_row = sqlx::query!("SELECT id FROM users WHERE discourse_id = ?", discourse_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = user_row {
            self.row_to_user(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_role(&self, role: &str) -> Result<Vec<User>, Error> {
        // This is a bit tricky since roles are stored as JSON
        // We'll need to fetch all users and filter in memory
        let users = self.find_all().await?;
        
        let filtered_users = users.into_iter()
            .filter(|user| user.has_role(role))
            .collect();
            
        Ok(filtered_users)
    }
    
    async fn add_role(&self, user_id: &str, role: &str) -> Result<User, Error> {
        // Get the user
        let mut user = self.find_by_id(&user_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("User with ID {} not found", user_id)))?;
        
        // Add the role
        user.add_role(role);
        
        // Update the user
        self.update(&user).await
    }
    
    async fn remove_role(&self, user_id: &str, role: &str) -> Result<User, Error> {
        // Get the user
        let mut user = self.find_by_id(&user_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("User with ID {} not found", user_id)))?;
        
        // Remove the role
        user.remove_role(role);
        
        // Update the user
        self.update(&user).await
    }
    
    async fn update_last_seen(&self, user_id: &str) -> Result<(), Error> {
        let now = Utc::now().to_rfc3339();
        
        sqlx::query!(
            "UPDATE users SET last_seen_at = ? WHERE id = ?",
            now,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
