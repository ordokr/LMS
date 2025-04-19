use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::services::{ErrorHandlingService, ErrorRecord, ErrorSeverity, ErrorCategory, RecoveryService};
use crate::AppState;

/// Error record response
#[derive(Debug, Serialize)]
pub struct ErrorRecordResponse {
    pub id: String,
    pub message: String,
    pub severity: String,
    pub category: String,
    pub source: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub details: Option<String>,
    pub timestamp: String,
    pub resolved: bool,
    pub resolved_at: Option<String>,
    pub resolution: Option<String>,
    pub retry_count: u32,
    pub max_retries: u32,
    pub retriable: bool,
    pub next_retry: Option<String>,
}

impl From<ErrorRecord> for ErrorRecordResponse {
    fn from(record: ErrorRecord) -> Self {
        Self {
            id: record.id.to_string(),
            message: record.message,
            severity: record.severity.to_string(),
            category: record.category.to_string(),
            source: record.source,
            entity_type: record.entity_type,
            entity_id: record.entity_id,
            details: record.details,
            timestamp: record.timestamp.to_rfc3339(),
            resolved: record.resolved,
            resolved_at: record.resolved_at.map(|dt| dt.to_rfc3339()),
            resolution: record.resolution,
            retry_count: record.retry_count,
            max_retries: record.max_retries,
            retriable: record.retriable,
            next_retry: record.next_retry.map(|dt| dt.to_rfc3339()),
        }
    }
}

/// Error filter request
#[derive(Debug, Deserialize)]
pub struct ErrorFilterRequest {
    pub severity: Option<String>,
    pub category: Option<String>,
    pub source: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub resolved: Option<bool>,
    pub retriable: Option<bool>,
}

/// Error resolution request
#[derive(Debug, Deserialize)]
pub struct ErrorResolutionRequest {
    pub resolution: String,
}

/// Get all errors
async fn get_all_errors(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ErrorRecordResponse>> {
    let errors = state.recovery_service.get_all_errors().await;
    Json(errors.into_iter().map(ErrorRecordResponse::from).collect())
}

/// Get unresolved errors
async fn get_unresolved_errors(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<ErrorRecordResponse>> {
    let errors = state.recovery_service.get_unresolved_errors().await;
    Json(errors.into_iter().map(ErrorRecordResponse::from).collect())
}

/// Get errors by filter
async fn get_errors_by_filter(
    State(state): State<Arc<AppState>>,
    Json(filter): Json<ErrorFilterRequest>,
) -> Json<Vec<ErrorRecordResponse>> {
    let mut errors = state.recovery_service.get_all_errors().await;
    
    // Apply filters
    if let Some(severity) = filter.severity {
        let severity = match severity.as_str() {
            "Critical" => Some(ErrorSeverity::Critical),
            "Error" => Some(ErrorSeverity::Error),
            "Warning" => Some(ErrorSeverity::Warning),
            "Info" => Some(ErrorSeverity::Info),
            _ => None,
        };
        
        if let Some(severity) = severity {
            errors = errors.into_iter().filter(|e| e.severity == severity).collect();
        }
    }
    
    if let Some(category) = filter.category {
        let category = match category.as_str() {
            "ApiConnection" => Some(ErrorCategory::ApiConnection),
            "Authentication" => Some(ErrorCategory::Authentication),
            "Authorization" => Some(ErrorCategory::Authorization),
            "Validation" => Some(ErrorCategory::Validation),
            "Synchronization" => Some(ErrorCategory::Synchronization),
            "Database" => Some(ErrorCategory::Database),
            "Configuration" => Some(ErrorCategory::Configuration),
            "System" => Some(ErrorCategory::System),
            "Unknown" => Some(ErrorCategory::Unknown),
            _ => None,
        };
        
        if let Some(category) = category {
            errors = errors.into_iter().filter(|e| e.category == category).collect();
        }
    }
    
    if let Some(source) = filter.source {
        errors = errors.into_iter().filter(|e| e.source == source).collect();
    }
    
    if let Some(entity_type) = filter.entity_type {
        errors = errors.into_iter().filter(|e| e.entity_type.as_deref() == Some(&entity_type)).collect();
    }
    
    if let Some(entity_id) = filter.entity_id {
        errors = errors.into_iter().filter(|e| e.entity_id.as_deref() == Some(&entity_id)).collect();
    }
    
    if let Some(resolved) = filter.resolved {
        errors = errors.into_iter().filter(|e| e.resolved == resolved).collect();
    }
    
    if let Some(retriable) = filter.retriable {
        errors = errors.into_iter().filter(|e| e.retriable == retriable).collect();
    }
    
    Json(errors.into_iter().map(ErrorRecordResponse::from).collect())
}

/// Get error by ID
async fn get_error_by_id(
    State(state): State<Arc<AppState>>,
    Path(error_id): Path<String>,
) -> Json<Option<ErrorRecordResponse>> {
    let errors = state.recovery_service.get_all_errors().await;
    
    let uuid = match Uuid::parse_str(&error_id) {
        Ok(uuid) => uuid,
        Err(_) => return Json(None),
    };
    
    let error = errors.into_iter().find(|e| e.id == uuid);
    
    Json(error.map(ErrorRecordResponse::from))
}

/// Resolve an error
async fn resolve_error(
    State(state): State<Arc<AppState>>,
    Path(error_id): Path<String>,
    Json(request): Json<ErrorResolutionRequest>,
) -> Json<bool> {
    let uuid = match Uuid::parse_str(&error_id) {
        Ok(uuid) => uuid,
        Err(_) => return Json(false),
    };
    
    match state.recovery_service.manually_resolve_error(uuid, request.resolution).await {
        Ok(_) => Json(true),
        Err(_) => Json(false),
    }
}

/// Retry an error
async fn retry_error(
    State(state): State<Arc<AppState>>,
    Path(error_id): Path<String>,
) -> Json<bool> {
    let uuid = match Uuid::parse_str(&error_id) {
        Ok(uuid) => uuid,
        Err(_) => return Json(false),
    };
    
    match state.error_service.retry_error(uuid).await {
        Ok(retried) => Json(retried),
        Err(_) => Json(false),
    }
}

/// Clear resolved errors
async fn clear_resolved_errors(
    State(state): State<Arc<AppState>>,
) -> Json<usize> {
    match state.recovery_service.clear_resolved_errors().await {
        Ok(count) => Json(count),
        Err(_) => Json(0),
    }
}

/// Create the error API routes
pub fn error_api_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/errors", get(get_all_errors))
        .route("/errors/unresolved", get(get_unresolved_errors))
        .route("/errors/filter", post(get_errors_by_filter))
        .route("/errors/:id", get(get_error_by_id))
        .route("/errors/:id/resolve", post(resolve_error))
        .route("/errors/:id/retry", post(retry_error))
        .route("/errors/clear-resolved", delete(clear_resolved_errors))
        .with_state(state)
}
