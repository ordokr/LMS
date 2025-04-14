@echo off
REM Test Script for Analyzers
REM This script runs all analyzers and generates reports

echo LMS Project Analyzer Test
echo ========================

REM Run the full analysis with all options
call unified-analyze.bat --full --tech-debt --code-quality --models --dashboard

REM Check if the analysis was successful
if %ERRORLEVEL% NEQ 0 (
    echo Analysis failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

echo.
echo Analysis completed successfully.
echo.
echo Generated reports:
echo - docs\central_reference_hub.md
echo - docs\SUMMARY_REPORT.md
echo - docs\technical_debt_report.md
echo - docs\code_quality_summary.md
echo - docs\model_summary.md
echo - docs\dashboard.html
echo.
echo Opening dashboard...
start "" "docs\dashboard.html"

echo.
echo Test completed.
