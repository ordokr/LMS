@echo off
REM Generate Statistical Trend Report Script
REM This script generates a statistical trend analysis report

echo Generating Statistical Trend Analysis Report
echo =========================================

REM Run the quick analysis
call unified-analyze.bat --quick

if %ERRORLEVEL% NEQ 0 (
    echo Analysis failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

REM Generate the statistical trend report
cd modules\analyzer
cargo run --bin analyze -- --quick

if %ERRORLEVEL% NEQ 0 (
    echo Failed to generate statistical trend report.
    exit /b %ERRORLEVEL%
)

echo.
echo Statistical trend report generated successfully.
echo Report is available at: docs\trends\statistical_trend_report.md
echo Summary is available at: docs\statistical_trend_summary.md
echo.
echo Opening statistical trend report...
start "" "..\..\docs\trends\statistical_trend_report.md"
