@echo off
setlocal enabledelayedexpansion

echo ===================================
echo Running Unified Analyzer Tests
echo ===================================
echo.

:: Change to the script's directory
cd %~dp0

:: Run the tests
echo Running tests...
echo.

cargo test -- --nocapture

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Error: Tests failed with error code %ERRORLEVEL%
    echo.
    pause
    exit /b %ERRORLEVEL%
)

echo.
echo ===================================
echo All tests passed!
echo ===================================
echo.

pause
