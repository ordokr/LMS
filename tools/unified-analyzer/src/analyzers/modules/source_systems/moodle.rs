use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use regex::Regex;

use crate::analyzers::modules::helix_db_integration::{
    HelixDbTable, HelixDbField, HelixDbIndex, HelixDbRelationship, SourceSystem, SourceSystemType
};

/// Moodle LMS source system
pub struct MoodleSourceSystem {
    /// Whether to use caching
    use_cache: bool,
}

impl MoodleSourceSystem {
    /// Create a new Moodle source system
    pub fn new() -> Self {
        Self {
            use_cache: true,
        }
    }

    /// Create a new Moodle source system with caching disabled
    pub fn new_without_cache() -> Self {
        Self {
            use_cache: false,
        }
    }

    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }

    /// Parse a Moodle install.xml file
    fn parse_install_xml(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // XML parsing using regex (in a real implementation, we would use a proper XML parser)
        let table_regex = Regex::new(r#"<TABLE NAME="([^"]+)"[^>]*>"#).unwrap();
        let field_regex = Regex::new(r#"<FIELD NAME="([^"]+)"[^>]*TYPE="([^"]+)"[^>]*(?:NOTNULL="([^"]+)")?[^>]*(?:DEFAULT="([^"]*)")?[^>]*(?:SEQUENCE="([^"]+)")?[^>]*(?:KEY="([^"]+)")?[^>]*>"#).unwrap();
        let key_regex = Regex::new(r#"<KEY NAME="([^"]+)"[^>]*TYPE="([^"]+)"[^>]*FIELDS="([^"]+)"[^>]*>"#).unwrap();

        // Extract tables
        let mut current_table: Option<HelixDbTable> = None;

        for line in content.lines() {
            // Check for table definition
            if let Some(captures) = table_regex.captures(line) {
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
            }

            // Check for field definition
            if let Some(captures) = field_regex.captures(line) {
                if let Some(table) = current_table.as_mut() {
                    let field_name = captures[1].to_string();
                    let field_type = captures[2].to_string();

                    // Parse field properties
                    let nullable = captures.get(3).map_or(true, |m| m.as_str() != "true");
                    let default = captures.get(4).map(|m| m.as_str().to_string());
                    let _is_sequence = captures.get(5).map_or(false, |m| m.as_str() == "true");
                    let is_key = captures.get(6).map_or(false, |m| m.as_str() == "true");

                    // Add the field to the table
                    table.fields.push(HelixDbField {
                        name: field_name,
                        field_type,
                        nullable,
                        default,
                        primary_key: is_key,
                        unique: false,
                    });
                }
            }

            // Check for key definition
            if let Some(captures) = key_regex.captures(line) {
                if let Some(table) = current_table.as_mut() {
                    let key_name = captures[1].to_string();
                    let key_type = captures[2].to_string();
                    let fields_str = captures[3].to_string();

                    // Parse fields
                    let fields: Vec<String> = fields_str.split(',').map(|s| s.trim().to_string()).collect();

                    // Check if this is a primary key
                    if key_type == "primary" {
                        // Update the primary key flag for the fields
                        for field in &mut table.fields {
                            if fields.contains(&field.name) {
                                field.primary_key = true;
                            }
                        }
                    }

                    // Check if this is a unique key
                    let unique = key_type == "unique";

                    // Add the index to the table
                    table.indexes.push(HelixDbIndex {
                        name: key_name,
                        fields,
                        unique,
                    });
                }
            }
        }

        // Don't forget to add the last table if we were processing one
        if let Some(table) = current_table {
            tables.push(table);
        }

        Ok(tables)
    }

    /// Parse a Moodle upgrade.php file
    fn parse_upgrade_php(&self, content: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // PHP parsing using regex
        let table_regex = Regex::new(r#"xmldb_table\s*\(\s*['"](\w+)['"]\s*\)"#).unwrap();
        let add_field_regex = Regex::new(r#"add_field\s*\(\s*['"](\w+)['"]\s*,\s*xmldb_field\s*\(\s*['"](\w+)['"]\s*,\s*['"](\w+)['"]\s*(?:,\s*['"](\w+)['"]\s*)?(?:,\s*([^,\)]+)\s*)?(?:,\s*([^,\)]+)\s*)?(?:,\s*([^,\)]+)\s*)?\)"#).unwrap();
        let add_key_regex = Regex::new(r#"add_key\s*\(\s*['"](\w+)['"]\s*,\s*xmldb_key\s*\(\s*['"](\w+)['"]\s*,\s*['"](\w+)['"]\s*,\s*\[\s*['"](\w+)['"]\s*(?:,\s*['"](\w+)['"]\s*)*\]\s*\)"#).unwrap();

        // Extract tables and fields
        let mut current_table: Option<String> = None;
        let mut tables_map: std::collections::HashMap<String, HelixDbTable> = std::collections::HashMap::new();

        for line in content.lines() {
            // Check for table definition
            if let Some(captures) = table_regex.captures(line) {
                let table_name = captures[1].to_string();
                current_table = Some(table_name.clone());

                // Create the table if it doesn't exist
                if !tables_map.contains_key(&table_name) {
                    tables_map.insert(table_name.clone(), HelixDbTable {
                        name: table_name,
                        fields: Vec::new(),
                        indexes: Vec::new(),
                        relationships: Vec::new(),
                        source: self.get_name(),
                    });
                }
            }

            // Check for field definition
            if let Some(captures) = add_field_regex.captures(line) {
                if let Some(table_name) = &current_table {
                    let field_name = captures[2].to_string();
                    let field_type = captures[3].to_string();

                    // Parse field properties
                    let _precision = captures.get(4).map(|m| m.as_str().to_string());
                    let nullable = captures.get(5).map_or(true, |m| m.as_str().contains("XMLDB_NOTNULL"));
                    let _has_default = captures.get(6).map_or(false, |m| m.as_str().contains("XMLDB_SEQUENCE"));
                    let is_key = captures.get(7).map_or(false, |m| m.as_str().contains("XMLDB_KEY"));

                    // Add the field to the table
                    if let Some(table) = tables_map.get_mut(table_name) {
                        table.fields.push(HelixDbField {
                            name: field_name,
                            field_type,
                            nullable: !nullable,
                            default: None,
                            primary_key: is_key,
                            unique: false,
                        });
                    }
                }
            }

            // Check for key definition
            if let Some(captures) = add_key_regex.captures(line) {
                if let Some(table_name) = &current_table {
                    let key_name = captures[2].to_string();
                    let key_type = captures[3].to_string();

                    // Parse fields
                    let mut fields = Vec::new();
                    for i in 4..captures.len() {
                        if let Some(field) = captures.get(i) {
                            fields.push(field.as_str().to_string());
                        }
                    }

                    // Check if this is a primary key
                    if key_type == "primary" {
                        // Update the primary key flag for the fields
                        if let Some(table) = tables_map.get_mut(table_name) {
                            for field in &mut table.fields {
                                if fields.contains(&field.name) {
                                    field.primary_key = true;
                                }
                            }
                        }
                    }

                    // Check if this is a unique key
                    let unique = key_type == "unique";

                    // Add the index to the table
                    if let Some(table) = tables_map.get_mut(table_name) {
                        table.indexes.push(HelixDbIndex {
                            name: key_name,
                            fields,
                            unique,
                        });
                    }
                }
            }
        }

        // Convert the HashMap to a Vec
        tables.extend(tables_map.into_values());

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

impl SourceSystem for MoodleSourceSystem {
    fn get_type(&self) -> SourceSystemType {
        SourceSystemType::Moodle
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>> {
        println!("Extracting database schema from Moodle codebase at: {}", path.display());

        let mut tables = Vec::new();

        // Look for install.xml files
        self.walk_directory(path, |file_path| {
            if file_path.file_name().map_or(false, |name| name == "install.xml") {
                println!("Found Moodle install.xml at: {}", file_path.display());
                if let Ok(content) = fs::read_to_string(file_path) {
                    if let Ok(xml_tables) = self.parse_install_xml(&content) {
                        tables.extend(xml_tables);
                    }
                }
            }
        })?;

        // Look for upgrade.php files
        self.walk_directory(path, |file_path| {
            if file_path.file_name().map_or(false, |name| name == "upgrade.php") {
                println!("Found Moodle upgrade.php at: {}", file_path.display());
                if let Ok(content) = fs::read_to_string(file_path) {
                    if let Ok(php_tables) = self.parse_upgrade_php(&content) {
                        tables.extend(php_tables);
                    }
                }
            }
        })?;

        Ok(tables)
    }
}

use std::collections::HashMap;
