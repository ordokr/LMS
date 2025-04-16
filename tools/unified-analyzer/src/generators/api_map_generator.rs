use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

use crate::output_schema::{UnifiedAnalysisOutput, ApiEndpointInfo};

pub struct ApiMapGenerator;

impl ApiMapGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating API map visualization...");

        // Create output directory if it doesn't exist
        if !output_dir.exists() {
            fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
        }

        // Generate API map as Markdown
        let markdown = self.generate_markdown(output)?;
        let md_path = output_dir.join("api_map.md");
        fs::write(&md_path, markdown)
            .map_err(|e| format!("Failed to write API map markdown: {}", e))?;

        // Generate API map as HTML
        let html = self.generate_html(output)?;
        let html_path = output_dir.join("api_map.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write API map HTML: {}", e))?;

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
        
        for endpoint in &output.api_map {
            let category = endpoint.category.clone().unwrap_or_else(|| "Uncategorized".to_string());
            categories.entry(category).or_insert_with(Vec::new).push(endpoint);
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
            
            // Create a table for each HTTP method
            let mut methods = HashMap::new();
            
            for endpoint in endpoints {
                methods.entry(&endpoint.method).or_insert_with(Vec::new).push(endpoint);
            }
            
            for (method, method_endpoints) in &methods {
                markdown.push_str(&format!("### {} Endpoints\n\n", method));
                
                // Create table
                markdown.push_str("| Path | Description | Auth Required | Parameters |\n");
                markdown.push_str("|------|-------------|--------------|------------|\n");
                
                for endpoint in method_endpoints {
                    let auth_required = if endpoint.auth_required { "Yes" } else { "No" };
                    let params = endpoint.parameters.join(", ");
                    
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
        markdown.push_str("```mermaid\ngraph LR\n");
        
        // Add client node
        markdown.push_str("    Client[Client]\n");
        
        // Add API endpoints
        for endpoint in &output.api_map {
            let node_id = self.path_to_id(&endpoint.path);
            let method = &endpoint.method;
            
            markdown.push_str(&format!("    {}[\"{} {}\"]\n", node_id, method, endpoint.path));
            
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

        Ok(markdown)
    }

    fn generate_html(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        // Group endpoints by category
        let mut categories = HashMap::new();
        
        for endpoint in &output.api_map {
            let category = endpoint.category.clone().unwrap_or_else(|| "Uncategorized".to_string());
            categories.entry(category).or_insert_with(Vec::new).push(endpoint);
        }

        // Generate HTML
        let mut html = String::new();
        
        html.push_str(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>API Map</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        h1, h2, h3 {
            color: #2c3e50;
        }
        .category {
            margin-bottom: 30px;
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 15px;
            background-color: #f9f9f9;
        }
        .method-group {
            margin-bottom: 20px;
        }
        .method-header {
            background-color: #2c3e50;
            color: white;
            padding: 8px 15px;
            border-radius: 4px;
            display: inline-block;
            margin-bottom: 10px;
        }
        .endpoint {
            border: 1px solid #ddd;
            border-radius: 4px;
            margin-bottom: 10px;
            background-color: white;
        }
        .endpoint-header {
            padding: 10px 15px;
            background-color: #f5f5f5;
            border-bottom: 1px solid #ddd;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .endpoint-path {
            font-family: monospace;
            font-weight: bold;
        }
        .endpoint-details {
            padding: 15px;
        }
        .params-table {
            width: 100%;
            border-collapse: collapse;
        }
        .params-table th, .params-table td {
            border: 1px solid #ddd;
            padding: 8px;
            text-align: left;
        }
        .params-table th {
            background-color: #f5f5f5;
        }
        .method-get { background-color: #61affe; }
        .method-post { background-color: #49cc90; }
        .method-put { background-color: #fca130; }
        .method-delete { background-color: #f93e3e; }
        .method-patch { background-color: #50e3c2; }
        .badge {
            display: inline-block;
            padding: 3px 7px;
            border-radius: 3px;
            font-size: 12px;
            font-weight: bold;
            color: white;
        }
        .toc {
            background-color: #f5f5f5;
            padding: 15px;
            border-radius: 4px;
            margin-bottom: 20px;
        }
        .toc ul {
            list-style-type: none;
            padding-left: 20px;
        }
        .auth-required {
            background-color: #e74c3c;
        }
        .auth-optional {
            background-color: #3498db;
        }
    </style>
</head>
<body>
    <h1>API Map</h1>
    <p>This document provides a comprehensive map of all API endpoints in the application.</p>
    
    <div class="toc">
        <h2>Table of Contents</h2>
        <ul>"#);
        
        // Generate table of contents
        for category in categories.keys() {
            html.push_str(&format!(r#"
            <li><a href="#{}">{}</a></li>"#, 
                self.to_anchor(category), category));
        }
        
        html.push_str(r#"
        </ul>
    </div>"#);
        
        // Generate API documentation by category
        for (category, endpoints) in &categories {
            html.push_str(&format!(r#"
    <div class="category" id="{}">
        <h2>{}</h2>"#, self.to_anchor(category), category));
            
            // Group by HTTP method
            let mut methods = HashMap::new();
            
            for endpoint in endpoints {
                methods.entry(&endpoint.method).or_insert_with(Vec::new).push(endpoint);
            }
            
            for (method, method_endpoints) in &methods {
                let method_class = format!("method-{}", method.to_lowercase());
                
                html.push_str(&format!(r#"
        <div class="method-group">
            <div class="method-header {}">
                {} Endpoints
            </div>"#, method_class, method));
                
                for endpoint in method_endpoints {
                    let auth_badge = if endpoint.auth_required {
                        r#"<span class="badge auth-required">Auth Required</span>"#
                    } else {
                        r#"<span class="badge auth-optional">Auth Optional</span>"#
                    };
                    
                    html.push_str(&format!(r#"
            <div class="endpoint">
                <div class="endpoint-header">
                    <span class="endpoint-path">{}</span>
                    {}
                </div>
                <div class="endpoint-details">
                    <p>{}</p>"#, 
                        endpoint.path, 
                        auth_badge,
                        endpoint.description.clone().unwrap_or_default()));
                    
                    if !endpoint.parameters.is_empty() {
                        html.push_str(r#"
                    <h4>Parameters</h4>
                    <table class="params-table">
                        <thead>
                            <tr>
                                <th>Name</th>
                            </tr>
                        </thead>
                        <tbody>"#);
                        
                        for param in &endpoint.parameters {
                            html.push_str(&format!(r#"
                            <tr>
                                <td>{}</td>
                            </tr>"#, param));
                        }
                        
                        html.push_str(r#"
                        </tbody>
                    </table>"#);
                    }
                    
                    html.push_str(r#"
                </div>
            </div>"#);
                }
                
                html.push_str(r#"
        </div>"#);
            }
            
            html.push_str(r#"
    </div>"#);
        }
        
        html.push_str(r#"
</body>
</html>"#);

        Ok(html)
    }

    fn to_anchor(&self, text: &str) -> String {
        text.to_lowercase()
            .replace(" ", "-")
            .replace("&", "and")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }

    fn path_to_id(&self, path: &str) -> String {
        path.replace("/", "_")
            .replace(":", "_")
            .replace("{", "_")
            .replace("}", "_")
            .replace("-", "_")
            .replace(".", "_")
    }

    fn sanitize_id(&self, name: &str) -> String {
        name.replace(" ", "_")
            .replace("-", "_")
            .replace(".", "_")
            .replace("/", "_")
            .replace("::", "_")
    }
}
