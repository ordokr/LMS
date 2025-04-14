use sqlx::sqlite::{Sqlite, SqlitePoolOptions};
use sqlx::{Error, SqlitePool};
use std::env;

pub type Db = SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Topic {
    pub id: i64,
    pub category_id: i64,
    pub title: String,
    pub content: Option<String>,
    pub user_id: i64,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub topic_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn init_db() -> Result<Db, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqlitePoolOptions::new()
        .connect(&database_url)
        .await
}
