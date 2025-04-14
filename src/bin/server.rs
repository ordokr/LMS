use actix_web::{web, App, HttpServer, middleware, Responder, HttpResponse};
use actix_cors::Cors;
use serde_json::json;
use dotenv::dotenv;
use sqlx::PgPool;

use crate::routes::{auth_routes, webhook_routes, notification_routes};
use crate::utils::logger::create_logger;

#[derive(Default)]
struct Config {
    env: String,
    port: u16,
}

impl Config {
    fn new() -> Self {
        // Example configuration setup
        Config {
            env: "development".to_string(),
            port: 3000,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let logger = create_logger(&config.env);
    let db = setup_database(&config).await?;
    let bind_address = format!("0.0.0.0:{}", config.port);

    // Example server setup
    Ok(())
}

async fn setup_database(config: &Config) -> Result<PgPool, Box<dyn std::error::Error>> {
    // Database setup logic
    let pool = PgPool::connect(&config.env).await?;
    Ok(pool)
}
