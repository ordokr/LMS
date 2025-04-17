use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use chrono::Local;

use crate::output_schema::UnifiedAnalysisOutput;

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

        // Create visualizations directory if it doesn't exist
        let vis_dir = output_dir.join("visualizations").join("migration_roadmap");
        fs::create_dir_all(&vis_dir).map_err(|e| format!("Failed to create visualizations directory: {}", e))?;

        // Generate the roadmap
        let roadmap = self.create_roadmap(output)?;

        // Save as JSON
        let json_path = vis_dir.join("migration_roadmap.json");
        let json = serde_json::to_string_pretty(&roadmap)
            .map_err(|e| format!("Failed to serialize roadmap to JSON: {}", e))?;
        fs::write(&json_path, json)
            .map_err(|e| format!("Failed to write roadmap JSON: {}", e))?;

        // Save as Markdown
        let md_path = vis_dir.join("migration_roadmap.md");
        let markdown = self.generate_markdown(&roadmap)?;
        fs::write(&md_path, markdown)
            .map_err(|e| format!("Failed to write roadmap Markdown: {}", e))?;

        // Generate HTML visualization
        let html = self.generate_html(&roadmap)?;
        let html_path = vis_dir.join("migration_roadmap.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write roadmap HTML: {}", e))?;

        // Update the existing implementation roadmap documentation to include a link to the visualization
        self.update_implementation_documentation(output_dir)?;

        println!("Migration roadmap generated at:");
        println!("  - JSON: {}", json_path.display());
        println!("  - Markdown: {}", md_path.display());
        println!("  - HTML: {}", html_path.display());

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
        for (name, _component) in &output.components {
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
        for endpoint in &output.api.endpoints {
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

    fn generate_html(&self, roadmap: &MigrationRoadmap) -> Result<String, String> {
        let mut html = String::new();

        // Create a simple HTML page with the roadmap information
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("    <title>{}</title>\n", roadmap.project_name));
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; line-height: 1.6; margin: 0; padding: 20px; color: #333; }\n");
        html.push_str("        h1, h2, h3 { color: #2c3e50; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; }\n");
        html.push_str("        .phase { background-color: #f9f9f9; border-radius: 5px; padding: 15px; margin-bottom: 20px; border-left: 5px solid #3498db; }\n");
        html.push_str("        .phase-header { display: flex; justify-content: space-between; align-items: center; }\n");
        html.push_str("        .phase-title { margin: 0; }\n");
        html.push_str("        .phase-effort { font-weight: bold; color: #e74c3c; }\n");
        html.push_str("        .phase-description { margin-top: 10px; color: #7f8c8d; }\n");
        html.push_str("        .section { margin-top: 10px; }\n");
        html.push_str("        .section-title { font-weight: bold; margin-bottom: 5px; color: #2980b9; }\n");
        html.push_str("        ul { margin-top: 5px; }\n");
        html.push_str("        .timeline { margin-top: 30px; }\n");
        html.push_str("        .critical-path { background-color: #fef9e7; border-radius: 5px; padding: 15px; margin-top: 20px; border-left: 5px solid #f1c40f; }\n");
        html.push_str("        .risks { background-color: #fdedec; border-radius: 5px; padding: 15px; margin-top: 20px; border-left: 5px solid #e74c3c; }\n");
        html.push_str("        .recommendations { background-color: #eafaf1; border-radius: 5px; padding: 15px; margin-top: 20px; border-left: 5px solid #2ecc71; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <div class=\"container\">\n");

        // Header
        html.push_str(&format!("        <h1>{}</h1>\n", roadmap.project_name));
        html.push_str(&format!("        <p>Generated: {}</p>\n", roadmap.generated_date));
        html.push_str(&format!("        <p>Total Estimated Effort: <strong>{}</strong></p>\n", roadmap.total_estimated_effort));

        // Phases
        html.push_str("        <h2>Migration Phases</h2>\n");

        for phase in &roadmap.phases {
            html.push_str("        <div class=\"phase\">\n");
            html.push_str("            <div class=\"phase-header\">\n");
            html.push_str(&format!("                <h3 class=\"phase-title\">{}</h3>\n", phase.name));
            html.push_str(&format!("                <span class=\"phase-effort\">{}</span>\n", phase.estimated_effort));
            html.push_str("            </div>\n");
            html.push_str(&format!("            <p class=\"phase-description\">{}</p>\n", phase.description));

            if !phase.dependencies.is_empty() {
                html.push_str("            <div class=\"section\">\n");
                html.push_str("                <div class=\"section-title\">Dependencies:</div>\n");
                html.push_str("                <ul>\n");
                for dep in &phase.dependencies {
                    html.push_str(&format!("                    <li>{}</li>\n", dep));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            if !phase.components.is_empty() {
                html.push_str("            <div class=\"section\">\n");
                html.push_str("                <div class=\"section-title\">Components:</div>\n");
                html.push_str("                <ul>\n");
                for component in &phase.components {
                    html.push_str(&format!("                    <li>{}</li>\n", component));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            if !phase.apis.is_empty() {
                html.push_str("            <div class=\"section\">\n");
                html.push_str("                <div class=\"section-title\">APIs:</div>\n");
                html.push_str("                <ul>\n");
                for api in &phase.apis {
                    html.push_str(&format!("                    <li>{}</li>\n", api));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            if !phase.routes.is_empty() {
                html.push_str("            <div class=\"section\">\n");
                html.push_str("                <div class=\"section-title\">Routes:</div>\n");
                html.push_str("                <ul>\n");
                for route in &phase.routes {
                    html.push_str(&format!("                    <li>{}</li>\n", route));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            if !phase.database_tables.is_empty() {
                html.push_str("            <div class=\"section\">\n");
                html.push_str("                <div class=\"section-title\">Database Tables:</div>\n");
                html.push_str("                <ul>\n");
                for table in &phase.database_tables {
                    html.push_str(&format!("                    <li>{}</li>\n", table));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        // Critical Path
        html.push_str("        <div class=\"critical-path\">\n");
        html.push_str("            <h2>Critical Path Items</h2>\n");
        html.push_str("            <ul>\n");
        for item in &roadmap.critical_path_items {
            html.push_str(&format!("                <li>{}</li>\n", item));
        }
        html.push_str("            </ul>\n");
        html.push_str("        </div>\n");

        // Risks
        html.push_str("        <div class=\"risks\">\n");
        html.push_str("            <h2>Risks</h2>\n");
        html.push_str("            <ul>\n");
        for risk in &roadmap.risks {
            html.push_str(&format!("                <li>{}</li>\n", risk));
        }
        html.push_str("            </ul>\n");
        html.push_str("        </div>\n");

        // Recommendations
        html.push_str("        <div class=\"recommendations\">\n");
        html.push_str("            <h2>Recommendations</h2>\n");
        html.push_str("            <ul>\n");
        for rec in &roadmap.recommendations {
            html.push_str(&format!("                <li>{}</li>\n", rec));
        }
        html.push_str("            </ul>\n");
        html.push_str("        </div>\n");

        html.push_str("    </div>\n");
        html.push_str("</body>\n");
        html.push_str("</html>\n");

        Ok(html)
    }

    /// Updates the existing implementation roadmap documentation to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_implementation_documentation(&self, output_dir: &PathBuf) -> Result<(), String> {
        let impl_doc_path = output_dir.join("implementation_roadmap.md");

        // Check if the implementation roadmap documentation file exists
        if !impl_doc_path.exists() {
            return Ok(());
        }

        // Read the existing implementation roadmap documentation
        let content = fs::read_to_string(&impl_doc_path)
            .map_err(|e| format!("Failed to read implementation roadmap documentation: {}", e))?;

        // Check if the visualization link already exists
        if content.contains("Migration Roadmap Visualization") {
            return Ok(());
        }

        // Add the visualization link to the implementation roadmap documentation
        let updated_content = format!("{}

## Migration Roadmap Visualization

For a detailed visualization of the migration roadmap, see:

- [Migration Roadmap (HTML)](visualizations/migration_roadmap/migration_roadmap.html)
- [Migration Roadmap (Markdown)](visualizations/migration_roadmap/migration_roadmap.md)
- [Migration Roadmap (JSON)](visualizations/migration_roadmap/migration_roadmap.json)
", content);

        // Write the updated implementation roadmap documentation
        fs::write(&impl_doc_path, updated_content)
            .map_err(|e| format!("Failed to write updated implementation roadmap documentation: {}", e))?;

        // Update the central reference hub
        self.update_central_reference_hub(output_dir)?;

        Ok(())
    }

    /// Updates the central reference hub to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_central_reference_hub(&self, output_dir: &PathBuf) -> Result<(), String> {
        let hub_path = output_dir.join("central_reference_hub.md");

        // Update the central reference hub if it exists
        if hub_path.exists() {
            let content = fs::read_to_string(&hub_path)
                .map_err(|e| format!("Failed to read central reference hub: {}", e))?;

            // Check if the visualizations section already exists
            if !content.contains("## Visualizations") {
                // Add the visualizations section to the central reference hub
                let updated_content = format!("{}

## Visualizations

- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html)
", content);

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            } else if !content.contains("[Migration Roadmap]") {
                // Add the migration roadmap link to the existing visualizations section
                let updated_content = content.replace("## Visualizations\n\n", "## Visualizations\n\n- [Migration Roadmap](visualizations/migration_roadmap/migration_roadmap.html)\n");

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            }
        }

        // Update the visualizations README.md file
        let vis_readme_path = output_dir.join("visualizations").join("README.md");
        if vis_readme_path.exists() {
            let content = fs::read_to_string(&vis_readme_path)
                .map_err(|e| format!("Failed to read visualizations README: {}", e))?;

            // Check if the migration roadmap link already exists
            if !content.contains("[Migration Roadmap]") {
                // Add the migration roadmap link to the README
                let updated_content = format!("{}
- [Migration Roadmap](migration_roadmap/migration_roadmap.html)", content);

                // Write the updated README
                fs::write(&vis_readme_path, updated_content)
                    .map_err(|e| format!("Failed to write updated visualizations README: {}", e))?;
            }
        } else {
            // Create the README.md file
            let vis_readme_content = "# Visualizations\n\n- [Migration Roadmap](migration_roadmap/migration_roadmap.html)\n";

            fs::write(&vis_readme_path, vis_readme_content)
                .map_err(|e| format!("Failed to write visualizations README: {}", e))?;
        }

        Ok(())
    }
}
