use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Course {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub created_at: DateTime<Utc>,
    pub status: CourseStatus, // Added field for workflow state
}

impl Course {
    pub fn activate(&mut self) {
        self.status = CourseStatus::Active;
    }

    pub fn archive(&mut self) {
        self.status = CourseStatus::Archived;
    }

    pub fn is_active(&self) -> bool {
        self.status == CourseStatus::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CourseStatus {
    Draft,
    Active,
    Archived,
}

impl Default for CourseStatus {
    fn default() -> Self {
        CourseStatus::Draft
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub published: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Option<i64>,
    pub course_id: i64,
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub points: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Option<i64>,
    pub assignment_id: i64,
    pub user_id: i64,
    pub content: String,
    pub submitted_at: Option<String>,
    pub grade: Option<f32>,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCreate {
    pub title: String,
    pub description: String,
    pub status: CourseStatus,
    pub modules: Option<Vec<String>>, // Optional initial modules
}