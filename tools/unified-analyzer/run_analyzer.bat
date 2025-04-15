@echo off
setlocal enabledelayedexpansion

echo ===================================
echo Unified Analyzer for LMS Project
echo ===================================
echo.

:: Change to the script's directory
cd %~dp0

:: Check if a path was provided
if "%~1"=="" (
    echo No path provided. Analyzing current directory...
    set "target_dir=%CD%"
) else (
    echo Analyzing directory: %~1
    set "target_dir=%~1"
)

:: Check if the config file exists
if exist "config.toml" (
    echo Using configuration from config.toml
) else (
    echo Warning: config.toml not found. Using default configuration.
)

:: Run the analyzer
echo.
echo Running analyzer...
echo.

cargo run -- "!target_dir!"

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error: Analyzer failed with error code %ERRORLEVEL%
    echo.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo ===================================
echo Analyzer completed successfully!
echo ===================================
echo.
echo Documentation generated in the docs directory.
echo.

:: Open the central reference hub
echo Would you like to open the central reference hub? (Y/N)
set /p open_hub=

if /i "!open_hub!"=="Y" (
    if exist "docs\central_reference_hub.md" (
        start "" "docs\central_reference_hub.md"
    ) else (
        echo Error: Central reference hub not found.
    )
)

pause
