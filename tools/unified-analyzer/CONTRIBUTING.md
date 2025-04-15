# Contributing to the Unified Analyzer

Thank you for your interest in contributing to the Unified Analyzer! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

Please be respectful and considerate of others when contributing to this project. We aim to foster an inclusive and welcoming community.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with the following information:

1. A clear and descriptive title
2. Steps to reproduce the bug
3. Expected behavior
4. Actual behavior
5. Screenshots (if applicable)
6. Environment information (OS, Rust version, etc.)

### Suggesting Enhancements

If you have an idea for an enhancement, please create an issue with the following information:

1. A clear and descriptive title
2. A detailed description of the enhancement
3. Why this enhancement would be useful
4. Any examples or mockups (if applicable)

### Pull Requests

1. Fork the repository
2. Create a new branch for your changes
3. Make your changes
4. Run tests to ensure your changes don't break existing functionality
5. Submit a pull request

## Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ordokr/LMS.git
   cd LMS/tools/unified-analyzer
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

## Project Structure

- `src/analyzers/` - Contains the analyzer implementations
- `src/utils/` - Contains utility functions
- `src/generators/` - Contains documentation generators
- `src/config.rs` - Configuration parser
- `tests/` - Contains test files

## Adding New Analyzers

To add a new analyzer:

1. Create a new file in `src/analyzers/`
2. Add the module to `src/analyzers/mod.rs`
3. Add the analyzer to the `UnifiedProjectAnalyzer` struct
4. Add the analyzer to the `analyze` method

## Adding New Generators

To add a new documentation generator:

1. Create a new file in `src/generators/`
2. Add the module to `src/generators/mod.rs`
3. Re-export the generator function in `src/generators/mod.rs`
4. Add the generator to the main function in `src/main.rs`
5. Update the configuration in `config.toml` to include the new generator

## Coding Standards

- Follow the Rust style guide
- Write clear and concise code
- Add comments to explain complex logic
- Write tests for new functionality
- Update documentation when necessary

## Testing

- Write unit tests for new functionality
- Run all tests before submitting a pull request
- Ensure all tests pass

## Documentation

- Update the README.md when adding new features
- Add comments to explain complex logic
- Update the MAINTENANCE.md when changing maintenance procedures

## Review Process

1. All pull requests will be reviewed by a maintainer
2. Feedback will be provided on the pull request
3. Changes may be requested before the pull request is merged
4. Once approved, the pull request will be merged

## License

By contributing to this project, you agree that your contributions will be licensed under the project's license.
