@echo off
REM Run the Rust project analyzer

echo Running LMS Project Analyzer...

cd %~dp0
cargo run --bin analyze full

echo Analysis complete. Check the docs/central_reference_hub.md file for results.
