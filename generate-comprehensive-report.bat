@echo off
REM Generate Comprehensive Report Script
REM This script generates a comprehensive analysis report

echo Generating Comprehensive Analysis Report
echo ======================================

cd modules\analyzer
cargo run --bin generate_comprehensive_report

if %ERRORLEVEL% NEQ 0 (
    echo Failed to generate comprehensive report.
    exit /b %ERRORLEVEL%
)

echo.
echo Comprehensive report generated successfully.
echo Report is available at: docs\comprehensive_report.md
echo.
echo Opening report...
start "" "..\docs\comprehensive_report.md"
