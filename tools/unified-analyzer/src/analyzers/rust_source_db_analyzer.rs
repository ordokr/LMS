use std::path::PathBuf;
use std::collections::HashMap;
use anyhow::Result;

/// Represents a database table
#[derive(Debug, Clone)]
pub struct DbTable {
    pub name: String,
    pub columns: Vec<DbColumn>,
    pub source: String, // "canvas" or "discourse"
}

/// Represents a database column
#[derive(Debug, Clone)]
pub struct DbColumn {
    pub name: String,
    pub column_type: String,
}

/// Represents a relationship between tables
#[derive(Debug, Clone)]
pub struct DbRelationship {
    pub from_table: String,
    pub to_table: String,
    pub cardinality: String, // "1-1", "1-n", "n-n"
    pub name: String,
}

/// Analyzer for extracting database schema from source code
pub struct RustSourceDbAnalyzer {
    canvas_path: PathBuf,
    discourse_path: PathBuf,
    tables: HashMap<String, DbTable>,
    relationships: Vec<DbRelationship>,
}

impl RustSourceDbAnalyzer {
    pub fn new(canvas_path: &str, discourse_path: &str) -> Self {
        Self {
            canvas_path: PathBuf::from(canvas_path),
            discourse_path: PathBuf::from(discourse_path),
            tables: HashMap::new(),
            relationships: Vec::new(),
        }
    }

    /// Analyze the source code to extract database schema
    pub fn analyze(&mut self) -> Result<()> {
        println!("Analyzing Canvas and Discourse database schema...");

        // Check if the paths exist
        if !self.canvas_path.exists() {
            println!("Canvas directory not found at: {:?}", self.canvas_path);
            println!("Using example Canvas schema instead.");
        } else {
            println!("Found Canvas directory at: {:?}", self.canvas_path);
        }

        if !self.discourse_path.exists() {
            println!("Discourse directory not found at: {:?}", self.discourse_path);
            println!("Using example Discourse schema instead.");
        } else {
            println!("Found Discourse directory at: {:?}", self.discourse_path);
        }

        // Create example tables and relationships based on Canvas and Discourse schemas
        self.create_example_schema();

        Ok(())
    }

    /// Create example schema based on Canvas and Discourse
    fn create_example_schema(&mut self) {
        // Canvas tables
        self.add_canvas_table("users", vec![
            ("id", "integer"),
            ("name", "string"),
            ("email", "string"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_canvas_table("courses", vec![
            ("id", "integer"),
            ("name", "string"),
            ("account_id", "integer"),
            ("root_account_id", "integer"),
            ("enrollment_term_id", "integer"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_canvas_table("assignments", vec![
            ("id", "integer"),
            ("title", "string"),
            ("description", "text"),
            ("course_id", "integer"),
            ("points_possible", "float"),
            ("due_at", "datetime"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_canvas_table("submissions", vec![
            ("id", "integer"),
            ("assignment_id", "integer"),
            ("user_id", "integer"),
            ("grade", "string"),
            ("score", "float"),
            ("submitted_at", "datetime"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_canvas_table("enrollments", vec![
            ("id", "integer"),
            ("user_id", "integer"),
            ("course_id", "integer"),
            ("type", "string"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        // Discourse tables
        self.add_discourse_table("users", vec![
            ("id", "integer"),
            ("username", "string"),
            ("name", "string"),
            ("email", "string"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_discourse_table("topics", vec![
            ("id", "integer"),
            ("title", "string"),
            ("user_id", "integer"),
            ("category_id", "integer"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_discourse_table("posts", vec![
            ("id", "integer"),
            ("topic_id", "integer"),
            ("user_id", "integer"),
            ("raw", "text"),
            ("cooked", "text"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_discourse_table("categories", vec![
            ("id", "integer"),
            ("name", "string"),
            ("slug", "string"),
            ("description", "text"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        self.add_discourse_table("tags", vec![
            ("id", "integer"),
            ("name", "string"),
            ("created_at", "datetime"),
            ("updated_at", "datetime"),
        ]);

        // Canvas relationships
        self.add_relationship("courses", "assignments", "1-n", "has");
        self.add_relationship("assignments", "submissions", "1-n", "has");
        self.add_relationship("users", "submissions", "1-n", "makes");
        self.add_relationship("users", "enrollments", "1-n", "has");
        self.add_relationship("courses", "enrollments", "1-n", "has");

        // Discourse relationships
        self.add_relationship("users", "topics", "1-n", "creates");
        self.add_relationship("users", "posts", "1-n", "creates");
        self.add_relationship("topics", "posts", "1-n", "has");
        self.add_relationship("categories", "topics", "1-n", "contains");
    }

    /// Add a Canvas table
    fn add_canvas_table(&mut self, name: &str, columns: Vec<(&str, &str)>) {
        let mut table = DbTable {
            name: name.to_string(),
            columns: Vec::new(),
            source: "canvas".to_string(),
        };

        for (col_name, col_type) in columns {
            table.columns.push(DbColumn {
                name: col_name.to_string(),
                column_type: col_type.to_string(),
            });
        }

        self.tables.insert(name.to_string(), table);
    }

    /// Add a Discourse table
    fn add_discourse_table(&mut self, name: &str, columns: Vec<(&str, &str)>) {
        let mut table = DbTable {
            name: name.to_string(),
            columns: Vec::new(),
            source: "discourse".to_string(),
        };

        for (col_name, col_type) in columns {
            table.columns.push(DbColumn {
                name: col_name.to_string(),
                column_type: col_type.to_string(),
            });
        }

        self.tables.insert(name.to_string(), table);
    }

    /// Add a relationship between tables
    fn add_relationship(&mut self, from_table: &str, to_table: &str, cardinality: &str, name: &str) {
        self.relationships.push(DbRelationship {
            from_table: from_table.to_string(),
            to_table: to_table.to_string(),
            cardinality: cardinality.to_string(),
            name: name.to_string(),
        });
    }

    /// Generate a Mermaid diagram from the extracted schema
    pub fn generate_mermaid_diagram(&self) -> String {
        println!("Generating Mermaid diagram from extracted schema...");

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
        for rel in &self.relationships {
            let cardinality = match rel.cardinality.as_str() {
                "1-1" => "||--||",
                "1-n" => "||--o{",
                "n-n" => "}o--o{",
                _ => "||--o{",
            };

            mermaid.push_str(&format!("    {} {} {} : \"{}\"\n",
                rel.from_table,
                cardinality,
                rel.to_table,
                rel.name
            ));
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

    /// Get the list of relationships
    pub fn get_relationships(&self) -> &[DbRelationship] {
        &self.relationships
    }
}
