# Ordo Quiz Standalone

This document provides instructions for running the Ordo Quiz module as a standalone application within the Ordo LMS environment.

## Prerequisites

- Rust and Cargo (latest stable version)
- SQLite3 installed and available in your PATH
- The Ordo LMS project

## Quick Start

### Windows

1. Double-click the `launch-ordo-quiz.bat` file
2. The application will build and launch automatically

### macOS/Linux

1. Open a terminal in this directory
2. Run: `./launch-ordo-quiz.sh`
3. The application will build and launch automatically

## Creating a Desktop Shortcut

If you want to create a permanent desktop shortcut:

### Windows

1. Double-click the `build-ordo-quiz-standalone.bat` file
2. Wait for the build to complete
3. A shortcut named "Ordo Quiz" will be created on your desktop

### macOS/Linux

1. Open a terminal in this directory
2. Run: `./build-ordo-quiz-standalone.sh`
3. Wait for the build to complete
4. A shortcut named "ordo-quiz" will be created on your desktop

## Manual Launch

You can also run the Ordo Quiz module directly from the command line:

1. Open a terminal or command prompt
2. Navigate to the Ordo LMS project directory
3. If you've already built the binary, run it directly:
   ```
   # From the project root
   ./target/debug/quiz-standalone    # macOS/Linux
   .\target\debug\quiz-standalone.exe  # Windows
   ```
4. Or build and run in one step:
   ```
   # From the src-tauri directory
   cd src-tauri
   cargo run --bin quiz-standalone
   ```

## Features

The Ordo Quiz module includes:

- Quiz creation and management
- Various question types
- Analytics and visualization
- cmi5 integration
- Offline functionality
- Mobile-responsive design

## Troubleshooting

If you encounter build errors, try the following solutions:

### Database Errors

If you see errors like `unable to open database file`:

1. Run `initialize-database.bat` to create a new database file
2. Make sure SQLite3 is installed and in your PATH
3. Check that the `data` directory exists in the `src-tauri` directory
4. Verify that you have write permissions to the directory

### Compilation Errors

If you see errors about missing modules or unresolved imports:

1. Run `fix-compilation-errors.bat` to create missing files and fix common issues
2. Run `build-minimal-ordo-quiz.bat` to build a minimal working version

### Linking Errors (LNK1104)

If you see errors like `LINK : fatal error LNK1104: cannot open file...`:

1. Run `clean-and-build.bat` to clean the build directory and try again
2. Run `check-disk-space.bat` to check for disk space and permission issues
3. Try building with different options using `build-with-options.bat`
4. Temporarily disable antivirus real-time scanning for the project directory
5. Restart your computer to release any locked files

### Other Build Errors

1. Make sure you have the latest Rust toolchain installed
2. Try using the nightly toolchain: `rustup default nightly`
3. Check that you have all required dependencies installed
4. Look for specific error messages and search for solutions online
