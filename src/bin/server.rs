// src/bin/server.rs
use actix_web::{web, App, HttpServer, middleware};
use log::info;
use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("127.0.0.1:{}", port);
    
    info!("Starting server on {}", address);
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            // Register routes
            .service(
                web::scope("/api/monitoring")
                    .configure(routes::monitoring::configure)
            )
            .default_service(web::route().to(|| async { "Not Found" }))
    })
    .bind(address)?
    .run()
    .await
}
