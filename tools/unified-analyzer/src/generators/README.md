# Unified Analyzer Generators

This directory contains the documentation generators for the Unified Analyzer. The generators are responsible for generating documentation based on the analysis results.

## Generator Structure

The generators are organized as follows:

```
generators/
├── mod.rs                              # Module definition
├── error.rs                            # Error handling
├── README.md                           # This file
├── api_doc_generator.rs                # API documentation generator
├── implementation_details_generator.rs # Implementation details generator
├── testing_doc_generator.rs            # Testing documentation generator
├── tech_debt_report_generator.rs       # Technical debt report generator
├── summary_report_generator.rs         # Summary report generator
├── enhanced_central_hub_generator.rs   # Enhanced central hub generator
├── sync_architecture_generator.rs      # Synchronization architecture generator
└── database_architecture_generator.rs  # Database architecture generator
```

## Generator Interface

All generators implement the same interface:

```rust
pub fn generate_xxx(result: &AnalysisResult, base_dir: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
```

Where:
- `result` is the analysis result from the Unified Analyzer
- `base_dir` is the base directory for the project
- The function returns a `Result` with an empty tuple on success or an error on failure

## Error Handling

The generators use the `GeneratorError` type for error handling. The `GeneratorError` type is defined in `error.rs` and includes the following error types:

- `DirectoryCreation`: Error when creating a directory
- `FileWrite`: Error when writing to a file
- `FileRead`: Error when reading from a file
- `DataParsing`: Error when parsing data
- `ContentGeneration`: Error when generating content
- `Other`: Other errors

## Generator Descriptions

### API Documentation Generator

The API documentation generator generates documentation for the API endpoints in the project. It includes information about the endpoints, their parameters, and their responses.

### Implementation Details Generator

The implementation details generator generates documentation about the implementation details of the project. It includes information about the models, API endpoints, UI components, and integration points.

### Testing Documentation Generator

The testing documentation generator generates documentation about the testing strategy and metrics for the project. It includes information about the test coverage, test types, and test results.

### Technical Debt Report Generator

The technical debt report generator generates a report about the technical debt in the project. It includes information about code quality issues, architectural issues, and recommendations for improvement.

### Summary Report Generator

The summary report generator generates a summary report about the project. It includes information about the project status, completion percentage, and next steps.

### Enhanced Central Hub Generator

The enhanced central hub generator generates a central reference hub for the project. It includes information about the project structure, technology stack, architecture principles, design patterns, integration architecture, model mapping, common code patterns, and implementation recommendations.

### Synchronization Architecture Generator

The synchronization architecture generator generates documentation about the synchronization architecture between Canvas and Discourse. It includes information about the synchronization strategy, conflict resolution, and offline capabilities.

### Database Architecture Generator

The database architecture generator generates documentation about the database architecture of the project. It includes information about the database schema, queries, and performance considerations.

## Adding New Generators

To add a new generator:

1. Create a new file in the `generators/` directory (e.g., `new_generator.rs`)
2. Add the module to `generators/mod.rs`
3. Re-export the generator module in `generators/mod.rs`
4. Implement the generator function
5. Update the main function in `main.rs` to use the new generator
6. Update the configuration in `config.toml` to include the new generator
