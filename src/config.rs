use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use anyhow::{Result, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub canvas: CanvasConfig,
    pub discourse: DiscourseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasConfig {
    pub api_url: String,
    pub api_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscourseConfig {
    pub api_url: String,
    pub api_key: Option<String>,
    pub api_username: Option<String>,
}

impl Config {
    // Load configuration from a file or create default if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            let default_config = Self::default();
            default_config.save()?;
            return Ok(default_config);
        }
        
        let config_str = fs::read_to_string(&config_path)
            .context(format!("Failed to read config file: {:?}", config_path))?;
            
        let config: Self = serde_json::from_str(&config_str)
            .context("Failed to parse config file")?;
            
        Ok(config)
    }
    
    // Save configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let config_dir = config_path.parent()
            .context("Failed to determine config directory")?;
            
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .context("Failed to create config directory")?;
        }
        
        let config_str = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
            
        fs::write(&config_path, config_str)
            .context(format!("Failed to write config to {:?}", config_path))?;
            
        Ok(())
    }
    
    // Get configuration file path
    fn get_config_path() -> Result<PathBuf> {
        // First check for environment variable
        if let Ok(path) = env::var("LMS_CONFIG_PATH") {
            return Ok(PathBuf::from(path));
        }
        
        // Then check for config in current directory
        let current_dir_config = Path::new("lms_config.json");
        if current_dir_config.exists() {
            return Ok(current_dir_config.to_path_buf());
        }
        
        // Finally, use platform-specific config directory
        let config_dir = dirs::config_dir()
            .context("Failed to determine config directory")?
            .join("lms");
            
        Ok(config_dir.join("config.json"))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            canvas: CanvasConfig {
                api_url: "https://canvas.instructure.com/api/v1".to_string(),
                api_token: None,
            },
            discourse: DiscourseConfig {
                api_url: "https://discuss.example.com".to_string(),
                api_key: None,
                api_username: None,
            },
        }
    }
}