use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use crate::db::DB;
use crate::error::Error;

/// Extended user profile information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub job_title: Option<String>,
    pub company: Option<String>,
    pub interests: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Social media links
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

impl Profile {
    pub fn new(user_id: Uuid) -> Self {
        Profile {
            id: Uuid::new_v4(),
            user_id,
            bio: None,
            location: None,
            website: None,
            job_title: None,
            company: None,
            interests: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            twitter: None,
            github: None,
            linkedin: None,
        }
    }

    // Database operations
    
    pub async fn find_by_user_id(db: &DB, user_id: Uuid) -> Result<Self, Error> {
        let profile = sqlx::query_as::<_, Profile>(
            "SELECT * FROM profiles WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(&db.pool)
        .await?;
        
        Ok(profile)
    }
    
    pub async fn create(db: &DB, profile: &Profile) -> Result<Uuid, Error> {
        let interests_json = serde_json::to_string(&profile.interests)?;
        
        sqlx::query(
            "INSERT INTO profiles 
            (id, user_id, bio, location, website, job_title, company, 
            interests, created_at, updated_at, twitter, github, linkedin)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(profile.id)
        .bind(profile.user_id)
        .bind(&profile.bio)
        .bind(&profile.location)
        .bind(&profile.website)
        .bind(&profile.job_title)
        .bind(&profile.company)
        .bind(&interests_json)
        .bind(profile.created_at)
        .bind(profile.updated_at)
        .bind(&profile.twitter)
        .bind(&profile.github)
        .bind(&profile.linkedin)
        .execute(&db.pool)
        .await?;
        
        Ok(profile.id)
    }
    
    pub async fn update(&self, db: &DB) -> Result<(), Error> {
        let interests_json = serde_json::to_string(&self.interests)?;
        
        sqlx::query(
            "UPDATE profiles SET
            bio = ?, location = ?, website = ?, job_title = ?, company = ?,
            interests = ?, updated_at = ?, twitter = ?, github = ?, linkedin = ?
            WHERE id = ?"
        )
        .bind(&self.bio)
        .bind(&self.location)
        .bind(&self.website)
        .bind(&self.job_title)
        .bind(&self.company)
        .bind(&interests_json)
        .bind(Utc::now())
        .bind(&self.twitter)
        .bind(&self.github)
        .bind(&self.linkedin)
        .bind(self.id)
        .execute(&db.pool)
        .await?;
        
        Ok(())
    }
}