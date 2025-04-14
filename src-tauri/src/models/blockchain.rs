use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: String,
    pub user_id: String,
    pub course_id: String,
    pub issued_at: DateTime<Utc>,
    pub metadata: String, // Additional information about the certificate
}