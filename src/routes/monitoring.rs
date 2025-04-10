// src/routes/monitoring.rs
//! API routes for monitoring the synchronization process

use actix_web::{web, HttpResponse, Responder, get, Error};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::services::monitoring::sync_monitor::SyncMonitor;
use crate::services::integration::sync_state::SyncState;
use crate::services::integration::sync_transaction::SyncTransaction;
use crate::services::integration::sync_service::SyncService;

/// Query parameters for the pending items endpoint
#[derive(Deserialize)]
pub struct PendingItemsQuery {
    limit: Option<usize>,
}

/// Configure monitoring routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    // Create shared monitoring services
    let sync_state = web::Data::new(SyncState::new());
    let sync_monitor = web::Data::new(SyncMonitor::new(sync_state.clone()));
    
    cfg.app_data(sync_state.clone())
       .app_data(sync_monitor.clone())
       .service(get_sync_stats)
       .service(get_pending_items)
       .service(get_entity_history)
       .service(get_failed_transactions)
       .service(get_dashboard_data);
}

/// Get synchronization statistics
#[get("/sync-stats")]
async fn get_sync_stats(
    sync_monitor: web::Data<SyncMonitor>,
) -> Result<impl Responder, Error> {
    match sync_monitor.get_statistics().await {
        Ok(stats) => Ok(HttpResponse::Ok().json(stats)),
        Err(e) => {
            log::error!("Error getting sync stats: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve synchronization statistics"
            })))
        }
    }
}

/// Get pending items that need attention
#[get("/pending-items")]
async fn get_pending_items(
    sync_monitor: web::Data<SyncMonitor>,
    query: web::Query<PendingItemsQuery>,
) -> Result<impl Responder, Error> {
    let limit = query.limit.unwrap_or(100);
    
    match sync_monitor.get_pending_items(limit).await {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(e) => {
            log::error!("Error getting pending items: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve pending items"
            })))
        }
    }
}

/// Get entity synchronization history
#[get("/entity-history/{entity_type}/{entity_id}")]
async fn get_entity_history(
    sync_monitor: web::Data<SyncMonitor>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, Error> {
    let (entity_type, entity_id) = path.into_inner();
    
    match sync_monitor.get_entity_history(&entity_type, &entity_id).await {
        Ok(history) => Ok(HttpResponse::Ok().json(history)),
        Err(e) => {
            log::error!("Error getting entity history: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve entity history"
            })))
        }
    }
}

/// Get failed transactions
#[get("/failed-transactions")]
async fn get_failed_transactions(
    sync_monitor: web::Data<SyncMonitor>,
) -> Result<impl Responder, Error> {
    match sync_monitor.get_failed_transactions().await {
        Ok(transactions) => Ok(HttpResponse::Ok().json(transactions)),
        Err(e) => {
            log::error!("Error getting failed transactions: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve failed transactions"
            })))
        }
    }
}

/// Get dashboard data
#[get("/dashboard-data")]
async fn get_dashboard_data(
    sync_monitor: web::Data<SyncMonitor>,
) -> Result<impl Responder, Error> {
    match sync_monitor.get_dashboard_data().await {
        Ok(data) => Ok(HttpResponse::Ok().json(data)),
        Err(e) => {
            log::error!("Error getting dashboard data: {}", e);
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to retrieve dashboard data"
            })))
        }
    }
}
