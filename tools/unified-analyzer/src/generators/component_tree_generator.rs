use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};

use crate::output_schema::{UnifiedAnalysisOutput, ComponentInfo};

/// Generator for creating component tree visualizations in HTML and Markdown formats.
///
/// This generator creates visualizations of the component relationships in the application,
/// showing how components depend on each other and their hierarchical structure.
pub struct ComponentTreeGenerator;

impl ComponentTreeGenerator {
    /// Creates a new instance of the ComponentTreeGenerator.
    pub fn new() -> Self {
        Self
    }

    /// Generates component tree visualizations in both Markdown and HTML formats.
    ///
    /// # Arguments
    /// * `output` - The unified analysis output containing component information
    /// * `output_dir` - The directory where the generated files will be saved
    ///
    /// # Returns
    /// * `Ok(())` if the generation was successful
    /// * `Err(String)` with an error message if the generation failed
    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating component tree visualization...");

        // Create visualizations directory if it doesn't exist
        let vis_dir = output_dir.join("visualizations").join("component_tree");
        fs::create_dir_all(&vis_dir).map_err(|e| format!("Failed to create visualizations directory: {}", e))?;

        // Generate component tree as Mermaid diagram
        let mermaid = self.generate_mermaid_diagram(output)?;
        let md_path = vis_dir.join("component_tree.md");
        fs::write(&md_path, mermaid)
            .map_err(|e| format!("Failed to write component tree diagram: {}", e))?;

        // Generate component tree as D3.js visualization
        let html = self.generate_d3_visualization(output)?;
        let html_path = vis_dir.join("component_tree.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write component tree visualization: {}", e))?;

        // Update the existing architecture documentation to include a link to the visualization
        self.update_architecture_documentation(output_dir)?;

        println!("Component tree visualization generated at:");
        println!("  - Markdown: {}", md_path.display());
        println!("  - HTML: {}", html_path.display());

        Ok(())
    }

    fn generate_mermaid_diagram(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str("# Component Tree Visualization\n\n");
        markdown.push_str("This diagram shows the component hierarchy and dependencies in the application.\n\n");

        // Create a graph of component dependencies
        let mut graph = HashMap::new();
        let mut all_components = HashSet::new();

        // Build the graph
        for (name, component) in &output.components {
            let mut dependencies = HashSet::new();

            // Add explicit dependencies
            for dep in &component.dependencies {
                dependencies.insert(dep.clone());
                all_components.insert(dep.clone());
            }

            // Add the component itself
            all_components.insert(name.clone());

            // Store in graph
            graph.insert(name.clone(), dependencies);
        }

        // Generate Mermaid flowchart
        markdown.push_str("```mermaid\ngraph TD\n");

        // Add nodes
        for component in &all_components {
            markdown.push_str(&format!("    {}[\"{}\"]\n", self.sanitize_id(component), component));
        }

        // Add edges
        for (component, dependencies) in &graph {
            for dep in dependencies {
                markdown.push_str(&format!("    {} --> {}\n",
                    self.sanitize_id(dep),
                    self.sanitize_id(component)
                ));
            }
        }

        markdown.push_str("```\n\n");

        // Add component details
        markdown.push_str("## Component Details\n\n");

        for (name, component) in &output.components {
            markdown.push_str(&format!("### {}\n\n", name));

            let description = "Component description";
            if !description.is_empty() {
                markdown.push_str(&format!("**Description**: {}\n\n", description));
            }

            markdown.push_str(&format!("**File**: `{}`\n\n", component.file_path));

            if !component.dependencies.is_empty() {
                markdown.push_str("**Dependencies**:\n");
                for dep in &component.dependencies {
                    markdown.push_str(&format!("- {}\n", dep));
                }
                markdown.push_str("\n");
            }
        }

        Ok(markdown)
    }

    fn generate_d3_visualization(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        // Create a JSON representation of the component graph
        let mut nodes = Vec::new();
        let mut links = Vec::new();
        let mut node_map = HashMap::new();
        let mut index = 0;

        // Create nodes
        for (name, component) in &output.components {
            node_map.insert(name.clone(), index);

            nodes.push(serde_json::json!({
                "id": index,
                "name": name,
                "group": self.determine_group(component),
                "file": component.file_path
            }));

            index += 1;
        }

        // Create links
        for (name, component) in &output.components {
            let source_index = *node_map.get(name).unwrap();

            for dep in &component.dependencies {
                if let Some(target_index) = node_map.get(dep) {
                    links.push(serde_json::json!({
                        "source": *target_index,
                        "target": source_index,
                        "value": 1
                    }));
                }
            }
        }

        // Create the graph data
        let graph_data = serde_json::json!({
            "nodes": nodes,
            "links": links
        });

        // Load template file using the embedded template directly
        // This ensures the template is always available regardless of the current working directory
        let embedded_template = include_str!("templates/component_tree_template.html");
        let template = embedded_template.to_string();

        // Check if placeholder exists in the template
        let placeholder = "<!-- GRAPH_DATA_PLACEHOLDER -->";
        if !template.contains(placeholder) {
            return Err(format!("Template is missing required placeholder: {}", placeholder));
        }

        // Replace the graph data placeholder
        let html = template.replace(placeholder, &graph_data.to_string());

        Ok(html)
    }

    /// Sanitizes a component name to be used as an ID in Mermaid diagrams.
    ///
    /// # Arguments
    /// * `name` - The component name to sanitize
    ///
    /// # Returns
    /// A string with spaces, hyphens, dots, and slashes replaced with underscores.
    fn sanitize_id(&self, name: &str) -> String {
        name.replace(" ", "_")
            .replace("-", "_")
            .replace(".", "_")
            .replace("/", "_")
    }

    /// Determines the group/category of a component based on its file path.
    ///
    /// # Arguments
    /// * `component` - The component information
    ///
    /// # Returns
    /// An integer representing the group:
    /// * 1 - UI Components (contains "ui" or "components")
    /// * 2 - Data Models (contains "model" or "data")
    /// * 3 - Services (contains "service" or "api")
    /// * 4 - Utilities (contains "util" or "helper")
    /// * 0 - Other (any other location)
    fn determine_group(&self, component: &ComponentInfo) -> i32 {
        if component.file_path.contains("ui") || component.file_path.contains("components") {
            1 // UI components
        } else if component.file_path.contains("model") || component.file_path.contains("data") {
            2 // Data models
        } else if component.file_path.contains("service") || component.file_path.contains("api") {
            3 // Services
        } else if component.file_path.contains("util") || component.file_path.contains("helper") {
            4 // Utilities
        } else {
            0 // Other
        }
    }

    /// Updates the existing architecture documentation to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_architecture_documentation(&self, output_dir: &PathBuf) -> Result<(), String> {
        let arch_doc_path = output_dir.join("architecture_overview.md");

        // Check if the architecture documentation file exists
        if !arch_doc_path.exists() {
            return Ok(());
        }

        // Read the existing architecture documentation
        let content = fs::read_to_string(&arch_doc_path)
            .map_err(|e| format!("Failed to read architecture documentation: {}", e))?;

        // Check if the visualization link already exists
        if content.contains("Component Tree Visualization") {
            return Ok(());
        }

        // Add the visualization link to the architecture documentation
        let updated_content = format!("{}

## Component Tree Visualization

For a detailed visualization of the component hierarchy and dependencies, see:

- [Component Tree (HTML)](visualizations/component_tree/component_tree.html)
- [Component Tree (Markdown)](visualizations/component_tree/component_tree.md)
", content);

        // Write the updated architecture documentation
        fs::write(&arch_doc_path, updated_content)
            .map_err(|e| format!("Failed to write updated architecture documentation: {}", e))?;

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

        // Update the central reference hub if it exists
        if hub_path.exists() {
            let content = fs::read_to_string(&hub_path)
                .map_err(|e| format!("Failed to read central reference hub: {}", e))?;

            // Check if the visualizations section already exists
            if !content.contains("## Visualizations") {
                // Add the visualizations section to the central reference hub
                let updated_content = format!("{}

## Visualizations

- [Component Tree](visualizations/component_tree/component_tree.html)
", content);

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            } else if !content.contains("[Component Tree]") {
                // Add the component tree link to the existing visualizations section
                let updated_content = content.replace("## Visualizations\n\n", "## Visualizations\n\n- [Component Tree](visualizations/component_tree/component_tree.html)\n");

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            }
        }

        // Update the visualizations README.md file
        let vis_readme_path = output_dir.join("visualizations").join("README.md");
        if vis_readme_path.exists() {
            let content = fs::read_to_string(&vis_readme_path)
                .map_err(|e| format!("Failed to read visualizations README: {}", e))?;

            // Check if the component tree link already exists
            if !content.contains("[Component Tree]") {
                // Add the component tree link to the README
                let updated_content = format!("{}
- [Component Tree](component_tree/component_tree.html)", content);

                // Write the updated README
                fs::write(&vis_readme_path, updated_content)
                    .map_err(|e| format!("Failed to write updated visualizations README: {}", e))?;
            }
        } else {
            // Create the README.md file
            let vis_readme_content = "# Visualizations\n\n- [Component Tree](component_tree/component_tree.html)\n";

            fs::write(&vis_readme_path, vis_readme_content)
                .map_err(|e| format!("Failed to write visualizations README: {}", e))?;
        }

        Ok(())
    }
}
