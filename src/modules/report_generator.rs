use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use chrono::Local;
use serde::{Serialize, Deserialize};
use log::info;

/// Database solution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSolution {
    pub engine: String,
    pub driver: String,
    pub configuration: String,
    pub database_path: String,
    pub migrations: String,
}

impl Default for DatabaseSolution {
    fn default() -> Self {
        Self {
            engine: "SQLite".to_string(),
            driver: "sqlx".to_string(),
            configuration: "Embedded, file-based".to_string(),
            database_path: "./src-tauri/educonnect.db".to_string(),
            migrations: "sqlx built-in migrations".to_string(),
        }
    }
}

/// Module for generating analysis reports
pub struct ReportGenerator<M> {
    metrics: M,
    base_dir: PathBuf,
    database_solution: DatabaseSolution,
}

impl<M> ReportGenerator<M> {
    /// Create a new report generator
    pub fn new(metrics: M, base_dir: PathBuf) -> Self {
        Self {
            metrics,
            base_dir,
            database_solution: DatabaseSolution::default(),
        }
    }
    
    /// Generate central reference hub for project documentation
    pub async fn generate_central_reference_hub<F>(&self, fs_utils: &F) -> Result<PathBuf>
    where
        M: AsRef<ProjectMetrics>,
        F: FileSystemUtils,
    {
        info!("Generating central reference hub...");
        
        let docs_dir = self.base_dir.join("docs");
        fs::create_dir_all(&docs_dir)
            .context(format!("Failed to create docs directory: {:?}", docs_dir))?;
        
        let output_path = docs_dir.join("central_reference_hub.md");
        
        // Create the content
        let mut content = format!("# LMS Project Central Reference Hub\n\n");
        content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        // Project overview section
        content.push_str("## Project Overview\n\n");
        
        let metrics = self.metrics.as_ref();
        let models_percent = self.get_percentage(
            metrics.models.implemented,
            metrics.models.total,
        );
        let api_percent = self.get_percentage(
            metrics.api_endpoints.implemented,
            metrics.api_endpoints.total,
        );
        let ui_percent = self.get_percentage(
            metrics.ui_components.implemented,
            metrics.ui_components.total,
        );
        
        content.push_str("| Component | Completion | Status |\n");
        content.push_str("|-----------|------------|--------|\n");
        content.push_str(&format!(
            "| Models | {}% | {} |\n",
            models_percent,
            self.get_status_emoji(models_percent),
        ));
        content.push_str(&format!(
            "| API Endpoints | {}% | {} |\n",
            api_percent,
            self.get_status_emoji(api_percent),
        ));
        content.push_str(&format!(
            "| UI Components | {}% | {} |\n",
            ui_percent,
            self.get_status_emoji(ui_percent),
        ));
        content.push_str(&format!(
            "| Test Coverage | {}% | {} |\n\n",
            metrics.tests.coverage,
            self.get_status_emoji(metrics.tests.coverage),
        ));
        
        // Technology stack section
        content.push_str("## Technology Stack\n\n");
        content.push_str("### Backend\n\n");
        content.push_str("- **Language**: Rust\n");
        content.push_str("- **Framework**: Actix Web\n");
        content.push_str(&format!("- **Database**: {}\n", self.database_solution.engine));
        content.push_str(&format!("  - **ORM/Driver**: {}\n", self.database_solution.driver));
        content.push_str(&format!("  - **Configuration**: {}\n", self.database_solution.configuration));
        content.push_str(&format!("  - **Migrations**: {}\n", self.database_solution.migrations));
        
        content.push_str("\n### Frontend\n\n");
        content.push_str("- **Framework**: React\n");
        content.push_str("- **State Management**: Redux\n");
        content.push_str("- **UI Library**: Material-UI\n");
        
        // Key documentation section
        content.push_str("\n## Key Documentation\n\n");
        
        // Find all markdown files in the docs directory
        let docs = fs_utils.list_files_with_extension(&docs_dir, "md")?;
        
        for doc_path in docs {
            // Skip the central reference hub itself
            if doc_path.file_name().unwrap_or_default() == "central_reference_hub.md" {
                continue;
            }
            
            let file_name = doc_path.file_name().unwrap_or_default().to_string_lossy();
            let title = self.get_title_from_filename(&file_name);
            let relative_path = pathdiff::diff_paths(&doc_path, &docs_dir)
                .unwrap_or_else(|| doc_path.file_name().unwrap_or_default().into());
                
            content.push_str(&format!(
                "- [{}]({})\n",
                title,
                relative_path.to_string_lossy(),
            ));
        }
        
        // API documentation section
        content.push_str("\n## API Reference\n\n");
        content.push_str("| Endpoint | Method | Description | Status |\n");
        content.push_str("|----------|--------|-------------|--------|\n");
        
        // Only include implemented API endpoints
        for endpoint in metrics.api_endpoints.endpoints.iter().filter(|e| e.implemented) {
            content.push_str(&format!(
                "| {} | {} | {} | ✅ |\n",
                endpoint.path,
                endpoint.method,
                endpoint.description,
            ));
        }
        
        // Integration documents section
        content.push_str("\n## Integration Documents\n\n");
        
        // Write content to file
        fs::write(&output_path, content)
            .context(format!("Failed to write to output file: {:?}", output_path))?;
            
        info!("Central reference hub generated at: {:?}", output_path);
        
        Ok(output_path)
    }
    
    /// Calculate percentage
    fn get_percentage(&self, implemented: usize, total: usize) -> usize {
        if total == 0 {
            return 0;
        }
        (implemented as f64 / total as f64 * 100.0).round() as usize
    }
    
    /// Get status emoji based on percentage
    fn get_status_emoji(&self, percentage: usize) -> &'static str {
        if percentage >= 80 {
            "✅" // Complete or near complete
        } else if percentage >= 50 {
            "🟡" // In progress
        } else if percentage > 0 {
            "🟠" // Started
        } else {
            "❌" // Not started
        }
    }
    
    /// Get a user-friendly title from a filename
    fn get_title_from_filename(&self, filename: &str) -> String {
        // Remove extension
        let name = filename.split('.').next().unwrap_or(filename);
        
        // Replace underscores and hyphens with spaces
        let name = name.replace('_', " ").replace('-', " ");
        
        // Capitalize words
        name.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    
    /// Generate API documentation
    pub async fn generate_api_documentation(&self) -> Result<PathBuf> {
        info!("Generating API documentation...");
        
        let docs_dir = self.base_dir.join("docs");
        fs::create_dir_all(&docs_dir)
            .context(format!("Failed to create docs directory: {:?}", docs_dir))?;
        
        let output_path = docs_dir.join("api_reference.md");
        
        // Create the content
        let mut content = format!("# API Reference\n\n");
        content.push_str(&format!("_Generated on: {}_\n\n", Local::now().format("%Y-%m-%d")));
        
        content.push_str("This document provides a comprehensive reference for all API endpoints in the LMS project.\n\n");
        
        let metrics = self.metrics.as_ref();
        
        // Group endpoints by category
        let mut categories: std::collections::HashMap<String, Vec<&ApiEndpoint>> = std::collections::HashMap::new();
        
        for endpoint in &metrics.api_endpoints.endpoints {
            let category = endpoint.category.clone().unwrap_or_else(|| "Other".to_string());
            categories.entry(category).or_default().push(endpoint);
        }
        
        // Generate documentation for each category
        for (category, endpoints) in categories {
            content.push_str(&format!("## {}\n\n", category));
            
            for endpoint in endpoints {
                content.push_str(&format!("### `{} {}`\n\n", endpoint.method, endpoint.path));
                content.push_str(&format!("**Description**: {}\n\n", endpoint.description));
                
                if let Some(auth) = &endpoint.authentication {
                    content.push_str(&format!("**Authentication**: {}\n\n", auth));
                }
                
                if !endpoint.parameters.is_empty() {
                    content.push_str("**Parameters**:\n\n");
                    content.push_str("| Name | Type | Required | Description |\n");
                    content.push_str("|------|------|----------|-------------|\n");
                    
                    for param in &endpoint.parameters {
                        content.push_str(&format!(
                            "| {} | {} | {} | {} |\n",
                            param.name,
                            param.param_type,
                            if param.required { "Yes" } else { "No" },
                            param.description,
                        ));
                    }
                    
                    content.push_str("\n");
                }
                
                if let Some(resp) = &endpoint.response_example {
                    content.push_str("**Response Example**:\n\n");
                    content.push_str("```json\n");
                    content.push_str(resp);
                    content.push_str("\n```\n\n");
                }
                
                content.push_str("**Status**:\n\n");
                content.push_str(if endpoint.implemented {
                    "✅ Implemented\n\n"
                } else {
                    "❌ Not implemented\n\n"
                });
                
                content.push_str("---\n\n");
            }
        }
        
        // Write content to file
        fs::write(&output_path, content)
            .context(format!("Failed to write to output file: {:?}", output_path))?;
            
        info!("API documentation generated at: {:?}", output_path);
        
        Ok(output_path)
    }
}

/// Project metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    pub models: ModelsMetrics,
    pub api_endpoints: ApiEndpointsMetrics,
    pub ui_components: UiComponentsMetrics,
    pub tests: TestsMetrics,
}

/// Models metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsMetrics {
    pub total: usize,
    pub implemented: usize,
    pub models: Vec<Model>,
}

/// Model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub implemented: bool,
    pub properties: Vec<ModelProperty>,
}

/// Model property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelProperty {
    pub name: String,
    pub property_type: String,
    pub required: bool,
}

/// API endpoints metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointsMetrics {
    pub total: usize,
    pub implemented: usize,
    pub endpoints: Vec<ApiEndpoint>,
}

/// API endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpoint {
    pub path: String,
    pub method: String,
    pub description: String,
    pub implemented: bool,
    pub category: Option<String>,
    pub authentication: Option<String>,
    pub parameters: Vec<ApiParameter>,
    pub response_example: Option<String>,
}

/// API parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiParameter {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

/// UI components metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiComponentsMetrics {
    pub total: usize,
    pub implemented: usize,
}

/// Tests metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestsMetrics {
    pub total: usize,
    pub passing: usize,
    pub coverage: usize,
}

/// FileSystemUtils trait for file operations
pub trait FileSystemUtils {
    /// List all files with a specific extension in a directory
    fn list_files_with_extension(&self, dir: &Path, extension: &str) -> Result<Vec<PathBuf>>;
}
