use std::sync::Once;
use sqlx::{SqlitePool, migrate::MigrateDatabase, Sqlite};
use tokio::runtime::Runtime;

// Path to test database
pub const TEST_DB_URL: &str = "sqlite::memory:";

static INIT: Once = Once::new();

// Initialize test environment
pub fn setup() -> SqlitePool {
    INIT.call_once(|| {
        // Set up test environment
        std::env::set_var("RUST_BACKTRACE", "1");
        
        // Initialize logging for tests
        env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .init();
    });
    
    // Create a new runtime for async tests
    let rt = Runtime::new().unwrap();
    
    // Set up test database
    rt.block_on(async {
        if !Sqlite::database_exists(TEST_DB_URL).await.unwrap_or(false) {
            Sqlite::create_database(TEST_DB_URL).await.unwrap();
        }
        
        let pool = SqlitePool::connect(TEST_DB_URL).await.unwrap();
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
            
        pool
    })
}

// Helper to create a test course
pub async fn create_test_course(pool: &SqlitePool) -> String {
    let course_id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query!(
        r#"
        INSERT INTO courses (id, name, code, start_date, end_date)
        VALUES (?, ?, ?, ?, ?)
        "#,
        course_id,
        "Test Course",
        "TEST-101",
        "2025-01-01",
        "2025-06-01"
    )
    .execute(pool)
    .await
    .expect("Failed to create test course");
    
    course_id
}

// Helper to create a test module
pub async fn create_test_module(pool: &SqlitePool, course_id: &str) -> String {
    let module_id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        INSERT INTO modules (id, course_id, name, position, published, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
        module_id,
        course_id,
        "Test Module",
        1,
        true,
        now,
        now
    )
    .execute(pool)
    .await
    .expect("Failed to create test module");
    
    module_id
}