// src/api/auth.rs

use axum::{extract::State, Json};
use crate::app_state::AppState;
use std::collections::hash_map::Entry;
use uuid::Uuid;
use axum::{routing::post, Router, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
}

async fn register(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterRequest>,
) -> (StatusCode, Json<AuthResponse>) {
    // Validate input
    if req.password != req.password_confirmation {
        return (StatusCode::BAD_REQUEST, Json(AuthResponse {
            token: "".to_string(),
            user_id: "".to_string(),
        }));
    }
    if req.email.is_empty() || req.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(AuthResponse {
            token: "".to_string(),
            user_id: "".to_string(),
        }));
    }
    // Register user
    let mut users = state.users.lock().unwrap();
    match users.entry(req.email.clone()) {
        Entry::Occupied(_) => {
            // Email already registered
            (StatusCode::CONFLICT, Json(AuthResponse {
                token: "".to_string(),
                user_id: "".to_string(),
            }))
        }
        Entry::Vacant(entry) => {
            entry.insert(req.password.clone());
            let user_id = Uuid::new_v4().to_string();
            // Issue a dummy JWT (replace with real JWT in production)
            let token = format!("dummy.jwt.token.for.{}", req.email);
            (StatusCode::CREATED, Json(AuthResponse {
                token,
                user_id,
            }))
        }
    }
}

pub fn auth_routes(_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/register", post(register))
}
