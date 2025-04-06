use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub points_possible: Option<f64>,
    pub due_date: Option<DateTime<Utc>>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_until: Option<DateTime<Utc>>,
    pub submission_types: Vec<String>,
    pub canvas_id: String,
    pub topic_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Assignment {
    pub fn new(
        course_id: Uuid,
        title: String,
        description: Option<String>,
        points_possible: Option<f64>,
        due_date: Option<DateTime<Utc>>,
        available_from: Option<DateTime<Utc>>,
        available_until: Option<DateTime<Utc>>,
        submission_types: Vec<String>,
        canvas_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            course_id,
            title,
            description,
            points_possible,
            due_date,
            available_from,
            available_until,
            submission_types,
            canvas_id,
            topic_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_discussion_assignment(&self) -> bool {
        self.submission_types.contains(&"discussion_topic".to_string())
    }

    pub fn is_available_now(&self) -> bool {
        let now = Utc::now();
        
        let after_start = match self.available_from {
            Some(from) => from <= now,
            None => true,
        };
        
        let before_end = match self.available_until {
            Some(until) => until >= now,
            None => true,
        };
        
        after_start && before_end
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AssignmentType {
    Discussion,
    Quiz,
    Assignment,
    Project,
}