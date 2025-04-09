use axum::{
    middleware::from_fn_with_state,
    routing::{post, get},
    Router,
    extract::State,
};
use std::sync::Arc;

use super::{handlers, middleware, refresh_handler, sso_handler};
use crate::AppState;

pub fn auth_routes(state: Arc<AppState>) -> Router {
    Router::new()
        // Basic authentication endpoints
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/refresh", post(refresh_handler::refresh_token))
        .route("/api/auth/revoke", post(refresh_handler::revoke_token))
        .route("/api/auth/verify", post(handlers::verify))
        
        // SSO integration endpoints
        .route("/api/auth/sso/generate", post(sso_handler::generate_sso_url))
        .route("/api/auth/sso/callback", get(sso_handler::sso_callback))
        .route("/api/auth/sso/verify", post(sso_handler::verify_discourse_sso))
        
        // Example protected route
        .route(
            "/api/auth/protected-example",
            get(|| async { "This is a protected route" })
                .route_layer(from_fn_with_state(state.clone(), middleware::auth_middleware))
        )
        .with_state(state)
}