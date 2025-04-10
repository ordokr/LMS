@echo off
REM Run the Rust project analyzer

echo Running LMS Project Analyzer...

cd %~dp0
cargo run --package project-analyzer

echo Analysis complete. Check the docs/ directory for results.