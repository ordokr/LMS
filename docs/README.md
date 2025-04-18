# Ordo Documentation

This directory contains all documentation for the Ordo project. The documentation is organized into the following sections:

## 🤖 AI Coder Documentation

If you're an AI coding assistant working on this project, start with these documents:

- [AI Coder Orientation Guide](ai_coder_orientation.md) - Comprehensive overview of the project
- [AI Coder Consolidated Reference](ai_coder_reference.md) - Links to all key resources

To onboard a new AI coder to the project, run:

```powershell
.\tools\ai_coder_onboarding.ps1
```

## Main Documentation

- [Central Reference Hub](central_reference_hub.md) - The main entry point for all documentation
- [API Documentation](api/reference.md) - Documentation for the API
- [Architecture Documentation](architecture/overview.md) - Documentation for the architecture
- [Modular Architecture](architecture/modular_architecture.md) - Documentation for the modular architecture
- [Module Categories](architecture/module_categories.md) - Detailed implementation plans for module categories
- [Models Documentation](models/overview.md) - Documentation for the data models
- [Modules Documentation](modules/overview.md) - Documentation for application modules
- [Integration Documentation](integration/overview.md) - Documentation for integration points
- [Security Documentation](security/implementation.md) - Documentation for security implementation
- [UI Components](ui/overview.md) - Overview of UI components
- [UI Component Strategy](ui/component_strategy.md) - Documentation for UI component implementation
- [Sync Engine Implementation](technical/sync_engine_implementation.md) - Documentation for sync engine implementation
- [Background Job System](technical/background_job_system.md) - Documentation for background job system
- [Technical Documentation](technical/overview.md) - Technical documentation

## Visualizations

- [API Map](visualizations/api_map/api_map.html) - Visualization of the API
- [Component Tree](visualizations/component_tree/component_tree.html) - Visualization of the component hierarchy
- [Database Schema](visualizations/db_schema/db_schema.html) - Visualization of the database schema
- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html) - Visualization of the migration roadmap

## Analysis Results

- [Canvas Analysis](analysis/canvas/analysis.md) - Analysis of the Canvas codebase
- [Discourse Analysis](analysis/discourse/analysis.md) - Analysis of the Discourse codebase
- [Conflict Analysis](analysis/conflicts/conflicts.md) - Analysis of conflicts between Canvas and Discourse

## RAG Knowledge Base

- [Canvas Knowledge Base](rag_knowledge_base/canvas/README.md) - Knowledge base for Canvas
- [Discourse Knowledge Base](rag_knowledge_base/discourse/README.md) - Knowledge base for Discourse
- [Integration Knowledge Base](rag_knowledge_base/integration/README.md) - Knowledge base for integration

## Development Guides

- [Development Setup](development/setup.md) - Guide for setting up the development environment
- [Coding Standards](development/coding_standards.md) - Coding standards for the project
- [Dependency Management](development/dependency_management.md) - Guidelines for managing dependencies
- [Contribution Guidelines](development/contribution.md) - Guidelines for contributing to the project

## Documentation Generation

All documentation is automatically generated from the codebase analysis. The documentation is updated whenever the codebase is analyzed.

To regenerate the documentation, run:

```bash
cd tools/unified-analyzer
cargo run --bin unified-analyzer -- update-hub --path /path/to/project
```

To update AI coder documentation specifically, run:

```powershell
.\tools\update_ai_documentation.ps1
```

This script will:
1. Run the unified analyzer to generate up-to-date reports
2. Copy reports to the docs directory
3. Update the AI coder orientation guide
4. Create a consolidated reference document
