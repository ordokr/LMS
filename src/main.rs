mod auth;
mod api;
mod repository;
mod services;
mod models;
mod app_state;
mod clients;
mod jobs;
mod monitoring;
mod error;
mod middleware;

use app_state::AppState;
use api::integration::integration_routes;
use api::topic_mapping::topic_mapping_routes;
use api::monitoring::monitoring_routes;
use api::webhooks::webhook_routes;
use jobs::sync_scheduler::init_sync_scheduler;
use middleware::tracing::correlation_id_middleware;
use monitoring::api_health_check::ApiHealthCheck;

use axum::{
    Router,
    middleware,
    routing::get,
    response::Html,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use log::{info, warn};

#[tokio::main]
async fn main() {
    // Initialize logger with more structured format
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    info!("Starting Canvas-Discourse Integration Service");
    
    // Set up database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    
    info!("Database connection established");
    
    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    info!("Migrations completed successfully");
    
    // Create app state
    let app_state = AppState::new(pool.clone());
    
    // Initialize health checks
    let canvas_health_url = std::env::var("CANVAS_HEALTH_URL")
        .unwrap_or_else(|_| "https://canvas.example.com/api/v1/health_check".to_string());
    let discourse_health_url = std::env::var("DISCOURSE_HEALTH_URL")
        .unwrap_or_else(|_| "https://discourse.example.com/about.json".to_string());
    
    let health_check = ApiHealthCheck::new(
        &canvas_health_url,
        &discourse_health_url,
        app_state.sync_monitor.clone(),
        5 // timeout in seconds
    );
    
    // Run initial health check
    info!("Running initial health check...");
    match health_check.run_all_health_checks(&pool).await {
        Ok(healthy) => {
            info!("Initial health check complete. System is {}", 
                  if healthy { "healthy" } else { "unhealthy" });
        },
        Err(e) => {
            warn!("Error during initial health check: {}", e);
        }
    }
    
    // Start health check scheduler
    let health_check_interval = std::env::var("HEALTH_CHECK_INTERVAL_SECONDS")
        .unwrap_or_else(|_| "300".to_string()) // Default: 5 minutes
        .parse::<u64>()
        .unwrap_or(300);
    
    if health_check_interval > 0 {
        info!("Starting health check scheduler...");
        health_check.start_health_check_scheduler(pool.clone(), health_check_interval).await;
    }
    
    // Initialize sync scheduler if enabled
    if std::env::var("ENABLE_SYNC_SCHEDULER").unwrap_or_else(|_| "false".to_string()) == "true" {
        info!("Initializing sync scheduler...");
        let _scheduler = init_sync_scheduler(&app_state).await;
    }
    
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any); // In production, restrict to specific origins
    
    // Root handler for quick confirmation service is running
    async fn root_handler() -> Html<&'static str> {
        Html("
            <html>
                <head><title>Canvas-Discourse Integration</title></head>
                <body>
                    <h1>Canvas-Discourse Integration Service</h1>
                    <p>API service is running.</p>
                    <p><a href='/health'>Check system health</a></p>
                    <p><a href='/dashboard'>View monitoring dashboard</a></p>
                </body>
            </html>
        ")
    }
    
    // Set up router with all routes
    let app = Router::new()
        .route("/", get(root_handler))
        .merge(integration_routes())
        .merge(topic_mapping_routes())
        .merge(webhook_routes())
        .merge(monitoring_routes())
        // Apply middleware
        .layer(middleware::from_fn(correlation_id_middleware))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(app_state);
    
    // Start server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Server listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
