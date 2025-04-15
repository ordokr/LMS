use std::fs;
use std::path::Path;
use chrono::Local;

use crate::analyzers::unified_analyzer::AnalysisResult;

/// Generate summary report
pub fn generate_summary_report(result: &AnalysisResult, base_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Generating summary report...");

    // Ensure docs directory exists
    let docs_dir = base_dir.join("docs");
    if !docs_dir.exists() {
        fs::create_dir_all(&docs_dir)?;
    }

    // Create the summary report path
    let report_path = docs_dir.join("SUMMARY_REPORT.md");

    // Generate the content
    let mut content = String::new();

    // Header
    content.push_str("# LMS Project: Summary Report\n");
    content.push_str(&format!("_Generated on {}_\n\n", Local::now().format("%Y-%m-%d")));

    // Project Status
    content.push_str("## Project Status\n\n");
    content.push_str(&format!("- **Phase**: {}\n", result.project_status.phase));
    content.push_str(&format!("- **Completion**: {:.1}%\n", result.project_status.completion_percentage));
    content.push_str(&format!("- **Last Active Area**: {}\n", result.project_status.last_active_area));

    if let Some(date) = &result.project_status.estimated_completion_date {
        content.push_str(&format!("- **Estimated Completion Date**: {}\n", date.format("%Y-%m-%d")));
    }

    content.push_str("\n");

    // Implementation Metrics
    content.push_str("## Implementation Metrics\n\n");
    content.push_str("| Component | Implemented | Total | Percentage |\n");
    content.push_str("|-----------|-------------|-------|------------|\n");
    content.push_str(&format!("| Models | {} | {} | {:.1}% |\n",
        result.models.implemented,
        result.models.total,
        result.models.implementation_percentage));
    content.push_str(&format!("| API Endpoints | {} | {} | {:.1}% |\n",
        result.api_endpoints.implemented,
        result.api_endpoints.total,
        result.api_endpoints.implementation_percentage));
    content.push_str(&format!("| UI Components | {} | {} | {:.1}% |\n",
        result.ui_components.implemented,
        result.ui_components.total,
        result.ui_components.implementation_percentage));
    content.push_str(&format!("| Integration Points | {} | {} | {:.1}% |\n",
        result.integration.implemented_points,
        result.integration.total_points,
        result.integration.implementation_percentage));

    content.push_str("\n");

    // Code Quality Metrics
    content.push_str("## Code Quality Metrics\n\n");

    for (metric, value) in &result.code_quality.metrics {
        content.push_str(&format!("- **{}**: {:.1}\n", metric, value));
    }

    content.push_str(&format!("- **Test Coverage**: {:.1}%\n", result.tests.coverage));

    content.push_str("\n");

    // Testing Metrics
    content.push_str("## Testing Metrics\n\n");
    content.push_str(&format!("- **Total Tests**: {}\n", result.tests.total));
    content.push_str(&format!("- **Passing Tests**: {}\n", result.tests.passing));

    // Calculate pass rate
    let pass_rate = if result.tests.total > 0 {
        (result.tests.passing as f32 / result.tests.total as f32) * 100.0
    } else {
        0.0
    };

    content.push_str(&format!("- **Pass Rate**: {:.1}%\n", pass_rate));

    content.push_str("\n");

    // Architecture
    content.push_str("## Architecture\n\n");
    content.push_str("### Frameworks\n\n");

    for framework in &result.architecture.frameworks {
        content.push_str(&format!("- {}\n", framework));
    }

    content.push_str("\n");

    content.push_str("### Design Patterns\n\n");

    for pattern in &result.architecture.design_patterns {
        content.push_str(&format!("- {}\n", pattern));
    }

    content.push_str("\n");

    // Blockchain
    content.push_str("## Blockchain\n\n");
    content.push_str(&format!("- **Status**: {}\n", result.blockchain.implementation_status));
    content.push_str("- **Features**:\n");

    for feature in &result.blockchain.features {
        content.push_str(&format!("  - {}\n", feature));
    }

    content.push_str("\n");

    // Recommendations
    content.push_str("## Recommendations\n\n");

    for recommendation in &result.recommendations {
        content.push_str(&format!("- **{}**: {}\n", recommendation.area, recommendation.description));
    }

    content.push_str("\n");

    // Next Steps
    content.push_str("## Next Steps\n\n");
    content.push_str("1. Address high priority technical debt\n");
    content.push_str("2. Implement remaining API endpoints\n");
    content.push_str("3. Increase test coverage\n");
    content.push_str("4. Complete model implementations\n");
    content.push_str("5. Improve documentation\n");

    // Write to file
    fs::write(&report_path, content)?;

    println!("Summary report generated at: {:?}", report_path);

    Ok(())
}
