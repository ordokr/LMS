use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationColumn {
    pub name: String,
    pub column_type: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationIndex {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub unique: bool,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationForeignKey {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
    pub name: Option<String>,
    pub on_delete: Option<String>,
    pub on_update: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MigrationTable {
    pub name: String,
    pub columns: Vec<MigrationColumn>,
    pub indexes: Vec<MigrationIndex>,
    pub foreign_keys: Vec<MigrationForeignKey>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnhancedRubyMigration {
    pub name: String,
    pub file_path: String,
    pub version: String,
    pub tables_created: Vec<MigrationTable>,
    pub tables_altered: Vec<MigrationTable>,
    pub tables_dropped: Vec<String>,
    pub raw_content: String,
}

#[derive(Debug, Default)]
pub struct EnhancedRubyMigrationAnalyzer {
    pub migrations: Vec<EnhancedRubyMigration>,
    pub tables: HashMap<String, MigrationTable>,
}

impl EnhancedRubyMigrationAnalyzer {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            tables: HashMap::new(),
        }
    }

    pub fn analyze_directory(&mut self, directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing Ruby migrations in directory: {:?}", directory);
        
        // Find migrations directory
        let migrations_dir = find_migrations_directory(directory);
        
        if let Some(migrations_dir) = migrations_dir {
            println!("Found migrations directory: {:?}", migrations_dir);
            
            // Collect all migration files
            let mut migration_files = Vec::new();
            for entry in WalkDir::new(&migrations_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_string_lossy();
                        if file_name_str.ends_with(".rb") && file_name_str.contains("_") {
                            // Check if it's a migration file (starts with a timestamp or version number)
                            if let Some(version) = extract_migration_version(&file_name_str) {
                                migration_files.push((path.to_path_buf(), version));
                            }
                        }
                    }
                }
            }
            
            // Sort migration files by version
            migration_files.sort_by(|a, b| a.1.cmp(&b.1));
            
            // Analyze each migration file in order
            for (file_path, version) in migration_files {
                self.analyze_migration_file(&file_path, &version)?;
            }
        } else {
            println!("No migrations directory found in {:?}", directory);
        }
        
        Ok(())
    }

    pub fn analyze_migration_file(&mut self, file_path: &Path, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("Analyzing migration file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)?;
        
        // Extract migration name
        let migration_name = extract_migration_name(file_path);
        
        let mut migration = EnhancedRubyMigration {
            name: migration_name,
            file_path: file_path.to_string_lossy().to_string(),
            version: version.to_string(),
            raw_content: content.clone(),
            ..Default::default()
        };
        
        // Extract create_table statements
        self.extract_create_tables(&content, &mut migration);
        
        // Extract change_table statements
        self.extract_change_tables(&content, &mut migration);
        
        // Extract drop_table statements
        self.extract_drop_tables(&content, &mut migration);
        
        // Update the tables map with the latest schema
        self.update_tables_map(&migration);
        
        // Add migration to the collection
        self.migrations.push(migration);
        
        Ok(())
    }

    fn extract_create_tables(&self, content: &str, migration: &mut EnhancedRubyMigration) {
        // Extract create_table statements
        lazy_static! {
            static ref CREATE_TABLE_REGEX: Regex = 
                Regex::new(r"create_table\s+:([a-z0-9_]+)(?:,\s*(.+?))?\s+do\s*\|t\|(.*?)end").unwrap();
        }
        
        for captures in CREATE_TABLE_REGEX.captures_iter(content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            let table_options_str = captures.get(2).map(|m| m.as_str().to_string());
            let table_body = captures.get(3).unwrap().as_str();
            
            let mut table = MigrationTable {
                name: table_name,
                ..Default::default()
            };
            
            // Extract table options
            if let Some(options_str) = table_options_str {
                self.extract_options(&options_str, &mut table.options);
            }
            
            // Extract columns
            self.extract_columns(table_body, &mut table);
            
            // Extract indexes
            self.extract_indexes(table_body, &mut table);
            
            // Extract foreign keys
            self.extract_foreign_keys(table_body, &mut table);
            
            migration.tables_created.push(table);
        }
    }

    fn extract_change_tables(&self, content: &str, migration: &mut EnhancedRubyMigration) {
        // Extract change_table statements
        lazy_static! {
            static ref CHANGE_TABLE_REGEX: Regex = 
                Regex::new(r"(?:change_table|add_column|add_index|add_foreign_key|remove_column|remove_index|remove_foreign_key)\s+:([a-z0-9_]+)").unwrap();
        }
        
        let mut altered_tables = HashMap::new();
        
        for captures in CHANGE_TABLE_REGEX.captures_iter(content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            
            if !altered_tables.contains_key(&table_name) {
                // Create a new table entry
                let table = MigrationTable {
                    name: table_name.clone(),
                    ..Default::default()
                };
                
                altered_tables.insert(table_name, table);
            }
        }
        
        // Extract add_column statements
        lazy_static! {
            static ref ADD_COLUMN_REGEX: Regex = 
                Regex::new(r"add_column\s+:([a-z0-9_]+),\s*:([a-z0-9_]+),\s*:([a-z0-9_]+)(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in ADD_COLUMN_REGEX.captures_iter(content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            let column_name = captures.get(2).unwrap().as_str().to_string();
            let column_type = captures.get(3).unwrap().as_str().to_string();
            let column_options_str = captures.get(4).map(|m| m.as_str().to_string());
            
            let mut column = MigrationColumn {
                name: column_name,
                column_type,
                ..Default::default()
            };
            
            // Extract column options
            if let Some(options_str) = column_options_str {
                self.extract_options(&options_str, &mut column.options);
            }
            
            if let Some(table) = altered_tables.get_mut(&table_name) {
                table.columns.push(column);
            }
        }
        
        // Extract add_index statements
        lazy_static! {
            static ref ADD_INDEX_REGEX: Regex = 
                Regex::new(r"add_index\s+:([a-z0-9_]+),\s*(?:\[([^\]]+)\]|:([a-z0-9_]+))(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in ADD_INDEX_REGEX.captures_iter(content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            
            let columns = if let Some(columns_array) = captures.get(2) {
                // Parse array of columns
                columns_array.as_str().split(',')
                    .map(|s| s.trim().trim_matches('\'').trim_matches('"').trim_matches(':').to_string())
                    .collect()
            } else if let Some(single_column) = captures.get(3) {
                // Single column
                vec![single_column.as_str().to_string()]
            } else {
                Vec::new()
            };
            
            let index_options_str = captures.get(4).map(|m| m.as_str().to_string());
            
            let mut index = MigrationIndex {
                columns,
                unique: false,
                ..Default::default()
            };
            
            // Extract index options
            if let Some(options_str) = index_options_str {
                let mut options = HashMap::new();
                self.extract_options(&options_str, &mut options);
                
                // Check for unique option
                if let Some(unique) = options.get("unique") {
                    if unique == "true" {
                        index.unique = true;
                    }
                }
                
                // Check for name option
                if let Some(name) = options.get("name") {
                    index.name = Some(name.clone());
                }
                
                index.options = options;
            }
            
            if let Some(table) = altered_tables.get_mut(&table_name) {
                table.indexes.push(index);
            }
        }
        
        // Extract add_foreign_key statements
        lazy_static! {
            static ref ADD_FOREIGN_KEY_REGEX: Regex = 
                Regex::new(r"add_foreign_key\s+:([a-z0-9_]+),\s*:([a-z0-9_]+)(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in ADD_FOREIGN_KEY_REGEX.captures_iter(content) {
            let from_table = captures.get(1).unwrap().as_str().to_string();
            let to_table = captures.get(2).unwrap().as_str().to_string();
            let fk_options_str = captures.get(3).map(|m| m.as_str().to_string());
            
            let mut foreign_key = MigrationForeignKey {
                from_table: from_table.clone(),
                to_table,
                from_column: "id".to_string(), // Default
                to_column: "id".to_string(),   // Default
                ..Default::default()
            };
            
            // Extract foreign key options
            if let Some(options_str) = fk_options_str {
                let mut options = HashMap::new();
                self.extract_options(&options_str, &mut options);
                
                // Check for column option
                if let Some(column) = options.get("column") {
                    foreign_key.from_column = column.clone();
                }
                
                // Check for primary_key option
                if let Some(primary_key) = options.get("primary_key") {
                    foreign_key.to_column = primary_key.clone();
                }
                
                // Check for name option
                if let Some(name) = options.get("name") {
                    foreign_key.name = Some(name.clone());
                }
                
                // Check for on_delete option
                if let Some(on_delete) = options.get("on_delete") {
                    foreign_key.on_delete = Some(on_delete.clone());
                }
                
                // Check for on_update option
                if let Some(on_update) = options.get("on_update") {
                    foreign_key.on_update = Some(on_update.clone());
                }
            }
            
            if let Some(table) = altered_tables.get_mut(&from_table) {
                table.foreign_keys.push(foreign_key);
            }
        }
        
        // Add all altered tables to the migration
        for (_, table) in altered_tables {
            migration.tables_altered.push(table);
        }
    }

    fn extract_drop_tables(&self, content: &str, migration: &mut EnhancedRubyMigration) {
        // Extract drop_table statements
        lazy_static! {
            static ref DROP_TABLE_REGEX: Regex = 
                Regex::new(r"drop_table\s+:([a-z0-9_]+)").unwrap();
        }
        
        for captures in DROP_TABLE_REGEX.captures_iter(content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            migration.tables_dropped.push(table_name);
        }
    }

    fn extract_columns(&self, table_body: &str, table: &mut MigrationTable) {
        // Extract column definitions
        lazy_static! {
            static ref COLUMN_REGEX: Regex = 
                Regex::new(r"t\.([a-z_]+)\s+:([a-z0-9_]+)(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in COLUMN_REGEX.captures_iter(table_body) {
            let column_type = captures.get(1).unwrap().as_str().to_string();
            let column_name = captures.get(2).unwrap().as_str().to_string();
            let column_options_str = captures.get(3).map(|m| m.as_str().to_string());
            
            let mut column = MigrationColumn {
                name: column_name,
                column_type,
                ..Default::default()
            };
            
            // Extract column options
            if let Some(options_str) = column_options_str {
                self.extract_options(&options_str, &mut column.options);
            }
            
            table.columns.push(column);
        }
    }

    fn extract_indexes(&self, table_body: &str, table: &mut MigrationTable) {
        // Extract index definitions
        lazy_static! {
            static ref INDEX_REGEX: Regex = 
                Regex::new(r"t\.index\s+(?:\[([^\]]+)\]|:([a-z0-9_]+))(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in INDEX_REGEX.captures_iter(table_body) {
            let columns = if let Some(columns_array) = captures.get(1) {
                // Parse array of columns
                columns_array.as_str().split(',')
                    .map(|s| s.trim().trim_matches('\'').trim_matches('"').trim_matches(':').to_string())
                    .collect()
            } else if let Some(single_column) = captures.get(2) {
                // Single column
                vec![single_column.as_str().to_string()]
            } else {
                Vec::new()
            };
            
            let index_options_str = captures.get(3).map(|m| m.as_str().to_string());
            
            let mut index = MigrationIndex {
                columns,
                unique: false,
                ..Default::default()
            };
            
            // Extract index options
            if let Some(options_str) = index_options_str {
                let mut options = HashMap::new();
                self.extract_options(&options_str, &mut options);
                
                // Check for unique option
                if let Some(unique) = options.get("unique") {
                    if unique == "true" {
                        index.unique = true;
                    }
                }
                
                // Check for name option
                if let Some(name) = options.get("name") {
                    index.name = Some(name.clone());
                }
                
                index.options = options;
            }
            
            table.indexes.push(index);
        }
    }

    fn extract_foreign_keys(&self, table_body: &str, table: &mut MigrationTable) {
        // Extract foreign key definitions
        lazy_static! {
            static ref FOREIGN_KEY_REGEX: Regex = 
                Regex::new(r"t\.references\s+:([a-z0-9_]+)(?:,\s*(.+?))?").unwrap();
        }
        
        for captures in FOREIGN_KEY_REGEX.captures_iter(table_body) {
            let referenced_table = captures.get(1).unwrap().as_str().to_string();
            let options_str = captures.get(2).map(|m| m.as_str().to_string());
            
            let mut options = HashMap::new();
            if let Some(opts) = options_str {
                self.extract_options(&opts, &mut options);
            }
            
            // Check if foreign_key option is true
            if let Some(foreign_key) = options.get("foreign_key") {
                if foreign_key == "true" {
                    let from_column = format!("{}_id", referenced_table);
                    
                    let foreign_key = MigrationForeignKey {
                        from_table: table.name.clone(),
                        from_column,
                        to_table: referenced_table,
                        to_column: "id".to_string(),
                        on_delete: options.get("on_delete").cloned(),
                        on_update: options.get("on_update").cloned(),
                        ..Default::default()
                    };
                    
                    table.foreign_keys.push(foreign_key);
                }
            }
        }
    }

    fn extract_options(&self, options_str: &str, options: &mut HashMap<String, String>) {
        // Extract options from a string like "null: false, default: 0, index: true"
        lazy_static! {
            static ref OPTION_REGEX: Regex = 
                Regex::new(r"([a-z_]+):\s*(?::([a-z_]+)|['\"]([^'\"]+)['\"]|([^,]+))").unwrap();
        }
        
        for captures in OPTION_REGEX.captures_iter(options_str) {
            let key = captures.get(1).unwrap().as_str().to_string();
            let value = captures.get(2)
                .or_else(|| captures.get(3))
                .or_else(|| captures.get(4))
                .map_or("".to_string(), |m| m.as_str().trim().to_string());
            
            options.insert(key, value);
        }
    }

    fn update_tables_map(&mut self, migration: &EnhancedRubyMigration) {
        // Update the tables map with the latest schema
        
        // Add or update created tables
        for table in &migration.tables_created {
            self.tables.insert(table.name.clone(), table.clone());
        }
        
        // Update altered tables
        for altered_table in &migration.tables_altered {
            if let Some(existing_table) = self.tables.get_mut(&altered_table.name) {
                // Add new columns
                for column in &altered_table.columns {
                    // Check if column already exists
                    if !existing_table.columns.iter().any(|c| c.name == column.name) {
                        existing_table.columns.push(column.clone());
                    }
                }
                
                // Add new indexes
                for index in &altered_table.indexes {
                    // Check if index already exists
                    if !existing_table.indexes.iter().any(|i| i.columns == index.columns) {
                        existing_table.indexes.push(index.clone());
                    }
                }
                
                // Add new foreign keys
                for foreign_key in &altered_table.foreign_keys {
                    // Check if foreign key already exists
                    if !existing_table.foreign_keys.iter().any(|fk| 
                        fk.from_column == foreign_key.from_column && 
                        fk.to_table == foreign_key.to_table
                    ) {
                        existing_table.foreign_keys.push(foreign_key.clone());
                    }
                }
            } else {
                // Table doesn't exist yet, create it
                self.tables.insert(altered_table.name.clone(), altered_table.clone());
            }
        }
        
        // Remove dropped tables
        for table_name in &migration.tables_dropped {
            self.tables.remove(table_name);
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.tables)
    }
    
    pub fn to_migrations_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.migrations)
    }
    
    pub fn generate_schema_sql(&self) -> String {
        let mut sql = String::new();
        
        for (table_name, table) in &self.tables {
            sql.push_str(&format!("CREATE TABLE {} (\n", table_name));
            
            // Add columns
            for (i, column) in table.columns.iter().enumerate() {
                sql.push_str(&format!("  {} {}", column.name, column.column_type));
                
                // Add column options
                if let Some(null) = column.options.get("null") {
                    if null == "false" {
                        sql.push_str(" NOT NULL");
                    }
                }
                
                if let Some(default) = column.options.get("default") {
                    sql.push_str(&format!(" DEFAULT {}", default));
                }
                
                if i < table.columns.len() - 1 {
                    sql.push_str(",\n");
                } else {
                    sql.push_str("\n");
                }
            }
            
            sql.push_str(");\n\n");
            
            // Add indexes
            for index in &table.indexes {
                if index.unique {
                    sql.push_str(&format!("CREATE UNIQUE INDEX {} ON {} ({});\n", 
                        index.name.as_deref().unwrap_or(&format!("idx_{}_unique", table_name)),
                        table_name,
                        index.columns.join(", ")
                    ));
                } else {
                    sql.push_str(&format!("CREATE INDEX {} ON {} ({});\n", 
                        index.name.as_deref().unwrap_or(&format!("idx_{}", table_name)),
                        table_name,
                        index.columns.join(", ")
                    ));
                }
            }
            
            // Add foreign keys
            for foreign_key in &table.foreign_keys {
                sql.push_str(&format!("ALTER TABLE {} ADD CONSTRAINT {} FOREIGN KEY ({}) REFERENCES {} ({});\n", 
                    foreign_key.from_table,
                    foreign_key.name.as_deref().unwrap_or(&format!("fk_{}_{}", foreign_key.from_table, foreign_key.to_table)),
                    foreign_key.from_column,
                    foreign_key.to_table,
                    foreign_key.to_column
                ));
            }
            
            sql.push_str("\n");
        }
        
        sql
    }
}

// Helper function to find migrations directory
fn find_migrations_directory(directory: &Path) -> Option<PathBuf> {
    // Check if the directory itself is a migrations directory
    if directory.file_name().map_or(false, |name| name == "migrations" || name == "migrate") {
        return Some(directory.to_path_buf());
    }
    
    // Look for db/migrate or db/migrations
    let db_migrate = directory.join("db").join("migrate");
    if db_migrate.exists() && db_migrate.is_dir() {
        return Some(db_migrate);
    }
    
    let db_migrations = directory.join("db").join("migrations");
    if db_migrations.exists() && db_migrations.is_dir() {
        return Some(db_migrations);
    }
    
    // Search recursively for migrations directories
    for entry in fs::read_dir(directory).ok()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                if let Some(migrations_dir) = find_migrations_directory(&path) {
                    return Some(migrations_dir);
                }
            }
        }
    }
    
    None
}

// Helper function to extract migration version from file name
fn extract_migration_version(file_name: &str) -> Option<String> {
    lazy_static! {
        static ref VERSION_REGEX: Regex = 
            Regex::new(r"^(\d+)_").unwrap();
    }
    
    if let Some(captures) = VERSION_REGEX.captures(file_name) {
        return Some(captures.get(1).unwrap().as_str().to_string());
    }
    
    None
}

// Helper function to extract migration name from path
fn extract_migration_name(file_path: &Path) -> String {
    if let Some(file_name) = file_path.file_name() {
        let file_name_str = file_name.to_string_lossy();
        
        // Remove version prefix and .rb extension
        lazy_static! {
            static ref NAME_REGEX: Regex = 
                Regex::new(r"^\d+_([a-z0-9_]+)\.rb$").unwrap();
        }
        
        if let Some(captures) = NAME_REGEX.captures(&file_name_str) {
            return captures.get(1).unwrap().as_str().to_string();
        }
        
        return file_name_str.to_string();
    }
    "unknown".to_string()
}
