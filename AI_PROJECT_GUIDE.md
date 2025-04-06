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
└── docs/              # Generated documentation
