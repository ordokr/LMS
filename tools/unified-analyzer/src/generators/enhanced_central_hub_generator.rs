use std::fs;
use std::path::Path;
use chrono::Local;

use crate::analyzers::unified_analyzer::AnalysisResult;

/// Generate enhanced central reference hub with insights from AI documentation
pub fn generate_enhanced_central_hub(result: &AnalysisResult, base_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Generating enhanced central reference hub...");

    // Ensure docs directory exists
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)?;
    }

    // Create the hub path
    let hub_path = docs_dir.join("central_reference_hub.md");

    // Generate the content
    let mut content = String::new();

    // Header
    content.push_str("# LMS Project: Central Reference Hub\n\n");
    content.push_str(&format!("_Last updated: {}_\n\n", Local::now().format("%Y-%m-%d")));

    // Project Overview
    content.push_str("## Project Overview\n\n");
    content.push_str("The LMS (Learning Management System) project is a migration and integration of Canvas LMS and Discourse forum into a unified Rust/Tauri/Leptos application with Haskell components. The project prioritizes performance, security, and offline-first capabilities.\n\n");

    // Project Status
    content.push_str("## Project Status\n\n");
    content.push_str(&format!("- **Phase**: {}\n", result.project_status.phase));
    content.push_str(&format!("- **Completion**: {:.1}%\n", result.project_status.completion_percentage));
    content.push_str(&format!("- **Last Active Area**: {}\n\n", result.project_status.last_active_area));

    // Project Structure
    content.push_str("## Project Structure\n\n");
    content.push_str("```plaintext\n");
    content.push_str("LMS/\n");
    content.push_str("├── src-tauri/         # Rust backend code\n");
    content.push_str("│   └── src/\n");
    content.push_str("│       ├── api/       # API endpoints\n");
    content.push_str("│       ├── core/      # Core business logic\n");
    content.push_str("│       ├── db/        # Database interactions\n");
    content.push_str("│       └── models/    # Data models\n");
    content.push_str("├── src/               # Frontend code\n");
    content.push_str("├── services/          # Integration services\n");
    content.push_str("│   └── integration/   # Canvas-Discourse integration\n");
    content.push_str("├── rag_knowledge_base/ # RAG documentation\n");
    content.push_str("│   └── integration/   # Integration-specific docs\n");
    content.push_str("├── docs/              # Generated documentation\n");
    content.push_str("│   ├── port/          # Port documentation\n");
    content.push_str("│   └── technical/     # Technical documentation\n");
    content.push_str("└── analysis_summary/  # Analysis results\n");
    content.push_str("    └── conflicts/     # Port conflict analysis\n");
    content.push_str("```\n\n");

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

    content.push_str("### Authentication\n");
    content.push_str("- JWT\n\n");

    content.push_str("### Blockchain\n");
    content.push_str("- Custom Rust implementation\n\n");

    // Architecture Principles
    content.push_str("## Architecture Principles\n\n");
    content.push_str("- Clean Architecture\n");
    content.push_str("- SOLID\n");
    content.push_str("- Offline-first\n\n");

    // Design Patterns
    content.push_str("## Design Patterns\n\n");
    content.push_str("- CQRS\n");
    content.push_str("- Event Sourcing\n");
    content.push_str("- Repository Pattern\n\n");

    // Implementation Metrics
    content.push_str("## Implementation Metrics\n\n");

    content.push_str("### Models\n\n");
    content.push_str(&format!("- **Implemented**: {}/{}\n", result.models.implemented, result.models.total));
    content.push_str(&format!("- **Implementation Percentage**: {:.1}%\n\n", result.models.implementation_percentage));

    content.push_str("### API Endpoints\n\n");
    content.push_str(&format!("- **Implemented**: {}/{}\n", result.api_endpoints.implemented, result.api_endpoints.total));
    content.push_str(&format!("- **Implementation Percentage**: {:.1}%\n\n", result.api_endpoints.implementation_percentage));

    content.push_str("### UI Components\n\n");
    content.push_str(&format!("- **Implemented**: {}/{}\n", result.ui_components.implemented, result.ui_components.total));
    content.push_str(&format!("- **Implementation Percentage**: {:.1}%\n\n", result.ui_components.implementation_percentage));

    // Code Quality Metrics
    content.push_str("## Code Quality Metrics\n\n");

    for (metric, value) in &result.code_quality.metrics {
        content.push_str(&format!("- **{}**: {:.1}\n", metric, value));
    }

    content.push_str(&format!("- **Test Coverage**: {:.1}%\n\n", result.tests.coverage));

    // Integration Status
    content.push_str("## Integration Status\n\n");
    content.push_str("| Integration | Source | Target | Status |\n");
    content.push_str("|-------------|--------|--------|--------|\n");
    content.push_str("| Canvas Course Management | Canvas | LMS | In Progress |\n");
    content.push_str("| Discourse Forums | Discourse | LMS | Planned |\n");
    content.push_str("| Blockchain Certification | Native | LMS | In Progress |\n\n");

    // Integration Architecture
    content.push_str("## Integration Architecture\n\n");
    content.push_str("The integration between Canvas and Discourse uses:\n\n");
    content.push_str("1. **Event-Driven Architecture**: For data synchronization\n");
    content.push_str("2. **Conflict Resolution**: Source of truth policies based on entity type\n");
    content.push_str("3. **Offline-First Capabilities**: Local storage, change tracking, sync queue\n\n");

    // Model Mapping
    content.push_str("## Model Mapping\n\n");
    content.push_str("| Canvas | Discourse | Notes |\n");
    content.push_str("|--------|-----------|-------|\n");
    content.push_str("| Course | Category | One-to-one mapping |\n");
    content.push_str("| Course Sections | Sub-categories | Optional |\n");
    content.push_str("| Discussion | Topic | One-to-one mapping |\n");
    content.push_str("| Discussion Entry | Post | One-to-one mapping |\n\n");

    // Common Code Patterns
    content.push_str("## Common Code Patterns\n\n");
    content.push_str("1. **Repository Pattern**: Data access through repository interfaces\n");
    content.push_str("2. **Type Safety**: Ensures runtime stability\n");
    content.push_str("3. **Error Handling**: Should be standardized across the codebase\n\n");

    // Implementation Recommendations
    content.push_str("## Implementation Recommendations\n\n");
    content.push_str("1. **API Responses**: Standardize response format\n");
    content.push_str("2. **Database Queries**: Use indexing for optimization\n");
    content.push_str("3. **Documentation**: Add comments to complex functions\n\n");

    // Documentation Links
    content.push_str("## Documentation Links\n\n");
    content.push_str("- [Architecture Documentation](./architecture/overview.md)\n");
    content.push_str("- [Models Documentation](./models/overview.md)\n");
    content.push_str("- [Integration Documentation](./integration/overview.md)\n");
    content.push_str("- [API Documentation](./api/reference.md)\n");
    content.push_str("- [Implementation Details](./implementation_details.md)\n");
    content.push_str("- [Testing Documentation](./tests.md)\n");
    content.push_str("- [Technical Debt Report](./technical_debt_report.md)\n");
    content.push_str("- [Synchronization Architecture](./synchronization_architecture.md)\n");
    content.push_str("- [Database Architecture](./database_architecture.md)\n\n");

    // Next Steps
    content.push_str("## Next Steps\n\n");

    for recommendation in &result.recommendations {
        content.push_str(&format!("- **{}**: {}\n", recommendation.area, recommendation.description));
    }

    content.push_str("\n");
    content.push_str("Additional recommended next steps:\n\n");
    content.push_str("1. **API**: Add authentication to remaining endpoints\n");
    content.push_str("2. **Models**: Implement remaining Canvas models\n");
    content.push_str("3. **Testing**: Increase test coverage\n");
    content.push_str("4. **Documentation**: Improve documentation\n");

    // Write to file
    fs::write(&hub_path, content)?;

    println!("Enhanced central reference hub generated at: {:?}", hub_path);

    Ok(())
}
