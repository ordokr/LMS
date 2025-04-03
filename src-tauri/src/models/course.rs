use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub code: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub status: CourseStatus,
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