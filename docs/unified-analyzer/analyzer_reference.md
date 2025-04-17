# LMS Project Analyzer Reference

This document serves as a reference for the unified analyzer system that generates documentation and insights for the LMS project.

## Overview

The LMS Project Analyzer is a unified system that analyzes the codebase, generates documentation, and provides insights into the project's structure, progress, and quality. It consolidates various analyzers that were previously scattered throughout the codebase.

## Analyzer Components

### Core Components

- **UnifiedAnalyzer**: The main analyzer that orchestrates the analysis process.
- **AnalyzerConfig**: Configuration for the analyzer, loaded from `analyzer_config.toml`.
- **AnalysisResult**: The result of the analysis, containing various metrics and insights.

### Analyzers

- **ModelAnalyzer**: Analyzes models in the codebase.
- **ApiAnalyzer**: Analyzes API endpoints.
- **UiAnalyzer**: Analyzes UI components.
- **CodeQualityAnalyzer**: Analyzes code quality metrics.
- **TestAnalyzer**: Analyzes tests and test coverage.
- **IntegrationAnalyzer**: Analyzes integration points.
- **ArchitectureAnalyzer**: Analyzes architecture patterns.
- **SyncAnalyzer**: Analyzes sync system.
- **BlockchainAnalyzer**: Analyzes blockchain components.

## Usage

### Command-Line Interface

The analyzer can be run using the following command:

```bash
cd tools/unified-analyzer
cargo run
```

### Options

- `--path PATH`: Specify the path to analyze (default: current directory)
- `--output DIR`: Specify the output directory for documentation (default: docs)
- `--verbose`: Enable verbose output

## Generated Documentation

The analyzer generates the following documentation:

- **Central Reference Hub** (`docs/central_reference_hub.md`): The main entry point for project documentation.
- **Analyzer Reference** (`docs/analyzer_reference.md`): Documentation for the analyzer itself.
- **Architecture Documentation** (`docs/architecture/overview.md`): Overview of the project's architecture.
- **Models Documentation** (`docs/models/overview.md`): Documentation for the project's data models.
- **Integration Documentation** (`docs/integration/overview.md`): Documentation for integration points.

