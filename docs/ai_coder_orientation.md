# AI Coder Orientation Guide for Ordo Project

Welcome to the Ordo project! This guide is designed to quickly orient AI coding assistants to the project structure, philosophy, and implementation details. Follow this reading path to gain a comprehensive understanding of the project.

## 🚀 Quick Start Reading Path

1. **Start here**: Read this orientation guide completely
2. **Project overview**: [Central Reference Hub](central_reference_hub.md)
3. **Implementation priorities**: [Next Steps Document](integration-advisor/next_steps.md)
4. **Migration strategy**: [Migration Recommendations](integration-advisor/reports/recommendations.md)
5. **Technical implementation**: [Database Architecture](architecture/database.md) and [Synchronization Architecture](architecture/synchronization.md)

## 📚 Project Overview

**Ordo** is a modern learning management system with integrated forum functionality, built with an offline-first approach. The project aims to combine the best features of Canvas LMS and Discourse into a unified application implemented in Rust and Haskell.

### Core Principles

1. **Offline-First**: All core functionality works without an internet connection
2. **Integrated Experience**: Seamless integration between LMS and forum components
3. **Performance**: Fast, responsive experience even on lower-end hardware
4. **Security**: Strong data protection and privacy controls
5. **Extensibility**: Modular architecture that allows for customization

### Technology Stack

- **Frontend**: Leptos (Rust-based reactive framework)
- **Desktop Shell**: Tauri
- **Backend**: Rust and Haskell ([details](architecture/haskell_integration.md))
- **Database**: Hybrid SQLite/Redb approach
- **ORM**: SQLx for type-safe SQL

## 🏗️ Project Architecture

Ordo follows a modular architecture with clear separation of concerns:

`plaintext
Ordo/
├── src-tauri/         # Rust backend code
│   └── src/
│       ├── api/       # API endpoints
│       ├── core/      # Core business logic
│       ├── db/        # Database interactions
│       ├── models/    # Data models
│       ├── sync/      # Synchronization engine
│       ├── modules/   # App-like modules
│       │   ├── quiz/      # Quiz module
│       │   ├── forum/     # Forum module
│       │   └── gradebook/ # Gradebook module
│       └── extensions/ # Extension system
├── src/               # Frontend code (Leptos)
│   ├── components/    # Reusable UI components
│   ├── pages/         # Application pages
│   ├── models/        # Frontend data models
│   └── services/      # Frontend services
`

### Key Architectural Patterns

1. **Clean Architecture**: Domain-centric design
2. **Offline-First**: Local-first data with synchronization
3. **Domain-Driven Design**: Focus on core domain logic
4. **Event Sourcing**: For reliable synchronization
5. **CQRS**: Separates read and write operations

## 💾 Database Architecture

Ordo uses a hybrid storage approach:

1. **SQLite with SQLx**: For structured domain data
   - Courses, assignments, users, discussions
   - Persistent, structured data with relationships

2. **Redb (Rust Embedded Database)**: For ephemeral state and sync metadata
   - Draft content, sync operations queue
   - Session state, real-time subscriptions
   - Offline queue handling

## 🔄 Synchronization Strategy

The sync engine handles data synchronization between local and remote databases:

1. **Operation-Based Sync**: Records all changes as operations
2. **Conflict Resolution**: Uses version vectors and domain-specific resolution
3. **Offline Queue**: Stores operations when offline
4. **Background Sync**: Processes queue when connectivity returns

## 🔗 Integration Architecture

Ordo integrates Canvas LMS and Discourse forum functionality:

1. **Entity Mapping**: Maps entities between source systems and Ordo
2. **Feature Mapping**: Identifies and implements features from source systems
3. **Migration Strategy**: Prioritizes modules for migration to Rust/Haskell

## 📊 Current Project Status

- **Phase**: Early development
- **Implementation Progress**: See [Central Reference Hub](central_reference_hub.md) for latest status
- **Current Focus**: Database implementation, sync engine, and UI components

## 🛠️ Development Guidelines

1. **Type Safety**: Use strong typing throughout the codebase
2. **Error Handling**: Use Result types for error propagation
3. **Documentation**: Document all public APIs and complex functions
4. **Testing**: Write unit tests for all business logic
5. **Offline Support**: Design all features with offline-first in mind

## 📝 Implementation Priorities

Current development priorities are:

1. **Database**: Implement transaction handling for Redb integration
2. **Sync Engine**: Add version vector conflict resolution
3. **UI**: Complete course listing components
4. **API**: Define core API contracts
5. **Migration**: Migrate key modules from Canvas and Discourse to Rust (see [MODEL_CONTROLLER_MIGRATION.md](project/MODEL_CONTROLLER_MIGRATION.md))

See the [Next Steps Document](integration-advisor/next_steps.md) for detailed priorities.

## 🔍 Key Implementation Details

### Hybrid Storage Example

`
ust
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
    }
}
`

### Sync Engine Example

`
ust
pub struct SyncEngine {
    sqlite_pool: SqlitePool,
    redb: Database,
    sync_state: Arc<RwLock<SyncState>>,
}

impl SyncEngine {
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
`

## 📚 Additional Resources

For more detailed information, refer to these resources:

1. **Architecture Documentation**: [Architecture Overview](architecture/overview.md)
2. **Database Documentation**: [Database Architecture](architecture/database.md)
3. **Sync Engine Documentation**: [Synchronization Architecture](architecture/synchronization.md)
4. **API Documentation**: [API Overview](api/overview.md)
5. **UI Components**: [UI Overview](ui/overview.md)
6. **Integration Documentation**: [Integration Overview](integration/overview.md)
7. **Migration Recommendations**: [Recommendations](integration-advisor/reports/recommendations.md)
8. **Feature Mappings**: [Feature Mappings](integration-advisor/reports/feature_mappings.md)

## 🤖 AI-Specific Guidance

As an AI coding assistant working on this project, keep these points in mind:

1. **Prioritize Offline-First**: Always consider how features will work without an internet connection
2. **Follow Rust/Haskell Best Practices**: Use idiomatic patterns for each language
3. **Respect the Architecture**: Maintain clean separation of concerns
4. **Consider Migration Context**: When implementing features, refer to the source systems (Canvas/Discourse) for guidance
5. **Update Documentation**: Ensure documentation is updated when implementing new features
6. **Focus on Current Priorities**: Refer to the Next Steps document for current focus areas

## 🔄 Keeping Up-to-Date

The unified analyzer continuously updates documentation based on codebase analysis. Always refer to the Central Reference Hub for the latest project status and priorities.

---

This orientation guide was last updated: 2025-04-17
