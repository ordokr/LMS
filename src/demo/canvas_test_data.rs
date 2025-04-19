use sqlx::SqlitePool;
use crate::models::course::{Course, CourseStatus};
use crate::models::user::User;
use crate::adapters::canvas_adapter::{CanvasClient, CanvasCourse, CanvasUser};
use uuid::Uuid;
use chrono::Utc;
use tokio::task;
use std::sync::Arc;
use log::{info, error};

// This module provides a test data generator for development, testing, or demonstration purposes only.
// It is NOT intended for production data migration from live Canvas deployments. 
// Ordo is a source-to-source port and does not support or recommend live data migration or user import from existing systems.
// This is purely for creating test/demo data for development purposes.

pub struct CanvasTestDataGenerator {
    db: SqlitePool,
    canvas_client: CanvasClient,
}

impl CanvasTestDataGenerator {
    /// Create a new test data generator for development/testing/demo use only.
    pub fn new(db: SqlitePool, canvas_url: &str, canvas_token: &str) -> Self {
        Self {
            db,
            canvas_client: CanvasClient::new(canvas_url, canvas_token),
        }
    }
    
    /// Generate test courses for development/testing/demo only. Not for production data migration.
    pub async fn generate_test_courses(&self) -> Result<(usize, usize, Vec<String>), String> {
        info!("Starting Canvas test course generation");
        let mut imported = 0;
        let mut updated = 0;
        let mut errors = Vec::new();
        
        // Fetch courses from Canvas
        let canvas_courses = match self.canvas_client.get_courses().await {
            Ok(courses) => courses,
            Err(e) => return Err(format!("Failed to fetch Canvas courses: {}", e)),
        };
        
        for canvas_course in canvas_courses {
            match self.create_test_course(&canvas_course).await {
                Ok(true) => imported += 1,
                Ok(false) => updated += 1,
                Err(e) => errors.push(format!("Error creating test course {}: {}", canvas_course.id, e)),
            }
        }
        
        info!("Canvas test course generation completed: {} created, {} updated, {} errors", 
             imported, updated, errors.len());
        
        Ok((imported, updated, errors))
    }
    
    /// Create a single test course for development/testing/demo only.
    async fn create_test_course(&self, canvas_course: &CanvasCourse) -> Result<bool, String> {
        // Check if course already exists
        let existing = sqlx::query!(
            "SELECT id FROM courses WHERE canvas_course_id = ?",
            canvas_course.id.to_string()
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| format!("Database error: {}", e))?;
        
        let now = Utc::now().to_rfc3339();
        
        if let Some(row) = existing {
            // Update existing course
            sqlx::query!(
                r#"
                UPDATE courses
                SET 
                    name = ?,
                    code = ?,
                    description = ?,
                    status = ?,
                    updated_at = ?
                WHERE id = ?
                "#,
                canvas_course.name,
                canvas_course.course_code,
                canvas_course.description,
                map_canvas_status(&canvas_course.workflow_state),
                now,
                row.id
            )
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to update course: {}", e))?;
            
            Ok(false) // Updated
        } else {
            // Create new course
            let id = Uuid::new_v4().to_string();
            
            sqlx::query!(
                r#"
                INSERT INTO courses (
                    id, code, name, description, status,
                    canvas_course_id, integration_status,
                    created_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                id,
                canvas_course.course_code,
                canvas_course.name,
                canvas_course.description,
                map_canvas_status(&canvas_course.workflow_state),
                canvas_course.id.to_string(),
                "partiallyintegrated", // Only Canvas integration at this point
                now,
                now
            )
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to create course: {}", e))?;
            
            // Generate test enrollments
            task::spawn(self.generate_test_enrollments(id.clone(), canvas_course.id));
            
            Ok(true) // New import
        }
    }
    
    /// Generate test enrollments for development/testing/demo only.
    async fn generate_test_enrollments(&self, course_id: String, canvas_course_id: i64) -> Result<(), String> {
        let enrollments = self.canvas_client.get_course_enrollments(canvas_course_id).await
            .map_err(|e| format!("Failed to fetch enrollments: {}", e))?;
        
        for enrollment in enrollments {
            // Create the test user if they don't exist
            let user_id = self.create_test_user(&enrollment.user).await?;
            
            // Create enrollment
            let enrollment_id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();
            
            sqlx::query!(
                r#"
                INSERT OR IGNORE INTO course_enrollments (
                    id, course_id, user_id, role, created_at
                ) VALUES (?, ?, ?, ?, ?)
                "#,
                enrollment_id,
                course_id,
                user_id,
                enrollment.role,
                now
            )
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to create enrollment: {}", e))?;
        }
        
        Ok(())
    }
    
    /// Create a test user for development/testing/demo only.
    async fn create_test_user(&self, canvas_user: &CanvasUser) -> Result<String, String> {
        // Check if user already exists
        if let Some(row) = sqlx::query!(
            "SELECT id FROM users WHERE canvas_user_id = ?",
            canvas_user.id.to_string()
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| format!("Database error: {}", e))? {
            return Ok(row.id);
        }
        
        // Create new user
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let username = generate_username_from_email(&canvas_user.email);
        
        sqlx::query!(
            r#"
            INSERT INTO users (
                id, username, email, display_name, canvas_user_id, 
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            username,
            canvas_user.email,
            canvas_user.name,
            canvas_user.id.to_string(),
            now,
            now
        )
        .execute(&self.db)
        .await
        .map_err(|e| format!("Failed to create user: {}", e))?;
        
        // Create user profile
        sqlx::query!(
            r#"
            INSERT INTO user_profiles (
                user_id, trust_level, is_moderator, is_admin
            ) VALUES (?, ?, ?, ?)
            "#,
            id,
            0, // Default trust level
            false,
            false
        )
        .execute(&self.db)
        .await
        .map_err(|e| format!("Failed to create user profile: {}", e))?;
        
        Ok(id)
    }
}

// Helper function to map Canvas course status to our status enum
fn map_canvas_status(status: &str) -> &'static str {
    match status {
        "available" => "active",
        "completed" => "completed",
        "deleted" => "archived",
        "unpublished" => "draft",
        _ => "active",
    }
}

// Generate username from email
fn generate_username_from_email(email: &str) -> String {
    email.split('@').next().unwrap_or("user").to_string()
}
