use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use regex::Regex;
use walkdir::WalkDir;

/// Represents a database table
#[derive(Debug, Clone)]
pub struct DbTable {
    pub name: String,
    pub columns: Vec<DbColumn>,
    pub source: String, // "canvas" or "discourse"
    pub file_path: String,
}

/// Represents a database column
#[derive(Debug, Clone)]
pub struct DbColumn {
    pub name: String,
    pub column_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<ForeignKey>,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

/// Represents a foreign key relationship
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub references_table: String,
    pub references_column: String,
}

/// Analyzer for extracting database schema from source code files (not from built databases)
///
/// IMPORTANT: This analyzer performs static analysis of source code files only.
/// It does NOT connect to any database, import any data, or perform any data migration.
/// The purpose is to understand the schema structure from the original source code
/// to inform the design of the new application's schema.
pub struct SourceDbSchemaAnalyzer {
    canvas_path: PathBuf,
    discourse_path: PathBuf,
    tables: HashMap<String, DbTable>,
    source_code_only: bool, // Always true - this analyzer only works with source code
}

impl SourceDbSchemaAnalyzer {
    pub fn new(canvas_path: &str, discourse_path: &str) -> Self {
        Self {
            canvas_path: PathBuf::from(canvas_path),
            discourse_path: PathBuf::from(discourse_path),
            tables: HashMap::new(),
            source_code_only: true,
        }
    }

    /// Analyze the source code to extract database schema (no database connection required)
    ///
    /// This method performs static analysis of Ruby source files to extract schema information.
    /// It does NOT connect to any database, import any data, or perform any data migration.
    /// The extracted schema is used only to inform the design of the new application's schema.
    ///
    /// This is part of the source-to-source migration process, not a data migration process.
    pub fn analyze(&mut self) -> Result<()> {
        println!("Analyzing Canvas source code for database schema through static source code analysis...");
        println!("NOTE: This does NOT connect to any database or perform data migration.");
        self.analyze_canvas()?;

        println!("Analyzing Discourse source code for database schema through static source code analysis...");
        println!("NOTE: This does NOT connect to any database or perform data migration.");
        self.analyze_discourse()?;

        Ok(())
    }

    /// Analyze Canvas source code for database schema
    fn analyze_canvas(&mut self) -> Result<()> {
        // Look for migration files in the Canvas codebase
        let migration_dir = self.canvas_path.join("db").join("migrate");

        if !migration_dir.exists() {
            println!("Canvas migration directory not found: {:?}", migration_dir);
            return Ok(());
        }

        // Process migration files
        for entry in WalkDir::new(&migration_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-Ruby files
            if path.extension().and_then(|ext| ext.to_str()) != Some("rb") {
                continue;
            }

            // Read the file content
            if let Ok(content) = fs::read_to_string(path) {
                // Extract table definitions
                self.extract_canvas_tables(path, &content)?;
            }
        }

        // Look for model files
        let model_dir = self.canvas_path.join("app").join("models");

        if !model_dir.exists() {
            println!("Canvas model directory not found: {:?}", model_dir);
            return Ok(());
        }

        // Process model files
        for entry in WalkDir::new(&model_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-Ruby files
            if path.extension().and_then(|ext| ext.to_str()) != Some("rb") {
                continue;
            }

            // Read the file content
            if let Ok(content) = fs::read_to_string(path) {
                // Extract model associations
                self.extract_canvas_associations(path, &content)?;
            }
        }

        Ok(())
    }

    /// Extract table definitions from Canvas migration files
    fn extract_canvas_tables(&mut self, path: &Path, content: &str) -> Result<()> {
        // Regex to match create_table statements
        let create_table_regex = Regex::new(r"create_table\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match column definitions
        let column_regex = Regex::new(r"t\.([\w_]+)\s+[\"':]([\w_]+)[\"']").unwrap();

        // Find all create_table statements
        for cap in create_table_regex.captures_iter(content) {
            if let Some(table_name) = cap.get(1) {
                let table_name = table_name.as_str().to_string();

                // Create a new table
                let mut table = DbTable {
                    name: table_name.clone(),
                    columns: Vec::new(),
                    source: "canvas".to_string(),
                    file_path: path.to_string_lossy().to_string(),
                };

                // Find all column definitions
                for col_cap in column_regex.captures_iter(content) {
                    if let (Some(col_type), Some(col_name)) = (col_cap.get(1), col_cap.get(2)) {
                        let col_type = col_type.as_str().to_string();
                        let col_name = col_name.as_str().to_string();

                        // Create a new column
                        let column = DbColumn {
                            name: col_name,
                            column_type: col_type,
                            nullable: true, // Default to true, we'll update this later if needed
                            primary_key: col_type == "primary_key",
                            foreign_key: None, // We'll update this later
                            default_value: None,
                            description: None,
                        };

                        // Add the column to the table
                        table.columns.push(column);
                    }
                }

                // Add the table to the tables map
                self.tables.insert(table_name, table);
            }
        }

        Ok(())
    }

    /// Extract model associations from Canvas model files
    fn extract_canvas_associations(&mut self, path: &Path, content: &str) -> Result<()> {
        // Regex to match belongs_to associations
        let belongs_to_regex = Regex::new(r"belongs_to\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match has_many associations
        let has_many_regex = Regex::new(r"has_many\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match class name
        let class_regex = Regex::new(r"class\s+([\w:]+)\s+<").unwrap();

        // Extract class name
        let class_name = if let Some(cap) = class_regex.captures(content) {
            if let Some(name) = cap.get(1) {
                name.as_str().to_string()
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        // Convert class name to table name (e.g., User -> users)
        let table_name = class_name.to_lowercase() + "s";

        // Find all belongs_to associations
        for cap in belongs_to_regex.captures_iter(content) {
            if let Some(assoc_name) = cap.get(1) {
                let assoc_name = assoc_name.as_str().to_string();

                // Convert association name to foreign key (e.g., user -> user_id)
                let foreign_key = assoc_name.clone() + "_id";

                // Convert association name to table name (e.g., user -> users)
                let references_table = assoc_name.to_lowercase() + "s";

                // Update the table if it exists
                if let Some(table) = self.tables.get_mut(&table_name) {
                    // Find the foreign key column
                    for column in &mut table.columns {
                        if column.name == foreign_key {
                            // Update the foreign key
                            column.foreign_key = Some(ForeignKey {
                                references_table,
                                references_column: "id".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Analyze Discourse source code for database schema
    fn analyze_discourse(&mut self) -> Result<()> {
        // Look for migration files in the Discourse codebase
        let migration_dir = self.discourse_path.join("db").join("migrate");

        if !migration_dir.exists() {
            println!("Discourse migration directory not found: {:?}", migration_dir);
            return Ok(());
        }

        // Process migration files
        for entry in WalkDir::new(&migration_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-Ruby files
            if path.extension().and_then(|ext| ext.to_str()) != Some("rb") {
                continue;
            }

            // Read the file content
            if let Ok(content) = fs::read_to_string(path) {
                // Extract table definitions
                self.extract_discourse_tables(path, &content)?;
            }
        }

        // Look for model files
        let model_dir = self.discourse_path.join("app").join("models");

        if !model_dir.exists() {
            println!("Discourse model directory not found: {:?}", model_dir);
            return Ok(());
        }

        // Process model files
        for entry in WalkDir::new(&model_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-Ruby files
            if path.extension().and_then(|ext| ext.to_str()) != Some("rb") {
                continue;
            }

            // Read the file content
            if let Ok(content) = fs::read_to_string(path) {
                // Extract model associations
                self.extract_discourse_associations(path, &content)?;
            }
        }

        Ok(())
    }

    /// Extract table definitions from Discourse migration files
    fn extract_discourse_tables(&mut self, path: &Path, content: &str) -> Result<()> {
        // Regex to match create_table statements
        let create_table_regex = Regex::new(r"create_table\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match column definitions
        let column_regex = Regex::new(r"t\.([\w_]+)\s+[\"':]([\w_]+)[\"']").unwrap();

        // Find all create_table statements
        for cap in create_table_regex.captures_iter(content) {
            if let Some(table_name) = cap.get(1) {
                let table_name = table_name.as_str().to_string();

                // Create a new table
                let mut table = DbTable {
                    name: table_name.clone(),
                    columns: Vec::new(),
                    source: "discourse".to_string(),
                    file_path: path.to_string_lossy().to_string(),
                };

                // Find all column definitions
                for col_cap in column_regex.captures_iter(content) {
                    if let (Some(col_type), Some(col_name)) = (col_cap.get(1), col_cap.get(2)) {
                        let col_type = col_type.as_str().to_string();
                        let col_name = col_name.as_str().to_string();

                        // Create a new column
                        let column = DbColumn {
                            name: col_name,
                            column_type: col_type,
                            nullable: true, // Default to true, we'll update this later if needed
                            primary_key: col_type == "primary_key",
                            foreign_key: None, // We'll update this later
                            default_value: None,
                            description: None,
                        };

                        // Add the column to the table
                        table.columns.push(column);
                    }
                }

                // Add the table to the tables map
                self.tables.insert(table_name, table);
            }
        }

        Ok(())
    }

    /// Extract model associations from Discourse model files
    fn extract_discourse_associations(&mut self, path: &Path, content: &str) -> Result<()> {
        // Regex to match belongs_to associations
        let belongs_to_regex = Regex::new(r"belongs_to\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match has_many associations
        let has_many_regex = Regex::new(r"has_many\s+[\"':]([\w_]+)[\"']").unwrap();

        // Regex to match class name
        let class_regex = Regex::new(r"class\s+([\w:]+)\s+<").unwrap();

        // Extract class name
        let class_name = if let Some(cap) = class_regex.captures(content) {
            if let Some(name) = cap.get(1) {
                name.as_str().to_string()
            } else {
                return Ok(());
            }
        } else {
            return Ok(());
        };

        // Convert class name to table name (e.g., User -> users)
        let table_name = class_name.to_lowercase() + "s";

        // Find all belongs_to associations
        for cap in belongs_to_regex.captures_iter(content) {
            if let Some(assoc_name) = cap.get(1) {
                let assoc_name = assoc_name.as_str().to_string();

                // Convert association name to foreign key (e.g., user -> user_id)
                let foreign_key = assoc_name.clone() + "_id";

                // Convert association name to table name (e.g., user -> users)
                let references_table = assoc_name.to_lowercase() + "s";

                // Update the table if it exists
                if let Some(table) = self.tables.get_mut(&table_name) {
                    // Find the foreign key column
                    for column in &mut table.columns {
                        if column.name == foreign_key {
                            // Update the foreign key
                            column.foreign_key = Some(ForeignKey {
                                references_table,
                                references_column: "id".to_string(),
                            });
                            break;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate a Mermaid diagram from the extracted schema (based on source code analysis)
    pub fn generate_mermaid_diagram(&self) -> String {
        println!("Generating Mermaid diagram from schema extracted from source code...");

        let mut mermaid = String::from("erDiagram\n");

        // Add each table to the diagram
        for (_, table) in &self.tables {
            mermaid.push_str(&format!("    {} {{\n", table.name));

            for column in &table.columns {
                mermaid.push_str(&format!("        {} {}\n", column.column_type, column.name));
            }

            mermaid.push_str("    }\n");
        }

        // Add relationships
        for (_, table) in &self.tables {
            for column in &table.columns {
                if let Some(fk) = &column.foreign_key {
                    mermaid.push_str(&format!("    {} {}--{} {} : \"{}\"\n",
                        table.name,
                        "1",
                        "*",
                        fk.references_table,
                        column.name
                    ));
                }
            }
        }

        mermaid
    }

    /// Get the list of tables
    pub fn get_tables(&self) -> Vec<&DbTable> {
        self.tables.values().collect()
    }

    /// Get a table by name
    pub fn get_table(&self, name: &str) -> Option<&DbTable> {
        self.tables.get(name)
    }
}
