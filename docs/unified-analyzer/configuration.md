# Configuration Guide

The Unified Analyzer can be configured using the `config.toml` file in the root directory of the project.

## General Configuration

```toml
# General configuration
[general]
# Output directory for reports and visualizations
output_dir = "output"
# Log level (debug, info, warn, error)
log_level = "info"
# Whether to include timestamps in logs
log_timestamps = true
```

## Paths Configuration

```toml
# Paths configuration
[paths]
# Path to the LMS codebase
lms_path = "C:\\Users\\Tim\\Desktop\\LMS"
# Path to the Canvas codebase
canvas_path = "C:\\Users\\Tim\\Desktop\\port\\canvas"
# Path to the Discourse codebase
discourse_path = "C:\\Users\\Tim\\Desktop\\port\\discourse"
```

## Analysis Configuration

```toml
# Analysis options
[analysis]
# Analyzers to use
analyzers = ["file_structure", "ruby_rails", "ember", "react", "template", "route", "api", "dependency", "auth_flow", "offline_first_readiness", "database_schema", "business_logic", "canvas", "discourse"]
# Maximum depth to search for files
max_depth = 10
# File extensions to include in analysis
include_extensions = [".rs", ".js", ".ts", ".jsx", ".tsx", ".html", ".css", ".scss", ".md", ".toml", ".json"]
# Directories to exclude from analysis
exclude_dirs = ["node_modules", "target", "dist", "build", ".git"]
```

## Integration Advisor Configuration

```toml
# Integration Advisor configuration
[analysis.integration_advisor]
enabled = true
entity_mapping = true
feature_detection = true
code_quality = true
conflict_detection = true
integration_tracking = true
recommendations = true

# Entity Mapping configuration
[analysis.entity_mapping]
similarity_threshold = 0.5
exact_match_bonus = 0.3
field_match_weight = 0.7
name_match_weight = 0.3

# Feature Detection configuration
[analysis.feature_detection]
categories = ["course_mgmt", "assignment_mgmt", "grading", "discussions", "auth", "roles", "moderation", "tagging"]
controller_weight = 0.5
route_weight = 0.3
view_weight = 0.2

# Code Quality configuration
[analysis.code_quality]
usefulness_threshold_high = 80
usefulness_threshold_medium = 50
complexity_weight = 0.4
loc_weight = 0.2
comment_coverage_weight = 0.2
cohesion_weight = 0.2

# Conflict Detection configuration
[analysis.conflict_detection]
naming_conflict_threshold = 0.8
semantic_conflict_threshold = 0.6

# Integration Tracking configuration
[analysis.integration_tracking]
entity_weight = 0.5
feature_weight = 0.5

# Recommendations configuration
[analysis.recommendations]
max_recommendations = 20
high_priority_threshold = 4
medium_priority_threshold = 2
```

## Visualization Configuration

```toml
# Visualization options
[visualization]
# Whether to generate visualizations
enabled = true
# Format for visualizations (svg, png, pdf)
format = "svg"
# Theme for visualizations (light, dark)
theme = "light"
# Whether to include labels in visualizations
include_labels = true
# Maximum number of nodes to include in visualizations
max_nodes = 100
```

## Report Configuration

```toml
# Report options
[report]
# Whether to generate reports
enabled = true
# Format for reports (markdown, html, json)
format = "markdown"
# Whether to include timestamps in reports
include_timestamps = true
# Whether to include source code snippets in reports
include_snippets = true
# Maximum number of items to include in reports
max_items = 100
```

## Advanced Configuration

```toml
# Advanced options
[advanced]
# Whether to use parallel processing
parallel = true
# Number of threads to use for parallel processing
threads = 4
# Whether to cache analysis results
cache = true
# Cache directory
cache_dir = ".cache"
# Whether to use incremental analysis
incremental = true
# Whether to use AI-assisted analysis
ai_assisted = true
```

## Environment Variables

The following environment variables can be used to override configuration values:

- `UNIFIED_ANALYZER_OUTPUT_DIR`: Override the output directory
- `UNIFIED_ANALYZER_LOG_LEVEL`: Override the log level
- `UNIFIED_ANALYZER_LMS_PATH`: Override the LMS path
- `UNIFIED_ANALYZER_CANVAS_PATH`: Override the Canvas path
- `UNIFIED_ANALYZER_DISCOURSE_PATH`: Override the Discourse path
