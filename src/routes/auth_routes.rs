use axum::{
    routing::{get, post},
    Router,
    extract::{Query, State},
    http::HeaderMap,
    middleware,
    Json,
};
use std::sync::Arc;

use crate::controllers::auth_controller::{AuthController, LoginRequest, SsoParams, AuthError};
use crate::middleware::auth_middleware::require_auth;

/// Create authentication routes
pub fn create_auth_routes(auth_controller: Arc<AuthController>) -> Router {
    // Create a router with the auth_controller as state
    Router::new()
        // POST /api/v1/auth/login
        .route(
            "/login",
            post(|State(auth_controller): State<Arc<AuthController>>, 
                 payload: Json<LoginRequest>| async move {
                match auth_controller.login(payload).await {
                    Ok(response) => response,
                    Err(err) => AuthController::handle_error(err),
                }
            }),
        )
        // GET /api/v1/auth/discourse-sso
        .route(
            "/discourse-sso",
            get(|State(auth_controller): State<Arc<AuthController>>,
                headers: HeaderMap,
                query: Query<SsoParams>| async move {
                match auth_controller.handle_discourse_sso(headers, query).await {
                    Ok(response) => response,
                    Err(err) => AuthController::handle_error(err),
                }
            })
            .route_layer(middleware::from_fn(require_auth)),
        )
}

/// Register auth routes with the main application
pub fn register_auth_routes(app: Router, auth_controller: Arc<AuthController>) -> Router {
    app.nest("/api/v1/auth", create_auth_routes(auth_controller))
}
