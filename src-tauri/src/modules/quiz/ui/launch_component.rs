use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use tracing::{info, error};
use std::path::Path;
use std::fs;

/// Launch configuration for the quiz module
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizLaunchConfig {
    pub quiz_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub return_url: Option<String>,
    pub standalone: bool,
    pub theme: Option<String>,
    pub language: Option<String>,
}

/// Launch response for the quiz module
#[derive(Debug, Serialize, Deserialize)]
pub struct QuizLaunchResponse {
    pub success: bool,
    pub message: String,
    pub url: Option<String>,
    pub session_id: Option<String>,
}

/// Quiz launch component
pub struct QuizLaunchComponent {
    base_url: String,
    data_dir: String,
}

impl QuizLaunchComponent {
    /// Create a new quiz launch component
    pub fn new(base_url: &str, data_dir: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            data_dir: data_dir.to_string(),
        }
    }
    
    /// Launch the quiz module
    pub async fn launch(&self, config: QuizLaunchConfig) -> Result<QuizLaunchResponse> {
        info!("Launching quiz module with config: {:?}", config);
        
        // Create a session ID if not provided
        let session_id = match config.session_id {
            Some(id) => id,
            None => uuid::Uuid::new_v4().to_string(),
        };
        
        // Create a launch file in the data directory
        let launch_dir = Path::new(&self.data_dir).join("launches");
        if !launch_dir.exists() {
            fs::create_dir_all(&launch_dir)?;
        }
        
        let launch_file = launch_dir.join(format!("{}.json", session_id));
        let launch_data = serde_json::to_string(&config)?;
        fs::write(&launch_file, launch_data)?;
        
        // Build the launch URL
        let mut url = if config.standalone {
            format!("{}?session={}", self.base_url, session_id)
        } else {
            format!("{}/quiz?session={}", self.base_url, session_id)
        };
        
        // Add optional parameters
        if let Some(theme) = &config.theme {
            url.push_str(&format!("&theme={}", theme));
        }
        
        if let Some(language) = &config.language {
            url.push_str(&format!("&lang={}", language));
        }
        
        Ok(QuizLaunchResponse {
            success: true,
            message: "Quiz module launched successfully".to_string(),
            url: Some(url),
            session_id: Some(session_id),
        })
    }
    
    /// Get launch configuration from session ID
    pub fn get_launch_config(&self, session_id: &str) -> Result<QuizLaunchConfig> {
        let launch_file = Path::new(&self.data_dir).join("launches").join(format!("{}.json", session_id));
        
        if !launch_file.exists() {
            return Err(anyhow!("Launch configuration not found for session: {}", session_id));
        }
        
        let launch_data = fs::read_to_string(launch_file)?;
        let config: QuizLaunchConfig = serde_json::from_str(&launch_data)?;
        
        Ok(config)
    }
    
    /// Clean up launch configuration
    pub fn cleanup_launch(&self, session_id: &str) -> Result<()> {
        let launch_file = Path::new(&self.data_dir).join("launches").join(format!("{}.json", session_id));
        
        if launch_file.exists() {
            fs::remove_file(launch_file)?;
        }
        
        Ok(())
    }
}
