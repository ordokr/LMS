//! Configuration module for Ordo
//!
//! This module provides a centralized way to access configuration values
//! loaded from environment variables and configuration files.
//!
//! For detailed implementation guide, see docs/technical/environment_variables.md

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tokio::fs;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};
use once_cell::sync::Lazy;
use anyhow::{Result, Context};

pub mod env;

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
    // Initialize environment variables
    env::init();

    // Default config path from environment variable or default
    let config_path = env::get_env_or("CONFIG_PATH", "config.toml");

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
        // Initialize environment variables if not already initialized
        env::init();

        Self {
            app: AppConfig {
                name: env::get_env_or("APP_NAME", "Ordo LMS".to_string()),
                version: env::get_env_or("APP_VERSION", "0.1.0".to_string()),
                debug: env::get_env_bool("APP_DEBUG", false),
                log_level: env::get_env_or("RUST_LOG", "info".to_string()),
            },
            forum: ForumConfig {
                title: env::get_env_or("FORUM_TITLE", "Discussion Forum".to_string()),
                description: env::get_env_or("FORUM_DESCRIPTION", "Course discussion forum".to_string()),
                posts_per_page: env::get_env_or("FORUM_POSTS_PER_PAGE", 20),
                topics_per_page: env::get_env_or("FORUM_TOPICS_PER_PAGE", 10),
                enable_markdown: env::get_env_bool("FORUM_ENABLE_MARKDOWN", true),
                enable_code_highlighting: env::get_env_bool("FORUM_ENABLE_CODE_HIGHLIGHTING", true),
                enable_latex: env::get_env_bool("FORUM_ENABLE_LATEX", true),
                max_post_length: env::get_env_or("FORUM_MAX_POST_LENGTH", 10000),
                max_title_length: env::get_env_or("FORUM_MAX_TITLE_LENGTH", 100),
            },
            database: DatabaseConfig {
                path: env::get_env_or("DATABASE_PATH", "ordo.db".to_string()),
                pool_size: env::get_env_or("DATABASE_POOL_SIZE", 5),
                max_connections: env::get_env_or("DATABASE_MAX_CONNECTIONS", 20),
                timeout_seconds: env::get_env_or("DATABASE_TIMEOUT_SECONDS", 30),
            },
            search: SearchConfig {
                enabled: env::get_env_bool("SEARCH_ENABLED", true),
                provider: env::get_env_or("SEARCH_PROVIDER", "meilisearch".to_string()),
                meilisearch_url: env::get_optional_env("MEILI_HOST")
                    .or_else(|| Some("http://localhost:7700".to_string())),
                meilisearch_key: env::get_optional_env("MEILI_MASTER_KEY"),
                auto_index: env::get_env_bool("SEARCH_AUTO_INDEX", true),
                index_interval_minutes: env::get_env_or("SEARCH_INDEX_INTERVAL_MINUTES", 10),
            },
            media: MediaConfig {
                storage_path: env::get_env_or("MEDIA_STORAGE_PATH", "uploads".to_string()),
                max_upload_size_mb: env::get_env_or("MEDIA_MAX_UPLOAD_SIZE_MB", 10),
                allowed_extensions: env::get_optional_env("MEDIA_ALLOWED_EXTENSIONS")
                    .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_else(|| vec![
                        "jpg".to_string(), "jpeg".to_string(), "png".to_string(),
                        "webp".to_string(), "gif".to_string(), "pdf".to_string(),
                    ]),
                optimize_images: env::get_env_bool("MEDIA_OPTIMIZE_IMAGES", true),
                max_image_width: env::get_env_or("MEDIA_MAX_IMAGE_WIDTH", 1200),
                thumbnail_size: env::get_env_or("MEDIA_THUMBNAIL_SIZE", 200),
            },
            websocket: WebSocketConfig {
                enabled: env::get_env_bool("WEBSOCKET_ENABLED", true),
                port: env::get_env_or("WEBSOCKET_PORT", 3001),
                ping_interval_seconds: env::get_env_or("WEBSOCKET_PING_INTERVAL_SECONDS", 30),
                max_connections: env::get_env_or("WEBSOCKET_MAX_CONNECTIONS", 100),
            },
        }
    }
}