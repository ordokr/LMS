use sqlx::PgPool;
use crate::models::user::User;
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await
        .ok()?
    }

    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, username, email, password_hash, role, canvas_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.username,
            user.email,
            user.password_hash,
            user.role,
            user.canvas_id,
            user.created_at,
            user.updated_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn check_user_exists(&self, user_id: &str) -> bool {
        let uuid = match Uuid::parse_str(user_id) {
            Ok(id) => id,
            Err(_) => return false,
        };

        sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE id = $1) as "exists!"
            "#,
            uuid
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| row.exists)
        .unwrap_or(false)
    }
}