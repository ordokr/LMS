use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::{Group, GroupJoinLevel, GroupMembership, GroupMembershipStatus};
use super::repository::Repository;
use super::group_repository::GroupRepository;

/// SQLite implementation of the group repository
pub struct SqliteGroupRepository {
    pool: Pool<Sqlite>,
}

impl SqliteGroupRepository {
    /// Create a new SQLite group repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a Group
    async fn row_to_group(&self, id: &str) -> Result<Option<Group>, Error> {
        let group_row = sqlx::query!(
            r#"
            SELECT 
                id, name, description, created_at, updated_at, context_id, context_type,
                group_category_id, join_level, max_membership, is_public, canvas_id,
                discourse_id, full_name, visibility_level, mentionable_level, messageable_level,
                automatic, sis_source_id, storage_quota, default_view, source_system, metadata
            FROM groups
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = group_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
            
            // Parse join level
            let join_level = GroupJoinLevel::from(row.join_level.as_str());
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Get memberships
            let memberships = self.get_memberships(&row.id).await?;
            
            // Create group
            let group = Group {
                id: row.id,
                name: row.name,
                description: row.description,
                created_at,
                updated_at,
                context_id: row.context_id,
                context_type: row.context_type,
                group_category_id: row.group_category_id,
                join_level,
                max_membership: row.max_membership,
                is_public: row.is_public != 0,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                full_name: row.full_name,
                visibility_level: row.visibility_level,
                mentionable_level: row.mentionable_level,
                messageable_level: row.messageable_level,
                automatic: row.automatic != 0,
                sis_source_id: row.sis_source_id,
                storage_quota: row.storage_quota,
                default_view: row.default_view,
                source_system: row.source_system,
                metadata,
                memberships: Some(memberships),
            };
            
            Ok(Some(group))
        } else {
            Ok(None)
        }
    }
    
    /// Helper method to convert a row to a GroupMembership
    async fn row_to_membership(&self, row: &sqlx::sqlite::SqliteRow) -> Result<GroupMembership, Error> {
        // Parse timestamps
        let created_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))
            .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
            .with_timezone(&Utc);
            
        let updated_at = chrono::DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))
            .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
            .with_timezone(&Utc);
        
        // Parse status
        let status = GroupMembershipStatus::from(row.get::<String, _>("status").as_str());
        
        // Create membership
        let membership = GroupMembership {
            id: row.get::<String, _>("id"),
            group_id: row.get::<String, _>("group_id"),
            user_id: row.get::<String, _>("user_id"),
            status,
            is_moderator: row.get::<i32, _>("is_moderator") != 0,
            created_at,
            updated_at,
        };
        
        Ok(membership)
    }
}

#[async_trait]
impl Repository<Group, String> for SqliteGroupRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Group>, Error> {
        self.row_to_group(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Group>, Error> {
        let group_ids = sqlx::query!(
            "SELECT id FROM groups"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut groups = Vec::new();
        for row in group_ids {
            if let Some(group) = self.row_to_group(&row.id).await? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
    
    async fn create(&self, group: &Group) -> Result<Group, Error> {
        // Serialize metadata
        let metadata_json = serde_json::to_string(&group.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Insert group
        sqlx::query!(
            r#"
            INSERT INTO groups (
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
                    INSERT INTO group_memberships (
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
        
        // Return the created group
        Ok(group.clone())
    }
    
    async fn update(&self, group: &Group) -> Result<Group, Error> {
        // Serialize metadata
        let metadata_json = serde_json::to_string(&group.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Update group
        sqlx::query!(
            r#"
            UPDATE groups SET
                name = ?, description = ?, updated_at = ?, context_id = ?, context_type = ?,
                group_category_id = ?, join_level = ?, max_membership = ?, is_public = ?, canvas_id = ?,
                discourse_id = ?, full_name = ?, visibility_level = ?, mentionable_level = ?, messageable_level = ?,
                automatic = ?, sis_source_id = ?, storage_quota = ?, default_view = ?, source_system = ?, metadata = ?
            WHERE id = ?
            "#,
            group.name,
            group.description,
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
            metadata_json,
            group.id
        )
        .execute(&mut tx)
        .await?;
        
        // Update memberships if any
        if let Some(memberships) = &group.memberships {
            // First, delete all existing memberships
            sqlx::query!(
                "DELETE FROM group_memberships WHERE group_id = ?",
                group.id
            )
            .execute(&mut tx)
            .await?;
            
            // Then, insert the updated memberships
            for membership in memberships {
                sqlx::query!(
                    r#"
                    INSERT INTO group_memberships (
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
        
        // Return the updated group
        Ok(group.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        // Start a transaction
        let mut tx = self.pool.begin().await?;
        
        // Delete memberships
        sqlx::query!(
            "DELETE FROM group_memberships WHERE group_id = ?",
            id
        )
        .execute(&mut tx)
        .await?;
        
        // Delete group
        sqlx::query!(
            "DELETE FROM groups WHERE id = ?",
            id
        )
        .execute(&mut tx)
        .await?;
        
        // Commit the transaction
        tx.commit().await?;
        
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM groups")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl GroupRepository for SqliteGroupRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<Group>, Error> {
        let group_row = sqlx::query!("SELECT id FROM groups WHERE name = ?", name)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = group_row {
            self.row_to_group(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Group>, Error> {
        let group_row = sqlx::query!("SELECT id FROM groups WHERE canvas_id = ?", canvas_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = group_row {
            self.row_to_group(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Group>, Error> {
        let group_row = sqlx::query!("SELECT id FROM groups WHERE discourse_id = ?", discourse_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = group_row {
            self.row_to_group(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_context(&self, context_id: &str, context_type: &str) -> Result<Vec<Group>, Error> {
        let group_rows = sqlx::query!(
            "SELECT id FROM groups WHERE context_id = ? AND context_type = ?",
            context_id,
            context_type
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut groups = Vec::new();
        for row in group_rows {
            if let Some(group) = self.row_to_group(&row.id).await? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
    
    async fn find_by_category(&self, category_id: &str) -> Result<Vec<Group>, Error> {
        let group_rows = sqlx::query!(
            "SELECT id FROM groups WHERE group_category_id = ?",
            category_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut groups = Vec::new();
        for row in group_rows {
            if let Some(group) = self.row_to_group(&row.id).await? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
    
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<Group>, Error> {
        let group_rows = sqlx::query!(
            r#"
            SELECT g.id
            FROM groups g
            JOIN group_memberships m ON g.id = m.group_id
            WHERE m.user_id = ? AND m.status = 'accepted'
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut groups = Vec::new();
        for row in group_rows {
            if let Some(group) = self.row_to_group(&row.id).await? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
    
    async fn find_by_user_and_context(&self, user_id: &str, context_id: &str, context_type: &str) -> Result<Vec<Group>, Error> {
        let group_rows = sqlx::query!(
            r#"
            SELECT g.id
            FROM groups g
            JOIN group_memberships m ON g.id = m.group_id
            WHERE m.user_id = ? AND m.status = 'accepted'
            AND g.context_id = ? AND g.context_type = ?
            "#,
            user_id,
            context_id,
            context_type
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut groups = Vec::new();
        for row in group_rows {
            if let Some(group) = self.row_to_group(&row.id).await? {
                groups.push(group);
            }
        }
        
        Ok(groups)
    }
    
    async fn add_user_to_group(&self, group_id: &str, user_id: &str, status: GroupMembershipStatus) -> Result<GroupMembership, Error> {
        // Check if the group exists
        let group = self.find_by_id(&group_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Group with ID {} not found", group_id)))?;
        
        // Check if the user is already a member
        let existing_membership = sqlx::query!(
            "SELECT id FROM group_memberships WHERE group_id = ? AND user_id = ?",
            group_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let now = Utc::now();
        let membership_id = if let Some(row) = existing_membership {
            // Update existing membership
            sqlx::query!(
                r#"
                UPDATE group_memberships
                SET status = ?, updated_at = ?
                WHERE id = ?
                "#,
                status.to_string(),
                now.to_rfc3339(),
                row.id
            )
            .execute(&self.pool)
            .await?;
            
            row.id
        } else {
            // Create new membership
            let membership_id = uuid::Uuid::new_v4().to_string();
            
            sqlx::query!(
                r#"
                INSERT INTO group_memberships (
                    id, group_id, user_id, status, is_moderator, created_at, updated_at
                ) VALUES (
                    ?, ?, ?, ?, 0, ?, ?
                )
                "#,
                membership_id,
                group_id,
                user_id,
                status.to_string(),
                now.to_rfc3339(),
                now.to_rfc3339()
            )
            .execute(&self.pool)
            .await?;
            
            membership_id
        };
        
        // Get the updated membership
        let membership_row = sqlx::query(
            r#"
            SELECT id, group_id, user_id, status, is_moderator, created_at, updated_at
            FROM group_memberships
            WHERE id = ?
            "#
        )
        .bind(membership_id)
        .fetch_one(&self.pool)
        .await?;
        
        self.row_to_membership(&membership_row).await
    }
    
    async fn remove_user_from_group(&self, group_id: &str, user_id: &str) -> Result<(), Error> {
        // Check if the group exists
        let _group = self.find_by_id(&group_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Group with ID {} not found", group_id)))?;
        
        // Delete the membership
        sqlx::query!(
            "DELETE FROM group_memberships WHERE group_id = ? AND user_id = ?",
            group_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn update_membership_status(&self, group_id: &str, user_id: &str, status: GroupMembershipStatus) -> Result<GroupMembership, Error> {
        // Check if the group exists
        let _group = self.find_by_id(&group_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Group with ID {} not found", group_id)))?;
        
        // Check if the membership exists
        let membership_row = sqlx::query!(
            "SELECT id FROM group_memberships WHERE group_id = ? AND user_id = ?",
            group_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Membership for user {} in group {} not found", user_id, group_id)))?;
        
        // Update the membership
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE group_memberships
            SET status = ?, updated_at = ?
            WHERE id = ?
            "#,
            status.to_string(),
            now.to_rfc3339(),
            membership_row.id
        )
        .execute(&self.pool)
        .await?;
        
        // Get the updated membership
        let membership_row = sqlx::query(
            r#"
            SELECT id, group_id, user_id, status, is_moderator, created_at, updated_at
            FROM group_memberships
            WHERE id = ?
            "#
        )
        .bind(membership_row.id)
        .fetch_one(&self.pool)
        .await?;
        
        self.row_to_membership(&membership_row).await
    }
    
    async fn set_moderator(&self, group_id: &str, user_id: &str, is_moderator: bool) -> Result<GroupMembership, Error> {
        // Check if the group exists
        let _group = self.find_by_id(&group_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Group with ID {} not found", group_id)))?;
        
        // Check if the membership exists
        let membership_row = sqlx::query!(
            "SELECT id FROM group_memberships WHERE group_id = ? AND user_id = ?",
            group_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Membership for user {} in group {} not found", user_id, group_id)))?;
        
        // Update the membership
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE group_memberships
            SET is_moderator = ?, updated_at = ?
            WHERE id = ?
            "#,
            if is_moderator { 1 } else { 0 },
            now.to_rfc3339(),
            membership_row.id
        )
        .execute(&self.pool)
        .await?;
        
        // Get the updated membership
        let membership_row = sqlx::query(
            r#"
            SELECT id, group_id, user_id, status, is_moderator, created_at, updated_at
            FROM group_memberships
            WHERE id = ?
            "#
        )
        .bind(membership_row.id)
        .fetch_one(&self.pool)
        .await?;
        
        self.row_to_membership(&membership_row).await
    }
    
    async fn get_memberships(&self, group_id: &str) -> Result<Vec<GroupMembership>, Error> {
        let membership_rows = sqlx::query(
            r#"
            SELECT id, group_id, user_id, status, is_moderator, created_at, updated_at
            FROM group_memberships
            WHERE group_id = ?
            "#
        )
        .bind(group_id)
        .fetch_all(&self.pool)
        .await?;
        
        let mut memberships = Vec::new();
        for row in membership_rows {
            let membership = self.row_to_membership(&row).await?;
            memberships.push(membership);
        }
        
        Ok(memberships)
    }
    
    async fn get_membership(&self, group_id: &str, user_id: &str) -> Result<Option<GroupMembership>, Error> {
        let membership_row = sqlx::query(
            r#"
            SELECT id, group_id, user_id, status, is_moderator, created_at, updated_at
            FROM group_memberships
            WHERE group_id = ? AND user_id = ?
            "#
        )
        .bind(group_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = membership_row {
            let membership = self.row_to_membership(&row).await?;
            Ok(Some(membership))
        } else {
            Ok(None)
        }
    }
}
