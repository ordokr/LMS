@echo off
echo Running Simple Cleanup...

echo Removing original project_analyzer.rs files...

if exist "%~dp0..\..\src\bin\project_analyzer.rs" (
    del "%~dp0..\..\src\bin\project_analyzer.rs"
    echo Removed src\bin\project_analyzer.rs
) else (
    echo src\bin\project_analyzer.rs not found
)

if exist "%~dp0..\..\src-tauri\src\bin\project_analyzer.rs" (
    del "%~dp0..\..\src-tauri\src\bin\project_analyzer.rs"
    echo Removed src-tauri\src\bin\project_analyzer.rs
) else (
    echo src-tauri\src\bin\project_analyzer.rs not found
)

echo.
echo Simple Cleanup completed.
pause
