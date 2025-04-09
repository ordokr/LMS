use std::path::{Path, PathBuf};
use axum::{
    routing::get,
    extract::Extension,
    Router,
    response::IntoResponse,
    http::{StatusCode, header},
    body::Body,
};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::sync::Arc;
use log::{info, error, debug};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use sha2::{Sha256, Digest};

// Asset server with HTTP/2 optimization and caching
pub struct AssetServer {
    root_dir: PathBuf,
    cache: Arc<RwLock<HashMap<String, CachedAsset>>>,
    etags: Arc<RwLock<HashMap<String, String>>>,
    cache_duration: Duration,
    push_enabled: bool,
}

struct CachedAsset {
    content: Vec<u8>,
    content_type: String,
    last_modified: SystemTime,
    etag: String,
}

impl AssetServer {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            etags: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(3600), // 1 hour default
            push_enabled: true, // Enable HTTP/2 push by default
        }
    }
    
    // Configure cache duration
    pub fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }
    
    // Control HTTP/2 push
    pub fn with_push(mut self, enabled: bool) -> Self {
        self.push_enabled = enabled;
        self
    }
    
    // Register static files for specific route
    pub fn router(self) -> Router {
        Router::new()
            .route("/*path", get(serve_static_file))
            .layer(Extension(Arc::new(self)))
    }
    
    // Preload common assets into memory
    pub async fn preload_common_assets(&self, files: &[&str]) -> Result<(), std::io::Error> {
        info!("Preloading {} common assets", files.len());
        
        let mut cache = self.cache.write().await;
        let mut etags = self.etags.write().await;
        
        for &file in files {
            let path = self.root_dir.join(file);
            if path.exists() {
                let content_type = mime_type_from_path(&path);
                let mut file = File::open(&path).await?;
                let mut content = Vec::new();
                file.read_to_end(&mut content).await?;
                
                // Generate ETag
                let etag = generate_etag(&content);
                
                // Store in cache
                cache.insert(file.to_string(), CachedAsset {
                    content,
                    content_type,
                    last_modified: SystemTime::now(),
                    etag: etag.clone(),
                });
                
                // Store ETag separately for quick lookups
                etags.insert(file.to_string(), etag);
                
                debug!("Preloaded asset: {}", file);
            }
        }
        
        info!("Preloaded {} assets into memory", cache.len());
        Ok(())
    }
    
    // Clean expired cache entries
    pub async fn clean_cache(&self) {
        let mut cache = self.cache.write().await;
        let now = SystemTime::now();
        
        let before_count = cache.len();
        cache.retain(|_, asset| {
            match now.duration_since(asset.last_modified) {
                Ok(age) => age < self.cache_duration,
                Err(_) => true, // Keep if error calculating age
            }
        });
        
        let removed = before_count - cache.len();
        if removed > 0 {
            debug!("Cleaned {} expired cache entries", removed);
        }
    }
}

// HTTP handler for static files
async fn serve_static_file(
    axum::extract::Path(path): axum::extract::Path<String>,
    Extension(asset_server): Extension<Arc<AssetServer>>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // Normalized path
    let path = path.trim_start_matches('/');
    
    // Check if-none-match header for ETags
    let etag_header = headers.get(header::IF_NONE_MATCH).and_then(|v| v.to_str().ok());
    
    // Check for cached ETags first (fast path)
    if let Some(etag) = etag_header {
        let etags = asset_server.etags.read().await;
        if let Some(stored_etag) = etags.get(path) {
            if etag.contains(stored_etag) {
                // Client already has current version
                return (StatusCode::NOT_MODIFIED, []).into_response();
            }
        }
    }
    
    // Try to get from cache
    let file_path = asset_server.root_dir.join(path);
    
    // Check cache
    {
        let cache = asset_server.cache.read().await;
        if let Some(cached) = cache.get(path) {
            // Check ETags again in case we missed in the fast path
            if let Some(etag) = etag_header {
                if etag.contains(&cached.etag) {
                    // Client already has current version
                    return (StatusCode::NOT_MODIFIED, []).into_response();
                }
            }
            
            // Return cached asset
            let mut headers = header::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, cached.content_type.parse().unwrap());
            headers.insert(header::ETAG, cached.etag.parse().unwrap());
            headers.insert(
                header::CACHE_CONTROL, 
                format!("public, max-age={}", asset_server.cache_duration.as_secs()).parse().unwrap()
            );
            
            // Add HTTP/2 push headers if enabled
            if asset_server.push_enabled {
                // For CSS, we might want to push fonts
                if cached.content_type == "text/css" {
                    let mut link_header = Vec::new();
                    
                    // Common font files that might be referenced in CSS
                    for font in &["woff2", "woff", "ttf"] {
                        link_header.push(format!("</fonts/font.{}>; rel=preload; as=font", font));
                    }
                    
                    if !link_header.is_empty() {
                        headers.insert(
                            header::LINK,
                            link_header.join(", ").parse().unwrap()
                        );
                    }
                }
            }
            
            return (headers, cached.content.clone()).into_response();
        }
    }
    
    // Not in cache, try to read from disk
    if !file_path.exists() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }
    
    // Read the file
    match tokio::fs::read(&file_path).await {
        Ok(content) => {
            let content_type = mime_type_from_path(&file_path);
            let etag = generate_etag(&content);
            
            // Cache the asset
            let mut cache = asset_server.cache.write().await;
            let mut etags = asset_server.etags.write().await;
            
            cache.insert(path.to_string(), CachedAsset {
                content: content.clone(),
                content_type: content_type.clone(),
                last_modified: SystemTime::now(),
                etag: etag.clone(),
            });
            
            etags.insert(path.to_string(), etag.clone());
            
            // Build response
            let mut headers = header::HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
            headers.insert(header::ETAG, etag.parse().unwrap());
            headers.insert(
                header::CACHE_CONTROL, 
                format!("public, max-age={}", asset_server.cache_duration.as_secs()).parse().unwrap()
            );
            
            (headers, content).into_response()
        },
        Err(err) => {
            error!("Failed to read file {}: {}", file_path.display(), err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read file").into_response()
        }
    }
}

// Helper for generating ETags
fn generate_etag(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    format!("\"{}\"", hex::encode(&result[..16])) // Use first 16 bytes of SHA-256
}

// Detect MIME type from file extension
fn mime_type_from_path(path: &Path) -> String {
    let extension = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
        
    match extension.to_lowercase().as_str() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "eot" => "application/vnd.ms-fontobject",
        "ico" => "image/x-icon",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }.to_string()
}