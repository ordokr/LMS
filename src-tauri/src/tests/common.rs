use sqlx::{Pool, Sqlite, SqlitePool};

pub struct TestContext {
    pub db_pool: SqlitePool,
}

impl TestContext {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // Create an in-memory database for testing
        let pool = SqlitePool::connect("sqlite::memory:").await?;
        
        // Run migrations to set up schema
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        Ok(TestContext {
            db_pool: pool,
        })
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        // Cleanup will happen automatically when the pool is dropped
    }
}