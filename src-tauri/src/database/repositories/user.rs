use sqlx::{Pool, Sqlite};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;

use crate::core::errors::AppError;
use crate::shared::models::user::{User, UserRole, UserProfile, RegisterRequest, LoginRequest};

pub struct UserRepository {
    db: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(db: Pool<Sqlite>) -> Self {
        Self { db }
    }
    
    pub async fn create_user(&self, request: RegisterRequest) -> Result<i64, AppError> {
        // Check if user with the same email already exists
        let existing_user = sqlx::query!(
            "SELECT id FROM users WHERE email = ?",
            request.email
        )
        .fetch_optional(&self.db)
        .await?;
        
        if existing_user.is_some() {
            return Err(AppError::ValidationError("Email already registered".to_string()));
        }
        
        // Hash the password
        let password_hash = hash(request.password, DEFAULT_COST)
            .map_err(|e| AppError::ServerError(format!("Failed to hash password: {}", e)))?;
        
        // Insert new user
        let user_id = sqlx::query!(
            r#"
            INSERT INTO users (name, email, password_hash)
            VALUES (?, ?, ?)
            RETURNING id
            "#,
            request.name,
            request.email,
            password_hash
        )
        .fetch_one(&self.db)
        .await?
        .id;
        
        // Add default student role
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role, context_type)
            VALUES (?, ?, ?)
            "#,
            user_id,
            "student",
            "system"
        )
        .execute(&self.db)
        .await?;
        
        // Initialize forum trust level
        sqlx::query!(
            r#"
            INSERT INTO forum_trust_levels (user_id, trust_level)
            VALUES (?, ?)
            "#,
            user_id,
            0 // New users start at trust level 0
        )
        .execute(&self.db)
        .await?;
        
        Ok(user_id)
    }
    
    pub async fn authenticate_user(&self, login: LoginRequest) -> Result<UserProfile, AppError> {
        // Find user by email
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password_hash, avatar_url, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
            login.email
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::AuthError("Invalid email or password".to_string()))?;
        
        // Verify password
        let password_hash = user.password_hash.as_ref()
            .ok_or_else(|| AppError::ServerError("User password hash is missing".to_string()))?;
            
        let password_valid = verify(login.password, password_hash)
            .map_err(|e| AppError::ServerError(format!("Password verification failed: {}", e)))?;
            
        if !password_valid {
            return Err(AppError::AuthError("Invalid email or password".to_string()));
        }
        
        // Get user roles
        let roles = self.get_user_roles(user.id.unwrap()).await?;
        
        // Get forum trust level
        let trust_level = self.get_user_trust_level(user.id.unwrap()).await?;
        
        Ok(UserProfile {
            user,
            roles,
            forum_trust_level: Some(trust_level),
        })
    }
    
    pub async fn get_user_by_id(&self, user_id: i64) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password_hash, avatar_url, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", user_id)))?;
        
        Ok(user)
    }
    
    pub async fn get_user_profile(&self, user_id: i64) -> Result<UserProfile, AppError> {
        let user = self.get_user_by_id(user_id).await?;
        let roles = self.get_user_roles(user_id).await?;
        let trust_level = self.get_user_trust_level(user_id).await?;
        
        Ok(UserProfile {
            user,
            roles,
            forum_trust_level: Some(trust_level),
        })
    }
    
    async fn get_user_roles(&self, user_id: i64) -> Result<Vec<UserRole>, AppError> {
        let roles = sqlx::query_as!(
            UserRole,
            r#"
            SELECT id, user_id, role, context_type, context_id
            FROM user_roles
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_all(&self.db)
        .await?;
        
        Ok(roles)
    }
    
    async fn get_user_trust_level(&self, user_id: i64) -> Result<i32, AppError> {
        let trust_level = sqlx::query!(
            r#"
            SELECT trust_level
            FROM forum_trust_levels
            WHERE user_id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.db)
        .await?
        .map(|row| row.trust_level)
        .unwrap_or(0); // Default to trust level 0 if not found
        
        Ok(trust_level)
    }
    
    pub async fn update_user_profile(&self, user_id: i64, name: &str, avatar_url: Option<&str>) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET name = ?, avatar_url = ?, updated_at = CURRENT_TIMESTAMP
            WHERE id = ?
            "#,
            name,
            avatar_url,
            user_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
    
    pub async fn add_user_role(&self, user_id: i64, role: &str, context_type: Option<&str>, context_id: Option<i64>) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO user_roles (user_id, role, context_type, context_id)
            VALUES (?, ?, ?, ?)
            ON CONFLICT (user_id, role, context_type, context_id) DO NOTHING
            "#,
            user_id,
            role,
            context_type,
            context_id
        )
        .execute(&self.db)
        .await?;
        
        Ok(())
    }
}