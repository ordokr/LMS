# Canvas-Discourse Source Code Migration Project

## Overview

This project migrates the source code from Canvas LMS and Discourse forum into a unified desktop application built with Tauri (Rust) and Leptos. The migration focuses exclusively on source code transformation, not data migration from built applications.

## Documentation

All project documentation is automatically generated and maintained in the `docs` directory:

- **Central Reference Hub**: [docs/central_reference_hub.md](docs/central_reference_hub.md)
- **Technical Documentation**: [docs/technical](docs/technical)
- **Port Documentation**: [docs/port](docs/port)
- **Integration Knowledge Base**: [rag_knowledge_base/integration](rag_knowledge_base/integration)

## Source Code Analysis & Migration Tools

The project includes comprehensive tools for analyzing, transforming, and documenting source code:

- **Unified Analyzer**: `tools/unified-analyzer` - Analyzes source code structure and patterns
- **Source Code Extraction**: Extracts models, controllers, and views from Ruby/JavaScript code
- **Code Transformation**: Converts Ruby/JavaScript code to Rust/Haskell
- **Schema Analysis**: Extracts database schema from migration files (no database connection required)
- **Conflict Detection**: `conflict_analyzer.rs` - Identifies conflicts between source and target code
- **Technical Documentation**: `docs_generator.rs` - Generates documentation from code
- **Migration Dashboard**: `dashboard_generator.rs` - Tracks migration progress

## Getting Started


## 👋 Welcome to the Project!

This project implements source code migration from Canvas LMS and Discourse forum systems to a unified Rust/Haskell application, combining the best of both platforms for educational institutions.

## 👥 For Developers

This project uses a documentation-driven development approach with automated analysis tools that extract information from source code and generate migration plans and documentation.

To get started:

1. Review the central reference hub at [`docs/central_reference_hub.md`](docs/central_reference_hub.md)
2. Read the source code migration guide at [`docs/source_code_migration.md`](docs/source_code_migration.md)

Ordo LMS & Forum
An offline-first learning management system with integrated discussion forums, built with Rust and modern web technologies.

<img alt="Status: Early Development" src="https://img.shields.io/badge/status-early development-orange">
🚀 Overview
Ordo is a modern learning management system that prioritizes offline-first functionality, enabling education to continue even in environments with limited or intermittent connectivity. It combines robust course management with integrated discussion forums to create a comprehensive learning ecosystem.

Built with Tauri (Rust) for the backend and modern web technologies for the frontend, Ordo offers a responsive, cross-platform experience while maintaining strong security, performance, and offline capabilities.

✨ Features
Learning Management
📚 Course creation and management
📝 Assignment submission and grading
📊 Student progress tracking
🗂️ Content organization with modules and sections
📅 Course scheduling and deadlines
Discussion Forums
💬 Course-integrated discussion boards
🏷️ Topic categorization and tagging
♾️ Nested replies and conversations
👍 Reaction and rating system
🔍 Full-text search functionality
Offline-First Capabilities
🔄 Seamless sync when connectivity returns
💾 Local data storage and persistence
🚫 No dependency on continuous internet connection
📱 Full functionality in offline mode
🛠️ Technology Stack
Backend: Tauri (Rust) with SQLite for data storage
Frontend: Leptos (Rust) with Tailwind CSS (Standalone CLI)
Sync Engine: Custom-built conflict resolution system
Business Logic: Haskell for complex domain logic
Packaging: Native applications for Windows, macOS, and Linux
🏗️ Project Status
Ordo is currently in early development phase:

✅ Source code analysis tools (100% complete)
🚧 Model migration (50% complete)
⏳ Controller migration (10% complete)
⏳ View migration (5% complete)
🧪 Test coverage (6%)
See the generated project_status.md for detailed implementation metrics.

🚀 Getting Started
Prerequisites
Rust (latest stable)
Tauri CLI
Installation & Development
Building for Production
📐 Architecture
Ordo follows a multi-layer architecture:

📊 Implementation Priorities
Current development focus:

API Layer: Implementing core API endpoints for forum features
UI Components: Building basic UI layout and critical components
Testing: Increasing test coverage for model and API layers
🤝 Contributing
Contributions are welcome! Please feel free to submit pull requests, report bugs, or suggest features.

Fork the repository
Create your feature branch (git checkout -b feature/amazing-feature)
Commit your changes (git commit -m 'Add some amazing feature')
Push to the branch (git push origin feature/amazing-feature)
Open a Pull Request
📝 License
This project is licensed under the MIT License - see the LICENSE file for details.

📧 Contact
Project Lead - Tim Vail

Project Repository: https://github.com/ordo/lms

Ordo - Connecting learners everywhere, with or without internet.

# LMS Application

## Testing

This project includes comprehensive testing to ensure reliability and correctness.

### Running Tests

To run the backend tests:

```bash
npm run test
```

To run tests in watch mode during development:

npm run test:watch

To generate test coverage report:

npm run test:coverage

Test Structure
Unit Tests: Test individual components and functions
Integration Tests: Test interactions between multiple components
End-to-End Tests: Test complete workflows
Frontend Tests
Frontend tests use Leptos testing utilities and wasm-bindgen-test. To run these tests:

cd src
wasm-pack test --chrome

Test Coverage
We aim to maintain at least 80% test coverage for all core functionality. Coverage reports are generated for each PR and can be viewed in the PR comments.

# LMS Integration Platform

## Testing Status

The project now has comprehensive test coverage for core components, with all tests passing:

- **Auth Services**: 100% coverage for JWT service and auth middleware
- **Models**: Full test coverage for Notification model and ~89% for User model
- **Services**: ~79% coverage for notification and integration services
- **Overall Coverage**: ~24% (improved from 15%)

To run tests:
```bash
# Run all tests
npm test

# Run with detection for open handles (for debugging)
npm run test:debug
```

## Project Status

As highlighted in our assessment:
- Models: ~91% complete (core models at ~60% implementation)
- UI Components: ~90% complete
- API Endpoints: ~12% complete (key focus area for development)

Priority areas for development include completing API endpoints and increasing test coverage for remaining components.

