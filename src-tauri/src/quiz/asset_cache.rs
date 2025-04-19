use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;
use uuid::Uuid;
use anyhow::{Result, anyhow};
use tracing::{debug, info, warn, error};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Types of media assets that can be cached
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Audio,
    Video,
    Document,
    Other,
}

impl AssetType {
    /// Get the appropriate file extension for this asset type
    pub fn extension(&self) -> &'static str {
        match self {
            AssetType::Image => "png",
            AssetType::Audio => "mp3",
            AssetType::Video => "mp4",
            AssetType::Document => "pdf",
            AssetType::Other => "bin",
        }
    }
    
    /// Determine asset type from file extension
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "svg" => AssetType::Image,
            "mp3" | "wav" | "ogg" | "m4a" => AssetType::Audio,
            "mp4" | "webm" | "mov" | "avi" => AssetType::Video,
            "pdf" | "doc" | "docx" | "txt" => AssetType::Document,
            _ => AssetType::Other,
        }
    }
    
    /// Get the MIME type for this asset type
    pub fn mime_type(&self) -> &'static str {
        match self {
            AssetType::Image => "image/png",
            AssetType::Audio => "audio/mpeg",
            AssetType::Video => "video/mp4",
            AssetType::Document => "application/pdf",
            AssetType::Other => "application/octet-stream",
        }
    }
}

/// Cached asset with metadata
#[derive(Debug, Clone)]
struct CachedAsset {
    data: Vec<u8>,
    asset_type: AssetType,
    etag: String,
    size: usize,
    last_accessed: Instant,
    access_count: u32,
    expires_at: Instant,
}

/// Asset metadata for tracking without loading the full asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMetadata {
    pub id: String,
    pub original_filename: String,
    pub asset_type: AssetType,
    pub size: usize,
    pub etag: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub quiz_id: Option<Uuid>,
    pub question_id: Option<Uuid>,
}

/// Cache configuration
#[derive(Debug, Clone)]
pub struct AssetCacheConfig {
    pub cache_dir: PathBuf,
    pub max_memory_size: usize,
    pub ttl: Duration,
    pub preload_assets: bool,
}

impl Default for AssetCacheConfig {
    fn default() -> Self {
        Self {
            cache_dir: PathBuf::from("cache/assets"),
            max_memory_size: 100 * 1024 * 1024, // 100 MB
            ttl: Duration::from_secs(3600), // 1 hour
            preload_assets: false,
        }
    }
}

/// Asset cache for quiz media
pub struct AssetCache {
    config: AssetCacheConfig,
    memory_cache: RwLock<HashMap<String, CachedAsset>>,
    metadata_cache: RwLock<HashMap<String, AssetMetadata>>,
    current_memory_usage: RwLock<usize>,
}

impl AssetCache {
    /// Create a new asset cache with the given configuration
    pub async fn new(config: AssetCacheConfig) -> Result<Arc<Self>> {
        // Ensure cache directory exists
        fs::create_dir_all(&config.cache_dir).await?;
        
        let cache = Arc::new(Self {
            config,
            memory_cache: RwLock::new(HashMap::new()),
            metadata_cache: RwLock::new(HashMap::new()),
            current_memory_usage: RwLock::new(0),
        });
        
        // Preload metadata for all assets
        cache.load_metadata().await?;
        
        // Preload assets into memory if configured
        if cache.config.preload_assets {
            cache.preload_assets().await?;
        }
        
        Ok(cache)
    }
    
    /// Load metadata for all assets in the cache directory
    async fn load_metadata(&self) -> Result<()> {
        let mut entries = fs::read_dir(&self.config.cache_dir).await?;
        let mut metadata = self.metadata_cache.write().await;
        
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            // Only process metadata files
            if path.extension().map_or(false, |ext| ext == "meta") {
                if let Ok(meta_content) = fs::read_to_string(&path).await {
                    if let Ok(asset_meta) = serde_json::from_str::<AssetMetadata>(&meta_content) {
                        metadata.insert(asset_meta.id.clone(), asset_meta);
                    }
                }
            }
        }
        
        info!("Loaded metadata for {} assets", metadata.len());
        Ok(())
    }
    
    /// Preload frequently accessed assets into memory
    async fn preload_assets(&self) -> Result<()> {
        let metadata = self.metadata_cache.read().await;
        let mut memory_cache = self.memory_cache.write().await;
        let mut memory_usage = self.current_memory_usage.write().await;
        
        // Sort by most recently created (as a proxy for importance)
        let mut assets: Vec<_> = metadata.values().collect();
        assets.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Preload up to 50% of max memory
        let preload_limit = self.config.max_memory_size / 2;
        let mut loaded_size = 0;
        
        for asset in assets {
            // Stop if we've reached the preload limit
            if loaded_size >= preload_limit {
                break;
            }
            
            // Try to load the asset
            let asset_path = self.get_asset_path(&asset.id);
            if let Ok(data) = fs::read(&asset_path).await {
                // Only cache if it fits in our remaining budget
                if loaded_size + data.len() <= preload_limit {
                    memory_cache.insert(asset.id.clone(), CachedAsset {
                        data: data.clone(),
                        asset_type: asset.asset_type,
                        etag: asset.etag.clone(),
                        size: data.len(),
                        last_accessed: Instant::now(),
                        access_count: 0,
                        expires_at: Instant::now() + self.config.ttl,
                    });
                    
                    loaded_size += data.len();
                }
            }
        }
        
        *memory_usage = loaded_size;
        info!("Preloaded {} bytes of assets into memory", loaded_size);
        Ok(())
    }
    
    /// Get the path to an asset file
    fn get_asset_path(&self, asset_id: &str) -> PathBuf {
        self.config.cache_dir.join(format!("{}", asset_id))
    }
    
    /// Get the path to an asset's metadata file
    fn get_metadata_path(&self, asset_id: &str) -> PathBuf {
        self.config.cache_dir.join(format!("{}.meta", asset_id))
    }
    
    /// Store an asset in the cache
    pub async fn store_asset(&self, data: Vec<u8>, filename: &str, quiz_id: Option<Uuid>, question_id: Option<Uuid>) -> Result<AssetMetadata> {
        // Generate a unique ID for the asset
        let asset_id = Uuid::new_v4().to_string();
        
        // Determine asset type from filename
        let extension = Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        
        let asset_type = AssetType::from_extension(extension);
        
        // Generate ETag
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();
        let etag = format!("\"{}\"", BASE64.encode(hash));
        
        // Create metadata
        let metadata = AssetMetadata {
            id: asset_id.clone(),
            original_filename: filename.to_string(),
            asset_type,
            size: data.len(),
            etag: etag.clone(),
            created_at: chrono::Utc::now(),
            quiz_id,
            question_id,
        };
        
        // Save the asset to disk
        let asset_path = self.get_asset_path(&asset_id);
        fs::write(&asset_path, &data).await?;
        
        // Save metadata
        let metadata_path = self.get_metadata_path(&asset_id);
        fs::write(&metadata_path, serde_json::to_string(&metadata)?).await?;
        
        // Update metadata cache
        {
            let mut meta_cache = self.metadata_cache.write().await;
            meta_cache.insert(asset_id.clone(), metadata.clone());
        }
        
        // Try to cache in memory if there's space
        self.try_cache_in_memory(asset_id.clone(), data, asset_type, etag).await?;
        
        Ok(metadata)
    }
    
    /// Try to cache an asset in memory if there's space
    async fn try_cache_in_memory(&self, id: String, data: Vec<u8>, asset_type: AssetType, etag: String) -> Result<()> {
        let data_size = data.len();
        let mut memory_cache = self.memory_cache.write().await;
        let mut memory_usage = self.current_memory_usage.write().await;
        
        // Check if we need to evict entries to make room
        if *memory_usage + data_size > self.config.max_memory_size {
            // Not enough space, need to evict
            self.evict_entries(*memory_usage + data_size - self.config.max_memory_size, &mut memory_cache).await?;
            *memory_usage = memory_cache.values().map(|asset| asset.size).sum();
        }
        
        // Now add the new entry if it fits
        if *memory_usage + data_size <= self.config.max_memory_size {
            memory_cache.insert(id, CachedAsset {
                data,
                asset_type,
                etag,
                size: data_size,
                last_accessed: Instant::now(),
                access_count: 0,
                expires_at: Instant::now() + self.config.ttl,
            });
            
            *memory_usage += data_size;
            Ok(())
        } else {
            // Still doesn't fit, just don't cache in memory
            debug!("Asset too large to fit in memory cache: {} bytes", data_size);
            Ok(())
        }
    }
    
    /// Evict entries from memory cache to free up the specified amount of space
    async fn evict_entries(&self, bytes_to_free: usize, cache: &mut HashMap<String, CachedAsset>) -> Result<()> {
        // Sort entries by access count and last accessed time
        let mut entries: Vec<_> = cache.iter().collect();
        entries.sort_by(|a, b| {
            // First by access count (ascending)
            let count_cmp = a.1.access_count.cmp(&b.1.access_count);
            if count_cmp != std::cmp::Ordering::Equal {
                return count_cmp;
            }
            
            // Then by last accessed time (oldest first)
            a.1.last_accessed.cmp(&b.1.last_accessed)
        });
        
        // Remove entries until we've freed enough space
        let mut freed = 0;
        let mut removed_keys = Vec::new();
        
        for (key, asset) in entries {
            if freed >= bytes_to_free {
                break;
            }
            
            freed += asset.size;
            removed_keys.push(key.clone());
            debug!("Evicting asset from memory cache: {}", key);
        }
        
        // Remove the entries
        for key in removed_keys {
            cache.remove(&key);
        }
        
        Ok(())
    }
    
    /// Get an asset by ID
    pub async fn get_asset(&self, asset_id: &str) -> Result<(Vec<u8>, AssetType, String)> {
        // Try memory cache first
        {
            let mut memory_cache = self.memory_cache.write().await;
            if let Some(asset) = memory_cache.get_mut(asset_id) {
                // Update access stats
                asset.last_accessed = Instant::now();
                asset.access_count += 1;
                
                // Extend expiration
                asset.expires_at = Instant::now() + self.config.ttl;
                
                return Ok((asset.data.clone(), asset.asset_type, asset.etag.clone()));
            }
        }
        
        // Not in memory, try to load from disk
        let asset_path = self.get_asset_path(asset_id);
        if !asset_path.exists() {
            return Err(anyhow!("Asset not found: {}", asset_id));
        }
        
        // Get metadata
        let metadata = {
            let meta_cache = self.metadata_cache.read().await;
            if let Some(meta) = meta_cache.get(asset_id) {
                meta.clone()
            } else {
                // Try to load metadata from disk
                let metadata_path = self.get_metadata_path(asset_id);
                let meta_content = fs::read_to_string(&metadata_path).await?;
                let meta: AssetMetadata = serde_json::from_str(&meta_content)?;
                
                // Update metadata cache
                let mut meta_cache = self.metadata_cache.write().await;
                meta_cache.insert(asset_id.to_string(), meta.clone());
                
                meta
            }
        };
        
        // Load the asset data
        let data = fs::read(&asset_path).await?;
        
        // Try to cache in memory for next time
        self.try_cache_in_memory(
            asset_id.to_string(),
            data.clone(),
            metadata.asset_type,
            metadata.etag.clone()
        ).await?;
        
        Ok((data, metadata.asset_type, metadata.etag))
    }
    
    /// Get asset metadata by ID
    pub async fn get_asset_metadata(&self, asset_id: &str) -> Result<AssetMetadata> {
        let meta_cache = self.metadata_cache.read().await;
        if let Some(meta) = meta_cache.get(asset_id) {
            Ok(meta.clone())
        } else {
            // Try to load metadata from disk
            let metadata_path = self.get_metadata_path(asset_id);
            let meta_content = fs::read_to_string(&metadata_path).await?;
            let meta: AssetMetadata = serde_json::from_str(&meta_content)?;
            
            // Update metadata cache
            let mut meta_cache = self.metadata_cache.write().await;
            meta_cache.insert(asset_id.to_string(), meta.clone());
            
            Ok(meta)
        }
    }
    
    /// Get all assets for a quiz
    pub async fn get_quiz_assets(&self, quiz_id: Uuid) -> Result<Vec<AssetMetadata>> {
        let meta_cache = self.metadata_cache.read().await;
        let assets = meta_cache.values()
            .filter(|meta| meta.quiz_id == Some(quiz_id))
            .cloned()
            .collect();
        
        Ok(assets)
    }
    
    /// Delete an asset
    pub async fn delete_asset(&self, asset_id: &str) -> Result<()> {
        // Remove from memory cache
        {
            let mut memory_cache = self.memory_cache.write().await;
            if let Some(asset) = memory_cache.remove(asset_id) {
                let mut memory_usage = self.current_memory_usage.write().await;
                *memory_usage -= asset.size;
            }
        }
        
        // Remove from metadata cache
        {
            let mut meta_cache = self.metadata_cache.write().await;
            meta_cache.remove(asset_id);
        }
        
        // Delete files
        let asset_path = self.get_asset_path(asset_id);
        let metadata_path = self.get_metadata_path(asset_id);
        
        if asset_path.exists() {
            fs::remove_file(&asset_path).await?;
        }
        
        if metadata_path.exists() {
            fs::remove_file(&metadata_path).await?;
        }
        
        Ok(())
    }
    
    /// Clear expired entries from memory cache
    pub async fn clear_expired(&self) -> Result<()> {
        let now = Instant::now();
        let mut memory_cache = self.memory_cache.write().await;
        let mut memory_usage = self.current_memory_usage.write().await;
        
        let expired_keys: Vec<_> = memory_cache.iter()
            .filter(|(_, asset)| asset.expires_at <= now)
            .map(|(key, _)| key.clone())
            .collect();
        
        let mut freed = 0;
        for key in expired_keys {
            if let Some(asset) = memory_cache.remove(&key) {
                freed += asset.size;
                debug!("Removed expired asset from memory cache: {}", key);
            }
        }
        
        *memory_usage -= freed;
        debug!("Cleared {} bytes of expired assets from memory", freed);
        
        Ok(())
    }
    
    /// Get cache statistics
    pub async fn get_stats(&self) -> Result<AssetCacheStats> {
        let memory_cache = self.memory_cache.read().await;
        let meta_cache = self.metadata_cache.read().await;
        let memory_usage = *self.current_memory_usage.read().await;
        
        Ok(AssetCacheStats {
            memory_cache_size: memory_usage,
            memory_cache_count: memory_cache.len(),
            disk_cache_count: meta_cache.len(),
            max_memory_size: self.config.max_memory_size,
        })
    }
}

/// Statistics about the asset cache
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetCacheStats {
    pub memory_cache_size: usize,
    pub memory_cache_count: usize,
    pub disk_cache_count: usize,
    pub max_memory_size: usize,
}
