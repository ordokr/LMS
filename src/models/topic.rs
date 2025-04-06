use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub category_id: Uuid,
    pub author_id: Uuid,
    pub pinned: bool,
    pub closed: bool,
    pub post_count: i32,
    pub view_count: i32,
    pub assignment_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Topic {
    pub fn new(
        title: String,
        category_id: Uuid,
        author_id: Uuid,
        assignment_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        let slug = generate_slug(&title);
        
        Self {
            id: Uuid::new_v4(),
            title,
            slug,
            category_id,
            author_id,
            pinned: false,
            closed: false,
            post_count: 0,
            view_count: 0,
            assignment_id,
            created_at: now,
            updated_at: now,
        }
    }
}

fn generate_slug(title: &str) -> String {
    title.to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        .chars()
        .take(80)  // Limit slug length
        .collect()
}