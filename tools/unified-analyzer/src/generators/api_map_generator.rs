use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};

use crate::output_schema::UnifiedAnalysisOutput;

/// Generator for creating API map visualizations in HTML and Markdown formats.
///
/// This generator creates visualizations of the API endpoints in the application,
/// including their paths, methods, controllers, and relationships.
pub struct ApiMapGenerator;

/// Represents an API endpoint.
///
/// This struct is used to store information about an API endpoint,
/// including its path, HTTP method, controller, description, authentication requirements,
/// and request parameters.
#[derive(Clone)]
struct ApiEndpoint {
    /// The path of the API endpoint
    path: String,
    /// The HTTP method of the API endpoint (GET, POST, PUT, DELETE, etc.)
    http_method: String,
    /// The controller that handles the API endpoint
    controller: Option<String>,
    /// A description of the API endpoint
    description: Option<String>,
    /// Whether authentication is required to access the API endpoint
    auth_required: bool,
    /// The request parameters for the API endpoint
    request_params: Vec<String>,
}

impl ApiMapGenerator {
    /// Creates a new instance of the ApiMapGenerator.
    pub fn new() -> Self {
        Self
    }

    /// Generates API map visualizations in both Markdown and HTML formats.
    ///
    /// # Arguments
    /// * `output` - The unified analysis output containing API endpoint information
    /// * `output_dir` - The directory where the generated files will be saved
    ///
    /// # Returns
    /// * `Ok(())` if the generation was successful
    /// * `Err(String)` with an error message if the generation failed
    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating API map visualization...");

        // Create visualizations directory if it doesn't exist
        let vis_dir = output_dir.join("visualizations").join("api_map");
        fs::create_dir_all(&vis_dir).map_err(|e| format!("Failed to create visualizations directory: {}", e))?;

        // Generate API map as Markdown
        let markdown = self.generate_markdown(output)?;
        let md_path = vis_dir.join("api_map.md");
        fs::write(&md_path, markdown)
            .map_err(|e| format!("Failed to write API map markdown: {}", e))?;

        // Generate API map as HTML
        let html = self.generate_html(output)?;
        let html_path = vis_dir.join("api_map.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write API map HTML: {}", e))?;

        // Update the existing API documentation to include a link to the visualization
        self.update_api_documentation(output_dir)?;

        println!("API map visualization generated at:");
        println!("  - Markdown: {}", md_path.display());
        println!("  - HTML: {}", html_path.display());

        Ok(())
    }

    fn generate_markdown(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str("# API Map\n\n");
        markdown.push_str("This document provides a comprehensive map of all API endpoints in the application.\n\n");

        // Group endpoints by category
        let mut categories = HashMap::new();

        // Get the endpoints from the output
        let endpoints = &output.api.endpoints;

        // If there are no endpoints, create a default category with a message
        if endpoints.is_empty() {
            categories.insert("No API Endpoints Found".to_string(), Vec::new());
            markdown.push_str("## No API Endpoints Found\n\n");
            markdown.push_str("No API endpoints were found in the application. This could be because:\n\n");
            markdown.push_str("1. The application does not have any API endpoints yet\n");
            markdown.push_str("2. The analyzer was not able to detect the API endpoints\n");
            markdown.push_str("3. The API endpoints are defined in a way that the analyzer does not understand\n\n");
            markdown.push_str("Please check the application code to verify if there are any API endpoints.\n\n");
            return Ok(markdown);
        } else {
            for endpoint in endpoints {
                let category = endpoint.controller.clone().unwrap_or_else(|| "Uncategorized".to_string());
                categories.entry(category).or_insert_with(Vec::new).push(endpoint);
            }
        }

        // Generate table of contents
        markdown.push_str("## Table of Contents\n\n");

        for category in categories.keys() {
            markdown.push_str(&format!("- [{}](#{})\n", category, self.to_anchor(category)));
        }

        markdown.push_str("\n");

        // Generate API documentation by category
        for (category, endpoints) in &categories {
            markdown.push_str(&format!("## {}\n\n", category));

            if endpoints.is_empty() {
                markdown.push_str("No API endpoints found in this category.\n\n");
                continue;
            }

            // Create a table for each HTTP method
            let mut methods = HashMap::new();

            for endpoint in endpoints {
                methods.entry(&endpoint.http_method).or_insert_with(Vec::new).push(endpoint);
            }

            for (method, method_endpoints) in &methods {
                markdown.push_str(&format!("### {} Endpoints\n\n", method));

                // Create table
                markdown.push_str("| Path | Description | Auth Required | Parameters |\n");
                markdown.push_str("|------|-------------|--------------|------------|\n");

                for endpoint in method_endpoints {
                    let auth_required = if endpoint.auth_required { "Yes" } else { "No" };
                    let params = endpoint.request_params.join(", ");

                    markdown.push_str(&format!("| `{}` | {} | {} | {} |\n",
                        endpoint.path,
                        endpoint.description.clone().unwrap_or_default(),
                        auth_required,
                        params
                    ));
                }

                markdown.push_str("\n");
            }
        }

        // Generate Mermaid diagram
        markdown.push_str("## API Flow Diagram\n\n");

        if endpoints.is_empty() {
            markdown.push_str("No API endpoints found to generate a flow diagram.\n\n");
        } else {
            markdown.push_str("```mermaid\ngraph LR\n");

            // Add client node
            markdown.push_str("    Client[Client]\n");

            // Add API endpoints
            for endpoint in endpoints {
                let node_id = self.path_to_id(&endpoint.path);
                let http_method = &endpoint.http_method;

                markdown.push_str(&format!("    {}[\"{} {}\"]\n", node_id, http_method, endpoint.path));

                // Connect client to endpoint
                markdown.push_str(&format!("    Client --> {}\n", node_id));

                // Connect endpoint to controller if available
                if let Some(controller) = &endpoint.controller {
                    let controller_id = self.sanitize_id(controller);

                    markdown.push_str(&format!("    {}[\"{}\"]\n", controller_id, controller));
                    markdown.push_str(&format!("    {} --> {}\n", node_id, controller_id));
                }
            }

            markdown.push_str("```\n");
        }

        Ok(markdown)
    }

    fn generate_html(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        // Group endpoints by category
        let mut categories = HashMap::new();

        // Get the endpoints from the output
        let endpoints = &output.api.endpoints;

        // If there are no endpoints, create a default category with a message
        if endpoints.is_empty() {
            categories.insert("No API Endpoints Found".to_string(), Vec::new());
        } else {
            // Create a vector to store all ApiEndpoint structs
            let mut api_endpoints = Vec::new();

            // First, create all ApiEndpoint structs
            for endpoint in endpoints {
                api_endpoints.push(ApiEndpoint {
                    path: endpoint.path.clone(),
                    http_method: endpoint.http_method.clone(),
                    controller: endpoint.controller.clone(),
                    description: endpoint.description.clone(),
                    auth_required: endpoint.auth_required,
                    request_params: endpoint.request_params.clone(),
                });
            }

            // Then, add them to categories
            for api_endpoint in &api_endpoints {
                // Get the category (controller or "Uncategorized")
                let category = api_endpoint.controller.clone().unwrap_or_else(|| "Uncategorized".to_string());

                // Add to categories
                categories.entry(category).or_insert_with(Vec::new).push(api_endpoint.clone());
            }
        }

        // Generate HTML
        let mut html = String::new();

        // Load template file using the embedded template directly
        // This ensures the template is always available regardless of the current working directory
        let embedded_template = include_str!("templates/api_map_template.html");
        let template = embedded_template.to_string();

        // Find the method filters placeholder
        let method_filters_placeholder = "<!-- METHOD_FILTERS_PLACEHOLDER -->";
        let categories_placeholder = "<!-- CATEGORIES_PLACEHOLDER -->";

        // Check if placeholders exist in the template
        if !template.contains(method_filters_placeholder) {
            return Err(format!("Template is missing required placeholder: {}", method_filters_placeholder));
        }
        if !template.contains(categories_placeholder) {
            return Err(format!("Template is missing required placeholder: {}", categories_placeholder));
        }

        // Split the template at the placeholders
        let parts: Vec<&str> = template.split(method_filters_placeholder).collect();
        if parts.len() != 2 {
            return Err("Invalid template format: method filters placeholder appears multiple times".to_string());
        }

        let header = parts[0];
        let rest = parts[1];

        let parts: Vec<&str> = rest.split(categories_placeholder).collect();
        if parts.len() != 2 {
            return Err("Invalid template format: categories placeholder appears multiple times".to_string());
        }

        let middle = parts[0];
        let footer = parts[1];

        // Start building the HTML
        html.push_str(header);

        // Add method filters
        let mut methods = HashSet::new();

        // If there are no endpoints, add a default method
        if endpoints.is_empty() {
            methods.insert("GET".to_string());
        } else {
            // Extract methods from endpoints
            for endpoint in endpoints {
                methods.insert(endpoint.http_method.clone());
            }
        }

        for method in methods {
            html.push_str(&format!(r#"<div class="filter-option" data-method="{}">{}</div>
"#,
                method.to_lowercase(), method));
        }

        html.push_str(middle);

        // Generate API documentation by category
        let mut categories_html = String::new();

        if categories.is_empty() || (categories.len() == 1 && categories.contains_key("No API Endpoints Found")) {
            // No endpoints found, show a message
            categories_html.push_str(r#"<div class="category" id="no-api-endpoints-found">
    <h2>No API Endpoints Found</h2>
    <p>No API endpoints were found in the application. This could be because:</p>
    <ul>
        <li>The application does not have any API endpoints yet</li>
        <li>The analyzer was not able to detect the API endpoints</li>
        <li>The API endpoints are defined in a way that the analyzer does not understand</li>
    </ul>
    <p>Please check the application code to verify if there are any API endpoints.</p>
</div>
"#);
        } else {
            // Generate HTML for each category
            for (category, endpoints) in &categories {
                categories_html.push_str(&format!(r#"<div class="category" id="{}">
    <h2>{}</h2>
"#, self.to_anchor(category), category));

                if endpoints.is_empty() {
                    categories_html.push_str(r#"    <p>No API endpoints found in this category.</p>
</div>
"#);
                    continue;
                }

                // Group by HTTP method
                let mut methods = HashMap::new();

                for endpoint in endpoints {
                    methods.entry(&endpoint.http_method).or_insert_with(Vec::new).push(endpoint);
                }

                for (method, method_endpoints) in &methods {
                    let method_class = format!("method-{}", method.to_lowercase());

                    categories_html.push_str(&format!(r#"    <div class="method-group">
        <div class="method-header {}">
            {} Endpoints
        </div>
"#, method_class, method));

                    for endpoint in method_endpoints {
                        let auth_class = if endpoint.auth_required { "auth-required" } else { "auth-optional" };
                        let auth_text = if endpoint.auth_required { "Auth Required" } else { "Auth Optional" };
                        let auth_badge = format!(r#"<span class="badge {}">{}</span>"#, auth_class, auth_text);

                        categories_html.push_str(&format!(r#"        <div class="endpoint" data-method="{}" data-auth="{}">
            <div class="endpoint-header">
                <span class="endpoint-path">{}</span>
                {}
            </div>
            <div class="endpoint-details">
                <p>{}</p>
"#,
                            method.to_lowercase(),
                            if endpoint.auth_required { "required" } else { "optional" },
                            endpoint.path,
                            auth_badge,
                            endpoint.description.clone().unwrap_or_default()));

                        if !endpoint.request_params.is_empty() {
                            categories_html.push_str(r#"                <h4>Parameters</h4>
                <table class="params-table">
                    <thead>
                        <tr>
                            <th>Name</th>
                        </tr>
                    </thead>
                    <tbody>
"#);

                            for param in &endpoint.request_params {
                                categories_html.push_str(&format!(r#"                        <tr>
                            <td>{}</td>
                        </tr>
"#, param));
                            }

                            categories_html.push_str(r#"                    </tbody>
                </table>
"#);
                        }

                        if let Some(controller) = &endpoint.controller {
                            categories_html.push_str(&format!(r#"                <h4>Controller</h4>
                <p>{}</p>
"#, controller));
                        }

                        categories_html.push_str(r#"            </div>
        </div>
"#);
                    }

                    categories_html.push_str(r#"    </div>
"#);
                }

                categories_html.push_str(r#"</div>
"#);
            }
        }

        html.push_str(&categories_html);
        html.push_str(footer);

        Ok(html)
    }

    /// Converts a text string to an anchor ID suitable for use in HTML/Markdown.
    ///
    /// # Arguments
    /// * `text` - The text to convert to an anchor
    ///
    /// # Returns
    /// A string with lowercase letters, spaces replaced with hyphens,
    /// '&' replaced with 'and', and only alphanumeric characters and hyphens.
    fn to_anchor(&self, text: &str) -> String {
        text.to_lowercase()
            .replace(" ", "-")
            .replace("&", "and")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }

    /// Converts an API path to an ID suitable for use in HTML/JavaScript.
    ///
    /// # Arguments
    /// * `path` - The API path to convert
    ///
    /// # Returns
    /// A string with slashes replaced with underscores, and special characters removed.
    fn path_to_id(&self, path: &str) -> String {
        path.replace("/", "_")
            .replace("{", "")
            .replace("}", "")
            .replace(":", "")
            .replace(".", "_")
            .replace("-", "_")
    }

    /// Sanitizes a text string to be used as an ID in HTML/JavaScript.
    ///
    /// # Arguments
    /// * `text` - The text to sanitize
    ///
    /// # Returns
    /// A string with spaces replaced with underscores and only alphanumeric characters and underscores.
    fn sanitize_id(&self, text: &str) -> String {
        text.replace(" ", "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_')
            .collect()
    }

    /// Updates the existing API documentation to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_api_documentation(&self, output_dir: &PathBuf) -> Result<(), String> {
        let api_doc_path = output_dir.join("api_documentation.md");

        // Check if the API documentation file exists
        if !api_doc_path.exists() {
            return Ok(());
        }

        // Read the existing API documentation
        let content = fs::read_to_string(&api_doc_path)
            .map_err(|e| format!("Failed to read API documentation: {}", e))?;

        // Check if the visualization link already exists
        if content.contains("API Map Visualization") {
            return Ok(());
        }

        // Add the visualization link to the API documentation
        let updated_content = format!("{}

## API Map Visualization

For a detailed visualization of the API endpoints, see:

- [API Map (HTML)](visualizations/api_map/api_map.html)
- [API Map (Markdown)](visualizations/api_map/api_map.md)
", content);

        // Write the updated API documentation
        fs::write(&api_doc_path, updated_content)
            .map_err(|e| format!("Failed to write updated API documentation: {}", e))?;

        // Update the central reference hub
        self.update_central_reference_hub(output_dir)?;

        Ok(())
    }

    /// Updates the central reference hub to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_central_reference_hub(&self, output_dir: &PathBuf) -> Result<(), String> {
        let hub_path = output_dir.join("central_reference_hub.md");
        let index_path = output_dir.join("index.md");

        // Update the central reference hub if it exists
        if hub_path.exists() {
            let content = fs::read_to_string(&hub_path)
                .map_err(|e| format!("Failed to read central reference hub: {}", e))?;

            // Check if the visualizations section already exists
            if !content.contains("## Visualizations") {
                // Add the visualizations section to the central reference hub
                let updated_content = format!("{}

## Visualizations

- [API Map](visualizations/api_map/api_map.html)
", content);

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            } else if !content.contains("[API Map]") {
                // Add the API map link to the existing visualizations section
                let updated_content = content.replace("## Visualizations\n\n", "## Visualizations\n\n- [API Map](visualizations/api_map/api_map.html)\n");

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            }
        }

        // Update the index file if it exists
        if index_path.exists() {
            let content = fs::read_to_string(&index_path)
                .map_err(|e| format!("Failed to read index file: {}", e))?;

            // Check if the visualizations section already exists
            if !content.contains("- [Visualizations]") {
                // Add the visualizations link to the table of contents
                let updated_content = content.replace("## Table of Contents\n\n", "## Table of Contents\n\n- [Visualizations](visualizations/README.md)\n");

                // Write the updated index file
                fs::write(&index_path, updated_content)
                    .map_err(|e| format!("Failed to write updated index file: {}", e))?;

                // Create a README.md file in the visualizations directory
                let vis_readme_path = output_dir.join("visualizations").join("README.md");
                let vis_readme_content = "# Visualizations\n\n- [API Map](api_map/api_map.html)\n";

                fs::write(&vis_readme_path, vis_readme_content)
                    .map_err(|e| format!("Failed to write visualizations README: {}", e))?;
            }
        }

        Ok(())
    }
}
