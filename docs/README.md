# Ordo Documentation

This directory contains all documentation for the Ordo project. The documentation is organized into the following sections:

## Main Documentation

- [Central Reference Hub](central_reference_hub.md) - The main entry point for all documentation
- [API Documentation](api/reference.md) - Documentation for the API
- [Architecture Documentation](architecture/overview.md) - Documentation for the architecture
- [Models Documentation](models/overview.md) - Documentation for the data models
- [Integration Documentation](integration/overview.md) - Documentation for integration points
- [Security Documentation](security/implementation.md) - Documentation for security implementation
- [UI Components](ui/overview.md) - Overview of UI components
- [UI Component Strategy](ui/component_strategy.md) - Documentation for UI component implementation
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
- [Contribution Guidelines](development/contribution.md) - Guidelines for contributing to the project

## Documentation Generation

All documentation is automatically generated from the codebase analysis. The documentation is updated whenever the codebase is analyzed.

To regenerate the documentation, run:

```bash
cd tools/unified-analyzer
cargo run --bin unified-analyzer -- update-hub --path /path/to/project
```

This will analyze the codebase and generate updated documentation.
