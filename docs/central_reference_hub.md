# Ordo & Forum: Central Reference Hub

_Last updated: 2025-04-18_

<img alt="Status: Early Development" src="https://img.shields.io/badge/status-early%20development-orange">

## 🚀 Project Vision & Mission

**Ordo** is a modern learning management system that prioritizes offline-first functionality, enabling education to continue even in environments with limited or intermittent connectivity. It combines robust course management with integrated discussion forums to create a comprehensive learning ecosystem.

### Core Principles

1. **Offline-First**: All core functionality works without an internet connection
2. **Integrated Experience**: Seamless integration between LMS and forum components
3. **Performance**: Fast, responsive experience even on lower-end hardware
4. **Security**: Strong data protection and privacy controls
5. **Extensibility**: Modular architecture that allows for customization

### Project Goals

- Create a unified application that combines the best features of Canvas LMS and Discourse
- Ensure all functionality works offline with seamless synchronization when connectivity returns
- Provide a native desktop experience with better performance than web-based alternatives
- Implement a modern, intuitive UI that improves upon the original systems
- Build a solid foundation for future extensions and customizations

## 📈 Project Status

- **Phase**: development
- **Completion**: 0.0%
- **Last Active Area**: unknown
- **Last Updated**: 2025-04-18 01:12

### Recent Activity

| Date | Component | Description | Developer |
|------|-----------|-------------|------------|
| 2025-04-18 | Unified Analyzer | Added activity tracking system | Team |
| 2025-04-18 | Database | Fixed SQLite schema issues | Team |
### Implementation Progress

```json
{
  "foundation_complete": true,
  "model_implementation": "0.0%",
  "api_implementation": "0.0%",
  "ui_implementation": "0.0%",
  "test_coverage": "0.0%",
  "technical_debt": "56%",
  "components": {
    "database": {
      "status": "in_progress",
      "completion": "45%",
      "priority": "high"
    },
    "sync_engine": {
      "status": "in_progress",
      "completion": "30%",
      "priority": "high"
    },
    "ui": {
      "status": "early_development",
      "completion": "15%",
      "priority": "medium"
    },
    "api": {
      "status": "planning",
      "completion": "5%",
      "priority": "medium"
    }
  }
}
```

### Component Status

| Component | Status | Completion | Priority | Next Steps |
|-----------|--------|------------|----------|------------|
| Database | In Progress | 45% | High | Implement transaction handling for Redb |
| Sync Engine | In Progress | 30% | High | Add version vector conflict resolution |
| UI | Early Development | 15% | Medium | Complete course listing components |
| API | Planning | 5% | Medium | Define core API contracts |

## 🔧 Technology Stack

Ordo is built with modern technologies that prioritize performance, security, and offline capabilities:

### Core Technologies

| Layer | Technology | Purpose |
|-------|------------|---------|
| **Frontend** | Leptos (Rust) | Reactive UI framework |
| **UI Styling** | Tailwind CSS | Utility-first CSS framework |
| **Desktop Shell** | Tauri | Native cross-platform wrapper |
| **Backend** | Rust | Performance-critical components |
| **Backend** | Haskell | Type-safe business logic |
| **Database** | SQLite | Local data storage |
| **ORM** | SQLx | Type-safe SQL |
| **Search** | MeiliSearch | Full-text search capabilities |
| **Authentication** | JWT | Secure user authentication |
| **Sync Engine** | Custom Rust | Conflict resolution system |

## 📚 Project Structure

The project follows a modular architecture with clear separation of concerns:

```plaintext
Ordo/
├── src-tauri/         # Rust backend code
│   └── src/
│       ├── api/       # API endpoints
│       ├── core/      # Core business logic
│       ├── db/        # Database interactions
│       ├── models/    # Data models
│       └── sync/      # Synchronization engine
├── src/               # Frontend code (Leptos)
│   ├── components/    # Reusable UI components
│   ├── pages/         # Application pages
│   ├── models/        # Frontend data models
│   └── services/      # Frontend services
├── services/          # Integration services
│   └── integration/   # Canvas-Discourse integration
├── tools/             # Development and analysis tools
│   └── unified-analyzer/ # Codebase analysis tool
├── rag_knowledge_base/ # RAG documentation
│   └── integration/   # Integration-specific docs
├── docs/              # Generated documentation
│   ├── port/          # Port documentation
│   └── technical/     # Technical documentation
└── analysis_summary/  # Analysis results
    └── conflicts/     # Port conflict analysis
```

## 🏗️ Architecture Principles

Ordo follows these key architectural principles:

1. **Clean Architecture**: Clear separation of concerns with domain-centric design
2. **SOLID Principles**: Single responsibility, Open-closed, Liskov substitution, Interface segregation, Dependency inversion
3. **Offline-First**: All core functionality works without an internet connection
4. **Domain-Driven Design**: Focus on core domain logic and bounded contexts
5. **Modular Design**: Components can be developed, tested, and maintained independently

### Design Patterns

The application implements these key design patterns:

- **Repository Pattern**: Abstracts data access logic
- **CQRS**: Separates read and write operations for better scalability
- **Event Sourcing**: Tracks all changes as events for reliable synchronization
- **Factory Pattern**: Creates complex objects with specific configurations
- **Strategy Pattern**: Allows selecting algorithms at runtime
- **Observer Pattern**: For reactive UI updates

## 📊 Code Quality Metrics

Current code quality metrics from static analysis:

| Metric | Value | Target |
|--------|-------|--------|
| **Test Coverage** | 0.0% | >80% |

### Implementation Details

| Component | Implemented | Total | Progress |
|-----------|-------------|-------|----------|
| **Models** | 0 | 0 | 0.0% |
| **API Endpoints** | 0 | 0 | 0.0% |
| **UI Components** | 0 | 0 | 0.0% |

## 🔧 Technical Implementation

This section provides technical details about key components to help developers understand the implementation.

### Hybrid Storage Implementation

Ordo uses a hybrid storage approach combining SQLite and Redb. Here's how they work together:

```rust
// Example: Database module structure
pub mod database {
    pub mod sqlite {
        // SQLite handles structured domain data
        pub async fn init_connection(path: &str) -> Result<SqlitePool> {
            let pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(path)
                .await?;
            Ok(pool)
        }
    }

    pub mod redb {
        // Redb handles ephemeral state and sync metadata
        pub fn open_database(path: &str) -> Result<Database> {
            let db = Database::create(path)?;
            Ok(db)
        }

        // Example: Storing draft content
        pub fn save_draft(db: &Database, user_id: &str, content: &str) -> Result<()> {
            let drafts_table = TableDefinition::<&str, &str>::new("drafts");
            let write_txn = db.begin_write()?;
            {
                let mut table = write_txn.open_table(drafts_table)?;
                table.insert(user_id, content)?;
            }
            write_txn.commit()?;
            Ok(())
        }
    }
}
```

### Sync Engine Implementation

The sync engine handles data synchronization between local and remote databases:

```rust
pub struct SyncEngine {
    sqlite_pool: SqlitePool,
    redb: Database,
    sync_state: Arc<RwLock<SyncState>>,
}

impl SyncEngine {
    // Initialize the sync engine
    pub fn new(sqlite_pool: SqlitePool, redb: Database) -> Self {
        Self {
            sqlite_pool,
            redb,
            sync_state: Arc::new(RwLock::new(SyncState::default())),
        }
    }

    // Queue an operation for sync
    pub async fn queue_operation(&self, operation: SyncOperation) -> Result<()> {
        // Store operation in Redb for durability
        let op_table = TableDefinition::<u64, &[u8]>::new("sync_operations");
        let write_txn = self.redb.begin_write()?;
        let mut table = write_txn.open_table(op_table)?;
        let op_id = self.next_operation_id().await?;
        let serialized = bincode::serialize(&operation)?;
        table.insert(op_id, serialized.as_slice())?;
        write_txn.commit()?;
        Ok(())
    }
}
```

## 🔗 Integration Architecture

Ordo integrates Canvas LMS and Discourse forum functionality into a unified application:

### Integration Status

| Integration | Source | Target | Status |
|-------------|--------|--------|--------|
| Canvas Course Management | Canvas | Ordo | In Progress |
| Canvas Assignments | Canvas | Ordo | In Progress |
| Canvas Discussions | Canvas | Ordo | Planned |
| Discourse Forums | Discourse | Ordo | Planned |
| Discourse User System | Discourse | Ordo | In Progress |
| Blockchain Certification | Native | Ordo | In Progress |

### Integration Strategy

The integration between Canvas and Discourse uses:

1. **Event-Driven Architecture**: For data synchronization
2. **Conflict Resolution**: Source of truth policies based on entity type
3. **Offline-First Capabilities**: Local storage, change tracking, sync queue
4. **Unified Authentication**: Single sign-on across all components
5. **Consistent UI/UX**: Unified design language across all features

### Model Mapping

Key entity mappings between source systems and Ordo:

| Canvas | Discourse | Ordo | Notes |
|--------|-----------|------------|-------|
| Course | Category | Course | One-to-one mapping |
| Course Sections | Sub-categories | CourseSection | Optional |
| Discussion | Topic | Discussion | One-to-one mapping |
| Discussion Entry | Post | DiscussionPost | One-to-one mapping |
| Assignment | - | Assignment | Canvas-only |
| User | User | User | Unified user model |
| - | Tags | Tags | Discourse-only |

## 🛠️ Development Guidelines

### Coding Standards

1. **Type Safety**: Use strong typing throughout the codebase
2. **Error Handling**: Use Result types for error propagation
3. **Documentation**: Document all public APIs and complex functions
4. **Testing**: Write unit tests for all business logic
5. **Naming**: Use descriptive names that reflect domain concepts

### Best Practices

1. **API Responses**: Standardize response format across all endpoints
2. **Database Queries**: Use indexing and prepared statements for optimization
3. **UI Components**: Create reusable components with clear interfaces
4. **State Management**: Use reactive state management patterns
5. **Offline Support**: Design all features with offline-first in mind

## 📑 Documentation

### Generated Documentation

- [Architecture Documentation](architecture/overview.md)
- [Models Documentation](models/overview.md)
- [Integration Documentation](integration/overview.md)
- [API Documentation](api/overview.md)
- [Implementation Details](technical/implementation_details.md)
- [Testing Documentation](technical/tests.md)
- [Technical Debt Report](technical/technical_debt_report.md)
- [Synchronization Architecture](architecture/synchronization.md)
- [Database Architecture](architecture/database.md)

### Visualizations

- [API Map](visualizations/api_map/api_map.html)
- [Component Tree](visualizations/component_tree/component_tree.html)
- [Database Schema](visualizations/db_schema/db_schema.html)
- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html)

### Implementation Guides

- [Implementation Roadmap](integration/roadmap.md)
- [Offline-First Implementation](technical/offline_readiness.md)
- [Authentication Implementation](technical/authentication_implementation.md)
- [Data Synchronization](technical/data_synchronization.md)

### Development Resources

- [Development Environment Setup](development/setup.md)
- [Coding Standards](development/coding_standards.md)
- [Testing Guidelines](development/testing_guidelines.md)
- [Contribution Guidelines](development/contribution.md)

## 🤖 AI-Friendly Metadata

This section contains structured information to help AI coding agents understand the project structure and make informed decisions.

### Component Dependencies

```json
{
  "components": {
    "database": {
      "depends_on": [],
      "used_by": ["models", "sync_engine", "api"]
    },
    "models": {
      "depends_on": ["database"],
      "used_by": ["api", "ui", "sync_engine"]
    },
    "api": {
      "depends_on": ["models", "database"],
      "used_by": ["ui"]
    },
    "ui": {
      "depends_on": ["api", "models"],
      "used_by": []
    },
    "sync_engine": {
      "depends_on": ["database", "models"],
      "used_by": ["api"]
    }
  }
}
```

### Implementation Status

```json
{
  "implemented_features": [
    "database_connection",
    "basic_models",
    "hybrid_storage"
  ],
  "in_progress_features": [
    "sync_engine",
    "api_endpoints",
    "ui_components"
  ],
  "planned_features": [
    "offline_queue",
    "conflict_resolution",
    "authentication"
  ]
}
```

## 📍 Implementation Priorities

Current development focus areas:


### Immediate Next Steps

1. **Database**: Implement transaction handling for Redb integration
2. **Sync Engine**: Add version vector conflict resolution
3. **UI**: Complete course listing components
4. **API**: Define core API contracts
5. **Testing**: Increase test coverage
6. **Documentation**: Improve documentation

## 👋 Conclusion

Ordo represents a significant advancement in learning management systems by prioritizing offline-first capabilities and integrating forum functionality directly into the core platform. By combining the best features of Canvas LMS and Discourse, while addressing their limitations, we're creating a more robust, performant, and accessible educational platform.

This central reference hub will be continuously updated as the project evolves. All documentation is automatically generated from the codebase analysis to ensure it remains accurate and up-to-date.
