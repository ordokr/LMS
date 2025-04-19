#!/bin/bash
echo "Launching Quenti Standalone..."

cd ~/Desktop/quenti
cargo run --bin quiz-standalone

if [ $? -ne 0 ]; then
    echo "Failed to run with 'cargo run', trying to build and run manually..."
    cargo build --bin quiz-standalone
    
    if [ $? -eq 0 ]; then
        echo "Build successful, running the application..."
        ./target/debug/quiz-standalone
    else
        echo "Build failed. Please check the error messages above."
        read -p "Press Enter to continue..."
    fi
fi
