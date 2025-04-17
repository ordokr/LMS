use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use std::fmt;

/// Represents a database table in HelixDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbTable {
    /// Table name
    pub name: String,
    /// Table fields
    pub fields: Vec<HelixDbField>,
    /// Table indexes
    pub indexes: Vec<HelixDbIndex>,
    /// Table relationships
    pub relationships: Vec<HelixDbRelationship>,
    /// Source system (canvas, discourse, ordo)
    pub source: String,
}

/// Represents a field in a HelixDB table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: String,
    /// Whether the field is nullable
    pub nullable: bool,
    /// Default value
    pub default: Option<String>,
    /// Whether the field is a primary key
    pub primary_key: bool,
    /// Whether the field is unique
    pub unique: bool,
}

/// Represents an index in a HelixDB table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbIndex {
    /// Index name
    pub name: String,
    /// Fields included in the index
    pub fields: Vec<String>,
    /// Whether the index is unique
    pub unique: bool,
}

/// Represents a relationship between HelixDB tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbRelationship {
    /// Relationship type (one-to-one, one-to-many, many-to-many)
    pub relationship_type: String,
    /// Foreign table name
    pub foreign_table: String,
    /// Local field name
    pub local_field: String,
    /// Foreign field name
    pub foreign_field: String,
}

/// Represents a source system type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SourceSystemType {
    /// Canvas LMS
    Canvas,
    /// Discourse forum
    Discourse,
    /// Ordo LMS
    Ordo,
    /// Moodle LMS
    Moodle,
    /// WordPress CMS
    WordPress,
    /// Custom source system
    Custom(String),
}

impl fmt::Display for SourceSystemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SourceSystemType::Canvas => write!(f, "canvas"),
            SourceSystemType::Discourse => write!(f, "discourse"),
            SourceSystemType::Ordo => write!(f, "ordo"),
            SourceSystemType::Moodle => write!(f, "moodle"),
            SourceSystemType::WordPress => write!(f, "wordpress"),
            SourceSystemType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Represents a source system for database schema extraction
pub trait SourceSystem: std::any::Any {
    /// Get the type of the source system
    fn get_type(&self) -> SourceSystemType;

    /// Extract database schema from the source system
    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>>;

    /// Get the name of the source system
    fn get_name(&self) -> String {
        self.get_type().to_string()
    }

    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;

    /// Convert to mutable Any for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Cache entry for a file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileCache {
    /// Last modified time
    last_modified: u64,
    /// Extracted tables
    tables: Vec<HelixDbTable>,
}

/// HelixDB Integration Analyzer
pub struct HelixDbIntegrationAnalyzer {
    /// Extracted tables by source system
    tables: HashMap<String, Vec<HelixDbTable>>,
    /// Table mappings between systems
    mappings: Vec<HelixDbTableMapping>,
    /// Source systems
    source_systems: Vec<Box<dyn SourceSystem>>,
    /// File cache to avoid re-analyzing unchanged files
    file_cache: HashMap<String, FileCache>,
    /// Whether to use caching
    use_cache: bool,
}

/// Represents a mapping between tables from different systems
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbTableMapping {
    /// Source table
    pub source_table: String,
    /// Source system
    pub source_system: String,
    /// Target table
    pub target_table: String,
    /// Target system
    pub target_system: String,
    /// Field mappings
    pub field_mappings: Vec<HelixDbFieldMapping>,
    /// Mapping confidence (0-100)
    pub confidence: u8,
}

/// Represents a mapping between fields from different tables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixDbFieldMapping {
    /// Source field
    pub source_field: String,
    /// Target field
    pub target_field: String,
    /// Mapping confidence (0-100)
    pub confidence: u8,
}

impl HelixDbIntegrationAnalyzer {
    /// Create a new HelixDbIntegrationAnalyzer
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            mappings: Vec::new(),
            source_systems: Vec::new(),
            file_cache: HashMap::new(),
            use_cache: true,
        }
    }

    /// Create a new HelixDbIntegrationAnalyzer with caching disabled
    pub fn new_without_cache() -> Self {
        let mut analyzer = Self::new();
        analyzer.use_cache = false;
        analyzer
    }

    /// Enable or disable caching
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;

        // Update caching for all source systems
        for source_system in &mut self.source_systems {
            // Use a match statement instead of if-else chain with downcasting
            match source_system.get_type() {
                SourceSystemType::Canvas => {
                    if let Some(canvas) = source_system.as_any_mut().downcast_mut::<crate::analyzers::modules::source_systems::canvas::CanvasSourceSystem>() {
                        canvas.set_use_cache(use_cache);
                    }
                },
                SourceSystemType::Discourse => {
                    if let Some(discourse) = source_system.as_any_mut().downcast_mut::<crate::analyzers::modules::source_systems::discourse::DiscourseSourceSystem>() {
                        discourse.set_use_cache(use_cache);
                    }
                },
                SourceSystemType::Ordo => {
                    if let Some(ordo) = source_system.as_any_mut().downcast_mut::<crate::analyzers::modules::source_systems::ordo::OrdoSourceSystem>() {
                        ordo.set_use_cache(use_cache);
                    }
                },
                SourceSystemType::Moodle => {
                    if let Some(moodle) = source_system.as_any_mut().downcast_mut::<crate::analyzers::modules::source_systems::moodle::MoodleSourceSystem>() {
                        moodle.set_use_cache(use_cache);
                    }
                },
                SourceSystemType::WordPress => {
                    if let Some(wordpress) = source_system.as_any_mut().downcast_mut::<crate::analyzers::modules::source_systems::wordpress::WordPressSourceSystem>() {
                        wordpress.set_use_cache(use_cache);
                    }
                },
                _ => {}
            }
        }
    }

    /// Clear the file cache
    pub fn clear_cache(&mut self) {
        self.file_cache.clear();
    }

    /// Register a source system
    pub fn register_source_system<T: SourceSystem + 'static>(&mut self, source_system: T) {
        self.source_systems.push(Box::new(source_system));
    }

    /// Extract database schema from Canvas codebase
    pub fn extract_canvas_schema(&mut self, canvas_path: &Path) -> Result<()> {
        // Check if we already have a Canvas source system
        let canvas_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Canvas);

        if canvas_system.is_none() {
            // Register a new Canvas source system
            use crate::analyzers::modules::source_systems::canvas::CanvasSourceSystem;
            let canvas_system = CanvasSourceSystem::new();
            self.register_source_system(canvas_system);
        }

        // Find the Canvas source system
        let canvas_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Canvas)
            .ok_or_else(|| anyhow!("Canvas source system not found"))?;

        // Extract schema
        let tables = canvas_system.extract_schema(canvas_path)?;

        // Store the extracted tables
        self.tables.insert("canvas".to_string(), tables);

        Ok(())
    }

    /// Extract database schema from Discourse codebase
    pub fn extract_discourse_schema(&mut self, discourse_path: &Path) -> Result<()> {
        // Check if we already have a Discourse source system
        let discourse_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Discourse);

        if discourse_system.is_none() {
            // Register a new Discourse source system
            use crate::analyzers::modules::source_systems::discourse::DiscourseSourceSystem;
            let discourse_system = DiscourseSourceSystem::new();
            self.register_source_system(discourse_system);
        }

        // Find the Discourse source system
        let discourse_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Discourse)
            .ok_or_else(|| anyhow!("Discourse source system not found"))?;

        // Extract schema
        let tables = discourse_system.extract_schema(discourse_path)?;

        // Store the extracted tables
        self.tables.insert("discourse".to_string(), tables);

        Ok(())
    }

    /// Extract database schema from Ordo codebase
    pub fn extract_ordo_schema(&mut self, ordo_path: &Path) -> Result<()> {
        // Check if we already have an Ordo source system
        let ordo_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Ordo);

        if ordo_system.is_none() {
            // Register a new Ordo source system
            use crate::analyzers::modules::source_systems::ordo::OrdoSourceSystem;
            let ordo_system = OrdoSourceSystem::new();
            self.register_source_system(ordo_system);
        }

        // Find the Ordo source system
        let ordo_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Ordo)
            .ok_or_else(|| anyhow!("Ordo source system not found"))?;

        // Extract schema
        let tables = ordo_system.extract_schema(ordo_path)?;

        // Store the extracted tables
        self.tables.insert("ordo".to_string(), tables);

        Ok(())
    }

    /// Extract database schema from Moodle codebase
    pub fn extract_moodle_schema(&mut self, moodle_path: &Path) -> Result<()> {
        // Check if we already have a Moodle source system
        let moodle_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Moodle);

        if moodle_system.is_none() {
            // Register a new Moodle source system
            use crate::analyzers::modules::source_systems::moodle::MoodleSourceSystem;
            let moodle_system = MoodleSourceSystem::new();
            self.register_source_system(moodle_system);
        }

        // Find the Moodle source system
        let moodle_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::Moodle)
            .ok_or_else(|| anyhow!("Moodle source system not found"))?;

        // Extract schema
        let tables = moodle_system.extract_schema(moodle_path)?;

        // Store the extracted tables
        self.tables.insert("moodle".to_string(), tables);

        Ok(())
    }

    /// Extract database schema from WordPress codebase
    pub fn extract_wordpress_schema(&mut self, wordpress_path: &Path) -> Result<()> {
        // Check if we already have a WordPress source system
        let wordpress_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::WordPress);

        if wordpress_system.is_none() {
            // Register a new WordPress source system
            use crate::analyzers::modules::source_systems::wordpress::WordPressSourceSystem;
            let wordpress_system = WordPressSourceSystem::new();
            self.register_source_system(wordpress_system);
        }

        // Find the WordPress source system
        let wordpress_system = self.source_systems.iter().find(|s| s.get_type() == SourceSystemType::WordPress)
            .ok_or_else(|| anyhow!("WordPress source system not found"))?;

        // Extract schema
        let tables = wordpress_system.extract_schema(wordpress_path)?;

        // Store the extracted tables
        self.tables.insert("wordpress".to_string(), tables);

        Ok(())
    }

    /// Generate mappings between tables from different systems
    pub fn generate_mappings(&mut self) -> Result<()> {
        println!("Generating table mappings...");

        let canvas_tables = self.tables.get("canvas").cloned().unwrap_or_default();
        let discourse_tables = self.tables.get("discourse").cloned().unwrap_or_default();
        let ordo_tables = self.tables.get("ordo").cloned().unwrap_or_default();

        // Map Canvas tables to Ordo tables
        for canvas_table in &canvas_tables {
            for ordo_table in &ordo_tables {
                let confidence = self.calculate_table_mapping_confidence(canvas_table, ordo_table);
                if confidence > 50 {
                    let field_mappings = self.generate_field_mappings(canvas_table, ordo_table);
                    self.mappings.push(HelixDbTableMapping {
                        source_table: canvas_table.name.clone(),
                        source_system: "canvas".to_string(),
                        target_table: ordo_table.name.clone(),
                        target_system: "ordo".to_string(),
                        field_mappings,
                        confidence,
                    });
                }
            }
        }

        // Map Discourse tables to Ordo tables
        for discourse_table in &discourse_tables {
            for ordo_table in &ordo_tables {
                let confidence = self.calculate_table_mapping_confidence(discourse_table, ordo_table);
                if confidence > 50 {
                    let field_mappings = self.generate_field_mappings(discourse_table, ordo_table);
                    self.mappings.push(HelixDbTableMapping {
                        source_table: discourse_table.name.clone(),
                        source_system: "discourse".to_string(),
                        target_table: ordo_table.name.clone(),
                        target_system: "ordo".to_string(),
                        field_mappings,
                        confidence,
                    });
                }
            }
        }

        println!("Generated {} table mappings", self.mappings.len());

        Ok(())
    }

    /// Generate a JSON report of table mappings
    pub fn generate_mapping_report(&self) -> Result<String> {
        let report = serde_json::to_string_pretty(&self.mappings)?;
        Ok(report)
    }

    /// Generate a Markdown report of table mappings
    pub fn generate_mapping_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# HelixDB Integration Plan\n\n");
        markdown.push_str("## Table Mappings\n\n");

        // Group mappings by target table
        let mut mappings_by_target: HashMap<String, Vec<&HelixDbTableMapping>> = HashMap::new();
        for mapping in &self.mappings {
            mappings_by_target
                .entry(mapping.target_table.clone())
                .or_default()
                .push(mapping);
        }

        for (target_table, mappings) in mappings_by_target {
            markdown.push_str(&format!("### {}\n\n", target_table));
            markdown.push_str("| Source System | Source Table | Confidence | Field Mappings |\n");
            markdown.push_str("|--------------|-------------|------------|---------------|\n");

            for mapping in mappings {
                let field_mappings_str = mapping
                    .field_mappings
                    .iter()
                    .map(|fm| format!("{} → {} ({}%)", fm.source_field, fm.target_field, fm.confidence))
                    .collect::<Vec<_>>()
                    .join("<br>");

                markdown.push_str(&format!(
                    "| {} | {} | {}% | {} |\n",
                    mapping.source_system, mapping.source_table, mapping.confidence, field_mappings_str
                ));
            }

            markdown.push_str("\n");
        }

        markdown.push_str("## Integration Recommendations\n\n");
        markdown.push_str("Based on the analysis of the database schemas, here are the recommendations for integrating Canvas and Discourse data into HelixDB:\n\n");

        // Add recommendations based on mappings
        let high_confidence_mappings: Vec<_> = self.mappings.iter().filter(|m| m.confidence > 80).collect();
        let medium_confidence_mappings: Vec<_> = self.mappings.iter().filter(|m| m.confidence > 60 && m.confidence <= 80).collect();
        let low_confidence_mappings: Vec<_> = self.mappings.iter().filter(|m| m.confidence <= 60).collect();

        markdown.push_str("### High Priority Integrations\n\n");
        if high_confidence_mappings.is_empty() {
            markdown.push_str("No high-confidence mappings found.\n\n");
        } else {
            for mapping in high_confidence_mappings {
                markdown.push_str(&format!(
                    "- Integrate **{}** from {} into **{}** ({}% confidence)\n",
                    mapping.source_table, mapping.source_system, mapping.target_table, mapping.confidence
                ));
            }
            markdown.push_str("\n");
        }

        markdown.push_str("### Medium Priority Integrations\n\n");
        if medium_confidence_mappings.is_empty() {
            markdown.push_str("No medium-confidence mappings found.\n\n");
        } else {
            for mapping in medium_confidence_mappings {
                markdown.push_str(&format!(
                    "- Integrate **{}** from {} into **{}** ({}% confidence)\n",
                    mapping.source_table, mapping.source_system, mapping.target_table, mapping.confidence
                ));
            }
            markdown.push_str("\n");
        }

        markdown.push_str("### Low Priority Integrations\n\n");
        if low_confidence_mappings.is_empty() {
            markdown.push_str("No low-confidence mappings found.\n\n");
        } else {
            for mapping in low_confidence_mappings {
                markdown.push_str(&format!(
                    "- Integrate **{}** from {} into **{}** ({}% confidence)\n",
                    mapping.source_table, mapping.source_system, mapping.target_table, mapping.confidence
                ));
            }
            markdown.push_str("\n");
        }

        markdown.push_str("## Implementation Plan\n\n");
        markdown.push_str("1. **Setup HelixDB Schema**: Create the necessary tables and relationships in HelixDB based on the high-priority integrations.\n");
        markdown.push_str("2. **Develop Data Migration Scripts**: Create scripts to migrate data from Canvas and Discourse to HelixDB.\n");
        markdown.push_str("3. **Implement Data Synchronization**: Develop a system to keep HelixDB in sync with Canvas and Discourse data.\n");
        markdown.push_str("4. **Test Data Integrity**: Verify that all data is correctly migrated and relationships are maintained.\n");
        markdown.push_str("5. **Optimize Performance**: Ensure that HelixDB performs well with the integrated data.\n");
        markdown.push_str("6. **Implement Offline-First Capabilities**: Develop mechanisms for offline data access and synchronization.\n");

        markdown
    }

    /// Parse a Rails schema.rb file
    fn parse_rails_schema(&self, content: &str, source: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();
        let mut current_table: Option<HelixDbTable> = None;
        let mut in_table_block = false;
        let _in_index_block = false;
        let mut table_block_content = String::new();

        // More robust parsing using line-by-line analysis
        let lines: Vec<&str> = content.lines().collect();

        // Table regex patterns
        let create_table_regex = regex::Regex::new(r#"create_table\s+["']([^"']+)["']"#).unwrap();
        let field_regex = regex::Regex::new(r#"\s*t\.([a-z_]+)\s+["']([^"']+)["'](?:,\s*([^\n]+))?"#).unwrap();
        let primary_key_regex = regex::Regex::new(r"primary_key:\s*true").unwrap();
        let null_regex = regex::Regex::new(r"null:\s*(true|false)").unwrap();
        let default_regex = regex::Regex::new(r"default:\s*([^,\s]+)").unwrap();
        let unique_regex = regex::Regex::new(r"unique:\s*true").unwrap();
        let end_regex = regex::Regex::new(r"\s*end\s*").unwrap();

        // Index regex patterns
        let add_index_regex = regex::Regex::new(r#"add_index\s+["']([^"']+)["'],\s*\[([^\]]+)\](?:,\s*([^\n]+))?"#).unwrap();
        let index_name_regex = regex::Regex::new(r#"name:\s*["']([^"']+)["']"#).unwrap();
        let index_unique_regex = regex::Regex::new(r"unique:\s*true").unwrap();

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
                    source: source.to_string(),
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
                        let index_name_clone = index_name.clone();
                        table.indexes.push(HelixDbIndex {
                            name: index_name_clone,
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
                            name: index_name.clone(),
                            fields: fields.clone(),
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

    /// Parse a Rails migration file
    fn parse_rails_migration(&self, content: &str, source: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // Migration regex patterns
        let create_table_regex = regex::Regex::new(r#"create_table\s+["']([^"']+)["']"#).unwrap();
        let add_column_regex = regex::Regex::new(r#"add_column\s+["']([^"']+)["'],\s*["']([^"']+)["'],\s*["']?([^"',\s]+)["']?(?:,\s*([^\n]+))?"#).unwrap();
        let change_column_regex = regex::Regex::new(r#"change_column\s+["']([^"']+)["'],\s*["']([^"']+)["'],\s*["']?([^"',\s]+)["']?(?:,\s*([^\n]+))?"#).unwrap();
        let add_index_regex = regex::Regex::new(r#"add_index\s+["']([^"']+)["'],\s*\[?([^\]\)]+)\]?(?:,\s*([^\n]+))?"#).unwrap();
        let remove_column_regex = regex::Regex::new(r#"remove_column\s+["']([^"']+)["'],\s*["']([^"']+)["']"#).unwrap();
        let drop_table_regex = regex::Regex::new(r#"drop_table\s+["']([^"']+)["']"#).unwrap();

        // Table properties regex patterns
        let primary_key_regex = regex::Regex::new(r"primary_key:\s*true").unwrap();
        let null_regex = regex::Regex::new(r"null:\s*(true|false)").unwrap();
        let default_regex = regex::Regex::new(r"default:\s*([^,\s]+)").unwrap();
        let unique_regex = regex::Regex::new(r"unique:\s*true").unwrap();

        // Track tables and their fields
        let mut migration_tables: HashMap<String, HelixDbTable> = HashMap::new();

        // Process the content line by line
        for line in content.lines() {
            // Check for create_table statements
            if let Some(captures) = create_table_regex.captures(line) {
                let table_name = captures[1].to_string();
                migration_tables.insert(table_name.clone(), HelixDbTable {
                    name: table_name,
                    fields: Vec::new(),
                    indexes: Vec::new(),
                    relationships: Vec::new(),
                    source: source.to_string(),
                });
            }

            // Check for add_column statements
            if let Some(captures) = add_column_regex.captures(line) {
                let table_name = captures[1].to_string();
                let field_name = captures[2].to_string();
                let field_type = captures[3].to_string();

                // Default field properties
                let mut nullable = true;
                let mut default = None;
                let mut primary_key = false;
                let mut unique = false;

                // Check for additional field properties
                if let Some(props) = captures.get(4) {
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

                // Add or update the field in the table
                if let Some(table) = migration_tables.get_mut(&table_name) {
                    // Check if the field already exists
                    let field_exists = table.fields.iter().any(|f| f.name == field_name);

                    if !field_exists {
                        table.fields.push(HelixDbField {
                            name: field_name,
                            field_type,
                            nullable,
                            default,
                            primary_key,
                            unique,
                        });
                    }
                } else {
                    // Create a new table if it doesn't exist
                    let mut table = HelixDbTable {
                        name: table_name.clone(),
                        fields: Vec::new(),
                        indexes: Vec::new(),
                        relationships: Vec::new(),
                        source: source.to_string(),
                    };

                    table.fields.push(HelixDbField {
                        name: field_name,
                        field_type,
                        nullable,
                        default,
                        primary_key,
                        unique,
                    });

                    migration_tables.insert(table_name, table);
                }
            }

            // Check for change_column statements
            if let Some(captures) = change_column_regex.captures(line) {
                let table_name = captures[1].to_string();
                let field_name = captures[2].to_string();
                let field_type = captures[3].to_string();

                // Default field properties
                let mut nullable = true;
                let mut default = None;
                let mut primary_key = false;
                let mut unique = false;

                // Check for additional field properties
                if let Some(props) = captures.get(4) {
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

                // Update the field in the table
                if let Some(table) = migration_tables.get_mut(&table_name) {
                    // Find and update the field
                    for field in &mut table.fields {
                        if field.name == field_name {
                            field.field_type = field_type;
                            field.nullable = nullable;
                            field.default = default;
                            field.primary_key = primary_key;
                            field.unique = unique;
                            break;
                        }
                    }
                }
            }

            // Check for add_index statements
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
                    if let Some(name_captures) = regex::Regex::new(r#"name:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
                        index_name = name_captures[1].to_string();
                    }

                    // Check if index is unique
                    if regex::Regex::new(r"unique:\s*true").unwrap().is_match(props_str) {
                        unique = true;
                    }
                }

                // Add the index to the table
                if let Some(table) = migration_tables.get_mut(&table_name) {
                    // Check if the index already exists
                    let index_exists = table.indexes.iter().any(|i| i.name == index_name);

                    if !index_exists {
                        table.indexes.push(HelixDbIndex {
                            name: index_name,
                            fields,
                            unique,
                        });
                    }
                }
            }

            // Check for remove_column statements
            if let Some(captures) = remove_column_regex.captures(line) {
                let table_name = captures[1].to_string();
                let field_name = captures[2].to_string();

                // Remove the field from the table
                if let Some(table) = migration_tables.get_mut(&table_name) {
                    table.fields.retain(|f| f.name != field_name);
                }
            }

            // Check for drop_table statements
            if let Some(captures) = drop_table_regex.captures(line) {
                let table_name = captures[1].to_string();

                // Remove the table
                migration_tables.remove(&table_name);
            }
        }

        // Convert the HashMap to a Vec
        tables.extend(migration_tables.into_values());

        Ok(tables)
    }

    /// Parse a Rails model file
    fn parse_rails_model(&self, content: &str, source: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();

        // Model regex patterns
        let class_regex = regex::Regex::new(r"class\s+([A-Za-z0-9_]+)\s+<\s+[A-Za-z0-9:]+").unwrap();

        // Relationship regex patterns
        let belongs_to_regex = regex::Regex::new(r"belongs_to\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_many_regex = regex::Regex::new(r"has_many\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_one_regex = regex::Regex::new(r"has_one\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();
        let has_and_belongs_to_many_regex = regex::Regex::new(r"has_and_belongs_to_many\s+:([a-z_]+)(?:,\s*([^\n]+))?").unwrap();

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
            source: source.to_string(),
        };

        // Extract belongs_to relationships
        for captures in belongs_to_regex.captures_iter(content) {
            let related_model = captures[1].to_string();

            // Determine the foreign key
            let foreign_key = if let Some(props) = captures.get(2) {
                let props_str = props.as_str();
                if let Some(fk_captures) = regex::Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
                    fk_captures[1].to_string()
                } else {
                    format!("{}_id", related_model)
                }
            } else {
                format!("{}_id", related_model)
            };

            // Clone the foreign_key for use in the relationship
            let foreign_key_clone = foreign_key.clone();

            // Add the relationship
            table.relationships.push(HelixDbRelationship {
                relationship_type: "belongs_to".to_string(),
                foreign_table: related_model,
                local_field: foreign_key_clone,
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
                if let Some(fk_captures) = regex::Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
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
                if let Some(fk_captures) = regex::Regex::new(r#"foreign_key:\s*["']([^"']+)["']"#).unwrap().captures(props_str) {
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

    /// Parse a Rust model file
    fn parse_rust_model(&self, content: &str, source: &str) -> Result<Vec<HelixDbTable>> {
        let mut tables = Vec::new();
        let mut current_struct: Option<HelixDbTable> = None;
        let mut in_struct_block = false;
        let mut struct_block_content = String::new();
        let mut brace_count = 0;

        // More robust parsing using line-by-line analysis
        let lines: Vec<&str> = content.lines().collect();

        // Struct regex patterns
        let struct_regex = regex::Regex::new(r"struct\s+([A-Za-z0-9_]+)").unwrap();
        let field_regex = regex::Regex::new(r"\s*(?:pub\s+)?([a-z_]+):\s+([A-Za-z0-9<>:,\s]+),?").unwrap();

        // Derive attribute regex patterns
        let _derive_regex = regex::Regex::new(r"#\[derive\(([^\)]+)\)\]").unwrap();
        let table_attr_regex = regex::Regex::new(r#"#\[table\(name\s*=\s*["']([^"']+)["']\)\]"#).unwrap();
        let primary_key_attr_regex = regex::Regex::new(r"#\[primary_key\]").unwrap();
        let column_attr_regex = regex::Regex::new(r"#\[column\(([^\)]+)\)\]").unwrap();

        // Relationship regex patterns
        let belongs_to_regex = regex::Regex::new(r"#\[belongs_to\(([^\)]+)\)\]").unwrap();
        let has_many_regex = regex::Regex::new(r"#\[has_many\(([^\)]+)\)\]").unwrap();
        let has_one_regex = regex::Regex::new(r"#\[has_one\(([^\)]+)\)\]").unwrap();

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
                    source: source.to_string(),
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
                                let default_regex = regex::Regex::new(r#"default\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
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
                            let foreign_table_regex = regex::Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = regex::Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

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
                            let foreign_table_regex = regex::Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = regex::Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

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
                            let foreign_table_regex = regex::Regex::new(r#"model\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();
                            let foreign_key_regex = regex::Regex::new(r#"foreign_key\s*=\s*["']?([^"',\s]+)["']?"#).unwrap();

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

    /// Calculate the confidence of a table mapping
    fn calculate_table_mapping_confidence(&self, source_table: &HelixDbTable, target_table: &HelixDbTable) -> u8 {
        let mut confidence = 0;

        // Check table name similarity
        let source_name = source_table.name.to_lowercase();
        let target_name = target_table.name.to_lowercase();
        if source_name == target_name {
            confidence += 40;
        } else if source_name.contains(&target_name) || target_name.contains(&source_name) {
            confidence += 20;
        } else if self.calculate_string_similarity(&source_name, &target_name) > 0.7 {
            confidence += 10;
        }

        // Check field similarity
        let mut matching_fields = 0;
        for source_field in &source_table.fields {
            for target_field in &target_table.fields {
                if source_field.name.to_lowercase() == target_field.name.to_lowercase() {
                    matching_fields += 1;
                    break;
                }
            }
        }

        let field_ratio = if !source_table.fields.is_empty() {
            matching_fields as f32 / source_table.fields.len() as f32
        } else {
            0.0
        };

        confidence += (field_ratio * 60.0) as u8;

        // Cap at 100
        confidence.min(100)
    }

    /// Generate field mappings between two tables
    fn generate_field_mappings(&self, source_table: &HelixDbTable, target_table: &HelixDbTable) -> Vec<HelixDbFieldMapping> {
        let mut field_mappings = Vec::new();

        for source_field in &source_table.fields {
            let mut best_match = None;
            let mut best_confidence = 0;

            for target_field in &target_table.fields {
                let confidence = self.calculate_field_mapping_confidence(source_field, target_field);
                if confidence > best_confidence {
                    best_match = Some(target_field);
                    best_confidence = confidence;
                }
            }

            if let Some(target_field) = best_match {
                if best_confidence > 50 {
                    field_mappings.push(HelixDbFieldMapping {
                        source_field: source_field.name.clone(),
                        target_field: target_field.name.clone(),
                        confidence: best_confidence,
                    });
                }
            }
        }

        field_mappings
    }

    /// Calculate the confidence of a field mapping
    fn calculate_field_mapping_confidence(&self, source_field: &HelixDbField, target_field: &HelixDbField) -> u8 {
        let mut confidence = 0;

        // Check field name similarity
        let source_name = source_field.name.to_lowercase();
        let target_name = target_field.name.to_lowercase();
        if source_name == target_name {
            confidence += 60;
        } else if source_name.contains(&target_name) || target_name.contains(&source_name) {
            confidence += 30;
        } else if self.calculate_string_similarity(&source_name, &target_name) > 0.7 {
            confidence += 20;
        }

        // Check field type compatibility
        if self.are_field_types_compatible(&source_field.field_type, &target_field.field_type) {
            confidence += 40;
        }

        // Cap at 100
        confidence.min(100)
    }

    /// Check if two field types are compatible
    fn are_field_types_compatible(&self, source_type: &str, target_type: &str) -> bool {
        // Simple compatibility check
        // In a real implementation, this would be more sophisticated
        let source_type = source_type.to_lowercase();
        let target_type = target_type.to_lowercase();

        if source_type == target_type {
            return true;
        }

        // Check for common type mappings
        match (source_type.as_str(), target_type.as_str()) {
            ("string", "string") | ("string", "text") | ("text", "string") | ("text", "text") => true,
            ("integer", "i32") | ("integer", "i64") | ("integer", "u32") | ("integer", "u64") => true,
            ("float", "f32") | ("float", "f64") | ("decimal", "f32") | ("decimal", "f64") => true,
            ("boolean", "bool") => true,
            ("datetime", "chrono::datetime") | ("timestamp", "chrono::datetime") => true,
            _ => false,
        }
    }

    /// Calculate the similarity between two strings
    fn calculate_string_similarity(&self, s1: &str, s2: &str) -> f32 {
        // Simple Levenshtein distance-based similarity
        // In a real implementation, this would be more sophisticated
        let distance = self.levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len()) as f32;
        if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance as f32 / max_len)
        }
    }

    /// Calculate the Levenshtein distance between two strings
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 == 0 {
            return len2;
        }
        if len2 == 0 {
            return len1;
        }

        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Walk a directory recursively and call a function on each file
    fn walk_directory<F>(&self, dir: &Path, mut callback: F) -> Result<()>
    where
        F: FnMut(&Path),
    {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.walk_directory(&path, &mut callback)?;
                } else {
                    callback(&path);
                }
            }
        }
        Ok(())
    }
}
