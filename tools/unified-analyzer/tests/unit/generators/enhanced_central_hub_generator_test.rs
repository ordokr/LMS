
use tempfile::TempDir;

use unified_analyzer::generators::enhanced_central_hub_generator::generate_enhanced_central_hub;
use crate::test_utils::{create_temp_dir, create_mock_analysis_result, assert_file_contains};

#[test]
fn test_generate_enhanced_central_hub() {
    // Create a temporary directory for testing
    let temp_dir: TempDir = create_temp_dir();
    let base_dir = temp_dir.path();

    // Create a mock analysis result
    let result = create_mock_analysis_result();

    // Generate the enhanced central hub
    let generate_result = generate_enhanced_central_hub(&result, base_dir);
    assert!(generate_result.is_ok(), "Failed to generate enhanced central hub: {:?}", generate_result.err());

    // Check that the file was created
    let hub_path = base_dir.join("docs").join("central_reference_hub.md");
    assert!(hub_path.exists(), "Central reference hub file was not created");

    // Check that the file contains the expected content
    assert_file_contains(&hub_path, "# LMS Project: Central Reference Hub");
    assert_file_contains(&hub_path, "## Project Overview");
    assert_file_contains(&hub_path, "## Project Status");
    assert_file_contains(&hub_path, "## Project Structure");
    assert_file_contains(&hub_path, "## Technology Stack");
    assert_file_contains(&hub_path, "## Architecture Principles");
    assert_file_contains(&hub_path, "## Design Patterns");
    assert_file_contains(&hub_path, "## Implementation Metrics");
    assert_file_contains(&hub_path, "## Code Quality Metrics");
    assert_file_contains(&hub_path, "## Integration Status");
    assert_file_contains(&hub_path, "## Integration Architecture");
    assert_file_contains(&hub_path, "## Model Mapping");
    assert_file_contains(&hub_path, "## Common Code Patterns");
    assert_file_contains(&hub_path, "## Implementation Recommendations");
    assert_file_contains(&hub_path, "## Documentation Links");
    assert_file_contains(&hub_path, "## Next Steps");
}
