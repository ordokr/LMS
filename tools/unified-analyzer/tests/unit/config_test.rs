use std::fs;
use tempfile::TempDir;

use unified_analyzer::config::Config;
use crate::test_utils::create_temp_dir;

#[test]
fn test_config_from_file() {
    // Create a temporary directory for testing
    let temp_dir: TempDir = create_temp_dir();
    let config_path = temp_dir.path().join("config.toml");
    
    // Create a test configuration file
    let config_content = r#"
# Output directories
[output]
docs_dir = "docs"
api_dir = "docs/api"
architecture_dir = "docs/architecture"
models_dir = "docs/models"
integration_dir = "docs/integration"

# Documentation generation options
[documentation]
# Whether to generate high priority documentation
generate_high_priority = true
# Whether to generate medium priority documentation
generate_medium_priority = true
# Whether to exclude AI/Gemini-related content
exclude_ai_content = true

# High priority documentation
[documentation.high_priority]
central_reference_hub = true
api_documentation = true
implementation_details = true
testing_documentation = true
technical_debt_report = true
summary_report = true

# Medium priority documentation
[documentation.medium_priority]
synchronization_architecture = true
database_architecture = true

# Project information
[project]
name = "LMS"
description = "Learning Management System"
version = "0.1.0"
repository = "https://github.com/ordokr/LMS.git"

# Analysis options
[analysis]
# Maximum depth to search for files
max_depth = 10
# File extensions to include in analysis
include_extensions = [".rs", ".js", ".ts", ".jsx", ".tsx", ".html", ".css", ".scss", ".md", ".toml", ".json"]
# Directories to exclude from analysis
exclude_dirs = ["node_modules", "target", "dist", "build", ".git"]
"#;
    
    fs::write(&config_path, config_content).expect("Failed to write test configuration file");
    
    // Load the configuration
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Failed to load configuration: {:?}", config.err());
    
    let config = config.unwrap();
    
    // Check that the configuration was loaded correctly
    assert_eq!(config.output.docs_dir, "docs");
    assert_eq!(config.output.api_dir, "docs/api");
    assert_eq!(config.output.architecture_dir, "docs/architecture");
    assert_eq!(config.output.models_dir, "docs/models");
    assert_eq!(config.output.integration_dir, "docs/integration");
    
    assert_eq!(config.documentation.generate_high_priority, true);
    assert_eq!(config.documentation.generate_medium_priority, true);
    assert_eq!(config.documentation.exclude_ai_content, true);
    
    assert_eq!(config.documentation.high_priority.central_reference_hub, true);
    assert_eq!(config.documentation.high_priority.api_documentation, true);
    assert_eq!(config.documentation.high_priority.implementation_details, true);
    assert_eq!(config.documentation.high_priority.testing_documentation, true);
    assert_eq!(config.documentation.high_priority.technical_debt_report, true);
    assert_eq!(config.documentation.high_priority.summary_report, true);
    
    assert_eq!(config.documentation.medium_priority.synchronization_architecture, true);
    assert_eq!(config.documentation.medium_priority.database_architecture, true);
    
    assert_eq!(config.project.name, "LMS");
    assert_eq!(config.project.description, "Learning Management System");
    assert_eq!(config.project.version, "0.1.0");
    assert_eq!(config.project.repository, "https://github.com/ordokr/LMS.git");
    
    assert_eq!(config.analysis.max_depth, 10);
    assert_eq!(config.analysis.include_extensions.len(), 11);
    assert_eq!(config.analysis.exclude_dirs.len(), 5);
}

#[test]
fn test_config_default() {
    // Load the default configuration
    let config = Config::default();
    
    // Check that the configuration was loaded correctly
    assert_eq!(config.documentation.generate_high_priority, true);
    assert_eq!(config.documentation.generate_medium_priority, true);
    
    assert_eq!(config.documentation.high_priority.central_reference_hub, true);
    assert_eq!(config.documentation.high_priority.api_documentation, true);
    assert_eq!(config.documentation.high_priority.implementation_details, true);
    assert_eq!(config.documentation.high_priority.testing_documentation, true);
    assert_eq!(config.documentation.high_priority.technical_debt_report, true);
    assert_eq!(config.documentation.high_priority.summary_report, true);
    
    assert_eq!(config.documentation.medium_priority.synchronization_architecture, true);
    assert_eq!(config.documentation.medium_priority.database_architecture, true);
}
