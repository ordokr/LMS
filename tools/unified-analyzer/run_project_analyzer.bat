@echo off
echo Running Project Analyzer...

cd %~dp0..\..\modules\analyzer
cargo run --bin project-analyzer

echo Project Analyzer completed.
pause
