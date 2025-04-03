use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: UserRole,
    pub created_at: Option<String>,
    pub last_login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    Student,
    Teacher,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Student
    }
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            username,
            email,
            display_name: None,
            password_hash: Some(password_hash),
            avatar_url: None,
            bio: None,
            role: UserRole::default(),
            created_at: Some(now.to_rfc3339()),
            last_login: None,
        }
    }
    
    pub fn is_active(&self) -> bool {
        !self.is_deleted && 
        !self.is_suspended || 
        self.suspended_until
            .map(|date| date < Utc::now())
            .unwrap_or(false)
    }
    
    pub fn display_name(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.username)
    }
}