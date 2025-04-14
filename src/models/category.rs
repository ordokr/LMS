use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub course_id: Option<Uuid>,  // Link back to Canvas course
    pub position: i32,
}

impl Category {
    pub fn new(
        name: String,
        description: Option<String>,
        parent_id: Option<Uuid>,
        course_id: Option<Uuid>,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        let slug = generate_slug(&name);
        
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            description,
            parent_id,
            created_at: now,
            updated_at: now,
            course_id,
            position,
        }
    }
}

fn generate_slug(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}