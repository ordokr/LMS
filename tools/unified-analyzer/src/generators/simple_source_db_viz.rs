use std::fs;
use std::path::PathBuf;
use anyhow::{Result, Context};

use crate::analyzers::simple_source_db_analyzer::SimpleSourceDbAnalyzer;

/// Generate a database schema visualization from source code
pub fn generate_simple_source_db_visualization(canvas_path: &str, discourse_path: &str, output_dir: &PathBuf) -> Result<()> {
    println!("Generating database schema visualization from source code...");

    // Create the visualizations directory if it doesn't exist
    let visualizations_dir = output_dir.join("docs").join("visualizations").join("source_db_schema");
    if !visualizations_dir.exists() {
        fs::create_dir_all(&visualizations_dir)
            .context("Failed to create visualizations directory")?;
    }

    // Create and run the source database schema analyzer
    let mut analyzer = SimpleSourceDbAnalyzer::new(canvas_path, discourse_path);
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
    </style>
</head>
<body>
    <div class="container">
        <h1>Source Database Schema</h1>
        <div class="mermaid">
{mermaid_content}
        </div>
        <div class="legend">
            <h3>Legend</h3>
            <p>This diagram shows the database schema extracted from the source code of Canvas and Discourse. Each box represents a table in the database, and the lines represent relationships between tables.</p>
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
