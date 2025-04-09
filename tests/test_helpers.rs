use axum::Router;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use crate::app_state::AppState;
use crate::api::integration::integration_routes;

pub async fn setup_test_app() -> (Router, Pool<Postgres>) {
    // Use a separate test database or a unique schema
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must be set for integration tests");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations on test database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations on test database");
    
    // Create app state
    let app_state = AppState::new(pool.clone());
    
    // Set up router with integration routes
    let app = Router::new()
        .merge(integration_routes())
        .with_state(app_state);
    
    (app, pool)
}