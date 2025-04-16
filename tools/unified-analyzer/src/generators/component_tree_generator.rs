use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::output_schema::{UnifiedAnalysisOutput, ComponentInfo};

pub struct ComponentTreeGenerator;

impl ComponentTreeGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating component tree visualization...");

        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
        }

        // Generate component tree as Mermaid diagram
        let mermaid = self.generate_mermaid_diagram(output)?;
        let md_path = output_dir.join("component_tree.md");
        fs::write(&md_path, mermaid)
            .map_err(|e| format!("Failed to write component tree diagram: {}", e))?;

        // Generate component tree as D3.js visualization
        let html = self.generate_d3_visualization(output)?;
        let html_path = output_dir.join("component_tree.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write component tree visualization: {}", e))?;

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
            
            if let Some(description) = &component.description {
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

        // Generate HTML with D3.js visualization
        let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Component Dependency Graph</title>
    <script src="https://d3js.org/d3.v7.min.js"></script>
    <style>
        body {{
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
            background-color: #f5f5f5;
        }}
        #graph {{
            width: 100%;
            height: 800px;
            background-color: white;
            border: 1px solid #ddd;
        }}
        .node {{
            stroke: #fff;
            stroke-width: 1.5px;
        }}
        .link {{
            stroke: #999;
            stroke-opacity: 0.6;
        }}
        .tooltip {{
            position: absolute;
            background-color: rgba(0, 0, 0, 0.7);
            color: white;
            padding: 5px 10px;
            border-radius: 4px;
            font-size: 12px;
            pointer-events: none;
        }}
        h1, h2 {{
            text-align: center;
        }}
    </style>
</head>
<body>
    <h1>Component Dependency Graph</h1>
    <div id="graph"></div>
    <script>
        // Graph data
        const graph = {graph_data};
        
        // Create a force-directed graph
        const width = document.getElementById('graph').clientWidth;
        const height = document.getElementById('graph').clientHeight;
        
        // Create tooltip
        const tooltip = d3.select("body").append("div")
            .attr("class", "tooltip")
            .style("opacity", 0);
        
        // Create color scale
        const color = d3.scaleOrdinal(d3.schemeCategory10);
        
        // Create simulation
        const simulation = d3.forceSimulation(graph.nodes)
            .force("link", d3.forceLink(graph.links).id(d => d.id).distance(100))
            .force("charge", d3.forceManyBody().strength(-300))
            .force("center", d3.forceCenter(width / 2, height / 2));
        
        // Create SVG
        const svg = d3.select("#graph")
            .append("svg")
            .attr("width", width)
            .attr("height", height);
        
        // Create links
        const link = svg.append("g")
            .selectAll("line")
            .data(graph.links)
            .enter().append("line")
            .attr("class", "link")
            .attr("stroke-width", d => Math.sqrt(d.value));
        
        // Create nodes
        const node = svg.append("g")
            .selectAll("circle")
            .data(graph.nodes)
            .enter().append("circle")
            .attr("class", "node")
            .attr("r", 8)
            .attr("fill", d => color(d.group))
            .call(d3.drag()
                .on("start", dragstarted)
                .on("drag", dragged)
                .on("end", dragended));
        
        // Add labels
        const text = svg.append("g")
            .selectAll("text")
            .data(graph.nodes)
            .enter().append("text")
            .attr("dx", 12)
            .attr("dy", ".35em")
            .text(d => d.name);
        
        // Add tooltips
        node.on("mouseover", function(event, d) {{
                tooltip.transition()
                    .duration(200)
                    .style("opacity", .9);
                tooltip.html(`<strong>${{d.name}}</strong><br/>File: ${{d.file}}`)
                    .style("left", (event.pageX + 10) + "px")
                    .style("top", (event.pageY - 28) + "px");
            }})
            .on("mouseout", function(d) {{
                tooltip.transition()
                    .duration(500)
                    .style("opacity", 0);
            }});
        
        // Update positions on tick
        simulation.on("tick", () => {{
            link
                .attr("x1", d => d.source.x)
                .attr("y1", d => d.source.y)
                .attr("x2", d => d.target.x)
                .attr("y2", d => d.target.y);
            
            node
                .attr("cx", d => d.x)
                .attr("cy", d => d.y);
            
            text
                .attr("x", d => d.x)
                .attr("y", d => d.y);
        }});
        
        // Drag functions
        function dragstarted(event, d) {{
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
        }}
        
        function dragged(event, d) {{
            d.fx = event.x;
            d.fy = event.y;
        }}
        
        function dragended(event, d) {{
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
        }}
    </script>
</body>
</html>"#);

        Ok(html)
    }

    fn sanitize_id(&self, name: &str) -> String {
        // Replace characters that might cause issues in Mermaid IDs
        name.replace(" ", "_")
            .replace("-", "_")
            .replace(".", "_")
            .replace("/", "_")
    }

    fn determine_group(&self, component: &ComponentInfo) -> i32 {
        // Determine component group based on its characteristics
        // This is a simple implementation that can be enhanced
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
}
