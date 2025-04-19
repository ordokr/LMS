@echo off
echo Building Ordo Quiz Module...

cd src-tauri
cargo build --bin quiz-standalone --release

if %ERRORLEVEL% neq 0 (
    echo Build failed. Please check the error messages above.
    cd ..
    exit /b 1
)

echo Build completed successfully.
echo The binary is available at: src-tauri\target\release\quiz-standalone.exe
cd ..
