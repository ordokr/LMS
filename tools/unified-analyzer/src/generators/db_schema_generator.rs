use std::path::PathBuf;
use std::fs;

use crate::output_schema::{UnifiedAnalysisOutput, DatabaseTableInfo};

/// Generator for creating database schema visualizations in HTML and Markdown formats.
///
/// This generator creates visualizations of the database schema, including tables,
/// columns, relationships, and other database objects.
pub struct DbSchemaGenerator;

impl DbSchemaGenerator {
    /// Creates a new instance of the DbSchemaGenerator.
    pub fn new() -> Self {
        Self
    }

    /// Generates database schema visualizations in both Markdown and HTML formats.
    ///
    /// # Arguments
    /// * `output` - The unified analysis output containing database schema information
    /// * `output_dir` - The directory where the generated files will be saved
    ///
    /// # Returns
    /// * `Ok(())` if the generation was successful
    /// * `Err(String)` with an error message if the generation failed
    pub fn generate(&self, output: &UnifiedAnalysisOutput, output_dir: &PathBuf) -> Result<(), String> {
        println!("Generating database schema visualization...");

        // Create visualizations directory if it doesn't exist
        let vis_dir = output_dir.join("visualizations").join("db_schema");
        fs::create_dir_all(&vis_dir).map_err(|e| format!("Failed to create visualizations directory: {}", e))?;

        // Generate database schema as Markdown with Mermaid diagram
        let markdown = self.generate_markdown(output)?;
        let md_path = vis_dir.join("db_schema.md");
        fs::write(&md_path, markdown)
            .map_err(|e| format!("Failed to write database schema markdown: {}", e))?;

        // Generate database schema as HTML
        let html = self.generate_html(output)?;
        let html_path = vis_dir.join("db_schema.html");
        fs::write(&html_path, html)
            .map_err(|e| format!("Failed to write database schema HTML: {}", e))?;

        // Update the existing database documentation to include a link to the visualization
        self.update_database_documentation(output_dir)?;

        println!("Database schema visualization generated at:");
        println!("  - Markdown: {}", md_path.display());
        println!("  - HTML: {}", html_path.display());

        Ok(())
    }

    fn generate_markdown(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        let mut markdown = String::new();

        // Header
        markdown.push_str("# Database Schema Visualization\n\n");
        markdown.push_str("This document provides a visualization of the database schema, including tables, columns, relationships, and indexes.\n\n");

        // Generate ERD diagram using Mermaid
        markdown.push_str("## Entity-Relationship Diagram\n\n");
        markdown.push_str("```mermaid\nerDiagram\n");

        // Add tables and columns
        for table in &output.database.tables {
            markdown.push_str(&format!("    {} {{\n", table.name));

            for column in &table.columns {
                let pk_indicator = if column.name == "id" || column.name.ends_with("_id") { "PK " } else { "" };
                markdown.push_str(&format!("        {} {} {}\n", column.data_type, column.name, pk_indicator));
            }

            markdown.push_str("    }\n");
        }

        // Add relationships
        let relationships = self.extract_relationships(&output.database.tables);
        for rel in &relationships {
            markdown.push_str(&format!("    {} ||--o{{ {} : \"{}\"\n",
                rel.parent_table,
                rel.child_table,
                rel.relationship_type
            ));
        }

        markdown.push_str("```\n\n");

        // Table details
        markdown.push_str("## Table Details\n\n");

        for table in &output.database.tables {
            markdown.push_str(&format!("### {}\n\n", table.name));

            // Create table for columns
            markdown.push_str("| Column | Type | Constraints | Description |\n");
            markdown.push_str("|--------|------|-------------|-------------|\n");

            for column in &table.columns {
                let constraints = if column.name == "id" {
                    "PRIMARY KEY"
                } else if column.name.ends_with("_id") {
                    "FOREIGN KEY"
                } else if column.name == "created_at" || column.name == "updated_at" {
                    "NOT NULL"
                } else {
                    ""
                };

                let description = if column.name == "id" {
                    "Unique identifier"
                } else if column.name.ends_with("_id") {
                    &format!("Reference to {}", column.name.replace("_id", "s"))
                } else if column.name == "created_at" {
                    "Creation timestamp"
                } else if column.name == "updated_at" {
                    "Last update timestamp"
                } else {
                    ""
                };

                markdown.push_str(&format!("| {} | {} | {} | {} |\n",
                    column.name,
                    column.data_type,
                    constraints,
                    description
                ));
            }

            markdown.push_str("\n");
        }

        // Relationships
        markdown.push_str("## Relationships\n\n");
        markdown.push_str("| Parent Table | Child Table | Relationship Type | Foreign Key |\n");
        markdown.push_str("|-------------|------------|-------------------|-------------|\n");

        for rel in &relationships {
            markdown.push_str(&format!("| {} | {} | {} | {} |\n",
                rel.parent_table,
                rel.child_table,
                rel.relationship_type,
                rel.foreign_key
            ));
        }

        Ok(markdown)
    }

    fn generate_html(&self, output: &UnifiedAnalysisOutput) -> Result<String, String> {
        // Load template file using the embedded template directly
        // This ensures the template is always available regardless of the current working directory
        let embedded_template = include_str!("templates/db_schema_template.html");
        let template = embedded_template.to_string();

        // Generate table cards HTML
        let mut table_cards_html = String::new();
        for table in &output.database.tables {
            table_cards_html.push_str(&format!(r#"<div class="col-md-6 table-item" data-table-name="{}">
    <div class="table-card">
        <div class="table-header">
            {}
        </div>
        <div class="table-body">
            <table class="table table-hover">
                <thead>
                    <tr>
                        <th>Column</th>
                        <th>Type</th>
                        <th>Constraints</th>
                    </tr>
                </thead>
                <tbody>
"#, table.name.to_lowercase(), table.name));

            for column in &table.columns {
                let constraints = if column.name == "id" {
                    r#"<span class="column-constraint primary-key">PRIMARY KEY</span>"#
                } else if column.name.ends_with("_id") {
                    r#"<span class="column-constraint foreign-key">FOREIGN KEY</span>"#
                } else if column.name == "created_at" || column.name == "updated_at" {
                    r#"<span class="column-constraint not-null">NOT NULL</span>"#
                } else {
                    ""
                };

                table_cards_html.push_str(&format!(r#"                    <tr>
                        <td class="column-name">{}</td>
                        <td class="column-type">{}</td>
                        <td>{}</td>
                    </tr>
"#, column.name, column.data_type, constraints));
            }

            table_cards_html.push_str(r#"                </tbody>
            </table>
        </div>
    </div>
</div>
"#);
        }

        // Generate relationships HTML
        let relationships = self.extract_relationships(&output.database.tables);
        let mut relationships_html = String::new();

        relationships_html.push_str(r#"<table class="table table-striped">
    <thead>
        <tr>
            <th>Parent Table</th>
            <th>Child Table</th>
            <th>Relationship Type</th>
            <th>Foreign Key</th>
        </tr>
    </thead>
    <tbody>
"#);

        for rel in &relationships {
            relationships_html.push_str(&format!(r#"        <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
        </tr>
"#, rel.parent_table, rel.child_table, rel.relationship_type, rel.foreign_key));
        }

        relationships_html.push_str(r#"    </tbody>
</table>"#);

        // Create schema data for D3.js
        let mut tables_data = Vec::new();
        for table in &output.database.tables {
            let mut columns = Vec::new();
            for col in &table.columns {
                columns.push(serde_json::json!({
                    "name": col.name,
                    "type": col.data_type,
                    "primary_key": col.name == "id",
                    "foreign_key": col.name.ends_with("_id")
                }));
            }

            tables_data.push(serde_json::json!({
                "name": table.name,
                "columns": columns
            }));
        }

        let mut relationships_data = Vec::new();
        for rel in &relationships {
            relationships_data.push(serde_json::json!({
                "source_table": rel.parent_table,
                "target_table": rel.child_table,
                "source_column": "id",
                "target_column": rel.foreign_key
            }));
        }

        let schema_data = serde_json::json!({
            "tables": tables_data,
            "relationships": relationships_data
        });

        // Check if placeholders exist in the template
        let table_list_placeholder = "<!-- TABLE_LIST_PLACEHOLDER -->";
        let relationships_list_placeholder = "<!-- RELATIONSHIPS_LIST_PLACEHOLDER -->";
        let schema_data_placeholder = "<!-- SCHEMA_DATA_PLACEHOLDER -->";

        if !template.contains(table_list_placeholder) {
            return Err(format!("Template is missing required placeholder: {}", table_list_placeholder));
        }
        if !template.contains(relationships_list_placeholder) {
            return Err(format!("Template is missing required placeholder: {}", relationships_list_placeholder));
        }
        if !template.contains(schema_data_placeholder) {
            return Err(format!("Template is missing required placeholder: {}", schema_data_placeholder));
        }

        // Replace placeholders in template
        let html = template
            .replace(table_list_placeholder, &table_cards_html)
            .replace(relationships_list_placeholder, &relationships_html)
            .replace(schema_data_placeholder, &schema_data.to_string());

        Ok(html)
    }

}

/// Represents a relationship between two database tables.
///
/// This struct is used to store information about foreign key relationships
/// between tables in the database schema.
struct Relationship {
    /// The name of the parent table (referenced table)
    parent_table: String,
    /// The name of the child table (referencing table)
    child_table: String,
    /// The type of relationship (e.g., "has many", "belongs to")
    relationship_type: String,
    /// The name of the foreign key column in the child table
    foreign_key: String
}

impl DbSchemaGenerator {
    /// Extracts relationships between database tables based on foreign key columns.
    ///
    /// This method analyzes the table columns to identify foreign key relationships.
    /// It looks for columns with names ending in "_id" and creates relationships
    /// between the tables.
    ///
    /// # Arguments
    /// * `tables` - A slice of DatabaseTableInfo objects representing the tables in the database
    ///
    /// # Returns
    /// A vector of Relationship objects representing the relationships between tables
    fn extract_relationships(&self, tables: &[DatabaseTableInfo]) -> Vec<Relationship> {
        let mut relationships = Vec::new();

        // Extract relationships based on foreign key columns
        for table in tables {
            for column in &table.columns {
                if column.name.ends_with("_id") {
                    let parent_table = column.name.replace("_id", "");

                    // Check if the parent table exists
                    if tables.iter().any(|t| t.name.to_lowercase() == parent_table) {
                        relationships.push(Relationship {
                            parent_table: parent_table,
                            child_table: table.name.clone(),
                            relationship_type: "has many".to_string(),
                            foreign_key: column.name.clone()
                        });
                    }
                }
            }
        }

        relationships
    }

    /// Updates the existing database documentation to include a link to the visualization.
    ///
    /// # Arguments
    /// * `output_dir` - The directory where the documentation is located
    ///
    /// # Returns
    /// * `Ok(())` if the update was successful
    /// * `Err(String)` with an error message if the update failed
    fn update_database_documentation(&self, output_dir: &PathBuf) -> Result<(), String> {
        let db_doc_path = output_dir.join("database_schema.md");
        let db_arch_path = output_dir.join("database_architecture.md");

        // Update database_schema.md if it exists
        if db_doc_path.exists() {
            let content = fs::read_to_string(&db_doc_path)
                .map_err(|e| format!("Failed to read database schema documentation: {}", e))?;

            // Check if the visualization link already exists
            if content.contains("Database Schema Visualization") {
                return Ok(());
            }

            // Add the visualization link to the database documentation
            let updated_content = format!("{}

## Database Schema Visualization

For a detailed visualization of the database schema, see:

- [Database Schema (HTML)](visualizations/db_schema/db_schema.html)
- [Database Schema (Markdown)](visualizations/db_schema/db_schema.md)
", content);

            // Write the updated database documentation
            fs::write(&db_doc_path, updated_content)
                .map_err(|e| format!("Failed to write updated database schema documentation: {}", e))?;
        }

        // Update database_architecture.md if it exists
        if db_arch_path.exists() {
            let content = fs::read_to_string(&db_arch_path)
                .map_err(|e| format!("Failed to read database architecture documentation: {}", e))?;

            // Check if the visualization link already exists
            if content.contains("Database Schema Visualization") {
                return Ok(());
            }

            // Add the visualization link to the database architecture documentation
            let updated_content = format!("{}

## Database Schema Visualization

For a detailed visualization of the database schema, see:

- [Database Schema (HTML)](visualizations/db_schema/db_schema.html)
- [Database Schema (Markdown)](visualizations/db_schema/db_schema.md)
", content);

            // Write the updated database architecture documentation
            fs::write(&db_arch_path, updated_content)
                .map_err(|e| format!("Failed to write updated database architecture documentation: {}", e))?;
        }

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

- [Database Schema](visualizations/db_schema/db_schema.html)
", content);

                // Write the updated central reference hub
                fs::write(&hub_path, updated_content)
                    .map_err(|e| format!("Failed to write updated central reference hub: {}", e))?;
            } else if !content.contains("[Database Schema]") {
                // Add the database schema link to the existing visualizations section
                let updated_content = content.replace("## Visualizations\n\n", "## Visualizations\n\n- [Database Schema](visualizations/db_schema/db_schema.html)\n");

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

            // Check if the database schema link already exists
            if !content.contains("[Database Schema]") {
                // Add the database schema link to the README
                let updated_content = format!("{}
- [Database Schema](db_schema/db_schema.html)", content);

                // Write the updated README
                fs::write(&vis_readme_path, updated_content)
                    .map_err(|e| format!("Failed to write updated visualizations README: {}", e))?;
            }
        } else {
            // Create the README.md file
            let vis_readme_content = "# Visualizations\n\n- [Database Schema](db_schema/db_schema.html)\n";

            fs::write(&vis_readme_path, vis_readme_content)
                .map_err(|e| format!("Failed to write visualizations README: {}", e))?;
        }

        Ok(())
    }
}
