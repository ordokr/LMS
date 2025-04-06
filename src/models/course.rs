use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: Uuid,
    pub canvas_id: String,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub instructor_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub category_id: Option<Uuid>,  // Link to Discourse category
    pub is_published: bool,
}

impl Course {
    pub fn new(
        canvas_id: String,
        name: String,
        code: String,
        description: Option<String>,
        instructor_id: Uuid,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            canvas_id,
            name,
            code,
            description,
            instructor_id,
            start_date,
            end_date,
            created_at: now,
            updated_at: now,
            category_id: None,
            is_published: false,
        }
    }
}