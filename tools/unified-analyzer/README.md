# Unified Analyzer for LMS Project

A comprehensive analyzer for the LMS project that provides insights into the codebase structure, implementation status, and recommendations for next steps. The Unified Analyzer is a standalone development tool, independent of the main LMS project.

## Features

- **Project Analysis**: Analyzes models, API endpoints, UI components, integration points, and more
- **Documentation Generation**: Generates comprehensive documentation based on the analysis results
- **Configurable**: Customize the documentation generation through a configuration file
- **Standalone**: Works as a standalone tool, independent of the main LMS project
- **Tracks Progress**: Monitors project completion percentage and estimated completion date
- **Integrated Analyzers**: Includes specialized analyzers for Canvas LMS, Discourse forum, database schema, and blockchain implementation

## Usage

```bash
# Run the analyzer on the current directory
cargo run

# Run the analyzer on a specific directory
cargo run -- /path/to/project

# Run the analyzer using the wrapper scripts
..\..\analyze-integrated.bat  # Windows batch file
..\..\analyze-integrated.ps1  # Windows PowerShell

# Run the tests
cargo test

# Run specific test
cargo test --test unified_analyzer_test

# Run integration tests
cargo test --test unified_analyzer_integration_test

cargo test --test unified_analyzer_test

# Run integration tests
cargo test --test unified_analyzer_integration_test
```

## Output

The analyzer generates comprehensive documentation in the `docs` directory of the analyzed project. The documentation includes:

### High Priority Documentation

- **Central Reference Hub**: A main entry point for all documentation
- **Architecture Documentation**: Overview of the system architecture
- **Models Documentation**: Documentation of data models
- **Integration Documentation**: Documentation of integration between Canvas and Discourse
- **API Documentation**: Documentation of API endpoints
- **Implementation Details**: Specific implementation information
- **Testing Documentation**: Information about testing
- **Technical Debt Report**: Report on technical debt in the project
- **Summary Report**: Summary of the project's status

### Medium Priority Documentation

- **Synchronization Architecture**: Documentation of synchronization architecture
- **Database Architecture**: Documentation of database architecture

### Visualizations

- **API Map**: Interactive visualization of API endpoints and their relationships
- **Component Tree**: Interactive visualization of component hierarchy and dependencies
- **Database Schema**: Interactive visualization of database schema and relationships
- **Migration Roadmap**: Interactive visualization of migration roadmap

For more information about visualizations, see the [Visualizations README](docs/visualizations/README.md).

## Configuration

The Unified Analyzer can be configured through a `config.toml` file. Here's an example configuration:

```toml
# Output directories
[output]
docs_dir = "docs"
api_dir = "docs/api"
architecture_dir = "docs/architecture"
models_dir = "docs/models"
integration_dir = "docs/integration"

# Documentation generation options
[documentation]
# Whether to generate high priority documentation
generate_high_priority = true
# Whether to generate medium priority documentation
generate_medium_priority = true
# Whether to exclude AI/Gemini-related content
exclude_ai_content = true

# High priority documentation
[documentation.high_priority]
central_reference_hub = true
api_documentation = true
implementation_details = true
testing_documentation = true
technical_debt_report = true
summary_report = true

# Medium priority documentation
[documentation.medium_priority]
synchronization_architecture = true
database_architecture = true
```

## Development

### Project Structure
unified-analyzer/
├── src/
│   ├── main.rs                  # Entry point
│   ├── config/                  # Configuration
│   │   └── mod.rs               # Configuration module
│   ├── analyzers/               # Analyzers
│   │   ├── mod.rs               # Analyzer module
│   │   ├── unified_analyzer.rs  # Unified analyzer
│   │   ├── integrated_migration_analyzer.rs  # Integrated migration analyzer
│   │   └── modules/             # Analyzer modules
│   │       ├── mod.rs           # Module definition
│   │       ├── ast_analyzer.rs  # AST analyzer
│   │       ├── canvas_analyzer.rs  # Canvas analyzer
│   │       ├── discourse_analyzer.rs  # Discourse analyzer
│   │       ├── db_schema_analyzer.rs  # Database schema analyzer
│   │       └── blockchain_analyzer.rs  # Blockchain analyzer
│   ├── generators/              # Documentation generators
│   │   ├── mod.rs               # Generator module
│   │   ├── error.rs             # Error handling
│   │   ├── api_map_generator.rs  # API map generator
│   │   ├── component_tree_generator.rs  # Component tree generator
│   │   ├── db_schema_generator.rs  # Database schema generator
│   │   ├── migration_roadmap_generator.rs  # Migration roadmap generator
│   │   ├── api_doc_generator.rs  # API documentation generator
│   │   ├── implementation_details_generator.rs  # Implementation details generator
│   │   ├── testing_doc_generator.rs  # Testing documentation generator
│   │   ├── tech_debt_report_generator.rs  # Technical debt report generator
│   │   ├── summary_report_generator.rs  # Summary report generator
│   │   ├── enhanced_central_hub_generator.rs  # Enhanced central hub generator
│   │   ├── sync_architecture_generator.rs  # Synchronization architecture generator
│   │   ├── database_architecture_generator.rs  # Database architecture generator
│   │   ├── templates/           # HTML templates for visualizations
│   │   │   ├── api_map_template.html  # API map template
│   │   │   ├── component_tree_template.html  # Component tree template
│   │   │   └── db_schema_template.html  # Database schema template
│   │   └── tests/              # Generator tests
│   │       ├── mod.rs           # Test module
│   │       ├── api_map_generator_tests.rs  # API map generator tests
│   │       ├── component_tree_generator_tests.rs  # Component tree generator tests
│   │       ├── db_schema_generator_tests.rs  # Database schema generator tests
│   │       └── migration_roadmap_generator_tests.rs  # Migration roadmap generator tests
│   └── utils/                   # Utilities
│       ├── mod.rs               # Utility module
│       ├── file_system.rs       # File system utilities
│       ├── performance.rs       # Performance utilities
│       └── template_cache.rs    # Template caching utilities
├── tests/                       # Tests
│   ├── main.rs                  # Test runner
│   ├── test_utils.rs            # Test utilities
│   ├── unit/                    # Unit tests
│   │   ├── config_test.rs       # Configuration tests
│   │   └── generators/          # Generator tests
│   │       ├── enhanced_central_hub_generator_test.rs  # Enhanced central hub generator tests
│   │       └── error_test.rs    # Error handling tests
│   └── integration/             # Integration tests
│       └── unified_analyzer_test.rs  # Unified analyzer tests
├── Cargo.toml                   # Cargo configuration
├── Cargo.lock                   # Cargo lock file
├── config.toml                  # Configuration file
└── README.md                    # This file
```

### Adding New Analyzers

To add a new analyzer:

1. Create a new file in `src/analyzers/`
2. Add the module to `src/analyzers/mod.rs`
3. Add the analyzer to the `UnifiedProjectAnalyzer` struct
4. Add the analyzer to the `analyze` method

### Adding New Generators

To add a new documentation generator:

1. Create a new file in `src/generators/`
2. Add the module to `src/generators/mod.rs`
3. Re-export the generator function in `src/generators/mod.rs`
4. Add the generator to the main function in `src/main.rs`
5. Update the configuration in `config.toml` to include the new generator

### Templates

The Unified Analyzer uses HTML templates to generate visualizations. These templates are located in the `src/generators/templates` directory.

#### Template Structure

Each template is an HTML file with placeholders that are replaced with data from the analysis. The placeholders are HTML comments with specific names.

##### API Map Template

The API Map template (`api_map_template.html`) has the following placeholders:

- `<!-- METHOD_FILTERS_PLACEHOLDER -->`: Replaced with HTML for filtering API endpoints by HTTP method
- `<!-- CATEGORIES_PLACEHOLDER -->`: Replaced with HTML for the API endpoint categories

##### Component Tree Template

The Component Tree template (`component_tree_template.html`) has the following placeholders:

- `<!-- GRAPH_DATA_PLACEHOLDER -->`: Replaced with JSON data for the component tree graph

##### Database Schema Template

The Database Schema template (`db_schema_template.html`) has the following placeholders:

- `<!-- TABLE_LIST_PLACEHOLDER -->`: Replaced with HTML for the list of database tables
- `<!-- RELATIONSHIPS_LIST_PLACEHOLDER -->`: Replaced with HTML for the list of table relationships
- `<!-- SCHEMA_DATA_PLACEHOLDER -->`: Replaced with JSON data for the database schema graph

#### Customizing Templates

You can customize the templates to change the appearance and behavior of the visualizations. Just make sure to keep the placeholders intact, or the generator will not be able to replace them with data.

To create a custom template:

1. Copy one of the existing templates to a new file
2. Modify the HTML, CSS, and JavaScript as needed
3. Make sure to keep the placeholders intact
4. Update the generator code to use your custom template

#### Template Caching

Templates are cached in memory to avoid repeated disk reads. The template cache is implemented in the `src/utils/template_cache.rs` file. To use the template cache in a generator:

```rust
use crate::utils::template_cache::get_template;

// Load template file using the template cache
let template_path = "src/generators/templates/my_template.html";
let embedded_template = include_str!("templates/my_template.html");
let template = get_template(template_path, Some(embedded_template))?;
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_unified_analyzer_initialization

# Run tests with specific features
cargo test --features "feature-name"
```
