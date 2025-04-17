# Canvas-Discourse LMS Integration AI Guide

## Introduction for AI Assistants

This document is the primary entry point for AI assistants working on the Canvas-Discourse LMS integration project. Start here to understand the project structure, documentation resources, and current development priorities.


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


## Known Issues with Source Code Migrations

When working with this project, be aware of these common issues between the source and target systems:

1. **Model Duplication**: Multiple definitions of core models may exist due to the migration process
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

## Analysis and Documentation Pipeline

The project maintains comprehensive documentation through an automated analysis pipeline:

1. **Data Flow**: Source code → Analysis tools → Documentation generation
   ```mermaid
   flowchart LR
     A[Source Code] --> B[Code Analysis]
     B --> C[Conflict Detection]
     C --> D[Documentation Generation]
     D --> E[Central Reference Hub]


