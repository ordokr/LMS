use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_connections: u32,
    pub sqlite_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: u64, // in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enabled: bool,
    pub sync_interval: u64, // in seconds
    pub batch_size: u32,
    pub sync_endpoint: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                connection_string: "sqlite:lms.db".to_string(),
                max_connections: 5,
                sqlite_path: "lms.db".to_string(),
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                jwt_secret: "change_this_to_a_secure_secret_key".to_string(),
                jwt_expiration: 86400, // 24 hours
            },
            sync: SyncConfig {
                enabled: true,
                sync_interval: 60,
                batch_size: 100,
                sync_endpoint: "https://api.example.com/sync".to_string(),
            },
        }
    }
}

impl AppConfig {
    pub fn load() -> Self {
        // Look for config file in current directory
        let config_path = Path::new("config.json");
        if config_path.exists() {
            let config_content = fs::read_to_string(config_path)
                .expect("Failed to read config file");
            
            serde_json::from_str(&config_content)
                .expect("Failed to parse config file")
        } else {
            // Create default config
            let default_config = Self::default();
            fs::write(
                config_path,
                serde_json::to_string_pretty(&default_config)
                    .expect("Failed to serialize default config"),
            )
            .expect("Failed to write default config file");
            
            default_config
        }
    }
}