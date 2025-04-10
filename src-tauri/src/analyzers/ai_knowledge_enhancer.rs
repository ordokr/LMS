use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local, Utc};

use crate::analyzers::unified_analyzer::AnalysisResult;

/// AI Knowledge Enhancer that generates specialized documentation for AI agents
pub struct AiKnowledgeEnhancer {
    base_dir: PathBuf,
    knowledge_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
struct KnowledgeSection {
    title: String,
    content: String,
    keywords: Vec<String>,
    importance: u8, // 1-10 with 10 being highest
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectKnowledge {
    sections: Vec<KnowledgeSection>,
    last_updated: DateTime<Utc>,
}

impl AiKnowledgeEnhancer {
    pub fn new(base_dir: PathBuf) -> Self {
        let knowledge_dir = base_dir.join("ai_knowledge");
        
        Self {
            base_dir,
            knowledge_dir,
        }
    }
    
    /// Generate enhanced knowledge base for AI agents
    pub fn enhance_knowledge_base(&self, result: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Enhancing AI knowledge base...");
        
        // Ensure knowledge directory exists
        if !self.knowledge_dir.exists() {
            fs::create_dir_all(&self.knowledge_dir)
                .map_err(|e| format!("Failed to create AI knowledge directory: {}", e))?;
        }
        
        // Generate structured knowledge from analysis
        let project_knowledge = self.extract_structured_knowledge(result);
        
        // Save knowledge JSON for potential future use by AI systems
        let knowledge_json = self.knowledge_dir.join("project_knowledge.json");
        let json = serde_json::to_string_pretty(&project_knowledge)
            .map_err(|e| format!("Failed to serialize project knowledge: {}", e))?;
        
        fs::write(&knowledge_json, json)
            .map_err(|e| format!("Failed to write project knowledge JSON: {}", e))?;
        
        // Generate the AI assistant guide
        let guide_path = self.generate_ai_assistant_guide(&project_knowledge)?;
        
        // Generate hardcoded knowledge summaries
        self.generate_hardcoded_knowledge_summaries(result)?;
        
        println!("AI knowledge base enhanced successfully");
        Ok(guide_path)
    }
    
    /// Extract structured knowledge from analysis result
    fn extract_structured_knowledge(&self, result: &AnalysisResult) -> ProjectKnowledge {
        let mut sections = Vec::new();
        
        // Project Overview section
        sections.push(KnowledgeSection {
            title: "Project Overview".to_string(),
            content: format!(
                "The LMS (Learning Management System) project is a Rust-based application integrating Canvas LMS and Discourse forum functionality. \
                The project uses Tauri and Leptos for the frontend, with SQLite for data storage. \
                The project is currently {:.1}% complete overall, with models at {:.1}% implementation, API endpoints at {:.1}% implementation, \
                and UI components at {:.1}% implementation.",
                result.project_status.overall_completion_percentage,
                result.models.implementation_percentage,
                result.api_endpoints.implementation_percentage,
                result.ui_components.implementation_percentage
            ),
            keywords: vec!["LMS".to_string(), "Canvas".to_string(), "Discourse".to_string(), "overview".to_string()],
            importance: 10,
        });
        
        // Architecture section
        sections.push(KnowledgeSection {
            title: "Architecture".to_string(),
            content: format!(
                "The project follows a {} architecture. Key components include: {}. \
                The application implements a clean architecture with SOLID principles and an offline-first deployment model.",
                result.architecture.architecture_style,
                result.architecture.key_components.iter()
                    .map(|c| format!("{} ({})", c.name, c.description))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            keywords: vec!["architecture".to_string(), "components".to_string(), "structure".to_string()],
            importance: 9,
        });
        
        // Technology Stack section
        sections.push(KnowledgeSection {
            title: "Technology Stack".to_string(),
            content: "The project uses the following technologies:\n\
                - **Primary Languages**: Rust, Haskell\n\
                - **Frontend**: Leptos, Tauri\n\
                - **Database**: SQLite with sqlx\n\
                - **Search**: MeiliSearch\n\
                - **Authentication**: JWT\n\
                - **Blockchain**: Custom Rust implementation".to_string(),
            keywords: vec!["technology".to_string(), "stack".to_string(), "languages".to_string(), "frameworks".to_string()],
            importance: 8,
        });
        
        // Models section
        sections.push(KnowledgeSection {
            title: "Data Models".to_string(),
            content: format!(
                "The project has {} models in total, with {} implemented ({:.1}% completion). \
                Key models include: {}.",
                result.models.total,
                result.models.implemented,
                result.models.implementation_percentage,
                result.models.key_models.iter()
                    .take(5)
                    .map(|m| format!("{} from {}", m.name, m.source_system))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            keywords: vec!["models".to_string(), "data models".to_string(), "entities".to_string()],
            importance: 7,
        });
        
        // API Endpoints section
        sections.push(KnowledgeSection {
            title: "API Endpoints".to_string(),
            content: format!(
                "The project has {} API endpoints in total, with {} implemented ({:.1}% completion). \
                The API follows RESTful principles with proper namespacing to avoid conflicts between Canvas and Discourse resources.",
                result.api_endpoints.total,
                result.api_endpoints.implemented,
                result.api_endpoints.implementation_percentage
            ),
            keywords: vec!["API".to_string(), "endpoints".to_string(), "REST".to_string()],
            importance: 7,
        });
        
        // Synchronization section
        sections.push(KnowledgeSection {
            title: "Synchronization System".to_string(),
            content: format!(
                "The application uses a {} synchronization strategy with {} status. \
                This allows for offline-first operation with data synchronization when connectivity is restored.",
                result.sync_system.strategy,
                result.sync_system.status
            ),
            keywords: vec!["synchronization".to_string(), "offline".to_string(), "sync".to_string()],
            importance: 8,
        });
        
        // UI Components section
        sections.push(KnowledgeSection {
            title: "UI Components".to_string(),
            content: format!(
                "The UI layer consists of {} components in total, with {} implemented ({:.1}% completion). \
                The frontend uses Leptos for reactive components and Tauri for the desktop shell.",
                result.ui_components.total,
                result.ui_components.implemented,
                result.ui_components.implementation_percentage
            ),
            keywords: vec!["UI".to_string(), "components".to_string(), "frontend".to_string(), "Leptos".to_string()],
            importance: 6,
        });
        
        // Tests section
        sections.push(KnowledgeSection {
            title: "Testing".to_string(),
            content: format!(
                "The project has {} tests with {:.1}% code coverage. \
                The testing strategy emphasizes integration tests for critical functionality.",
                result.tests.total,
                result.tests.coverage_percentage
            ),
            keywords: vec!["tests".to_string(), "testing".to_string(), "coverage".to_string()],
            importance: 5,
        });
        
        // Implementation Priorities section
        sections.push(KnowledgeSection {
            title: "Implementation Priorities".to_string(),
            content: format!(
                "Current implementation priorities:\n{}",
                result.project_status.next_steps.iter()
                    .enumerate()
                    .map(|(i, step)| format!("{}. {}", i+1, step))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            keywords: vec!["priorities".to_string(), "next steps".to_string(), "roadmap".to_string()],
            importance: 9,
        });
        
        // Technical Debt section
        sections.push(KnowledgeSection {
            title: "Technical Debt".to_string(),
            content: format!(
                "Current technical debt items:\n{}",
                result.code_quality.tech_debt_items.iter()
                    .map(|item| format!("- **{}**: {}", item.category, item.description))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
            keywords: vec!["technical debt".to_string(), "issues".to_string(), "quality".to_string()],
            importance: 6,
        });
        
        // Migration status section
        sections.push(KnowledgeSection {
            title: "JavaScript to Rust Migration".to_string(),
            content: "The project is in the process of migrating JavaScript code to Rust implementations. \
                When assisting with development, prefer Rust implementations over JavaScript solutions. \
                The js_migration_analyzer tool identifies JavaScript files for migration based on priority.",
            keywords: vec!["migration".to_string(), "JavaScript".to_string(), "Rust".to_string()],
            importance: 8,
        });
        
        ProjectKnowledge {
            sections,
            last_updated: Utc::now(),
        }
    }
    
    /// Generate an AI assistant guide from knowledge
    fn generate_ai_assistant_guide(&self, knowledge: &ProjectKnowledge) -> Result<PathBuf, String> {
        let guide_path = self.base_dir.join("docs").join("AI_ASSISTANT_GUIDE.md");
        
        // Sort sections by importance
        let mut sorted_sections = knowledge.sections.clone();
        sorted_sections.sort_by(|a, b| b.importance.cmp(&a.importance));
        
        // Generate the guide content
        let mut content = String::new();
        
        content.push_str("# AI Assistant Guide for LMS Project\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## IMPORTANT INSTRUCTIONS FOR AI ASSISTANTS\n\n");
        content.push_str("This guide provides essential information about the LMS (Learning Management System) project to help AI assistants \
                         provide accurate and relevant assistance. When helping with this project, please:\n\n");
        content.push_str("1. **Prefer Rust solutions** over JavaScript/TypeScript implementations\n");
        content.push_str("2. **Maintain architectural consistency** with the existing codebase\n");
        content.push_str("3. **Support offline-first design** principles in all implementations\n");
        content.push_str("4. **Respect the technology choices** outlined in this document\n");
        content.push_str("5. **Focus on current implementation priorities** when suggesting solutions\n\n");
        
        content.push_str("## Project Knowledge Sections\n\n");
        
        for section in sorted_sections {
            content.push_str(&format!("### {}\n\n", section.title));
            content.push_str(&format!("{}\n\n", section.content));
            content.push_str(&format!("**Keywords**: {}\n\n", section.keywords.join(", ")));
            content.push_str("---\n\n");
        }
        
        content.push_str("## Response Guidance\n\n");
        content.push_str("When responding to questions about this project:\n\n");
        content.push_str("1. **Be specific to this project** - Use the information in this guide to tailor your responses\n");
        content.push_str("2. **Follow the architecture** - Suggest solutions that align with the project's architecture\n");
        content.push_str("3. **Prefer Rust** - When suggesting code, use Rust for new implementations\n");
        content.push_str("4. **Respect technology choices** - Don't suggest alternative technologies unless specifically asked\n");
        content.push_str("5. **Acknowledge limitations** - If a question requires information not in this guide, acknowledge this\n\n");
        
        content.push_str("This guide was automatically generated based on project analysis and may not cover all aspects of the project.\n");
        
        fs::write(&guide_path, content)
            .map_err(|e| format!("Failed to write AI assistant guide: {}", e))?;
        
        Ok(guide_path)
    }
    
    /// Generate hardcoded knowledge summaries for specific components
    fn generate_hardcoded_knowledge_summaries(&self, result: &AnalysisResult) -> Result<(), String> {
        // Create specific knowledge summaries directory
        let summaries_dir = self.knowledge_dir.join("component_summaries");
        if !summaries_dir.exists() {
            fs::create_dir_all(&summaries_dir)
                .map_err(|e| format!("Failed to create summaries directory: {}", e))?;
        }
        
        // Blockchain Component Summary
        let blockchain_summary = self.generate_blockchain_summary(result)?;
        fs::write(summaries_dir.join("blockchain_summary.md"), blockchain_summary)
            .map_err(|e| format!("Failed to write blockchain summary: {}", e))?;
        
        // Synchronization System Summary
        let sync_summary = self.generate_sync_system_summary(result)?;
        fs::write(summaries_dir.join("sync_system_summary.md"), sync_summary)
            .map_err(|e| format!("Failed to write sync system summary: {}", e))?;
        
        // Database Design Summary
        let db_summary = self.generate_database_summary(result)?;
        fs::write(summaries_dir.join("database_summary.md"), db_summary)
            .map_err(|e| format!("Failed to write database summary: {}", e))?;
        
        // Authentication System Summary
        let auth_summary = self.generate_auth_summary(result)?;
        fs::write(summaries_dir.join("auth_summary.md"), auth_summary)
            .map_err(|e| format!("Failed to write auth summary: {}", e))?;
        
        // Frontend Architecture Summary
        let frontend_summary = self.generate_frontend_summary(result)?;
        fs::write(summaries_dir.join("frontend_summary.md"), frontend_summary)
            .map_err(|e| format!("Failed to write frontend summary: {}", e))?;
        
        println!("Generated {} component knowledge summaries", 5);
        Ok(())
    }
    
    /// Generate blockchain component summary
    fn generate_blockchain_summary(&self, result: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("# Blockchain Implementation Knowledge\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## Overview\n\n");
        content.push_str(&format!("The LMS project implements a custom blockchain component with status: **{}**.\n\n", 
            result.blockchain.status));
        
        content.push_str("## Implementation Details\n\n");
        content.push_str(&format!("- **Type**: {}\n", result.blockchain.implementation_type));
        content.push_str(&format!("- **Completion**: {}%\n", result.blockchain.completion_percentage));
        content.push_str("- **Purpose**: The blockchain component provides data integrity and verification for critical records\n\n");
        
        content.push_str("## Features\n\n");
        for feature in &result.blockchain.features {
            content.push_str(&format!("- {}\n", feature));
        }
        content.push_str("\n");
        
        content.push_str("## Integration Points\n\n");
        for point in &result.blockchain.integration_points {
            content.push_str(&format!("- {}\n", point));
        }
        content.push_str("\n");
        
        content.push_str("## Technical Implementation\n\n");
        content.push_str("The blockchain implementation uses a custom Rust-based solution with the following characteristics:\n\n");
        content.push_str("- **Consensus Mechanism**: Proof of Authority for efficient verification\n");
        content.push_str("- **Block Structure**: Includes transactions related to grade records, certificate issuance, and content integrity\n");
        content.push_str("- **Storage**: Blocks are stored in SQLite with optional export to external storage\n");
        content.push_str("- **Verification**: Digital signatures using EdDSA (Edwards-curve Digital Signature Algorithm)\n\n");
        
        content.push_str("## Usage Guidelines\n\n");
        content.push_str("When working with the blockchain component:\n\n");
        content.push_str("1. Use the `BlockchainService` for all interactions\n");
        content.push_str("2. Records should be added as transactions via the provided API\n");
        content.push_str("3. Verification should use the public verification methods\n");
        content.push_str("4. For new blockchain features, extend the existing architecture rather than introducing new paradigms\n");
        
        Ok(content)
    }
    
    /// Generate synchronization system summary
    fn generate_sync_system_summary(&self, result: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("# Synchronization System Knowledge\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## Overview\n\n");
        content.push_str(&format!("The LMS project implements a **{}** synchronization system with status: **{}**.\n\n",
            result.sync_system.sync_type,
            result.sync_system.status));
        
        content.push_str("## Strategy\n\n");
        content.push_str(&format!("The synchronization strategy is: **{}**\n\n", result.sync_system.strategy));
        content.push_str("This strategy enables:\n\n");
        content.push_str("- Offline-first operation for all core functionality\n");
        content.push_str("- Conflict resolution when connectivity is restored\n");
        content.push_str("- Priority-based synchronization for critical data\n\n");
        
        content.push_str("## Technical Implementation\n\n");
        content.push_str("The synchronization system uses the following components:\n\n");
        content.push_str("1. **SyncManager**: Central coordination of all sync operations\n");
        content.push_str("2. **SyncQueue**: Persistent queue for tracking pending changes\n");
        content.push_str("3. **ConflictResolver**: Handles merge conflicts using timestamp-based resolution with manual override option\n");
        content.push_str("4. **NetworkMonitor**: Detects connectivity changes to trigger sync operations\n\n");
        
        content.push_str("## Data Flow\n\n");
        content.push_str("```\n");
        content.push_str("┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐\n");
        content.push_str("│  Local Changes  │─────►│   Sync Queue    │─────►│ Conflict Check  │\n");
        content.push_str("└─────────────────┘      └─────────────────┘      └─────────────────┘\n");
        content.push_str("                                                          │\n");
        content.push_str("                                                          ▼\n");
        content.push_str("┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐\n");
        content.push_str("│  Remote System  │◄─────│   API Client    │◄─────│ Change Resolver │\n");
        content.push_str("└─────────────────┘      └─────────────────┘      └─────────────────┘\n");
        content.push_str("        │                                                  ▲\n");
        content.push_str("        ▼                                                  │\n");
        content.push_str("┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐\n");
        content.push_str("│ Response Parser │─────►│ Response Queue  │─────►│  Local Storage  │\n");
        content.push_str("└─────────────────┘      └─────────────────┘      └─────────────────┘\n");
        content.push_str("```\n\n");
        
        content.push_str("## Usage Guidelines\n\n");
        content.push_str("When working with the synchronization system:\n\n");
        content.push_str("1. All data modifications should go through the appropriate repository classes\n");
        content.push_str("2. Never bypass the sync system by directly modifying remote systems\n");
        content.push_str("3. Handle sync conflicts in UI components using the provided conflict resolution interfaces\n");
        content.push_str("4. For new features, always consider both online and offline operation modes\n");
        
        Ok(content)
    }
    
    /// Generate database system summary
    fn generate_database_summary(&self, _result: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("# Database System Knowledge\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## Overview\n\n");
        content.push_str("The LMS project uses SQLite with the sqlx crate for type-safe database access. The database schema integrates data models from both Canvas LMS and Discourse forum systems.\n\n");
        
        content.push_str("## Database Architecture\n\n");
        content.push_str("The database follows a clean architecture approach with these layers:\n\n");
        content.push_str("1. **Models**: Domain entities representing core business objects\n");
        content.push_str("2. **Repositories**: Type-safe data access interfaces\n");
        content.push_str("3. **Services**: Business logic that coordinates data operations\n");
        content.push_str("4. **Migrations**: Managed with sqlx-cli for schema versioning\n\n");
        
        content.push_str("## Key Schema Relationships\n\n");
        content.push_str("```\n");
        content.push_str("┌───────────┐     ┌────────────┐     ┌───────────┐\n");
        content.push_str("│   User    │◄───►│    Role    │◄───►│ Permission │\n");
        content.push_str("└───────────┘     └────────────┘     └───────────┘\n");
        content.push_str("      ▲                                    ▲\n");
        content.push_str("      │                                    │\n");
        content.push_str("      ▼                                    ▼\n");
        content.push_str("┌───────────┐     ┌────────────┐     ┌───────────┐\n");
        content.push_str("│  Course   │◄───►│  Category   │◄───►│   Group   │\n");
        content.push_str("└───────────┘     └────────────┘     └───────────┘\n");
        content.push_str("      ▲                 ▲                 ▲\n");
        content.push_str("      │                 │                 │\n");
        content.push_str("      ▼                 ▼                 ▼\n");
        content.push_str("┌───────────┐     ┌────────────┐     ┌───────────┐\n");
        content.push_str("│ Assignment │◄───►│   Topic    │◄───►│    Post   │\n");
        content.push_str("└───────────┘     └────────────┘     └───────────┘\n");
        content.push_str("```\n\n");
        
        content.push_str("## Data Access Pattern\n\n");
        content.push_str("All data access follows the Repository pattern:\n\n");
        content.push_str("```rust\n");
        content.push_str("// Example repository implementation\n");
        content.push_str("pub struct CourseRepository {\n");
        content.push_str("    pool: Pool<Sqlite>,\n");
        content.push_str("}\n\n");
        content.push_str("impl CourseRepository {\n");
        content.push_str("    pub async fn find_by_id(&self, id: i64) -> Result<Course, Error> {\n");
        content.push_str("        sqlx::query_as!(Course, \"SELECT * FROM courses WHERE id = ?\", id)\n");
        content.push_str("            .fetch_one(&self.pool)\n");
        content.push_str("            .await\n");
        content.push_str("    }\n");
        content.push_str("    \n");
        content.push_str("    // Other methods...\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        content.push_str("## Offline Data Strategy\n\n");
        content.push_str("The database implements an offline-first strategy with:\n\n");
        content.push_str("1. **Local-first writes**: All writes go to local SQLite database first\n");
        content.push_str("2. **Sync metadata**: Each record includes sync status and timestamp information\n");
        content.push_str("3. **Conflict detection**: Vector clock mechanism to detect conflicts\n");
        content.push_str("4. **Sync queue**: Pending changes tracked for eventual synchronization\n\n");
        
        content.push_str("## Usage Guidelines\n\n");
        content.push_str("When working with the database:\n\n");
        content.push_str("1. Always use repository classes for data access\n");
        content.push_str("2. Use transactions for operations that modify multiple tables\n");
        content.push_str("3. Follow the migration pattern for schema changes\n");
        content.push_str("4. Include sync metadata for all new table designs\n");
        content.push_str("5. Include appropriate indexes for frequent query patterns\n");
        
        Ok(content)
    }
    
    /// Generate authentication system summary
    fn generate_auth_summary(&self, _result: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("# Authentication System Knowledge\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## Overview\n\n");
        content.push_str("The LMS project implements a unified JWT-based authentication system that integrates with both Canvas OAuth and Discourse SSO mechanisms.\n\n");
        
        content.push_str("## Authentication Flow\n\n");
        content.push_str("```\n");
        content.push_str("┌─────────────┐     ┌───────────────┐     ┌─────────────┐\n");
        content.push_str("│   Client    │────►│ Auth Endpoint │────►│  JWT Token  │\n");
        content.push_str("└─────────────┘     └───────────────┘     └─────────────┘\n");
        content.push_str("                           │                     │\n");
        content.push_str("                           ▼                     ▼\n");
        content.push_str("                    ┌─────────────┐     ┌─────────────┐\n");
        content.push_str("                    │ Canvas OAuth│     │   API Call  │\n");
        content.push_str("                    └─────────────┘     └─────────────┘\n");
        content.push_str("                           │                     ▲\n");
        content.push_str("                           ▼                     │\n");
        content.push_str("                    ┌─────────────┐     ┌─────────────┐\n");
        content.push_str("                    │Discourse SSO│     │Token Verify │\n");
        content.push_str("                    └─────────────┘     └─────────────┘\n");
        content.push_str("```\n\n");
        
        content.push_str("## Key Components\n\n");
        content.push_str("1. **JwtService**: Generates and validates JWT tokens\n");
        content.push_str("2. **AuthController**: Handles authentication endpoints\n");
        content.push_str("3. **CanvasOAuthProvider**: Integrates with Canvas OAuth\n");
        content.push_str("4. **DiscourseSsoProvider**: Integrates with Discourse SSO\n");
        content.push_str("5. **RoleBasedAccessControl**: Manages permissions based on roles\n\n");
        
        content.push_str("## Token Structure\n\n");
        content.push_str("```json\n");
        content.push_str("{\n");
        content.push_str("  \"header\": {\n");
        content.push_str("    \"alg\": \"RS256\",\n");
        content.push_str("    \"typ\": \"JWT\"\n");
        content.push_str("  },\n");
        content.push_str("  \"payload\": {\n");
        content.push_str("    \"sub\": \"user_id\",\n");
        content.push_str("    \"name\": \"User Name\",\n");
        content.push_str("    \"email\": \"user@example.com\",\n");
        content.push_str("    \"roles\": [\"student\", \"instructor\"],\n");
        content.push_str("    \"canvas_id\": \"canvas_user_id\",\n");
        content.push_str("    \"discourse_id\": \"discourse_user_id\",\n");
        content.push_str("    \"iat\": 1616617768,\n");
        content.push_str("    \"exp\": 1616621368\n");
        content.push_str("  }\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        content.push_str("## Offline Authentication\n\n");
        content.push_str("The authentication system supports offline operation through:\n\n");
        content.push_str("1. **Extended token validity**: Longer token lifetimes when offline mode is detected\n");
        content.push_str("2. **Refresh tokens**: Stored securely in the local database\n");
        content.push_str("3. **Permission caching**: Role and permission data cached locally\n");
        content.push_str("4. **Synchronization**: Authentication state synchronized when online\n\n");
        
        content.push_str("## Usage Guidelines\n\n");
        content.push_str("When working with the authentication system:\n\n");
        content.push_str("1. Use the `authenticate` middleware for protected routes\n");
        content.push_str("2. Check role-based permissions using the `can` helper\n");
        content.push_str("3. Refresh tokens proactively before expiration\n");
        content.push_str("4. Handle offline authentication gracefully in the UI\n");
        content.push_str("5. Use proper HTTP status codes for authentication errors (401, 403)\n");
        
        Ok(content)
    }
    
    /// Generate frontend architecture summary
    fn generate_frontend_summary(&self, _result: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        content.push_str("# Frontend Architecture Knowledge\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));
        
        content.push_str("## Overview\n\n");
        content.push_str("The LMS project frontend is built with Leptos for reactive UI components and Tauri for the native desktop shell. The architecture follows a component-based design with reactivity and state management.\n\n");
        
        content.push_str("## Architecture Layers\n\n");
        content.push_str("```\n");
        content.push_str("┌─────────────────────────────────────────────────────┐\n");
        content.push_str("│                  UI Components                       │\n");
        content.push_str("│  ┌───────────┐  ┌───────────┐  ┌───────────┐        │\n");
        content.push_str("│  │   Pages   │  │   Forms   │  │  Widgets  │  ...   │\n");
        content.push_str("│  └───────────┘  └───────────┘  └───────────┘        │\n");
        content.push_str("└─────────────────────────────────────────────────────┘\n");
        content.push_str("                        ▲\n");
        content.push_str("                        │\n");
        content.push_str("┌─────────────────────────────────────────────────────┐\n");
        content.push_str("│                  State Management                    │\n");
        content.push_str("│  ┌───────────┐  ┌───────────┐  ┌───────────┐        │\n");
        content.push_str("│  │  Signals  │  │  Actions  │  │  Effects  │  ...   │\n");
        content.push_str("│  └───────────┘  └───────────┘  └───────────┘        │\n");
        content.push_str("└─────────────────────────────────────────────────────┘\n");
        content.push_str("                        ▲\n");
        content.push_str("                        │\n");
        content.push_str("┌─────────────────────────────────────────────────────┐\n");
        content.push_str("│                   API Integration                    │\n");
        content.push_str("│  ┌───────────┐  ┌───────────┐  ┌───────────┐        │\n");
        content.push_str("│  │  Queries  │  │ Mutations │  │  Tauri    │  ...   │\n");
        content.push_str("│  └───────────┘  └───────────┘  └───────────┘        │\n");
        content.push_str("└─────────────────────────────────────────────────────┘\n");
        content.push_str("```\n\n");
        
        content.push_str("## Key Technologies\n\n");
        content.push_str("- **Leptos**: Reactive UI framework for WebAssembly\n");
        content.push_str("- **Tauri**: Native desktop application framework\n");
        content.push_str("- **Web Components**: Custom elements for shared functionality\n");
        content.push_str("- **CSS Modules**: Scoped styling for components\n");
        content.push_str("- **Service Workers**: Offline caching and synchronization\n\n");
        
        content.push_str("## Component Structure\n\n");
        content.push_str("Frontend components follow this general structure:\n\n");
        content.push_str("```rust\n");
        content.push_str("#[component]\n");
        content.push_str("pub fn CourseView(course_id: i64) -> impl IntoView {\n");
        content.push_str("    // State management using signals\n");
        content.push_str("    let (course, set_course) = create_signal(None);\n");
        content.push_str("    \n");
        content.push_str("    // Effect for data loading\n");
        content.push_str("    create_effect(move |_| {\n");
        content.push_str("        spawn_local(async move {\n");
        content.push_str("            if let Ok(data) = get_course(course_id).await {\n");
        content.push_str("                set_course.set(Some(data));\n");
        content.push_str("            }\n");
        content.push_str("        });\n");
        content.push_str("    });\n");
        content.push_str("    \n");
        content.push_str("    // UI rendering with reactive data\n");
        content.push_str("    view! {\n");
        content.push_str("        <div class=\"course-view\">\n");
        content.push_str("            {move || {\n");
        content.push_str("                course.get().map(|c| view! {\n");
        content.push_str("                    <h1>{c.title}</h1>\n");
        content.push_str("                    <CourseDetails course=c />\n");
        content.push_str("                })\n");
        content.push_str("            }}\n");
        content.push_str("        </div>\n");
        content.push_str("    }\n");
        content.push_str("}\n");
        content.push_str("```\n\n");
        
        content.push_str("## Offline Support\n\n");
        content.push_str("The frontend implements offline support through:\n\n");
        content.push_str("1. **Local Data Cache**: Critical data cached in IndexedDB\n");
        content.push_str("2. **Optimistic UI Updates**: Changes applied locally before server confirmation\n");
        content.push_str("3. **Background Synchronization**: Queues changes for sync when online\n");
        content.push_str("4. **Conflict Resolution UI**: Interfaces for resolving data conflicts\n\n");
        
        content.push_str("## Usage Guidelines\n\n");
        content.push_str("When developing frontend components:\n\n");
        content.push_str("1. Use reactive signals for state management\n");
        content.push_str("2. Keep components small and focused on a single responsibility\n");
        content.push_str("3. Handle loading, success, and error states explicitly\n");
        content.push_str("4. Consider offline operation for all user interactions\n");
        content.push_str("5. Follow the established UI design patterns and component hierarchy\n");
        
        Ok(content)
    }
}
