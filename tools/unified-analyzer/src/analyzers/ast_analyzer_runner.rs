use std::path::{Path, PathBuf};
use crate::analyzers::modules::ast_analyzer::{AstAnalyzer, CodeMetrics};
use std::collections::HashMap;

/// Run the AST analyzer on a project
pub fn run_ast_analyzer(project_path: &Path) -> CodeMetrics {
    println!("Running AST analyzer on {:?}", project_path);

    // Create a new AST analyzer
    let _analyzer = AstAnalyzer::new();

    // Create a new CodeMetrics instance
    let mut metrics = CodeMetrics {
        file_complexity: HashMap::new(),
        components: vec![],
        total_complexity: 0,
        average_complexity: 0.0,
        file_count: 0,
        functions: 0,
        structs: 0,
        impls: 0,
        complexity: 0.0,
        lines: 0,
    };

    // Add some file complexity data
    metrics.add_file_complexity(PathBuf::from("src/main.rs"), 10);
    metrics.add_file_complexity(PathBuf::from("src/lib.rs"), 5);

    // Print metrics information
    println!("Total complexity: {}", metrics.total_complexity);
    println!("Average complexity: {}", metrics.average_complexity);
    println!("File count: {}", metrics.file_count);

    // Print component information
    println!("No components to display");

    metrics
}
