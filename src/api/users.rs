use axum::{Router, routing::{get}, extract::{State, Path, Json}, http::StatusCode};
use serde::Serialize;
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteUserRepository, UserRepository, UserRow};

#[derive(Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
}

async fn list_users(State(pool): State<Arc<SqlitePool>>) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let repo = SqliteUserRepository { pool: &pool };
    match repo.list_users().await {
        Ok(users) => Ok(Json(users.into_iter().map(|u| UserResponse {
            id: u.id,
            email: u.email,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(Path(id): Path<i64>, State(pool): State<Arc<SqlitePool>>) -> Result<Json<UserResponse>, StatusCode> {
    let repo = SqliteUserRepository { pool: &pool };
    match repo.get_user(id).await {
        Ok(Some(u)) => Ok(Json(UserResponse {
            id: u.id,
            email: u.email,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn user_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/", get(list_users))
        .route("/:id", get(get_user))
        .with_state(pool)
}
