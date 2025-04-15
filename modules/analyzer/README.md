# LMS Project Analyzer

A unified analyzer system for the LMS project that analyzes the codebase, generates documentation, and provides insights into the project's structure, progress, and quality.

## Overview

The LMS Project Analyzer is designed to be a standalone tool that can analyze the LMS codebase without being tightly coupled to the application itself. It provides a comprehensive set of analyzers and report generators that can be used to understand the project's structure, track its progress, and identify areas for improvement.

## Features

- **Unified Analysis**: Consolidates various analyzers that were previously scattered throughout the codebase.
- **Comprehensive Reports**: Generates detailed reports on various aspects of the codebase.
- **Modular Design**: Each analyzer and report generator is designed as a separate module that can be used independently.
- **Extensible**: New analyzers and report generators can be easily added to the system.

## Components

### Core Components

- **UnifiedAnalyzer**: The main analyzer that orchestrates the analysis process.
- **ProjectAnalyzer**: Analyzes project structure, models, components, and routes.
- **AnalyzerConfig**: Configuration for the analyzer, loaded from `analyzer_config.toml`.
- **AnalysisResult**: The result of the analysis, containing various metrics and insights.
- **TechDebtAnalyzer**: Analyzes technical debt in the codebase.
- **CodeQualityAnalyzer**: Analyzes code quality in the codebase.
- **ModelAnalyzer**: Analyzes data models in the codebase.

### Generators

- **CentralHubGenerator**: Generates the central reference hub (`docs/central_reference_hub.md`).
- **ArchitectureDocGenerator**: Generates architecture documentation (`docs/architecture/overview.md`).
- **ModelsDocGenerator**: Generates models documentation (`docs/models/overview.md`).
- **ApiDocGenerator**: Generates API documentation (`docs/api/reference.md`).
- **ProjectDocGenerator**: Generates comprehensive project documentation using the ProjectAnalyzer.
- **TechDebtReportGenerator**: Generates technical debt report (`docs/technical_debt_report.md`).
- **CodeQualityReportGenerator**: Generates code quality report (`docs/quality/code_quality_report.md`).
- **ModelReportGenerator**: Generates model report (`docs/models/model_report.md`).
- **DashboardGenerator**: Generates a visual dashboard (`docs/dashboard.html`).

### Runners

- **AnalysisRunner**: Runs the analysis with various configurations.

### Utilities

- **FileSystemUtils**: Utilities for file system operations.

## Usage

### Command-Line Interface

The analyzer can be run using the `unified-analyze.bat` script:

```batch
unified-analyze.bat [command] [options]
```

#### Commands

- `--full`: Run complete analysis
- `--quick`: Run quick analysis (default)
- `--update-hub`: Update central reference hub
- `--summary`: Generate summary report
- `--update-rag`: Update RAG knowledge base
- `--project`: Run project analyzer and generate comprehensive documentation

#### Options for `--full` command

- `--target-dirs DIR1,DIR2,...`: Target directories to analyze
- `--exclude PAT1,PAT2,...`: Patterns to exclude
- `--output DIR`: Output directory for documentation
- `--rag`: Update RAG knowledge base
- `--ai`: Generate AI insights
- `--js`: Analyze JavaScript files for Rust migration
- `--dashboard`: Generate visual dashboard
- `--tech-debt`: Analyze technical debt

### Configuration

The analyzer is configured using the `analyzer_config.toml` file in the project root. This file contains settings for the technology stack, architecture principles, and integration configurations.

## Generated Documentation

The analyzer generates the following documentation:

- **Central Reference Hub** (`docs/central_reference_hub.md`): The main entry point for project documentation.
- **Architecture Documentation** (`docs/architecture/overview.md`): Overview of the project's architecture.
- **Models Documentation** (`docs/models/overview.md`): Documentation for the project's data models.
- **API Documentation** (`docs/api/reference.md`): Documentation for the project's API endpoints.
- **Technical Debt Report** (`docs/technical_debt_report.md`): Report on technical debt in the project.
- **Code Quality Report** (`docs/quality/code_quality_report.md`): Report on code quality in the project.
- **Model Report** (`docs/models/model_report.md`): Report on data models in the project.
- **Dashboard** (`docs/dashboard.html`): Visual dashboard of project metrics.
- **Summary Report** (`docs/SUMMARY_REPORT.md`): Summary of the project's status.

## Iterative Development Process

The analyzer is designed to support an iterative development process:

1. **Analyze**: Run the analyzer to gather insights about the codebase.
2. **Report**: Generate documentation and reports.
3. **Read**: Review the generated documentation and reports.
4. **Build**: Implement changes based on the insights.
5. **Repeat**: Run the analyzer again to track progress.

This iterative process ensures that the project's documentation is always up-to-date and that development is guided by data-driven insights.

## Development

### Adding a New Analyzer

To add a new analyzer:

1. Create a new file in the `core` directory (e.g., `new_analyzer.rs`).
2. Implement the analyzer with appropriate methods for analyzing the codebase.
3. Update the `core/mod.rs` file to include the new analyzer.
4. Create a new report generator in the `generators` directory.
5. Update the `generators/mod.rs` file to include the new report generator.
6. Update the `generators/report_generator.rs` file to include the new report generator in the `generate_reports` function.

### Adding a New Report Generator

To add a new report generator:

1. Create a new file in the `generators` directory (e.g., `new_report_generator.rs`).
2. Implement the report generator with appropriate methods for generating reports.
3. Update the `generators/mod.rs` file to include the new report generator.
4. Update the `generators/report_generator.rs` file to include the new report generator in the `generate_reports` function.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
