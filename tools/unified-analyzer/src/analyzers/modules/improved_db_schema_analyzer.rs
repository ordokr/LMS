use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use regex::Regex;
use walkdir::WalkDir;
use anyhow::{Result, Context};

/// Represents a database column with its properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub primary_key: bool,
    pub foreign_key: Option<ForeignKey>,
    pub unique: bool,
    pub default_value: Option<String>,
    pub description: Option<String>,
}

/// Represents a foreign key relationship
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForeignKey {
    pub references_table: String,
    pub references_column: String,
}

/// Represents an index on a table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

/// Represents a database table with its columns, indexes, and relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub source_file: String,
    pub description: Option<String>,
}

/// Represents a database migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub operations: Vec<String>,
    pub file_path: String,
}

/// Improved database schema analyzer that extracts schema from multiple sources
#[derive(Debug, Serialize, Deserialize)]
pub struct ImprovedDbSchemaAnalyzer {
    pub tables: HashMap<String, Table>,
    pub migrations: Vec<Migration>,
}

impl Default for ImprovedDbSchemaAnalyzer {
    fn default() -> Self {
        Self {
            tables: HashMap::new(),
            migrations: Vec::new(),
        }
    }
}

impl ImprovedDbSchemaAnalyzer {
    /// Create a new instance of the analyzer
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze the database schema from the codebase
    pub fn analyze(&mut self, base_dir: &PathBuf) -> Result<()> {
        println!("Analyzing database schema...");

        // Extract schema from SQL migration files
        self.extract_from_migrations(base_dir)?;

        // Extract schema from schema.rs files
        self.extract_from_schema_rs(base_dir)?;

        // Extract schema from model definitions
        self.extract_from_models(base_dir)?;

        // Extract relationships based on foreign keys
        self.extract_relationships();

        println!("Database schema analysis complete. Found {} tables and {} migrations.",
            self.tables.len(), self.migrations.len());

        Ok(())
    }

    /// Extract schema from SQL migration files
    fn extract_from_migrations(&mut self, base_dir: &PathBuf) -> Result<()> {
        println!("Extracting schema from SQL migration files...");

        // Look for migration files in src-tauri/migrations
        let migrations_dir = base_dir.join("src-tauri").join("migrations");
        if !migrations_dir.exists() {
            println!("Migrations directory not found: {:?}", migrations_dir);
            return Ok(());
        }

        // Process each migration file
        for entry in WalkDir::new(&migrations_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-SQL files
            if path.extension().and_then(|ext| ext.to_str()) != Some("sql") {
                continue;
            }

            // Read the file content
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read migration file: {:?}", path))?;

            // Extract migration version and name from filename
            let file_name = path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            let parts: Vec<&str> = file_name.split('_').collect();
            let version = parts.first().unwrap_or(&"unknown").to_string();
            let name = parts[1..].join("_").replace(".sql", "");

            // Extract operations
            let mut operations = Vec::new();
            if content.contains("CREATE TABLE") {
                operations.push("create_table".to_string());
            }
            if content.contains("ALTER TABLE") {
                operations.push("alter_table".to_string());
            }
            if content.contains("DROP TABLE") {
                operations.push("drop_table".to_string());
            }
            if content.contains("CREATE INDEX") {
                operations.push("create_index".to_string());
            }
            if content.contains("DROP INDEX") {
                operations.push("drop_index".to_string());
            }

            // Add migration
            self.migrations.push(Migration {
                version,
                name,
                operations,
                file_path: path.to_string_lossy().to_string(),
            });

            // Extract table definitions
            self.extract_tables_from_sql(&content, path.to_string_lossy().to_string())?;
        }

        Ok(())
    }

    /// Extract table definitions from SQL content
    fn extract_tables_from_sql(&mut self, content: &str, file_path: String) -> Result<()> {
        // Regex to match CREATE TABLE statements
        let create_table_regex = Regex::new(r"CREATE TABLE(?:\s+IF NOT EXISTS)?\s+(\w+)\s*\(([\s\S]*?)\)")?;

        // Find all CREATE TABLE statements
        for cap in create_table_regex.captures_iter(content) {
            if let (Some(table_name), Some(columns_def)) = (cap.get(1), cap.get(2)) {
                let table_name = table_name.as_str().to_string();

                // Create a new table
                let mut table = Table {
                    name: table_name.clone(),
                    columns: Vec::new(),
                    indexes: Vec::new(),
                    source_file: file_path.clone(),
                    description: None,
                };

                // Extract columns
                let columns_str = columns_def.as_str();
                let column_regex = Regex::new(r"(?m)^\s*(\w+)\s+(\w+(?:\([^)]+\))?)\s*((?:NOT NULL|PRIMARY KEY|UNIQUE|DEFAULT [^,]+|REFERENCES [^,]+)*)")?;

                for col_cap in column_regex.captures_iter(columns_str) {
                    if let (Some(col_name), Some(col_type), Some(constraints)) = (col_cap.get(1), col_cap.get(2), col_cap.get(3)) {
                        let col_name = col_name.as_str().to_string();
                        let col_type = col_type.as_str().to_string();
                        let constraints = constraints.as_str().to_string();

                        // Parse constraints
                        let nullable = !constraints.contains("NOT NULL");
                        let primary_key = constraints.contains("PRIMARY KEY");
                        let unique = constraints.contains("UNIQUE");

                        // Extract default value
                        let default_regex = Regex::new(r"DEFAULT\s+([^,\s]+)")?;
                        let default_value = default_regex.captures(&constraints)
                            .and_then(|cap| cap.get(1))
                            .map(|m| m.as_str().to_string());

                        // Extract foreign key
                        let foreign_key_regex = Regex::new(r"REFERENCES\s+(\w+)\s*\((\w+)\)")?;
                        let foreign_key = foreign_key_regex.captures(&constraints)
                            .map(|cap| {
                                let references_table = cap.get(1).map_or("", |m| m.as_str()).to_string();
                                let references_column = cap.get(2).map_or("", |m| m.as_str()).to_string();

                                ForeignKey {
                                    references_table,
                                    references_column,
                                }
                            });

                        // Create column
                        let column = Column {
                            name: col_name,
                            data_type: col_type,
                            nullable,
                            primary_key,
                            foreign_key,
                            unique,
                            default_value,
                            description: None,
                        };

                        table.columns.push(column);
                    }
                }

                // Extract indexes
                let index_regex = Regex::new(r"CREATE(?:\s+UNIQUE)?\s+INDEX(?:\s+IF NOT EXISTS)?\s+(\w+)\s+ON\s+(\w+)\s*\(([^)]+)\)")?;

                for idx_cap in index_regex.captures_iter(content) {
                    if let (Some(idx_name), Some(idx_table), Some(idx_columns)) = (idx_cap.get(1), idx_cap.get(2), idx_cap.get(3)) {
                        let idx_name = idx_name.as_str().to_string();
                        let idx_table = idx_table.as_str().to_string();
                        let idx_columns = idx_columns.as_str().to_string();

                        // Only add index if it belongs to this table
                        if idx_table == table_name {
                            let columns: Vec<String> = idx_columns.split(',')
                                .map(|s| s.trim().to_string())
                                .collect();

                            let unique = idx_cap.get(0).map_or("", |m| m.as_str()).contains("UNIQUE");

                            let index = Index {
                                name: idx_name,
                                columns,
                                unique,
                            };

                            table.indexes.push(index);
                        }
                    }
                }

                // Add table to the collection
                self.tables.insert(table_name, table);
            }
        }

        Ok(())
    }

    /// Extract schema from schema.rs files
    fn extract_from_schema_rs(&mut self, base_dir: &PathBuf) -> Result<()> {
        println!("Extracting schema from schema.rs files...");

        // Look for schema.rs files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-schema.rs files
            if path.file_name().and_then(|name| name.to_str()) != Some("schema.rs") {
                continue;
            }

            // Read the file content
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read schema.rs file: {:?}", path))?;

            // Extract table definitions
            self.extract_tables_from_schema_rs(&content, path.to_string_lossy().to_string())?;
        }

        Ok(())
    }

    /// Extract table definitions from schema.rs content
    fn extract_tables_from_schema_rs(&mut self, content: &str, file_path: String) -> Result<()> {
        // Regex to match table! macro
        let table_regex = Regex::new(r"table!\s*\{\s*(\w+)\s*\(\s*(\w+)\s*\)\s*\{([\s\S]*?)\}\s*\}")?;

        // Find all table! macros
        for cap in table_regex.captures_iter(content) {
            if let (Some(table_name), Some(primary_key), Some(columns_def)) = (cap.get(1), cap.get(2), cap.get(3)) {
                let table_name = table_name.as_str().to_string();
                let primary_key = primary_key.as_str().to_string();

                // Create a new table or get existing one
                let table = self.tables.entry(table_name.clone()).or_insert(Table {
                    name: table_name.clone(),
                    columns: Vec::new(),
                    indexes: Vec::new(),
                    source_file: file_path.clone(),
                    description: None,
                });

                // Extract columns
                let column_regex = Regex::new(r"(\w+)\s*->\s*(\w+)(?:\s*,\s*(\w+))?")?;

                for col_cap in column_regex.captures_iter(columns_def.as_str()) {
                    if let (Some(col_name), Some(col_type)) = (col_cap.get(1), col_cap.get(2)) {
                        let col_name = col_name.as_str().to_string();
                        let col_type = col_type.as_str().to_string();

                        // Check if column already exists
                        if !table.columns.iter().any(|c| c.name == col_name) {
                            // Create column
                            let column = Column {
                                name: col_name.clone(),
                                data_type: col_type,
                                nullable: col_cap.get(3).map_or(false, |m| m.as_str() == "Nullable"),
                                primary_key: col_name == primary_key,
                                foreign_key: None,
                                unique: false,
                                default_value: None,
                                description: None,
                            };

                            table.columns.push(column);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract schema from model definitions
    fn extract_from_models(&mut self, base_dir: &PathBuf) -> Result<()> {
        println!("Extracting schema from model definitions...");

        // Look for model files
        for entry in WalkDir::new(base_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Skip non-Rust files
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
                continue;
            }

            // Read the file content
            let content = fs::read_to_string(path)
                .with_context(|| format!("Failed to read model file: {:?}", path))?;

            // Extract struct definitions
            self.extract_structs_from_models(&content, path.to_string_lossy().to_string())?;
        }

        Ok(())
    }

    /// Extract struct definitions from model files
    fn extract_structs_from_models(&mut self, content: &str, file_path: String) -> Result<()> {
        // Regex to match struct definitions with derive attributes
        let struct_regex = Regex::new(r"#\[derive\([^)]*(?:Serialize|Deserialize|FromRow)[^)]*\)\]\s*(?:#\[[^\]]+\])?\s*pub\s+struct\s+(\w+)\s*\{([\s\S]*?)\}")?;

        // Find all struct definitions
        for cap in struct_regex.captures_iter(content) {
            if let (Some(struct_name), Some(fields_def)) = (cap.get(1), cap.get(2)) {
                let struct_name = struct_name.as_str().to_string();

                // Try to extract struct description from comments
                let struct_description = self.extract_struct_description(content, &struct_name);

                // Convert struct name to snake_case for table name
                let table_name = to_snake_case(&struct_name);

                // Check if we need to create a new table or update an existing one
                if !self.tables.contains_key(&table_name) {
                    // Create a new table
                    self.tables.insert(table_name.clone(), Table {
                        name: table_name.clone(),
                        columns: Vec::new(),
                        indexes: Vec::new(),
                        source_file: file_path.clone(),
                        description: struct_description,
                    });
                } else if let Some(table) = self.tables.get_mut(&table_name) {
                    // Update description if it's None
                    if table.description.is_none() && struct_description.is_some() {
                        table.description = struct_description;
                    }
                }

                // Get a mutable reference to the table
                let table = self.tables.get_mut(&table_name).unwrap();

                // First, collect all fields and indexes we need to add
                let field_regex = Regex::new(r"pub\s+(\w+):\s*([^,\n]+)")?;
                let mut fields_to_add = Vec::new();

                // Get existing column names for checking
                let existing_columns: Vec<String> = table.columns.iter().map(|c| c.name.clone()).collect();

                // Drop the mutable borrow of table
                std::mem::drop(table);

                // Now process fields without a mutable borrow
                for field_cap in field_regex.captures_iter(fields_def.as_str()) {
                    if let (Some(field_name), Some(field_type)) = (field_cap.get(1), field_cap.get(2)) {
                        let field_name = field_name.as_str().to_string();
                        let field_type = field_type.as_str().trim().to_string();

                        // Only process fields that don't already exist
                        if !existing_columns.contains(&field_name) {
                            // Process this field
                            let field_info = self.process_field(fields_def.as_str(), &field_name, &field_type);
                            fields_to_add.push(field_info);
                        }
                    }
                }

                // Extract indexes from attributes
                let indexes = self.extract_indexes_from_attributes(content, &struct_name);

                // Now get a new mutable reference to the table and add the fields and indexes
                if let Some(table) = self.tables.get_mut(&table_name) {
                    // Add fields
                    for field in fields_to_add {
                        table.columns.push(field);
                    }

                    // Add indexes
                    for index in indexes {
                        table.indexes.push(index);
                    }
                }
            }
        }

        Ok(())
    }

    /// Extract relationships based on foreign keys
    fn extract_relationships(&mut self) {
        println!("Extracting relationships based on foreign keys...");

        // Create a copy of table names to avoid borrowing issues
        let table_names: Vec<String> = self.tables.keys().cloned().collect();

        // Create a copy of tables that might be referenced
        let referenced_tables: std::collections::HashSet<String> = self.tables.keys().cloned().collect();

        for table_name in table_names {
            // First, collect all potential foreign key columns
            let mut foreign_key_updates = Vec::new();

            if let Some(table) = self.tables.get(&table_name) {
                // Find columns that might be foreign keys
                for col in &table.columns {
                    if col.name.ends_with("_id") && col.foreign_key.is_none() {
                        let referenced_table = col.name.replace("_id", "s");
                        if referenced_tables.contains(&referenced_table) {
                            foreign_key_updates.push((col.name.clone(), referenced_table));
                        }
                    }
                }
            }

            // Now update the columns with foreign key information
            if !foreign_key_updates.is_empty() {
                if let Some(table) = self.tables.get_mut(&table_name) {
                    for (col_name, referenced_table) in foreign_key_updates {
                        // Find the column and update its foreign key
                        for col in &mut table.columns {
                            if col.name == col_name {
                                col.foreign_key = Some(ForeignKey {
                                    references_table: referenced_table.clone(),
                                    references_column: "id".to_string(),
                                });
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Detect foreign key relationships from field name and type
    fn detect_foreign_key(&self, field_name: &str, field_type: &str) -> Option<ForeignKey> {
        // Case 1: Standard _id suffix
        if field_name.ends_with("_id") {
            let base_name = field_name.trim_end_matches("_id");
            let references_table = self.pluralize(base_name);

            // Check if the referenced table exists
            if self.tables.contains_key(&references_table) {
                return Some(ForeignKey {
                    references_table,
                    references_column: "id".to_string(),
                });
            }
        }

        // Case 2: Field type is a reference to another entity
        if field_type.contains("Id") || field_type.contains("Reference") {
            // Extract the entity name from the type
            let type_parts: Vec<&str> = field_type.split(['<', '>', ':', ',', ' ']).collect();
            for part in type_parts {
                if part.ends_with("Id") || part.ends_with("Reference") {
                    let entity_name = part.trim_end_matches("Id").trim_end_matches("Reference");
                    let references_table = to_snake_case(entity_name);

                    // Check if the referenced table exists
                    if self.tables.contains_key(&references_table) {
                        return Some(ForeignKey {
                            references_table,
                            references_column: "id".to_string(),
                        });
                    }
                }
            }
        }

        None
    }

    /// Clean up type names for better display
    fn clean_type_name(&self, type_name: &str) -> String {
        // Handle Option<T> types
        if type_name.starts_with("Option<") && type_name.ends_with(">") {
            let inner_type = &type_name[7..type_name.len()-1];
            return format!("Option<{}>", self.clean_type_name(inner_type));
        }

        // Handle Vec<T> types
        if type_name.starts_with("Vec<") && type_name.ends_with(">") {
            let inner_type = &type_name[4..type_name.len()-1];
            return format!("Vec<{}>", self.clean_type_name(inner_type));
        }

        // Handle HashMap<K, V> types
        if type_name.starts_with("HashMap<") && type_name.ends_with(">") {
            let inner_part = &type_name[8..type_name.len()-1];
            let parts: Vec<&str> = inner_part.split(',').collect();
            if parts.len() == 2 {
                return format!("HashMap<{}, {}>",
                    self.clean_type_name(parts[0].trim()),
                    self.clean_type_name(parts[1].trim())
                );
            }
        }

        // Handle other generic types
        if let Some(pos) = type_name.find('<') {
            if type_name.ends_with('>') {
                let base_type = &type_name[0..pos];
                let inner_part = &type_name[pos+1..type_name.len()-1];
                return format!("{}<{}>", base_type, self.clean_type_name(inner_part));
            }
        }

        type_name.to_string()
    }

    /// Helper function to pluralize a word
    fn pluralize(&self, word: &str) -> String {
        if word.ends_with('y') {
            format!("{}{}", &word[0..word.len()-1], "ies")
        } else if word.ends_with('s') || word.ends_with('x') || word.ends_with('z') ||
                  word.ends_with("ch") || word.ends_with("sh") {
            format!("{}{}", word, "es")
        } else {
            format!("{}{}", word, "s")
        }
    }

    /// Extract struct description from comments
    fn extract_struct_description(&self, content: &str, struct_name: &str) -> Option<String> {
        // Look for doc comments before the struct definition
        let comment_regex = Regex::new(&format!(r"(?ms)\s*((?:///?[^\n]*\n)+)\s*#\[derive\([^)]*\)\]\s*(?:#\[[^\]]+\])?\s*pub\s+struct\s+{}\s*\{{", struct_name))
            .unwrap_or_else(|_| Regex::new(r"a^b").unwrap()); // Fallback to a regex that won't match anything

        if let Some(cap) = comment_regex.captures(content) {
            if let Some(comments) = cap.get(1) {
                // Extract and clean up the comments
                let comment_lines: Vec<String> = comments.as_str()
                    .lines()
                    .map(|line| line.trim_start_matches("//").trim_start_matches("/").trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();

                if !comment_lines.is_empty() {
                    return Some(comment_lines.join(" "));
                }
            }
        }

        None
    }

    /// Extract field description from comments
    fn extract_field_description(&self, fields_def: &str, field_name: &str) -> Option<String> {
        // Look for comments before the field definition
        let comment_regex = Regex::new(&format!(r"(?ms)\s*((?:///?[^\n]*\n)+)\s*pub\s+{}\s*:", field_name))
            .unwrap_or_else(|_| Regex::new(r"a^b").unwrap()); // Fallback to a regex that won't match anything

        if let Some(cap) = comment_regex.captures(fields_def) {
            if let Some(comments) = cap.get(1) {
                // Extract and clean up the comments
                let comment_lines: Vec<String> = comments.as_str()
                    .lines()
                    .map(|line| line.trim_start_matches("//").trim_start_matches("/").trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();

                if !comment_lines.is_empty() {
                    return Some(comment_lines.join(" "));
                }
            }
        }

        // Also look for comments at the end of the line
        let inline_comment_regex = Regex::new(&format!(r"pub\s+{}\s*:[^,\n]+,?\s*///?\s*([^\n]+)", field_name))
            .unwrap_or_else(|_| Regex::new(r"a^b").unwrap());

        if let Some(cap) = inline_comment_regex.captures(fields_def) {
            if let Some(comment) = cap.get(1) {
                return Some(comment.as_str().trim().to_string());
            }
        }

        None
    }

    /// Process a field and create a Column object
    fn process_field(&self, fields_def: &str, field_name: &str, field_type: &str) -> Column {
        // Try to extract field description from comments
        let field_description = self.extract_field_description(fields_def, field_name);

        // Determine if field is a foreign key using our helper method
        let foreign_key = self.detect_foreign_key(field_name, field_type);

        // Clean up type name for better display
        let clean_type = self.clean_type_name(field_type);

        // Create column
        let is_nullable = field_type.starts_with("Option<");
        Column {
            name: field_name.to_string(),
            data_type: clean_type,
            nullable: is_nullable,
            primary_key: field_name == "id",
            foreign_key,
            unique: field_name == "id" || field_name.contains("_key") || field_name.contains("unique"),
            default_value: None,
            description: field_description,
        }
    }

    /// Extract indexes from struct attributes
    fn extract_indexes_from_attributes(&self, content: &str, struct_name: &str) -> Vec<Index> {
        let mut indexes = Vec::new();

        // Look for index attributes like #[index(name = "idx_name", columns = ["col1", "col2"])]
        let regex_pattern = format!(r##"#\[index\(\s*name\s*=\s*"([^"]+)"\s*,\s*columns\s*=\s*\[([^\]]+)\](?:\s*,\s*unique\s*=\s*(true|false))?\s*\)\]\s*(?:#[^\[]+)*struct\s+{}\s*\{{"##, struct_name);
        let index_regex = Regex::new(&regex_pattern)
            .unwrap_or_else(|_| Regex::new(r"a^b").unwrap()); // Fallback to a regex that won't match anything

        if let Some(cap) = index_regex.captures(content) {
            if let (Some(idx_name), Some(columns_str)) = (cap.get(1), cap.get(2)) {
                let idx_name = idx_name.as_str().to_string();
                let columns: Vec<String> = columns_str.as_str()
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .collect();
                let unique = cap.get(3).map_or(false, |m| m.as_str() == "true");

                indexes.push(Index {
                    name: idx_name,
                    columns,
                    unique,
                });
            }
        }

        indexes
    }

    /// Generate a Mermaid diagram from the extracted schema
    pub fn generate_mermaid_diagram(&self) -> String {
        println!("Generating Mermaid diagram from extracted schema...");

        let mut mermaid = String::from("erDiagram\n");

        // Limit the number of tables to avoid overwhelming the diagram
        // Focus on the most important tables (those with relationships)
        let mut important_tables: HashSet<String> = HashSet::new();

        // First, identify tables involved in relationships
        for (_, table) in &self.tables {
            for column in &table.columns {
                if let Some(fk) = &column.foreign_key {
                    important_tables.insert(table.name.clone());
                    important_tables.insert(fk.references_table.clone());
                }
            }
        }

        // If we have too few important tables, add some more until we reach a reasonable number
        if important_tables.len() < 20 {
            for (_, table) in &self.tables {
                if important_tables.len() >= 30 {
                    break;
                }
                important_tables.insert(table.name.clone());
            }
        }

        // Add each important table to the diagram
        for (_, table) in &self.tables {
            if !important_tables.contains(&table.name) {
                continue;
            }

            mermaid.push_str(&format!("    {} {{\n", table.name));

            // Limit the number of columns to avoid overwhelming the diagram
            // Focus on primary keys, foreign keys, and a few other important columns
            let mut important_columns: Vec<&Column> = table.columns.iter()
                .filter(|c| c.primary_key || c.foreign_key.is_some() || c.unique)
                .collect();

            // Add a few more columns if we don't have enough
            if important_columns.len() < 3 {
                for column in &table.columns {
                    if !important_columns.contains(&column) && important_columns.len() < 5 {
                        important_columns.push(column);
                    }
                }
            }

            for column in important_columns {
                let pk_indicator = if column.primary_key { "PK " } else { "" };
                let fk_indicator = if column.foreign_key.is_some() { "FK " } else { "" };

                // Sanitize the data type for Mermaid compatibility
                let sanitized_type = self.sanitize_for_mermaid(&column.data_type);

                mermaid.push_str(&format!("        {} {} {}{}\n",
                    sanitized_type,
                    column.name,
                    pk_indicator,
                    fk_indicator
                ));
            }

            mermaid.push_str("    }\n");
        }

        // Add relationships
        for (_, table) in &self.tables {
            if !important_tables.contains(&table.name) {
                continue;
            }

            for column in &table.columns {
                if let Some(fk) = &column.foreign_key {
                    if important_tables.contains(&fk.references_table) {
                        mermaid.push_str(&format!("    {} ||--o{{ {} : \"{}\"\n",
                            fk.references_table,
                            table.name,
                            column.name
                        ));
                    }
                }
            }
        }

        mermaid
    }

    /// Sanitize a type name for Mermaid compatibility
    fn sanitize_for_mermaid(&self, type_name: &str) -> String {
        // First, simplify complex types
        let simplified = if type_name.contains('<') {
            // For complex types like Vec<String>, just use the base type
            let base_type = type_name.split('<').next().unwrap_or(type_name);
            format!("{}_of", base_type)
        } else {
            type_name.to_string()
        };

        // Replace colons with underscores
        let s1 = simplified.replace("::", "_");

        // Replace any other problematic characters
        let s2 = s1.replace("[", "_").replace("]", "_");

        s2
    }

    /// Generate a JSON representation of the schema
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self).context("Failed to serialize schema to JSON")
    }

    /// Generate a Markdown documentation of the schema
    pub fn generate_markdown_documentation(&self) -> String {
        let mut markdown = String::from("# Database Schema Documentation\n\n");
        markdown.push_str("This document provides a comprehensive overview of the database schema, including tables, relationships, and migrations.\n\n");

        // Tables section
        markdown.push_str("## Tables\n\n");

        for (_, table) in &self.tables {
            markdown.push_str(&format!("### {}\n\n", table.name));

            if let Some(description) = &table.description {
                markdown.push_str(&format!("{}\n\n", description));
            }

            // Create table for columns
            markdown.push_str("| Column | Type | Constraints | Description |\n");
            markdown.push_str("|--------|------|-------------|-------------|\n");

            for column in &table.columns {
                let mut constraints = Vec::new();

                if column.primary_key {
                    constraints.push("PRIMARY KEY".to_string());
                }

                if !column.nullable {
                    constraints.push("NOT NULL".to_string());
                }

                if column.unique {
                    constraints.push("UNIQUE".to_string());
                }

                if column.foreign_key.is_some() {
                    constraints.push("FOREIGN KEY".to_string());
                }

                if let Some(default) = &column.default_value {
                    constraints.push(format!("DEFAULT {}", default));
                }

                let constraints_str = constraints.join(", ");

                let description = column.description.as_deref().unwrap_or("");

                markdown.push_str(&format!("| {} | {} | {} | {} |\n",
                    column.name,
                    column.data_type,
                    constraints_str,
                    description
                ));
            }

            markdown.push_str("\n");

            // Add indexes if any
            if !table.indexes.is_empty() {
                markdown.push_str("#### Indexes\n\n");
                markdown.push_str("| Name | Columns | Unique |\n");
                markdown.push_str("|------|---------|--------|\n");

                for index in &table.indexes {
                    markdown.push_str(&format!("| {} | {} | {} |\n",
                        index.name,
                        index.columns.join(", "),
                        if index.unique { "Yes" } else { "No" }
                    ));
                }

                markdown.push_str("\n");
            }
        }

        // Relationships section
        markdown.push_str("## Relationships\n\n");
        markdown.push_str("| Parent Table | Child Table | Relationship Type | Foreign Key |\n");
        markdown.push_str("|-------------|------------|-------------------|-------------|\n");

        for (_, table) in &self.tables {
            for column in &table.columns {
                if let Some(fk) = &column.foreign_key {
                    markdown.push_str(&format!("| {} | {} | {} | {} |\n",
                        fk.references_table,
                        table.name,
                        "one-to-many",
                        column.name
                    ));
                }
            }
        }

        markdown.push_str("\n");

        // Migrations section
        if !self.migrations.is_empty() {
            markdown.push_str("## Migrations\n\n");
            markdown.push_str("| Version | Name | Operations |\n");
            markdown.push_str("|---------|------|------------|\n");

            for migration in &self.migrations {
                markdown.push_str(&format!("| {} | {} | {} |\n",
                    migration.version,
                    migration.name,
                    migration.operations.join(", ")
                ));
            }
        }

        markdown
    }
}

/// Convert a CamelCase string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, c) in s.char_indices() {
        if i > 0 && c.is_uppercase() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    // Pluralize the name
    if !result.ends_with('s') {
        result.push('s');
    }

    result
}
