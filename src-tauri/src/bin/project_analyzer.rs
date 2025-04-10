use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use chrono::Local;

// Define structures for project analysis
#[derive(Debug, Serialize, Deserialize)]
struct ProjectAnalysis {
    timestamp: String,
    summary: ProjectSummary,
    components: Vec<Component>,
    models: Vec<Model>,
    routes: Vec<Route>,
    integrations: Vec<Integration>,
    tech_stack: TechStack,
    architecture: ArchitectureInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectSummary {
    total_files: usize,
    lines_of_code: usize,
    file_types: HashMap<String, usize>,
    rust_files: usize,
    haskell_files: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Component {
    name: String,
    file_path: String,
    dependencies: Vec<String>,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Model {
    name: String,
    file_path: String,
    fields: Vec<String>,
    associations: Vec<String>,
    source_system: String, // "Canvas", "Discourse", or "Native"
}

#[derive(Debug, Serialize, Deserialize)]
struct Route {
    path: String,
    component: String,
    methods: Vec<String>,
    auth_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Integration {
    name: String,
    source_system: String,
    target_system: String,
    integration_points: Vec<String>,
    status: String, // "Planned", "In Progress", "Completed"
}

#[derive(Debug, Serialize, Deserialize)]
struct TechStack {
    frontend: Vec<String>,
    backend: Vec<String>,
    database: Vec<String>,
    search: Vec<String>,
    ai: Vec<String>,
    blockchain: Vec<String>,
    authentication: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArchitectureInfo {
    patterns: Vec<String>,
    principles: Vec<String>,
    diagrams: Vec<String>,
}

// Main function to run the analysis
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting LMS Project Analysis...");
    
    // Run the analysis
    let analysis = analyze_project()?;
    
    // Generate documentation
    generate_central_reference_hub(&analysis)?;
    generate_architecture_doc(&analysis)?;
    generate_models_doc(&analysis)?;
    generate_integration_doc(&analysis)?;
    
    println!("Analysis complete. Documentation updated.");
    Ok(())
}

// Analyze the entire project
fn analyze_project() -> Result<ProjectAnalysis, Box<dyn Error>> {
    let project_root = Path::new(".");
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
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        
        // Skip node_modules, target directories, and other irrelevant paths
        if should_skip_path(path) {
            continue;
        }
        
        // Count files by type
        summary.total_files += 1;
        
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            *summary.file_types.entry(ext.to_string()).or_insert(0) += 1;
            
            // Count specific file types
            if ext == "rs" {
                summary.rust_files += 1;
                analyze_rust_file(path, &mut components, &mut models, &mut routes)?;
            } else if ext == "hs" {
                summary.haskell_files += 1;
                analyze_haskell_file(path, &mut components, &mut models)?;
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

// Helper function to determine if a path should be skipped
fn should_skip_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    
    path_str.contains("node_modules") ||
    path_str.contains("target") ||
    path_str.contains(".git") ||
    path_str.contains("obsolete_backup") ||
    path_str.contains(".vscode")
}

// Analyze Rust files for components, models, and routes
fn analyze_rust_file(
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

// Analyze Haskell files for components and models
fn analyze_haskell_file(
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

// Generate the central reference hub document
fn generate_central_reference_hub(analysis: &ProjectAnalysis) -> Result<(), Box<dyn Error>> {
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)?;
    }
    
    let hub_path = docs_dir.join("central_reference_hub.md");
    let mut file = File::create(hub_path)?;
    
    // Write header
    writeln!(file, "# LMS Project: Central Reference Hub")?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)?;
    
    // Write project overview
    writeln!(file, "## Project Overview")?;
    writeln!(file, "\nThe LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n")?;
    
    // Write tech stack
    writeln!(file, "## Technology Stack")?;
    writeln!(file, "\n### Frontend")?;
    for tech in &analysis.tech_stack.frontend {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### Backend")?;
    for tech in &analysis.tech_stack.backend {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### Database")?;
    for tech in &analysis.tech_stack.database {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### Search")?;
    for tech in &analysis.tech_stack.search {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### AI Integration")?;
    for tech in &analysis.tech_stack.ai {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### Blockchain")?;
    for tech in &analysis.tech_stack.blockchain {
        writeln!(file, "- {}", tech)?;
    }
    
    writeln!(file, "\n### Authentication")?;
    for tech in &analysis.tech_stack.authentication {
        writeln!(file, "- {}", tech)?;
    }
    
    // Write architecture principles
    writeln!(file, "\n## Architecture Principles")?;
    for principle in &analysis.architecture.principles {
        writeln!(file, "- {}", principle)?;
    }
    
    // Write design patterns
    writeln!(file, "\n## Design Patterns")?;
    for pattern in &analysis.architecture.patterns {
        writeln!(file, "- {}", pattern)?;
    }
    
    // Write project statistics
    writeln!(file, "\n## Project Statistics")?;
    writeln!(file, "\n- **Total Files**: {}", analysis.summary.total_files)?;
    writeln!(file, "- **Lines of Code**: {}", analysis.summary.lines_of_code)?;
    writeln!(file, "- **Rust Files**: {}", analysis.summary.rust_files)?;
    writeln!(file, "- **Haskell Files**: {}", analysis.summary.haskell_files)?;
    
    // Write file types
    writeln!(file, "\n### File Types")?;
    writeln!(file, "\n| Extension | Count |")?;
    writeln!(file, "|-----------|-------|")?;
    for (ext, count) in &analysis.summary.file_types {
        writeln!(file, "| {} | {} |", ext, count)?;
    }
    
    // Write integration status
    writeln!(file, "\n## Integration Status")?;
    writeln!(file, "\n| Integration | Source | Target | Status |")?;
    writeln!(file, "|-------------|--------|--------|--------|")?;
    for integration in &analysis.integrations {
        writeln!(file, "| {} | {} | {} | {} |", 
            integration.name, 
            integration.source_system, 
            integration.target_system, 
            integration.status)?;
    }
    
    // Write documentation links
    writeln!(file, "\n## Documentation Links")?;
    writeln!(file, "\n- [Architecture Documentation](./architecture/overview.md)")?;
    writeln!(file, "- [Models Documentation](./models/overview.md)")?;
    writeln!(file, "- [Integration Documentation](./integration/overview.md)")?;
    writeln!(file, "- [Blockchain Implementation](../rag_knowledge_base/integration/blockchain_implementation.md)")?;
    
    // Write AI guidance
    writeln!(file, "\n## AI Development Guidance")?;
    writeln!(file, "\nThis project is built with Rust and Haskell as the primary languages. When developing new features or modifying existing ones, adhere to the following principles:")?;
    writeln!(file, "\n1. **Rust-First Approach**: Implement core functionality in Rust whenever possible.")?;
    writeln!(file, "2. **Functional Paradigm**: Use functional programming patterns, especially for complex business logic.")?;
    writeln!(file, "3. **No JavaScript Dependencies**: Avoid JavaScript/TypeScript dependencies unless absolutely necessary.")?;
    writeln!(file, "4. **Performance Focus**: Prioritize performance in all implementations.")?;
    writeln!(file, "5. **Offline-First**: Design features to work offline by default.")?;
    writeln!(file, "6. **Security**: Implement proper authentication and authorization checks.")?;
    
    println!("Central reference hub generated at {}", hub_path.display());
    Ok(())
}

// Generate architecture documentation
fn generate_architecture_doc(analysis: &ProjectAnalysis) -> Result<(), Box<dyn Error>> {
    let arch_dir = Path::new("docs").join("architecture");
    if !arch_dir.exists() {
        fs::create_dir_all(&arch_dir)?;
    }
    
    let overview_path = arch_dir.join("overview.md");
    let mut file = File::create(overview_path)?;
    
    // Write header
    writeln!(file, "# LMS Architecture Overview")?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)?;
    
    // Write architecture overview
    writeln!(file, "## Architecture Principles")?;
    writeln!(file, "\nThe LMS project follows these key architectural principles:")?;
    for principle in &analysis.architecture.principles {
        writeln!(file, "- **{}**", principle)?;
    }
    
    // Write design patterns
    writeln!(file, "\n## Design Patterns")?;
    writeln!(file, "\nThe following design patterns are employed throughout the codebase:")?;
    for pattern in &analysis.architecture.patterns {
        writeln!(file, "- **{}**", pattern)?;
    }
    
    // Write component overview
    writeln!(file, "\n## Component Overview")?;
    writeln!(file, "\nThe system is composed of the following major components:")?;
    
    // Group components by directory/module
    let mut component_groups: HashMap<String, Vec<&Component>> = HashMap::new();
    for component in &analysis.components {
        let path = Path::new(&component.file_path);
        let parent = path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Other");
        
        component_groups.entry(parent.to_string()).or_default().push(component);
    }
    
    for (group, components) in component_groups {
        writeln!(file, "\n### {} Components", group)?;
        for component in components {
            writeln!(file, "- **{}**: {}", component.name, component.description)?;
        }
    }
    
    // Write integration architecture
    writeln!(file, "\n## Integration Architecture")?;
    writeln!(file, "\nThe LMS integrates Canvas and Discourse through these mechanisms:")?;
    
    writeln!(file, "\n### Canvas Integration")?;
    writeln!(file, "- Course management functionality is migrated from Canvas")?;
    writeln!(file, "- Assignment and grading systems are preserved")?;
    writeln!(file, "- User authentication is unified")?;
    
    writeln!(file, "\n### Discourse Integration")?;
    writeln!(file, "- Discussion forums are embedded within course contexts")?;
    writeln!(file, "- User profiles are synchronized")?;
    writeln!(file, "- Notifications are unified")?;
    
    // Write data flow
    writeln!(file, "\n## Data Flow")?;
    writeln!(file, "\n```mermaid")?;
    writeln!(file, "graph TD")?;
    writeln!(file, "    User[User] --> UI[Leptos UI]")?;
    writeln!(file, "    UI --> API[Rust API Layer]")?;
    writeln!(file, "    API --> DB[(SQLite Database)]")?;
    writeln!(file, "    API --> Search[MeiliSearch]")?;
    writeln!(file, "    API --> Blockchain[Blockchain Verification]")?;
    writeln!(file, "    DB --> Sync[Sync Manager]")?;
    writeln!(file, "    Sync --> Remote[Remote Services]")?;
    writeln!(file, "```")?;
    
    // Write tech stack details
    writeln!(file, "\n## Technology Stack Details")?;
    
    writeln!(file, "\n### Frontend")?;
    writeln!(file, "- **Leptos**: Reactive web framework for Rust")?;
    writeln!(file, "- **Tauri**: Desktop application framework")?;
    
    writeln!(file, "\n### Backend")?;
    writeln!(file, "- **Rust**: Primary language for performance and safety")?;
    writeln!(file, "- **Haskell**: Used for complex business logic")?;
    
    writeln!(file, "\n### Database")?;
    writeln!(file, "- **SQLite**: Embedded database for offline-first operation")?;
    writeln!(file, "- **sqlx**: Rust SQL toolkit and ORM")?;
    
    writeln!(file, "\n### Search")?;
    writeln!(file, "- **MeiliSearch**: Embedded search engine")?;
    
    println!("Architecture documentation generated at {}", overview_path.display());
    Ok(())
}

// Generate models documentation
fn generate_models_doc(analysis: &ProjectAnalysis) -> Result<(), Box<dyn Error>> {
    let models_dir = Path::new("docs").join("models");
    if !models_dir.exists() {
        fs::create_dir_all(&models_dir)?;
    }
    
    let overview_path = models_dir.join("overview.md");
    let mut file = File::create(overview_path)?;
    
    // Write header
    writeln!(file, "# LMS Data Models")?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)?;
    
    // Group models by source system
    let mut system_models: HashMap<String, Vec<&Model>> = HashMap::new();
    for model in &analysis.models {
        system_models.entry(model.source_system.clone()).or_default().push(model);
    }
    
    // Write models by source system
    for (system, models) in system_models {
        writeln!(file, "## {} Models", system)?;
        writeln!(file, "\n| Model | Fields | File Path |")?;
        writeln!(file, "|-------|--------|-----------|")?;
        
        for model in models {
            let fields_str = if model.fields.is_empty() {
                "None detected".to_string()
            } else {
                model.fields.join(", ")
            };
            
            writeln!(file, "| {} | {} | {} |", 
                model.name, 
                fields_str, 
                model.file_path)?;
        }
        writeln!(file)?;
    }
    
    // Write entity relationships
    writeln!(file, "## Entity Relationships")?;
    writeln!(file, "\n```mermaid")?;
    writeln!(file, "erDiagram")?;
    
    // Add some key relationships (these would ideally be detected from code)
    writeln!(file, "    USER ||--o{ COURSE : enrolls")?;
    writeln!(file, "    COURSE ||--o{ ASSIGNMENT : contains")?;
    writeln!(file, "    ASSIGNMENT ||--o{ SUBMISSION : receives")?;
    writeln!(file, "    USER ||--o{ SUBMISSION : submits")?;
    writeln!(file, "    COURSE ||--o{ DISCUSSION : hosts")?;
    writeln!(file, "    USER ||--o{ POST : writes")?;
    writeln!(file, "    DISCUSSION ||--o{ POST : contains")?;
    writeln!(file, "    USER ||--o{ CERTIFICATE : earns")?;
    writeln!(file, "```")?;
    
    // Write data migration notes
    writeln!(file, "\n## Data Migration Notes")?;
    writeln!(file, "\n### Canvas to LMS")?;
    writeln!(file, "- Course data is migrated with structure preserved")?;
    writeln!(file, "- User accounts are synchronized with unified authentication")?;
    writeln!(file, "- Assignment and submission history is maintained")?;
    
    writeln!(file, "\n### Discourse to LMS")?;
    writeln!(file, "- Discussion forums are embedded within course contexts")?;
    writeln!(file, "- User profiles are synchronized")?;
    writeln!(file, "- Post history and attachments are preserved")?;
    
    println!("Models documentation generated at {}", overview_path.display());
    Ok(())
}

// Generate integration documentation
fn generate_integration_doc(analysis: &ProjectAnalysis) -> Result<(), Box<dyn Error>> {
    let integration_dir = Path::new("docs").join("integration");
    if !integration_dir.exists() {
        fs::create_dir_all(&integration_dir)?;
    }
    
    let overview_path = integration_dir.join("overview.md");
    let mut file = File::create(overview_path)?;
    
    // Write header
    writeln!(file, "# LMS Integration Overview")?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)?;
    
    // Write integration overview
    writeln!(file, "## Integration Strategy")?;
    writeln!(file, "\nThe LMS project integrates Canvas LMS and Discourse forum functionality into a unified Rust/Tauri/Leptos application. The integration follows these principles:")?;
    writeln!(file, "\n- **Unified Data Model**: Core entities from both systems are mapped to a common data model")?;
    writeln!(file, "- **Consistent UI**: A unified UI experience across all functionality")?;
    writeln!(file, "- **Offline-First**: All features work offline with synchronization when online")?;
    writeln!(file, "- **Performance**: Optimized for speed and resource efficiency")?;
    writeln!(file, "- **Security**: Comprehensive security model across all integrated components")?;
    
    // Write integration details
    writeln!(file, "\n## Integration Components")?;
    
    for integration in &analysis.integrations {
        writeln!(file, "- **{}**: Integrates {} with {}. Status: {}", 
            integration.name, 
            integration.source_system, 
            integration.target_system, 
            integration.status)?;
    }
    
    println!("Integration documentation generated at {}", overview_path.display());
    Ok(())
}