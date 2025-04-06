use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use sqlx::PgPool;
use std::sync::Arc;
use tower::ServiceExt;

use lms::{
    AppState,
    controllers::auth_controller::{LoginRequest, LoginResponse},
    db::user_repository::UserRepository,
    setup_app,
};

async fn setup_test_app() -> (axum::Router, PgPool) {
    // Use test database URL
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/lms_test".to_string());
    
    // Set up test JWT secret
    std::env::set_var("JWT_SECRET", "test_jwt_secret_key");
    
    // Create test database pool
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Run migrations on test database
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    // Clear test database
    sqlx::query!("TRUNCATE TABLE users CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to truncate users table");
    
    // Create app state with test database
    let user_repo = UserRepository::new(pool.clone());
    let app_state = Arc::new(AppState {
        db: user_repo,
    });
    
    // Set up app with routes
    let app = setup_app(app_state);
    
    (app, pool)
}

#[tokio::test]
async fn test_register_and_login_flow() {
    let (app, _pool) = setup_test_app().await;
    
    // Test user registration
    let register_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "password": "password123",
                        "canvas_id": "canvas123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(register_response.status(), StatusCode::CREATED);
    
    // Test user login
    let login_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), StatusCode::OK);
    
    // Extract and validate login response body
    let body = login_response.into_body().collect().await.unwrap().to_bytes();
    let login_result: LoginResponse = serde_json::from_slice(&body).unwrap();
    
    assert!(!login_result.token.is_empty());
    assert_eq!(login_result.user.username, "testuser");
    assert_eq!(login_result.user.email, "test@example.com");
}

#[tokio::test]
async fn test_login_with_invalid_credentials() {
    let (app, _pool) = setup_test_app().await;
    
    // Register a test user first
    app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "email": "test@example.com",
                        "password": "password123",
                        "canvas_id": "canvas123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Test login with wrong password
    let login_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "password": "wrongpassword"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);
    
    // Test login with non-existent user
    let login_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "username": "nonexistentuser",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);
}