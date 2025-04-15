# Unified Analyzer Configuration

This directory contains the configuration module for the Unified Analyzer. The configuration module is responsible for loading and parsing the configuration file.

## Configuration Structure

The configuration is organized as follows:

```
config/
├── mod.rs      # Module definition (in this case, it's src/config.rs)
└── README.md   # This file
```

## Configuration File

The configuration file is a TOML file named `config.toml` and is located in the root directory of the Unified Analyzer. Here's an example configuration file:

```toml
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
```

## Configuration Options

### Output Directories

- `docs_dir`: The main documentation directory
- `api_dir`: The API documentation directory
- `architecture_dir`: The architecture documentation directory
- `models_dir`: The models documentation directory
- `integration_dir`: The integration documentation directory

### Documentation Generation Options

- `generate_high_priority`: Whether to generate high priority documentation
- `generate_medium_priority`: Whether to generate medium priority documentation
- `exclude_ai_content`: Whether to exclude AI/Gemini-related content

### High Priority Documentation

- `central_reference_hub`: Whether to generate the central reference hub
- `api_documentation`: Whether to generate API documentation
- `implementation_details`: Whether to generate implementation details
- `testing_documentation`: Whether to generate testing documentation
- `technical_debt_report`: Whether to generate the technical debt report
- `summary_report`: Whether to generate the summary report

### Medium Priority Documentation

- `synchronization_architecture`: Whether to generate synchronization architecture documentation
- `database_architecture`: Whether to generate database architecture documentation

### Project Information

- `name`: The project name
- `description`: The project description
- `version`: The project version
- `repository`: The project repository

### Analysis Options

- `max_depth`: The maximum depth to search for files
- `include_extensions`: The file extensions to include in the analysis
- `exclude_dirs`: The directories to exclude from the analysis

## Loading Configuration

The configuration is loaded using the `Config::from_file()` function:

```rust
let config = match Config::from_file("config.toml") {
    Ok(config) => {
        println!("Loaded configuration from config.toml");
        config
    },
    Err(e) => {
        println!("Failed to load configuration: {}", e);
        println!("Using default configuration");
        Config::default()
    }
};
```

If the configuration file is not found or cannot be parsed, the default configuration is used.

## Default Configuration

The default configuration is defined in the `Config::default()` function and is the same as the example configuration above.

## Adding New Configuration Options

To add a new configuration option:

1. Add the option to the appropriate struct in `config.rs`
2. Update the default configuration in `Config::default()`
3. Update the configuration file example in this README
4. Update the configuration options documentation in this README
