use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

use crate::analyzers::rust_source_db_analyzer::RustSourceDbAnalyzer;

/// Generate a database schema visualization from source code using Rust
pub fn generate_rust_source_db_visualization(canvas_path: &str, discourse_path: &str, output_dir: &PathBuf) -> Result<()> {
    println!("Generating database schema visualization from source code using Rust...");

    // Create the visualizations directory if it doesn't exist
    let visualizations_dir = output_dir.join("docs").join("visualizations").join("source_db_schema");
    if !visualizations_dir.exists() {
        fs::create_dir_all(&visualizations_dir)
            .context("Failed to create visualizations directory")?;
    }

    // Create and run the source database schema analyzer
    let mut analyzer = RustSourceDbAnalyzer::new(canvas_path, discourse_path);
    analyzer.analyze()?;

    // Generate Mermaid diagram
    let mermaid_content = analyzer.generate_mermaid_diagram();

    // Generate HTML file with Mermaid diagram
    let html_content = generate_html_with_mermaid(&mermaid_content)?;

    // Write HTML file
    let html_path = visualizations_dir.join("source_db_schema.html");
    fs::write(&html_path, html_content)
        .context("Failed to write source database schema visualization HTML file")?;

    println!("Source database schema visualization generated at: {:?}", html_path);

    // Also generate a Markdown file with the Mermaid diagram
    let md_content = format!("# Source Database Schema\n\n```mermaid\n{}\n```\n", mermaid_content);
    let md_path = visualizations_dir.join("source_db_schema.md");
    fs::write(&md_path, md_content)
        .context("Failed to write source database schema visualization Markdown file")?;

    println!("Source database schema Markdown generated at: {:?}", md_path);

    // Generate a summary file with information about the schema
    let summary_content = generate_schema_summary(&analyzer);
    let summary_path = visualizations_dir.join("schema_summary.md");
    fs::write(&summary_path, summary_content)
        .context("Failed to write schema summary file")?;

    println!("Schema summary generated at: {:?}", summary_path);

    Ok(())
}

/// Generate HTML with Mermaid diagram
fn generate_html_with_mermaid(mermaid_content: &str) -> Result<String> {
    println!("Generating HTML with Mermaid diagram...");

    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Source Database Schema</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid/dist/mermaid.min.js"></script>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }}
        h1 {{
            color: #333;
            border-bottom: 1px solid #eee;
            padding-bottom: 10px;
        }}
        .mermaid {{
            margin-top: 20px;
        }}
        .legend {{
            margin-top: 20px;
            padding: 10px;
            background-color: #f9f9f9;
            border-radius: 5px;
        }}
        .legend h3 {{
            margin-top: 0;
        }}
        .source-info {{
            margin-top: 20px;
            padding: 10px;
            background-color: #f0f8ff;
            border-radius: 5px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Source Database Schema</h1>
        
        <div class="source-info">
            <h3>About This Visualization</h3>
            <p>This diagram shows the database schema extracted from the source code of Canvas and Discourse. It represents the core data models that will be implemented in the Ordo application.</p>
        </div>
        
        <div class="mermaid">
{mermaid_content}
        </div>
        
        <div class="legend">
            <h3>Legend</h3>
            <p>Each box represents a table in the database, and the lines represent relationships between tables.</p>
            <p>Cardinality notation:</p>
            <ul>
                <li><strong>||--||</strong>: One-to-one relationship</li>
                <li><strong>||--o{{</strong>: One-to-many relationship</li>
                <li><strong>}}o--o{{</strong>: Many-to-many relationship</li>
            </ul>
        </div>
    </div>
    <script>
        mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
    </script>
</body>
</html>"#, mermaid_content = mermaid_content);

    Ok(html)
}

/// Generate a summary of the schema
fn generate_schema_summary(analyzer: &RustSourceDbAnalyzer) -> String {
    println!("Generating schema summary...");

    let tables = analyzer.get_tables();
    let relationships = analyzer.get_relationships();

    let mut summary = String::from("# Database Schema Summary\n\n");
    
    // Add overview
    summary.push_str("## Overview\n\n");
    summary.push_str(&format!("- Total tables: {}\n", tables.len()));
    summary.push_str(&format!("- Total relationships: {}\n", relationships.len()));
    
    // Count tables by source
    let canvas_tables = tables.iter().filter(|t| t.source == "canvas").count();
    let discourse_tables = tables.iter().filter(|t| t.source == "discourse").count();
    summary.push_str(&format!("- Canvas tables: {}\n", canvas_tables));
    summary.push_str(&format!("- Discourse tables: {}\n", discourse_tables));
    
    // Add tables section
    summary.push_str("\n## Tables\n\n");
    
    // Canvas tables
    summary.push_str("### Canvas Tables\n\n");
    for table in tables.iter().filter(|t| t.source == "canvas") {
        summary.push_str(&format!("#### {}\n\n", table.name));
        summary.push_str("| Column | Type |\n");
        summary.push_str("|--------|------|\n");
        for column in &table.columns {
            summary.push_str(&format!("| {} | {} |\n", column.name, column.column_type));
        }
        summary.push_str("\n");
    }
    
    // Discourse tables
    summary.push_str("### Discourse Tables\n\n");
    for table in tables.iter().filter(|t| t.source == "discourse") {
        summary.push_str(&format!("#### {}\n\n", table.name));
        summary.push_str("| Column | Type |\n");
        summary.push_str("|--------|------|\n");
        for column in &table.columns {
            summary.push_str(&format!("| {} | {} |\n", column.name, column.column_type));
        }
        summary.push_str("\n");
    }
    
    // Add relationships section
    summary.push_str("\n## Relationships\n\n");
    summary.push_str("| From Table | To Table | Cardinality | Description |\n");
    summary.push_str("|------------|----------|-------------|-------------|\n");
    for rel in relationships {
        summary.push_str(&format!("| {} | {} | {} | {} |\n", 
            rel.from_table, 
            rel.to_table, 
            rel.cardinality, 
            rel.name
        ));
    }
    
    summary
}
