use axum::{
    routing::{get, post},
    Router, Extension, extract::DefaultBodyLimit,
    http::{header, StatusCode, HeaderValue},
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use tower_http::limit::RequestBodyLimitLayer;
use tower::{Service, ServiceBuilder};

pub async fn start_server(
    db_pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
    forum_repo: Arc<crate::db::forum::ForumRepository>,
    search_client: Arc<crate::search::meilisearch::MeiliSearchClient>,
) -> Result<axum::Server<hyper::server::conn::AddrIncoming, Router>, anyhow::Error> {
    // Create optimized middleware stack
    let middleware_stack = ServiceBuilder::new()
        // Add request tracing
        .layer(TraceLayer::new_for_http())
        // Add response compression
        .layer(CompressionLayer::new())
        // Add CORS support
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(HeaderValue::from_static("http://localhost:3000")))
                .allow_methods(tower_http::cors::Any)
                .allow_headers(AllowHeaders::any())
                .max_age(std::time::Duration::from_secs(3600))
        )
        // Limit request body size
        .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10)) // 10MB limit
        // Set timeout
        .timeout(std::time::Duration::from_secs(30));

    // Create router with optimized routes
    let app = Router::new()
        // Forum routes
        .nest(
            "/api/forum", 
            crate::routes::forum::create_routes(forum_repo.clone())
        )
        // Search routes
        .nest(
            "/api/search",
            crate::routes::search::create_routes(search_client.clone())
        )
        // Health check for monitoring
        .route("/health", get(health_handler))
        // Apply middleware
        .layer(middleware_stack);
    
    // Start server with optimized settings
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr)
        .tcp_nodelay(true) // Enable TCP_NODELAY for lower latency
        .tcp_keepalive(Some(std::time::Duration::from_secs(60)))
        .serve(app.into_make_service());
    
    Ok(server)
}

async fn health_handler() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}