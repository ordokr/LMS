use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

use crate::core::analysis_result::AnalysisResult;

/// Generate the central reference hub
pub fn generate_central_reference_hub(result: &AnalysisResult) -> Result<(), String> {
    println!("Generating central reference hub...");
    
    // Ensure docs directory exists
    let docs_dir = Path::new("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(docs_dir)
            .map_err(|e| format!("Failed to create docs directory: {}", e))?;
    }
    
    // Create the hub path
    let hub_path = docs_dir.join("central_reference_hub.md");
    
    // Generate the content
    let content = generate_hub_content(result)?;
    
    // Write to file
    fs::write(&hub_path, content)
        .map_err(|e| format!("Failed to write central reference hub: {}", e))?;
    
    println!("Central reference hub generated at: {:?}", hub_path);
    
    Ok(())
}

/// Generate the content for the central reference hub
fn generate_hub_content(result: &AnalysisResult) -> Result<String, String> {
    let mut content = String::new();
    
    // Header
    content.push_str("# LMS Project: Central Reference Hub\n\n");
    content.push_str(&format!("_Last updated: {}_\n\n", result.timestamp.to_rfc3339()));
    
    // Project Overview
    content.push_str("## Project Overview\n\n");
    content.push_str("The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n\n");
    
    // Technology Stack
    content.push_str("## Technology Stack\n\n");
    
    content.push_str("### Frontend\n");
    content.push_str("- Leptos\n");
    content.push_str("- Tauri\n\n");
    
    content.push_str("### Backend\n");
    content.push_str("- Rust\n");
    content.push_str("- Haskell\n\n");
    
    content.push_str("### Database\n");
    content.push_str("- SQLite\n");
    content.push_str("- sqlx\n\n");
    
    content.push_str("### Search\n");
    content.push_str("- MeiliSearch\n\n");
    
    content.push_str("### AI Integration\n");
    content.push_str("- Local AI Model implementation via LM Studio or the like\n\n");
    
    content.push_str("### Blockchain\n");
    content.push_str("- Custom Rust implementation\n\n");
    
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
    
    content.push_str(&format!("- **Total Files**: {}\n", result.summary.total_files));
    content.push_str(&format!("- **Lines of Code**: {}\n", result.summary.lines_of_code));
    content.push_str(&format!("- **Rust Files**: {}\n", result.summary.rust_files));
    content.push_str(&format!("- **Haskell Files**: {}\n\n", result.summary.haskell_files));
    
    // File Types
    content.push_str("### File Types\n\n");
    content.push_str("| Extension | Count |\n");
    content.push_str("|-----------|-------|\n");
    
    for (ext, count) in &result.summary.file_types {
        content.push_str(&format!("| {} | {} |\n", ext, count));
    }
    content.push_str("\n");
    
    // Integration Status
    content.push_str("## Integration Status\n\n");
    content.push_str("| Integration | Source | Target | Status |\n");
    content.push_str("|-------------|--------|--------|--------|\n");
    content.push_str("| Canvas Course Management | Canvas | LMS | Completed |\n");
    content.push_str("| Discourse Forums | Discourse | LMS | Planned |\n");
    content.push_str("| Blockchain Certification | Native | LMS | Completed |\n\n");
    
    // Recent Updates
    content.push_str("## Recent Updates\n\n");
    for change in &result.recent_changes {
        content.push_str(&format!("- **{}**\n", change));
    }
    content.push_str("\n");
    
    // Documentation Links
    content.push_str("## Documentation Links\n\n");
    content.push_str("- [Architecture Documentation](./architecture/overview.md)\n");
    content.push_str("- [Models Documentation](./models/overview.md)\n");
    content.push_str("- [Integration Documentation](./integration/overview.md)\n");
    content.push_str("- [Blockchain Implementation](../rag_knowledge_base/integration/blockchain_implementation.md)\n\n");
    
    // AI Development Guidance
    content.push_str("## AI Development Guidance\n\n");
    content.push_str("This project is built with Rust and Haskell as the primary languages. When developing new features or modifying existing ones, adhere to the following principles:\n\n");
    content.push_str("1. **Offline-First**: All features should work without an internet connection\n");
    content.push_str("2. **Performance**: Optimize for speed and resource efficiency\n");
    content.push_str("3. **Security**: Follow best practices for secure coding\n");
    content.push_str("4. **Testability**: Write unit tests for all new code\n");
    content.push_str("5. **Documentation**: Update relevant documentation\n\n");
    
    // Next Steps
    content.push_str("## Next Steps\n\n");
    for step in &result.next_steps {
        content.push_str(&format!("- {}\n", step));
    }
    content.push_str("\n");
    
    // Model Reference
    content.push_str("## Model Reference\n\n");
    content.push_str("The following models are implemented in the system:\n\n");
    
    // TODO: Add actual model references
    
    Ok(content)
}
