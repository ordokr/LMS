rust
use std::fs;
use std::path::Path;
use tools::unified_analyzer::src::generators::documentation_generator::generate_documentation;
use tools::unified_analyzer::src::output_schema::UnifiedOutput;

#[test]
fn test_documentation_generator_integration() {
    // Create a dummy UnifiedOutput for testing
    let unified_output = UnifiedOutput {
        files: vec![],
        routes: vec![],
        components: vec![],
        api_map: vec![],
        templates: vec![],
        auth: None,
        database: None,
        business_logic: None,
        offline_plan: None,
    };

    // Define the output directory for the generated documentation
    let output_dir = "test_documentation_output";

    // Call the documentation generator
    let result = generate_documentation(unified_output, output_dir.to_string());
    assert!(result.is_ok());

    // Verify that the documentation files have been created
    let component_tree_path = Path::new(output_dir).join("component_tree.md");
    assert!(component_tree_path.exists());

    let api_route_map_path = Path::new(output_dir).join("api_route_map.md");
    assert!(api_route_map_path.exists());

    let database_schema_path = Path::new(output_dir).join("database_schema.md");
    assert!(database_schema_path.exists());

    let auth_flow_path = Path::new(output_dir).join("auth_flow.md");
    assert!(auth_flow_path.exists());

    let business_logic_path = Path::new(output_dir).join("business_logic.md");
    assert!(business_logic_path.exists());

    let offline_plan_path = Path::new(output_dir).join("offline_plan.md");
    assert!(offline_plan_path.exists());

    // Clean up the output directory
    fs::remove_dir_all(output_dir).unwrap();
}