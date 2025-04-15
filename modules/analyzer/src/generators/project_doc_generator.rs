use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::project_analyzer::{ProjectAnalysis, ProjectAnalyzer};
use crate::core::analysis_result::AnalysisResult;

/// Generate all project documentation
pub fn generate_project_docs(analysis: &ProjectAnalysis) -> Result<(), String> {
    // Generate all documentation
    generate_central_reference_hub(analysis)?;
    generate_architecture_doc(analysis)?;
    generate_models_doc(analysis)?;
    generate_integration_doc(analysis)?;
    
    Ok(())
}

/// Generate the central reference hub document
pub fn generate_central_reference_hub(analysis: &ProjectAnalysis) -> Result<(), String> {
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    let hub_path = docs_dir.join("central_reference_hub.md");
    let mut file = File::create(&hub_path)
        .map_err(|e| format!("Failed to create central reference hub file: {}", e))?;
    
    // Write header
    writeln!(file, "# LMS Project: Central Reference Hub")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    
    // Write project overview
    writeln!(file, "## Project Overview")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\nThe LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    
    // Write tech stack
    writeln!(file, "## Technology Stack")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n### Frontend")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.frontend {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### Backend")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.backend {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### Database")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.database {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### Search")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.search {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### AI Integration")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.ai {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### Blockchain")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.blockchain {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    writeln!(file, "\n### Authentication")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for tech in &analysis.tech_stack.authentication {
        writeln!(file, "- {}", tech)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    // Write architecture principles
    writeln!(file, "\n## Architecture Principles")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for principle in &analysis.architecture.principles {
        writeln!(file, "- {}", principle)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    // Write design patterns
    writeln!(file, "\n## Design Patterns")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for pattern in &analysis.architecture.patterns {
        writeln!(file, "- {}", pattern)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    // Write project statistics
    writeln!(file, "\n## Project Statistics")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n- **Total Files**: {}", analysis.summary.total_files)
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- **Lines of Code**: {}", analysis.summary.lines_of_code)
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- **Rust Files**: {}", analysis.summary.rust_files)
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- **Haskell Files**: {}", analysis.summary.haskell_files)
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    
    // Write file types
    writeln!(file, "\n### File Types")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n| Extension | Count |")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "|-----------|-------|")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for (ext, count) in &analysis.summary.file_types {
        writeln!(file, "| {} | {} |", ext, count)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    // Write integration status
    writeln!(file, "\n## Integration Status")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n| Integration | Source | Target | Status |")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "|-------------|--------|--------|--------|")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    for integration in &analysis.integrations {
        writeln!(file, "| {} | {} | {} | {} |", 
            integration.name, 
            integration.source_system, 
            integration.target_system, 
            integration.status)
            .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    }
    
    // Write documentation links
    writeln!(file, "\n## Documentation Links")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n- [Architecture Documentation](./architecture/overview.md)")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- [Models Documentation](./models/overview.md)")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- [Integration Documentation](./integration/overview.md)")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "- [Blockchain Implementation](../rag_knowledge_base/integration/blockchain_implementation.md)")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    
    // Write AI guidance
    writeln!(file, "\n## AI Development Guidance")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\nThis project is built with Rust and Haskell as the primary languages. When developing new features or modifying existing ones, adhere to the following principles:")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "\n1. **Rust-First Approach**: Implement core functionality in Rust whenever possible.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "2. **Functional Paradigm**: Use functional programming patterns, especially for complex business logic.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "3. **No JavaScript Dependencies**: Avoid JavaScript/TypeScript dependencies unless absolutely necessary.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "4. **Performance Focus**: Prioritize performance in all implementations.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "5. **Offline-First**: Design features to work offline by default.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    writeln!(file, "6. **Security**: Implement proper authentication and authorization checks.")
        .map_err(|e| format!("Failed to write to central reference hub: {}", e))?;
    
    println!("Central reference hub generated at {}", hub_path.display());
    Ok(())
}

/// Generate architecture documentation
pub fn generate_architecture_doc(analysis: &ProjectAnalysis) -> Result<(), String> {
    let arch_dir = Path::new("docs").join("architecture");
    if !arch_dir.exists() {
        fs::create_dir_all(&arch_dir)
            .map_err(|e| format!("Failed to create architecture directory: {}", e))?;
    }
    
    let overview_path = arch_dir.join("overview.md");
    let mut file = File::create(&overview_path)
        .map_err(|e| format!("Failed to create architecture overview file: {}", e))?;
    
    // Write header
    writeln!(file, "# LMS Architecture Overview")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    // Write architecture overview
    writeln!(file, "## Architecture Principles")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\nThe LMS project follows these key architectural principles:")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    for principle in &analysis.architecture.principles {
        writeln!(file, "- **{}**", principle)
            .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    }
    
    // Write design patterns
    writeln!(file, "\n## Design Patterns")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\nThe following design patterns are employed throughout the codebase:")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    for pattern in &analysis.architecture.patterns {
        writeln!(file, "- **{}**", pattern)
            .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    }
    
    // Write component overview
    writeln!(file, "\n## Component Overview")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\nThe system is composed of the following major components:")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    // Group components by directory/module
    use std::collections::HashMap;
    let mut component_groups: HashMap<String, Vec<&crate::core::project_analyzer::Component>> = HashMap::new();
    for component in &analysis.components {
        let path = Path::new(&component.file_path);
        let parent = path.parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("Other");
        
        component_groups.entry(parent.to_string()).or_default().push(component);
    }
    
    for (group, components) in component_groups {
        writeln!(file, "\n### {} Components", group)
            .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
        for component in components {
            writeln!(file, "- **{}**: {}", component.name, component.description)
                .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
        }
    }
    
    // Write integration architecture
    writeln!(file, "\n## Integration Architecture")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\nThe LMS integrates Canvas and Discourse through these mechanisms:")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Canvas Integration")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- Course management functionality is migrated from Canvas")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- Assignment and grading systems are preserved")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- User authentication is unified")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Discourse Integration")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- Discussion forums are embedded within course contexts")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- User profiles are synchronized")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- Notifications are unified")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    // Write data flow
    writeln!(file, "\n## Data Flow")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "\n```mermaid")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "graph TD")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    User[User] --> UI[Leptos UI]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    UI --> API[Rust API Layer]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    API --> DB[(SQLite Database)]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    API --> Search[MeiliSearch]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    API --> Blockchain[Blockchain Verification]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    DB --> Sync[Sync Manager]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "    Sync --> Remote[Remote Services]")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "```")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    // Write tech stack details
    writeln!(file, "\n## Technology Stack Details")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Frontend")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **Leptos**: Reactive web framework for Rust")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **Tauri**: Desktop application framework")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Backend")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **Rust**: Primary language for performance and safety")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **Haskell**: Used for complex business logic")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Database")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **SQLite**: Embedded database for offline-first operation")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **sqlx**: Rust SQL toolkit and ORM")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    writeln!(file, "\n### Search")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    writeln!(file, "- **MeiliSearch**: Embedded search engine")
        .map_err(|e| format!("Failed to write to architecture overview: {}", e))?;
    
    println!("Architecture documentation generated at {}", overview_path.display());
    Ok(())
}

/// Generate models documentation
pub fn generate_models_doc(analysis: &ProjectAnalysis) -> Result<(), String> {
    let models_dir = Path::new("docs").join("models");
    if !models_dir.exists() {
        fs::create_dir_all(&models_dir)
            .map_err(|e| format!("Failed to create models directory: {}", e))?;
    }
    
    let overview_path = models_dir.join("overview.md");
    let mut file = File::create(&overview_path)
        .map_err(|e| format!("Failed to create models overview file: {}", e))?;
    
    // Write header
    writeln!(file, "# LMS Data Models")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    
    // Group models by source system
    use std::collections::HashMap;
    let mut system_models: HashMap<String, Vec<&crate::core::project_analyzer::Model>> = HashMap::new();
    for model in &analysis.models {
        system_models.entry(model.source_system.clone()).or_default().push(model);
    }
    
    // Write models by source system
    for (system, models) in system_models {
        writeln!(file, "## {} Models", system)
            .map_err(|e| format!("Failed to write to models overview: {}", e))?;
        writeln!(file, "\n| Model | Fields | File Path |")
            .map_err(|e| format!("Failed to write to models overview: {}", e))?;
        writeln!(file, "|-------|--------|-----------|")
            .map_err(|e| format!("Failed to write to models overview: {}", e))?;
        
        for model in models {
            let fields_str = if model.fields.is_empty() {
                "None detected".to_string()
            } else {
                model.fields.join(", ")
            };
            
            writeln!(file, "| {} | {} | {} |", 
                model.name, 
                fields_str, 
                model.file_path)
                .map_err(|e| format!("Failed to write to models overview: {}", e))?;
        }
        writeln!(file)
            .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    }
    
    // Write entity relationships
    writeln!(file, "## Entity Relationships")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "\n```mermaid")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "erDiagram")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    
    // Add some key relationships (these would ideally be detected from code)
    writeln!(file, "    USER ||--o{{ COURSE : enrolls")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    COURSE ||--o{{ ASSIGNMENT : contains")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    ASSIGNMENT ||--o{{ SUBMISSION : receives")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    USER ||--o{{ SUBMISSION : submits")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    COURSE ||--o{{ DISCUSSION : hosts")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    USER ||--o{{ POST : writes")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    DISCUSSION ||--o{{ POST : contains")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "    USER ||--o{{ CERTIFICATE : earns")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "```")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    
    // Write data migration notes
    writeln!(file, "\n## Data Migration Notes")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "\n### Canvas to LMS")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- Course data is migrated with structure preserved")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- User accounts are synchronized with unified authentication")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- Assignment and submission history is maintained")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    
    writeln!(file, "\n### Discourse to LMS")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- Discussion forums are embedded within course contexts")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- User profiles are synchronized")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    writeln!(file, "- Post history and attachments are preserved")
        .map_err(|e| format!("Failed to write to models overview: {}", e))?;
    
    println!("Models documentation generated at {}", overview_path.display());
    Ok(())
}

/// Generate integration documentation
pub fn generate_integration_doc(analysis: &ProjectAnalysis) -> Result<(), String> {
    let integration_dir = Path::new("docs").join("integration");
    if !integration_dir.exists() {
        fs::create_dir_all(&integration_dir)
            .map_err(|e| format!("Failed to create integration directory: {}", e))?;
    }
    
    let overview_path = integration_dir.join("overview.md");
    let mut file = File::create(&overview_path)
        .map_err(|e| format!("Failed to create integration overview file: {}", e))?;
    
    // Write header
    writeln!(file, "# LMS Integration Overview")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "\n_Last updated: {}_\n", analysis.timestamp)
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    
    // Write integration overview
    writeln!(file, "## Integration Strategy")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "\nThe LMS project integrates Canvas LMS and Discourse forum functionality into a unified Rust/Tauri/Leptos application. The integration follows these principles:")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "\n- **Unified Data Model**: Core entities from both systems are mapped to a common data model")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "- **Consistent UI**: A unified UI experience across all functionality")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "- **Offline-First**: All features work offline with synchronization when online")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "- **Performance**: Optimized for speed and resource efficiency")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    writeln!(file, "- **Security**: Comprehensive security model across all integrated components")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    
    // Write integration details
    writeln!(file, "\n## Integration Components")
        .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    
    for integration in &analysis.integrations {
        writeln!(file, "- **{}**: Integrates {} with {}. Status: {}", 
            integration.name, 
            integration.source_system, 
            integration.target_system, 
            integration.status)
            .map_err(|e| format!("Failed to write to integration overview: {}", e))?;
    }
    
    println!("Integration documentation generated at {}", overview_path.display());
    Ok(())
}

/// Run project analysis and generate documentation
pub async fn run_project_analysis_and_generate_docs(config: &crate::core::analyzer_config::AnalyzerConfig) -> Result<(), String> {
    // Create project analyzer
    let analyzer = ProjectAnalyzer::new(config.clone());
    
    // Run analysis
    let analysis = analyzer.analyze()
        .map_err(|e| format!("Project analysis failed: {}", e))?;
    
    // Generate documentation
    generate_project_docs(&analysis)?;
    
    // Convert to AnalysisResult for unified analyzer
    let result = analyzer.to_analysis_result(&analysis);
    
    Ok(())
}
