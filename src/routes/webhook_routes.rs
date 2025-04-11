use actix_web::{web, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::webhook_service;

/// Common webhook response format
#[derive(Debug, Serialize)]
struct WebhookResponse {
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

/// Handle Canvas webhook
async fn handle_canvas_webhook(
    payload: web::Json<Value>,
) -> Result<impl Responder, Error> {
    match webhook_service::handle_canvas_webhook(&payload).await {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(WebhookResponse {
                status: "success".to_string(),
                result: Some(result),
                message: None,
            }))
        },
        Err(e) => {
            log::error!("Error in Canvas webhook: {}", e);
            Ok(HttpResponse::InternalServerError().json(WebhookResponse {
                status: "error".to_string(),
                result: None,
                message: Some(e.to_string()),
            }))
        }
    }
}

/// Handle Discourse webhook
async fn handle_discourse_webhook(
    payload: web::Json<Value>,
) -> Result<impl Responder, Error> {
    match webhook_service::handle_discourse_webhook(&payload).await {
        Ok(result) => {
            Ok(HttpResponse::Ok().json(WebhookResponse {
                status: "success".to_string(),
                result: Some(result),
                message: None,
            }))
        },
        Err(e) => {
            log::error!("Error in Discourse webhook: {}", e);
            Ok(HttpResponse::InternalServerError().json(WebhookResponse {
                status: "error".to_string(),
                result: None,
                message: Some(e.to_string()),
            }))
        }
    }
}

/// Configure webhook routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/webhooks")
            .route("/canvas", web::post().to(handle_canvas_webhook))
            .route("/discourse", web::post().to(handle_discourse_webhook))
    );
}
