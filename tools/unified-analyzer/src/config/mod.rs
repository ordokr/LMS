use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

/// Configuration for the unified analyzer
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Output directories
    pub output: OutputConfig,
    /// Documentation generation options
    pub documentation: DocumentationConfig,
    /// Project information
    pub project: ProjectConfig,
    /// Analysis options
    pub analysis: AnalysisConfig,
}

/// Output directory configuration
#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    /// Main documentation directory
    pub docs_dir: String,
    /// API documentation directory
    pub api_dir: String,
    /// Architecture documentation directory
    pub architecture_dir: String,
    /// Models documentation directory
    pub models_dir: String,
    /// Integration documentation directory
    pub integration_dir: String,
}

/// Documentation generation configuration
#[derive(Debug, Deserialize)]
pub struct DocumentationConfig {
    /// Whether to generate high priority documentation
    pub generate_high_priority: bool,
    /// Whether to generate medium priority documentation
    pub generate_medium_priority: bool,
    /// Whether to exclude AI/Gemini-related content
    pub exclude_ai_content: bool,
    /// High priority documentation options
    pub high_priority: HighPriorityConfig,
    /// Medium priority documentation options
    pub medium_priority: MediumPriorityConfig,
}

/// High priority documentation configuration
#[derive(Debug, Deserialize)]
pub struct HighPriorityConfig {
    /// Whether to generate central reference hub
    pub central_reference_hub: bool,
    /// Whether to generate API documentation
    pub api_documentation: bool,
    /// Whether to generate implementation details
    pub implementation_details: bool,
    /// Whether to generate testing documentation
    pub testing_documentation: bool,
    /// Whether to generate technical debt report
    pub technical_debt_report: bool,
    /// Whether to generate summary report
    pub summary_report: bool,
}

/// Medium priority documentation configuration
#[derive(Debug, Deserialize)]
pub struct MediumPriorityConfig {
    /// Whether to generate synchronization architecture
    pub synchronization_architecture: bool,
    /// Whether to generate database architecture
    pub database_architecture: bool,
}

/// Project information configuration
#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Project version
    pub version: String,
    /// Project repository
    pub repository: String,
}

/// Analysis configuration
#[derive(Debug, Deserialize)]
pub struct AnalysisConfig {
    /// Maximum depth to search for files
    pub max_depth: usize,
    /// File extensions to include in analysis
    pub include_extensions: Vec<String>,
    /// Directories to exclude from analysis
    pub exclude_dirs: Vec<String>,
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)
            .context("Failed to read configuration file")?;

        let config: Config = toml::from_str(&content)
            .context("Failed to parse configuration file")?;

        Ok(config)
    }

    /// Get default configuration
    pub fn default() -> Self {
        toml::from_str(include_str!("../../config.toml"))
            .expect("Failed to parse default configuration")
    }
}
