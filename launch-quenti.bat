@echo off
echo Launching Quenti Standalone...

cd C:\Users\Tim\Desktop\quenti
cargo run --bin quiz-standalone

if %ERRORLEVEL% NEQ 0 (
    echo Failed to run with 'cargo run', trying to build and run manually...
    cargo build --bin quiz-standalone
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful, running the application...
        .\target\debug\quiz-standalone.exe
    ) else (
        echo Build failed. Please check the error messages above.
        pause
    )
)
