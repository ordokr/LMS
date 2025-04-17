use std::path::Path;
use crate::analyzers::project_structure::ProjectStructure;

/// Run the project structure analyzer on a project
pub fn run_project_structure_analyzer(project_path: &Path) {
    println!("Running project structure analyzer on {:?}", project_path);

    // Create a new project structure analyzer
    let analyzer = ProjectStructure::new(project_path);

    // We can't call the private analyze method directly
    // So we'll just use the public methods

    // Print project structure information
    println!("File count: {}", analyzer.get_file_count());
    println!("Directory count: {}", analyzer.get_directory_count());

    // Get files by extension
    let rust_files = analyzer.get_files_by_extension("rs");
    println!("Rust files: {}", rust_files.len());
    for file in rust_files {
        println!("  {:?}", file);
    }
}
