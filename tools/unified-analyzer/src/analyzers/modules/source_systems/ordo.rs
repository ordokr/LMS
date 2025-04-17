use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use regex::Regex;

use crate::analyzers::modules::helix_db_integration::{
    HelixDbTable, HelixDbField, HelixDbIndex, HelixDbRelationship, SourceSystem, SourceSystemType
};

/// Ordo LMS source system
pub struct OrdoSourceSystem {
    /// Whether to use caching
    use_cache: bool,
}

impl OrdoSourceSystem {
    /// Create a new Ordo source system
    pub fn new() -> Self {
        Self {
            use_cache: true,
        }
    }

    /// Create a new Ordo source system with caching disabled
    pub fn new_without_cache() -> Self {
        Self {
            use_cache: false,
        }
    }

    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Parse a Rust model file
    fn parse_rust_model(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();
        let mut current_struct: Option<HelixDbTable> = None;
        let mut in_struct_block = false;
        let mut struct_block_content = String::new();
        let mut brace_count = 0;

        // More robust parsing using line-by-line analysis
        let lines: Vec<&str> = content.lines().collect();

        // Struct regex patterns
        let struct_regex = Regex::new(r"struct\s+([A-Za-z0-9_]+)").unwrap();
        let field_regex = Regex::new(r"\s*(?:pub\s+)?([a-z_]+):\s+([A-Za-z0-9<>:,\s]+),?").unwrap();

        // Derive attribute regex patterns
        let _derive_regex = Regex::new(r"#\[derive\(([^\)]+)\)\]").unwrap();
        let table_attr_regex = Regex::new(r#"#\[table\(name\s*=\s*["']([^"']+)["']\)\]"#).unwrap();
        let primary_key_attr_regex = Regex::new(r"#\[primary_key\]").unwrap();
        let column_attr_regex = Regex::new(r"#\[column\(([^\)]+)\)\]").unwrap();

        // Relationship regex patterns
        let belongs_to_regex = Regex::new(r"#\[belongs_to\(([^\)]+)\)\]").unwrap();
        let has_many_regex = Regex::new(r"#\[has_many\(([^\)]+)\)\]").unwrap();
        let has_one_regex = Regex::new(r"#\[has_one\(([^\)]+)\)\]").unwrap();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];

            // Check for struct definition
            if let Some(captures) = struct_regex.captures(line) {
                // If we were already processing a struct, add it to our list
                if let Some(table) = current_struct.take() {
                    tables.push(table);
                }

                // Start a new struct
                let struct_name = captures[1].to_string();

                // Check for table name in attributes
                let mut table_name = struct_name.clone();
                if i > 0 {
                    let prev_line = lines[i - 1];
                    if let Some(table_captures) = table_attr_regex.captures(prev_line) {
                        table_name = table_captures[1].to_string();
                    }
                }

                current_struct = Some(HelixDbTable {
                    name: table_name,
                    fields: Vec::new(),
                    indexes: Vec::new(),
                    relationships: Vec::new(),
                    source: self.get_name(),
                });

                in_struct_block = true;
                struct_block_content = line.to_string();

                // Count opening braces
                brace_count = 0;
                for c in line.chars() {
                    if c == '{' {
                        brace_count += 1;
                    } else if c == '}' {
                        brace_count -= 1;
                    }
                }
            }
            // If we're in a struct block, collect the content
            else if in_struct_block {
                struct_block_content.push_str("\n");
                struct_block_content.push_str(line);

                // Count braces to determine when the struct ends
                for c in line.chars() {
                    if c == '{' {
                        brace_count += 1;
                    } else if c == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            in_struct_block = false;
                        }
                    }
                }

                // Check for field definitions
                if let Some(captures) = field_regex.captures(line) {
                    if let Some(table) = current_struct.as_mut() {
                        let field_name = captures[1].to_string();
                        let field_type = captures[2].to_string().trim().to_string();

                        // Default field properties
                        let nullable = field_type.contains("Option<");
                        let mut default = None;
                        let mut primary_key = field_name == "id";
                        let mut unique = false;

                        // Check for primary key attribute
                        if i > 0 && primary_key_attr_regex.is_match(lines[i - 1]) {
                            primary_key = true;
                        }

                        // Check for column attributes
                        if i > 0 {
                            let prev_line = lines[i - 1];
                            if let Some(col_captures) = column_attr_regex.captures(prev_line) {
                                let col_attrs = col_captures[1].to_string();

                                // Check for unique constraint
                                if col_attrs.contains("unique = true") {
                                    unique = true;
                                }

                                // Check for default value
                                let default_regex = Regex::new(r#"default\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                                if let Some(default_captures) = default_regex.captures(&col_attrs) {
                                    default = Some(default_captures[1].to_string());
                                }
                            }
                        }

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

                // Check for relationship attributes
                if i > 0 {
                    let prev_line = lines[i - 1];

                    // Check for belongs_to relationship
                    if let Some(rel_captures) = belongs_to_regex.captures(prev_line) {
                        if let Some(table) = current_struct.as_mut() {
                            let rel_attrs = rel_captures[1].to_string();
                            let foreign_table_regex = Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

                            let foreign_table = if let Some(ft_captures) = foreign_table_regex.captures(&rel_attrs) {
                                ft_captures[1].to_string()
                            } else {
                                continue;
                            };

                            let foreign_key = if let Some(fk_captures) = foreign_key_regex.captures(&rel_attrs) {
                                fk_captures[1].to_string()
                            } else {
                                format!("{}_id", foreign_table.to_lowercase())
                            };

                            table.relationships.push(HelixDbRelationship {
                                relationship_type: "belongs_to".to_string(),
                                foreign_table,
                                local_field: foreign_key,
                                foreign_field: "id".to_string(),
                            });
                        }
                    }

                    // Check for has_many relationship
                    if let Some(rel_captures) = has_many_regex.captures(prev_line) {
                        if let Some(table) = current_struct.as_mut() {
                            let rel_attrs = rel_captures[1].to_string();
                            let foreign_table_regex = Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

                            let foreign_table = if let Some(ft_captures) = foreign_table_regex.captures(&rel_attrs) {
                                ft_captures[1].to_string()
                            } else {
                                continue;
                            };

                            let foreign_key = if let Some(fk_captures) = foreign_key_regex.captures(&rel_attrs) {
                                fk_captures[1].to_string()
                            } else {
                                format!("{}_id", table.name.to_lowercase())
                            };

                            table.relationships.push(HelixDbRelationship {
                                relationship_type: "has_many".to_string(),
                                foreign_table,
                                local_field: "id".to_string(),
                                foreign_field: foreign_key,
                            });
                        }
                    }

                    // Check for has_one relationship
                    if let Some(rel_captures) = has_one_regex.captures(prev_line) {
                        if let Some(table) = current_struct.as_mut() {
                            let rel_attrs = rel_captures[1].to_string();
                            let foreign_table_regex = Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

                            let foreign_table = if let Some(ft_captures) = foreign_table_regex.captures(&rel_attrs) {
                                ft_captures[1].to_string()
                            } else {
                                continue;
                            };

                            let foreign_key = if let Some(fk_captures) = foreign_key_regex.captures(&rel_attrs) {
                                fk_captures[1].to_string()
                            } else {
                                format!("{}_id", table.name.to_lowercase())
                            };

                            table.relationships.push(HelixDbRelationship {
                                relationship_type: "has_one".to_string(),
                                foreign_table,
                                local_field: "id".to_string(),
                                foreign_field: foreign_key,
                            });
                        }
                    }
                }
            }

            i += 1;
        }

        // Don't forget to add the last struct if we were processing one
        if let Some(table) = current_struct {
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

impl SourceSystem for OrdoSourceSystem {
    fn get_type(&self) -> SourceSystemType {
        SourceSystemType::Ordo
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>> {
        println!("Extracting database schema from Ordo codebase at: {}", path.display());

        let mut tables = Vec::new();

        // Look for Rust model files
        let src_dir = path.join("src");
        if src_dir.exists() {
            println!("Found Ordo src directory at: {}", src_dir.display());
            self.walk_directory(&src_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "rs" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_rust_model(&content) {
                                tables.extend(model_tables);
                            }
                        }
                    }
                }
            })?;
        }

        // Look for Tauri src model files
        let tauri_src_dir = path.join("src-tauri").join("src");
        if tauri_src_dir.exists() {
            println!("Found Ordo Tauri src directory at: {}", tauri_src_dir.display());
            self.walk_directory(&tauri_src_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "rs" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_rust_model(&content) {
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
