# Ordo Project Glossary

This glossary defines key terms used throughout the Ordo project documentation to ensure clarity and consistency, particularly regarding the source-to-source migration approach.

## Key Terms

### Source-to-Source Transformation

The process of converting code from one programming language to another without migrating data or connecting to existing deployments. In Ordo, this refers to transforming Canvas LMS and Discourse code from Ruby/JavaScript to Rust/Haskell.

### Code Porting

The process of reimplementing functionality from one codebase to another, typically in a different programming language. In Ordo, we port features from Canvas LMS and Discourse to our Rust/Tauri/Leptos stack.

### Code Mapping

The process of tracking relationships between original source code entities and their corresponding implementations in the new codebase. This is for development tracking only, not for data migration.

### Schema Transformation

The process of converting database schema definitions from one format to another. In Ordo, we analyze Ruby migration files to inform the design of our SQLite schema, but we do not migrate actual data.

### Static Analysis

The process of examining source code without executing it. Ordo's analyzers perform static analysis on Canvas and Discourse code to understand its structure and behavior.

### Code Generation

The automated process of producing code in the target language based on analysis of the source language. Ordo uses code generators to transform Ruby models to Rust structs, views to Leptos components, etc.

## Terms to Avoid Confusion

### Migration (Clarified Usage)

In database contexts, "migration" typically refers to moving data from one system to another. In Ordo documentation, we use more specific terms:
- **Schema Migration**: SQL files that set up the database structure for the new application
- **Source Code Transformation**: Converting code from one language to another
- **Code Porting**: Reimplementing functionality in a new language/framework

We do NOT perform data migration from existing Canvas or Discourse deployments.

### Integration (Clarified Usage)

In system contexts, "integration" typically refers to connecting different systems. In Ordo documentation:
- **Component Integration**: How different parts of Ordo work together
- **Source Code Porting**: How we reimplement Canvas and Discourse features

We do NOT integrate with existing Canvas or Discourse deployments.

### Import (Clarified Usage)

In data contexts, "import" typically refers to bringing data from one system into another. In Ordo:
- **Test Data Generation**: Creating sample data for development/testing
- **Code Analysis**: Extracting information from source code

We do NOT import data from existing Canvas or Discourse deployments.

## Database-Related Terms

### Schema Definition

The structure of the database, including tables, columns, relationships, and constraints. In Ordo, our schema is defined in SQL migration files in `src-tauri/migrations/`.

### Schema Migration

SQL files that create or modify the database structure. In Ordo, these files set up the schema for the new application and do not migrate data from existing systems.

### Database Connection

Establishing a link to a database for reading or writing data. Ordo connects only to its own local SQLite database, not to external Canvas or Discourse databases.

## Development Process Terms

### Analyzer

A tool that examines source code to extract information about its structure, behavior, and relationships. Ordo's analyzers perform static analysis on Canvas and Discourse code.

### Generator

A tool that produces code in the target language based on analysis of the source language. Ordo's generators create Rust and Leptos code from Ruby and JavaScript.

### Transformation Pipeline

The end-to-end process of analyzing source code, extracting information, and generating equivalent code in the target language.

## Conclusion

This glossary helps ensure consistent terminology throughout the Ordo project. When writing documentation or code comments, please refer to this glossary to use the most appropriate terms, especially when discussing the source-to-source transformation approach.
