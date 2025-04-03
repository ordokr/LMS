use crate::models::User;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result, Row};

pub struct UserRepository<'a> {
    conn: &'a Connection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }
    
    // Convert a database row to a User model
    fn row_to_user(row: &Row) -> Result<User> {
        Ok(User {
            id: Some(row.get(0)?),
            username: row.get(1)?,
            email: row.get(2)?,
            display_name: row.get(3)?,
            password_hash: row.get(4)?,
            avatar_url: row.get(5)?,
            bio: row.get(6)?,
            website: row.get(7)?,
            location: row.get(8)?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(9, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(10)?)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(10, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc),
            last_seen_at: row.get::<_, Option<String>>(11)?
                .map(|dt| DateTime::parse_from_rfc3339(&dt)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(11, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            trust_level: row.get(12)?,
            is_admin: row.get(13)?,
            is_moderator: row.get(14)?,
            is_suspended: row.get(15)?,
            suspended_until: row.get::<_, Option<String>>(16)?
                .map(|dt| DateTime::parse_from_rfc3339(&dt)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(16, rusqlite::types::Type::Text, Box::new(e)))
                    .map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            is_deleted: row.get(17)?,
        })
    }

    // Create a new user
    pub fn create(&self, user: &User) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO users 
            (username, email, display_name, password_hash, avatar_url, bio, website, location, 
            created_at, updated_at, last_seen_at, trust_level, is_admin, is_moderator, 
            is_suspended, suspended_until, is_deleted) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            params![
                user.username, user.email, user.display_name, user.password_hash, user.avatar_url, 
                user.bio, user.website, user.location, user.created_at.to_rfc3339(), user.updated_at.to_rfc3339(), 
                user.last_seen_at.map(|dt| dt.to_rfc3339()), user.trust_level, user.is_admin, 
                user.is_moderator, user.is_suspended, user.suspended_until.map(|dt| dt.to_rfc3339()), 
                user.is_deleted
            ],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }

    // Get a user by ID
    pub fn find_by_id(&self, id: i64) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM users WHERE id = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([id])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_user(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Get a user by username
    pub fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM users WHERE username = ? AND is_deleted = 0"
        )?;
        
        let mut rows = stmt.query([username])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(Self::row_to_user(row)?))
        } else {
            Ok(None)
        }
    }
    
    // Update a user
    pub fn update(&self, user: &User) -> Result<()> {
        if user.id.is_none() {
            return Err(rusqlite::Error::InvalidParameterName("User ID is required for update".to_string()));
        }
        
        self.conn.execute(
            "UPDATE users SET 
            username = ?1, email = ?2, display_name = ?3, avatar_url = ?4, 
            bio = ?5, website = ?6, location = ?7, updated_at = ?8, 
            trust_level = ?9, is_admin = ?10, is_moderator = ?11, 
            is_suspended = ?12, suspended_until = ?13 
            WHERE id = ?14",
            params![
                user.username, user.email, user.display_name, user.avatar_url, 
                user.bio, user.website, user.location, Utc::now().to_rfc3339(), 
                user.trust_level, user.is_admin, user.is_moderator, 
                user.is_suspended, user.suspended_until.map(|dt| dt.to_rfc3339()), 
                user.id
            ],
        )?;
        
        Ok(())
    }
    
    // Soft delete a user
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE users SET is_deleted = 1, updated_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
    
    // List active users
    pub fn list_active(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT * FROM users 
             WHERE is_deleted = 0 
             ORDER BY created_at DESC 
             LIMIT ? OFFSET ?"
        )?;
        
        let rows = stmt.query_map(params![limit, offset], |row| {
            Self::row_to_user(row)
        })?;
        
        let mut users = Vec::new();
        for user_result in rows {
            users.push(user_result?);
        }
        
        Ok(users)
    }
    
    // Update last seen time for a user
    pub fn update_last_seen(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE users SET last_seen_at = ? WHERE id = ?",
            params![Utc::now().to_rfc3339(), id],
        )?;
        
        Ok(())
    }
}