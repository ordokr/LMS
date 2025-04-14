@echo off
REM Generate Trend Report Script
REM This script generates a trend analysis report

echo Generating Trend Analysis Report
echo ==============================

REM Run the quick analysis
call unified-analyze.bat --quick

if %ERRORLEVEL% NEQ 0 (
    echo Analysis failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

REM Generate the trend report
cd modules\analyzer
cargo run --bin analyze -- --quick

if %ERRORLEVEL% NEQ 0 (
    echo Failed to generate trend report.
    exit /b %ERRORLEVEL%
)

echo.
echo Trend report generated successfully.
echo Report is available at: docs\trends\trend_report.md
echo Summary is available at: docs\trend_summary.md
echo.
echo Opening trend report...
start "" "..\..\docs\trends\trend_report.md"
