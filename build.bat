@echo off
echo LMS Build System

if "%1"=="build" (
    cargo build
    exit /b
)

if "%1"=="test" (
    cargo test
    exit /b
)

if "%1"=="update-audit" (
    cargo run --bin update_audit
    exit /b
)

if "%1"=="run" (
    cargo run -- %2 %3 %4 %5 %6 %7 %8 %9
    exit /b
)

if "%1"=="doc" (
    cargo doc --no-deps --open
    exit /b
)

echo Unknown command. Available commands:
echo   build        - Build the project
echo   test         - Run tests
echo   update-audit - Update the audit report
echo   run          - Run the application (pass additional args after --)
echo   doc          - Generate and open documentation
