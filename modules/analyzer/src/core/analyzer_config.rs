use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// Configuration for the unified analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    /// Base directory for analysis
    pub base_dir: PathBuf,

    /// Output directory for generated reports
    pub output_dir: PathBuf,

    /// Directories to analyze
    pub target_dirs: Vec<PathBuf>,

    /// Patterns to exclude from analysis
    pub exclude_patterns: Vec<String>,

    /// Whether to run in quick mode (minimal analysis)
    pub quick_mode: bool,

    /// Whether to update the RAG knowledge base
    pub update_rag_knowledge_base: bool,

    /// Whether to generate AI insights
    pub generate_ai_insights: bool,

    /// Whether to analyze JavaScript files for Rust migration
    pub analyze_js_files: bool,

    /// Whether to generate a visual dashboard
    pub generate_dashboard: bool,

    /// Whether to analyze technical debt
    pub analyze_tech_debt: bool,

    /// Whether to analyze code quality
    pub analyze_code_quality: bool,

    /// Whether to analyze data models
    pub analyze_models: bool,

    /// Technology stack configuration
    pub tech_stack: TechStack,

    /// Architecture configuration
    pub architecture: ArchitectureConfig,

    /// Integration configurations
    pub integrations: Vec<IntegrationConfig>,
}

/// Technology stack configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechStack {
    pub frontend: Vec<String>,
    pub backend: Vec<String>,
    pub database: Vec<String>,
    pub search: Vec<String>,
    pub ai: Vec<String>,
    pub blockchain: Vec<String>,
    pub authentication: Vec<String>,
}

/// Architecture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureConfig {
    pub patterns: Vec<String>,
    pub principles: Vec<String>,
    pub diagrams: Vec<String>,
}

/// Integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub name: String,
    pub source_system: String,
    pub target_system: String,
    pub integration_points: Vec<String>,
    pub status: String,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            base_dir: PathBuf::from("."),
            output_dir: PathBuf::from("docs"),
            target_dirs: vec![PathBuf::from(".")],
            exclude_patterns: vec![
                String::from("node_modules"),
                String::from("target"),
                String::from(".git"),
                String::from("build-output"),
            ],
            quick_mode: false,
            update_rag_knowledge_base: false,
            generate_ai_insights: false,
            analyze_js_files: false,
            generate_dashboard: false,
            analyze_tech_debt: false,
            analyze_code_quality: false,
            analyze_models: false,
            tech_stack: TechStack {
                frontend: vec![String::from("Leptos"), String::from("Tauri")],
                backend: vec![String::from("Rust"), String::from("Haskell")],
                database: vec![String::from("SQLite"), String::from("sqlx")],
                search: vec![String::from("MeiliSearch")],
                ai: vec![String::from("Gemini")],
                blockchain: vec![String::from("Custom Rust implementation")],
                authentication: vec![String::from("JWT")],
            },
            architecture: ArchitectureConfig {
                patterns: vec![
                    String::from("CQRS"),
                    String::from("Event Sourcing"),
                    String::from("Repository Pattern"),
                ],
                principles: vec![
                    String::from("Clean Architecture"),
                    String::from("SOLID"),
                    String::from("Offline-first"),
                ],
                diagrams: vec![
                    String::from("docs/architecture/high_level.md"),
                    String::from("docs/architecture/data_flow.md"),
                ],
            },
            integrations: vec![
                IntegrationConfig {
                    name: String::from("Canvas Course Management"),
                    source_system: String::from("Canvas"),
                    target_system: String::from("LMS"),
                    integration_points: vec![
                        String::from("Course creation"),
                        String::from("Assignment management"),
                        String::from("Grading system"),
                    ],
                    status: String::from("In Progress"),
                },
                IntegrationConfig {
                    name: String::from("Discourse Forums"),
                    source_system: String::from("Discourse"),
                    target_system: String::from("LMS"),
                    integration_points: vec![
                        String::from("Discussion threads"),
                        String::from("User profiles"),
                        String::from("Notifications"),
                    ],
                    status: String::from("Planned"),
                },
                IntegrationConfig {
                    name: String::from("Blockchain Certification"),
                    source_system: String::from("Native"),
                    target_system: String::from("LMS"),
                    integration_points: vec![
                        String::from("Certificate issuance"),
                        String::from("Achievement verification"),
                        String::from("Credential storage"),
                    ],
                    status: String::from("In Progress"),
                },
            ],
        }
    }
}

impl AnalyzerConfig {
    /// Load configuration from a file
    pub fn load(config_path: Option<&str>) -> Result<Self, String> {
        let config_path = config_path.unwrap_or("analyzer_config.toml");

        // Check if the config file exists
        if !Path::new(config_path).exists() {
            println!("Config file not found at {}. Using default configuration.", config_path);
            return Ok(Self::default());
        }

        // Read the config file
        let config_content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        // Parse the config file
        let config: Self = toml::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        Ok(config)
    }

    /// Save configuration to a file
    pub fn save(&self, config_path: Option<&str>) -> Result<(), String> {
        let config_path = config_path.unwrap_or("analyzer_config.toml");

        // Convert to TOML
        let config_content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        // Write to file
        fs::write(config_path, config_content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}
