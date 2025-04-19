use sqlx::{Pool, Sqlite};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use log::{info, warn, error};
use crate::models::unified_models::{Group as UnifiedGroup, GroupJoinLevel, GroupMembership, GroupMembershipStatus};
use crate::error::Error;

/// Utility for migrating group data from old tables to the new unified table
pub struct GroupMigration {
    pool: Pool<Sqlite>,
}

impl GroupMigration {
    /// Create a new group migration utility
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Migrate all groups from old tables to the new unified table
    pub async fn migrate_all_groups(&self) -> Result<MigrationStats, Error> {
        info!("Starting group migration...");
        
        let mut stats = MigrationStats::default();
        
        // Migrate from Canvas-style groups
        stats += self.migrate_from_canvas_groups().await?;
        
        // Migrate from Discourse-style groups
        stats += self.migrate_from_discourse_groups().await?;
        
        info!("Group migration completed: {}", stats);
        
        Ok(stats)
    }
    
    /// Migrate groups from Canvas-style tables
    async fn migrate_from_canvas_groups(&self) -> Result<MigrationStats, Error> {
        info!("Migrating groups from Canvas-style tables...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='canvas_groups'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old groups table (canvas_groups) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all groups from the old table
        let old_groups = sqlx::query!(
            r#"
            SELECT 
                id, name, description, context_id, context_type, group_category_id,
                join_level, max_membership, is_public, created_at, updated_at,
                sis_source_id, storage_quota, default_view
            FROM canvas_groups
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_group in old_groups {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_group.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for group {}: {}", old_group.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_group.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for group {}: {}", old_group.id, e);
                    Utc::now()
                }
            };
            
            // Parse join level
            let join_level = match old_group.join_level.as_deref() {
                Some("invitation_only") => GroupJoinLevel::InvitationOnly,
                Some("request") => GroupJoinLevel::RequestToJoin,
                Some("free") => GroupJoinLevel::Free,
                _ => GroupJoinLevel::InvitationOnly,
            };
            
            // Create unified group
            let unified_group = UnifiedGroup {
                id: Uuid::new_v4().to_string(),
                name: old_group.name,
                description: old_group.description,
                created_at,
                updated_at,
                context_id: old_group.context_id,
                context_type: old_group.context_type,
                group_category_id: old_group.group_category_id,
                join_level,
                max_membership: old_group.max_membership,
                is_public: old_group.is_public.unwrap_or(0) != 0,
                canvas_id: Some(old_group.id.to_string()),
                discourse_id: None,
                full_name: None,
                visibility_level: None,
                mentionable_level: None,
                messageable_level: None,
                automatic: false,
                sis_source_id: old_group.sis_source_id,
                storage_quota: old_group.storage_quota,
                default_view: old_group.default_view,
                source_system: Some("canvas".to_string()),
                metadata: HashMap::new(),
                memberships: Some(Vec::new()),
            };
            
            // Fetch memberships for this group
            let old_memberships = sqlx::query!(
                r#"
                SELECT 
                    id, user_id, workflow_state, moderator, created_at, updated_at
                FROM canvas_group_memberships
                WHERE group_id = ?
                "#,
                old_group.id
            )
            .fetch_all(&self.pool)
            .await?;
            
            let mut memberships = Vec::new();
            
            for old_membership in old_memberships {
                // Parse timestamps
                let membership_created_at = match chrono::DateTime::parse_from_rfc3339(&old_membership.created_at) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        error!("Failed to parse created_at for membership {}: {}", old_membership.id, e);
                        Utc::now()
                    }
                };
                
                let membership_updated_at = match chrono::DateTime::parse_from_rfc3339(&old_membership.updated_at) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        error!("Failed to parse updated_at for membership {}: {}", old_membership.id, e);
                        Utc::now()
                    }
                };
                
                // Parse status
                let status = match old_membership.workflow_state.as_deref() {
                    Some("accepted") => GroupMembershipStatus::Accepted,
                    Some("invited") => GroupMembershipStatus::Invited,
                    Some("requested") => GroupMembershipStatus::Requested,
                    Some("rejected") => GroupMembershipStatus::Rejected,
                    Some("deleted") => GroupMembershipStatus::Deleted,
                    _ => GroupMembershipStatus::Accepted,
                };
                
                // Create membership
                let membership = GroupMembership {
                    id: Uuid::new_v4().to_string(),
                    group_id: unified_group.id.clone(),
                    user_id: old_membership.user_id.unwrap_or_default(),
                    status,
                    is_moderator: old_membership.moderator.unwrap_or(0) != 0,
                    created_at: membership_created_at,
                    updated_at: membership_updated_at,
                };
                
                memberships.push(membership);
            }
            
            // Add memberships to the group
            let mut group_with_memberships = unified_group;
            group_with_memberships.memberships = Some(memberships);
            
            // Insert into new table
            if let Err(e) = self.insert_unified_group(&group_with_memberships).await {
                error!("Failed to insert unified group {}: {}", group_with_memberships.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} groups from Canvas-style tables", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate groups from Discourse-style tables
    async fn migrate_from_discourse_groups(&self) -> Result<MigrationStats, Error> {
        info!("Migrating groups from Discourse-style tables...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='discourse_groups'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old groups table (discourse_groups) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all groups from the old table
        let old_groups = sqlx::query!(
            r#"
            SELECT 
                id, name, description, full_name, visibility_level, mentionable_level,
                messageable_level, automatic, created_at, updated_at
            FROM discourse_groups
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_group in old_groups {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_group.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for group {}: {}", old_group.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_group.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for group {}: {}", old_group.id, e);
                    Utc::now()
                }
            };
            
            // Create unified group
            let unified_group = UnifiedGroup {
                id: Uuid::new_v4().to_string(),
                name: old_group.name,
                description: old_group.description,
                created_at,
                updated_at,
                context_id: None,
                context_type: None,
                group_category_id: None,
                join_level: GroupJoinLevel::InvitationOnly,
                max_membership: None,
                is_public: true,
                canvas_id: None,
                discourse_id: Some(old_group.id.to_string()),
                full_name: old_group.full_name,
                visibility_level: old_group.visibility_level,
                mentionable_level: old_group.mentionable_level,
                messageable_level: old_group.messageable_level,
                automatic: old_group.automatic.unwrap_or(0) != 0,
                sis_source_id: None,
                storage_quota: None,
                default_view: Some("feed".to_string()),
                source_system: Some("discourse".to_string()),
                metadata: HashMap::new(),
                memberships: Some(Vec::new()),
            };
            
            // Fetch memberships for this group
            let old_memberships = sqlx::query!(
                r#"
                SELECT 
                    id, user_id, created_at, updated_at
                FROM discourse_group_memberships
                WHERE group_id = ?
                "#,
                old_group.id
            )
            .fetch_all(&self.pool)
            .await?;
            
            let mut memberships = Vec::new();
            
            for old_membership in old_memberships {
                // Parse timestamps
                let membership_created_at = match chrono::DateTime::parse_from_rfc3339(&old_membership.created_at) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        error!("Failed to parse created_at for membership {}: {}", old_membership.id, e);
                        Utc::now()
                    }
                };
                
                let membership_updated_at = match chrono::DateTime::parse_from_rfc3339(&old_membership.updated_at) {
                    Ok(dt) => dt.with_timezone(&Utc),
                    Err(e) => {
                        error!("Failed to parse updated_at for membership {}: {}", old_membership.id, e);
                        Utc::now()
                    }
                };
                
                // Create membership
                let membership = GroupMembership {
                    id: Uuid::new_v4().to_string(),
                    group_id: unified_group.id.clone(),
                    user_id: old_membership.user_id.unwrap_or_default(),
                    status: GroupMembershipStatus::Accepted,
                    is_moderator: false,
                    created_at: membership_created_at,
                    updated_at: membership_updated_at,
                };
                
                memberships.push(membership);
            }
            
            // Add memberships to the group
            let mut group_with_memberships = unified_group;
            group_with_memberships.memberships = Some(memberships);
            
            // Insert into new table
            if let Err(e) = self.insert_unified_group(&group_with_memberships).await {
                error!("Failed to insert unified group {}: {}", group_with_memberships.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} groups from Discourse-style tables", stats.migrated);
        
        Ok(stats)
    }
    
    /// Insert a unified group into the new table
    async fn insert_unified_group(&self, group: &UnifiedGroup) -> Result<(), Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&group.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert group
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO groups (
                id, name, description, created_at, updated_at, context_id, context_type,
                group_category_id, join_level, max_membership, is_public, canvas_id,
                discourse_id, full_name, visibility_level, mentionable_level, messageable_level,
                automatic, sis_source_id, storage_quota, default_view, source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            group.id,
            group.name,
            group.description,
            group.created_at.to_rfc3339(),
            group.updated_at.to_rfc3339(),
            group.context_id,
            group.context_type,
            group.group_category_id,
            group.join_level.to_string(),
            group.max_membership,
            if group.is_public { 1 } else { 0 },
            group.canvas_id,
            group.discourse_id,
            group.full_name,
            group.visibility_level,
            group.mentionable_level,
            group.messageable_level,
            if group.automatic { 1 } else { 0 },
            group.sis_source_id,
            group.storage_quota,
            group.default_view,
            group.source_system,
            metadata_json
        )
        .execute(&mut tx)
        .await?;
        
        // Insert memberships if any
        if let Some(memberships) = &group.memberships {
            for membership in memberships {
                sqlx::query!(
                    r#"
                    INSERT OR REPLACE INTO group_memberships (
                        id, group_id, user_id, status, is_moderator, created_at, updated_at
                    ) VALUES (
                        ?, ?, ?, ?, ?, ?, ?
                    )
                    "#,
                    membership.id,
                    membership.group_id,
                    membership.user_id,
                    membership.status.to_string(),
                    if membership.is_moderator { 1 } else { 0 },
                    membership.created_at.to_rfc3339(),
                    membership.updated_at.to_rfc3339()
                )
                .execute(&mut tx)
                .await?;
            }
        }
        
        // Commit the transaction
        tx.commit().await?;
        
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
