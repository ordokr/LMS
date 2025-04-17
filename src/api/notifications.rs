use axum::{Router, routing::get, extract::{State, Path}, http::StatusCode, Json};
use serde::Serialize;
use std::sync::Arc;
use crate::db::sqlite::{SqlitePool, SqliteNotificationRepository, NotificationRepository, NotificationRow};

#[derive(Serialize)]
pub struct NotificationResponse {
    pub id: i64,
    pub user_id: i64,
    pub message: String,
}

async fn list_user_notifications(
    Path(user_id): Path<i64>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Vec<NotificationResponse>>, StatusCode> {
    let repo = SqliteNotificationRepository { pool: &pool };
    match repo.list_notifications(user_id).await {
        Ok(notifs) => Ok(Json(notifs.into_iter().map(|n| NotificationResponse {
            id: n.id,
            user_id: n.user_id,
            message: n.message,
        }).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn notification_routes(pool: Arc<SqlitePool>) -> Router {
    Router::new()
        .route("/users/:user_id/notifications", get(list_user_notifications))
        .with_state(pool)
}
