// Define the User struct and related functionality

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password_hash: String, created_at: chrono::NaiveDateTime) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
            created_at,
            updated_at: None,
        }
    }
}
