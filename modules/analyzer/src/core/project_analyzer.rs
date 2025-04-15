use std::collections::HashMap;
use std::error::Error;
use std::fs::{self};
use std::path::{Path, PathBuf};
use chrono::Local;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};

use crate::core::analysis_result::{AnalysisResult, Model, ModelField, ModelRelationship, UiComponent, ApiEndpoint};
use crate::core::analyzer_config::AnalyzerConfig;

/// Project analysis result structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectAnalysis {
    pub timestamp: String,
    pub summary: ProjectSummary,
    pub components: Vec<Component>,
    pub models: Vec<Model>,
    pub routes: Vec<Route>,
    pub integrations: Vec<Integration>,
    pub tech_stack: TechStack,
    pub architecture: ArchitectureInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSummary {
    pub total_files: usize,
    pub lines_of_code: usize,
    pub file_types: HashMap<String, usize>,
    pub rust_files: usize,
    pub haskell_files: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Component {
    pub name: String,
    pub file_path: String,
    pub dependencies: Vec<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub file_path: String,
    pub fields: Vec<String>,
    pub associations: Vec<String>,
    pub source_system: String, // "Canvas", "Discourse", or "Native"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Route {
    pub path: String,
    pub component: String,
    pub methods: Vec<String>,
    pub auth_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Integration {
    pub name: String,
    pub source_system: String,
    pub target_system: String,
    pub integration_points: Vec<String>,
    pub status: String, // "Planned", "In Progress", "Completed"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechStack {
    pub frontend: Vec<String>,
    pub backend: Vec<String>,
    pub database: Vec<String>,
    pub search: Vec<String>,
    pub ai: Vec<String>,
    pub blockchain: Vec<String>,
    pub authentication: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArchitectureInfo {
    pub patterns: Vec<String>,
    pub principles: Vec<String>,
    pub diagrams: Vec<String>,
}

/// Project analyzer for the LMS project
pub struct ProjectAnalyzer {
    /// Configuration for the analyzer
    config: AnalyzerConfig,

    /// Base directory for analysis
    base_dir: PathBuf,
}

impl ProjectAnalyzer {
    /// Create a new project analyzer
    pub fn new(config: AnalyzerConfig) -> Self {
        Self {
            base_dir: config.base_dir.clone(),
            config,
        }
    }

    /// Run the project analysis
    pub fn analyze(&self) -> Result<ProjectAnalysis, Box<dyn Error>> {
        println!("Starting LMS Project Analysis...");

        let now = Local::now();

        // Initialize analysis structures
        let mut summary = ProjectSummary {
            total_files: 0,
            lines_of_code: 0,
            file_types: HashMap::new(),
            rust_files: 0,
            haskell_files: 0,
        };

        let mut components = Vec::new();
        let mut models = Vec::new();
        let mut routes = Vec::new();

        // Walk through the project directory
        for entry in WalkDir::new(&self.base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip node_modules, target directories, and other irrelevant paths
            if self.should_skip_path(path) {
                continue;
            }

            // Count files by type
            summary.total_files += 1;

            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                *summary.file_types.entry(ext.to_string()).or_insert(0) += 1;

                // Count specific file types
                if ext == "rs" {
                    summary.rust_files += 1;
                    self.analyze_rust_file(path, &mut components, &mut models, &mut routes)?;
                } else if ext == "hs" {
                    summary.haskell_files += 1;
                    self.analyze_haskell_file(path, &mut components, &mut models)?;
                }

                // Count lines of code
                if let Ok(content) = fs::read_to_string(path) {
                    summary.lines_of_code += content.lines().count();
                }
            }
        }

        // Define known integrations
        let integrations = vec![
            Integration {
                name: "Canvas Course Management".to_string(),
                source_system: "Canvas".to_string(),
                target_system: "LMS".to_string(),
                integration_points: vec![
                    "Course creation".to_string(),
                    "Assignment management".to_string(),
                    "Grading system".to_string(),
                ],
                status: "In Progress".to_string(),
            },
            Integration {
                name: "Discourse Forums".to_string(),
                source_system: "Discourse".to_string(),
                target_system: "LMS".to_string(),
                integration_points: vec![
                    "Discussion threads".to_string(),
                    "User profiles".to_string(),
                    "Notifications".to_string(),
                ],
                status: "Planned".to_string(),
            },
            Integration {
                name: "Blockchain Certification".to_string(),
                source_system: "Native".to_string(),
                target_system: "LMS".to_string(),
                integration_points: vec![
                    "Certificate issuance".to_string(),
                    "Achievement verification".to_string(),
                    "Credential storage".to_string(),
                ],
                status: "In Progress".to_string(),
            },
        ];

        // Define tech stack
        let tech_stack = TechStack {
            frontend: vec!["Leptos".to_string(), "Tauri".to_string()],
            backend: vec!["Rust".to_string(), "Haskell".to_string()],
            database: vec!["SQLite".to_string(), "sqlx".to_string()],
            search: vec!["MeiliSearch".to_string()],
            ai: vec!["Gemini".to_string()],
            blockchain: vec!["Custom Rust implementation".to_string()],
            authentication: vec!["JWT".to_string()],
        };

        // Define architecture info
        let architecture = ArchitectureInfo {
            patterns: vec![
                "CQRS".to_string(),
                "Event Sourcing".to_string(),
                "Repository Pattern".to_string(),
            ],
            principles: vec![
                "Clean Architecture".to_string(),
                "SOLID".to_string(),
                "Offline-first".to_string(),
            ],
            diagrams: vec![
                "docs/architecture/high_level.md".to_string(),
                "docs/architecture/data_flow.md".to_string(),
            ],
        };

        Ok(ProjectAnalysis {
            timestamp: now.to_rfc3339(),
            summary,
            components,
            models,
            routes,
            integrations,
            tech_stack,
            architecture,
        })
    }

    /// Helper function to determine if a path should be skipped
    fn should_skip_path(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        // Check against exclude patterns from config
        for pattern in &self.config.exclude_patterns {
            if path_str.contains(pattern) {
                return true;
            }
        }

        // Default patterns to skip
        path_str.contains("node_modules") ||
        path_str.contains("target") ||
        path_str.contains(".git") ||
        path_str.contains("obsolete_backup") ||
        path_str.contains(".vscode")
    }

    /// Analyze Rust files for components, models, and routes
    fn analyze_rust_file(
        &self,
        path: &Path,
        components: &mut Vec<Component>,
        models: &mut Vec<Model>,
        routes: &mut Vec<Route>
    ) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(path)?;

        // Extract component information
        if content.contains("struct") && (content.contains("impl") || content.contains("#[component]")) {
            // Simple regex-like extraction for component name
            if let Some(struct_line) = content.lines().find(|line| line.contains("struct ")) {
                if let Some(name) = struct_line.split("struct ").nth(1).and_then(|s| s.split('{').next()) {
                    let name = name.trim().to_string();

                    // Extract dependencies (very simplified)
                    let mut dependencies = Vec::new();
                    for line in content.lines() {
                        if line.contains("use ") {
                            if let Some(dep) = line.split("use ").nth(1) {
                                dependencies.push(dep.trim().trim_end_matches(';').to_string());
                            }
                        }
                    }

                    components.push(Component {
                        name,
                        file_path: path.to_string_lossy().to_string(),
                        dependencies,
                        description: "Auto-detected component".to_string(),
                    });
                }
            }
        }

        // Extract model information
        if content.contains("#[derive(") && content.contains("struct") &&
           (content.contains("Serialize") || content.contains("Deserialize") || content.contains("sqlx::FromRow")) {
            // Simple extraction for model name
            if let Some(struct_line) = content.lines().find(|line| line.contains("struct ")) {
                if let Some(name) = struct_line.split("struct ").nth(1).and_then(|s| s.split('{').next()) {
                    let name = name.trim().to_string();

                    // Extract fields (simplified)
                    let mut fields = Vec::new();
                    let mut in_struct = false;
                    for line in content.lines() {
                        if line.contains("struct ") && line.contains(&name) {
                            in_struct = true;
                            continue;
                        }

                        if in_struct {
                            if line.contains('}') {
                                in_struct = false;
                                break;
                            }

                            if line.contains(':') && !line.trim().starts_with("//") {
                                if let Some(field) = line.split(':').next() {
                                    fields.push(field.trim().to_string());
                                }
                            }
                        }
                    }

                    // Determine source system based on file path or content
                    let source_system = if path.to_string_lossy().contains("canvas") {
                        "Canvas".to_string()
                    } else if path.to_string_lossy().contains("discourse") {
                        "Discourse".to_string()
                    } else {
                        "Native".to_string()
                    };

                    models.push(Model {
                        name,
                        file_path: path.to_string_lossy().to_string(),
                        fields,
                        associations: Vec::new(), // Would need more complex parsing
                        source_system,
                    });
                }
            }
        }

        // Extract route information
        if content.contains("Route::new") || content.contains("route!") {
            // Simple extraction for routes
            for line in content.lines() {
                if line.contains("Route::new") || line.contains("route!") {
                    // Very simplified extraction
                    if let Some(path_part) = line.split('"').nth(1) {
                        routes.push(Route {
                            path: path_part.to_string(),
                            component: "Unknown".to_string(), // Would need more complex parsing
                            methods: vec!["GET".to_string()], // Default assumption
                            auth_required: line.contains("auth") || line.contains("protected"),
                        });
                    }
                }
            }
        }

        Ok(())
    }

    /// Analyze Haskell files for components and models
    fn analyze_haskell_file(
        &self,
        path: &Path,
        components: &mut Vec<Component>,
        models: &mut Vec<Model>
    ) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(path)?;

        // Extract data types (models)
        if content.contains("data ") && content.contains("=") {
            for line in content.lines() {
                if line.trim().starts_with("data ") && line.contains("=") {
                    if let Some(name) = line.split("data ").nth(1).and_then(|s| s.split('=').next()) {
                        let name = name.trim().to_string();

                        // Extract fields (very simplified)
                        let mut fields = Vec::new();
                        let mut in_data_type = false;
                        for data_line in content.lines() {
                            if data_line.contains(&format!("data {}", name)) {
                                in_data_type = true;
                                continue;
                            }

                            if in_data_type {
                                if data_line.trim().starts_with("}") || data_line.trim().starts_with("deriving") {
                                    in_data_type = false;
                                    break;
                                }

                                if data_line.contains("::") && !data_line.trim().starts_with("--") {
                                    if let Some(field) = data_line.split("::").next() {
                                        fields.push(field.trim().to_string());
                                    }
                                }
                            }
                        }

                        models.push(Model {
                            name,
                            file_path: path.to_string_lossy().to_string(),
                            fields,
                            associations: Vec::new(),
                            source_system: "Haskell".to_string(),
                        });
                    }
                }
            }
        }

        // Extract components (functions)
        for line in content.lines() {
            if line.contains("::") && !line.trim().starts_with("--") && !line.contains("data ") {
                if let Some(name) = line.split("::").next() {
                    let name = name.trim().to_string();

                    // Skip if it's not a meaningful name
                    if name.is_empty() || name.starts_with("(") {
                        continue;
                    }

                    components.push(Component {
                        name,
                        file_path: path.to_string_lossy().to_string(),
                        dependencies: Vec::new(),
                        description: "Haskell function/component".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Convert ProjectAnalysis to AnalysisResult
    pub fn to_analysis_result(&self, project_analysis: &ProjectAnalysis) -> AnalysisResult {
        let mut result = AnalysisResult::default();

        // Set timestamp
        result.timestamp = Local::now();

        // Set project summary
        result.summary.total_files = project_analysis.summary.total_files;
        result.summary.lines_of_code = project_analysis.summary.lines_of_code;
        result.summary.file_types = project_analysis.summary.file_types.clone();
        result.summary.rust_files = project_analysis.summary.rust_files;
        result.summary.haskell_files = project_analysis.summary.haskell_files;

        // Set models
        result.models.total = project_analysis.models.len();
        result.models.implemented = project_analysis.models.len();
        if project_analysis.models.len() > 0 {
            result.models.implementation_percentage = 100.0;
        }

        // Convert models to the AnalysisResult format
        for model in &project_analysis.models {
            // Create model fields
            let mut fields = Vec::new();
            for field in &model.fields {
                fields.push(ModelField {
                    name: field.clone(),
                    field_type: "unknown".to_string(),
                    required: false,
                    description: None,
                });
            }

            // Create model relationships
            let mut relationships = Vec::new();
            for assoc in &model.associations {
                relationships.push(ModelRelationship {
                    relationship_type: "association".to_string(),
                    related_model: assoc.clone(),
                    description: None,
                });
            }

            // Create and push the model
            result.models.models.push(Model {
                name: model.name.clone(),
                file_path: model.file_path.clone(),
                source_system: model.source_system.clone(),
                implemented: true,
                fields,
                relationships,
            });
        }

        // Set components
        result.ui_components.total = project_analysis.components.len();
        result.ui_components.implemented = project_analysis.components.len();
        if project_analysis.components.len() > 0 {
            result.ui_components.implementation_percentage = 100.0;
        }

        // Convert components to the AnalysisResult format
        for component in &project_analysis.components {
            let ui_component = UiComponent {
                name: component.name.clone(),
                file_path: component.file_path.clone(),
                description: component.description.clone(),
                implemented: true,
                category: None,
            };
            result.ui_components.components.push(ui_component);
        }

        // Set API endpoints
        result.api_endpoints.total = project_analysis.routes.len();
        result.api_endpoints.implemented = project_analysis.routes.len();
        if project_analysis.routes.len() > 0 {
            result.api_endpoints.implementation_percentage = 100.0;
        }

        // Convert routes to the AnalysisResult format
        for route in &project_analysis.routes {
            let method = if route.methods.is_empty() {
                "GET".to_string()
            } else {
                route.methods[0].clone()
            };

            let endpoint = ApiEndpoint {
                path: route.path.clone(),
                method,
                description: format!("Route to {}", route.component),
                implemented: true,
                category: None,
                parameters: Vec::new(),
                response_fields: Vec::new(),
            };

            result.api_endpoints.endpoints.push(endpoint);
        }

        // Set overall progress
        let total_items = result.models.total + result.ui_components.total + result.api_endpoints.total;
        let implemented_items = result.models.implemented + result.ui_components.implemented + result.api_endpoints.implemented;

        if total_items > 0 {
            result.overall_progress = (implemented_items as f32 / total_items as f32) * 100.0;
        }

        // Add some next steps
        result.next_steps.push("Complete integration of Canvas LMS features".to_string());
        result.next_steps.push("Implement Discourse forum integration".to_string());
        result.next_steps.push("Enhance blockchain certification module".to_string());

        // Add some recent changes
        result.recent_changes.push("Integrated project analyzer into unified analyzer".to_string());
        result.recent_changes.push("Added comprehensive project documentation".to_string());
        result.recent_changes.push("Consolidated analyzer modules".to_string());

        result
    }
}
