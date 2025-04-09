use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::user::user::User;
use crate::models::course::enrollment::Enrollment;
use crate::models::content::assignment::Assignment;
use crate::models::forum::topic::Topic;
use crate::models::forum::category::Category;
use crate::models::ids::CourseId;
use crate::models::ids::UserId;

/// Course model based primarily on Canvas Course
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub instructor_id: Option<i64>,
    
    // Convert string dates to proper DateTime types
    #[serde(default)]
    pub start_date: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub end_date: Option<DateTime<Utc>>,
    
    pub status: CourseStatus,
    
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseStatus {
    Active,
    Archived,
    Unpublished,
    Deleted,
}

impl Course {
    pub fn new(name: String, code: String) -> Self {
        Course {
            id: Uuid::new_v4(),
            name,
            code,
            description: None,
            syllabus_body: None,
            start_date: None,
            end_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            timezone: None,
            license: None,
            is_public: false,
            is_published: false,
            default_view: "modules".to_string(),
            canvas_course_id: None,
        }
    }

    /// Validate course data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Name shouldn't be empty
        if self.name.trim().is_empty() {
            errors.push("Course name cannot be empty".to_string());
        }
        
        // Course code shouldn't be empty
        if self.code.trim().is_empty() {
            errors.push("Course code cannot be empty".to_string());
        }
        
        // End date should be after start date if both exist
        if let (Some(start), Some(end)) = (self.start_date, self.end_date) {
            if end < start {
                errors.push("End date cannot be before start date".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    /// Create a migration function to convert from legacy string-based dates
    pub fn from_legacy(legacy_course: LegacyCourse) -> Self {
        use crate::utils::date_utils::parse_date_string;
        
        Self {
            id: legacy_course.id,
            name: legacy_course.name,
            code: legacy_course.code,
            description: legacy_course.description,
            instructor_id: legacy_course.instructor_id,
            start_date: parse_date_string(legacy_course.start_date.as_deref()),
            end_date: parse_date_string(legacy_course.end_date.as_deref()),
            status: legacy_course.status,
            created_at: parse_date_string(legacy_course.created_at.as_deref()),
            updated_at: parse_date_string(legacy_course.updated_at.as_deref()),
        }
    }

    // Canvas API integration method
    pub fn from_canvas_api(canvas_course: &serde_json::Value) -> Result<Self, String> {
        use crate::utils::date_utils::parse_date_string;
        
        // Extract and validate required fields
        let name = canvas_course["name"]
            .as_str()
            .ok_or("Missing or invalid course name")?
            .to_string();
            
        let code = canvas_course["course_code"]
            .as_str()
            .ok_or("Missing or invalid course code")?
            .to_string();
        
        // Extract optional fields
        let id = canvas_course["id"].as_i64();
        let description = canvas_course["description"].as_str().map(String::from);
        let instructor_id = canvas_course["account_id"].as_i64();
        
        // Parse dates
        let start_date = parse_date_string(canvas_course["start_at"].as_str());
        let end_date = parse_date_string(canvas_course["end_at"].as_str());
        let created_at = parse_date_string(canvas_course["created_at"].as_str());
        let updated_at = parse_date_string(canvas_course["updated_at"].as_str());
        
        // Determine status
        let status_str = canvas_course["workflow_state"].as_str().unwrap_or("unpublished");
        let status = match status_str {
            "available" | "active" => CourseStatus::Active,
            "completed" | "concluded" => CourseStatus::Archived,
            "deleted" => CourseStatus::Deleted,
            _ => CourseStatus::Unpublished,
        };
        
        Ok(Self {
            id,
            name,
            code,
            description,
            instructor_id,
            start_date,
            end_date,
            status,
            created_at,
            updated_at,
        })
    }

    // Database operations
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let course = sqlx::query_as::<_, Course>(
            "SELECT * FROM courses WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(course)
    }
    
    pub async fn find_by_code(db: &DB, code: &str) -> Result<Self, Error> {
        let course = sqlx::query_as::<_, Course>(
            "SELECT * FROM courses WHERE code = ?"
        )
        .bind(code)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(course)
    }
    
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let course = sqlx::query_as::<_, Course>(
            "SELECT * FROM courses WHERE canvas_course_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(course)
    }
    
    pub async fn find_all(db: &DB) -> Result<Vec<Self>, Error> {
        let courses = sqlx::query_as::<_, Course>(
            "SELECT * FROM courses ORDER BY name"
        )
        .fetch_all(&db.pool)
        .await?;
        
        Ok(courses)
    }
    
    pub async fn find_public(db: &DB) -> Result<Vec<Self>, Error> {
        let courses = sqlx::query_as::<_, Course>(
            "SELECT * FROM courses WHERE is_public = 1 AND is_published = 1 ORDER BY name ASC"
        )
        .fetch_all(&db.pool)
        .await?;
        
        Ok(courses)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO courses 
            (id, name, code, description, syllabus_body, start_date, end_date, 
            created_at, updated_at, timezone, license, is_public, is_published, 
            default_view, canvas_course_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(&self.name)
        .bind(&self.code)
        .bind(&self.description)
        .bind(&self.syllabus_body)
        .bind(self.start_date)
        .bind(self.end_date)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(&self.timezone)
        .bind(&self.license)
        .bind(self.is_public)
        .bind(self.is_published)
        .bind(&self.default_view)
        .bind(&self.canvas_course_id)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE courses SET 
            name = ?, code = ?, description = ?, syllabus_body = ?, 
            start_date = ?, end_date = ?, updated_at = ?, timezone = ?, 
            license = ?, is_public = ?, is_published = ?, default_view = ?,
            canvas_course_id = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.code)
        .bind(&self.description)
        .bind(&self.syllabus_body)
        .bind(self.start_date)
        .bind(self.end_date)
        .bind(Utc::now())
        .bind(&self.timezone)
        .bind(&self.license)
        .bind(self.is_public)
        .bind(self.is_published)
        .bind(&self.default_view)
        .bind(&self.canvas_course_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        // In a real application, we would need to handle all dependencies
        // (enrollments, assignments, topics, etc.) before deleting the course
        sqlx::query("DELETE FROM courses WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Relationship methods
    
    pub async fn enrollments(&self, db: &DB) -> Result<Vec<Enrollment>, Error> {
        Enrollment::find_by_course_id(db, self.id).await
    }
    
    pub async fn assignments(&self, db: &DB) -> Result<Vec<Assignment>, Error> {
        Assignment::find_by_course_id(db, self.id).await
    }
    
    pub async fn topics(&self, db: &DB) -> Result<Vec<Topic>, Error> {
        Topic::find_by_course_id(db, self.id).await
    }
    
    pub async fn categories(&self, db: &DB) -> Result<Vec<Category>, Error> {
        Category::find_by_course_id(db, self.id).await
    }
    
    pub async fn students(&self, db: &DB) -> Result<Vec<User>, Error> {
        let enrollments = Enrollment::find_by_course_id_and_role(db, self.id, "student").await?;
        let mut students = Vec::with_capacity(enrollments.len());
        
        for enrollment in enrollments {
            students.push(User::find(db, enrollment.user_id).await?);
        }
        
        Ok(students)
    }
    
    pub async fn teachers(&self, db: &DB) -> Result<Vec<User>, Error> {
        let enrollments = Enrollment::find_by_course_id_and_role(db, self.id, "teacher").await?;
        let mut teachers = Vec::with_capacity(enrollments.len());
        
        for enrollment in enrollments {
            teachers.push(User::find(db, enrollment.user_id).await?);
        }
        
        Ok(teachers)
    }
    
    pub async fn user_is_enrolled(&self, db: &DB, user_id: Uuid) -> Result<bool, Error> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM enrollments WHERE course_id = ? AND user_id = ?"
        )
        .bind(self.id)
        .bind(user_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(count > 0)
    }
}

/// Legacy course model with string dates for migration purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyCourse {
    pub id: Option<i64>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub instructor_id: Option<i64>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: CourseStatus,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}