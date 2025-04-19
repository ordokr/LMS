@echo off
echo Running Quenti Module Standalone...

cd src-tauri
echo Building and running the application...
cargo tauri dev

if %ERRORLEVEL% NEQ 0 (
    echo Failed to run with 'cargo tauri dev', trying alternative method...
    cargo run --bin quiz-standalone

    if %ERRORLEVEL% NEQ 0 (
        echo Failed to run with 'cargo run', trying to build and run manually...
        cargo build

        if %ERRORLEVEL% EQU 0 (
            echo Build successful, running the application...
            .\target\debug\lms-tauri.exe
        ) else (
            echo Build failed. Please check the error messages above.
        )
    )
)

cd ..
echo Done.
