#!/bin/bash
echo "Launching Ordo Quiz Standalone..."

# Check if the binary exists in the target directory
if [ -f "target/release/quiz-standalone" ]; then
    echo "Found release binary, launching..."
    "./target/release/quiz-standalone" &
    exit 0
fi

if [ -f "target/debug/quiz-standalone" ]; then
    echo "Found debug binary, launching..."
    "./target/debug/quiz-standalone" &
    exit 0
fi

if [ -f "src-tauri/target/debug/quiz-standalone" ]; then
    echo "Found debug binary in src-tauri, launching..."
    "./src-tauri/target/debug/quiz-standalone" &
    exit 0
fi

# If binary not found, try to build the minimal version
echo "Binary not found, attempting to build minimal version..."

# Run the minimal build script
./build-minimal-ordo-quiz.sh

# Check if the build was successful
if [ -f "src-tauri/target/debug/quiz-standalone" ]; then
    echo "Build successful, launching..."
    "./src-tauri/target/debug/quiz-standalone" &
    exit 0
fi

echo "Could not find or build the Ordo Quiz module."
echo "Please check the error messages above."
read -p "Press Enter to continue..."
