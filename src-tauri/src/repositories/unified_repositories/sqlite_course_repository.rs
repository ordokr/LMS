use async_trait::async_trait;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use std::collections::HashMap;
use crate::error::Error;
use crate::models::unified_models::{Course, CourseStatus, CourseVisibility, HomepageType};
use super::repository::Repository;
use super::course_repository::CourseRepository;

/// SQLite implementation of the course repository
pub struct SqliteCourseRepository {
    pool: Pool<Sqlite>,
}

impl SqliteCourseRepository {
    /// Create a new SQLite course repository
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    /// Helper method to convert a row to a Course
    async fn row_to_course(&self, id: &str) -> Result<Option<Course>, Error> {
        let course_row = sqlx::query!(
            r#"
            SELECT 
                id, name, code, description, created_at, updated_at, status, visibility,
                is_public, is_published, start_date, end_date, instructor_id,
                allow_self_enrollment, enrollment_code, enrollment_count, syllabus_body,
                homepage_type, default_view, theme_color, banner_image_url, timezone,
                license, canvas_id, discourse_id, category_id, slug, color, position,
                parent_id, last_sync, source_system, metadata
            FROM courses
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if let Some(row) = course_row {
            // Parse timestamps
            let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse created_at: {}", e)))?
                .with_timezone(&Utc);
                
            let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map_err(|e| Error::ParseError(format!("Failed to parse updated_at: {}", e)))?
                .with_timezone(&Utc);
                
            let start_date = if let Some(ts) = row.start_date {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .map_err(|e| Error::ParseError(format!("Failed to parse start_date: {}", e)))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            let end_date = if let Some(ts) = row.end_date {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .map_err(|e| Error::ParseError(format!("Failed to parse end_date: {}", e)))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            let last_sync = if let Some(ts) = row.last_sync {
                Some(
                    chrono::DateTime::parse_from_rfc3339(&ts)
                        .map_err(|e| Error::ParseError(format!("Failed to parse last_sync: {}", e)))?
                        .with_timezone(&Utc)
                )
            } else {
                None
            };
            
            // Parse status
            let status = CourseStatus::from(row.status.as_str());
            
            // Parse visibility
            let visibility = match row.visibility.as_str() {
                "public" => CourseVisibility::Public,
                "institution" => CourseVisibility::Institution,
                "private" => CourseVisibility::Private,
                _ => CourseVisibility::Course,
            };
            
            // Parse homepage type
            let homepage_type = match row.homepage_type.as_str() {
                "feed" => HomepageType::Feed,
                "assignments" => HomepageType::Assignments,
                "syllabus" => HomepageType::Syllabus,
                "custom" => HomepageType::Custom,
                _ => HomepageType::Modules,
            };
            
            // Parse metadata
            let metadata: HashMap<String, serde_json::Value> = if let Some(meta_str) = row.metadata {
                serde_json::from_str(&meta_str)
                    .map_err(|e| Error::ParseError(format!("Failed to parse metadata: {}", e)))?
            } else {
                HashMap::new()
            };
            
            // Create course
            let course = Course {
                id: row.id,
                name: row.name,
                code: row.code,
                description: row.description,
                created_at,
                updated_at,
                status,
                visibility,
                is_public: row.is_public != 0,
                is_published: row.is_published != 0,
                start_date,
                end_date,
                instructor_id: row.instructor_id,
                allow_self_enrollment: row.allow_self_enrollment != 0,
                enrollment_code: row.enrollment_code,
                enrollment_count: row.enrollment_count,
                syllabus_body: row.syllabus_body,
                homepage_type,
                default_view: row.default_view,
                theme_color: row.theme_color,
                banner_image_url: row.banner_image_url,
                timezone: row.timezone,
                license: row.license,
                canvas_id: row.canvas_id,
                discourse_id: row.discourse_id,
                category_id: row.category_id,
                slug: row.slug,
                color: row.color,
                position: row.position,
                parent_id: row.parent_id,
                last_sync,
                source_system: row.source_system,
                metadata,
            };
            
            Ok(Some(course))
        } else {
            Ok(None)
        }
    }
}

#[async_trait]
impl Repository<Course, String> for SqliteCourseRepository {
    async fn find_by_id(&self, id: &String) -> Result<Option<Course>, Error> {
        self.row_to_course(id).await
    }
    
    async fn find_all(&self) -> Result<Vec<Course>, Error> {
        let course_ids = sqlx::query!(
            "SELECT id FROM courses"
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut courses = Vec::new();
        for row in course_ids {
            if let Some(course) = self.row_to_course(&row.id).await? {
                courses.push(course);
            }
        }
        
        Ok(courses)
    }
    
    async fn create(&self, course: &Course) -> Result<Course, Error> {
        // Serialize metadata
        let metadata_json = serde_json::to_string(&course.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Insert course
        sqlx::query!(
            r#"
            INSERT INTO courses (
                id, name, code, description, created_at, updated_at, status, visibility,
                is_public, is_published, start_date, end_date, instructor_id,
                allow_self_enrollment, enrollment_code, enrollment_count, syllabus_body,
                homepage_type, default_view, theme_color, banner_image_url, timezone,
                license, canvas_id, discourse_id, category_id, slug, color, position,
                parent_id, last_sync, source_system, metadata
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            "#,
            course.id,
            course.name,
            course.code,
            course.description,
            course.created_at.to_rfc3339(),
            course.updated_at.to_rfc3339(),
            course.status.to_string(),
            course.visibility.to_string(),
            if course.is_public { 1 } else { 0 },
            if course.is_published { 1 } else { 0 },
            course.start_date.map(|dt| dt.to_rfc3339()),
            course.end_date.map(|dt| dt.to_rfc3339()),
            course.instructor_id,
            if course.allow_self_enrollment { 1 } else { 0 },
            course.enrollment_code,
            course.enrollment_count,
            course.syllabus_body,
            course.homepage_type.to_string(),
            course.default_view,
            course.theme_color,
            course.banner_image_url,
            course.timezone,
            course.license,
            course.canvas_id,
            course.discourse_id,
            course.category_id,
            course.slug,
            course.color,
            course.position,
            course.parent_id,
            course.last_sync.map(|dt| dt.to_rfc3339()),
            course.source_system,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        
        // Return the created course
        Ok(course.clone())
    }
    
    async fn update(&self, course: &Course) -> Result<Course, Error> {
        // Serialize metadata
        let metadata_json = serde_json::to_string(&course.metadata)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize metadata: {}", e)))?;
        
        // Update course
        sqlx::query!(
            r#"
            UPDATE courses SET
                name = ?, code = ?, description = ?, updated_at = ?, status = ?, visibility = ?,
                is_public = ?, is_published = ?, start_date = ?, end_date = ?, instructor_id = ?,
                allow_self_enrollment = ?, enrollment_code = ?, enrollment_count = ?, syllabus_body = ?,
                homepage_type = ?, default_view = ?, theme_color = ?, banner_image_url = ?, timezone = ?,
                license = ?, canvas_id = ?, discourse_id = ?, category_id = ?, slug = ?, color = ?, position = ?,
                parent_id = ?, last_sync = ?, source_system = ?, metadata = ?
            WHERE id = ?
            "#,
            course.name,
            course.code,
            course.description,
            course.updated_at.to_rfc3339(),
            course.status.to_string(),
            course.visibility.to_string(),
            if course.is_public { 1 } else { 0 },
            if course.is_published { 1 } else { 0 },
            course.start_date.map(|dt| dt.to_rfc3339()),
            course.end_date.map(|dt| dt.to_rfc3339()),
            course.instructor_id,
            if course.allow_self_enrollment { 1 } else { 0 },
            course.enrollment_code,
            course.enrollment_count,
            course.syllabus_body,
            course.homepage_type.to_string(),
            course.default_view,
            course.theme_color,
            course.banner_image_url,
            course.timezone,
            course.license,
            course.canvas_id,
            course.discourse_id,
            course.category_id,
            course.slug,
            course.color,
            course.position,
            course.parent_id,
            course.last_sync.map(|dt| dt.to_rfc3339()),
            course.source_system,
            metadata_json,
            course.id
        )
        .execute(&self.pool)
        .await?;
        
        // Return the updated course
        Ok(course.clone())
    }
    
    async fn delete(&self, id: &String) -> Result<(), Error> {
        sqlx::query!("DELETE FROM courses WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
    
    async fn count(&self) -> Result<i64, Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM courses")
            .fetch_one(&self.pool)
            .await?;
            
        Ok(result.count)
    }
}

#[async_trait]
impl CourseRepository for SqliteCourseRepository {
    async fn find_by_code(&self, code: &str) -> Result<Option<Course>, Error> {
        let course_row = sqlx::query!("SELECT id FROM courses WHERE code = ?", code)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = course_row {
            self.row_to_course(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<Course>, Error> {
        let course_row = sqlx::query!("SELECT id FROM courses WHERE canvas_id = ?", canvas_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = course_row {
            self.row_to_course(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<Course>, Error> {
        let course_row = sqlx::query!("SELECT id FROM courses WHERE discourse_id = ?", discourse_id)
            .fetch_optional(&self.pool)
            .await?;
            
        if let Some(row) = course_row {
            self.row_to_course(&row.id).await
        } else {
            Ok(None)
        }
    }
    
    async fn find_by_instructor_id(&self, instructor_id: &str) -> Result<Vec<Course>, Error> {
        let course_rows = sqlx::query!("SELECT id FROM courses WHERE instructor_id = ?", instructor_id)
            .fetch_all(&self.pool)
            .await?;
            
        let mut courses = Vec::new();
        for row in course_rows {
            if let Some(course) = self.row_to_course(&row.id).await? {
                courses.push(course);
            }
        }
        
        Ok(courses)
    }
    
    async fn find_by_status(&self, status: CourseStatus) -> Result<Vec<Course>, Error> {
        let status_str = status.to_string();
        let course_rows = sqlx::query!("SELECT id FROM courses WHERE status = ?", status_str)
            .fetch_all(&self.pool)
            .await?;
            
        let mut courses = Vec::new();
        for row in course_rows {
            if let Some(course) = self.row_to_course(&row.id).await? {
                courses.push(course);
            }
        }
        
        Ok(courses)
    }
    
    async fn find_active_courses(&self) -> Result<Vec<Course>, Error> {
        self.find_by_status(CourseStatus::Active).await
    }
    
    async fn find_archived_courses(&self) -> Result<Vec<Course>, Error> {
        self.find_by_status(CourseStatus::Archived).await
    }
    
    async fn activate_course(&self, course_id: &str) -> Result<Course, Error> {
        // Get the course
        let mut course = self.find_by_id(&course_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Course with ID {} not found", course_id)))?;
        
        // Activate the course
        course.activate();
        
        // Update the course
        self.update(&course).await
    }
    
    async fn archive_course(&self, course_id: &str) -> Result<Course, Error> {
        // Get the course
        let mut course = self.find_by_id(&course_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Course with ID {} not found", course_id)))?;
        
        // Archive the course
        course.archive();
        
        // Update the course
        self.update(&course).await
    }
    
    async fn delete_course(&self, course_id: &str) -> Result<(), Error> {
        // Get the course
        let mut course = self.find_by_id(&course_id.to_string()).await?
            .ok_or_else(|| Error::NotFound(format!("Course with ID {} not found", course_id)))?;
        
        // Mark the course as deleted
        course.delete();
        
        // Update the course
        self.update(&course).await?;
        
        Ok(())
    }
}
