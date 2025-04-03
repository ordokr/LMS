use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub instructor_id: i64,
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
pub struct Enrollment {
    pub id: Option<i64>,
    pub user_id: i64,
    pub course_id: i64,
    pub role: EnrollmentRole,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EnrollmentRole {
    Student,
    Teacher,
    TeachingAssistant,
    Observer,
}