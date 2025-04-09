use crate::models::user::User;
use sqlx::{SqlitePool, Error};

pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }
    
    pub async fn find_all(&self) -> Result<Vec<User>, Error> {
        sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }
    
    pub async fn create(&self, user: &User) -> Result<i64, Error> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES (?, ?, ?)
            "#,
            user.username,
            user.email,
            user.password_hash
        )
        .execute(&self.pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }
}