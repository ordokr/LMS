use sqlx::{Pool, Sqlite};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use log::{info, warn, error};
use crate::models::unified_models::User as UnifiedUser;
use crate::error::Error;

/// Utility for migrating user data from old tables to the new unified table
pub struct UserMigration {
    pool: Pool<Sqlite>,
}

impl UserMigration {
    /// Create a new user migration utility
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Migrate all users from old tables to the new unified table
    pub async fn migrate_all_users(&self) -> Result<MigrationStats, Error> {
        info!("Starting user migration...");
        
        let mut stats = MigrationStats::default();
        
        // Migrate from src-tauri/src/models/user/user.rs
        stats += self.migrate_from_user_model().await?;
        
        // Migrate from src-tauri/src/models/user.rs
        stats += self.migrate_from_simple_user_model().await?;
        
        // Migrate from src-tauri/src/models/unified/user.rs
        stats += self.migrate_from_old_unified_user_model().await?;
        
        info!("User migration completed: {}", stats);
        
        Ok(stats)
    }
    
    /// Migrate users from the model in src-tauri/src/models/user/user.rs
    async fn migrate_from_user_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating users from user model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='users_old'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old users table (users_old) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all users from the old table
        let old_users = sqlx::query!(
            r#"
            SELECT 
                id, name, email, username, avatar_url, created_at, updated_at, last_seen_at,
                canvas_user_id, sis_user_id, lti_user_id, sortable_name, short_name,
                discourse_user_id, trust_level, post_count
            FROM users_old
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_user in old_users {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_user.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for user {}: {}", old_user.id, e);
                    stats.errors += 1;
                    continue;
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_user.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for user {}: {}", old_user.id, e);
                    stats.errors += 1;
                    continue;
                }
            };
            
            let last_seen_at = if let Some(ts) = old_user.last_seen_at {
                match chrono::DateTime::parse_from_rfc3339(&ts) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse last_seen_at for user {}: {}", old_user.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            // Create unified user
            let unified_user = UnifiedUser {
                id: old_user.id.to_string(),
                name: old_user.name,
                email: old_user.email,
                username: old_user.username,
                avatar_url: old_user.avatar_url,
                created_at,
                updated_at,
                last_seen_at,
                roles: vec!["student".to_string()], // Default role
                trust_level: old_user.trust_level,
                is_admin: false,
                is_moderator: false,
                canvas_id: old_user.canvas_user_id,
                discourse_id: old_user.discourse_user_id.map(|id| id.to_string()),
                sis_id: old_user.sis_user_id,
                lti_id: old_user.lti_user_id,
                bio: None,
                location: None,
                website: None,
                timezone: None,
                sortable_name: old_user.sortable_name,
                short_name: old_user.short_name,
                post_count: old_user.post_count,
                source_system: None,
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_user(&unified_user).await {
                error!("Failed to insert unified user {}: {}", unified_user.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} users from user model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate users from the model in src-tauri/src/models/user.rs
    async fn migrate_from_simple_user_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating users from simple user model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='users_simple'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old users table (users_simple) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all users from the old table
        let old_users = sqlx::query!(
            r#"
            SELECT 
                id, email, first_name, last_name, role, created_at, updated_at
            FROM users_simple
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_user in old_users {
            // Create unified user
            let unified_user = UnifiedUser {
                id: old_user.id,
                name: format!("{} {}", old_user.first_name, old_user.last_name),
                email: old_user.email,
                username: old_user.email.split('@').next().unwrap_or("user").to_string(),
                avatar_url: None,
                created_at: Utc::now(), // Default to now if parsing fails
                updated_at: Utc::now(),
                last_seen_at: None,
                roles: vec![old_user.role.to_lowercase()],
                trust_level: None,
                is_admin: old_user.role.to_lowercase() == "admin",
                is_moderator: false,
                canvas_id: None,
                discourse_id: None,
                sis_id: None,
                lti_id: None,
                bio: None,
                location: None,
                website: None,
                timezone: None,
                sortable_name: None,
                short_name: Some(old_user.first_name),
                post_count: None,
                source_system: None,
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_user(&unified_user).await {
                error!("Failed to insert unified user {}: {}", unified_user.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} users from simple user model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate users from the model in src-tauri/src/models/unified/user.rs
    async fn migrate_from_old_unified_user_model(&self) -> Result<MigrationStats, Error> {
        info!("Migrating users from old unified user model...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='unified_users'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old users table (unified_users) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all users from the old table
        let old_users = sqlx::query!(
            r#"
            SELECT 
                id, name, email, username, avatar, canvas_id, discourse_id, 
                last_login, source_system, roles, metadata
            FROM unified_users
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_user in old_users {
            // Parse roles
            let roles: Vec<String> = match serde_json::from_str(&old_user.roles) {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to parse roles for user {}: {}", old_user.id, e);
                    vec!["student".to_string()] // Default role
                }
            };
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = old_user.metadata {
                match serde_json::from_str(&meta_str) {
                    Ok(m) => m,
                    Err(e) => {
                        error!("Failed to parse metadata for user {}: {}", old_user.id, e);
                        HashMap::new()
                    }
                }
            } else {
                HashMap::new()
            };
            
            // Parse last login
            let last_seen_at = if let Some(ts) = old_user.last_login {
                match chrono::DateTime::parse_from_rfc3339(&ts) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse last_login for user {}: {}", old_user.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            // Create unified user
            let unified_user = UnifiedUser {
                id: old_user.id,
                name: old_user.name,
                email: old_user.email,
                username: old_user.username,
                avatar_url: Some(old_user.avatar),
                created_at: Utc::now(), // Default to now
                updated_at: Utc::now(),
                last_seen_at,
                roles,
                trust_level: None,
                is_admin: roles.contains(&"admin".to_string()),
                is_moderator: roles.contains(&"moderator".to_string()),
                canvas_id: old_user.canvas_id,
                discourse_id: old_user.discourse_id,
                sis_id: None,
                lti_id: None,
                bio: None,
                location: None,
                website: None,
                timezone: None,
                sortable_name: None,
                short_name: None,
                post_count: None,
                source_system: old_user.source_system,
                metadata,
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_user(&unified_user).await {
                error!("Failed to insert unified user {}: {}", unified_user.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} users from old unified user model", stats.migrated);
        
        Ok(stats)
    }
    
    /// Insert a unified user into the new table
    async fn insert_unified_user(&self, user: &UnifiedUser) -> Result<(), Error> {
        // Serialize roles and metadata
        let roles_json = serde_json::to_string(&user.roles)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize roles: {}", e)))?;
            
        let metadata_json = serde_json::to_string(&user.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert user
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO users (
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
        
        Ok(())
    }
}

/// Statistics for the migration process
#[derive(Debug, Default, Clone, Copy)]
pub struct MigrationStats {
    pub migrated: usize,
    pub errors: usize,
}

impl std::ops::AddAssign for MigrationStats {
    fn add_assign(&mut self, other: Self) {
        self.migrated += other.migrated;
        self.errors += other.errors;
    }
}

impl std::fmt::Display for MigrationStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Migrated: {}, Errors: {}", self.migrated, self.errors)
    }
}
