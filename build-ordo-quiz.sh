#!/bin/bash
echo "Building Ordo Quiz Module..."

cd src-tauri
cargo build --bin quiz-standalone --release

if [ $? -ne 0 ]; then
    echo "Build failed. Please check the error messages above."
    cd ..
    exit 1
fi

echo "Build completed successfully."
echo "The binary is available at: src-tauri/target/release/quiz-standalone"
cd ..
