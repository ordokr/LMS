use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tokio::fs;
use tokio::time::{sleep, Duration};
use log::{debug, error, info, warn};
use once_cell::sync::Lazy;
use anyhow::{Result, Context};

// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub forum: ForumConfig,
    pub database: DatabaseConfig,
    pub search: SearchConfig,
    pub media: MediaConfig,
    pub websocket: WebSocketConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub debug: bool,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumConfig {
    pub title: String,
    pub description: String,
    pub posts_per_page: usize,
    pub topics_per_page: usize,
    pub enable_markdown: bool,
    pub enable_code_highlighting: bool,
    pub enable_latex: bool,
    pub max_post_length: usize,
    pub max_title_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub pool_size: usize,
    pub max_connections: usize,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchConfig {
    pub enabled: bool,
    pub provider: String,
    pub meilisearch_url: Option<String>,
    pub meilisearch_key: Option<String>,
    pub auto_index: bool,
    pub index_interval_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaConfig {
    pub storage_path: String,
    pub max_upload_size_mb: usize,
    pub allowed_extensions: Vec<String>,
    pub optimize_images: bool,
    pub max_image_width: u32,
    pub thumbnail_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketConfig {
    pub enabled: bool,
    pub port: u16,
    pub ping_interval_seconds: u64,
    pub max_connections: usize,
}

// Config with reload support
pub struct ConfigManager {
    config: RwLock<Config>,
    path: PathBuf,
    last_modified: RwLock<SystemTime>,
    reload_handlers: RwLock<Vec<Box<dyn Fn(&Config) + Send + Sync>>>,
}

// Global config singleton
static CONFIG: Lazy<Arc<ConfigManager>> = Lazy::new(|| {
    // Default config path
    let config_path = std::env::var("CONFIG_PATH")
        .unwrap_or_else(|_| "config.toml".to_string());
        
    match ConfigManager::from_file(&config_path) {
        Ok(config) => Arc::new(config),
        Err(e) => {
            error!("Failed to load config: {}", e);
            panic!("Cannot start without valid configuration");
        }
    }
});

impl ConfigManager {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        
        // Read config file
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;
        
        let config: Config = toml::from_str(&content)
            .context("Failed to parse TOML config")?;
            
        let last_modified = std::fs::metadata(&path)
            .context("Failed to get config file metadata")?
            .modified()
            .context("Failed to get modified time")?;
            
        Ok(Self {
            config: RwLock::new(config),
            path,
            last_modified: RwLock::new(last_modified),
            reload_handlers: RwLock::new(Vec::new()),
        })
    }
    
    pub fn get() -> &'static Arc<ConfigManager> {
        &CONFIG
    }
    
    pub fn config(&self) -> Config {
        self.config.read().unwrap().clone()
    }
    
    pub async fn check_and_reload(&self) -> Result<bool> {
        let metadata = fs::metadata(&self.path).await?;
        let modified = metadata.modified()?;
        
        let reload_needed = {
            let last = self.last_modified.read().unwrap();
            modified > *last
        };
        
        if reload_needed {
            let content = fs::read_to_string(&self.path).await?;
            let new_config: Config = toml::from_str(&content)?;
            
            // Update config
            {
                let mut config = self.config.write().unwrap();
                *config = new_config.clone();
                
                let mut last = self.last_modified.write().unwrap();
                *last = modified;
            }
            
            // Notify handlers
            let handlers = self.reload_handlers.read().unwrap();
            let config = self.config.read().unwrap();
            
            for handler in handlers.iter() {
                handler(&config);
            }
            
            info!("Configuration reloaded successfully");
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn on_reload<F>(&self, handler: F)
    where
        F: Fn(&Config) + Send + Sync + 'static,
    {
        let mut handlers = self.reload_handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }
    
    // Start the background config watcher
    pub fn start_watcher(manager: Arc<ConfigManager>, interval_seconds: u64) {
        tokio::spawn(async move {
            let interval = Duration::from_secs(interval_seconds);
            
            loop {
                sleep(interval).await;
                
                match manager.check_and_reload().await {
                    Ok(true) => debug!("Config reloaded"),
                    Ok(false) => {}, // No changes
                    Err(e) => warn!("Failed to reload config: {}", e),
                }
            }
        });
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig {
                name: "EduConnect LMS".to_string(),
                version: "0.1.0".to_string(),
                debug: false,
                log_level: "info".to_string(),
            },
            forum: ForumConfig {
                title: "Discussion Forum".to_string(),
                description: "Course discussion forum".to_string(),
                posts_per_page: 20,
                topics_per_page: 10,
                enable_markdown: true,
                enable_code_highlighting: true,
                enable_latex: true,
                max_post_length: 10000,
                max_title_length: 100,
            },
            database: DatabaseConfig {
                path: "educonnect.db".to_string(),
                pool_size: 5,
                max_connections: 20,
                timeout_seconds: 30,
            },
            search: SearchConfig {
                enabled: true,
                provider: "meilisearch".to_string(),
                meilisearch_url: Some("http://localhost:7700".to_string()),
                meilisearch_key: None,
                auto_index: true,
                index_interval_minutes: 10,
            },
            media: MediaConfig {
                storage_path: "uploads".to_string(),
                max_upload_size_mb: 10,
                allowed_extensions: vec![
                    "jpg".to_string(), "jpeg".to_string(), "png".to_string(),
                    "webp".to_string(), "gif".to_string(), "pdf".to_string(),
                ],
                optimize_images: true,
                max_image_width: 1200,
                thumbnail_size: 200,
            },
            websocket: WebSocketConfig {
                enabled: true,
                port: 3001,
                ping_interval_seconds: 30,
                max_connections: 100,
            },
        }
    }
}