mod auth;
mod courses;
mod users;
mod forum;
mod integration;
mod discussion_routes;

use axum::{Router, routing::get};
use std::sync::Arc;
use crate::db::AppState;
use reqwest::{Client, header};
use std::sync::OnceLock;
use std::time::Duration;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/auth", auth::auth_routes())
        .nest("/api/courses", courses::course_routes())
        .nest("/api/users", users::user_routes())
        .nest("/api/forum", forum::forum_routes())
        .route("/api/health", get(health_check))
        .route(
            "/api/courses/:id/category",
            get(integration::get_course_category)
                .post(integration::get_or_create_course_category)
        )
        .route(
            "/api/courses/:course_id/modules/:module_id/discussion",
            get(integration::get_module_topic)
                .post(integration::create_module_discussion)
        )
        .route(
            "/api/courses/:course_id/assignments/:assignment_id/discussion",
            get(integration::get_assignment_topic)
                .post(integration::create_assignment_discussion)
        )
        .route(
            "/api/courses/:id/forum/activity",
            get(integration::get_course_forum_activity)
        )
        .merge(discussion_routes::discussion_routes())
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

pub mod forum;
pub mod courses;

use axum::Router;
use std::sync::Arc;
use crate::AppState;

pub fn api_router() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/forum", forum::forum_routes())
        .nest("/lms", courses::course_routes())
}

// Organize routes by domain with middleware layers
pub fn forum_router() -> Router<Arc<AppState>> {
    Router::new()
        // Category routes
        .route("/categories", get(get_categories).post(create_category))
        .route("/categories/:id", get(get_category).put(update_category).delete(delete_category))
        // Topic routes
        .route("/topics", get(get_topics).post(create_topic))
        .route("/topics/:id", get(get_topic).put(update_topic).delete(delete_topic))
        // Common middleware for auth
        .layer(middleware::from_fn(verify_auth))
}

// Singleton HTTP client with connection pooling
pub fn get_http_client() -> &'static Client {
    static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
    
    HTTP_CLIENT.get_or_init(|| {
        // Create a client with optimized connection pooling
        Client::builder()
            .timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(60))
            .tcp_keepalive(Duration::from_secs(60))
            .gzip(true)
            .brotli(true)
            .build()
            .expect("Failed to create HTTP client")
    })
}