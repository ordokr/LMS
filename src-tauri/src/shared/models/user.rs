use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: i64,
    pub user_id: i64,
    pub role: String,
    pub context_type: String,
    pub context_id: Option<String>,
}

impl UserRole {
    pub fn to_string(&self) -> String {
        self.role.clone()
    }
    
    pub fn Admin() -> Self {
        Self {
            id: 0,
            user_id: 0,
            role: "admin".to_string(),
            context_type: "system".to_string(),
            context_id: None,
        }
    }
    
    pub fn Instructor() -> Self {
        Self {
            id: 0,
            user_id: 0,
            role: "instructor".to_string(),
            context_type: "system".to_string(),
            context_id: None,
        }
    }
    
    pub fn Student() -> Self {
        Self {
            id: 0,
            user_id: 0,
            role: "student".to_string(),
            context_type: "system".to_string(),
            context_id: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
    pub roles: Vec<UserRole>,
    pub trust_level: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub avatar_url: Option<String>,
}
