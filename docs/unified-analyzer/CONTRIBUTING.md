# Contributing to Unified Analyzer

Thank you for your interest in contributing to the Unified Analyzer! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Environment](#development-environment)
- [Project Structure](#project-structure)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Review Process](#review-process)
- [Community](#community)

## Code of Conduct

We expect all contributors to follow our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it before participating.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/unified-analyzer.git`
3. Add the upstream repository: `git remote add upstream https://github.com/original-org/unified-analyzer.git`
4. Create a new branch for your changes: `git checkout -b feature/your-feature-name`

## Development Environment

### Prerequisites

- Rust (latest stable version)
- Cargo
- Git
- Visual Studio Code (recommended) with the following extensions:
  - rust-analyzer
  - Better TOML
  - CodeLLDB

### Setup

1. Install Rust and Cargo using [rustup](https://rustup.rs/)
2. Install dependencies: `cargo build`
3. Run the tests: `cargo test`

## Project Structure

The project is organized as follows:

```
unified-analyzer/
├── src/
│   ├── analyzers/         # Analysis modules
│   │   ├── modules/       # Individual analyzer modules
│   │   └── ...
│   ├── generators/        # Visualization generators
│   ├── integrator/        # Integration of analysis results
│   ├── output_schema/     # Output schema definitions
│   ├── utils/             # Utility functions
│   └── main.rs            # Entry point
├── docs/                  # Documentation
├── tests/                 # Tests
├── examples/              # Example projects
└── ...
```

## Making Changes

1. Make sure your changes are consistent with the project's style and goals
2. Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
3. Keep your changes focused and atomic
4. Write clear, descriptive commit messages

### Adding a New Generator

To add a new generator:

1. Create a new file in `src/generators/` (e.g., `src/generators/my_generator.rs`)
2. Implement the generator using the `UnifiedAnalysisOutput` as input
3. Add the generator to `src/generators/mod.rs`
4. Update the command-line interface in `unified-analyze.bat` to include the new generator
5. Add documentation for the generator in `docs/generators_guide.md`

## Testing

1. Write tests for your changes
2. Make sure all existing tests pass: `cargo test`
3. Test your changes with real-world examples

## Documentation

1. Update the documentation to reflect your changes
2. Follow the existing documentation style
3. Include examples where appropriate

## Submitting Changes

1. Commit your changes: `git commit -m "Add feature X"`
2. Push to your fork: `git push origin feature/your-feature-name`
3. Create a pull request from your fork to the main repository
4. Fill out the pull request template with all required information

## Review Process

1. All pull requests will be reviewed by at least one maintainer
2. Address any feedback or requested changes
3. Once approved, your changes will be merged

## Community

- Join our [Discord server](https://discord.gg/example) to chat with other contributors
- Subscribe to our [mailing list](https://example.com/mailing-list) for updates
- Follow us on [Twitter](https://twitter.com/example) for announcements

## Thank You!

Your contributions help make the Unified Analyzer better for everyone. We appreciate your time and effort!
