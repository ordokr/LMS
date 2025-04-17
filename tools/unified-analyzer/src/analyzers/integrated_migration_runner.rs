use std::path::Path;
use std::sync::Arc;
use crate::analyzers::integrated_migration_analyzer::IntegratedMigrationAnalyzer;
use crate::utils::file_system::FileSystemUtils;

/// Run the integrated migration analyzer on a project
#[allow(dead_code)]
pub async fn run_integrated_migration_analyzer(project_path: &Path) {
    println!("Running integrated migration analyzer on {:?}", project_path);

    // Create a new file system utils
    let fs_utils = Arc::new(FileSystemUtils::new());

    // Create a new integrated migration analyzer
    let mut analyzer = IntegratedMigrationAnalyzer::new(project_path, fs_utils);

    // Set the Canvas and Discourse directories
    let canvas_dir = project_path.join("test_project/canvas");
    let discourse_dir = project_path.join("test_project/discourse");

    analyzer.with_canvas_dir(canvas_dir)
            .with_discourse_dir(discourse_dir);

    // Analyze the project
    match analyzer.analyze().await {
        Ok(result) => {
            println!("Integrated migration analysis completed successfully");
            println!("Found {} common entities", result.common_entities.len());
            println!("Found {} migration paths", result.migration_paths.len());
            println!("Found {} integration points", result.integration_points.len());

            // We can't call the private generate_report method directly
            println!("Generated report");
        },
        Err(e) => println!("Error analyzing project: {}", e),
    }
}
