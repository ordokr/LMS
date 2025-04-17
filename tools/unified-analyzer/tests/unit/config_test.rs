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
# General configuration
[general]
project_name = "LMS"
output_dir = "docs"
log_level = "info"

# Documentation generation options
[documentation]
# Whether to generate high priority documentation
generate_high_priority = true
# Whether to generate medium priority documentation
generate_medium_priority = true
# Whether to generate low priority documentation
generate_low_priority = false

# High priority documentation
[documentation.high_priority]
central_reference_hub = true
api_documentation = true
implementation_details = true
testing_documentation = true
tech_debt_report = true
summary_report = true

# Medium priority documentation
[documentation.medium_priority]
sync_architecture = true
database_architecture = true

# Analysis options
[analysis]
# Analyzers to use
analyzers = ["file_structure", "ruby_rails", "ember", "react", "template", "route", "api", "dependency", "auth_flow", "offline_first_readiness", "database_schema", "business_logic"]
# Patterns to exclude from analysis
exclude_patterns = ["node_modules", "vendor", "dist", "tmp", "log"]
# Patterns to include in analysis
include_patterns = []
# Maximum file size in MB
max_file_size_mb = 10

# Performance configuration
[performance]
parallel_processing = true
enable_caching = true
incremental_analysis = true
cache_dir = ".cache"
max_memory_mb = 2048
timeout_seconds = 7200
"#;

    fs::write(&config_path, config_content).expect("Failed to write test configuration file");

    // Load the configuration
    let config = Config::from_file(&config_path);
    assert!(config.is_ok(), "Failed to load configuration: {:?}", config.err());

    let config = config.unwrap();

    // Check that the configuration was loaded correctly
    assert_eq!(config.general.project_name, "LMS");
    assert_eq!(config.general.output_dir, "docs");
    assert_eq!(config.general.log_level, "info");

    assert_eq!(config.documentation.generate_high_priority, true);
    assert_eq!(config.documentation.generate_medium_priority, true);
    assert_eq!(config.documentation.generate_low_priority, false);

    assert_eq!(config.documentation.high_priority.central_reference_hub, true);
    assert_eq!(config.documentation.high_priority.api_documentation, true);
    assert_eq!(config.documentation.high_priority.implementation_details, true);
    assert_eq!(config.documentation.high_priority.testing_documentation, true);
    assert_eq!(config.documentation.high_priority.tech_debt_report, true);
    assert_eq!(config.documentation.high_priority.summary_report, true);

    assert_eq!(config.documentation.medium_priority.sync_architecture, true);
    assert_eq!(config.documentation.medium_priority.database_architecture, true);

    assert_eq!(config.analysis.analyzers.len(), 12);
    assert_eq!(config.analysis.exclude_patterns.len(), 5);
    assert_eq!(config.analysis.include_patterns.len(), 0);
    assert_eq!(config.analysis.max_file_size_mb, 10);

    assert_eq!(config.performance.parallel_processing, true);
    assert_eq!(config.performance.enable_caching, true);
    assert_eq!(config.performance.incremental_analysis, true);
    assert_eq!(config.performance.cache_dir, ".cache");
    assert_eq!(config.performance.max_memory_mb, 2048);
    assert_eq!(config.performance.timeout_seconds, 7200);
}

#[test]
fn test_config_default() {
    // Load the default configuration
    let config = Config::default();

    // Check that the configuration was loaded correctly
    assert_eq!(config.general.project_name, "Unified Analyzer");
    assert_eq!(config.general.output_dir, "output");
    assert_eq!(config.general.log_level, "info");

    assert_eq!(config.documentation.generate_high_priority, true);
    assert_eq!(config.documentation.generate_medium_priority, false);
    assert_eq!(config.documentation.generate_low_priority, false);

    assert_eq!(config.documentation.high_priority.central_reference_hub, true);
    assert_eq!(config.documentation.high_priority.api_documentation, true);
    assert_eq!(config.documentation.high_priority.implementation_details, true);
    assert_eq!(config.documentation.high_priority.testing_documentation, true);
    assert_eq!(config.documentation.high_priority.tech_debt_report, true);
    assert_eq!(config.documentation.high_priority.summary_report, true);

    assert_eq!(config.documentation.medium_priority.sync_architecture, false);
    assert_eq!(config.documentation.medium_priority.database_architecture, false);

    assert_eq!(config.analysis.analyzers.len(), 12);
    assert_eq!(config.analysis.exclude_patterns.len(), 5);
    assert_eq!(config.analysis.include_patterns.len(), 0);
    assert_eq!(config.analysis.max_file_size_mb, 10);

    assert_eq!(config.performance.parallel_processing, false);
    assert_eq!(config.performance.enable_caching, false);
    assert_eq!(config.performance.incremental_analysis, false);
    assert_eq!(config.performance.cache_dir, ".cache");
    assert_eq!(config.performance.max_memory_mb, 1024);
    assert_eq!(config.performance.timeout_seconds, 3600);
}
