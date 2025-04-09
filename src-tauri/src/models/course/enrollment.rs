use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::db::DB;
use crate::error::Error;
use crate::models::user::user::User;
use crate::models::course::course::Course;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentType {
    Student,
    Teacher,
    TA,
    Observer,
    Designer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentState {
    Active,
    Invited,
    Inactive,
    Completed,
    Rejected,
}

/// Enrollment model, representing a user's enrollment in a course
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Enrollment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub role: String, // "student", "teacher", "ta", "designer", "observer"
    pub state: String, // "active", "invited", "completed", "rejected"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub canvas_enrollment_id: Option<String>,
}

impl Enrollment {
    pub fn new(user_id: Uuid, course_id: Uuid, role: EnrollmentType) -> Self {
        Enrollment {
            id: Uuid::new_v4(),
            user_id,
            course_id,
            role: role.to_string(),
            state: EnrollmentState::Active.to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_activity_at: None,
            start_at: None,
            end_at: None,
            canvas_enrollment_id: None,
        }
    }

    // Database operations
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let enrollment = sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollments WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(enrollment)
    }
    
    pub async fn find_by_user_id(db: &DB, user_id: Uuid) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollments WHERE user_id = ? ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(enrollments)
    }
    
    pub async fn find_by_course_id(db: &DB, course_id: Uuid) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollments WHERE course_id = ? ORDER BY role, created_at"
        )
        .bind(course_id)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(enrollments)
    }
    
    pub async fn find_by_course_id_and_role(db: &DB, course_id: Uuid, role: &str) -> Result<Vec<Self>, Error> {
        let enrollments = sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollments WHERE course_id = ? AND role = ? ORDER BY created_at"
        )
        .bind(course_id)
        .bind(role)
        .fetch_all(&db.pool)
        .await?;
        
        Ok(enrollments)
    }
    
    pub async fn find_by_user_and_course(db: &DB, user_id: Uuid, course_id: Uuid) -> Result<Self, Error> {
        let enrollment = sqlx::query_as::<_, Enrollment>(
            "SELECT * FROM enrollments WHERE user_id = ? AND course_id = ?"
        )
        .bind(user_id)
        .bind(course_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(enrollment)
    }
    
    pub async fn create(&self, db: &DB) -> Result<Uuid, Error> {
        sqlx::query(
            "INSERT INTO enrollments 
            (id, user_id, course_id, role, state, created_at, updated_at,
            last_activity_at, start_at, end_at, canvas_enrollment_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(self.id)
        .bind(self.user_id)
        .bind(self.course_id)
        .bind(&self.role)
        .bind(&self.state)
        .bind(self.created_at)
        .bind(self.updated_at)
        .bind(self.last_activity_at)
        .bind(self.start_at)
        .bind(self.end_at)
        .bind(&self.canvas_enrollment_id)
        .execute(&db.pool)
        .await?;
        
        Ok(self.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE enrollments SET
            user_id = ?, course_id = ?, role = ?, state = ?, updated_at = ?,
            last_activity_at = ?, start_at = ?, end_at = ?, canvas_enrollment_id = ?
            WHERE id = ?"
        )
        .bind(self.user_id)
        .bind(self.course_id)
        .bind(&self.role)
        .bind(&self.state)
        .bind(Utc::now())
        .bind(self.last_activity_at)
        .bind(self.start_at)
        .bind(self.end_at)
        .bind(&self.canvas_enrollment_id)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM enrollments WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    pub async fn update_last_activity(&mut self, db: &DB) -> Result<(), Error> {
        self.last_activity_at = Some(Utc::now());
        
        sqlx::query(
            "UPDATE enrollments SET last_activity_at = ? WHERE id = ?"
        )
        .bind(self.last_activity_at)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    // Relationship methods
    
    pub async fn user(&self, db: &DB) -> Result<User, Error> {
        User::find(db, self.user_id).await
    }
    
    pub async fn course(&self, db: &DB) -> Result<Course, Error> {
        Course::find(db, self.course_id).await
    }
}

// Implement conversions between string and enum
impl ToString for EnrollmentType {
    fn to_string(&self) -> String {
        match self {
            EnrollmentType::Student => "student".to_string(),
            EnrollmentType::Teacher => "teacher".to_string(),
            EnrollmentType::TA => "ta".to_string(),
            EnrollmentType::Observer => "observer".to_string(),
            EnrollmentType::Designer => "designer".to_string(),
        }
    }
}

impl From<&str> for EnrollmentType {
    fn from(s: &str) -> Self {
        match s {
            "student" => EnrollmentType::Student,
            "teacher" => EnrollmentType::Teacher,
            "ta" => EnrollmentType::TA,
            "observer" => EnrollmentType::Observer,
            "designer" => EnrollmentType::Designer,
            _ => EnrollmentType::Student, // Default
        }
    }
}

impl ToString for EnrollmentState {
    fn to_string(&self) -> String {
        match self {
            EnrollmentState::Active => "active".to_string(),
            EnrollmentState::Invited => "invited".to_string(),
            EnrollmentState::Inactive => "inactive".to_string(),
            EnrollmentState::Completed => "completed".to_string(),
            EnrollmentState::Rejected => "rejected".to_string(),
        }
    }
}

impl From<&str> for EnrollmentState {
    fn from(s: &str) -> Self {
        match s {
            "active" => EnrollmentState::Active,
            "invited" => EnrollmentState::Invited,
            "inactive" => EnrollmentState::Inactive,
            "completed" => EnrollmentState::Completed,
            "rejected" => EnrollmentState::Rejected,
            _ => EnrollmentState::Active, // Default
        }
    }
}