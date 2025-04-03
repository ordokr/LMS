use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub instructor_id: i64,
    pub created_at: String,
    pub updated_at: String,
}
