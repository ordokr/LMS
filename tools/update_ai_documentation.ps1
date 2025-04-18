# Update AI Documentation Script for Ordo Project
# This script consolidates and updates all key documentation for AI coders

# Define colors for better readability
$headerColor = "Cyan"
$subHeaderColor = "Yellow"
$highlightColor = "Green"
$warningColor = "Red"
$successColor = "Magenta"

# Clear the screen
Clear-Host

# Display welcome message
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host "       ORDO PROJECT - UPDATE AI DOCUMENTATION            " -ForegroundColor $headerColor
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host ""

# Get the base directory
$baseDir = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$docsDir = Join-Path $baseDir "docs"
$toolsDir = Join-Path $baseDir "tools"
$unifiedAnalyzerDir = Join-Path $toolsDir "unified-analyzer"

# Create docs directory if it doesn't exist
if (-not (Test-Path $docsDir)) {
    Write-Host "Creating docs directory..." -ForegroundColor $subHeaderColor
    New-Item -ItemType Directory -Path $docsDir | Out-Null
}

# Define paths for key documents
$orientationPath = Join-Path $docsDir "ai_coder_orientation.md"
$centralHubPath = Join-Path $docsDir "central_reference_hub.md"
$nextStepsPath = Join-Path $docsDir "integration-advisor\next_steps.md"
$recommendationsPath = Join-Path $unifiedAnalyzerDir "docs\integration-advisor\reports\recommendations.md"

# Step 1: Run the unified analyzer to generate up-to-date reports
Write-Host "Step 1: Running unified analyzer to generate up-to-date reports..." -ForegroundColor $headerColor
Write-Host ""

# Check if unified analyzer exists
if (-not (Test-Path (Join-Path $unifiedAnalyzerDir "Cargo.toml"))) {
    Write-Host "ERROR: Unified analyzer not found at $unifiedAnalyzerDir" -ForegroundColor $warningColor
    Write-Host "Please ensure the unified analyzer is properly installed." -ForegroundColor $warningColor
    exit 1
}

# Run the unified analyzer
try {
    Set-Location $unifiedAnalyzerDir
    Write-Host "Running unified analyzer..." -ForegroundColor $subHeaderColor
    & cargo run --bin unified-analyzer -- integration-advisor
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "ERROR: Unified analyzer failed with exit code $LASTEXITCODE" -ForegroundColor $warningColor
        exit 1
    }
    
    Write-Host "Unified analyzer completed successfully." -ForegroundColor $successColor
} catch {
    Write-Host "ERROR: Failed to run unified analyzer: $_" -ForegroundColor $warningColor
    exit 1
} finally {
    Set-Location $baseDir
}

# Step 2: Copy reports to the docs directory
Write-Host ""
Write-Host "Step 2: Copying reports to the docs directory..." -ForegroundColor $headerColor
Write-Host ""

# Create integration-advisor directory if it doesn't exist
$integrAdvisorDir = Join-Path $docsDir "integration-advisor"
if (-not (Test-Path $integrAdvisorDir)) {
    Write-Host "Creating integration-advisor directory..." -ForegroundColor $subHeaderColor
    New-Item -ItemType Directory -Path $integrAdvisorDir | Out-Null
}

$integrAdvisorReportsDir = Join-Path $integrAdvisorDir "reports"
if (-not (Test-Path $integrAdvisorReportsDir)) {
    Write-Host "Creating integration-advisor/reports directory..." -ForegroundColor $subHeaderColor
    New-Item -ItemType Directory -Path $integrAdvisorReportsDir | Out-Null
}

# Copy next steps document
$sourceNextStepsPath = Join-Path $unifiedAnalyzerDir "docs\integration-advisor\next_steps.md"
if (Test-Path $sourceNextStepsPath) {
    Write-Host "Copying next steps document..." -ForegroundColor $subHeaderColor
    Copy-Item -Path $sourceNextStepsPath -Destination $nextStepsPath -Force
    Write-Host "Next steps document copied to $nextStepsPath" -ForegroundColor $successColor
} else {
    Write-Host "WARNING: Next steps document not found at $sourceNextStepsPath" -ForegroundColor $warningColor
}

# Copy reports
$sourceReportsDir = Join-Path $unifiedAnalyzerDir "docs\integration-advisor\reports"
if (Test-Path $sourceReportsDir) {
    Write-Host "Copying reports..." -ForegroundColor $subHeaderColor
    
    # Get all report files
    $reportFiles = Get-ChildItem -Path $sourceReportsDir -Filter "*.md"
    
    foreach ($file in $reportFiles) {
        $destPath = Join-Path $integrAdvisorReportsDir $file.Name
        Copy-Item -Path $file.FullName -Destination $destPath -Force
        Write-Host "Copied $($file.Name) to $destPath" -ForegroundColor $successColor
    }
} else {
    Write-Host "WARNING: Reports directory not found at $sourceReportsDir" -ForegroundColor $warningColor
}

# Step 3: Update the AI coder orientation guide
Write-Host ""
Write-Host "Step 3: Updating AI coder orientation guide..." -ForegroundColor $headerColor
Write-Host ""

# Get the current date
$currentDate = Get-Date -Format "yyyy-MM-dd"

# Create or update the orientation guide
$orientationContent = @"
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
- **Backend**: Rust and Haskell
- **Database**: Hybrid SQLite/Redb approach
- **ORM**: SQLx for type-safe SQL

## 🏗️ Project Architecture

Ordo follows a modular architecture with clear separation of concerns:

```plaintext
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
```

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
5. **Migration**: Migrate key modules from Canvas and Discourse to Rust

See the [Next Steps Document](integration-advisor/next_steps.md) for detailed priorities.

## 🔍 Key Implementation Details

### Hybrid Storage Example

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
    }
}
```

### Sync Engine Example

```rust
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
```

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

This orientation guide was last updated: $currentDate
"@

# Write the orientation guide
Write-Host "Writing AI coder orientation guide..." -ForegroundColor $subHeaderColor
Set-Content -Path $orientationPath -Value $orientationContent
Write-Host "AI coder orientation guide updated at $orientationPath" -ForegroundColor $successColor

# Step 4: Create a consolidated reference document
Write-Host ""
Write-Host "Step 4: Creating consolidated reference document..." -ForegroundColor $headerColor
Write-Host ""

$consolidatedPath = Join-Path $docsDir "ai_coder_reference.md"

# Create the consolidated reference
$consolidatedContent = @"
# AI Coder Consolidated Reference for Ordo Project

This document provides links to all key resources for AI coders working on the Ordo project.

## 🚀 Essential Documents

1. [AI Coder Orientation Guide](ai_coder_orientation.md) - Start here for a comprehensive overview
2. [Central Reference Hub](central_reference_hub.md) - Main source of truth for the project
3. [Next Steps Document](integration-advisor/next_steps.md) - Current implementation priorities
4. [Migration Recommendations](integration-advisor/reports/recommendations.md) - Migration strategies

## 📊 Integration Reports

1. [Feature Mappings](integration-advisor/reports/feature_mappings.md) - Mapping of features between systems
2. [Code Quality Report](integration-advisor/reports/code_quality.md) - Code quality analysis
3. [Conflicts Report](integration-advisor/reports/conflicts.md) - Detected conflicts between systems
4. [Integration Progress](integration-advisor/reports/integration_progress.md) - Current integration status

## 🏗️ Architecture Documentation

1. [Architecture Overview](architecture/overview.md) - High-level architecture
2. [Database Architecture](architecture/database.md) - Database design and implementation
3. [Synchronization Architecture](architecture/synchronization.md) - Sync engine design
4. [Modular Architecture](architecture/modular_architecture.md) - Module system design

## 🛠️ Development Resources

1. [Development Setup](development/setup.md) - Setting up the development environment
2. [Coding Standards](development/coding_standards.md) - Coding standards and best practices
3. [Testing Guidelines](development/testing_guidelines.md) - Guidelines for writing tests
4. [Contribution Guidelines](development/contribution.md) - How to contribute to the project

---

This reference was last updated: $currentDate
"@

# Write the consolidated reference
Write-Host "Writing consolidated reference document..." -ForegroundColor $subHeaderColor
Set-Content -Path $consolidatedPath -Value $consolidatedContent
Write-Host "Consolidated reference document created at $consolidatedPath" -ForegroundColor $successColor

# Step 5: Create architecture directory if it doesn't exist
$architectureDir = Join-Path $docsDir "architecture"
if (-not (Test-Path $architectureDir)) {
    Write-Host ""
    Write-Host "Step 5: Creating architecture documentation directory..." -ForegroundColor $headerColor
    Write-Host ""
    
    New-Item -ItemType Directory -Path $architectureDir | Out-Null
    Write-Host "Architecture directory created at $architectureDir" -ForegroundColor $successColor
    
    # Create placeholder files for architecture documentation
    $overviewPath = Join-Path $architectureDir "overview.md"
    $databasePath = Join-Path $architectureDir "database.md"
    $syncPath = Join-Path $architectureDir "synchronization.md"
    $modularPath = Join-Path $architectureDir "modular_architecture.md"
    
    # Create overview.md
    $overviewContent = @"
# Architecture Overview

This document provides a high-level overview of the Ordo architecture.

## Core Components

1. **Frontend (Leptos)**: Reactive UI framework built with Rust
2. **Backend (Rust/Haskell)**: Core business logic and data processing
3. **Database (SQLite/Redb)**: Hybrid storage approach
4. **Sync Engine**: Handles offline-first synchronization

## Architecture Diagram

```
+------------------+     +------------------+     +------------------+
|                  |     |                  |     |                  |
|  Leptos Frontend |<--->|  Rust Backend    |<--->|  Database Layer  |
|                  |     |                  |     |                  |
+------------------+     +------------------+     +------------------+
                               |
                               v
                         +------------------+
                         |                  |
                         |  Sync Engine     |
                         |                  |
                         +------------------+
```

## Key Principles

1. **Clean Architecture**: Domain-centric design
2. **Offline-First**: Local-first data with synchronization
3. **Domain-Driven Design**: Focus on core domain logic

*Note: This is a placeholder document. Please update with actual architecture details.*
"@
    Set-Content -Path $overviewPath -Value $overviewContent
    
    # Create database.md
    $databaseContent = @"
# Database Architecture

This document describes the database architecture of the Ordo application.

## Hybrid Storage Approach

Ordo uses a hybrid storage approach combining SQLite and Redb:

1. **SQLite with SQLx**: For structured domain data
   - Courses, assignments, users, discussions
   - Persistent, structured data with relationships

2. **Redb (Rust Embedded Database)**: For ephemeral state and sync metadata
   - Draft content, sync operations queue
   - Session state, real-time subscriptions
   - Offline queue handling

## Schema Design

*Note: This is a placeholder document. Please update with actual database schema details.*
"@
    Set-Content -Path $databasePath -Value $databaseContent
    
    # Create synchronization.md
    $syncContent = @"
# Synchronization Architecture

This document describes the synchronization architecture of the Ordo application.

## Sync Engine

The sync engine handles data synchronization between local and remote databases:

1. **Operation-Based Sync**: Records all changes as operations
2. **Conflict Resolution**: Uses version vectors and domain-specific resolution
3. **Offline Queue**: Stores operations when offline
4. **Background Sync**: Processes queue when connectivity returns

## Sync Process

1. User makes changes locally
2. Changes are recorded as operations in the local database
3. When online, operations are sent to the server
4. Server processes operations and returns results
5. Client updates local state based on server response

*Note: This is a placeholder document. Please update with actual synchronization details.*
"@
    Set-Content -Path $syncPath -Value $syncContent
    
    # Create modular_architecture.md
    $modularContent = @"
# Modular Architecture

This document describes the modular architecture of the Ordo application.

## Module System

Ordo uses a modular architecture that allows for app-like modules to be turned on and off:

```
src-tauri/
└── src/
    └── modules/
        ├── quiz/      # Quiz module
        ├── forum/     # Forum module
        └── gradebook/ # Gradebook module
```

## Module Interface

Each module implements a standard interface:

```rust
pub trait Module {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&self) -> Result<()>;
    fn routes(&self) -> Vec<Route>;
    fn permissions(&self) -> Vec<Permission>;
}
```

*Note: This is a placeholder document. Please update with actual module system details.*
"@
    Set-Content -Path $modularPath -Value $modularContent
    
    Write-Host "Created placeholder architecture documentation files:" -ForegroundColor $successColor
    Write-Host "- $overviewPath" -ForegroundColor $successColor
    Write-Host "- $databasePath" -ForegroundColor $successColor
    Write-Host "- $syncPath" -ForegroundColor $successColor
    Write-Host "- $modularPath" -ForegroundColor $successColor
}

# Step 6: Create development directory if it doesn't exist
$developmentDir = Join-Path $docsDir "development"
if (-not (Test-Path $developmentDir)) {
    Write-Host ""
    Write-Host "Step 6: Creating development documentation directory..." -ForegroundColor $headerColor
    Write-Host ""
    
    New-Item -ItemType Directory -Path $developmentDir | Out-Null
    Write-Host "Development directory created at $developmentDir" -ForegroundColor $successColor
    
    # Create placeholder files for development documentation
    $setupPath = Join-Path $developmentDir "setup.md"
    $standardsPath = Join-Path $developmentDir "coding_standards.md"
    $testingPath = Join-Path $developmentDir "testing_guidelines.md"
    $contributionPath = Join-Path $developmentDir "contribution.md"
    
    # Create setup.md
    $setupContent = @"
# Development Environment Setup

This document provides instructions for setting up the development environment for the Ordo project.

## Prerequisites

1. **Rust**: Install the latest stable version of Rust using rustup
2. **Haskell**: Install GHC and Cabal using ghcup
3. **Node.js**: Install the latest LTS version
4. **SQLite**: Install the latest version

## Setup Steps

1. Clone the repository
2. Install dependencies
3. Set up the database
4. Run the development server

*Note: This is a placeholder document. Please update with actual setup instructions.*
"@
    Set-Content -Path $setupPath -Value $setupContent
    
    # Create coding_standards.md
    $standardsContent = @"
# Coding Standards

This document outlines the coding standards for the Ordo project.

## Rust Standards

1. **Formatting**: Use rustfmt for consistent formatting
2. **Linting**: Use clippy for linting
3. **Documentation**: Document all public APIs
4. **Error Handling**: Use Result types for error propagation
5. **Naming**: Follow Rust naming conventions

## Haskell Standards

1. **Formatting**: Use hindent for consistent formatting
2. **Linting**: Use hlint for linting
3. **Documentation**: Document all public functions
4. **Error Handling**: Use Either or custom error types
5. **Naming**: Follow Haskell naming conventions

*Note: This is a placeholder document. Please update with actual coding standards.*
"@
    Set-Content -Path $standardsPath -Value $standardsContent
    
    # Create testing_guidelines.md
    $testingContent = @"
# Testing Guidelines

This document outlines the testing guidelines for the Ordo project.

## Test Types

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test interactions between components
3. **End-to-End Tests**: Test the entire application flow

## Test Coverage

Aim for at least 80% test coverage for all code.

## Test Organization

1. **Unit Tests**: Place in the same file as the code being tested
2. **Integration Tests**: Place in a separate tests directory
3. **End-to-End Tests**: Place in a separate e2e directory

*Note: This is a placeholder document. Please update with actual testing guidelines.*
"@
    Set-Content -Path $testingPath -Value $testingContent
    
    # Create contribution.md
    $contributionContent = @"
# Contribution Guidelines

This document outlines the contribution guidelines for the Ordo project.

## Contribution Process

1. **Fork the repository**: Create your own fork of the repository
2. **Create a branch**: Create a branch for your changes
3. **Make changes**: Make your changes following the coding standards
4. **Write tests**: Write tests for your changes
5. **Submit a pull request**: Submit a pull request for review

## Code Review Process

1. **Automated checks**: All pull requests must pass automated checks
2. **Code review**: All pull requests must be reviewed by at least one maintainer
3. **Approval**: Pull requests must be approved before merging

*Note: This is a placeholder document. Please update with actual contribution guidelines.*
"@
    Set-Content -Path $contributionPath -Value $contributionContent
    
    Write-Host "Created placeholder development documentation files:" -ForegroundColor $successColor
    Write-Host "- $setupPath" -ForegroundColor $successColor
    Write-Host "- $standardsPath" -ForegroundColor $successColor
    Write-Host "- $testingPath" -ForegroundColor $successColor
    Write-Host "- $contributionPath" -ForegroundColor $successColor
}

# Final message
Write-Host ""
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host "       AI DOCUMENTATION UPDATE COMPLETED                 " -ForegroundColor $headerColor
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host ""
Write-Host "All AI documentation has been updated successfully." -ForegroundColor $successColor
Write-Host ""
Write-Host "Key documents:" -ForegroundColor $highlightColor
Write-Host "1. AI Coder Orientation Guide: $orientationPath" -ForegroundColor $highlightColor
Write-Host "2. Consolidated Reference: $consolidatedPath" -ForegroundColor $highlightColor
Write-Host "3. Next Steps Document: $nextStepsPath" -ForegroundColor $highlightColor
Write-Host "4. Migration Recommendations: $recommendationsPath" -ForegroundColor $highlightColor
Write-Host ""
Write-Host "Run the AI coder onboarding script to orient new AI coders:" -ForegroundColor $highlightColor
Write-Host "   .\tools\ai_coder_onboarding.ps1" -ForegroundColor $highlightColor
Write-Host ""
