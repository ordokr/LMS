use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::{Assignment, AssignmentStatus, GradingType, SubmissionType};
use super::repository::Repository;
use super::assignment_repository::AssignmentRepository;

/// SQLite implementation of the assignment repository
pub struct SqliteAssignmentRepository {
    pool: Pool<Sqlite>,
}

impl SqliteAssignmentRepository {
    /// Create a new SQLite assignment repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to an Assignment
    async fn row_to_assignment(&self, id: &str) -> Result<Option<Assignment>, Error> {
        let assignment_row = sqlx::query!(
            r#"
            SELECT 
                id, title, description, created_at, updated_at, course_id,
                due_date, unlock_date, lock_date, points_possible, grading_type,
                submission_types, status, is_published, group_category_id,
                assignment_group_id, peer_reviews, automatic_peer_reviews,
                peer_review_count, canvas_id, discourse_id, quiz_id,
                discussion_topic_id, position, source_system, metadata
            FROM assignments
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = assignment_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
            
            // Parse optional dates
            let due_date = if let Some(date_str) = row.due_date {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse due_date: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let unlock_date = if let Some(date_str) = row.unlock_date {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse unlock_date: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            let lock_date = if let Some(date_str) = row.lock_date {
                Some(chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse lock_date: {}", e)))?
                    .with_timezone(&Utc))
            } else {
                None
            };
            
            // Parse grading type
            let grading_type = GradingType::from(row.grading_type.as_str());
            
            // Parse submission types
            let submission_types: Vec<SubmissionType> = if let Some(types_str) = row.submission_types {
                serde_json::from_str::<Vec<String>>(&types_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse submission_types: {}", e)))?
                    .iter()
                    .map(|s| SubmissionType::from(s.as_str()))
                    .collect()
            } else {
                vec![SubmissionType::None]
            };
            
            // Parse status
            let status = AssignmentStatus::from(row.status.as_str());
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Create assignment
            let assignment = Assignment {
                id: row.id,
                title: row.title,
                description: row.description,
                created_at,
                updated_at,
                course_id: row.course_id,
                due_date,
                unlock_date,
                lock_date,
                points_possible: row.points_possible,
                grading_type,
                submission_types,
                status,
                is_published: row.is_published != 0,
                group_category_id: row.group_category_id,
                assignment_group_id: row.assignment_group_id,
                peer_reviews: row.peer_reviews != 0,
                automatic_peer_reviews: row.automatic_peer_reviews != 0,
                peer_review_count: row.peer_review_count,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                quiz_id: row.quiz_id,
                discussion_topic_id: row.discussion_topic_id,
                position: row.position,
                source_system: row.source_system,
                metadata,
            };
            
            Ok(Some(assignment))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl Repository<Assignment, String> for SqliteAssignmentRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Assignment>, Error> {
        self.row_to_assignment(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Assignment>, Error> {
        let assignment_ids = sqlx::query!(
            "SELECT id FROM assignments"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_ids {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn create(&self, assignment: &Assignment) -> Result<Assignment, Error> {
        // Serialize submission types
        let submission_types_json = serde_json::to_string(&assignment.submission_types.iter().map(|st| st.to_string()).collect::<Vec<String>>())
            .map_err(|e| Error::SerializationError(format!("Failed to serialize submission_types: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&assignment.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert assignment
        sqlx::query!(
            r#"
            INSERT INTO assignments (
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
        
        // Return the created assignment
        Ok(assignment.clone())
    }
    
    async fn update(&self, assignment: &Assignment) -> Result<Assignment, Error> {
        // Serialize submission types
        let submission_types_json = serde_json::to_string(&assignment.submission_types.iter().map(|st| st.to_string()).collect::<Vec<String>>())
            .map_err(|e| Error::SerializationError(format!("Failed to serialize submission_types: {}", e)))?;
        
        // Serialize metadata
        let metadata_json = serde_json::to_string(&assignment.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Update assignment
        sqlx::query!(
            r#"
            UPDATE assignments SET
                title = ?, description = ?, updated_at = ?, course_id = ?,
                due_date = ?, unlock_date = ?, lock_date = ?, points_possible = ?,
                grading_type = ?, submission_types = ?, status = ?, is_published = ?,
                group_category_id = ?, assignment_group_id = ?, peer_reviews = ?,
                automatic_peer_reviews = ?, peer_review_count = ?, canvas_id = ?,
                discourse_id = ?, quiz_id = ?, discussion_topic_id = ?, position = ?,
                source_system = ?, metadata = ?
            WHERE id = ?
            "#,
            assignment.title,
            assignment.description,
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
            metadata_json,
            assignment.id
        )
        .execute(&self.pool)
        .await?;
        
        // Return the updated assignment
        Ok(assignment.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM assignments WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM assignments")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl AssignmentRepository for SqliteAssignmentRepository {
    async fn find_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error> {
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE course_id = ? ORDER BY due_date ASC",
            course_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_by_course_id_and_status(&self, course_id: &str, status: AssignmentStatus) -> Result<Vec<Assignment>, Error> {
        let status_str = status.to_string();
        
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE course_id = ? AND status = ? ORDER BY due_date ASC",
            course_id,
            status_str
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Assignment>, Error> {
        let assignment_row = sqlx::query!(
            "SELECT id FROM assignments WHERE canvas_id = ?",
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = assignment_row {
            self.row_to_assignment(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Assignment>, Error> {
        let assignment_row = sqlx::query!(
            "SELECT id FROM assignments WHERE discourse_id = ?",
            discourse_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = assignment_row {
            self.row_to_assignment(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_group_category_id(&self, group_category_id: &str) -> Result<Vec<Assignment>, Error> {
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE group_category_id = ? ORDER BY due_date ASC",
            group_category_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_by_assignment_group_id(&self, assignment_group_id: &str) -> Result<Vec<Assignment>, Error> {
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE assignment_group_id = ? ORDER BY position ASC",
            assignment_group_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_by_quiz_id(&self, quiz_id: &str) -> Result<Option<Assignment>, Error> {
        let assignment_row = sqlx::query!(
            "SELECT id FROM assignments WHERE quiz_id = ?",
            quiz_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = assignment_row {
            self.row_to_assignment(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discussion_topic_id(&self, discussion_topic_id: &str) -> Result<Option<Assignment>, Error> {
        let assignment_row = sqlx::query!(
            "SELECT id FROM assignments WHERE discussion_topic_id = ?",
            discussion_topic_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = assignment_row {
            self.row_to_assignment(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_due_date_range(&self, start_date: &str, end_date: &str) -> Result<Vec<Assignment>, Error> {
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE due_date >= ? AND due_date <= ? ORDER BY due_date ASC",
            start_date,
            end_date
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_overdue_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error> {
        let now = Utc::now().to_rfc3339();
        
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE course_id = ? AND due_date < ? AND status = 'published' ORDER BY due_date ASC",
            course_id,
            now
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn find_upcoming_by_course_id(&self, course_id: &str) -> Result<Vec<Assignment>, Error> {
        let now = Utc::now().to_rfc3339();
        
        let assignment_rows = sqlx::query!(
            "SELECT id FROM assignments WHERE course_id = ? AND due_date >= ? AND status = 'published' ORDER BY due_date ASC",
            course_id,
            now
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut assignments = Vec::new();
        for row in assignment_rows {
            if let Some(assignment) = self.row_to_assignment(&row.id).await? {
                assignments.push(assignment);
            }
        }
        
        Ok(assignments)
    }
    
    async fn publish(&self, id: &str) -> Result<Assignment, Error> {
        // Get the assignment
        let mut assignment = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Assignment with ID {} not found", id)))?;
        
        // Publish the assignment
        assignment.publish();
        
        // Update the assignment
        self.update(&assignment).await
    }
    
    async fn unpublish(&self, id: &str) -> Result<Assignment, Error> {
        // Get the assignment
        let mut assignment = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Assignment with ID {} not found", id)))?;
        
        // Unpublish the assignment
        assignment.unpublish();
        
        // Update the assignment
        self.update(&assignment).await
    }
    
    async fn delete_assignment(&self, id: &str) -> Result<(), Error> {
        // Get the assignment
        let mut assignment = self.find_by_id(&id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Assignment with ID {} not found", id)))?;
        
        // Mark the assignment as deleted
        assignment.delete();
        
        // Update the assignment
        self.update(&assignment).await?;
        
        Ok(())
    }
}
