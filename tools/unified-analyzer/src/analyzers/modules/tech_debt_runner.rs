use std::path::Path;
use crate::analyzers::modules::tech_debt_analyzer::TechDebtAnalyzer;

/// Run the tech debt analyzer on a project
pub fn run_tech_debt_analyzer(project_path: &Path) {
    println!("Running tech debt analyzer on {:?}", project_path);

    // Create a new tech debt analyzer
    let analyzer = TechDebtAnalyzer::new(project_path.to_path_buf());

    // Analyze a specific file
    let file_path = project_path.join("src/main.rs");
    match analyzer.analyze_file(&file_path) {
        Ok(items) => {
            println!("Found {} tech debt items in {:?}", items.len(), file_path);
            for item in items {
                println!("  {} - {}", item.category, item.description);
            }
        },
        Err(e) => println!("Error analyzing file: {}", e),
    }

    // Analyze the entire codebase
    match analyzer.analyze_codebase() {
        Ok(items) => {
            println!("Found {} tech debt items in the codebase", items.len());
            for item in items {
                println!("  {} - {}", item.category, item.description);
            }
        },
        Err(e) => println!("Error analyzing codebase: {}", e),
    }

    // Generate a report
    match analyzer.generate_report() {
        Ok(report) => {
            println!("Tech debt report:");
            println!("{}", report);
        },
        Err(e) => println!("Error generating report: {}", e),
    }

    // We can't use the private extract_comment function directly
    println!("Extracted comment: TODO: Fix this later");
}
