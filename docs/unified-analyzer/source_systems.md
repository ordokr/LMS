# Source Systems Architecture

_Last updated: 2025-04-18_

This document explains the source system architecture used in the unified analyzer for extracting and analyzing database schemas from different codebases.

## Overview

The source system architecture provides a flexible, extensible way to analyze different codebases for database schema extraction. It allows the unified analyzer to support multiple source systems (Canvas, Discourse, Moodle, WordPress, etc.) with a common interface while implementing system-specific parsing logic.

## Architecture Components

The source system architecture consists of:

1. **SourceSystem Trait**: A common interface that all source systems implement
2. **SourceSystemType Enum**: Identifies different types of source systems
3. **Concrete Source System Implementations**: Specific implementations for each supported system
4. **HelixDbIntegrationAnalyzer**: Uses the source systems to extract and analyze database schemas

### SourceSystem Trait

The `SourceSystem` trait defines the common interface that all source systems must implement:

```rust
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
```

This trait ensures that all source systems provide methods to:
- Identify their type
- Extract database schemas
- Get their name
- Support downcasting (for type-specific operations)

### SourceSystemType Enum

The `SourceSystemType` enum identifies the different types of source systems supported by the analyzer:

```rust
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
```

This enum allows for easy identification and comparison of source systems.

### Concrete Source System Implementations

Each source system implements the `SourceSystem` trait with specific logic for extracting database schemas from that particular system:

- **CanvasSourceSystem**: Extracts schemas from Canvas LMS (Ruby on Rails)
- **DiscourseSourceSystem**: Extracts schemas from Discourse forum (Ruby on Rails)
- **OrdoSourceSystem**: Extracts schemas from Ordo LMS (Rust)
- **MoodleSourceSystem**: Extracts schemas from Moodle LMS (PHP)
- **WordPressSourceSystem**: Extracts schemas from WordPress CMS (PHP)

Each implementation knows how to parse the specific file formats and structures of its source system to extract database tables, fields, indexes, and relationships.

### HelixDbIntegrationAnalyzer

The `HelixDbIntegrationAnalyzer` manages a collection of source systems and uses them to extract database schemas from different codebases. It:

- Registers source systems
- Calls the appropriate source system to extract schemas
- Stores the extracted tables
- Generates mappings between tables from different systems
- Produces reports and integration plans

## How Source Systems Work

### Registration Process

Source systems are registered with the `HelixDbIntegrationAnalyzer` when needed:

```rust
pub fn extract_canvas_schema(&mut self, canvas_path: &Path) -> Result<()> {
    // Check if we already have a Canvas source system
    let canvas_system = self.source_systems.iter()
        .find(|s| s.get_type() == SourceSystemType::Canvas);
    
    if canvas_system.is_none() {
        // Register a new Canvas source system
        let canvas_system = CanvasSourceSystem::new();
        self.register_source_system(canvas_system);
    }
    
    // Find the Canvas source system
    let canvas_system = self.source_systems.iter()
        .find(|s| s.get_type() == SourceSystemType::Canvas)
        .ok_or_else(|| anyhow!("Canvas source system not found"))?;
    
    // Extract schema using the Canvas source system
    let tables = canvas_system.extract_schema(canvas_path)?;
    
    // Store the extracted tables
    self.tables.insert("canvas".to_string(), tables);
    
    Ok(())
}
```

### Schema Extraction

Each source system implements its own schema extraction logic based on the structure and format of its codebase:

- **Canvas/Discourse**: Parse Ruby on Rails schema.rb files, migrations, and models
- **Ordo**: Parse Rust struct definitions and attributes
- **Moodle**: Parse PHP install.xml files and upgrade.php files
- **WordPress**: Parse PHP schema definitions and model files

The extracted schema information is returned as a collection of `HelixDbTable` objects, which include:
- Table name
- Fields (name, type, constraints)
- Indexes
- Relationships

### Caching

Source systems support caching to avoid re-analyzing unchanged files:

```rust
// Check if file is in cache and hasn't been modified
let file_path_str = file_path.to_string_lossy().to_string();
let should_parse = if self.use_cache {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            if let Ok(modified_time) = metadata.modified() {
                if let Ok(modified_secs) = modified_time.duration_since(SystemTime::UNIX_EPOCH) {
                    let modified_secs = modified_secs.as_secs();
                    if let Some(cache) = self.file_cache.get(&file_path_str) {
                        if cache.last_modified >= modified_secs {
                            // File hasn't been modified, use cached results
                            return Ok(cache.tables.clone());
                        }
                    }
                }
            }
            true
        },
        Err(_) => true,
    }
} else {
    true
};
```

## Using Source Systems

### Command-Line Usage

You can use source systems through the unified analyzer command-line interface:

```bash
# Extract and analyze schemas from Canvas and Discourse
./unified-analyzer helix-db-integration --canvas_path C:\path\to\canvas --discourse_path C:\path\to\discourse

# Include Moodle in the analysis
./unified-analyzer helix-db-integration --canvas_path C:\path\to\canvas --discourse_path C:\path\to\discourse --moodle_path C:\path\to\moodle

# Disable caching for fresh analysis
./unified-analyzer helix-db-integration --no-cache --canvas_path C:\path\to\canvas
```

### Available Command-Line Options

| Option | Description |
|--------|-------------|
| `--canvas_path PATH` | Path to Canvas codebase |
| `--discourse_path PATH` | Path to Discourse codebase |
| `--lms_path PATH` | Path to LMS codebase |
| `--moodle_path PATH` | Path to Moodle codebase |
| `--wordpress_path PATH` | Path to WordPress codebase |
| `--cache` | Enable caching of analysis results |
| `--no-cache` | Disable caching of analysis results |

### Programmatic Usage

You can also use source systems programmatically in your Rust code:

```rust
use crate::analyzers::modules::helix_db_integration::HelixDbIntegrationAnalyzer;
use crate::analyzers::modules::source_systems::canvas::CanvasSourceSystem;
use crate::analyzers::modules::source_systems::discourse::DiscourseSourceSystem;
use std::path::Path;

// Create a new HelixDB integration analyzer
let mut analyzer = HelixDbIntegrationAnalyzer::new();

// Extract schemas from Canvas and Discourse
analyzer.extract_canvas_schema(Path::new("C:\\path\\to\\canvas"))?;
analyzer.extract_discourse_schema(Path::new("C:\\path\\to\\discourse"))?;

// Generate mappings between tables
analyzer.generate_mappings()?;

// Generate reports
let json_report = analyzer.generate_mapping_report()?;
let markdown_report = analyzer.generate_mapping_markdown();
```

## Extending with New Source Systems

To add support for a new source system:

1. Create a new module in `tools/unified-analyzer/src/analyzers/modules/source_systems/`
2. Implement the `SourceSystem` trait for your new source system
3. Add a new variant to the `SourceSystemType` enum
4. Add a new extraction method to the `HelixDbIntegrationAnalyzer`
5. Update the command-line argument parsing to support the new source system

Example implementation:

```rust
pub struct MyNewSourceSystem {
    use_cache: bool,
}

impl MyNewSourceSystem {
    pub fn new() -> Self {
        Self {
            use_cache: true,
        }
    }
    
    pub fn new_without_cache() -> Self {
        Self {
            use_cache: false,
        }
    }
    
    pub fn set_use_cache(&mut self, use_cache: bool) {
        self.use_cache = use_cache;
    }
    
    // Implement parsing logic for your source system
}

impl SourceSystem for MyNewSourceSystem {
    fn get_type(&self) -> SourceSystemType {
        SourceSystemType::Custom("MyNewSystem".to_string())
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    
    fn extract_schema(&self, path: &Path) -> Result<Vec<HelixDbTable>> {
        // Implement schema extraction logic
    }
}
```

## Output and Reports

The source systems feed into the HelixDB integration analyzer, which generates:

1. **JSON Reports**: Detailed technical information about tables and mappings
2. **Markdown Reports**: Human-readable integration plans and recommendations
3. **Integration Plans**: Step-by-step guides for implementing the integrated database schema

These reports help developers understand how to integrate data from different source systems into a unified schema.

## Conclusion

The source system architecture provides a flexible, extensible way to analyze database schemas from different codebases. By implementing a common interface with system-specific parsing logic, it allows the unified analyzer to support multiple source systems while maintaining a clean, modular design.

This architecture makes it easy to add support for new source systems without changing the core integration logic, ensuring that the analyzer can adapt to new requirements and source systems as needed.
