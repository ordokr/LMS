# Running Quenti Module Standalone

This document provides instructions for running the Quenti module as a standalone application.

## Prerequisites

- Rust and Cargo (latest stable version)
- Node.js and npm
- Tauri CLI: `cargo install tauri-cli`

## Setup

Before running the application for the first time, you need to set up the environment:

### Windows

1. Double-click the `setup-quenti-standalone.bat` file
2. Wait for the setup to complete

### macOS/Linux

1. Open a terminal in this directory
2. Run: `./setup-quenti-standalone.sh`
3. Wait for the setup to complete

## Quick Start

After setting up the environment:

### Windows

1. Double-click the `run-quenti-standalone.bat` file
2. The application will build and launch automatically

### macOS/Linux

1. Open a terminal in this directory
2. Run: `./run-quenti-standalone.sh`
3. The application will build and launch automatically

## Manual Launch

If the scripts don't work for you, follow these manual steps:

1. Navigate to the src-tauri directory:
   ```
   cd src-tauri
   ```

2. Run the application in development mode:
   ```
   cargo tauri dev
   ```

3. If that doesn't work, try running the quiz-standalone binary:
   ```
   cargo run --bin quiz-standalone
   ```

4. If you want to build a production version:
   ```
   cargo tauri build
   ```

## Troubleshooting

If you encounter workspace-related errors:

1. Make sure you're running the commands from the correct directory
2. Check that the workspace configurations in Cargo.toml files don't conflict
3. Try running with the `--manifest-path` option:
   ```
   cargo tauri dev --manifest-path src-tauri/Cargo.toml
   ```

## Testing

To run tests for specific components:

```
cd src-tauri
cargo test -p lms_lib
```

## Features

The Quenti module includes:

- Quiz creation and management
- Various question types
- Analytics and visualization
- cmi5 integration
- Offline functionality
- Mobile-responsive design
