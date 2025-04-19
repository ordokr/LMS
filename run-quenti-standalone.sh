#!/bin/bash
echo "Running Quenti Module Standalone..."

cd src-tauri
echo "Building and running the application..."
cargo tauri dev

if [ $? -ne 0 ]; then
    echo "Failed to run with 'cargo tauri dev', trying alternative method..."
    cargo run --bin quiz-standalone

    if [ $? -ne 0 ]; then
        echo "Failed to run with 'cargo run', trying to build and run manually..."
        cargo build

        if [ $? -eq 0 ]; then
            echo "Build successful, running the application..."
            ./target/debug/lms-tauri
        else
            echo "Build failed. Please check the error messages above."
        fi
    fi
fi

cd ..
echo "Done."
