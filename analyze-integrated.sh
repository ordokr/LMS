#!/bin/bash
cd "$(dirname "$0")"
cd tools\unified-analyzer
if [ ! -d "target" ]; then
    echo "Building analyzer..."
    cargo build --release
fi
./target/release/unified-analyzer --integrated "$@"
