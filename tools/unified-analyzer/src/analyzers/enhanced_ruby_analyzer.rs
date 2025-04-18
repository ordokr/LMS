use crate::analyzers::modules::enhanced_ruby_model_analyzer::EnhancedRubyModelAnalyzer;
use crate::analyzers::modules::enhanced_ruby_controller_analyzer::EnhancedRubyControllerAnalyzer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EnhancedRubyError {
    IoError(std::io::Error),
    SerializationError(serde_json::Error),
    AnalysisError(String),
}

impl fmt::Display for EnhancedRubyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnhancedRubyError::IoError(err) => write!(f, "IO error: {}", err),
            EnhancedRubyError::SerializationError(err) => write!(f, "Serialization error: {}", err),
            EnhancedRubyError::AnalysisError(msg) => write!(f, "Analysis error: {}", msg),
        }
    }
}

impl Error for EnhancedRubyError {}

impl From<std::io::Error> for EnhancedRubyError {
    fn from(err: std::io::Error) -> Self {
        EnhancedRubyError::IoError(err)
    }
}

impl From<serde_json::Error> for EnhancedRubyError {
    fn from(err: serde_json::Error) -> Self {
        EnhancedRubyError::SerializationError(err)
    }
}

impl From<Box<dyn Error + Send + Sync>> for EnhancedRubyError {
    fn from(err: Box<dyn Error + Send + Sync>) -> Self {
        EnhancedRubyError::AnalysisError(err.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnhancedRubyAnalysisResult {
    pub models: serde_json::Value,
    pub controllers: serde_json::Value,
    pub source_system: String,
    pub file_count: usize,
    pub model_count: usize,
    pub controller_count: usize,
}

#[derive(Debug)]
pub struct EnhancedRubyAnalyzer {
    pub model_analyzer: EnhancedRubyModelAnalyzer,
    pub controller_analyzer: EnhancedRubyControllerAnalyzer,
    pub source_system: String,
}

impl EnhancedRubyAnalyzer {
    pub fn new(source_system: &str) -> Self {
        Self {
            model_analyzer: EnhancedRubyModelAnalyzer::new(),
            controller_analyzer: EnhancedRubyControllerAnalyzer::new(),
            source_system: source_system.to_string(),
        }
    }

    pub fn analyze(&mut self, directory: &str) -> Result<String, EnhancedRubyError> {
        println!("Starting enhanced Ruby analysis for {} at {}", self.source_system, directory);
        
        let path = PathBuf::from(directory);
        if !path.exists() {
            return Err(EnhancedRubyError::AnalysisError(format!(
                "Directory does not exist: {}", directory
            )));
        }
        
        // Analyze models
        println!("Analyzing Ruby models...");
        self.model_analyzer.analyze_directory(&path)?;
        
        // Analyze controllers
        println!("Analyzing Ruby controllers...");
        self.controller_analyzer.analyze_directory(&path)?;
        
        // Count files
        let file_count = count_ruby_files(&path);
        
        // Create result
        let result = EnhancedRubyAnalysisResult {
            models: serde_json::from_str(&self.model_analyzer.to_json()?)?,
            controllers: serde_json::from_str(&self.controller_analyzer.to_json()?)?,
            source_system: self.source_system.clone(),
            file_count,
            model_count: self.model_analyzer.models.len(),
            controller_count: self.controller_analyzer.controllers.len(),
        };
        
        // Convert to JSON
        let json = serde_json::to_string_pretty(&result)?;
        
        println!("Enhanced Ruby analysis complete for {}. Found {} models and {} controllers.",
            self.source_system, result.model_count, result.controller_count);
        
        Ok(json)
    }
    
    pub fn analyze_canvas(&mut self, directory: &str) -> Result<String, EnhancedRubyError> {
        self.source_system = "Canvas".to_string();
        self.analyze(directory)
    }
    
    pub fn analyze_discourse(&mut self, directory: &str) -> Result<String, EnhancedRubyError> {
        self.source_system = "Discourse".to_string();
        self.analyze(directory)
    }
}

// Helper function to count Ruby files in a directory
fn count_ruby_files(directory: &Path) -> usize {
    let mut count = 0;
    
    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            
            if path.is_dir() {
                count += count_ruby_files(&path);
            } else if let Some(extension) = path.extension() {
                if extension == "rb" {
                    count += 1;
                }
            }
        }
    }
    
    count
}
