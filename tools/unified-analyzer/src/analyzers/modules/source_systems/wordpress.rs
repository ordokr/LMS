use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use regex::Regex;

use crate::analyzers::modules::helix_db_integration::{
    HelixDbTable, HelixDbField, HelixDbIndex, HelixDbRelationship, SourceSystem, SourceSystemType
};

/// WordPress CMS source system
pub struct WordPressSourceSystem {
    /// Whether to use caching
    use_cache: bool,
}

impl WordPressSourceSystem {
    /// Create a new WordPress source system
    pub fn new() -> Self {
        Self {
            use_cache: true,
        }
    }

    /// Create a new WordPress source system with caching disabled
    pub fn new_without_cache() -> Self {
        Self {
            use_cache: false,
        }
    }

    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Parse a WordPress schema file
    fn parse_wp_schema(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // WordPress schema parsing using regex
        let create_table_regex = Regex::new(r#"CREATE TABLE\s+(?:\$table_prefix\s*\.\s*)?[`'"]([\w_]+)[`'"]"#).unwrap();
        let field_regex = Regex::new(r#"\s*[`'"]([\w_]+)[`'"]\s+([^,\n]+)(?:\s+NOT NULL)?(?:\s+DEFAULT\s+([^,\n]+))?(?:\s+AUTO_INCREMENT)?"#).unwrap();
        let primary_key_regex = Regex::new(r#"PRIMARY KEY\s+\(\s*[`'"]([\w_]+)[`'"]\s*\)"#).unwrap();
        let unique_key_regex = Regex::new(r#"UNIQUE KEY\s+[`'"]([\w_]+)[`'"]\s+\(\s*[`'"]([\w_]+)[`'"]\s*(?:,\s*[`'"]([\w_]+)[`'"]\s*)*\)"#).unwrap();
        let key_regex = Regex::new(r#"KEY\s+[`'"]([\w_]+)[`'"]\s+\(\s*[`'"]([\w_]+)[`'"]\s*(?:,\s*[`'"]([\w_]+)[`'"]\s*)*\)"#).unwrap();

        // Extract tables
        let mut current_table: Option<HelixDbTable> = None;
        let mut in_table_block = false;

        for line in content.lines() {
            // Check for table definition
            if let Some(captures) = create_table_regex.captures(line) {
                // If we were already processing a table, add it to our list
                if let Some(table) = current_table.take() {
                    tables.push(table);
                }

                // Start a new table
                let table_name = captures[1].to_string();
                current_table = Some(HelixDbTable {
                    name: table_name,
                    fields: Vec::new(),
                    indexes: Vec::new(),
                    relationships: Vec::new(),
                    source: self.get_name(),
                });

                in_table_block = true;
            }

            // Check for field definition
            if in_table_block {
                if let Some(captures) = field_regex.captures(line) {
                    if let Some(table) = current_table.as_mut() {
                        let field_name = captures[1].to_string();
                        let field_type = captures[2].to_string();

                        // Parse field properties
                        let nullable = !line.contains("NOT NULL");
                        let default = captures.get(3).map(|m| m.as_str().to_string());
                        let primary_key = false; // We'll set this later when we find the PRIMARY KEY definition
                        let unique = false; // We'll set this later when we find the UNIQUE KEY definition

                        // Add the field to the table
                        table.fields.push(HelixDbField {
                            name: field_name,
                            field_type,
                            nullable,
                            default,
                            primary_key,
                            unique,
                        });
                    }
                }

                // Check for primary key definition
                if let Some(captures) = primary_key_regex.captures(line) {
                    if let Some(table) = current_table.as_mut() {
                        let primary_key_field = captures[1].to_string();

                        // Update the primary key flag for the field
                        for field in &mut table.fields {
                            if field.name == primary_key_field {
                                field.primary_key = true;
                                break;
                            }
                        }
                    }
                }

                // Check for unique key definition
                if let Some(captures) = unique_key_regex.captures(line) {
                    if let Some(table) = current_table.as_mut() {
                        let key_name = captures[1].to_string();

                        // Parse fields
                        let mut fields = Vec::new();
                        for i in 2..captures.len() {
                            if let Some(field) = captures.get(i) {
                                fields.push(field.as_str().to_string());
                            }
                        }

                        // Update the unique flag for the fields
                        for field in &mut table.fields {
                            if fields.contains(&field.name) {
                                field.unique = true;
                            }
                        }

                        // Add the index to the table
                        table.indexes.push(HelixDbIndex {
                            name: key_name,
                            fields,
                            unique: true,
                        });
                    }
                }

                // Check for regular key definition
                if let Some(captures) = key_regex.captures(line) {
                    if let Some(table) = current_table.as_mut() {
                        let key_name = captures[1].to_string();

                        // Parse fields
                        let mut fields = Vec::new();
                        for i in 2..captures.len() {
                            if let Some(field) = captures.get(i) {
                                fields.push(field.as_str().to_string());
                            }
                        }

                        // Add the index to the table
                        table.indexes.push(HelixDbIndex {
                            name: key_name,
                            fields,
                            unique: false,
                        });
                    }
                }

                // Check if we're ending the table block
                if line.contains(");") {
                    in_table_block = false;
                }
            }
        }

        // Don't forget to add the last table if we were processing one
        if let Some(table) = current_table {
            tables.push(table);
        }

        Ok(tables)
    }

    /// Parse a WordPress model file
    fn parse_wp_model(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // WordPress model parsing using regex
        let class_regex = Regex::new(r#"class\s+([A-Za-z0-9_]+)\s+extends\s+([A-Za-z0-9_]+)"#).unwrap();
        let table_name_regex = Regex::new(r#"\$this->table\s*=\s*['"](\w+)['"]"#).unwrap();
        let primary_key_regex = Regex::new(r#"\$this->primary_key\s*=\s*['"](\w+)['"]"#).unwrap();

        // Extract class name and table name
        let mut class_name = String::new();
        let mut table_name = String::new();
        let mut primary_key = String::new();

        for line in content.lines() {
            // Check for class definition
            if let Some(captures) = class_regex.captures(line) {
                class_name = captures[1].to_string();
            }

            // Check for table name
            if let Some(captures) = table_name_regex.captures(line) {
                table_name = captures[1].to_string();
            }

            // Check for primary key
            if let Some(captures) = primary_key_regex.captures(line) {
                primary_key = captures[1].to_string();
            }
        }

        // If we found a class and table name, create a table
        if !class_name.is_empty() && !table_name.is_empty() {
            let mut table = HelixDbTable {
                name: table_name,
                fields: Vec::new(),
                indexes: Vec::new(),
                relationships: Vec::new(),
                source: self.get_name(),
            };

            // Add the primary key field if we found one
            if !primary_key.is_empty() {
                table.fields.push(HelixDbField {
                    name: primary_key,
                    field_type: "bigint(20)".to_string(),
                    nullable: false,
                    default: None,
                    primary_key: true,
                    unique: true,
                });
            }

            tables.push(table);
        }

        Ok(tables)
    }

    /// Walk a directory recursively and call a function on each file
    fn walk_directory<F>(&self, dir: &Path, mut callback: F) -> Result<()>
    where
        F: FnMut(&Path),
    {
        if dir.is_dir() {
            // First collect all files to avoid recursion issues
            let mut all_files = Vec::new();
            let mut dirs_to_process = vec![dir.to_path_buf()];

            while let Some(current_dir) = dirs_to_process.pop() {
                for entry in fs::read_dir(&current_dir)? {
                    let entry = entry?;
                    let path = entry.path();

                    if path.is_dir() {
                        dirs_to_process.push(path);
                    } else {
                        all_files.push(path);
                    }
                }
            }

            // Now process all files
            for file in all_files {
                callback(&file);
            }
        }

        Ok(())
    }
}

impl SourceSystem for WordPressSourceSystem {
    fn get_type(&self) -> SourceSystemType {
        SourceSystemType::WordPress
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>> {
        println!("Extracting database schema from WordPress codebase at: {}", path.display());

        let mut tables = Vec::new();

        // Look for wp-admin/includes/schema.php
        let schema_path = path.join("wp-admin").join("includes").join("schema.php");
        if schema_path.exists() {
            println!("Found WordPress schema.php at: {}", schema_path.display());
            if let Ok(content) = fs::read_to_string(&schema_path) {
                tables.extend(self.parse_wp_schema(&content)?);
            }
        }

        // Look for wp-includes/wp-db.php
        let db_path = path.join("wp-includes").join("wp-db.php");
        if db_path.exists() {
            println!("Found WordPress wp-db.php at: {}", db_path.display());
            if let Ok(content) = fs::read_to_string(&db_path) {
                tables.extend(self.parse_wp_schema(&content)?);
            }
        }

        // Look for model files in wp-includes
        let includes_dir = path.join("wp-includes");
        if includes_dir.exists() {
            println!("Found WordPress includes directory at: {}", includes_dir.display());
            self.walk_directory(&includes_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "php" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_wp_model(&content) {
                                tables.extend(model_tables);
                            }
                        }
                    }
                }
            })?;
        }

        // Look for plugin model files
        let plugins_dir = path.join("wp-content").join("plugins");
        if plugins_dir.exists() {
            println!("Found WordPress plugins directory at: {}", plugins_dir.display());
            self.walk_directory(&plugins_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "php" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_wp_model(&content) {
                                tables.extend(model_tables);
                            }
                        }
                    }
                }
            })?;
        }

        Ok(tables)
    }
}

use std::collections::HashMap;
