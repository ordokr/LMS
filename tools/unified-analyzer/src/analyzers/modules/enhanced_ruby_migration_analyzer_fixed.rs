use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct EnhancedRubyColumn {
    pub name: String,
    pub column_type: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyIndex {
    pub name: Option<String>,
    pub columns: Vec<String>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyForeignKey {
    pub from_table: String,
    pub from_column: String,
    pub to_table: String,
    pub to_column: String,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyTable {
    pub name: String,
    pub columns: Vec<EnhancedRubyColumn>,
    pub indexes: Vec<EnhancedRubyIndex>,
    pub foreign_keys: Vec<EnhancedRubyForeignKey>,
    pub options: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct EnhancedRubyMigration {
    pub name: String,
    pub version: Option<String>,
    pub file_path: String,
    pub tables: Vec<EnhancedRubyTable>,
    pub content: String,
}

pub struct EnhancedRubyMigrationAnalyzer {
    pub migrations: Vec<EnhancedRubyMigration>,
    pub tables: HashMap<String, EnhancedRubyTable>,
}

impl EnhancedRubyMigrationAnalyzer {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            tables: HashMap::new(),
        }
    }
    
    pub fn analyze_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(format!("Directory does not exist: {:?}", dir_path).into());
        }
        
        for entry in fs::read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.analyze_directory(&path)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rb" && path.to_string_lossy().contains("migrate") {
                    self.analyze_file(&path)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path)?;
        
        // Extract migration name and version from file name
        let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
        let parts: Vec<&str> = file_name.split('_').collect();
        
        let version = if parts.len() > 0 && parts[0].parse::<u64>().is_ok() {
            Some(parts[0].to_string())
        } else {
            None
        };
        
        let name = if version.is_some() {
            parts[1..].join("_").replace(".rb", "")
        } else {
            file_name.replace(".rb", "")
        };
        
        let mut migration = EnhancedRubyMigration {
            name,
            version,
            file_path: file_path.to_string_lossy().to_string(),
            tables: Vec::new(),
            content,
        };
        
        // Extract tables
        self.extract_tables(&mut migration);
        
        // Add tables to global tables map
        for table in &migration.tables {
            self.tables.insert(table.name.clone(), table.clone());
        }
        
        self.migrations.push(migration);
        
        Ok(())
    }
    
    fn extract_tables(&self, migration: &mut EnhancedRubyMigration) {
        // Extract create_table blocks
        lazy_static! {
            static ref CREATE_TABLE_REGEX: Regex =
                Regex::new(r#"create_table\s+:([a-z0-9_]+)(?:,\s*(.+?))?\s+do\s*\|t\|(.*?)end"#).unwrap();
        }
        
        for captures in CREATE_TABLE_REGEX.captures_iter(&migration.content) {
            let table_name = captures.get(1).unwrap().as_str().to_string();
            
            let mut table = EnhancedRubyTable {
                name: table_name,
                columns: Vec::new(),
                indexes: Vec::new(),
                foreign_keys: Vec::new(),
                options: HashMap::new(),
            };
            
            // Extract table options
            if let Some(options_str) = captures.get(2) {
                let options = options_str.as_str();
                
                // Store all options in the options map
                lazy_static! {
                    static ref OPTION_REGEX: Regex =
                        Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
                }
                for option_match in OPTION_REGEX.captures_iter(options) {
                    let key = option_match.get(1).unwrap().as_str().to_string();
                    let value = option_match.get(2)
                        .or_else(|| option_match.get(3))
                        .or_else(|| option_match.get(4))
                        .unwrap()
                        .as_str()
                        .to_string();
                    
                    table.options.insert(key, value);
                }
            }
            
            // Extract columns
            if let Some(table_body) = captures.get(3) {
                let table_body_str = table_body.as_str();
                
                // Extract column definitions
                lazy_static! {
                    static ref COLUMN_REGEX: Regex =
                        Regex::new(r#"t\.([a-z_]+)\s+:([a-z0-9_]+)(?:,\s*(.+?))?"#).unwrap();
                }
                
                for column_match in COLUMN_REGEX.captures_iter(table_body_str) {
                    let column_type = column_match.get(1).unwrap().as_str().to_string();
                    let column_name = column_match.get(2).unwrap().as_str().to_string();
                    
                    let mut column = EnhancedRubyColumn {
                        name: column_name,
                        column_type,
                        options: HashMap::new(),
                    };
                    
                    // Extract column options
                    if let Some(options_str) = column_match.get(3) {
                        let options = options_str.as_str();
                        
                        // Store all options in the options map
                        lazy_static! {
                            static ref OPTION_REGEX: Regex =
                                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
                        }
                        for option_match in OPTION_REGEX.captures_iter(options) {
                            let key = option_match.get(1).unwrap().as_str().to_string();
                            let value = option_match.get(2)
                                .or_else(|| option_match.get(3))
                                .or_else(|| option_match.get(4))
                                .unwrap()
                                .as_str()
                                .to_string();
                            
                            column.options.insert(key, value);
                        }
                    }
                    
                    table.columns.push(column);
                }
                
                // Extract indexes
                lazy_static! {
                    static ref INDEX_REGEX: Regex =
                        Regex::new(r#"t\.index\s+(?:\[([^\]]+)\]|:([a-z0-9_]+))(?:,\s*(.+?))?"#).unwrap();
                }
                
                for index_match in INDEX_REGEX.captures_iter(table_body_str) {
                    let mut columns = Vec::new();
                    
                    if let Some(columns_list) = index_match.get(1) {
                        // Multiple columns
                        for column in columns_list.as_str().split(',') {
                            columns.push(column.trim().trim_matches(':').trim_matches('\'').trim_matches('"').to_string());
                        }
                    } else if let Some(column) = index_match.get(2) {
                        // Single column
                        columns.push(column.as_str().to_string());
                    }
                    
                    let mut index = EnhancedRubyIndex {
                        name: None,
                        columns,
                        options: HashMap::new(),
                    };
                    
                    // Extract index options
                    if let Some(options_str) = index_match.get(3) {
                        let options = options_str.as_str();
                        
                        // Extract name
                        if options.contains("name:") {
                            let name_start = options.find("name:").unwrap() + 5;
                            let name_end = options[name_start..].find(',').unwrap_or(options.len() - name_start);
                            let name = &options[name_start..name_start + name_end].trim().trim_matches(':').trim_matches('\'').trim_matches('"');
                            index.name = Some(name.to_string());
                        }
                        
                        // Store all options in the options map
                        lazy_static! {
                            static ref OPTION_REGEX: Regex =
                                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
                        }
                        for option_match in OPTION_REGEX.captures_iter(options) {
                            let key = option_match.get(1).unwrap().as_str().to_string();
                            let value = option_match.get(2)
                                .or_else(|| option_match.get(3))
                                .or_else(|| option_match.get(4))
                                .unwrap()
                                .as_str()
                                .to_string();
                            
                            index.options.insert(key, value);
                        }
                    }
                    
                    table.indexes.push(index);
                }
            }
            
            // Extract foreign keys
            lazy_static! {
                static ref FOREIGN_KEY_REGEX: Regex =
                    Regex::new(r#"add_foreign_key\s+:([a-z0-9_]+),\s*:([a-z0-9_]+)(?:,\s*(.+?))?"#).unwrap();
            }
            
            for fk_match in FOREIGN_KEY_REGEX.captures_iter(&migration.content) {
                let from_table = fk_match.get(1).unwrap().as_str().to_string();
                
                if from_table == table.name {
                    let to_table = fk_match.get(2).unwrap().as_str().to_string();
                    
                    let mut foreign_key = EnhancedRubyForeignKey {
                        from_table,
                        from_column: "id".to_string(), // Default
                        to_table,
                        to_column: "id".to_string(),   // Default
                        options: HashMap::new(),
                    };
                    
                    // Extract foreign key options
                    if let Some(options_str) = fk_match.get(3) {
                        let options = options_str.as_str();
                        
                        // Extract column
                        if options.contains("column:") {
                            let column_start = options.find("column:").unwrap() + 7;
                            let column_end = options[column_start..].find(',').unwrap_or(options.len() - column_start);
                            let column = &options[column_start..column_start + column_end].trim().trim_matches(':').trim_matches('\'').trim_matches('"');
                            foreign_key.from_column = column.to_string();
                        }
                        
                        // Extract primary_key
                        if options.contains("primary_key:") {
                            let pk_start = options.find("primary_key:").unwrap() + 12;
                            let pk_end = options[pk_start..].find(',').unwrap_or(options.len() - pk_start);
                            let pk = &options[pk_start..pk_start + pk_end].trim().trim_matches(':').trim_matches('\'').trim_matches('"');
                            foreign_key.to_column = pk.to_string();
                        }
                        
                        // Store all options in the options map
                        lazy_static! {
                            static ref OPTION_REGEX: Regex =
                                Regex::new(r#"([a-z_]+):\s*(?::([a-z_]+)|['"]([^'"]+)['"]|([^,]+))"#).unwrap();
                        }
                        for option_match in OPTION_REGEX.captures_iter(options) {
                            let key = option_match.get(1).unwrap().as_str().to_string();
                            let value = option_match.get(2)
                                .or_else(|| option_match.get(3))
                                .or_else(|| option_match.get(4))
                                .unwrap()
                                .as_str()
                                .to_string();
                            
                            foreign_key.options.insert(key, value);
                        }
                    }
                    
                    table.foreign_keys.push(foreign_key);
                }
            }
            
            migration.tables.push(table);
        }
    }
}
