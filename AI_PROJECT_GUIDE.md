# Canvas-Discourse LMS Integration AI Guide

## Introduction for AI Assistants

This document is the primary entry point for AI assistants working on the Canvas-Discourse LMS integration project. Start here to understand the project structure, documentation resources, and current development priorities.

## Documentation Resources

### Primary Documentation (Start Here)
- **Central Reference Hub**: [docs/central_reference_hub.md](docs/central_reference_hub.md)  
  The one source of truth for all project components and integration status.
- **Integration Reference**: [docs/canvas_discourse_integration.md](docs/canvas_discourse_integration.md)  
  Comprehensive guide to the Canvas-Discourse integration.

### Technical Implementation
- **Technical Implementation Details**: [rag_knowledge_base/integration/technical_implementation.md](rag_knowledge_base/integration/technical_implementation.md)  
  Automatically generated documentation from the source code.
- **Implementation Status**: [rag_knowledge_base/integration/status_and_plan.md](rag_knowledge_base/integration/status_and_plan.md)  
  Current implementation status and roadmap.

### Project Architecture
- **System Architecture**: [docs/system_architecture.md](docs/system_architecture.md)  
  Overview of the system design and components.
- **Relationship Map**: [docs/relationship_map.md](docs/relationship_map.md)  
  Diagram of entity relationships across systems.

### Source Code Port
- **Port Status**: [docs/port/porting_status_dashboard.md](docs/port/porting_status_dashboard.md)  
  Current status of the Canvas and Discourse port.
- **Port Strategy**: [docs/port/porting_strategy.md](docs/port/porting_strategy.md)  
  Strategy for porting Canvas and Discourse functionality.
- **Model Mapping**: [docs/port/model_mapping.md](docs/port/model_mapping.md)  
  Mapping of data models between systems.
- **Integration Challenges**: [docs/port/integration_challenges.md](docs/port/integration_challenges.md)  
  Challenges and solutions for the integration.

## Blockchain Capabilities

The LMS now includes a blockchain implementation for academic record integrity. When assisting with this project, remember:

1. **Memory Efficiency**: The blockchain implementation uses zero-copy deserialization and stack allocation for performance-critical paths. Prioritize memory-efficient code patterns.

2. **Sync Priorities**: Different academic records have different synchronization priorities:
   - Critical: Grades, certificates, exam results
   - High: Course completions, badges
   - Background: Forum posts, profile updates

3. **Transaction Processing**: The system uses context-aware batching to optimize transaction processing. Consider this when making recommendations for data persistence.

4. **Resource Limits**: The blockchain subsystem enforces memory and CPU budgets. Be mindful of resource usage when suggesting improvements.

5. **Performance Metrics**: The blockchain tracks performance metrics. When suggesting optimizations, consider how to measure their impact.

## Analysis Tools

The project includes several analysis tools that provide insights into the code:

1. **Unified Project Analyzer**: `src-tauri/src/analyzers/unified_analyzer.rs`  
   Main tool for analyzing project components and generating documentation.

2. **Analysis Runner**: `src-tauri/src/analyzers/analysis_runner.rs`  
   Comprehensive analysis system that executes the analysis process.

3. **Analysis Commands**: `src-tauri/src/analyzers/analysis_commands.rs`  
   Command interface for running analysis from Tauri or CLI.

4. **Gemini AI Analyzer**: `src-tauri/src/ai/gemini_analyzer.rs`  
   AI integration for generating insights about the codebase.

To update all documentation based on the current state of the project, run:
```bash
cargo run --bin analyze full
```

## Project Structure

```plaintext
LMS/
├── src-tauri/         # Rust backend code
│   └── src/
│       ├── api/       # API endpoints
│       ├── core/      # Core business logic
│       ├── db/        # Database interactions
│       └── models/    # Data models
├── src/               # Frontend code
├── services/          # Integration services
│   └── integration/   # Canvas-Discourse integration
├── rag_knowledge_base/ # RAG documentation
│   └── integration/   # Integration-specific docs
├── docs/              # Generated documentation
│   ├── port/          # Port documentation
│   └── technical/     # Technical documentation
└── analysis_summary/  # Analysis results
    └── conflicts/     # Port conflict analysis
```

## Known Issues with Source Code Ports

When working with this project, be aware of these common issues between the source and target systems:

1. **Model Duplication**: Multiple definitions of core models may exist due to the porting process
   - Example: Both Canvas and Discourse have their own User models
   - Location: `src-tauri/src/models/user.rs` vs. imported models

2. **API Path Conflicts**: Watch for overlapping API paths between Canvas and Discourse endpoints
   - Example: Both systems have `/api/v1/users/:id` endpoints with different behaviors
   - Resolution: Our unified API uses namespaced endpoints

3. **Authentication Differences**: The systems use different authentication approaches being unified
   - Canvas uses OAuth 2.0
   - Discourse uses SSO/API key authentication
   - Our system uses unified JWT authentication

4. **Naming Inconsistencies**: Variable naming conventions differ between ported components
   - Canvas uses snake_case
   - Discourse uses camelCase in some JavaScript components
   - Our system standardizes on snake_case for backend, camelCase for frontend

Refer to the [Port Conflicts Report](analysis_summary/conflicts/port_conflicts.md) for details on known issues.

## Analysis and Documentation Pipeline

The project maintains comprehensive documentation through an automated analysis pipeline:

1. **Data Flow**: Source code → Analysis tools → Documentation generation
   ```mermaid
   flowchart LR
     A[Source Code] --> B[Code Analysis]
     B --> C[Conflict Detection]
     C --> D[Documentation Generation]
     D --> E[Central Reference Hub]


