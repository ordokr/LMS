#!/bin/bash

echo "Launching Ordo Quiz Module Standalone..."

# Change to the src-tauri directory
cd src-tauri

# Check if we're in development or production mode
if [ -f "Cargo.toml" ]; then
    echo "Development environment detected"
    
    # Try to build and run with Tauri
    echo "Building and running with Tauri..."
    if command -v cargo &> /dev/null; then
        if cargo tauri dev; then
            echo "Ordo Quiz launched successfully with Tauri"
            exit 0
        else
            echo "Failed to launch with Tauri, trying direct binary..."
        fi
    else
        echo "Cargo not found, trying direct binary..."
    fi
    
    # Try to run the binary directly
    if [ -f "target/debug/quiz-standalone" ]; then
        echo "Running debug binary..."
        ./target/debug/quiz-standalone
        exit $?
    elif [ -f "target/release/quiz-standalone" ]; then
        echo "Running release binary..."
        ./target/release/quiz-standalone
        exit $?
    else
        echo "Building binary..."
        cargo build --bin quiz-standalone
        if [ $? -eq 0 ]; then
            echo "Running newly built binary..."
            ./target/debug/quiz-standalone
            exit $?
        else
            echo "Failed to build binary"
            exit 1
        fi
    fi
else
    echo "Production environment detected"
    
    # Try to find and run the binary
    if [ -f "quiz-standalone" ]; then
        ./quiz-standalone
        exit $?
    elif [ -f "bin/quiz-standalone" ]; then
        ./bin/quiz-standalone
        exit $?
    else
        echo "Could not find Ordo Quiz binary"
        exit 1
    fi
fi

echo "Failed to launch Ordo Quiz Module"
exit 1
