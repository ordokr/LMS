use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use chrono::Local;
use toml; // For TOML parsing
use syn::{visit::{self, Visit}, ItemStruct, ItemUse, Meta};

mod conflict_analyzer; // Port of port-conflict-analyzer.js
mod docs_generator;    // Port of technical-docs-generator.js
mod summary_generator; // Port of summary-report-generator.js
mod dashboard_generator; // Port of visual-dashboard-generator.js
mod js_migration_analyzer; // New module for tracking JS-to-Rust migration

// Structure to hold Gemini analysis results
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
struct GeminiInsights {
    code_quality: String,
    potential_conflicts: String,
    architecture_adherence: String,
    next_steps: String,
}

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
    #[serde(default)]
    gemini_insights: Option<GeminiInsights>,
    #[serde(default)]
    js_migration: Option<js_migration_analyzer::JsMigrationAnalysis>,
}

impl ProjectAnalysis {
    fn with_gemini_insights(mut self, insights: GeminiInsights) -> Self {
        // Add the insights to the struct
        self.gemini_insights = Some(insights);
        self
    }
    
    fn with_js_migration(mut self, js_migration: js_migration_analyzer::JsMigrationAnalysis) -> Self {
        // Add the JS migration analysis to the struct
        self.js_migration = Some(js_migration);
        self
    }
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

// Structure to hold the configuration loaded from analyzer_config.toml
#[derive(Debug, Deserialize)]
struct AnalyzerConfig {
    tech_stack: TechStack,
    architecture: ArchitectureInfo,
    integrations: Vec<Integration>,
}

// Mock GenerativeAi struct since actual implementation may require external dependencies
struct GenerativeAi {
    api_key: String,
}

impl GenerativeAi {
    fn new(api_key: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self { api_key })
    }
}

// Main function to run the analysis
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting LMS Project Analysis...");
    
    // Run the analysis
    let mut analysis = analyze_project()?;
    
    // Run conflict analysis
    let conflicts = conflict_analyzer::analyze_conflicts(Path::new("."))?;
    
    // Run JavaScript migration analysis
    let js_migration_analysis = js_migration_analyzer::analyze_js_migration(Path::new("."))?;
    
    // Add JS migration analysis to the project analysis
    analysis = analysis.with_js_migration(js_migration_analysis);
    
    // Generate JavaScript migration report
    generate_js_migration_report(&analysis)?;
    
    // Generate summary report
    let summary = summary_generator::generate_summary(&analysis)?;
    
    // Generate documentation
    generate_central_reference_hub(&analysis)?;
    generate_architecture_doc(&analysis)?;
    generate_models_doc(&analysis)?;
    generate_integration_doc(&analysis)?;
    
    // Generate dashboard
    dashboard_generator::generate_dashboard(&analysis, &conflicts, &summary)?;
    
    println!("Analysis complete. Documentation updated.");
    Ok(())
}

// Load configuration from analyzer_config.toml
fn load_config() -> Result<AnalyzerConfig, Box<dyn Error>> {
    let config_path = Path::new("analyzer_config.toml"); // Assumes run from workspace root
    let mut file = File::open(config_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let config: AnalyzerConfig = toml::from_str(&content)?;
    Ok(config)
}

// Analyze the entire project
fn analyze_project() -> Result<ProjectAnalysis, Box<dyn Error>> {
    let project_root = Path::new(".");
    let now = Local::now();
    let config = load_config()?; // Load configuration
    
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
    
    // Create Gemini insights
    let mut gemini_insights = GeminiInsights::default();

    // Attempt to get API key if available
    let gemini_api_key = std::env::var("GEMINI_API_KEY").ok();
    
    if let Some(api_key) = gemini_api_key {
        // Use actual Gemini API if key is available
        match GenerativeAi::new(api_key) {
            Ok(genai) => {
                // Call Gemini for code quality analysis
                if let Ok(analysis) = analyze_code_quality(&genai, &components, &models) {
                    gemini_insights.code_quality = analysis;
                }

                // Call Gemini for potential conflicts analysis
                if let Ok(analysis) = analyze_potential_conflicts(&genai, &components, &models) {
                    gemini_insights.potential_conflicts = analysis;
                }

                // Call Gemini for architecture adherence analysis
                if let Ok(analysis) = analyze_architecture_adherence(&genai, &components, &models, &config.architecture) {
                    gemini_insights.architecture_adherence = analysis;
                }

                // Call Gemini for next steps analysis
                if let Ok(analysis) = analyze_next_steps(&genai, &components, &models) {
                    gemini_insights.next_steps = analysis;
                }
            },
            Err(e) => {
                println!("Failed to initialize Gemini API: {}", e);
                gemini_insights.code_quality = "Gemini analysis failed to initialize.".to_string();
                gemini_insights.potential_conflicts = "Gemini analysis failed to initialize.".to_string();
                gemini_insights.architecture_adherence = "Gemini analysis failed to initialize.".to_string();
                gemini_insights.next_steps = "Gemini analysis failed to initialize.".to_string();
            }
        }
    } else {
        // Use placeholder messages if API key is not available
        gemini_insights.code_quality = "Gemini analysis is disabled due to missing API key.".to_string();
        gemini_insights.potential_conflicts = "Gemini analysis is disabled due to missing API key.".to_string();
        gemini_insights.architecture_adherence = "Gemini analysis is disabled due to missing API key.".to_string();
        gemini_insights.next_steps = "Gemini analysis is disabled due to missing API key.".to_string();
    }
    
    Ok(ProjectAnalysis {
        timestamp: now.to_rfc3339(),
        summary,
        components,
        models,
        routes,
        integrations: config.integrations,
        tech_stack: config.tech_stack,
        architecture: config.architecture,
        gemini_insights: Some(gemini_insights),
    })
}

// Analyze code quality using Gemini
fn analyze_code_quality(genai: &GenerativeAi, components: &Vec<Component>, models: &Vec<Model>) -> Result<String, Box<dyn Error>> {
    let prompt = format!("Analyze the code quality of the following components and models:\nComponents: {:?}\nModels: {:?}\nProvide a summary of potential code quality issues and suggestions for improvement.", components, models);
    let response = call_gemini_api(genai, &prompt)?;
    Ok(response)
}

// Analyze potential conflicts using Gemini
fn analyze_potential_conflicts(genai: &GenerativeAi, components: &Vec<Component>, models: &Vec<Model>) -> Result<String, Box<dyn Error>> {
    let prompt = format!("Analyze the potential conflicts between the following components and models:\nComponents: {:?}\nModels: {:?}\nProvide a summary of potential conflicts and suggestions for resolution.", components, models);
    let response = call_gemini_api(genai, &prompt)?;
    Ok(response)
}

// Analyze architecture adherence using Gemini
fn analyze_architecture_adherence(genai: &GenerativeAi, components: &Vec<Component>, models: &Vec<Model>, architecture: &ArchitectureInfo) -> Result<String, Box<dyn Error>> {
    let prompt = format!("Analyze the adherence to the following architecture principles for the following components and models:\nComponents: {:?}\nModels: {:?}\nArchitecture Principles: {:?}\nProvide a summary of adherence and suggestions for improvement.", components, models, architecture);
    let response = call_gemini_api(genai, &prompt)?;
    Ok(response)
}

// Analyze next steps using Gemini
fn analyze_next_steps(genai: &GenerativeAi, components: &Vec<Component>, models: &Vec<Model>) -> Result<String, Box<dyn Error>> {
    let prompt = format!("Analyze the next steps for the following components and models:\nComponents: {:?}\nModels: {:?}\nProvide a summary of potential next steps and suggestions for implementation.", components, models);
    let response = call_gemini_api(genai, &prompt)?;
    Ok(response)
}

// Call Gemini API
fn call_gemini_api(genai: &GenerativeAi, prompt: &str) -> Result<String, Box<dyn Error>> {
    // Call Gemini API here
    // Replace this with actual Gemini API call
    // For now, return a placeholder response
    println!("Calling Gemini API with prompt: {}", prompt);
    let response = "Gemini API response placeholder".to_string();
    Ok(response)
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

// Visitor struct to traverse the Rust AST
struct RustCodeVisitor<'a> {
    components: &'a mut Vec<Component>,
    models: &'a mut Vec<Model>,
    // routes: &'a mut Vec<Route>, // Route analysis via AST is complex, keep simple logic for now
    current_file_path: String,
    current_dependencies: Vec<String>,
}

impl<'ast, 'a> Visit<'ast> for RustCodeVisitor<'a> {
    // Visit 'use' statements to gather dependencies for the current file
    fn visit_item_use(&mut self, i: &'ast ItemUse) {
        // A simple way to capture the use path as a string
        // More complex parsing could handle aliases and specific imports
        let path_str = parse_use_tree(&i.tree);
        self.current_dependencies.push(path_str);
        // Continue visiting nested use trees if any
        visit::visit_item_use(self, i);
    }

    // Visit struct definitions
    fn visit_item_struct(&mut self, i: &'ast ItemStruct) {
        let struct_name = i.ident.to_string();
        let file_path = self.current_file_path.clone();

        // Check attributes for derives indicating a model
        let is_model = i.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            let mut has_model_trait = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("Serialize") || 
                   meta.path.is_ident("Deserialize") || 
                   meta.path.is_ident("FromRow") {
                    has_model_trait = true;
                }
                Ok(())
            });
            if has_model_trait {
                return true;
            }
        }
        false
        });

        // Check attributes for Leptos component macro
        let is_component_macro = i.attrs.iter().any(|attr| {
             attr.path().is_ident("component") // Use path() method
        });

        if is_model {
            let mut fields = Vec::new();
            if let syn::Fields::Named(fields_named) = &i.fields {
                for field in fields_named.named.iter() {
                    if let Some(ident) = &field.ident {
                        fields.push(ident.to_string());
                    }
                }
            }

            // Determine source system (simple path-based logic retained)
            let source_system = if file_path.contains("canvas") {
                "Canvas".to_string()
            } else if file_path.contains("discourse") {
                "Discourse".to_string()
            } else {
                "Native".to_string()
            };

            self.models.push(Model {
                name: struct_name,
                file_path,
                fields,
                associations: Vec::new(), // Association detection needs more logic
                source_system,
            });

        } else if is_component_macro || struct_name.ends_with("Component") || struct_name.ends_with("View") {
             // Treat structs marked with #[component] or ending in Component/View as components
             // This is a heuristic and might need refinement based on project conventions
            self.components.push(Component {
                name: struct_name,
                file_path,
                dependencies: self.current_dependencies.clone(), // Associate file-level dependencies
                description: if is_component_macro { "Leptos component".to_string() } else { "Potential UI component".to_string() },
            });
        }

        // Continue visiting nested items within the struct if needed
        visit::visit_item_struct(self, i);
    }

    // Potentially visit functions (ItemFn) or macros (ItemMacro) later if needed
}

// Helper to get a string representation of a UseTree (simplified)
fn parse_use_tree(tree: &syn::UseTree) -> String {
    match tree {
        syn::UseTree::Path(p) => format!("{}::{}", p.ident, parse_use_tree(&p.tree)),
        syn::UseTree::Name(n) => n.ident.to_string(),
        syn::UseTree::Rename(r) => format!("{} as {}", r.ident, r.rename),
        syn::UseTree::Glob(_) => "*".to_string(),
        syn::UseTree::Group(g) => {
            let items = g.items.iter().map(parse_use_tree).collect::<Vec<_>>().join(", ");
            format!("{{{}}}", items)
        }
    }
}

// Analyze Rust files using AST parsing
fn analyze_rust_file(
    path: &Path, 
    components: &mut Vec<Component>,
    models: &mut Vec<Model>,
    routes: &mut Vec<Route>
) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    // Attempt to parse the file. If parsing fails (e.g., syntax errors), log and skip.
    let ast = match syn::parse_file(&content) {
         Ok(file) => file,
         Err(e) => {
              eprintln!("Failed to parse Rust file: {} - Error: {}", path.display(), e);
              return Ok(()); // Skip this file on parsing error
         }
    };

    // Create and run the visitor
    let mut visitor = RustCodeVisitor {
        components,
        models,
        // routes, // Pass routes if AST analysis for them is added later
        current_file_path: path.to_string_lossy().to_string(),
        current_dependencies: Vec::new(),
    };
    visitor.visit_file(&ast);


    // Keep simple route detection for now, as AST analysis is complex and framework-specific
    // This part might need to be adapted based on how routes are defined (Axum, Leptos Router, etc.)
    if content.contains("Route::new") || content.contains("route!") || content.contains(".route(") {
        for line in content.lines() {
             // Look for patterns like route("/path", get(handler)) or Route::new(...)
             // This remains a heuristic approach.
            if let Some(path_part) = line.split('"').nth(1) {
                 if path_part.starts_with('/') { // Basic check for a path string
                      routes.push(Route {
                          path: path_part.to_string(),
                          component: "Unknown (AST analysis needed)".to_string(),
                          methods: vec!["GET".to_string()], // Default assumption, needs improvement
                          auth_required: line.contains("auth") || line.contains("protected") || line.contains("Auth"), // Heuristic
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
    let mut file = File::create(&hub_path)?;
    
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
    
    // Write AI Coding Agent Guidance
    writeln!(file, "\n## AI Coding Agent Guidance")?;
    writeln!(file, "\nThis section provides guidance for AI coding agents working on this project.")?;
    writeln!(file, "\n### Project Goals")?;
    writeln!(file, "\nThe primary goal is to create a unified LMS system by integrating Canvas and Discourse, prioritizing performance, security, and offline-first capabilities.")?;
    writeln!(file, "\n### Architectural Constraints")?;
    writeln!(file, "\n- **Languages**: Use Rust and Haskell exclusively. Avoid JavaScript/TypeScript.")?;
    writeln!(file, "- **Frameworks**: Adhere to Rust-idiomatic frameworks (e.g., Leptos, Tauri, Axum) and avoid introducing incompatible technologies.")?;
    writeln!(file, "- **Data Storage**: Utilize SQLite for local data storage and consider MeiliSearch for search indexing.")?;
    writeln!(file, "\n### Key Components")?;
    if analysis.components.is_empty() {
        writeln!(file, "\nNo components detected during analysis.")?;
    } else {
        writeln!(file, "\nThe following components were detected:")?;
        for component in &analysis.components {
            writeln!(file, "- **{}**: {} ({})", component.name, component.description, component.file_path)?;
        }
    }
    writeln!(file, "\n### Models")?;
    if analysis.models.is_empty() {
        writeln!(file, "\nNo models detected during analysis.")?;
    } else {
        writeln!(file, "\nThe following data models were detected:")?;
        for model in &analysis.models {
            writeln!(file, "- **{}**: {} ({})", model.name, model.source_system, model.file_path)?;
        }
    }
    
    // Write Gemini Insights
    writeln!(file, "\n## Gemini Insights")?;
    writeln!(file, "\n### Code Quality")?;
    writeln!(file, "\n{}", analysis.gemini_insights.as_ref().map(|i| i.code_quality.clone()).unwrap_or_else(|| "No insights available".to_string()))?;

    writeln!(file, "\n### Potential Conflicts")?;
    writeln!(file, "\n{}", analysis.gemini_insights.as_ref().map(|i| i.potential_conflicts.clone()).unwrap_or_else(|| "No insights available".to_string()))?;

    writeln!(file, "\n### Architecture Adherence")?;
    writeln!(file, "\n{}", analysis.gemini_insights.as_ref().map(|i| i.architecture_adherence.clone()).unwrap_or_else(|| "No insights available".to_string()))?;

    writeln!(file, "\n### Next Steps")?;
    writeln!(file, "\n{}", analysis.gemini_insights.as_ref().map(|i| i.next_steps.clone()).unwrap_or_else(|| "No insights available".to_string()))?;
    
    writeln!(file, "\n### Potential Next Steps")?;
    writeln!(file, "\n- Implement missing features in Rust, following the existing architecture.")?;
    writeln!(file, "- Refactor existing code to improve performance and maintainability.")?;
    writeln!(file, "- Integrate Gemini for code analysis and automated documentation.")?;
    writeln!(file, "- Implement robust testing strategies to ensure code quality and security.")?;
    
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
    let mut file = File::create(&overview_path)?;
    
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
    let mut file = File::create(&overview_path)?;
    
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
    writeln!(file, "\n## Entity Relationships")?;
    writeln!(file, "\n```mermaid")?;
    writeln!(file, "erDiagram")?;
    writeln!(file, "    USER ||--o{{ COURSE : enrolls")?;
    writeln!(file, "    COURSE ||--o{{ ASSIGNMENT : contains")?;
    writeln!(file, "    ASSIGNMENT ||--o{{ SUBMISSION : receives")?;
    writeln!(file, "    USER ||--o{{ SUBMISSION : submits")?;
    writeln!(file, "    COURSE ||--o{{ DISCUSSION : hosts")?;
    writeln!(file, "    USER ||--o{{ POST : writes")?;
    writeln!(file, "    DISCUSSION ||--o{{ POST : contains")?;
    writeln!(file, "    USER ||--o{{ CERTIFICATE : earns")?;
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
    let mut file = File::create(&overview_path)?;
    
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

// Generate JavaScript migration report
fn generate_js_migration_report(analysis: &ProjectAnalysis) -> Result<(), Box<dyn Error>> {
    if let Some(js_migration) = &analysis.js_migration {
        let docs_dir = Path::new("docs");
        if !docs_dir.exists() {
            fs::create_dir_all(docs_dir)?;
        }
        
        let migration_dir = docs_dir.join("migration");
        if !migration_dir.exists() {
            fs::create_dir_all(&migration_dir)?;
        }
        
        // Generate the report using the js_migration_analyzer
        let report_content = js_migration_analyzer::generate_js_migration_report(js_migration)?;
        
        // Write the report to a file
        let report_path = migration_dir.join("js_to_rust_migration.md");
        let mut file = File::create(&report_path)?;
        file.write_all(report_content.as_bytes())?;
        
        // Also update the central tracking file
        let tracking_path = Path::new("JavaScript to Rust Migration Tracking.md");
        let mut tracking_file = File::create(tracking_path)?;
        
        // Write header
        writeln!(tracking_file, "# JavaScript to Rust Migration Tracking")?;
        writeln!(tracking_file, "\n_Last updated: {}_\n", Local::now().to_rfc3339())?;
        writeln!(tracking_file, "## Migration Progress")?;
        writeln!(tracking_file, "\n- Total JavaScript files: {}", js_migration.total_js_files)?;
        writeln!(tracking_file, "- Migration completed: {} files ({}%)", 
                 js_migration.migrated_count,
                 js_migration.completion_percentage.round())?;
        writeln!(tracking_file, "- Migration not started: {} files", js_migration.not_started_count)?;
        writeln!(tracking_file, "- Migration in progress: {} files", js_migration.in_progress_count)?;
        writeln!(tracking_file, "- Migration not needed: {} files\n", js_migration.not_needed_count)?;
        
        // Write migration tracking list
        writeln!(tracking_file, "## Completed Migrations")?;
        writeln!(tracking_file, "\n| JavaScript File | Rust Equivalent |")?;
        writeln!(tracking_file, "|----------------|-----------------|")?;
        
        for file in &js_migration.js_files {
            if let js_migration_analyzer::MigrationStatus::Completed = file.migration_status {
                let rust_path = file.rust_equivalent_path.as_deref().unwrap_or("Unknown");
                writeln!(tracking_file, "| [x] {} | {} |", file.relative_path, rust_path)?;
            }
        }
        
        writeln!(tracking_file, "\n## In Progress Migrations")?;
        writeln!(tracking_file, "\n| JavaScript File | Planned Rust Equivalent |")?;
        writeln!(tracking_file, "|----------------|--------------------------|")?;
        
        for file in &js_migration.js_files {
            if let js_migration_analyzer::MigrationStatus::InProgress = file.migration_status {
                let rust_path = file.rust_equivalent_path.as_deref().unwrap_or("TBD");
                writeln!(tracking_file, "| [ ] {} | {} |", file.relative_path, rust_path)?;
            }
        }
        
        // Add to central reference hub
        update_central_hub_with_js_migration(js_migration)?;
        
        println!("JavaScript migration report generated at {}", report_path.display());
        println!("Migration tracking file updated at {}", tracking_path.display());
    } else {
        println!("No JavaScript migration analysis available to generate report");
    }
    
    Ok(())
}

// Update central reference hub with JS migration information
fn update_central_hub_with_js_migration(js_migration: &js_migration_analyzer::JsMigrationAnalysis) -> Result<(), Box<dyn Error>> {
    let hub_path = Path::new("docs").join("central_reference_hub.md");
    
    if hub_path.exists() {
        let mut content = fs::read_to_string(&hub_path)?;
        
        // Check if the JS Migration section already exists
        if !content.contains("## JavaScript to Rust Migration") {
            // Add the migration section
            content.push_str("\n## JavaScript to Rust Migration\n\n");
            content.push_str(&format!("- **Total JavaScript files**: {}\n", js_migration.total_js_files));
            content.push_str(&format!("- **Migration progress**: {}%\n", js_migration.completion_percentage.round()));
            content.push_str(&format!("- **Files completed**: {}\n", js_migration.migrated_count));
            content.push_str(&format!("- **Files in progress**: {}\n", js_migration.in_progress_count));
            content.push_str(&format!("- **Files not started**: {}\n", js_migration.not_started_count));
            content.push_str(&format!("- **Files not needing migration**: {}\n", js_migration.not_needed_count));
            
            content.push_str("\n### High Priority Migration Files\n\n");
            
            if js_migration.high_priority_files.is_empty() {
                content.push_str("*All high priority files have been migrated!*\n");
            } else {
                for file in &js_migration.high_priority_files.iter().take(5) {
                    content.push_str(&format!("- `{}`\n", file));
                }
                
                if js_migration.high_priority_files.len() > 5 {
                    content.push_str(&format!("- ... and {} more high priority files\n", 
                        js_migration.high_priority_files.len() - 5));
                }
            }
            
            content.push_str("\n[View full JavaScript migration report](./migration/js_to_rust_migration.md)\n");
            
            // Write the updated content back to the file
            fs::write(&hub_path, content)?;
        }
    }
    
    Ok(())
}


