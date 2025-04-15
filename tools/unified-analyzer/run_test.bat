@echo off
echo Running Project Analyzer Test...

cd %~dp0..\..\modules\analyzer
cargo run --bin test_project_analyzer

echo Test completed.
pause
