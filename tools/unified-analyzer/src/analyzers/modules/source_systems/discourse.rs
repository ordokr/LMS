use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use regex::Regex;

use crate::analyzers::modules::helix_db_integration::{
    HelixDbTable, HelixDbField, HelixDbIndex, HelixDbRelationship, SourceSystem, SourceSystemType
};

/// Discourse forum source system
pub struct DiscourseSourceSystem {
    /// Whether to use caching
    use_cache: bool,
}

impl DiscourseSourceSystem {
    /// Create a new Discourse source system
    pub fn new() -> Self {
        Self {
            use_cache: true,
        }
    }

    /// Create a new Discourse source system with caching disabled
    pub fn new_without_cache() -> Self {
        Self {
            use_cache: false,
        }
    }

    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Parse a Rails schema.rb file
    fn parse_rails_schema(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();
        let mut current_table: Option<HelixDbTable> = None;
        let mut in_table_block = false;
        let mut table_block_content = String::new();

        // More robust parsing using line-by-line analysis
        let lines: Vec<&str> = content.lines().collect();

        // Table regex patterns
        let create_table_regex = Regex::new(r#"create_table\s+["']([^"']+)["']"#).unwrap();
        let field_regex = Regex::new(r#"\s*t\.([a-z_]+)\s+["']([^"']+)["'](?:,\s*([^\n]+))?"#).unwrap();
        let primary_key_regex = Regex::new(r"primary_key:\s*true").unwrap();
        let null_regex = Regex::new(r"null:\s*(true|false)").unwrap();
        let default_regex = Regex::new(r"default:\s*([^,\s]+)").unwrap();
        let unique_regex = Regex::new(r"unique:\s*true").unwrap();
        let end_regex = Regex::new(r"\s*end\s*").unwrap();

        // Index regex patterns
        let add_index_regex = Regex::new(r#"add_index\s+["']([^"']+)["'],\s*\[([^\]]+)\](?:,\s*([^\n]+))?"#).unwrap();
        let index_name_regex = Regex::new(r#"name:\s*["']([^"']+)["']"#).unwrap();
        let index_unique_regex = Regex::new(r"unique:\s*true").unwrap();

        for line in lines {
            // Check if we're starting a new table definition
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
                table_block_content = line.to_string();
                continue;
            }

            // If we're in a table block, collect the content
            if in_table_block {
                table_block_content.push_str("\n");
                table_block_content.push_str(line);

                // Check for field definitions
                if let Some(captures) = field_regex.captures(line) {
                    if let Some(table) = current_table.as_mut() {
                        let field_type = captures[1].to_string();
                        let field_name = captures[2].to_string();

                        // Default field properties
                        let mut nullable = true;
                        let mut default = None;
                        let mut primary_key = false;
                        let mut unique = false;

                        // Check for additional field properties
                        if let Some(props) = captures.get(3) {
                            let props_str = props.as_str();

                            // Check if field is primary key
                            if primary_key_regex.is_match(props_str) {
                                primary_key = true;
                            }

                            // Check if field is nullable
                            if let Some(null_captures) = null_regex.captures(props_str) {
                                nullable = null_captures[1].to_string() == "true";
                            }

                            // Check for default value
                            if let Some(default_captures) = default_regex.captures(props_str) {
                                default = Some(default_captures[1].to_string());
                            }

                            // Check if field is unique
                            if unique_regex.is_match(props_str) {
                                unique = true;
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

                // Check if we're ending the table block
                if end_regex.is_match(line) {
                    in_table_block = false;
                }
            }

            // Check for index definitions
            if let Some(captures) = add_index_regex.captures(line) {
                let table_name = captures[1].to_string();
                let fields_str = captures[2].to_string();

                // Parse the fields
                let fields: Vec<String> = fields_str
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                    .collect();

                // Default index properties
                let mut index_name = format!("index_{}_on_{}", table_name, fields.join("_"));
                let mut unique = false;

                // Check for additional index properties
                if let Some(props) = captures.get(3) {
                    let props_str = props.as_str();

                    // Check for index name
                    if let Some(name_captures) = index_name_regex.captures(props_str) {
                        index_name = name_captures[1].to_string();
                    }

                    // Check if index is unique
                    if index_unique_regex.is_match(props_str) {
                        unique = true;
                    }
                }

                // Find the table and add the index
                for table in &mut tables {
                    if table.name == table_name {
                        table.indexes.push(HelixDbIndex {
                            name: index_name.clone(),
                            fields: fields.clone(),
                            unique,
                        });
                        break;
                    }
                }

                // If we're currently processing a table with this name, add the index to it
                if let Some(table) = current_table.as_mut() {
                    if table.name == table_name {
                        table.indexes.push(HelixDbIndex {
                            name: index_name,
                            fields,
                            unique,
                        });
                    }
                }
            }
        }

        // Don't forget to add the last table if we were processing one
        if let Some(table) = current_table {
            tables.push(table);
        }

        Ok(tables)
    }

    /// Parse a Rails model file
    fn parse_rails_model(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // Model regex patterns
        let class_regex = Regex::new(r"class\s+([A-Za-z0-9_]+)\s+<\s+[A-Za-z0-9:]+").unwrap();

        // Relationship regex patterns
        let belongs_to_regex = Regex::new(r"belongs_to\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_many_regex = Regex::new(r"has_many\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_one_regex = Regex::new(r"has_one\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_and_belongs_to_many_regex = Regex::new(r"has_and_belongs_to_many\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();

        // Extract class name
        let class_name = if let Some(captures) = class_regex.captures(content) {
            captures[1].to_string()
        } else {
            // If we can't find a class name, we can't extract relationships
            return Ok(Vec::new());
        };

        // Create a table for this model
        let mut table = HelixDbTable {
            name: class_name.to_lowercase(),
            fields: Vec::new(),
            indexes: Vec::new(),
            relationships: Vec::new(),
            source: self.get_name(),
        };

        // Extract belongs_to relationships
        for captures in belongs_to_regex.captures_iter(content) {
            let related_model = captures[1].to_string();

            // Determine the foreign key
            let foreign_key = if let Some(props) = captures.get(2) {
                let props_str = props.as_str();
                if let Some(fk_captures) = Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
                    fk_captures[1].to_string()
                } else {
                    format!("{}_id", related_model)
                }
            } else {
                format!("{}_id", related_model)
            };

            // Add the relationship
            table.relationships.push(HelixDbRelationship {
                relationship_type: "belongs_to".to_string(),
                foreign_table: related_model,
                local_field: foreign_key.clone(),
                foreign_field: "id".to_string(),
            });

            // Add the foreign key field if it doesn't exist
            if !table.fields.iter().any(|f| f.name == foreign_key) {
                table.fields.push(HelixDbField {
                    name: foreign_key,
                    field_type: "integer".to_string(),
                    nullable: true,
                    default: None,
                    primary_key: false,
                    unique: false,
                });
            }
        }

        // Extract has_many relationships
        for captures in has_many_regex.captures_iter(content) {
            let related_model = captures[1].to_string();

            // Determine the foreign key
            let foreign_key = if let Some(props) = captures.get(2) {
                let props_str = props.as_str();
                if let Some(fk_captures) = Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
                    fk_captures[1].to_string()
                } else {
                    format!("{}_id", class_name.to_lowercase())
                }
            } else {
                format!("{}_id", class_name.to_lowercase())
            };

            // Add the relationship
            table.relationships.push(HelixDbRelationship {
                relationship_type: "has_many".to_string(),
                foreign_table: related_model,
                local_field: "id".to_string(),
                foreign_field: foreign_key,
            });
        }

        // Extract has_one relationships
        for captures in has_one_regex.captures_iter(content) {
            let related_model = captures[1].to_string();

            // Determine the foreign key
            let foreign_key = if let Some(props) = captures.get(2) {
                let props_str = props.as_str();
                if let Some(fk_captures) = Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
                    fk_captures[1].to_string()
                } else {
                    format!("{}_id", class_name.to_lowercase())
                }
            } else {
                format!("{}_id", class_name.to_lowercase())
            };

            // Add the relationship
            table.relationships.push(HelixDbRelationship {
                relationship_type: "has_one".to_string(),
                foreign_table: related_model,
                local_field: "id".to_string(),
                foreign_field: foreign_key,
            });
        }

        // Extract has_and_belongs_to_many relationships
        for captures in has_and_belongs_to_many_regex.captures_iter(content) {
            let related_model = captures[1].to_string();

            // Add the relationship
            table.relationships.push(HelixDbRelationship {
                relationship_type: "many_to_many".to_string(),
                foreign_table: related_model,
                local_field: "id".to_string(),
                foreign_field: "id".to_string(),
            });
        }

        // Add the table to the result
        tables.push(table);

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

impl SourceSystem for DiscourseSourceSystem {
    fn get_type(&self) -> SourceSystemType {
        SourceSystemType::Discourse
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>> {
        println!("Extracting database schema from Discourse codebase at: {}", path.display());

        let mut tables = Vec::new();

        // Look for schema.rb file
        let schema_path = path.join("db").join("schema.rb");
        if schema_path.exists() {
            println!("Found Discourse schema.rb at: {}", schema_path.display());
            if let Ok(content) = fs::read_to_string(&schema_path) {
                tables.extend(self.parse_rails_schema(&content)?);
            }
        }

        // Look for model files
        let models_dir = path.join("app").join("models");
        if models_dir.exists() {
            println!("Found Discourse models directory at: {}", models_dir.display());
            self.walk_directory(&models_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "rb" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_rails_model(&content) {
                                tables.extend(model_tables);
                            }
                        }
                    }
                }
            })?;
        }

        // Look for plugin models
        let plugins_dir = path.join("plugins");
        if plugins_dir.exists() {
            println!("Found Discourse plugins directory at: {}", plugins_dir.display());
            self.walk_directory(&plugins_dir, |file_path| {
                if let Some(ext) = file_path.extension() {
                    if ext == "rb" {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            if let Ok(model_tables) = self.parse_rails_model(&content) {
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
