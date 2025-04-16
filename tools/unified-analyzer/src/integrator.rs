use serde_json::{Value, json};

/// Integrates the results from all analyzers into a unified JSON structure
pub fn integrate_analysis_results(
    file_structure_result: String,
    ruby_rails_result: String,
    ember_result: String,
    react_result: String,
    template_result: String,
    route_result: String,
    api_result: String,
    dependency_result: String,
    auth_flow_result: String,
    offline_first_readiness_result: String,
    database_schema_result: String,
    business_logic_result: String,
    canvas_result: String,
    discourse_result: String,
) -> Value {
    // Parse each result into a JSON value
    let file_structure = parse_json_result(&file_structure_result);
    let ruby_rails = parse_json_result(&ruby_rails_result);
    let ember = parse_json_result(&ember_result);
    let react = parse_json_result(&react_result);
    let template = parse_json_result(&template_result);
    let route = parse_json_result(&route_result);
    let api = parse_json_result(&api_result);
    let dependency = parse_json_result(&dependency_result);
    let auth_flow = parse_json_result(&auth_flow_result);
    let offline_first_readiness = parse_json_result(&offline_first_readiness_result);
    let database_schema = parse_json_result(&database_schema_result);
    let business_logic = parse_json_result(&business_logic_result);
    let canvas = parse_json_result(&canvas_result);
    let discourse = parse_json_result(&discourse_result);

    // Create a unified JSON structure
    json!({
        "file_structure": file_structure,
        "ruby_rails": ruby_rails,
        "ember": ember,
        "react": react,
        "templates": template,
        "routes": route,
        "api": api,
        "dependencies": dependency,
        "auth_flow": auth_flow,
        "offline_first_readiness": offline_first_readiness,
        "database_schema": database_schema,
        "business_logic": business_logic,
        "canvas": canvas,
        "discourse": discourse,
        "metadata": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Unified analysis of Canvas and Discourse for LMS project",
        }
    })
}

/// Helper function to parse a JSON string into a Value, with error handling
fn parse_json_result(json_str: &str) -> Value {
    match serde_json::from_str(json_str) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error parsing JSON result: {}", e);
            json!({"error": format!("Failed to parse: {}", e), "raw": json_str})
        }
    }
}