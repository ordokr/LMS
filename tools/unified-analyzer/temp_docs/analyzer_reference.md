# LMS Project Analyzer Reference

This document serves as a reference for the unified analyzer system that generates documentation and insights for the LMS project.

## Overview

The LMS Project Analyzer is a unified system that analyzes the codebase, generates documentation, and provides insights into the project's structure, progress, and quality. It consolidates various analyzers that were previously scattered throughout the codebase.

## Analyzer Components

### Core Components

- **UnifiedAnalyzer**: The main analyzer that orchestrates the analysis process.
- **AnalyzerConfig**: Configuration for the analyzer, loaded from `analyzer_config.toml`.
- **AnalysisResult**: The result of the analysis, containing various metrics and insights.

### Generators

- **CentralHubGenerator**: Generates the central reference hub (`docs/central_reference_hub.md`).
- **ArchitectureDocGenerator**: Generates architecture documentation (`docs/architecture/overview.md`).
- **ModelsDocGenerator**: Generates models documentation (`docs/models/overview.md`).
- **ApiDocGenerator**: Generates API documentation (`docs/api/reference.md`).
- **TechDebtReportGenerator**: Generates technical debt report (`docs/technical_debt_report.md`).
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
