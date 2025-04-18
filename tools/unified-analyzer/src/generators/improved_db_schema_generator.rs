use std::path::PathBuf;
use std::fs;
use anyhow::{Result, Context};

use crate::analyzers::modules::improved_db_schema_analyzer::ImprovedDbSchemaAnalyzer;

/// Generator for creating database schema visualizations in HTML and Markdown formats.
///
/// This generator creates visualizations of the database schema, including tables,
/// columns, relationships, and other database objects.
pub struct ImprovedDbSchemaGenerator;

impl ImprovedDbSchemaGenerator {
    /// Creates a new instance of the ImprovedDbSchemaGenerator.
    pub fn new() -> Self {
        Self
    }

    /// Generates database schema visualizations in both Markdown and HTML formats.
    ///
    /// # Arguments
    /// * `base_dir` - The base directory of the project
    /// * `output_dir` - The directory where the generated files will be saved
    ///
    /// # Returns
    /// * `Ok(())` if the generation was successful
    /// * `Err(anyhow::Error)` with an error message if the generation failed
    pub fn generate(&self, base_dir: &PathBuf, output_dir: &PathBuf) -> Result<()> {
        println!("Generating improved database schema visualization...");

        // Create visualizations directory if it doesn't exist
        let vis_dir = output_dir.join("visualizations").join("db_schema");
        fs::create_dir_all(&vis_dir).context("Failed to create visualizations directory")?;

        // Create models directory if it doesn't exist
        let models_dir = output_dir.join("models");
        fs::create_dir_all(&models_dir).context("Failed to create models directory")?;

        // Initialize the improved database schema analyzer
        let mut analyzer = ImprovedDbSchemaAnalyzer::new();
        
        // Analyze the database schema
        analyzer.analyze(base_dir)?;

        // Generate database schema as Markdown with Mermaid diagram
        let mermaid_diagram = analyzer.generate_mermaid_diagram();
        let markdown = self.generate_markdown(&mermaid_diagram)?;
        let md_path = vis_dir.join("db_schema.md");
        fs::write(&md_path, markdown).context("Failed to write database schema markdown")?;

        // Generate database schema as HTML
        let html = self.generate_html(&mermaid_diagram)?;
        let html_path = vis_dir.join("db_schema.html");
        fs::write(&html_path, html).context("Failed to write database schema HTML")?;

        // Generate comprehensive database schema documentation
        let doc = analyzer.generate_markdown_documentation();
        let doc_path = models_dir.join("database_schema.md");
        fs::write(&doc_path, doc).context("Failed to write database schema documentation")?;

        // Generate JSON representation of the schema
        let json = analyzer.to_json()?;
        let json_path = vis_dir.join("db_schema.json");
        fs::write(&json_path, json).context("Failed to write database schema JSON")?;

        println!("Database schema visualization generated at:");
        println!("  - Markdown: {}", md_path.display());
        println!("  - HTML: {}", html_path.display());
        println!("  - Documentation: {}", doc_path.display());
        println!("  - JSON: {}", json_path.display());

        // Update the central reference hub
        self.update_central_reference_hub(output_dir)?;

        Ok(())
    }

    /// Generates a Markdown file with the Mermaid diagram.
    ///
    /// # Arguments
    /// * `mermaid_diagram` - The Mermaid diagram as a string
    ///
    /// # Returns
    /// * `Ok(String)` with the generated Markdown content
    /// * `Err(anyhow::Error)` with an error message if the generation failed
    fn generate_markdown(&self, mermaid_diagram: &str) -> Result<String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str("# Database Schema\n\n");
        
        // Mermaid diagram
        markdown.push_str("```mermaid\n");
        markdown.push_str(mermaid_diagram);
        markdown.push_str("```\n\n");

        // Additional information
        markdown.push_str("## Table Details\n\n");
        markdown.push_str("This diagram shows the database schema for the Ordo application, including all tables and their relationships. The schema is designed to support both offline-first functionality and integration with Canvas and Discourse.\n\n");
        
        markdown.push_str("### Key Tables\n\n");
        markdown.push_str("- **users**: Central user table that harmonizes user data from Canvas and Discourse\n");
        markdown.push_str("- **courses**: Course information with integration fields for Canvas and Discourse\n");
        markdown.push_str("- **assignments**: Assignment data that can be synchronized with Canvas\n");
        markdown.push_str("- **discussions**: Discussion topics that can be synchronized with Discourse\n");
        markdown.push_str("- **modules**: Course modules for organizing content\n");
        markdown.push_str("- **sync_status** and **sync_history**: Tables for tracking synchronization between systems\n\n");
        
        markdown.push_str("### Relationships\n\n");
        markdown.push_str("The schema includes several types of relationships:\n");
        markdown.push_str("- One-to-one relationships (e.g., users to user_profiles)\n");
        markdown.push_str("- One-to-many relationships (e.g., courses to assignments)\n");
        markdown.push_str("- Many-to-many relationships through mapping tables\n\n");
        
        markdown.push_str("### Integration Design\n\n");
        markdown.push_str("The schema is designed to support seamless integration between:\n");
        markdown.push_str("- Canvas LMS (for courses, assignments, etc.)\n");
        markdown.push_str("- Discourse forums (for discussions)\n");
        markdown.push_str("- Ordo's native offline-first functionality\n\n");
        
        markdown.push_str("This allows for a unified experience while maintaining compatibility with existing systems.\n");

        Ok(markdown)
    }

    /// Generates an HTML file with the Mermaid diagram.
    ///
    /// # Arguments
    /// * `mermaid_diagram` - The Mermaid diagram as a string
    ///
    /// # Returns
    /// * `Ok(String)` with the generated HTML content
    /// * `Err(anyhow::Error)` with an error message if the generation failed
    fn generate_html(&self, mermaid_diagram: &str) -> Result<String> {
        let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ordo Database Schema</title>
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
        <h1>Ordo Database Schema</h1>
        <div class="mermaid">
{mermaid_diagram}
        </div>
        <div class="legend">
            <h3>Legend</h3>
            <p>This diagram shows the database schema for the Ordo application. Each box represents a table in the database, and the lines represent relationships between tables.</p>
            <p>Cardinality notation:</p>
            <ul>
                <li><strong>||--||</strong>: One-to-one relationship</li>
                <li><strong>||--o{{</strong>: One-to-many relationship</li>
                <li><strong>}}o--o{{</strong>: Many-to-many relationship</li>
            </ul>
            <h3>Key Tables</h3>
            <ul>
                <li><strong>users</strong>: Central user table that harmonizes user data from Canvas and Discourse</li>
                <li><strong>courses</strong>: Course information with integration fields for Canvas and Discourse</li>
                <li><strong>assignments</strong>: Assignment data that can be synchronized with Canvas</li>
                <li><strong>discussions</strong>: Discussion topics that can be synchronized with Discourse</li>
                <li><strong>modules</strong>: Course modules for organizing content</li>
                <li><strong>sync_status</strong> and <strong>sync_history</strong>: Tables for tracking synchronization between systems</li>
            </ul>
            <h3>Integration Design</h3>
            <p>The schema is designed to support seamless integration between:</p>
            <ul>
                <li>Canvas LMS (for courses, assignments, etc.)</li>
                <li>Discourse forums (for discussions)</li>
                <li>Ordo's native offline-first functionality</li>
            </ul>
        </div>
    </div>
    <script>
        mermaid.initialize({{ startOnLoad: true, theme: 'default' }});
    </script>
</body>
</html>"#, mermaid_diagram = mermaid_diagram);

        Ok(html)
    }

    /// Updates the central reference hub to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(anyhow::Error)` with an error message if the update failed
    fn update_central_reference_hub(&self, output_dir: &PathBuf) -> Result<()> {
        let hub_path = output_dir.join("central_reference_hub.md");

        // Update the central reference hub if it exists
        if hub_path.exists() {
            let content = fs::read_to_string(&hub_path)
                .context("Failed to read central reference hub")?;

            // Check if the visualizations section already exists
            if !content.contains("## Visualizations") {
                // Add the visualizations section to the central reference hub
                let updated_content = format!("{}

## Visualizations

- [Database Schema](visualizations/db_schema/db_schema.html) - Interactive visualization of the database schema
- [Database Schema Documentation](models/database_schema.md) - Comprehensive documentation of the database schema
", content);

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .context("Failed to write updated central reference hub")?;
            } else if !content.contains("[Database Schema]") {
                // Add the database schema link to the existing visualizations section
                let updated_content = content.replace(
                    "## Visualizations\n\n", 
                    "## Visualizations\n\n- [Database Schema](visualizations/db_schema/db_schema.html) - Interactive visualization of the database schema\n- [Database Schema Documentation](models/database_schema.md) - Comprehensive documentation of the database schema\n"
                );

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .context("Failed to write updated central reference hub")?;
            }
        }

        Ok(())
    }
}
