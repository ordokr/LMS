use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};

use crate::middleware::auth_middleware::require_auth;
use crate::services::notification_service::{self, NotificationOptions};

/// Query parameters for notifications
#[derive(Debug, Deserialize)]
struct NotificationsQuery {
    read: Option<bool>,
    #[serde(rename = "type")]
    notification_type: Option<String>,
    since: Option<String>,
    limit: Option<usize>,
}

/// Request for marking a notification as read
#[derive(Debug, Deserialize)]
struct MarkAsReadRequest {
    source: String,
}

/// Get notifications for the authenticated user
async fn get_user_notifications(
    query: web::Query<NotificationsQuery>,
    user_info: web::ReqData<crate::middleware::auth_middleware::UserInfo>,
) -> Result<impl Responder, Error> {
    let options = NotificationOptions {
        read: query.read.unwrap_or(false),
        notification_type: query.notification_type.clone(),
        since: query.since.clone(),
        limit: query.limit.unwrap_or(20),
    };
    
    match notification_service::get_user_notifications(&user_info.id, &options).await {
        Ok(notifications) => Ok(HttpResponse::Ok().json(notifications)),
        Err(e) => {
            log::error!("Error fetching notifications: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

/// Mark a notification as read
async fn mark_notification_as_read(
    path: web::Path<String>,
    req: web::Json<MarkAsReadRequest>,
    user_info: web::ReqData<crate::middleware::auth_middleware::UserInfo>,
) -> Result<impl Responder, Error> {
    let id = path.into_inner();
    
    // Validate source parameter
    if req.source.is_empty() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Source parameter is required"
        })));
    }
    
    match notification_service::mark_as_read(&id, &req.source).await {
        Ok(notification) => Ok(HttpResponse::Ok().json(notification)),
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.to_string()
            })))
        }
    }
}

/// Configure notification routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .wrap(require_auth())
            .route("", web::get().to(get_user_notifications))
            .route("/{id}/read", web::post().to(mark_notification_as_read))
    );
}
