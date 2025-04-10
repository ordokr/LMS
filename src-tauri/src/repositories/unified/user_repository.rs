use sqlx::{Pool, Sqlite};
use crate::models::unified::User;
use crate::core::errors::AppError;
use uuid::Uuid;

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    pub async fn find_by_id(&self, id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, name, email, username, avatar as "avatar", 
                canvas_id, discourse_id, last_login, source_system,
                roles as "roles: Vec<String>", metadata as "metadata: serde_json::Value"
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_canvas_id(&self, canvas_id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, name, email, username, avatar as "avatar", 
                canvas_id, discourse_id, last_login, source_system,
                roles as "roles: Vec<String>", metadata as "metadata: serde_json::Value"
            FROM users
            WHERE canvas_id = ?
            "#,
            canvas_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn find_by_discourse_id(&self, discourse_id: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, name, email, username, avatar as "avatar", 
                canvas_id, discourse_id, last_login, source_system,
                roles as "roles: Vec<String>", metadata as "metadata: serde_json::Value"
            FROM users
            WHERE discourse_id = ?
            "#,
            discourse_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    pub async fn create(&self, user: &User) -> Result<User, AppError> {
        let id = if user.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            user.id.clone()
        };
        
        let roles_json = serde_json::to_string(&user.roles)?;
        let metadata_json = serde_json::to_string(&user.metadata)?;
        
        sqlx::query!(
            r#"
            INSERT INTO users 
            (id, name, email, username, avatar, canvas_id, discourse_id, last_login, source_system, roles, metadata) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            id,
            user.name,
            user.email,
            user.username,
            user.avatar,
            user.canvas_id,
            user.discourse_id,
            user.last_login,
            user.source_system,
            roles_json,
            metadata_json
        )
        .execute(&self.pool)
        .await?;
        
        let mut created_user = user.clone();
        created_user.id = id;
        
        Ok(created_user)
    }
    
    pub async fn update(&self, user: &User) -> Result<User, AppError> {
        let roles_json = serde_json::to_string(&user.roles)?;
        let metadata_json = serde_json::to_string(&user.metadata)?;
        
        sqlx::query!(
            r#"
            UPDATE users 
            SET name = ?, email = ?, username = ?, avatar = ?, 
                canvas_id = ?, discourse_id = ?, last_login = ?, 
                source_system = ?, roles = ?, metadata = ?
            WHERE id = ?
            "#,
            user.name,
            user.email,
            user.username,
            user.avatar,
            user.canvas_id,
            user.discourse_id,
            user.last_login,
            user.source_system,
            roles_json,
            metadata_json,
            user.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(user.clone())
    }
    
    pub async fn delete(&self, id: &str) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
            
        Ok(())
    }
    
    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT 
                id, name, email, username, avatar as "avatar", 
                canvas_id, discourse_id, last_login, source_system,
                roles as "roles: Vec<String>", metadata as "metadata: serde_json::Value"
            FROM users
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(users)
    }
}