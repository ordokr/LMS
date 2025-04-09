use axum::{
    body::Body,
    extract::State,
    http::{Request, Response},
    middleware::Next,
};
use uuid::Uuid;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::Layer;

// Correlation ID header names
const X_CORRELATION_ID: &str = "X-Correlation-ID";
const X_REQUEST_ID: &str = "X-Request-ID";

pub async fn correlation_id_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    // Extract correlation ID from headers or generate a new one
    let correlation_id = request
        .headers()
        .get(X_CORRELATION_ID)
        .or_else(|| request.headers().get(X_REQUEST_ID))
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    
    // Store correlation ID in request extensions
    request.extensions_mut().insert(CorrelationId(correlation_id.clone()));
    
    // Process the request
    let mut response = next.run(request).await;
    
    // Add correlation ID to response headers
    response.headers_mut().insert(
        X_CORRELATION_ID, 
        correlation_id.parse().expect("Failed to parse correlation ID")
    );
    
    response
}

// Correlation ID struct to store in request extensions
#[derive(Debug, Clone)]
pub struct CorrelationId(pub String);

// Helper function to get correlation ID from request extensions
pub fn get_correlation_id<B>(request: &Request<B>) -> Option<String> {
    request.extensions()
        .get::<CorrelationId>()
        .map(|id| id.0.clone())
}