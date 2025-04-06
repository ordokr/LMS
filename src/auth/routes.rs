use axum::{
    middleware::from_fn_with_state,
    routing::{post, get},
    Router,
    extract::State,
};
use std::sync::Arc;

use super::{handlers, middleware};
use crate::AppState;

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/refresh", post(handlers::refresh_token))
        .route("/api/auth/verify", post(handlers::verify))
        .route(
            "/api/auth/protected-example",
            get(|| async { "This is a protected route" })
                .route_layer(from_fn_with_state(state.clone(), middleware::auth_middleware))
        )
        .with_state(state)
}