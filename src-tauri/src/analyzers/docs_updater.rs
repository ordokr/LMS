use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};

use crate::analyzers::unified_analyzer::{AnalysisResult, ModelMetrics, ApiEndpointMetrics};
use crate::utils::file_system::FileSystemUtils;

/// Documentation updater that keeps project documentation in sync with code
pub struct DocsUpdater {
    base_dir: PathBuf,
    docs_dir: PathBuf,
    rag_dir: PathBuf,
}

impl DocsUpdater {
    pub fn new(base_dir: PathBuf) -> Self {
        let docs_dir = base_dir.join("docs");
        let rag_dir = base_dir.join("rag_knowledge_base");
        
        Self {
            base_dir,
            docs_dir,
            rag_dir,
        }
    }
    
    /// Update the central reference hub with the latest analysis results
    pub fn update_central_reference_hub(&self, analysis: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Updating central reference hub...");
        
        // Ensure docs directory exists
        if !self.docs_dir.exists() {
            fs::create_dir_all(&self.docs_dir)
                .map_err(|e| format!("Failed to create docs directory: {}", e))?;
        }
        
        let hub_path = self.docs_dir.join("central_reference_hub.md");
        
        // Generate the hub content
        let content = self.generate_hub_content(analysis)?;
        
        // Write to file
        fs::write(&hub_path, content)
            .map_err(|e| format!("Failed to write central reference hub: {}", e))?;
        
        println!("Central reference hub updated: {:?}", hub_path);
        Ok(hub_path)
    }
    
    /// Generate content for the central reference hub
    fn generate_hub_content(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let now = Local::now();
        
        let mut content = String::new();
        
        // Header
        content.push_str("<!-- filepath: c:\\Users\\Tim\\Desktop\\LMS\\docs\\central_reference_hub.md -->\n");
        content.push_str("# LMS Project: Central Reference Hub\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", now.to_rfc3339()));
        
        // Project Overview
        content.push_str("## Project Overview\n\n");
        content.push_str("The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n\n");
        
        // Technology Stack
        content.push_str("## Technology Stack\n\n");
        
        // Frontend
        content.push_str("### Frontend\n");
        content.push_str("- Leptos\n");
        content.push_str("- Tauri\n\n");
        
        // Backend
        content.push_str("### Backend\n");
        content.push_str("- Rust\n");
        content.push_str("- Haskell\n\n");
        
        // Database
        content.push_str("### Database\n");
        content.push_str("- SQLite\n");
        content.push_str("- sqlx\n\n");
        
        // Search
        content.push_str("### Search\n");
        content.push_str("- MeiliSearch\n\n");
        
        // AI Integration
        content.push_str("### AI Integration\n");
        content.push_str("- Gemini\n\n");
        
        // Blockchain
        content.push_str("### Blockchain\n");
        content.push_str("- Custom Rust implementation\n\n");
        
        // Authentication
        content.push_str("### Authentication\n");
        content.push_str("- JWT\n\n");
        
        // Architecture Principles
        content.push_str("## Architecture Principles\n");
        content.push_str("- Clean Architecture\n");
        content.push_str("- SOLID\n");
        content.push_str("- Offline-first\n\n");
        
        // Design Patterns
        content.push_str("## Design Patterns\n");
        content.push_str("- CQRS\n");
        content.push_str("- Event Sourcing\n");
        content.push_str("- Repository Pattern\n\n");
        
        // Project Statistics
        content.push_str("## Project Statistics\n\n");
        
        // Add file statistics
        let file_stats = analysis.project_status.file_stats.clone();
        content.push_str(&format!("- **Total Files**: {}\n", file_stats.get("total").unwrap_or(&0)));
        content.push_str(&format!("- **Lines of Code**: {}\n", analysis.project_status.total_lines_of_code));
        content.push_str(&format!("- **Rust Files**: {}\n", file_stats.get("rs").unwrap_or(&0)));
        content.push_str(&format!("- **Haskell Files**: {}\n", file_stats.get("hs").unwrap_or(&0)));
        content.push_str("\n");
        
        // File Types Table
        content.push_str("### File Types\n\n");
        content.push_str("| Extension | Count |\n");
        content.push_str("|-----------|-------|\n");
        
        // Sort extensions for consistent display
        let mut extensions: Vec<(&String, &i32)> = file_stats.iter().collect();
        extensions.sort_by(|a, b| a.0.cmp(b.0));
        
        for (ext, count) in extensions {
            if ext != "total" {
                content.push_str(&format!("| {} | {} |\n", ext, count));
            }
        }
        content.push_str("\n");
        
        // Integration Status
        content.push_str("## Integration Status\n\n");
        content.push_str("| Integration | Source | Target | Status |\n");
        content.push_str("|-------------|--------|--------|--------|\n");
        
        // Add integration status from feature areas
        for (name, feature) in &analysis.feature_areas {
            content.push_str(&format!("| {} | {} | LMS | {} |\n", 
                name, 
                feature.source_system,
                match feature.completion_percentage {
                    p if p < 25 => "Planned",
                    p if p < 50 => "Started",
                    p if p < 75 => "In Progress",
                    p if p < 95 => "Almost Complete",
                    _ => "Complete"
                }
            ));
        }
        content.push_str("\n");
        
        // Models
        content.push_str("## Models\n\n");
        content.push_str(&format!("- **Total Models**: {}\n", analysis.models.total));
        content.push_str(&format!("- **Implemented Models**: {}\n", analysis.models.implemented));
        content.push_str(&format!("- **Implementation Percentage**: {}%\n\n", analysis.models.implementation_percentage));
        
        content.push_str("### Key Models\n\n");
        content.push_str("| Model | Source | Status | Description |\n");
        content.push_str("|-------|--------|--------|-------------|\n");
        
        // Add top models from analysis
        for model in analysis.models.key_models.iter().take(10) {
            content.push_str(&format!("| {} | {} | {} | {} |\n",
                model.name,
                model.source_system,
                match model.completion_percentage {
                    p if p < 25 => "Planned",
                    p if p < 50 => "Started",
                    p if p < 75 => "In Progress",
                    p if p < 95 => "Almost Complete",
                    _ => "Complete"
                },
                model.description
            ));
        }
        content.push_str("\n");
        
        // API Endpoints
        content.push_str("## API Endpoints\n\n");
        content.push_str(&format!("- **Total Endpoints**: {}\n", analysis.api_endpoints.total));
        content.push_str(&format!("- **Implemented Endpoints**: {}\n", analysis.api_endpoints.implemented));
        content.push_str(&format!("- **Implementation Percentage**: {}%\n\n", analysis.api_endpoints.implementation_percentage));
        
        // Architecture
        content.push_str("## Architecture\n\n");
        content.push_str(&format!("### Project Type: {}\n", analysis.architecture.project_type));
        content.push_str(&format!("### Architecture Style: {}\n\n", analysis.architecture.architecture_style));
        
        content.push_str("### Key Components\n\n");
        for component in &analysis.architecture.key_components {
            content.push_str(&format!("- **{}**: {}\n", component.name, component.description));
        }
        content.push_str("\n");
        
        // Blockchain
        content.push_str("## Blockchain Implementation\n\n");
        content.push_str(&format!("- **Status**: {}\n", analysis.blockchain.status));
        content.push_str(&format!("- **Implementation Type**: {}\n", analysis.blockchain.implementation_type));
        content.push_str(&format!("- **Features**: {}\n\n", analysis.blockchain.features.join(", ")));
        
        // Synchronization System
        content.push_str("## Synchronization System\n\n");
        content.push_str(&format!("- **Type**: {}\n", analysis.sync_system.sync_type));
        content.push_str(&format!("- **Strategy**: {}\n", analysis.sync_system.strategy));
        content.push_str(&format!("- **Status**: {}\n\n", analysis.sync_system.status));
        
        // Recent Changes
        content.push_str("## Recent Changes\n\n");
        if analysis.project_status.recent_changes.is_empty() {
            content.push_str("No changes detected in this analysis run.\n\n");
        } else {
            for change in &analysis.project_status.recent_changes {
                content.push_str(&format!("- {}\n", change));
            }
            content.push_str("\n");
        }
        
        // Next Steps
        content.push_str("## Next Steps\n\n");
        for (i, step) in analysis.project_status.next_steps.iter().enumerate() {
            content.push_str(&format!("{}. {}\n", i+1, step));
        }
        
        Ok(content)
    }
    
    /// Update RAG knowledge base with the latest analysis
    pub fn update_rag_knowledge_base(&self, analysis: &AnalysisResult) -> Result<(), String> {
        println!("Updating RAG knowledge base...");
        
        // Ensure RAG directory exists
        if !self.rag_dir.exists() {
            fs::create_dir_all(&self.rag_dir)
                .map_err(|e| format!("Failed to create RAG directory: {}", e))?;
        }
        
        // Update integration directory
        self.update_integration_knowledge(analysis)?;
        
        // Update canvas directory
        self.update_canvas_knowledge(analysis)?;
        
        // Update discourse directory
        self.update_discourse_knowledge(analysis)?;
        
        println!("RAG knowledge base updated successfully");
        Ok(())
    }
    
    /// Update integration knowledge in RAG knowledge base
    fn update_integration_knowledge(&self, analysis: &AnalysisResult) -> Result<(), String> {
        let integration_dir = self.rag_dir.join("integration");
        if !integration_dir.exists() {
            fs::create_dir_all(&integration_dir)
                .map_err(|e| format!("Failed to create integration directory: {}", e))?;
        }
        
        // Update architecture blueprint
        let architecture_path = integration_dir.join("architecture-blueprint.md");
        let architecture_content = self.generate_architecture_blueprint(analysis)?;
        fs::write(&architecture_path, architecture_content)
            .map_err(|e| format!("Failed to write architecture blueprint: {}", e))?;
        
        // Update integration points
        let points_path = integration_dir.join("integration_points.md");
        let points_content = self.generate_integration_points(analysis)?;
        fs::write(&points_path, points_content)
            .map_err(|e| format!("Failed to write integration points: {}", e))?;
        
        // Update synchronization implementation
        let sync_path = integration_dir.join("synchronization_implementation.md");
        let sync_content = self.generate_sync_implementation(analysis)?;
        fs::write(&sync_path, sync_content)
            .map_err(|e| format!("Failed to write synchronization implementation: {}", e))?;
        
        // Update blockchain implementation
        let blockchain_path = integration_dir.join("blockchain_implementation.md");
        if blockchain_path.exists() {
            let existing_content = fs::read_to_string(&blockchain_path)
                .map_err(|e| format!("Failed to read blockchain implementation: {}", e))?;
            
            let updated_content = self.update_blockchain_implementation(&existing_content, analysis)?;
            fs::write(&blockchain_path, updated_content)
                .map_err(|e| format!("Failed to write blockchain implementation: {}", e))?;
        }
        
        Ok(())
    }
    
    /// Generate architecture blueprint content
    fn generate_architecture_blueprint(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# LMS Integration Architecture Blueprint\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the architecture of the integrated LMS system, ");
        content.push_str("combining Canvas LMS and Discourse forum functionality into a unified Rust/Tauri/Leptos application.\n\n");
        
        // Architecture Style
        content.push_str("## Architecture Style\n\n");
        content.push_str(&format!("The system follows a **{}** architecture with the following characteristics:\n\n", 
            analysis.architecture.architecture_style));
        
        // Key components
        content.push_str("## Key Components\n\n");
        for component in &analysis.architecture.key_components {
            content.push_str(&format!("### {}\n\n", component.name));
            content.push_str(&format!("{}\n\n", component.description));
            
            if !component.files.is_empty() {
                content.push_str("**Key Files:**\n\n");
                for file in &component.files {
                    content.push_str(&format!("- `{}`\n", file));
                }
                content.push_str("\n");
            }
            
            if !component.dependencies.is_empty() {
                content.push_str("**Dependencies:**\n\n");
                for dep in &component.dependencies {
                    content.push_str(&format!("- {}\n", dep));
                }
                content.push_str("\n");
            }
        }
        
        // Communication Patterns
        content.push_str("## Communication Patterns\n\n");
        content.push_str("### Internal Communication\n\n");
        content.push_str("The system utilizes the following internal communication patterns:\n\n");
        content.push_str("1. **Event Sourcing**: State changes are stored as a sequence of events\n");
        content.push_str("2. **CQRS**: Separate command and query responsibilities\n");
        content.push_str("3. **Message Bus**: For decoupled component communication\n\n");
        
        content.push_str("### External Communication\n\n");
        content.push_str("External systems are integrated via:\n\n");
        content.push_str("1. **REST APIs**: Primary integration method\n");
        content.push_str("2. **Webhooks**: For real-time notifications\n");
        content.push_str("3. **Batch Synchronization**: For bulk data operations\n\n");
        
        // Data Flow
        content.push_str("## Data Flow\n\n");
        content.push_str("```\n");
        content.push_str("┌─────────────┐           ┌──────────────┐\n");
        content.push_str("│    Canvas   │◄─────────►│   Discourse  │\n");
        content.push_str("│    LMS      │   APIs    │    Forums    │\n");
        content.push_str("│             │           │              │\n");
        content.push_str("└─────────────┘           └──────────────┘\n");
        content.push_str("       ▲                         ▲\n");
        content.push_str("       │                         │\n");
        content.push_str("       │         ┌───────────────┘\n");
        content.push_str("       │         │\n");
        content.push_str("┌──────▼─────────▼──┐\n");
        content.push_str("│                   │\n");
        content.push_str("│   Integration     │\n");
        content.push_str("│   Service         │\n");
        content.push_str("│                   │\n");
        content.push_str("└───────────────────┘\n");
        content.push_str("       ▲\n");
        content.push_str("       │\n");
        content.push_str("┌──────▼──────┐\n");
        content.push_str("│             │\n");
        content.push_str("│  Database   │\n");
        content.push_str("│  (SQLite)   │\n");
        content.push_str("│             │\n");
        content.push_str("└─────────────┘\n");
        content.push_str("```\n\n");
        
        // Technology Stack
        content.push_str("## Technology Stack\n\n");
        content.push_str("- **Frontend**: Leptos, Tauri\n");
        content.push_str("- **Backend**: Rust, Haskell (for specialized components)\n");
        content.push_str("- **Database**: SQLite with sqlx for type-safe queries\n");
        content.push_str("- **Search**: MeiliSearch for fast, typo-tolerant search functionality\n");
        content.push_str("- **Authentication**: JWT-based authentication system\n");
        content.push_str("- **Blockchain**: Custom Rust implementation for data integrity\n\n");
        
        // Implementation Status
        content.push_str("## Implementation Status\n\n");
        content.push_str(&format!("Overall project completion: **{}%**\n\n", 
            (analysis.project_status.overall_completion_percentage).round()));
        
        content.push_str("| Component | Completion |\n");
        content.push_str("|-----------|------------|\n");
        content.push_str(&format!("| Models | {}% |\n", analysis.models.implementation_percentage));
        content.push_str(&format!("| API Endpoints | {}% |\n", analysis.api_endpoints.implementation_percentage));
        content.push_str(&format!("| UI Components | {}% |\n", analysis.ui_components.implementation_percentage));
        content.push_str(&format!("| Tests | {}% |\n", analysis.tests.coverage_percentage));
        
        Ok(content)
    }
    
    /// Generate integration points content
    fn generate_integration_points(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Canvas-Discourse Integration Points\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document outlines the key integration points between Canvas LMS and Discourse forum functionality.\n\n");
        
        // Core Integration Points
        content.push_str("## Core Integration Points\n\n");
        content.push_str("### 1. User Authentication and Identity\n\n");
        content.push_str("- **Approach**: Unified JWT authentication system\n");
        content.push_str("- **Status**: Implemented\n");
        content.push_str("- **Key Files**: `src-tauri/src/auth/jwt_service.rs`, `src-tauri/src/auth/providers.rs`\n\n");
        
        content.push_str("### 2. Course-Category Mapping\n\n");
        content.push_str("- **Approach**: Canvas courses map to Discourse categories\n");
        content.push_str("- **Status**: In Progress\n");
        content.push_str("- **Key Files**: `src-tauri/src/models/course.rs`, `src-tauri/src/models/category.rs`\n\n");
        
        content.push_str("### 3. Discussion Integration\n\n");
        content.push_str("- **Approach**: Canvas discussions map to Discourse topics\n");
        content.push_str("- **Status**: Planned\n");
        content.push_str("- **Key Files**: `src-tauri/src/models/discussion.rs`, `src-tauri/src/models/topic.rs`\n\n");
        
        content.push_str("### 4. Notification System\n\n");
        content.push_str("- **Approach**: Unified notification system\n");
        content.push_str("- **Status**: Started\n");
        content.push_str("- **Key Files**: `src-tauri/src/notifications/notification_service.rs`\n\n");
        
        content.push_str("### 5. Assignments and Grading\n\n");
        content.push_str("- **Approach**: Canvas assignments with Discourse discussion component\n");
        content.push_str("- **Status**: Planned\n");
        content.push_str("- **Key Files**: `src-tauri/src/models/assignment.rs`\n\n");
        
        // Model Mapping Table
        content.push_str("## Model Mapping\n\n");
        content.push_str("| Canvas Model | Discourse Model | Mapping Type | Description |\n");
        content.push_str("|--------------|-----------------|--------------|-------------|\n");
        content.push_str("| Course | Category | 1:1 | Each Canvas course maps to a Discourse category |\n");
        content.push_str("| Discussion | Topic | 1:1 | Canvas discussions become Discourse topics |\n");
        content.push_str("| DiscussionEntry | Post | 1:1 | Canvas discussion replies map to Discourse posts |\n");
        content.push_str("| User | User | 1:1 | Users have corresponding accounts in both systems |\n");
        content.push_str("| Group | Group | 1:1 | Canvas groups map to Discourse groups |\n");
        content.push_str("| Announcement | Topic | 1:1 | Canvas announcements become Discourse topics with special flags |\n");
        
        Ok(content)
    }
    
    /// Generate synchronization implementation content
    fn generate_sync_implementation(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Synchronization Implementation\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the implementation of the synchronization system between Canvas LMS and Discourse forum components.\n\n");
        
        // Sync Strategy
        content.push_str("## Synchronization Strategy\n\n");
        content.push_str(&format!("The application uses a **{}** approach for data synchronization with the following characteristics:\n\n", 
            analysis.sync_system.strategy));
        
        content.push_str("- **Offline-First**: Data is stored locally first, then synchronized when online\n");
        content.push_str("- **Conflict Resolution**: Uses timestamp-based resolution with user override option\n");
        content.push_str("- **Batch Processing**: Bulk operations for efficient synchronization\n");
        content.push_str("- **Delta Sync**: Only changed data is synchronized to minimize bandwidth usage\n\n");
        
        // Sync Components
        content.push_str("## Synchronization Components\n\n");
        
        content.push_str("### Sync Manager\n\n");
        content.push_str("Central component that orchestrates all synchronization processes.\n\n");
        content.push_str("- **Status**: Implemented\n");
        content.push_str("- **Key Files**: `src-tauri/src/sync/sync_manager.rs`\n\n");
        
        content.push_str("### Sync Queue\n\n");
        content.push_str("Maintains a queue of pending sync operations when offline.\n\n");
        content.push_str("- **Status**: Implemented\n");
        content.push_str("- **Key Files**: `src-tauri/src/sync/sync_queue.rs`\n\n");
        
        content.push_str("### Conflict Resolver\n\n");
        content.push_str("Handles data conflicts during synchronization.\n\n");
        content.push_str("- **Status**: In Progress\n");
        content.push_str("- **Key Files**: `src-tauri/src/sync/conflict_resolver.rs`\n\n");
        
        content.push_str("### Network Monitor\n\n");
        content.push_str("Monitors network availability to trigger sync when online.\n\n");
        content.push_str("- **Status**: Implemented\n");
        content.push_str("- **Key Files**: `src-tauri/src/utils/network_monitor.rs`\n\n");
        
        // Sync Process
        content.push_str("## Synchronization Process\n\n");
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
        
        // Implementation Status
        content.push_str("## Implementation Status\n\n");
        content.push_str(&format!("Overall synchronization system status: **{}**\n\n", analysis.sync_system.status));
        
        content.push_str("| Component | Status | Notes |\n");
        content.push_str("|-----------|--------|-------|\n");
        content.push_str("| Sync Manager | Complete | Core functionality implemented |\n");
        content.push_str("| Sync Queue | Complete | Persistent queue with SQLite storage |\n");
        content.push_str("| Conflict Resolver | In Progress | Basic resolution implemented |\n");
        content.push_str("| Delta Sync | Started | Initial implementation in progress |\n");
        content.push_str("| Network Monitor | Complete | Detects online/offline transitions |\n");
        
        Ok(content)
    }
    
    /// Update blockchain implementation based on existing content
    fn update_blockchain_implementation(&self, existing_content: &str, analysis: &AnalysisResult) -> Result<String, String> {
        // For blockchain, we'll preserve most of the existing content, but update the status and features sections
        
        // Simple implementation - in a real solution, you'd use a more robust approach
        let status_marker = "## Implementation Status";
        let features_marker = "## Features";
        
        let mut updated_content = String::from(existing_content);
        
        // Update the last updated timestamp
        let updated_time = format!("_Last updated: {}_", Local::now().to_rfc3339());
        if let Some(pos) = updated_content.find("_Last updated:") {
            let end_pos = updated_content[pos..].find('\n').unwrap_or(updated_content.len()) + pos;
            updated_content.replace_range(pos..end_pos, &updated_time);
        } else {
            // If no timestamp exists, add one after the title
            if let Some(pos) = updated_content.find('\n') {
                updated_content.insert_str(pos + 1, &format!("\n{}\n", updated_time));
            }
        }
        
        // Update the status section if found
        if let Some(status_pos) = updated_content.find(status_marker) {
            let next_section_pos = updated_content[status_pos + status_marker.len()..]
                .find("## ")
                .map(|pos| pos + status_pos + status_marker.len())
                .unwrap_or(updated_content.len());
            
            let status_content = format!("\n\n- **Current Status**: {}\n- **Completion**: {}%\n- **Integration Points**: {}\n\n",
                analysis.blockchain.status,
                analysis.blockchain.completion_percentage,
                analysis.blockchain.integration_points.join(", ")
            );
            
            updated_content.replace_range(
                status_pos + status_marker.len()..next_section_pos,
                &status_content
            );
        }
        
        // Update the features section if found
        if let Some(features_pos) = updated_content.find(features_marker) {
            let next_section_pos = updated_content[features_pos + features_marker.len()..]
                .find("## ")
                .map(|pos| pos + features_pos + features_marker.len())
                .unwrap_or(updated_content.len());
            
            let mut features_content = String::from("\n\n");
            for feature in &analysis.blockchain.features {
                features_content.push_str(&format!("- {}\n", feature));
            }
            features_content.push_str("\n");
            
            updated_content.replace_range(
                features_pos + features_marker.len()..next_section_pos,
                &features_content
            );
        }
        
        Ok(updated_content)
    }
    
    /// Update Canvas knowledge in RAG knowledge base
    fn update_canvas_knowledge(&self, analysis: &AnalysisResult) -> Result<(), String> {
        let canvas_dir = self.rag_dir.join("canvas");
        if !canvas_dir.exists() {
            fs::create_dir_all(&canvas_dir)
                .map_err(|e| format!("Failed to create canvas directory: {}", e))?;
        }
        
        // Update models documentation
        let models_path = canvas_dir.join("models.md");
        let models_content = self.generate_canvas_models(analysis)?;
        fs::write(&models_path, models_content)
            .map_err(|e| format!("Failed to write canvas models: {}", e))?;
        
        // Update API documentation 
        let api_path = canvas_dir.join("api.md");
        let api_content = self.generate_canvas_api(analysis)?;
        fs::write(&api_path, api_content)
            .map_err(|e| format!("Failed to write canvas API: {}", e))?;
        
        Ok(())
    }
    
    /// Generate Canvas models content
    fn generate_canvas_models(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Canvas LMS Models\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the key models in the Canvas LMS system that have been ported to the integrated LMS application.\n\n");
        
        // Models
        content.push_str("## Canvas Models\n\n");
        
        // Filter canvas models
        let canvas_models = analysis.models.key_models.iter()
            .filter(|m| m.source_system == "Canvas" || m.source_system == "canvas")
            .collect::<Vec<_>>();
        
        for model in canvas_models {
            content.push_str(&format!("### {}\n\n", model.name));
            content.push_str(&format!("{}\n\n", model.description));
            
            content.push_str("**Properties:**\n\n");
            if model.properties.is_empty() {
                content.push_str("*Properties information not available*\n\n");
            } else {
                content.push_str("| Name | Type | Description |\n");
                content.push_str("|------|------|-------------|\n");
                
                for prop in &model.properties {
                    content.push_str(&format!("| {} | {} | {} |\n", 
                        prop.name, 
                        prop.property_type,
                        prop.description
                    ));
                }
                content.push_str("\n");
            }
            
            content.push_str("**Relationships:**\n\n");
            if model.relationships.is_empty() {
                content.push_str("*No relationships defined*\n\n");
            } else {
                content.push_str("| Related Model | Relationship Type | Description |\n");
                content.push_str("|---------------|------------------|-------------|\n");
                
                for rel in &model.relationships {
                    content.push_str(&format!("| {} | {} | {} |\n", 
                        rel.target_model, 
                        rel.relationship_type,
                        rel.description
                    ));
                }
                content.push_str("\n");
            }
            
            content.push_str(&format!("**Implementation Status**: {}%\n\n", model.completion_percentage));
            
            if !model.files.is_empty() {
                content.push_str("**Key Files:**\n\n");
                for file in &model.files {
                    content.push_str(&format!("- `{}`\n", file));
                }
                content.push_str("\n");
            }
            
            content.push_str("---\n\n");
        }
        
        Ok(content)
    }
    
    /// Generate Canvas API content
    fn generate_canvas_api(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Canvas LMS API\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the key API endpoints from Canvas LMS that have been ported to the integrated LMS application.\n\n");
        
        // API Endpoints
        content.push_str("## API Endpoints\n\n");
        
        // Filter canvas endpoints
        let canvas_endpoints = analysis.api_endpoints.endpoints.iter()
            .filter(|e| e.source_system == "Canvas" || e.source_system == "canvas")
            .collect::<Vec<_>>();
        
        // Group endpoints by resource
        let mut grouped_endpoints: HashMap<String, Vec<&_>> = HashMap::new();
        for endpoint in canvas_endpoints {
            let resource = endpoint.resource.clone();
            grouped_endpoints.entry(resource).or_insert_with(Vec::new).push(endpoint);
        }
        
        // Sort resources alphabetically
        let mut resources: Vec<String> = grouped_endpoints.keys().cloned().collect();
        resources.sort();
        
        for resource in resources {
            content.push_str(&format!("### {}\n\n", resource));
            
            let endpoints = grouped_endpoints.get(&resource).unwrap();
            for endpoint in endpoints {
                content.push_str(&format!("#### {} {}\n\n", endpoint.method, endpoint.path));
                content.push_str(&format!("{}\n\n", endpoint.description));
                
                if !endpoint.parameters.is_empty() {
                    content.push_str("**Parameters:**\n\n");
                    content.push_str("| Name | Type | Required | Description |\n");
                    content.push_str("|------|------|----------|-------------|\n");
                    
                    for param in &endpoint.parameters {
                        content.push_str(&format!("| {} | {} | {} | {} |\n", 
                            param.name, 
                            param.param_type,
                            if param.required { "Yes" } else { "No" },
                            param.description
                        ));
                    }
                    content.push_str("\n");
                }
                
                content.push_str(&format!("**Implementation Status**: {}%\n\n", endpoint.completion_percentage));
                
                if !endpoint.files.is_empty() {
                    content.push_str("**Key Files:**\n\n");
                    for file in &endpoint.files {
                        content.push_str(&format!("- `{}`\n", file));
                    }
                    content.push_str("\n");
                }
                
                content.push_str("---\n\n");
            }
        }
        
        Ok(content)
    }
    
    /// Update Discourse knowledge in RAG knowledge base
    fn update_discourse_knowledge(&self, analysis: &AnalysisResult) -> Result<(), String> {
        let discourse_dir = self.rag_dir.join("discourse");
        if !discourse_dir.exists() {
            fs::create_dir_all(&discourse_dir)
                .map_err(|e| format!("Failed to create discourse directory: {}", e))?;
        }
        
        // Update models documentation
        let models_path = discourse_dir.join("models.md");
        let models_content = self.generate_discourse_models(analysis)?;
        fs::write(&models_path, models_content)
            .map_err(|e| format!("Failed to write discourse models: {}", e))?;
        
        // Update API documentation
        let api_path = discourse_dir.join("api.md");
        let api_content = self.generate_discourse_api(analysis)?;
        fs::write(&api_path, api_content)
            .map_err(|e| format!("Failed to write discourse API: {}", e))?;
        
        Ok(())
    }
    
    /// Generate Discourse models content
    fn generate_discourse_models(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Discourse Forum Models\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the key models in the Discourse forum system that have been ported to the integrated LMS application.\n\n");
        
        // Models
        content.push_str("## Discourse Models\n\n");
        
        // Filter discourse models
        let discourse_models = analysis.models.key_models.iter()
            .filter(|m| m.source_system == "Discourse" || m.source_system == "discourse")
            .collect::<Vec<_>>();
        
        for model in discourse_models {
            content.push_str(&format!("### {}\n\n", model.name));
            content.push_str(&format!("{}\n\n", model.description));
            
            content.push_str("**Properties:**\n\n");
            if model.properties.is_empty() {
                content.push_str("*Properties information not available*\n\n");
            } else {
                content.push_str("| Name | Type | Description |\n");
                content.push_str("|------|------|-------------|\n");
                
                for prop in &model.properties {
                    content.push_str(&format!("| {} | {} | {} |\n", 
                        prop.name, 
                        prop.property_type,
                        prop.description
                    ));
                }
                content.push_str("\n");
            }
            
            content.push_str("**Relationships:**\n\n");
            if model.relationships.is_empty() {
                content.push_str("*No relationships defined*\n\n");
            } else {
                content.push_str("| Related Model | Relationship Type | Description |\n");
                content.push_str("|---------------|------------------|-------------|\n");
                
                for rel in &model.relationships {
                    content.push_str(&format!("| {} | {} | {} |\n", 
                        rel.target_model, 
                        rel.relationship_type,
                        rel.description
                    ));
                }
                content.push_str("\n");
            }
            
            content.push_str(&format!("**Implementation Status**: {}%\n\n", model.completion_percentage));
            
            if !model.files.is_empty() {
                content.push_str("**Key Files:**\n\n");
                for file in &model.files {
                    content.push_str(&format!("- `{}`\n", file));
                }
                content.push_str("\n");
            }
            
            content.push_str("---\n\n");
        }
        
        Ok(content)
    }
    
    /// Generate Discourse API content
    fn generate_discourse_api(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header
        content.push_str("# Discourse Forum API\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        // Overview
        content.push_str("## Overview\n\n");
        content.push_str("This document describes the key API endpoints from Discourse forum that have been ported to the integrated LMS application.\n\n");
        
        // API Endpoints
        content.push_str("## API Endpoints\n\n");
        
        // Filter discourse endpoints
        let discourse_endpoints = analysis.api_endpoints.endpoints.iter()
            .filter(|e| e.source_system == "Discourse" || e.source_system == "discourse")
            .collect::<Vec<_>>();
        
        // Group endpoints by resource
        let mut grouped_endpoints: HashMap<String, Vec<&_>> = HashMap::new();
        for endpoint in discourse_endpoints {
            let resource = endpoint.resource.clone();
            grouped_endpoints.entry(resource).or_insert_with(Vec::new).push(endpoint);
        }
        
        // Sort resources alphabetically
        let mut resources: Vec<String> = grouped_endpoints.keys().cloned().collect();
        resources.sort();
        
        for resource in resources {
            content.push_str(&format!("### {}\n\n", resource));
            
            let endpoints = grouped_endpoints.get(&resource).unwrap();
            for endpoint in endpoints {
                content.push_str(&format!("#### {} {}\n\n", endpoint.method, endpoint.path));
                content.push_str(&format!("{}\n\n", endpoint.description));
                
                if !endpoint.parameters.is_empty() {
                    content.push_str("**Parameters:**\n\n");
                    content.push_str("| Name | Type | Required | Description |\n");
                    content.push_str("|------|------|----------|-------------|\n");
                    
                    for param in &endpoint.parameters {
                        content.push_str(&format!("| {} | {} | {} | {} |\n", 
                            param.name, 
                            param.param_type,
                            if param.required { "Yes" } else { "No" },
                            param.description
                        ));
                    }
                    content.push_str("\n");
                }
                
                content.push_str(&format!("**Implementation Status**: {}%\n\n", endpoint.completion_percentage));
                
                if !endpoint.files.is_empty() {
                    content.push_str("**Key Files:**\n\n");
                    for file in &endpoint.files {
                        content.push_str(&format!("- `{}`\n", file));
                    }
                    content.push_str("\n");
                }
                
                content.push_str("---\n\n");
            }
        }
        
        Ok(content)
    }
    
    /// Generate the last analysis results for AI agents
    pub fn generate_last_analysis_results(&self, analysis: &AnalysisResult) -> Result<PathBuf, String> {
        println!("Generating last analysis results for AI agents...");
        
        let output_path = self.docs_dir.join("LAST_ANALYSIS_RESULTS.md");
        
        // Generate the content
        let content = self.generate_ai_agent_content(analysis)?;
        
        // Write to file
        fs::write(&output_path, content)
            .map_err(|e| format!("Failed to write last analysis results: {}", e))?;
        
        println!("Last analysis results written to: {:?}", output_path);
        Ok(output_path)
    }
    
    /// Generate content for AI agents
    fn generate_ai_agent_content(&self, analysis: &AnalysisResult) -> Result<String, String> {
        let mut content = String::new();
        
        // Header designed specifically for AI agents
        content.push_str("# LMS Project Analysis Results for AI Agents\n\n");
        content.push_str(&format!("_Last updated: {}_\n\n", Local::now().to_rfc3339()));
        
        content.push_str("## AI AGENT INSTRUCTIONS\n\n");
        content.push_str("This document contains the latest analysis of the LMS project codebase. As an AI agent, your role is to:\n\n");
        content.push_str("1. Understand the architecture and implementation details outlined below\n");
        content.push_str("2. Provide assistance that aligns with the established architecture and technology choices\n");
        content.push_str("3. Suggest improvements that maintain consistency with the project's vision\n");
        content.push_str("4. Avoid suggesting JavaScript or TypeScript solutions - prefer Rust and Haskell implementations\n");
        content.push_str("5. Focus on helping implement the next steps listed at the end of this document\n\n");
        
        content.push_str("## Project Overview\n\n");
        content.push_str("The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum functionality into a unified application with these key characteristics:\n\n");
        content.push_str("- **Primary Languages**: Rust, Haskell\n");
        content.push_str("- **Frontend**: Leptos, Tauri\n");
        content.push_str("- **Database**: SQLite with sqlx\n");
        content.push_str("- **Architecture**: Clean Architecture with SOLID principles\n");
        content.push_str("- **Deployment Model**: Offline-first, multi-platform (desktop, web, mobile)\n\n");
        
        // Project Status
        content.push_str("## Current Project Status\n\n");
        content.push_str(&format!("Overall project completion: **{}%**\n\n", 
            (analysis.project_status.overall_completion_percentage).round()));
        
        content.push_str("| Component | Status | Completion |\n");
        content.push_str("|-----------|--------|------------|\n");
        content.push_str(&format!("| Models | {} | {}% |\n", 
            Self::status_text(analysis.models.implementation_percentage),
            analysis.models.implementation_percentage));
        content.push_str(&format!("| API Endpoints | {} | {}% |\n", 
            Self::status_text(analysis.api_endpoints.implementation_percentage),
            analysis.api_endpoints.implementation_percentage));
        content.push_str(&format!("| UI Components | {} | {}% |\n", 
            Self::status_text(analysis.ui_components.implementation_percentage),
            analysis.ui_components.implementation_percentage));
        content.push_str(&format!("| Tests | {} | {}% |\n", 
            Self::status_text(analysis.tests.coverage_percentage),
            analysis.tests.coverage_percentage));
        content.push_str("\n");
        
        // Architecture Specifics
        content.push_str("## Architecture Implementation\n\n");
        content.push_str(&format!("The project implements a **{}** architecture with these key components:\n\n", 
            analysis.architecture.architecture_style));
        
        for component in &analysis.architecture.key_components {
            content.push_str(&format!("### {}\n\n", component.name));
            content.push_str(&format!("{}\n\n", component.description));
            
            if !component.files.is_empty() {
                content.push_str("**Key Files:**\n\n");
                for file in &component.files {
                    content.push_str(&format!("- `{}`\n", file));
                }
                content.push_str("\n");
            }
        }
        
        // Technical Implementation Details
        content.push_str("## Technical Implementation Details\n\n");
        
        // Database
        content.push_str("### Database Implementation\n\n");
        content.push_str("The application uses SQLite with sqlx for type-safe queries. Key aspects:\n\n");
        content.push_str("- All database access is through the Repository pattern\n");
        content.push_str("- Migrations are managed with sqlx-cli\n");
        content.push_str("- Offline-first design with local database that syncs when online\n\n");
        
        // Authentication
        content.push_str("### Authentication System\n\n");
        content.push_str("JWT-based authentication system with:\n\n");
        content.push_str("- Token-based auth with refresh tokens\n");
        content.push_str("- Integration with Canvas OAuth and Discourse SSO\n");
        content.push_str("- Role-based access control\n\n");
        
        // Blockchain
        content.push_str("### Blockchain Implementation\n\n");
        content.push_str(&format!("Status: **{}**\n\n", analysis.blockchain.status));
        content.push_str("The blockchain component provides:\n\n");
        for feature in &analysis.blockchain.features {
            content.push_str(&format!("- {}\n", feature));
        }
        content.push_str("\n");
        
        // Models and Mappings
        content.push_str("## Model Integration\n\n");
        content.push_str("Key model mappings between Canvas and Discourse:\n\n");
        content.push_str("| Canvas Model | Discourse Model | Mapping Type |\n");
        content.push_str("|--------------|-----------------|---------------|\n");
        content.push_str("| Course | Category | 1:1 |\n");
        content.push_str("| Discussion | Topic | 1:1 |\n");
        content.push_str("| DiscussionEntry | Post | 1:1 |\n");
        content.push_str("| User | User | 1:1 |\n");
        content.push_str("| Group | Group | 1:1 |\n");
        content.push_str("\n");
        
        // Sync System
        content.push_str("## Synchronization System\n\n");
        content.push_str(&format!("The application uses a **{}** synchronization strategy with **{}** status.\n\n", 
            analysis.sync_system.strategy,
            analysis.sync_system.status));
        
        // Recent Changes
        content.push_str("## Recent Changes\n\n");
        if analysis.project_status.recent_changes.is_empty() {
            content.push_str("No significant changes detected in this analysis run.\n\n");
        } else {
            for change in &analysis.project_status.recent_changes {
                content.push_str(&format!("- {}\n", change));
            }
            content.push_str("\n");
        }
        
        // Next Steps
        content.push_str("## Next Development Steps\n\n");
        for (i, step) in analysis.project_status.next_steps.iter().enumerate() {
            content.push_str(&format!("{}. {}\n", i+1, step));
        }
        content.push_str("\n");
        
        // Technical Debt
        content.push_str("## Current Technical Debt\n\n");
        for issue in &analysis.code_quality.tech_debt_items {
            content.push_str(&format!("- **{}**: {}\n", issue.category, issue.description));
        }
        content.push_str("\n");
        
        // Critical Files
        content.push_str("## Critical Files for Understanding\n\n");
        for file in &analysis.project_status.critical_files {
            content.push_str(&format!("- `{}`: {}\n", file.path, file.description));
        }
        
        Ok(content)
    }
    
    // Helper function to convert percentage to status text
    fn status_text(percentage: f32) -> &'static str {
        match percentage as i32 {
            0..=10 => "Planning",
            11..=25 => "Started",
            26..=50 => "In Progress",
            51..=75 => "Advanced",
            76..=95 => "Almost Complete",
            _ => "Complete",
        }
    }
}
