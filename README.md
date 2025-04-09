# Canvas-Discourse LMS Integration Project

## Overview

This project integrates and adapts functionality from Canvas LMS and Discourse forum into a unified desktop application built with Tauri (Rust) and Leptos.

## Documentation

All project documentation is automatically generated and maintained in the `docs` directory:

- **Central Reference Hub**: [docs/central_reference_hub.md](docs/central_reference_hub.md)
- **Technical Documentation**: [docs/technical](docs/technical)
- **Port Documentation**: [docs/port](docs/port)
- **Integration Knowledge Base**: [rag_knowledge_base/integration](rag_knowledge_base/integration)

## Analysis & Visualization Tools

The project includes comprehensive tools for analyzing, documenting, and visualizing the integration:

- **Master Orchestration**: `analyze.rs` - Unified Rust analysis tool
- **Conflict Detection**: `port-conflict-analyzer.js` - Identifies conflicts between source and target code
- **Technical Documentation**: `technical-docs-generator.js` - Generates documentation from code
- **Summary Report**: `summary-report-generator.js` - Provides an executive summary
- **Visual Dashboard**: `visual-dashboard-generator.js` - Creates an interactive HTML dashboard

## Getting Started

1. Clone the repository
2. Install dependencies:

```bash
npm install
```

## 👋 Welcome to the Project!

This project implements integration between Canvas LMS and Discourse forum systems, combining the best of both platforms for educational institutions.

## 🤖 For AI Assistants

If you're an AI assistant (like GitHub Copilot) helping with this project, please refer to the [`AI_PROJECT_GUIDE.md`](AI_PROJECT_GUIDE.md) file for a comprehensive guide to the project structure, documentation, and development workflow.

This guide will help you understand:
- Where to find documentation
- How the code is structured
- The current development priorities
- How to contribute to the feedback loop between code and documentation

## 👥 For Developers

This project uses a documentation-driven development approach with automated analysis tools that generate comprehensive documentation from source code.

To get started:

1. Review the central reference hub at [`docs/central_reference_hub.md`](docs/central_reference_hub.md)
2. Set up your development environment following our [setup guide](docs/setup_guide.md)
3. Run `node cli.js analyze` to see the current project status
4. Check the [implementation plan](rag_knowledge_base/integration/status_and_plan.md) for current priorities

EduConnect LMS & Forum
An offline-first learning management system with integrated discussion forums, built with Rust and modern web technologies.

<img alt="Status: Early Development" src="https://img.shields.io/badge/status-early development-orange">
🚀 Overview
EduConnect is a modern learning management system that prioritizes offline-first functionality, enabling education to continue even in environments with limited or intermittent connectivity. It combines robust course management with integrated discussion forums to create a comprehensive learning ecosystem.

Built with Tauri (Rust) for the backend and modern web technologies for the frontend, EduConnect offers a responsive, cross-platform experience while maintaining strong security, performance, and offline capabilities.

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
Frontend: Modern Web Technologies (React/TypeScript)
Sync Engine: Custom-built conflict resolution system
Packaging: Native applications for Windows, macOS, and Linux
🏗️ Project Status
EduConnect is currently in early development phase:

✅ Core data models (100% complete)
⏳ API layer (0% implemented)
🚧 UI components (50% implemented)
🧪 Test coverage (6%)
See the generated project_status.md for detailed implementation metrics.

🚀 Getting Started
Prerequisites
Rust (latest stable)
Node.js (v16 or later)
Tauri CLI
Installation & Development
Building for Production
📐 Architecture
EduConnect follows a multi-layer architecture:

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

EduConnect - Connecting learners everywhere, with or without internet.

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

