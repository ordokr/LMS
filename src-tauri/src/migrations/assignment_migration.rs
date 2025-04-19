use sqlx::{Pool, Sqlite};
use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use log::{info, warn, error};
use crate::models::unified_models::{Assignment, AssignmentStatus, GradingType, SubmissionType};
use crate::error::Error;

/// Utility for migrating assignment data from old tables to the new unified table
pub struct AssignmentMigration {
    pool: Pool<Sqlite>,
}

impl AssignmentMigration {
    /// Create a new assignment migration utility
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Migrate all assignments from old tables to the new unified table
    pub async fn migrate_all_assignments(&self) -> Result<MigrationStats, Error> {
        info!("Starting assignment migration...");
        
        let mut stats = MigrationStats::default();
        
        // Migrate from Canvas-style assignments
        stats += self.migrate_from_canvas_assignments().await?;
        
        // Migrate from Discourse-style topics
        stats += self.migrate_from_discourse_topics().await?;
        
        info!("Assignment migration completed: {}", stats);
        
        Ok(stats)
    }
    
    /// Migrate assignments from Canvas-style tables
    async fn migrate_from_canvas_assignments(&self) -> Result<MigrationStats, Error> {
        info!("Migrating assignments from Canvas-style tables...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='canvas_assignments'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old assignments table (canvas_assignments) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all assignments from the old table
        let old_assignments = sqlx::query!(
            r#"
            SELECT 
                id, course_id, name, description, due_at, unlock_at, lock_at,
                points_possible, grading_type, submission_types, position,
                published, group_category_id, peer_reviews, automatic_peer_reviews,
                peer_review_count, created_at, updated_at
            FROM canvas_assignments
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_assignment in old_assignments {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_assignment.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for assignment {}: {}", old_assignment.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_assignment.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for assignment {}: {}", old_assignment.id, e);
                    Utc::now()
                }
            };
            
            // Parse optional dates
            let due_date = if let Some(date_str) = old_assignment.due_at {
                match chrono::DateTime::parse_from_rfc3339(&date_str) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse due_at for assignment {}: {}", old_assignment.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            let unlock_date = if let Some(date_str) = old_assignment.unlock_at {
                match chrono::DateTime::parse_from_rfc3339(&date_str) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse unlock_at for assignment {}: {}", old_assignment.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            let lock_date = if let Some(date_str) = old_assignment.lock_at {
                match chrono::DateTime::parse_from_rfc3339(&date_str) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(e) => {
                        error!("Failed to parse lock_at for assignment {}: {}", old_assignment.id, e);
                        None
                    }
                }
            } else {
                None
            };
            
            // Parse grading type
            let grading_type = match old_assignment.grading_type.as_deref() {
                Some("points") => GradingType::Points,
                Some("percent") => GradingType::Percentage,
                Some("letter_grade") => GradingType::LetterGrade,
                Some("gpa_scale") => GradingType::GpaScale,
                Some("pass_fail") => GradingType::PassFail,
                Some("not_graded") => GradingType::NotGraded,
                _ => GradingType::Points,
            };
            
            // Parse submission types
            let submission_types = if let Some(types_str) = old_assignment.submission_types {
                types_str.split(',')
                    .map(str::trim)
                    .map(SubmissionType::from)
                    .collect()
            } else {
                vec![SubmissionType::None]
            };
            
            // Parse status
            let is_published = old_assignment.published.unwrap_or(0) != 0;
            let status = if is_published {
                AssignmentStatus::Published
            } else {
                AssignmentStatus::Unpublished
            };
            
            // Create unified assignment
            let unified_assignment = Assignment {
                id: Uuid::new_v4().to_string(),
                title: old_assignment.name,
                description: old_assignment.description,
                created_at,
                updated_at,
                course_id: old_assignment.course_id.map(|id| id.to_string()),
                due_date,
                unlock_date,
                lock_date,
                points_possible: old_assignment.points_possible,
                grading_type,
                submission_types,
                status,
                is_published,
                group_category_id: old_assignment.group_category_id.map(|id| id.to_string()),
                assignment_group_id: None,
                peer_reviews: old_assignment.peer_reviews.unwrap_or(0) != 0,
                automatic_peer_reviews: old_assignment.automatic_peer_reviews.unwrap_or(0) != 0,
                peer_review_count: old_assignment.peer_review_count,
                canvas_id: Some(old_assignment.id.to_string()),
                discourse_id: None,
                quiz_id: None,
                discussion_topic_id: None,
                position: old_assignment.position,
                source_system: Some("canvas".to_string()),
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_assignment(&unified_assignment).await {
                error!("Failed to insert unified assignment {}: {}", unified_assignment.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} assignments from Canvas-style tables", stats.migrated);
        
        Ok(stats)
    }
    
    /// Migrate assignments from Discourse-style topics
    async fn migrate_from_discourse_topics(&self) -> Result<MigrationStats, Error> {
        info!("Migrating assignments from Discourse-style topics...");
        
        let mut stats = MigrationStats::default();
        
        // Check if the old table exists
        let table_exists = sqlx::query!(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='discourse_topics'"
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        
        if !table_exists {
            warn!("Old topics table (discourse_topics) not found, skipping migration");
            return Ok(stats);
        }
        
        // Fetch all topics from the old table
        let old_topics = sqlx::query!(
            r#"
            SELECT 
                id, category_id, title, raw, created_at, updated_at
            FROM discourse_topics
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        for old_topic in old_topics {
            // Parse timestamps
            let created_at = match chrono::DateTime::parse_from_rfc3339(&old_topic.created_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse created_at for topic {}: {}", old_topic.id, e);
                    Utc::now()
                }
            };
            
            let updated_at = match chrono::DateTime::parse_from_rfc3339(&old_topic.updated_at) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(e) => {
                    error!("Failed to parse updated_at for topic {}: {}", old_topic.id, e);
                    Utc::now()
                }
            };
            
            // Create unified assignment
            let unified_assignment = Assignment {
                id: Uuid::new_v4().to_string(),
                title: old_topic.title,
                description: old_topic.raw,
                created_at,
                updated_at,
                course_id: old_topic.category_id.map(|id| id.to_string()),
                due_date: None,
                unlock_date: None,
                lock_date: None,
                points_possible: None,
                grading_type: GradingType::NotGraded,
                submission_types: vec![SubmissionType::DiscussionTopic],
                status: AssignmentStatus::Published,
                is_published: true,
                group_category_id: None,
                assignment_group_id: None,
                peer_reviews: false,
                automatic_peer_reviews: false,
                peer_review_count: None,
                canvas_id: None,
                discourse_id: Some(old_topic.id.to_string()),
                quiz_id: None,
                discussion_topic_id: Some(old_topic.id.to_string()),
                position: None,
                source_system: Some("discourse".to_string()),
                metadata: HashMap::new(),
            };
            
            // Insert into new table
            if let Err(e) = self.insert_unified_assignment(&unified_assignment).await {
                error!("Failed to insert unified assignment {}: {}", unified_assignment.id, e);
                stats.errors += 1;
            } else {
                stats.migrated += 1;
            }
        }
        
        info!("Migrated {} assignments from Discourse-style topics", stats.migrated);
        
        Ok(stats)
    }
    
    /// Insert a unified assignment into the new table
    async fn insert_unified_assignment(&self, assignment: &Assignment) -> Result<(), Error> {
        // Serialize submission types
        let submission_types_json = serde_json::to_string(&assignment.submission_types.iter().map(|st| st.to_string()).collect::<Vec<String>>())
            .map_err(|e| Error::SerializationError(format!("Failed to serialize submission_types: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&assignment.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert assignment
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO assignments (
                id, title, description, created_at, updated_at, course_id,
                due_date, unlock_date, lock_date, points_possible, grading_type,
                submission_types, status, is_published, group_category_id,
                assignment_group_id, peer_reviews, automatic_peer_reviews,
                peer_review_count, canvas_id, discourse_id, quiz_id,
                discussion_topic_id, position, source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            assignment.id,
            assignment.title,
            assignment.description,
            assignment.created_at.to_rfc3339(),
            assignment.updated_at.to_rfc3339(),
            assignment.course_id,
            assignment.due_date.map(|dt| dt.to_rfc3339()),
            assignment.unlock_date.map(|dt| dt.to_rfc3339()),
            assignment.lock_date.map(|dt| dt.to_rfc3339()),
            assignment.points_possible,
            assignment.grading_type.to_string(),
            submission_types_json,
            assignment.status.to_string(),
            if assignment.is_published { 1 } else { 0 },
            assignment.group_category_id,
            assignment.assignment_group_id,
            if assignment.peer_reviews { 1 } else { 0 },
            if assignment.automatic_peer_reviews { 1 } else { 0 },
            assignment.peer_review_count,
            assignment.canvas_id,
            assignment.discourse_id,
            assignment.quiz_id,
            assignment.discussion_topic_id,
            assignment.position,
            assignment.source_system,
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
