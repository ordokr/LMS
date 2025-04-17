# Visualization Generator Tasks

This document outlines the tasks needed to fix the visualization generators in the unified-analyzer tool.

## Current Issues

1. The visualization generators are not properly creating the HTML, Markdown, and JSON files
2. The directory structure exists, but the files are not being generated
3. The links in the README file are broken because the files don't exist

## Tasks to Fix Visualization Generators

### 1. API Map Generator

- [ ] Review `src/generators/api_map_generator.rs` to identify issues
- [ ] Ensure the `generate` function is properly creating the output files
- [ ] Verify that the template is being loaded correctly
- [ ] Fix any issues with the template placeholders
- [ ] Update the `main.rs` file to properly call the generator function
- [ ] Test the generator by running `cargo run --bin unified-analyzer -- --api-map .`

### 2. Component Tree Generator

- [ ] Review `src/generators/component_tree_generator.rs` to identify issues
- [ ] Ensure the `generate` function is properly creating the output files
- [ ] Verify that the template is being loaded correctly
- [ ] Fix any issues with the template placeholders
- [ ] Update the `main.rs` file to properly call the generator function
- [ ] Test the generator by running `cargo run --bin unified-analyzer -- --component-tree .`

### 3. Database Schema Generator

- [ ] Review `src/generators/db_schema_generator.rs` to identify issues
- [ ] Ensure the `generate` function is properly creating the output files
- [ ] Verify that the template is being loaded correctly
- [ ] Fix any issues with the template placeholders
- [ ] Update the `main.rs` file to properly call the generator function
- [ ] Test the generator by running `cargo run --bin unified-analyzer -- --db-schema .`

### 4. Migration Roadmap Generator

- [ ] Review `src/generators/migration_roadmap_generator.rs` to identify issues
- [ ] Ensure the `generate` function is properly creating the output files
- [ ] Verify that the template is being loaded correctly
- [ ] Fix any issues with the template placeholders
- [ ] Update the `main.rs` file to properly call the generator function
- [ ] Test the generator by running `cargo run --bin unified-analyzer -- --roadmap .`

### 5. Main Visualization Command

- [ ] Review the `--viz` command in `src/main.rs` to ensure it calls all visualization generators
- [ ] Test the command by running `cargo run --bin unified-analyzer -- --viz .`

## Common Issues to Look For

1. **File Path Issues**: Ensure the output paths are correct and the directories exist
2. **Template Loading**: Verify that templates are being loaded correctly
3. **Template Placeholders**: Check that placeholders in templates are being replaced with data
4. **Error Handling**: Improve error handling to provide better diagnostics
5. **Missing Function Calls**: Ensure generator functions are being called in the main function
6. **Data Preparation**: Verify that data is being prepared correctly before generating visualizations

## Testing

After fixing each generator, test it by running the appropriate command and verifying that the output files are created and contain the expected content.

## Documentation

Once the generators are fixed, update the README.md file to remove the warning messages and update the instructions.
