use axum::body::{Body, BoxBody};
use axum::http::{Request, Response, HeaderValue, HeaderName};
use axum::response::IntoResponse;
use futures::future::BoxFuture;
use async_compression::tokio::bufread::{BrotliEncoder, GzipEncoder, DeflateEncoder};
use std::task::{Context, Poll};
use tower::{Layer, Service};
use pin_project::pin_project;
use std::pin::Pin;

// Compression layer for API responses
pub struct CompressionLayer {
    quality: u32,
    min_size: usize,
}

impl CompressionLayer {
    pub fn new() -> Self {
        Self {
            quality: 4, // Default compression level (0-11, higher = better compression but slower)
            min_size: 1024, // Only compress responses larger than 1KB
        }
    }
    
    pub fn quality(mut self, quality: u32) -> Self {
        self.quality = quality.clamp(0, 11);
        self
    }
    
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }
}

impl<S> Layer<S> for CompressionLayer {
    type Service = CompressionMiddleware<S>;
    
    fn layer(&self, service: S) -> Self::Service {
        CompressionMiddleware {
            inner: service,
            quality: self.quality,
            min_size: self.min_size,
        }
    }
}

// Service implementation
pub struct CompressionMiddleware<S> {
    inner: S,
    quality: u32,
    min_size: usize,
}

impl<S, ReqBody> Service<Request<ReqBody>> for CompressionMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response<BoxBody>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    
    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        // Extract accepted encodings from the request
        let accepts_encodings = req.headers()
            .get("accept-encoding")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_lowercase())
            .unwrap_or_default();
            
        let quality = self.quality;
        let min_size = self.min_size;
        let mut svc = self.inner.clone();
        
        Box::pin(async move {
            let res = svc.call(req).await?;
            
            // Check if compression should be applied
            let content_length = res.headers()
                .get("content-length")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(0);
                
            let content_type = res.headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("");
                
            // Only compress text-based content above min_size
            let should_compress = content_length > min_size && (
                content_type.contains("text/") || 
                content_type.contains("application/json") || 
                content_type.contains("application/javascript") || 
                content_type.contains("application/xml") ||
                content_type.contains("image/svg")
            );
            
            if should_compress {
                // Choose compression algorithm based on accepted encodings
                let compression_type = if accepts_encodings.contains("br") {
                    Some(("br", CompressAlgo::Brotli))
                } else if accepts_encodings.contains("gzip") {
                    Some(("gzip", CompressAlgo::Gzip))
                } else if accepts_encodings.contains("deflate") {
                    Some(("deflate", CompressAlgo::Deflate))
                } else {
                    None
                };
                
                if let Some((encoding_name, algo)) = compression_type {
                    // Apply compression
                    let (parts, body) = res.into_parts();
                    let compressed_body = compress_body(body, algo, quality).await;
                    
                    let mut res = Response::from_parts(parts, BoxBody::new(compressed_body));
                    
                    // Add compression headers
                    res.headers_mut().insert(
                        "content-encoding",
                        HeaderValue::from_static(encoding_name),
                    );
                    
                    // Remove content-length as it's no longer accurate
                    res.headers_mut().remove("content-length");
                    
                    // Add Vary header to inform caches
                    res.headers_mut().insert(
                        "vary",
                        HeaderValue::from_static("accept-encoding"),
                    );
                    
                    return Ok(res);
                }
            }
            
            // If no compression needed, return unmodified response
            Ok(res)
        })
    }
}

// Compression algorithm enum
enum CompressAlgo {
    Brotli,
    Gzip,
    Deflate,
}

// Function to compress response body
async fn compress_body(body: BoxBody, algo: CompressAlgo, quality: u32) -> BoxBody {
    // Implementation depends on compression libraries
    // This is a simplified placeholder
    BoxBody::new(body)
}

// Performance-optimized response headers middleware
pub fn add_performance_headers<B>(mut res: Response<B>) -> Response<B> {
    // Add performance optimization headers
    let headers = res.headers_mut();
    
    // Cache control (adjust based on content type)
    headers.insert(
        HeaderName::from_static("cache-control"),
        HeaderValue::from_static("public, max-age=3600"),
    );
    
    // Security headers
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    
    // Performance headers
    headers.insert(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );
    
    res
}