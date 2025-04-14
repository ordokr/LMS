@echo off
REM Generate LLM Insights Script
REM This script generates insights using LM Studio with Qwen 2.5

echo Generating LLM Insights
echo =====================

REM Check if LM Studio is running
echo Checking if LM Studio is running...
curl -s http://localhost:1234/v1/models > nul
if %ERRORLEVEL% NEQ 0 (
    echo LM Studio is not running.
    echo Please start LM Studio and ensure it's running on http://localhost:1234
    echo.
    echo 1. Open LM Studio
    echo 2. Load the Qwen 2.5 model
    echo 3. Start the server
    echo.
    echo Then run this script again.
    exit /b 1
)

echo LM Studio is running.

REM Run the quick analysis
call unified-analyze.bat --quick

if %ERRORLEVEL% NEQ 0 (
    echo Analysis failed with error code %ERRORLEVEL%
    exit /b %ERRORLEVEL%
)

REM Generate the LLM insights
cd modules\analyzer
cargo run --bin generate_llm_insights

if %ERRORLEVEL% NEQ 0 (
    echo Failed to generate LLM insights.
    exit /b %ERRORLEVEL%
)

echo.
echo LLM insights generated successfully.
echo Report is available at: docs\insights\llm_insights_report.md
echo.
echo Opening insights report...
start "" "..\..\docs\insights\llm_insights_report.md"
