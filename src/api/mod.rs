```rust
// src/api/mod.rs

pub mod auth;
pub mod courses;
pub mod assignments;
pub mod users;
pub mod submissions;
pub mod discussions;
pub mod notifications;
pub mod integration;
pub mod migration;
pub mod assignment_migration;
pub mod discussion_migration;
pub mod post_migration;
pub mod sync;
pub mod sync_status;

use axum::Router;
use std::sync::Arc;
use crate::AppState;

pub fn api_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/api/auth", auth::auth_routes(state.clone()))
        .nest("/api/courses", courses::course_routes(state.clone()))
        .nest("/api/assignments", assignments::assignment_routes(state.clone()))
        .nest("/api/users", users::user_routes(state.clone()))
        .nest("/api", submissions::submission_routes(state.clone()))
        .nest("/api", discussions::discussion_routes(state.clone()))
        .nest("/api", notifications::notification_routes(state.clone()))
        .nest("/api", integration::integration_routes(state.clone()))
        .nest("/api", migration::migration_routes(state.clone()))
        .nest("/api", assignment_migration::assignment_migration_routes(state.clone()))
        .nest("/api", discussion_migration::discussion_migration_routes(state.clone()))
        .nest("/api", post_migration::post_migration_routes(state.clone()))
        .nest("/api", sync::sync_routes(state.clone()))
        .nest("/api", sync_status::sync_status_routes(state.clone()))
}
```
