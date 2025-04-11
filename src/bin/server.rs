use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use actix_web::{web, App, HttpServer, middleware, Responder, HttpResponse};
use actix_cors::Cors;
use mongodb::{Client, options::ClientOptions};
use serde_json::json;
use dotenv::dotenv;

use crate::routes::auth_routes;
use crate::routes::webhook_routes;
use crate::routes::notification_routes;
use crate::utils::logger::create_logger;

/// Application state
pub struct AppState {
    db: mongodb::Database,
    logger: slog::Logger,
}

/// Application configuration
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub mongodb_uri: String,
    pub port: u16,
    pub env: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mongodb_uri: env::var("MONGODB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017/lms-integration".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5000),
            env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
        }
    }
}

/// Initialize the application
pub async fn init_app(config: Option<AppConfig>) -> Result<actix_web::dev::Server, Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get configuration
    let config = config.unwrap_or_default();
    
    // Set up logger
    let logger = create_logger("app");
    slog::info!(logger, "Initializing application"; 
        "environment" => &config.env,
        "port" => config.port
    );
    
    // Connect to MongoDB if not in test environment
    let db = if config.env != "test" {
        slog::info!(logger, "Connecting to MongoDB"; "uri" => &config.mongodb_uri);
        
        let client_options = ClientOptions::parse(&config.mongodb_uri).await?;
        let client = Client::with_options(client_options)?;
        let db_name = config.mongodb_uri.split('/').last().unwrap_or("lms-integration");
        
        client.database(db_name)
    } else {
        // Create a mock database for testing
        slog::info!(logger, "Using mock database for testing");
        
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        let client = Client::with_options(client_options)?;
        
        client.database("lms-integration-test")
    };
    
    // Create app state
    let app_state = web::Data::new(AppState {
        db,
        logger: logger.clone(),
    });
    
    // Create and configure HTTP server
    let bind_address = format!("0.0.0.0:{}", config.port);
    
    let server = HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            // Register routes
            .service(
                web::scope("/api/v1")
                    .configure(auth_routes::configure)
                    .configure(webhook_routes::configure)
                    .configure(notification_routes::configure)
            )
            // Health check endpoint
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?;
    
    slog::info!(logger, "Server starting"; "address" => &bind_address);
    
    Ok(server.run())
}

/// Health check endpoint
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Main entry point for the application
#[actix_web::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = init_app(None).await?;
    server.await?;
    Ok(())
}
