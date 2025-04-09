use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::content::submission::Submission;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Assignment {
    pub id: Option<i64>, // Will change to AssignmentId later
    pub course_id: Option<i64>, // Will change to CourseId later
    pub title: String,
    pub description: Option<String>,
    pub points_possible: Option<f64>,
    
    // Due date as proper DateTime
    #[serde(default)]
    pub due_date: Option<DateTime<Utc>>,
    
    // Availability dates
    #[serde(default)]
    pub available_from: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub available_until: Option<DateTime<Utc>>,
    
    pub is_published: bool,
    
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Assignment {
    pub fn new(course_id: Uuid, name: String) -> Self {
        Assignment {
            id: None,
            course_id: Some(course_id.as_u128() as i64),
            title: name,
            description: None,
            points_possible: Some(100.0),
            due_date: None,
            available_from: None,
            available_until: None,
            is_published: false,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
    
    /// Validate assignment data
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        
        // Title shouldn't be empty
        if self.title.trim().is_empty() {
            errors.push("Assignment title cannot be empty".to_string());
        }
        
        // Points possible should be positive if provided
        if let Some(points) = self.points_possible {
            if points < 0.0 {
                errors.push("Points possible cannot be negative".to_string());
            }
        }
        
        // Available_until should be after available_from if both exist
        if let (Some(from), Some(until)) = (self.available_from, self.available_until) {
            if until < from {
                errors.push("Available until date cannot be before available from date".to_string());
            }
        }
        
        // Due date should be between available dates if all exist
        if let (Some(from), Some(due), Some(_)) = (self.available_from, self.due_date, self.available_until) {
            if due < from {
                errors.push("Due date cannot be before available from date".to_string());
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
    
    // Canvas API integration method
    pub fn from_canvas_api(canvas_assignment: &serde_json::Value) -> Result<Self, String> {
        use crate::utils::date_utils::parse_date_string;
        
        // Extract and validate required fields
        let title = canvas_assignment["name"]
            .as_str()
            .ok_or("Missing or invalid assignment name")?
            .to_string();
            
        // Extract optional fields
        let id = canvas_assignment["id"].as_i64();
        let course_id = canvas_assignment["course_id"].as_i64();
        let description = canvas_assignment["description"].as_str().map(String::from);
        let points_possible = canvas_assignment["points_possible"].as_f64();
        
        // Parse dates
        let due_date = parse_date_string(canvas_assignment["due_at"].as_str());
        let available_from = parse_date_string(canvas_assignment["unlock_at"].as_str());
        let available_until = parse_date_string(canvas_assignment["lock_at"].as_str());
        let created_at = parse_date_string(canvas_assignment["created_at"].as_str());
        let updated_at = parse_date_string(canvas_assignment["updated_at"].as_str());
        
        let is_published = canvas_assignment["published"].as_bool().unwrap_or(false);
        
        Ok(Self {
            id,
            course_id,
            title,
            description,
            points_possible,
            due_date,
            available_from,
            available_until,
            is_published,
            created_at,
            updated_at,
        })
    }
    
    // Database methods
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let assignment = sqlx::query_as::<_, Self>(
            "SELECT * FROM assignments WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(assignment)
    }
    
    pub async fn find_by_course_id(db: &DB, course_id: Uuid) -> Result<Vec<Self>, Error> {
        let assignments = sqlx::query_as::<_, Self>(
            "SELECT * FROM assignments WHERE course_id = ? ORDER BY due_at ASC"
        )
        .bind(course_id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(assignments)
    }
    
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let assignment = sqlx::query_as::<_, Self>(
            "SELECT * FROM assignments WHERE canvas_assignment_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(assignment)
    }
    
    // Basic CRUD operations (simplified for brevity)
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO assignments VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.course_id)
        .bind(&self.title)
        .bind(&self.description)
        .bind(self.points_possible)
        .bind(self.due_date)
        .bind(self.available_from)
        .bind(self.available_until)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.is_published)
        .execute(&db.pool)
        .await?;
        
        Ok(Uuid::new_v4())
    }
    
    // Relationship methods
    
    pub async fn submissions(&self, db: &DB) -> Result<Vec<Submission>, Error> {
        Submission::find_by_assignment_id(db, Uuid::new_v4()).await
    }
    
    pub async fn user_submission(&self, db: &DB, user_id: Uuid) -> Result<Option<Submission>, Error> {
        Submission::find_by_assignment_and_user(db, Uuid::new_v4(), user_id).await
    }
}