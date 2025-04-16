use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::Local;

use crate::output_schema::{
    UnifiedAnalysisOutput, 
    FileInfo, 
    ComponentInfo, 
    ApiEndpointInfo, 
    RouteInfo, 
    DatabaseTableInfo,
    AuthInfo
};

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationPhase {
    pub name: String,
    pub description: String,
    pub components: Vec<String>,
    pub apis: Vec<String>,
    pub routes: Vec<String>,
    pub database_tables: Vec<String>,
    pub estimated_effort: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationRoadmap {
    pub project_name: String,
    pub generated_date: String,
    pub phases: Vec<MigrationPhase>,
    pub total_estimated_effort: String,
    pub critical_path_items: Vec<String>,
    pub risks: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct MigrationRoadmapGenerator;

impl MigrationRoadmapGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating migration roadmap...");

        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
        }

        // Generate the roadmap
        let roadmap = self.create_roadmap(output)?;

        // Save as JSON
        let json_path = output_dir.join("migration_roadmap.json");
        let json = serde_json::to_string_pretty(&roadmap)
            .map_err(|e| format!("Failed to serialize roadmap to JSON: {}", e))?;
        fs::write(&json_path, json)
            .map_err(|e| format!("Failed to write roadmap JSON: {}", e))?;

        // Save as Markdown
        let md_path = output_dir.join("migration_roadmap.md");
        let markdown = self.generate_markdown(&roadmap)?;
        fs::write(&md_path, markdown)
            .map_err(|e| format!("Failed to write roadmap Markdown: {}", e))?;

        println!("Migration roadmap generated at:");
        println!("  - JSON: {}", json_path.display());
        println!("  - Markdown: {}", md_path.display());

        Ok(())
    }

    fn create_roadmap(&self, output: &UnifiedAnalysisOutput) -> Result<MigrationRoadmap, String> {
        // Analyze the components, APIs, routes, and database tables to determine dependencies
        // and group them into logical phases

        // Phase 1: Core Infrastructure
        let mut phase1 = MigrationPhase {
            name: "Core Infrastructure".to_string(),
            description: "Set up the basic Rust/Tauri/Leptos infrastructure and implement core database models".to_string(),
            components: Vec::new(),
            apis: Vec::new(),
            routes: Vec::new(),
            database_tables: Vec::new(),
            estimated_effort: "4 weeks".to_string(),
            dependencies: Vec::new(),
        };

        // Phase 2: Authentication & User Management
        let mut phase2 = MigrationPhase {
            name: "Authentication & User Management".to_string(),
            description: "Implement user authentication, registration, and profile management".to_string(),
            components: Vec::new(),
            apis: Vec::new(),
            routes: Vec::new(),
            database_tables: Vec::new(),
            estimated_effort: "3 weeks".to_string(),
            dependencies: vec!["Core Infrastructure".to_string()],
        };

        // Phase 3: Content Management
        let mut phase3 = MigrationPhase {
            name: "Content Management".to_string(),
            description: "Implement content creation, editing, and viewing functionality".to_string(),
            components: Vec::new(),
            apis: Vec::new(),
            routes: Vec::new(),
            database_tables: Vec::new(),
            estimated_effort: "5 weeks".to_string(),
            dependencies: vec!["Authentication & User Management".to_string()],
        };

        // Phase 4: Interaction Features
        let mut phase4 = MigrationPhase {
            name: "Interaction Features".to_string(),
            description: "Implement commenting, voting, notifications, and other interaction features".to_string(),
            components: Vec::new(),
            apis: Vec::new(),
            routes: Vec::new(),
            database_tables: Vec::new(),
            estimated_effort: "4 weeks".to_string(),
            dependencies: vec!["Content Management".to_string()],
        };

        // Phase 5: Offline Capabilities
        let mut phase5 = MigrationPhase {
            name: "Offline Capabilities".to_string(),
            description: "Implement offline-first functionality with data synchronization".to_string(),
            components: Vec::new(),
            apis: Vec::new(),
            routes: Vec::new(),
            database_tables: Vec::new(),
            estimated_effort: "6 weeks".to_string(),
            dependencies: vec!["Interaction Features".to_string()],
        };

        // Assign components to phases
        for (name, component) in &output.components {
            if name.contains("User") || name.contains("Auth") || name.contains("Login") || name.contains("Register") {
                phase2.components.push(name.clone());
            } else if name.contains("Post") || name.contains("Topic") || name.contains("Article") || name.contains("Content") {
                phase3.components.push(name.clone());
            } else if name.contains("Comment") || name.contains("Vote") || name.contains("Notification") {
                phase4.components.push(name.clone());
            } else if name.contains("Sync") || name.contains("Offline") {
                phase5.components.push(name.clone());
            } else {
                // Default to core infrastructure
                phase1.components.push(name.clone());
            }
        }

        // Assign APIs to phases
        for endpoint in &output.api_map {
            if endpoint.path.contains("/auth") || endpoint.path.contains("/users") {
                phase2.apis.push(endpoint.path.clone());
            } else if endpoint.path.contains("/posts") || endpoint.path.contains("/topics") || endpoint.path.contains("/articles") {
                phase3.apis.push(endpoint.path.clone());
            } else if endpoint.path.contains("/comments") || endpoint.path.contains("/votes") || endpoint.path.contains("/notifications") {
                phase4.apis.push(endpoint.path.clone());
            } else if endpoint.path.contains("/sync") {
                phase5.apis.push(endpoint.path.clone());
            } else {
                // Default to core infrastructure
                phase1.apis.push(endpoint.path.clone());
            }
        }

        // Assign routes to phases
        for route in &output.routes {
            if route.path.contains("/auth") || route.path.contains("/users") || route.path.contains("/login") || route.path.contains("/register") {
                phase2.routes.push(route.path.clone());
            } else if route.path.contains("/posts") || route.path.contains("/topics") || route.path.contains("/articles") {
                phase3.routes.push(route.path.clone());
            } else if route.path.contains("/comments") || route.path.contains("/votes") || route.path.contains("/notifications") {
                phase4.routes.push(route.path.clone());
            } else if route.path.contains("/sync") || route.path.contains("/offline") {
                phase5.routes.push(route.path.clone());
            } else {
                // Default to core infrastructure
                phase1.routes.push(route.path.clone());
            }
        }

        // Assign database tables to phases
        for table in &output.database.tables {
            if table.name.contains("user") || table.name.contains("auth") || table.name.contains("session") {
                phase2.database_tables.push(table.name.clone());
            } else if table.name.contains("post") || table.name.contains("topic") || table.name.contains("article") || table.name.contains("content") {
                phase3.database_tables.push(table.name.clone());
            } else if table.name.contains("comment") || table.name.contains("vote") || table.name.contains("notification") {
                phase4.database_tables.push(table.name.clone());
            } else if table.name.contains("sync") || table.name.contains("offline") {
                phase5.database_tables.push(table.name.clone());
            } else {
                // Default to core infrastructure
                phase1.database_tables.push(table.name.clone());
            }
        }

        // Create the roadmap
        let roadmap = MigrationRoadmap {
            project_name: "Discourse + Canvas Migration to Rust/Tauri/Leptos".to_string(),
            generated_date: Local::now().format("%Y-%m-%d").to_string(),
            phases: vec![phase1, phase2, phase3, phase4, phase5],
            total_estimated_effort: "22 weeks".to_string(),
            critical_path_items: vec![
                "Database schema design".to_string(),
                "Authentication system".to_string(),
                "Offline data synchronization".to_string(),
                "Content editor component".to_string(),
            ],
            risks: vec![
                "Complex offline synchronization logic may require additional time".to_string(),
                "Integration between Canvas and Discourse features may be challenging".to_string(),
                "Performance optimization for large datasets".to_string(),
            ],
            recommendations: vec![
                "Start with a small subset of features for initial proof of concept".to_string(),
                "Implement comprehensive test suite early in the process".to_string(),
                "Use feature flags to enable incremental deployment".to_string(),
                "Consider using CRDT for conflict resolution in offline mode".to_string(),
            ],
        };

        Ok(roadmap)
    }

    fn generate_markdown(&self, roadmap: &MigrationRoadmap) -> Result<String, String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str(&format!("# Migration Roadmap: {}\n\n", roadmap.project_name));
        markdown.push_str(&format!("Generated: {}\n\n", roadmap.generated_date));
        markdown.push_str(&format!("Total Estimated Effort: **{}**\n\n", roadmap.total_estimated_effort));

        // Phases
        markdown.push_str("## Migration Phases\n\n");

        for phase in &roadmap.phases {
            markdown.push_str(&format!("### Phase: {}\n\n", phase.name));
            markdown.push_str(&format!("**Description**: {}\n\n", phase.description));
            markdown.push_str(&format!("**Estimated Effort**: {}\n\n", phase.estimated_effort));
            
            if !phase.dependencies.is_empty() {
                markdown.push_str("**Dependencies**:\n");
                for dep in &phase.dependencies {
                    markdown.push_str(&format!("- {}\n", dep));
                }
                markdown.push_str("\n");
            }

            if !phase.components.is_empty() {
                markdown.push_str("**Components**:\n");
                for component in &phase.components {
                    markdown.push_str(&format!("- {}\n", component));
                }
                markdown.push_str("\n");
            }

            if !phase.apis.is_empty() {
                markdown.push_str("**APIs**:\n");
                for api in &phase.apis {
                    markdown.push_str(&format!("- {}\n", api));
                }
                markdown.push_str("\n");
            }

            if !phase.routes.is_empty() {
                markdown.push_str("**Routes**:\n");
                for route in &phase.routes {
                    markdown.push_str(&format!("- {}\n", route));
                }
                markdown.push_str("\n");
            }

            if !phase.database_tables.is_empty() {
                markdown.push_str("**Database Tables**:\n");
                for table in &phase.database_tables {
                    markdown.push_str(&format!("- {}\n", table));
                }
                markdown.push_str("\n");
            }
        }

        // Critical Path
        markdown.push_str("## Critical Path Items\n\n");
        for item in &roadmap.critical_path_items {
            markdown.push_str(&format!("- {}\n", item));
        }
        markdown.push_str("\n");

        // Risks
        markdown.push_str("## Risks\n\n");
        for risk in &roadmap.risks {
            markdown.push_str(&format!("- {}\n", risk));
        }
        markdown.push_str("\n");

        // Recommendations
        markdown.push_str("## Recommendations\n\n");
        for rec in &roadmap.recommendations {
            markdown.push_str(&format!("- {}\n", rec));
        }

        // Gantt Chart
        markdown.push_str("\n## Timeline\n\n");
        markdown.push_str("```mermaid\ngantt\n");
        markdown.push_str("    title Migration Roadmap\n");
        markdown.push_str("    dateFormat  YYYY-MM-DD\n");
        markdown.push_str("    section Core Infrastructure\n");
        
        // Start date is today
        let start_date = Local::now().format("%Y-%m-%d").to_string();
        
        // Add 4 weeks for phase 1
        markdown.push_str(&format!("    Core Infrastructure: p1, {}, 4w\n", start_date));
        
        // Phase 2 starts after phase 1
        markdown.push_str("    section Authentication & User Management\n");
        markdown.push_str("    Authentication & User Management: p2, after p1, 3w\n");
        
        // Phase 3 starts after phase 2
        markdown.push_str("    section Content Management\n");
        markdown.push_str("    Content Management: p3, after p2, 5w\n");
        
        // Phase 4 starts after phase 3
        markdown.push_str("    section Interaction Features\n");
        markdown.push_str("    Interaction Features: p4, after p3, 4w\n");
        
        // Phase 5 starts after phase 4
        markdown.push_str("    section Offline Capabilities\n");
        markdown.push_str("    Offline Capabilities: p5, after p4, 6w\n");
        
        markdown.push_str("```\n");

        Ok(markdown)
    }
}
