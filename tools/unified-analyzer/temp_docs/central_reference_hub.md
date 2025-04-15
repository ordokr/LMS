# LMS Project: Central Reference Hub

_Last updated: 2025-04-14T21:24:09.752430900-04:00_

## Project Overview

The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.

## Technology Stack

### Frontend
- Leptos
- Tauri

### Backend
- Rust
- Haskell

### Database
- SQLite
- sqlx

### Search
- MeiliSearch

### AI Integration
- Gemini

### Blockchain
- Custom Rust implementation

### Authentication
- JWT

## Architecture Principles
- Clean Architecture
- SOLID
- Offline-first

## Design Patterns
- CQRS
- Event Sourcing
- Repository Pattern

## Project Statistics

- **Total Files**: 1363
- **Lines of Code**: 295244
- **Rust Files**: 898
- **Haskell Files**: 5

### File Types

| Extension | Count |
|-----------|-------|
| db | 1 |
| bat | 13 |
| ico | 1 |
| png | 16 |
| c | 1 |
| hs | 5 |
| toml | 17 |
| md | 152 |
| yaml | 3 |
| example | 1 |
| code-workspace | 1 |
| bak | 8 |
| h | 1 |
| info | 1 |
| pdb | 1 |
| ts | 6 |
| sh | 2 |
| log | 9 |
| x | 2 |
| css or appropriate CSS file | 1 |
| rb | 1 |
| css | 27 |
| sql | 33 |
| py | 19 |
| wasm | 2 |
| icns | 1 |
| js | 21 |
| y | 2 |
| xml | 1 |
| pyc | 1 |
|  | 1 |
| json | 22 |
| lock | 3 |
| rs | 898 |
| new | 2 |
| ps1 | 9 |
| clean | 1 |
| html | 62 |
| svg | 4 |

## Integration Status

| Integration | Source | Target | Status |
|-------------|--------|--------|--------|
| Canvas Course Management | Canvas | LMS | In Progress |
| Discourse Forums | Discourse | LMS | Planned |
| Blockchain Certification | Native | LMS | In Progress |

## Documentation Links

- [Architecture Documentation](./architecture/overview.md)
- [Models Documentation](./models/overview.md)
- [Integration Documentation](./integration/overview.md)
- [Blockchain Implementation](../rag_knowledge_base/integration/blockchain_implementation.md)

## AI Development Guidance

This project is built with Rust and Haskell as the primary languages. When developing new features or modifying existing ones, adhere to the following principles:

1. **Rust-First Approach**: Implement core functionality in Rust whenever possible.
2. **Functional Paradigm**: Use functional programming patterns, especially for complex business logic.
3. **No JavaScript Dependencies**: Avoid JavaScript/TypeScript dependencies unless absolutely necessary.
4. **Performance Focus**: Prioritize performance in all implementations.
5. **Offline-First**: Design features to work offline by default.
6. **Security**: Implement proper authentication and authorization checks.
