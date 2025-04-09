use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

use crate::models::forum::topic::Topic;
use crate::models::forum::post::Post;
use crate::models::content::submission::Submission;
use crate::models::course::enrollment::Enrollment;
use crate::db::DB;
use crate::error::Error;

/// User model that harmonizes Canvas and Discourse user models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    
    // Canvas-specific fields
    pub canvas_user_id: Option<String>,
    pub sis_user_id: Option<String>,
    pub lti_user_id: Option<String>,
    pub sortable_name: Option<String>,
    pub short_name: Option<String>,
    
    // Discourse-specific fields
    pub discourse_user_id: Option<i64>,
    pub trust_level: Option<i32>,
    pub post_count: Option<i32>,
}

impl User {
    pub fn new(name: String, email: String, username: String) -> Self {
        User {
            id: Uuid::new_v4(),
            name,
            email,
            username,
            avatar_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_seen_at: None,
            canvas_user_id: None,
            sis_user_id: None,
            lti_user_id: None,
            sortable_name: None,
            short_name: None,
            discourse_user_id: None,
            trust_level: None,
            post_count: None,
        }
    }

    // Database operations
    
    pub async fn find(db: &DB, id: Uuid) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = ?"
        )
        .bind(id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_email(db: &DB, email: &str) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = ?"
        )
        .bind(email)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_canvas_id(db: &DB, canvas_id: &str) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE canvas_user_id = ?"
        )
        .bind(canvas_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_discourse_id(db: &DB, discourse_id: i64) -> Result<Self, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE discourse_user_id = ?"
        )
        .bind(discourse_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn create(db: &DB, user: &User) -> Result<Uuid, Error> {
        let id = sqlx::query(
            "INSERT INTO users 
            (id, name, email, username, avatar_url, created_at, updated_at, 
            canvas_user_id, sis_user_id, lti_user_id, sortable_name, short_name,
            discourse_user_id, trust_level, post_count)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.avatar_url)
        .bind(user.created_at)
        .bind(user.updated_at)
        .bind(&user.canvas_user_id)
        .bind(&user.sis_user_id)
        .bind(&user.lti_user_id)
        .bind(&user.sortable_name)
        .bind(&user.short_name)
        .bind(user.discourse_user_id)
        .bind(user.trust_level)
        .bind(user.post_count)
        .execute(&db.pool)
        .await?;
        
        Ok(user.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        sqlx::query(
            "UPDATE users SET 
            name = ?, email = ?, username = ?, avatar_url = ?, updated_at = ?,
            canvas_user_id = ?, sis_user_id = ?, lti_user_id = ?, 
            sortable_name = ?, short_name = ?, discourse_user_id = ?,
            trust_level = ?, post_count = ?, last_seen_at = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.email)
        .bind(&self.username)
        .bind(&self.avatar_url)
        .bind(Utc::now())
        .bind(&self.canvas_user_id)
        .bind(&self.sis_user_id)
        .bind(&self.lti_user_id)
        .bind(&self.sortable_name)
        .bind(&self.short_name)
        .bind(self.discourse_user_id)
        .bind(self.trust_level)
        .bind(self.post_count)
        .bind(self.last_seen_at)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete(db: &DB, id: Uuid) -> Result<(), Error> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&db.pool)
            .await?;
            
        Ok(())
    }
    
    // Relationship methods
    
    pub async fn enrollments(&self, db: &DB) -> Result<Vec<Enrollment>, Error> {
        Enrollment::find_by_user_id(db, self.id).await
    }
    
    pub async fn topics(&self, db: &DB) -> Result<Vec<Topic>, Error> {
        Topic::find_by_author_id(db, self.id).await
    }
    
    pub async fn posts(&self, db: &DB) -> Result<Vec<Post>, Error> {
        Post::find_by_author_id(db, self.id).await
    }
    
    pub async fn submissions(&self, db: &DB) -> Result<Vec<Submission>, Error> {
        Submission::find_by_user_id(db, self.id).await
    }
}