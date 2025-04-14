@echo off
REM Generate Enhanced Dashboard Script
REM This script generates an enhanced visual dashboard

echo Generating Enhanced Dashboard
echo ===========================

REM Run the quick analysis
call unified-analyze.bat --quick

if %ERRORLEVEL% NEQ 0 (
    echo Analysis failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

REM Generate the enhanced dashboard
cd modules\analyzer
cargo run --bin analyze -- --quick

if %ERRORLEVEL% NEQ 0 (
    echo Failed to generate enhanced dashboard.
    exit /b %ERRORLEVEL%
)

echo.
echo Enhanced dashboard generated successfully.
echo Dashboard is available at: docs\enhanced_dashboard.html
echo.
echo Opening dashboard...
start "" "..\..\docs\enhanced_dashboard.html"
