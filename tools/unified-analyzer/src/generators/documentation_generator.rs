use anyhow::{Context, Result};
use chrono::Local;
use log::info;
use serde_json::Value;
// use crate::output_schema::UnifiedOutput;
use std::fs;
use std::path::PathBuf;

pub fn generate_documentation(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    // Ensure docs directory exists
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)
            .context("Failed to create docs directory")?;
    }

    // Create subdirectories
    let api_dir = docs_dir.join("api");
    let architecture_dir = docs_dir.join("architecture");
    let models_dir = docs_dir.join("models");
    let integration_dir = docs_dir.join("integration");
    let technical_dir = docs_dir.join("technical");
    let visualizations_dir = docs_dir.join("visualizations");
    let analysis_dir = docs_dir.join("analysis");

    // Create directories if they don't exist
    for dir in &[&api_dir, &architecture_dir, &models_dir, &integration_dir,
                 &technical_dir, &visualizations_dir, &analysis_dir] {
        if !dir.exists() {
            fs::create_dir_all(dir)
                .context(format!("Failed to create directory: {}", dir.display()))?;
        }
    }

    info!("Generating documentation in {}", docs_dir.display());

    // Generate central reference hub
    generate_central_hub(unified_output, base_dir)?;

    // Generate file structure documentation
    generate_file_structure_doc(unified_output, base_dir)?;

    // Generate API documentation
    generate_api_doc(unified_output, base_dir)?;

    // Generate database schema documentation
    generate_database_schema_doc(unified_output, base_dir)?;

    // Generate business logic documentation
    generate_business_logic_doc(unified_output, base_dir)?;

    // Generate offline-first readiness report
    generate_offline_readiness_doc(unified_output, base_dir)?;

    // Generate architecture overview
    generate_architecture_overview(unified_output, base_dir)?;

    // Generate implementation roadmap
    generate_implementation_roadmap(unified_output, base_dir)?;

    info!("Documentation generation completed");
    Ok(())
}

fn generate_central_hub(_unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Central Reference Hub\n\n");
    content.push_str(&format!("Generated on: {}\n\n", Local::now().format("%Y-%m-%d %H:%M:%S")));

    content.push_str("## Table of Contents\n\n");
    content.push_str("- [File Structure](file_structure.md)\n");
    content.push_str("- [API Documentation](api_documentation.md)\n");
    content.push_str("- [Database Schema](database_schema.md)\n");
    content.push_str("- [Business Logic](business_logic.md)\n");
    content.push_str("- [Offline-First Readiness](offline_readiness.md)\n");
    content.push_str("- [Architecture Overview](architecture_overview.md)\n");
    content.push_str("- [Implementation Roadmap](implementation_roadmap.md)\n\n");

    content.push_str("## Project Overview\n\n");
    content.push_str("This documentation is generated from a comprehensive analysis of the Canvas and Discourse codebases. ");
    content.push_str("It provides insights into the structure, functionality, and implementation details of both systems, ");
    content.push_str("with the goal of recreating their functionality in a Rust/Tauri/Leptos environment with offline-first capabilities.\n\n");

    content.push_str("### Analysis Scope\n\n");
    content.push_str("- **File Structure**: Complete mapping of all source files and their relationships\n");
    content.push_str("- **Code Logic**: Full understanding of business logic implementation (backend + frontend)\n");
    content.push_str("- **API Flows**: Comprehensive API surface area and data flow mapping\n");
    content.push_str("- **Routing**: Complete route mapping and parameter handling\n");
    content.push_str("- **Template Rendering**: UI component structure and rendering logic\n");
    content.push_str("- **Authentication/Authorization**: Security model and permission system\n");
    content.push_str("- **Offline-First Readiness**: Data synchronization and offline capabilities\n\n");

    let path = base_dir.join("docs").join("index.md");
    std::fs::write(&path, content).context("Failed to write central hub documentation")?;
    info!("Generated central reference hub at {}", path.display());
    Ok(())
}

fn generate_file_structure_doc(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# File Structure Analysis\n\n");

    if let Some(file_structure) = unified_output.get("file_structure") {
        // Extract key directories and their purposes
        content.push_str("## Key Directories\n\n");

        if let Some(directories) = file_structure.get("key_directories") {
            if let Some(dirs_array) = directories.as_array() {
                for dir in dirs_array {
                    if let (Some(path), Some(purpose)) = (dir.get("path"), dir.get("purpose")) {
                        content.push_str(&format!("- **{}**: {}\n",
                            path.as_str().unwrap_or("Unknown"),
                            purpose.as_str().unwrap_or("Unknown purpose")));
                    }
                }
            }
        }

        content.push_str("\n## File Statistics\n\n");

        // Add file type statistics if available
        if let Some(stats) = file_structure.get("file_type_stats") {
            content.push_str("### File Types\n\n");
            content.push_str("| File Type | Count |\n|-----------|-------|\n");

            if let Some(stats_obj) = stats.as_object() {
                for (file_type, count) in stats_obj {
                    content.push_str(&format!("| {} | {} |\n", file_type, count));
                }
            }
        }

        // Add file dependency graph information
        content.push_str("\n## File Dependencies\n\n");
        content.push_str("The analysis has identified the following key file dependencies:\n\n");

        if let Some(dependencies) = file_structure.get("file_dependencies") {
            if let Some(deps_obj) = dependencies.as_object() {
                let mut count = 0;
                for (file, deps) in deps_obj {
                    if count >= 10 { // Limit to 10 examples to keep the doc manageable
                        break;
                    }

                    content.push_str(&format!("- **{}** depends on:\n", file));

                    if let Some(deps_array) = deps.as_array() {
                        for dep in deps_array.iter().take(5) { // Show at most 5 dependencies per file
                            content.push_str(&format!("  - {}\n", dep.as_str().unwrap_or("Unknown")));
                        }

                        if deps_array.len() > 5 {
                            content.push_str(&format!("  - ... and {} more\n", deps_array.len() - 5));
                        }
                    }

                    count += 1;
                }

                if deps_obj.len() > 10 {
                    content.push_str(&format!("\n... and {} more files with dependencies\n", deps_obj.len() - 10));
                }
            }
        }
    } else {
        content.push_str("No file structure information available.\n");
    }

    let path = base_dir.join("docs").join("file_structure.md");
    std::fs::write(&path, content).context("Failed to write file structure documentation")?;
    info!("Generated file structure documentation at {}", path.display());
    Ok(())
}

fn generate_api_doc(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# API Documentation\n\n");

    if let Some(api) = unified_output.get("api") {
        content.push_str("## API Endpoints\n\n");
        content.push_str("| Method | Path | Controller | Action | Auth Required |\n");
        content.push_str("|--------|------|------------|--------|--------------|\n");

        if let Some(endpoints) = api.get("endpoints") {
            if let Some(endpoints_obj) = endpoints.as_object() {
                for (_, endpoint) in endpoints_obj {
                    let method = endpoint.get("method").and_then(|v| v.as_str()).unwrap_or("-");
                    let path = endpoint.get("path").and_then(|v| v.as_str()).unwrap_or("-");
                    let controller = endpoint.get("controller").and_then(|v| v.as_str()).unwrap_or("-");
                    let action = endpoint.get("action").and_then(|v| v.as_str()).unwrap_or("-");
                    let auth_required = endpoint.get("authentication_required").and_then(|v| v.as_bool()).unwrap_or(false);

                    content.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                        method, path, controller, action,
                        if auth_required { "Yes" } else { "No" }));
                }
            }
        }

        // Add API patterns section
        content.push_str("\n## API Patterns\n\n");

        if let Some(patterns) = api.get("route_patterns") {
            if let Some(patterns_obj) = patterns.as_object() {
                for (pattern_name, routes) in patterns_obj {
                    content.push_str(&format!("### {}\n\n", pattern_name));

                    if let Some(routes_array) = routes.as_array() {
                        for route in routes_array {
                            content.push_str(&format!("- {}\n", route.as_str().unwrap_or("Unknown")));
                        }
                    }

                    content.push_str("\n");
                }
            }
        }

        // Add authentication requirements section
        content.push_str("## Authentication Requirements\n\n");
        content.push_str("The following routes require authentication:\n\n");

        if let Some(auth_routes) = api.get("auth_protected_routes") {
            if let Some(auth_routes_array) = auth_routes.as_array() {
                for route in auth_routes_array {
                    content.push_str(&format!("- {}\n", route.as_str().unwrap_or("Unknown")));
                }
            }
        }
    } else {
        content.push_str("No API information available.\n");
    }

    let path = base_dir.join("docs").join("api").join("overview.md");
    std::fs::write(&path, content).context("Failed to write API documentation")?;
    info!("Generated API documentation at {}", path.display());
    Ok(())
}

fn generate_database_schema_doc(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Database Schema Documentation\n\n");

    if let Some(database) = unified_output.get("database_schema") {
        // Tables section
        content.push_str("## Tables\n\n");

        if let Some(tables) = database.get("tables") {
            if let Some(tables_obj) = tables.as_object() {
                for (table_name, table_info) in tables_obj {
                    content.push_str(&format!("### {}\n\n", table_name));

                    // Table columns
                    content.push_str("| Column | Type | Nullable | Default | Description |\n");
                    content.push_str("|--------|------|----------|---------|-------------|\n");

                    if let Some(columns) = table_info.get("columns") {
                        if let Some(columns_array) = columns.as_array() {
                            for column in columns_array {
                                let name = column.get("name").and_then(|v| v.as_str()).unwrap_or("-");
                                let col_type = column.get("column_type").and_then(|v| v.as_str()).unwrap_or("-");
                                let nullable = column.get("nullable").and_then(|v| v.as_bool()).unwrap_or(true);
                                let default = column.get("default").and_then(|v| v.as_str()).unwrap_or("-");

                                content.push_str(&format!("| {} | {} | {} | {} | - |\n",
                                    name, col_type,
                                    if nullable { "Yes" } else { "No" },
                                    default));
                            }
                        }
                    }

                    // Primary key
                    if let Some(primary_key) = table_info.get("primary_key") {
                        content.push_str("\n**Primary Key**: ");
                        content.push_str(primary_key.as_str().unwrap_or("Unknown"));
                        content.push_str("\n");
                    }

                    // Indexes
                    if let Some(indexes) = table_info.get("indexes") {
                        if let Some(indexes_array) = indexes.as_array() {
                            if !indexes_array.is_empty() {
                                content.push_str("\n**Indexes**:\n\n");

                                for index in indexes_array {
                                    let name = index.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                                    let unique = index.get("unique").and_then(|v| v.as_bool()).unwrap_or(false);
                                    let empty_vec = Vec::new();
                                    let columns = index.get("columns").and_then(|v| v.as_array()).unwrap_or(&empty_vec);

                                    content.push_str(&format!("- {} ({}): ",
                                        name,
                                        if unique { "UNIQUE" } else { "INDEX" }));

                                    let column_names: Vec<String> = columns.iter()
                                        .filter_map(|c| c.as_str().map(|s| s.to_string()))
                                        .collect();

                                    content.push_str(&column_names.join(", "));
                                    content.push_str("\n");
                                }
                            }
                        }
                    }

                    // Foreign keys
                    if let Some(foreign_keys) = table_info.get("foreign_keys") {
                        if let Some(fks_array) = foreign_keys.as_array() {
                            if !fks_array.is_empty() {
                                content.push_str("\n**Foreign Keys**:\n\n");

                                for fk in fks_array {
                                    let from_column = fk.get("from_column").and_then(|v| v.as_str()).unwrap_or("Unknown");
                                    let to_table = fk.get("to_table").and_then(|v| v.as_str()).unwrap_or("Unknown");
                                    let to_column = fk.get("to_column").and_then(|v| v.as_str()).unwrap_or("Unknown");

                                    content.push_str(&format!("- {} → {}.{}\n",
                                        from_column, to_table, to_column));
                                }
                            }
                        }
                    }

                    content.push_str("\n");
                }
            }
        }

        // Migrations section
        content.push_str("## Migrations\n\n");
        content.push_str("| Version | Name | Operations |\n");
        content.push_str("|---------|------|------------|\n");

        if let Some(migrations) = database.get("migrations") {
            if let Some(migrations_array) = migrations.as_array() {
                for migration in migrations_array {
                    let version = migration.get("version").and_then(|v| v.as_str()).unwrap_or("-");
                    let name = migration.get("name").and_then(|v| v.as_str()).unwrap_or("-");

                    let operations = if let Some(ops_array) = migration.get("operations").and_then(|v| v.as_array()) {
                        let ops: Vec<String> = ops_array.iter()
                            .filter_map(|op| op.as_str().map(|s| s.to_string()))
                            .collect();

                        if ops.len() <= 3 {
                            ops.join(", ")
                        } else {
                            format!("{}, ... ({} more)",
                                ops[0..2].join(", "),
                                ops.len() - 2)
                        }
                    } else {
                        "-".to_string()
                    };

                    content.push_str(&format!("| {} | {} | {} |\n", version, name, operations));
                }
            }
        }
    } else {
        content.push_str("No database schema information available.\n");
    }

    let path = base_dir.join("docs").join("models").join("database_schema.md");
    std::fs::write(&path, content).context("Failed to write database schema documentation")?;
    info!("Generated database schema documentation at {}", path.display());
    Ok(())
}

fn generate_business_logic_doc(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Business Logic Documentation\n\n");

    if let Some(business_logic) = unified_output.get("business_logic") {
        // Business Logic Patterns section
        content.push_str("## Core Business Logic Patterns\n\n");

        if let Some(patterns) = business_logic.get("patterns") {
            if let Some(patterns_array) = patterns.as_array() {
                for pattern in patterns_array {
                    let name = pattern.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Pattern");
                    let description = pattern.get("description").and_then(|v| v.as_str()).unwrap_or("No description available");

                    content.push_str(&format!("### {}\n\n", name));
                    content.push_str(&format!("{} \n\n", description));

                    // List files implementing this pattern
                    content.push_str("**Implementation Files**:\n\n");

                    if let Some(files) = pattern.get("files").and_then(|v| v.as_array()) {
                        for file in files {
                            content.push_str(&format!("- {}\n", file.as_str().unwrap_or("Unknown")));
                        }
                    }

                    // Add code snippets if available
                    if let Some(snippets) = pattern.get("code_snippets").and_then(|v| v.as_array()) {
                        if !snippets.is_empty() {
                            content.push_str("\n**Example Code**:\n\n");

                            // Just show the first snippet to keep the doc manageable
                            if let Some(snippet) = snippets.get(0).and_then(|v| v.as_str()) {
                                content.push_str("```\n");
                                content.push_str(snippet);
                                content.push_str("\n```\n\n");
                            }
                        }
                    }
                }
            }
        }

        // Domain Algorithms section
        content.push_str("## Domain-Specific Algorithms\n\n");

        if let Some(algorithms) = business_logic.get("algorithms") {
            if let Some(algorithms_array) = algorithms.as_array() {
                for algorithm in algorithms_array {
                    let name = algorithm.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Algorithm");
                    let description = algorithm.get("description").and_then(|v| v.as_str()).unwrap_or("No description available");
                    let complexity = algorithm.get("complexity").and_then(|v| v.as_str()).unwrap_or("Unknown");

                    content.push_str(&format!("### {} (Complexity: {})\n\n", name, complexity));
                    content.push_str(&format!("{} \n\n", description));

                    // List files implementing this algorithm
                    if let Some(files) = algorithm.get("files").and_then(|v| v.as_array()) {
                        content.push_str("**Implementation Files**:\n\n");
                        for file in files {
                            content.push_str(&format!("- {}\n", file.as_str().unwrap_or("Unknown")));
                        }
                        content.push_str("\n");
                    }
                }
            }
        }

        // Workflows section
        content.push_str("## Critical Workflows\n\n");

        if let Some(workflows) = business_logic.get("workflows") {
            if let Some(workflows_array) = workflows.as_array() {
                for workflow in workflows_array {
                    let name = workflow.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Workflow");
                    let description = workflow.get("description").and_then(|v| v.as_str()).unwrap_or("No description available");

                    content.push_str(&format!("### {}\n\n", name));
                    content.push_str(&format!("{} \n\n", description));

                    // List workflow steps
                    if let Some(steps) = workflow.get("steps").and_then(|v| v.as_array()) {
                        content.push_str("**Workflow Steps**:\n\n");

                        for (i, step) in steps.iter().enumerate() {
                            let step_name = step.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Step");
                            let step_desc = step.get("description").and_then(|v| v.as_str()).unwrap_or("No description");

                            content.push_str(&format!("{:2}. **{}**: {}\n", i+1, step_name, step_desc));
                        }
                        content.push_str("\n");
                    }

                    // List files implementing this workflow
                    if let Some(files) = workflow.get("files").and_then(|v| v.as_array()) {
                        content.push_str("**Implementation Files**:\n\n");
                        for file in files {
                            content.push_str(&format!("- {}\n", file.as_str().unwrap_or("Unknown")));
                        }
                        content.push_str("\n");
                    }
                }
            }
        }

        // Edge Cases section
        content.push_str("## Edge Cases and Error Handling\n\n");

        if let Some(edge_cases) = business_logic.get("edge_cases") {
            if let Some(edge_cases_array) = edge_cases.as_array() {
                content.push_str("| Scenario | Handling | Files |\n");
                content.push_str("|----------|----------|-------|\n");

                for edge_case in edge_cases_array {
                    let scenario = edge_case.get("scenario").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let handling = edge_case.get("handling").and_then(|v| v.as_str()).unwrap_or("Unknown");

                    let files = if let Some(files_array) = edge_case.get("files").and_then(|v| v.as_array()) {
                        let file_list: Vec<String> = files_array.iter()
                            .filter_map(|f| f.as_str().map(|s| s.to_string()))
                            .collect();

                        if file_list.len() <= 2 {
                            file_list.join(", ")
                        } else {
                            format!("{}, ... ({} more)", file_list[0], file_list.len() - 1)
                        }
                    } else {
                        "-".to_string()
                    };

                    // Truncate handling text if too long
                    let handling_display = if handling.len() > 50 {
                        format!("{} ...", &handling[0..47])
                    } else {
                        handling.to_string()
                    };

                    content.push_str(&format!("| {} | {} | {} |\n", scenario, handling_display, files));
                }
            }
        }

        // Business Rules section
        content.push_str("\n## Business Rules and Constraints\n\n");

        if let Some(rules) = business_logic.get("business_rules") {
            if let Some(rules_array) = rules.as_array() {
                for rule in rules_array {
                    let name = rule.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown Rule");
                    let description = rule.get("description").and_then(|v| v.as_str()).unwrap_or("No description available");

                    content.push_str(&format!("- **{}**: {}\n", name, description));
                }
            }
        }
    } else {
        content.push_str("No business logic information available.\n");
    }

    let path = base_dir.join("docs").join("technical").join("business_logic.md");
    std::fs::write(&path, content).context("Failed to write business logic documentation")?;
    info!("Generated business logic documentation at {}", path.display());
    Ok(())
}

fn generate_offline_readiness_doc(unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Offline-First Readiness Report\n\n");

    if let Some(offline) = unified_output.get("offline_first_readiness") {
        // Overall readiness score
        if let Some(score) = offline.get("offline_readiness_score").and_then(|v| v.as_u64()) {
            content.push_str(&format!("## Overall Readiness Score: {}%\n\n", score));

            // Add a visual indicator of the score
            let score_category = if score >= 80 {
                "High readiness"
            } else if score >= 50 {
                "Moderate readiness"
            } else {
                "Low readiness"
            };

            content.push_str(&format!("**{}**\n\n", score_category));
        }

        // Data Access Patterns section
        content.push_str("## Data Access Patterns\n\n");
        content.push_str("| Pattern | Description | Sync Feasibility |\n");
        content.push_str("|---------|-------------|-----------------|\n");

        if let Some(patterns) = offline.get("data_access_patterns") {
            if let Some(patterns_array) = patterns.as_array() {
                for pattern in patterns_array {
                    let pattern_type = pattern.get("pattern_type").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let description = pattern.get("description").and_then(|v| v.as_str()).unwrap_or("No description");
                    let feasibility = pattern.get("sync_feasibility").and_then(|v| v.as_str()).unwrap_or("Unknown");

                    content.push_str(&format!("| {} | {} | {} |\n", pattern_type, description, feasibility));
                }
            }
        }

        // Data Update Patterns section
        content.push_str("\n## Data Update Patterns\n\n");

        if let Some(update_patterns) = offline.get("data_update_patterns") {
            if let Some(patterns_array) = update_patterns.as_array() {
                for pattern in patterns_array {
                    let pattern_type = pattern.get("pattern_type").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let description = pattern.get("description").and_then(|v| v.as_str()).unwrap_or("No description");

                    content.push_str(&format!("- **{}**: {}\n", pattern_type, description));
                }
            }
        }

        // Conflict Resolution Strategies section
        content.push_str("\n## Conflict Resolution Strategies\n\n");

        if let Some(strategies) = offline.get("conflict_resolution_strategies") {
            if let Some(strategies_array) = strategies.as_array() {
                for strategy in strategies_array {
                    let name = strategy.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let description = strategy.get("description").and_then(|v| v.as_str()).unwrap_or("No description");

                    content.push_str(&format!("### {}\n\n", name));
                    content.push_str(&format!("{} \n\n", description));

                    // List files implementing this strategy
                    if let Some(files) = strategy.get("files").and_then(|v| v.as_array()) {
                        content.push_str("**Implementation Files**:\n\n");
                        for file in files {
                            content.push_str(&format!("- {}\n", file.as_str().unwrap_or("Unknown")));
                        }
                        content.push_str("\n");
                    }
                }
            }
        }

        // Real-time Update Requirements section
        content.push_str("## Real-time Update Requirements\n\n");
        content.push_str("The following features require real-time updates, which may present challenges for offline-first implementation:\n\n");
        content.push_str("| Feature | Description | Criticality |\n");
        content.push_str("|---------|-------------|------------|\n");

        if let Some(requirements) = offline.get("real_time_update_requirements") {
            if let Some(requirements_array) = requirements.as_array() {
                for requirement in requirements_array {
                    let feature = requirement.get("feature").and_then(|v| v.as_str()).unwrap_or("Unknown");
                    let description = requirement.get("description").and_then(|v| v.as_str()).unwrap_or("No description");
                    let criticality = requirement.get("criticality").and_then(|v| v.as_str()).unwrap_or("Unknown");

                    content.push_str(&format!("| {} | {} | {} |\n", feature, description, criticality));
                }
            }
        }

        // Recommendations section
        content.push_str("\n## Recommendations\n\n");

        if let Some(recommendations) = offline.get("recommendations") {
            if let Some(recommendations_array) = recommendations.as_array() {
                for (i, recommendation) in recommendations_array.iter().enumerate() {
                    if let Some(rec_str) = recommendation.as_str() {
                        content.push_str(&format!("{:2}. {}\n", i+1, rec_str));
                    }
                }
            }
        }
    } else {
        content.push_str("No offline-first readiness information available.\n");
    }

    let path = base_dir.join("docs").join("technical").join("offline_readiness.md");
    std::fs::write(&path, content).context("Failed to write offline readiness documentation")?;
    info!("Generated offline readiness documentation at {}", path.display());
    Ok(())
}

fn generate_architecture_overview(_unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Architecture Overview\n\n");

    content.push_str("This document provides a high-level overview of the Canvas and Discourse architectures ");
    content.push_str("and proposes an architecture for the Rust/Tauri/Leptos implementation.\n\n");

    // Canvas Architecture
    content.push_str("## Canvas LMS Architecture\n\n");
    content.push_str("Canvas LMS is built using:\n\n");
    content.push_str("- **Backend**: Ruby on Rails\n");
    content.push_str("- **Frontend**: React.js\n");
    content.push_str("- **Database**: PostgreSQL\n");
    content.push_str("- **Caching**: Redis\n");
    content.push_str("- **Background Jobs**: Delayed Job\n\n");

    content.push_str("### Key Components\n\n");
    content.push_str("1. **Models**: ActiveRecord models representing the domain entities\n");
    content.push_str("2. **Controllers**: Rails controllers handling HTTP requests\n");
    content.push_str("3. **API**: RESTful API endpoints for frontend communication\n");
    content.push_str("4. **React Components**: UI components for the frontend\n");
    content.push_str("5. **Background Jobs**: Asynchronous processing for tasks like notifications and grading\n\n");

    // Discourse Architecture
    content.push_str("## Discourse Architecture\n\n");
    content.push_str("Discourse is built using:\n\n");
    content.push_str("- **Backend**: Ruby on Rails\n");
    content.push_str("- **Frontend**: Ember.js\n");
    content.push_str("- **Database**: PostgreSQL\n");
    content.push_str("- **Caching**: Redis\n");
    content.push_str("- **Background Jobs**: Sidekiq\n\n");

    content.push_str("### Key Components\n\n");
    content.push_str("1. **Models**: ActiveRecord models for forum entities (posts, topics, users, etc.)\n");
    content.push_str("2. **Controllers**: Rails controllers for handling requests\n");
    content.push_str("3. **API**: JSON API for frontend communication\n");
    content.push_str("4. **Ember Components**: UI components and templates\n");
    content.push_str("5. **Plugins**: Modular extension system\n\n");

    // Proposed Rust/Tauri/Leptos Architecture
    content.push_str("## Proposed Rust/Tauri/Leptos Architecture\n\n");
    content.push_str("The proposed architecture for the offline-first implementation uses:\n\n");
    content.push_str("- **Backend**: Rust with Axum or Actix Web\n");
    content.push_str("- **Frontend**: Leptos (Rust-based reactive framework)\n");
    content.push_str("- **Desktop Shell**: Tauri (Rust-based desktop framework)\n");
    content.push_str("- **Database**: SQLite for local storage, PostgreSQL for server sync\n");
    content.push_str("- **Sync Engine**: Custom Rust-based sync engine\n\n");

    content.push_str("### Key Components\n\n");
    content.push_str("1. **Domain Models**: Rust structs representing the domain entities\n");
    content.push_str("2. **API Handlers**: Rust functions handling API requests\n");
    content.push_str("3. **Database Layer**: Diesel ORM for database interactions\n");
    content.push_str("4. **Leptos Components**: Reactive UI components written in Rust\n");
    content.push_str("5. **Sync Engine**: Handles data synchronization between local and remote databases\n");
    content.push_str("6. **Offline Queue**: Manages operations performed while offline\n\n");

    content.push_str("### Architecture Diagram\n\n");
    content.push_str("```\n");
    content.push_str("+----------------------------------+\n");
    content.push_str("|            Tauri Shell           |\n");
    content.push_str("+----------------------------------+\n");
    content.push_str("|                                  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |      Leptos Frontend       |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |  |   UI Components     |  |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|                                  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |      Rust Backend         |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |  |   Domain Logic      |  |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |  |   Sync Engine       |  |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |  |   SQLite (Local)    |  |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|                                  |\n");
    content.push_str("+----------------------------------+\n");
    content.push_str("              |   |\n");
    content.push_str("              |   |  (Sync when online)\n");
    content.push_str("              v   v\n");
    content.push_str("+----------------------------------+\n");
    content.push_str("|         Remote Server            |\n");
    content.push_str("|                                  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |      API Endpoints         |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |  |   PostgreSQL        |  |  |\n");
    content.push_str("|  |  +----------------------+  |  |\n");
    content.push_str("|  |                            |  |\n");
    content.push_str("|  +----------------------------+  |\n");
    content.push_str("|                                  |\n");
    content.push_str("+----------------------------------+\n");
    content.push_str("```\n");

    let path = base_dir.join("docs").join("architecture").join("overview.md");
    std::fs::write(&path, content).context("Failed to write architecture overview documentation")?;
    info!("Generated architecture overview at {}", path.display());
    Ok(())
}

fn generate_implementation_roadmap(_unified_output: &Value, base_dir: &PathBuf) -> Result<()> {
    let mut content = String::from("# Implementation Roadmap\n\n");

    content.push_str("This document outlines the proposed roadmap for implementing the offline-first ");
    content.push_str("Rust/Tauri/Leptos version of Canvas and Discourse functionality.\n\n");

    // Phase 1: Foundation
    content.push_str("## Phase 1: Foundation (Months 1-3)\n\n");
    content.push_str("### Goals\n\n");
    content.push_str("- Set up the basic project architecture\n");
    content.push_str("- Implement core data models\n");
    content.push_str("- Create the local database schema\n");
    content.push_str("- Build the basic UI shell\n\n");

    content.push_str("### Tasks\n\n");
    content.push_str("1. Set up Tauri project with Leptos integration\n");
    content.push_str("2. Define core domain models in Rust\n");
    content.push_str("3. Set up SQLite database with Diesel ORM\n");
    content.push_str("4. Implement basic UI navigation shell\n");
    content.push_str("5. Create authentication system\n");
    content.push_str("6. Implement basic user profile functionality\n\n");

    // Phase 2: Core Functionality
    content.push_str("## Phase 2: Core Functionality (Months 4-6)\n\n");
    content.push_str("### Goals\n\n");
    content.push_str("- Implement core LMS features\n");
    content.push_str("- Build basic forum functionality\n");
    content.push_str("- Create the sync engine foundation\n\n");

    content.push_str("### Tasks\n\n");
    content.push_str("1. Implement course creation and management\n");
    content.push_str("2. Build assignment submission system\n");
    content.push_str("3. Create basic forum with topics and posts\n");
    content.push_str("4. Implement file upload/download with local storage\n");
    content.push_str("5. Build the basic sync engine for data synchronization\n");
    content.push_str("6. Implement offline queue for operations while disconnected\n\n");

    // Phase 3: Advanced Features
    content.push_str("## Phase 3: Advanced Features (Months 7-9)\n\n");
    content.push_str("### Goals\n\n");
    content.push_str("- Implement advanced LMS features\n");
    content.push_str("- Enhance forum capabilities\n");
    content.push_str("- Improve sync engine with conflict resolution\n\n");

    content.push_str("### Tasks\n\n");
    content.push_str("1. Implement grading and feedback system\n");
    content.push_str("2. Build quiz and assessment functionality\n");
    content.push_str("3. Add forum moderation tools\n");
    content.push_str("4. Implement advanced forum features (categories, tags, etc.)\n");
    content.push_str("5. Enhance sync engine with conflict resolution strategies\n");
    content.push_str("6. Implement data compression for efficient sync\n\n");

    // Phase 4: Polish and Optimization
    content.push_str("## Phase 4: Polish and Optimization (Months 10-12)\n\n");
    content.push_str("### Goals\n\n");
    content.push_str("- Optimize performance\n");
    content.push_str("- Enhance user experience\n");
    content.push_str("- Prepare for production release\n\n");

    content.push_str("### Tasks\n\n");
    content.push_str("1. Performance optimization for large datasets\n");
    content.push_str("2. Implement advanced caching strategies\n");
    content.push_str("3. Add comprehensive error handling and recovery\n");
    content.push_str("4. Create comprehensive test suite\n");
    content.push_str("5. Implement analytics and telemetry (opt-in)\n");
    content.push_str("6. Prepare deployment and distribution pipeline\n\n");

    // Implementation Priorities
    content.push_str("## Implementation Priorities\n\n");
    content.push_str("Based on the analysis of the Canvas and Discourse codebases, the following features ");
    content.push_str("should be prioritized for implementation:\n\n");

    content.push_str("### High Priority\n\n");
    content.push_str("1. User authentication and profiles\n");
    content.push_str("2. Course management\n");
    content.push_str("3. Basic forum functionality\n");
    content.push_str("4. Offline data synchronization\n");
    content.push_str("5. File management with local storage\n\n");

    content.push_str("### Medium Priority\n\n");
    content.push_str("1. Assignment submission and grading\n");
    content.push_str("2. Advanced forum features\n");
    content.push_str("3. Notifications system\n");
    content.push_str("4. Search functionality\n");
    content.push_str("5. Calendar and scheduling\n\n");

    content.push_str("### Lower Priority\n\n");
    content.push_str("1. Analytics and reporting\n");
    content.push_str("2. Integration with external tools\n");
    content.push_str("3. Advanced quiz features\n");
    content.push_str("4. Video conferencing\n");
    content.push_str("5. Mobile-specific optimizations\n\n");

    content.push_str("## Technical Challenges\n\n");
    content.push_str("The following technical challenges have been identified and will need special attention:\n\n");

    content.push_str("1. **Conflict Resolution**: Handling conflicts when syncing data modified both locally and remotely\n");
    content.push_str("2. **Large File Handling**: Efficiently managing large files in an offline-first context\n");
    content.push_str("3. **Real-time Collaboration**: Implementing collaborative features that work offline\n");
    content.push_str("4. **Performance**: Ensuring good performance with potentially large local databases\n");
    content.push_str("5. **Security**: Maintaining proper security in a distributed system\n");

    let path = base_dir.join("docs").join("integration").join("roadmap.md");
    std::fs::write(&path, content).context("Failed to write implementation roadmap documentation")?;
    info!("Generated implementation roadmap at {}", path.display());
    Ok(())
}
