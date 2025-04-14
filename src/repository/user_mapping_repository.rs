use sqlx::{Pool, Postgres};
use crate::models::notification::UserMapping;
use crate::error::AppError;

#[derive(Clone)]
pub struct UserMappingRepository {
    pool: Pool<Postgres>,
}

impl UserMappingRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
        }
    }

    // Add functions for creating, retrieving, updating, and deleting user mappings
}


