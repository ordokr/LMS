use crate::services::integration::sync_transaction::SyncTransaction;
use crate::services::integration::sync_state::SyncState;
use crate::services::integration::sync_service::SyncService;
use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router, Json, extract::{State, Path},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::{info, error, Level};
use socketio_server::{SocketIoServer, SocketIoBuilder};
use std::time::Duration;
use tokio::time;
use thiserror::Error;

/// Error types for the sync dashboard
#[derive(Debug, Error)]
pub enum DashboardError {
    #[error("Failed to start server: {0}")]
    ServerError(String),
    
    #[error("Sync state error: {0}")]
    SyncStateError(String),
    
    #[error("Sync transaction error: {0}")]
    SyncTransactionError(String),
    
    #[error("Sync service error: {0}")]
    SyncServiceError(String),
}

/// Dashboard application state
pub struct AppState {
    sync_state: Arc<SyncState>,
    sync_transaction: Arc<SyncTransaction>,
    sync_service: Arc<SyncService>,
    socket_io: Arc<SocketIoServer>,
}

/// Trigger sync request payload
#[derive(Debug, Deserialize)]
pub struct TriggerSyncRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub priority: Option<String>,
}

/// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// Sync Dashboard for monitoring and managing the sync process
pub struct SyncDashboard {
    port: u16,
    app_state: Arc<AppState>,
}

impl SyncDashboard {
    /// Create a new sync dashboard service
    pub fn new(
        port: u16,
        sync_state: Arc<SyncState>,
        sync_transaction: Arc<SyncTransaction>,
        sync_service: Arc<SyncService>,
    ) -> Self {
        // Initialize Socket.IO server
        let socket_io = Arc::new(SocketIoBuilder::new().build());
        
        let app_state = Arc::new(AppState {
            sync_state,
            sync_transaction,
            sync_service,
            socket_io: socket_io.clone(),
        });
        
        SyncDashboard {
            port,
            app_state,
        }
    }
    
    /// Initialize and start the dashboard service
    pub async fn start(&self) -> Result<(), DashboardError> {
        // Set up metrics collection
        self.setup_metrics_collection().await;
        
        // Set up the Axum router with our routes
        let app = self.create_router();
        
        // Start the server
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("Sync monitoring dashboard running on port {}", self.port);
        
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| DashboardError::ServerError(e.to_string()))?;
        
        axum::serve(listener, app)
            .await
            .map_err(|e| DashboardError::ServerError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Create the API router with all routes
    fn create_router(&self) -> Router {
        let api_router = Router::new()
            .route("/sync/status", get(Self::get_sync_status))
            .route("/sync/transactions", get(Self::get_recent_transactions))
            .route("/sync/entities/:type", get(Self::get_entities_by_type))
            .route("/sync/trigger", post(Self::trigger_sync));
        
        // Main router with static files and API routes
        Router::new()
            .nest("/api", api_router)
            .nest_service("/", ServeDir::new("public/dashboard"))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            )
            .with_state(self.app_state.clone())
    }
    
    /// Set up periodic metrics collection for real-time updates
    async fn setup_metrics_collection(&self) {
        let app_state = self.app_state.clone();
        
        // Spawn a background task to collect metrics periodically
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(10));
            
            loop {
                interval.tick().await;
                
                // Collect sync metrics
                match Self::collect_sync_metrics(&app_state).await {
                    Ok(metrics) => {
                        // Broadcast metrics to all connected clients
                        let _ = app_state.socket_io.emit("syncMetrics", &metrics);
                    },
                    Err(e) => {
                        error!("Failed to collect sync metrics: {}", e);
                    }
                }
            }
        });
    }
    
    /// Collect sync metrics for real-time updates
    async fn collect_sync_metrics(app_state: &AppState) -> Result<serde_json::Value, DashboardError> {
        // Get overall status
        let status = app_state.sync_state.get_overall_status()
            .await
            .map_err(|e| DashboardError::SyncStateError(e.to_string()))?;
        
        // Get recent transactions
        let transactions = app_state.sync_transaction.get_recent_transactions(10)
            .await
            .map_err(|e| DashboardError::SyncTransactionError(e.to_string()))?;
        
        // Combine into a metrics object
        let metrics = serde_json::json!({
            "status": status,
            "recentTransactions": transactions,
            "timestamp": chrono::Utc::now().timestamp(),
        });
        
        Ok(metrics)
    }
    
    // API handler functions
    
    /// Get the overall sync status
    async fn get_sync_status(
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        match state.sync_state.get_overall_status().await {
            Ok(status) => {
                let response = ApiResponse {
                    success: true,
                    message: None,
                    data: Some(status),
                    error: None,
                };
                (StatusCode::OK, Json(response))
            },
            Err(e) => {
                let response = ApiResponse::<serde_json::Value> {
                    success: false,
                    message: None,
                    data: None,
                    error: Some(e.to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }
    
    /// Get recent sync transactions
    async fn get_recent_transactions(
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        match state.sync_transaction.get_recent_transactions(20).await {
            Ok(transactions) => {
                let response = ApiResponse {
                    success: true,
                    message: None,
                    data: Some(transactions),
                    error: None,
                };
                (StatusCode::OK, Json(response))
            },
            Err(e) => {
                let response = ApiResponse::<Vec<serde_json::Value>> {
                    success: false,
                    message: None,
                    data: None,
                    error: Some(e.to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }
    
    /// Get entities by type
    async fn get_entities_by_type(
        State(state): State<Arc<AppState>>,
        Path(entity_type): Path<String>,
    ) -> impl IntoResponse {
        match state.sync_state.get_entities_by_type(&entity_type).await {
            Ok(entities) => {
                let response = ApiResponse {
                    success: true,
                    message: None,
                    data: Some(entities),
                    error: None,
                };
                (StatusCode::OK, Json(response))
            },
            Err(e) => {
                let response = ApiResponse::<Vec<serde_json::Value>> {
                    success: false,
                    message: None,
                    data: None,
                    error: Some(e.to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }
    
    /// Trigger a manual sync for an entity
    async fn trigger_sync(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<TriggerSyncRequest>,
    ) -> impl IntoResponse {
        // Create the sync event
        let sync_event = serde_json::json!({
            "entityType": payload.entity_type,
            "entityId": payload.entity_id,
            "operation": "SYNC",
            "priority": payload.priority.unwrap_or_else(|| "high".to_string())
        });
        
        // Publish the sync event
        match state.sync_service.publish_sync_event(&sync_event).await {
            Ok(_) => {
                let response = ApiResponse::<()> {
                    success: true,
                    message: Some(format!(
                        "Sync triggered for {} {}", 
                        payload.entity_type, 
                        payload.entity_id
                    )),
                    data: None,
                    error: None,
                };
                (StatusCode::OK, Json(response))
            },
            Err(e) => {
                let response = ApiResponse::<()> {
                    success: false,
                    message: None,
                    data: None,
                    error: Some(e.to_string()),
                };
                (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
    }
}
