use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub column_type: String,
    pub nullable: bool,
    pub default: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ForeignKey {
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub primary_key: Option<String>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<ForeignKey>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Migration {
    pub version: String,
    pub name: String,
    pub operations: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DatabaseSchemaAnalyzer {
    pub tables: HashMap<String, Table>,
    pub migrations: Vec<Migration>,
}

impl DatabaseSchemaAnalyzer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn analyze(&self, base_dir: &PathBuf) -> Result<String, String> {
        let mut analyzer = DatabaseSchemaAnalyzer::default();
        
        // Extract schema from schema.rb or structure.sql
        analyzer.extract_schema(base_dir);
        
        // Extract migrations
        analyzer.extract_migrations(base_dir);
        
        // Extract foreign keys
        analyzer.extract_foreign_keys(base_dir);
        
        match serde_json::to_string_pretty(&analyzer) {
            Ok(json) => Ok(json),
            Err(e) => Err(format!("Failed to serialize DatabaseSchemaAnalyzer: {}", e)),
        }
    }
    
    fn extract_schema(&mut self, base_dir: &PathBuf) {
        // Look for schema.rb
        let schema_path = base_dir.join("db").join("schema.rb");
        if schema_path.exists() {
            if let Ok(content) = fs::read_to_string(&schema_path) {
                // Extract table definitions
                let table_regex = Regex::new(r#"create_table\s+["']([^"']+)["'](?:,\s*([^\n]+))?\s+do\s+\|t\|(.*?)\s+end"#).unwrap();
                
                for table_capture in table_regex.captures_iter(&content) {
                    if let Some(table_name) = table_capture.get(1) {
                        let table_name_str = table_name.as_str();
                        let mut table = Table {
                            name: table_name_str.to_string(),
                            ..Default::default()
                        };
                        
                        // Extract table options
                        if let Some(options) = table_capture.get(2) {
                            let options_str = options.as_str();
                            
                            // Extract primary key
                            if options_str.contains("primary_key:") {
                                let pk_regex = Regex::new(r#"primary_key:\s*["']([^"']+)["']"#).unwrap();
                                
                                if let Some(pk_capture) = pk_regex.captures(options_str) {
                                    if let Some(pk) = pk_capture.get(1) {
                                        table.primary_key = Some(pk.as_str().to_string());
                                    }
                                }
                            }
                        }
                        
                        // Extract columns
                        if let Some(columns_block) = table_capture.get(3) {
                            let columns_str = columns_block.as_str();
                            let column_regex = Regex::new(r#"t\.(\w+)\s+["']([^"']+)["'](?:,\s*([^\n]+))?"#).unwrap();
                            
                            for column_capture in column_regex.captures_iter(columns_str) {
                                if let (Some(column_type), Some(column_name)) = (column_capture.get(1), column_capture.get(2)) {
                                    let mut column = Column {
                                        name: column_name.as_str().to_string(),
                                        column_type: column_type.as_str().to_string(),
                                        nullable: true,
                                        default: None,
                                    };
                                    
                                    // Extract column options
                                    if let Some(options) = column_capture.get(3) {
                                        let options_str = options.as_str();
                                        
                                        // Check if column is nullable
                                        if options_str.contains("null: false") {
                                            column.nullable = false;
                                        }
                                        
                                        // Extract default value
                                        if options_str.contains("default:") {
                                            let default_regex = Regex::new(r#"default:\s*([^,\s]+)"#).unwrap();
                                            
                                            if let Some(default_capture) = default_regex.captures(options_str) {
                                                if let Some(default_value) = default_capture.get(1) {
                                                    column.default = Some(default_value.as_str().to_string());
                                                }
                                            }
                                        }
                                    }
                                    
                                    table.columns.push(column);
                                }
                            }
                        }
                        
                        self.tables.insert(table_name_str.to_string(), table);
                    }
                }
                
                // Extract indexes
                for (table_name_str, table) in &mut self.tables {
                    let index_regex = Regex::new(&format!(r#"add_index\s+["']{}["'],\s+\[([^\]]+)\](?:,\s*([^\n]+))?"#, table_name_str)).unwrap();
                    
                    for index_capture in index_regex.captures_iter(&content) {
                        if let Some(columns) = index_capture.get(1) {
                            let columns_str = columns.as_str();
                            let column_names: Vec<String> = columns_str.split(',')
                                .map(|s| s.trim().trim_matches('"').trim_matches('\'').to_string())
                                .collect();
                            
                            let mut index = Index {
                                name: format!("index_{}_on_{}", table_name_str, columns_str.replace(",", "_").replace("\"", "").replace("'", "")),
                                columns: column_names,
                                unique: false,
                            };
                            
                            // Extract index options
                            if let Some(options) = index_capture.get(2) {
                                let options_str = options.as_str();
                                
                                // Check if index is unique
                                if options_str.contains("unique: true") {
                                    index.unique = true;
                                }
                                
                                // Extract index name if specified
                                if options_str.contains("name:") {
                                    let name_regex = Regex::new(r#"name:\s*["']([^"']+)["']"#).unwrap();
                                    
                                    if let Some(name_capture) = name_regex.captures(options_str) {
                                        if let Some(name) = name_capture.get(1) {
                                            index.name = name.as_str().to_string();
                                        }
                                    }
                                }
                            }
                            
                            table.indexes.push(index);
                        }
                    }
                }
            }
        }
    }
    
    fn extract_migrations(&mut self, base_dir: &PathBuf) {
        // Look for migration files
        let migrations_dir = base_dir.join("db").join("migrate");
        if migrations_dir.exists() {
            for entry in WalkDir::new(&migrations_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                    if let Some(file_name) = path.file_name() {
                        if let Some(file_name_str) = file_name.to_str() {
                            // Parse migration version and name
                            let parts: Vec<&str> = file_name_str.split('_').collect();
                            if !parts.is_empty() {
                                let version = parts[0].to_string();
                                let name = parts[1..].join("_").replace(".rb", "");
                                
                                // Extract operations
                                let mut operations = Vec::new();
                                if let Ok(content) = fs::read_to_string(path) {
                                    // Look for common migration operations
                                    if content.contains("create_table") {
                                        operations.push("create_table".to_string());
                                    }
                                    if content.contains("add_column") {
                                        operations.push("add_column".to_string());
                                    }
                                    if content.contains("remove_column") {
                                        operations.push("remove_column".to_string());
                                    }
                                    if content.contains("add_index") {
                                        operations.push("add_index".to_string());
                                    }
                                    if content.contains("remove_index") {
                                        operations.push("remove_index".to_string());
                                    }
                                    if content.contains("drop_table") {
                                        operations.push("drop_table".to_string());
                                    }
                                    if content.contains("rename_") {
                                        operations.push("rename".to_string());
                                    }
                                    if content.contains("add_foreign_key") {
                                        operations.push("add_foreign_key".to_string());
                                    }
                                    if content.contains("remove_foreign_key") {
                                        operations.push("remove_foreign_key".to_string());
                                    }
                                }
                                
                                self.migrations.push(Migration {
                                    version,
                                    name,
                                    operations,
                                });
                            }
                        }
                    }
                }
            }
            
            // Sort migrations by version
            self.migrations.sort_by(|a, b| a.version.cmp(&b.version));
        }
    }
    
    fn extract_foreign_keys(&mut self, base_dir: &PathBuf) {
        // Look for foreign key definitions in schema.rb
        let schema_path = base_dir.join("db").join("schema.rb");
        if schema_path.exists() {
            if let Ok(content) = fs::read_to_string(&schema_path) {
                let fk_regex = Regex::new(r#"add_foreign_key\s+["']([^"']+)["'],\s+["']([^"']+)["'](?:,\s*([^\n]+))?"#).unwrap();
                
                for fk_capture in fk_regex.captures_iter(&content) {
                    if let (Some(from_table), Some(to_table)) = (fk_capture.get(1), fk_capture.get(2)) {
                        let from_table_str = from_table.as_str();
                        let to_table_str = to_table.as_str();
                        
                        let mut from_column = format!("{}_id", to_table_str.trim_end_matches('s'));
                        let mut to_column = "id".to_string();
                        
                        // Extract custom column names if specified
                        if let Some(options) = fk_capture.get(3) {
                            let options_str = options.as_str();
                            
                            // Extract from column
                            if options_str.contains("column:") {
                                let column_regex = Regex::new(r#"column:\s*["']([^"']+)["']"#).unwrap();
                                
                                if let Some(column_capture) = column_regex.captures(options_str) {
                                    if let Some(column) = column_capture.get(1) {
                                        from_column = column.as_str().to_string();
                                    }
                                }
                            }
                            
                            // Extract to column
                            if options_str.contains("primary_key:") {
                                let pk_regex = Regex::new(r#"primary_key:\s*["']([^"']+)["']"#).unwrap();
                                
                                if let Some(pk_capture) = pk_regex.captures(options_str) {
                                    if let Some(pk) = pk_capture.get(1) {
                                        to_column = pk.as_str().to_string();
                                    }
                                }
                            }
                        }
                        
                        // Add foreign key to the table
                        if let Some(table) = self.tables.get_mut(from_table_str) {
                            table.foreign_keys.push(ForeignKey {
                                from_column,
                                to_table: to_table_str.to_string(),
                                to_column,
                            });
                        }
                    }
                }
                
                // Also look for belongs_to associations in model files
                let models_dir = base_dir.join("app").join("models");
                if models_dir.exists() {
                    for entry in WalkDir::new(&models_dir)
                        .into_iter()
                        .filter_map(|e| e.ok())
                    {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "rb") {
                            if let Some(file_name) = path.file_name() {
                                if let Some(file_name_str) = file_name.to_str() {
                                    let model_name = file_name_str.replace(".rb", "");
                                    let table_name = model_name.to_lowercase() + "s"; // Simple pluralization
                                    
                                    if let Ok(content) = fs::read_to_string(path) {
                                        let belongs_to_regex = Regex::new(r#"belongs_to\s+:([^,\s]+)(?:,\s*([^\n]+))?"#).unwrap();
                                        
                                        for bt_capture in belongs_to_regex.captures_iter(&content) {
                                            if let Some(association) = bt_capture.get(1) {
                                                let association_str = association.as_str();
                                                let target_table = association_str.to_lowercase() + "s"; // Simple pluralization
                                                
                                                let mut from_column = format!("{}_id", association_str);
                                                let to_column = "id".to_string();
                                                
                                                // Extract custom foreign key if specified
                                                if let Some(options) = bt_capture.get(2) {
                                                    let options_str = options.as_str();
                                                    
                                                    if options_str.contains("foreign_key:") {
                                                        let fk_regex = Regex::new(r#"foreign_key:\s*:([^,\s]+)"#).unwrap();
                                                        
                                                        if let Some(fk_capture) = fk_regex.captures(options_str) {
                                                            if let Some(fk) = fk_capture.get(1) {
                                                                from_column = fk.as_str().to_string();
                                                            }
                                                        }
                                                    }
                                                }
                                                
                                                // Add foreign key to the table if not already present
                                                if let Some(table) = self.tables.get_mut(&table_name) {
                                                    if !table.foreign_keys.iter().any(|fk| 
                                                        fk.from_column == from_column && 
                                                        fk.to_table == target_table
                                                    ) {
                                                        table.foreign_keys.push(ForeignKey {
                                                            from_column,
                                                            to_table: target_table,
                                                            to_column,
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
