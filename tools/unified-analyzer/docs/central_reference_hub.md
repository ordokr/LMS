# Ordo Central Reference Hub

> **Important Disclaimer:** All references to “integration,” “migration,” and “feature mapping” in this document and the unified analyzer refer solely to source code, schema, or feature porting. The unified analyzer does **not** support or perform data migration, user import, or live system integration. All analysis and recommendations are for codebase transformation and feature parity only.

This document serves as the central reference for the Ordo project.

## 🎯 Project Vision

Ordo is a modular, offline-first Learning Management System (LMS) and Forum built with Rust, Tauri, and Leptos. It aims to provide a seamless educational experience even in environments with limited connectivity.

## 🏗️ Architecture Overview

Ordo follows a client-server architecture with offline-first capabilities:

- **Frontend**: Leptos (Rust-based reactive framework)
- **Backend**: Rust with Axum/Actix Web
- **Desktop Wrapper**: Tauri
- **Database**: SQLite with SQLx ORM
- **Sync Engine**: Custom Rust implementation

## 🔍 Integration Advisor Findings

### Integration Progress

- Overall integration: 25.0%
- Entity integration: 0.0%
- Feature integration: 50.0%

**Integration by Category:**

- discussions: 100.0%
- auth: 100.0%
- roles: 100.0%
- moderation: 100.0%
- tagging: 100.0%

- [Detailed integration progress report](integration-advisor/reports/integration_progress.md)

### Key Recommendations

- **Refactor Low-Quality File: C:\Users\Tim\Desktop\port\canvas\spec\lib\user_comm_screener_spec.rb**: File has low code quality (score: 49). It has 328 lines of code, complexity of 40, and comment coverage of 0.3%.
- **Refactor Low-Quality File: C:\Users\Tim\Desktop\port\discourse\plugins\chat\spec\services\chat\update_message_spec.rb**: File has low code quality (score: 30). It has 1056 lines of code, complexity of 59, and comment coverage of 0.2%.
- **Refactor Low-Quality File: C:\Users\Tim\Desktop\port\canvas\spec\requests\admin\badges_controller_spec.rb**: File has low code quality (score: 41). It has 623 lines of code, complexity of 36, and comment coverage of 0.2%.
- **Refactor Low-Quality File: C:\Users\Tim\Desktop\port\discourse\app\assets\javascripts\custom-proxy\index.js**: File has low code quality (score: 44). It has 329 lines of code, complexity of 54, and comment coverage of 4.3%.

- [Full recommendations report](integration-advisor/reports/recommendations.md)
- [Next steps](integration-advisor/next_steps.md)

### Feature Mapping Status

- Canvas features: 385
- Discourse features: 385
- Ordo features: 385
- [Detailed feature mapping report](integration-advisor/reports/feature_mappings.md)

### Code Quality Summary

- Files recommended for reuse: 8230
- Files recommended for refactoring: 7566
- Files recommended for rebuilding: 767
- [Detailed code quality report](integration-advisor/reports/code_quality.md)

### Conflict Analysis

- Total conflicts detected: 0
- Naming conflicts: 0
- Field conflicts: 0
- Semantic conflicts: 0
- [Detailed conflict report](integration-advisor/reports/conflicts.md)

## 📍 Implementation Priorities

1. **Core LMS Features**
   - Course management
   - Assignment submission
   - Grading system
   - Calendar and scheduling

2. **Forum Integration**
   - Discussion boards
   - Q&A functionality
   - Notifications

3. **Offline Capabilities**
   - Local-first data storage
   - Sync when online
   - Conflict resolution

4. **Security & Privacy**
   - End-to-end encryption
   - Data sovereignty
   - Privacy by design

## 🔄 Porting Strategy (formerly Migration Strategy)

The porting from Canvas and Discourse to Ordo will follow these steps (source-to-source/code/feature only):

1. Analyze existing codebases
2. Extract core functionality
3. Reimplement in Rust
4. Test for feature parity
5. Enhance with offline capabilities

## 📊 Progress Tracking

- **Core LMS Features**: 15% complete
- **Forum Integration**: 10% complete
- **Offline Capabilities**: 5% complete
- **Security & Privacy**: 20% complete

## 📚 Documentation

- [Architecture Documentation](architecture.md)
- [API Documentation](api.md)
- [Database Schema](database_schema.md)
- [Offline Sync Protocol](offline_sync.md)

## 🧪 Testing Strategy

- Unit tests for all core functionality
- Integration tests for system components
- End-to-end tests for critical user flows
- Offline capability tests

## 📅 Roadmap

### Phase 1: Foundation (Current)
- Core architecture setup
- Basic course management
- Simple forum functionality
- Local data storage

### Phase 2: Feature Parity
- Complete LMS feature set
- Full forum integration
- Basic offline capabilities
- Initial security implementation

### Phase 3: Enhancement
- Advanced offline-first features
- Comprehensive security model
- Performance optimization
- Extended platform support


## Visualizations

- [Database Schema](visualizations/db_schema/db_schema.html) - Interactive visualization of the database schema
- [Database Schema Documentation](models/database_schema.md) - Comprehensive documentation of the database schema
