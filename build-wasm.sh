#!/bin/bash
# Build the Rust FileSystemUtils module for WebAssembly

echo "Building Rust FileSystemUtils for WebAssembly..."

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null
then
    echo "wasm-pack is not installed. Installing..."
    cargo install wasm-pack
fi

# Build the Rust code targeting WebAssembly
wasm-pack build --target web --out-dir pkg

echo "Build completed. WebAssembly module is available in ./pkg directory"
