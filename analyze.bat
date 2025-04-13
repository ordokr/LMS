@echo off
REM Project Analyzer Script
REM This script runs the codebase analysis tool with various options

echo LMS Project Analysis Tool
echo =======================

set "ANALYZE_ALL=false"
set "UPDATE_RAG=false"
set "GENERATE_AI=false"
set "ANALYZE_JS=false"
set "QUICK=false"
set "DASHBOARD=false"
set "TECH_DEBT=false"

REM Parse command line arguments
:parse_args
if "%~1"=="" goto :after_args
if /i "%~1"=="--all" set "ANALYZE_ALL=true" & goto :next_arg
if /i "%~1"=="--rag" set "UPDATE_RAG=true" & goto :next_arg
if /i "%~1"=="--ai" set "GENERATE_AI=true" & goto :next_arg
if /i "%~1"=="--js" set "ANALYZE_JS=true" & goto :next_arg
if /i "%~1"=="--quick" set "QUICK=true" & goto :next_arg
if /i "%~1"=="--dashboard" set "DASHBOARD=true" & goto :next_arg
if /i "%~1"=="--tech-debt" set "TECH_DEBT=true" & goto :next_arg
if /i "%~1"=="--help" goto :show_help
echo Unknown option: %~1
goto :show_help

:next_arg
shift
goto :parse_args

:after_args
if "%ANALYZE_ALL%"=="true" (
    set "UPDATE_RAG=true"
    set "GENERATE_AI=true"
    set "ANALYZE_JS=true"
    set "DASHBOARD=true"
    set "TECH_DEBT=true"
)

REM If no specific flags were set, run a quick analysis
if "%UPDATE_RAG%%GENERATE_AI%%ANALYZE_JS%%QUICK%%DASHBOARD%%TECH_DEBT%"=="falsefalsefalsefalsefalsefalse" set "QUICK=true"

echo Running analysis with options:
if "%QUICK%"=="true" echo - Quick analysis: YES
if "%UPDATE_RAG%"=="true" echo - Update RAG knowledge base: YES
if "%GENERATE_AI%"=="true" echo - Generate AI insights: YES
if "%ANALYZE_JS%"=="true" echo - Analyze JavaScript files: YES
if "%DASHBOARD%"=="true" echo - Generate visual dashboard: YES
if "%TECH_DEBT%"=="true" echo - Analyze technical debt: YES

REM Build options string for cargo run
set "OPTIONS=--"
if "%QUICK%"=="true" set "OPTIONS=%OPTIONS% --quick"
if "%UPDATE_RAG%"=="true" set "OPTIONS=%OPTIONS% --update-rag"
if "%GENERATE_AI%"=="true" set "OPTIONS=%OPTIONS% --generate-ai"
if "%ANALYZE_JS%"=="true" set "OPTIONS=%OPTIONS% --analyze-js"
if "%DASHBOARD%"=="true" set "OPTIONS=%OPTIONS% --dashboard"
if "%TECH_DEBT%"=="true" set "OPTIONS=%OPTIONS% --tech-debt"

REM Run the analyzer
echo.
echo Starting analysis...
echo.

cd src-tauri
cargo run --bin analyze_project %OPTIONS%

echo.
echo Analysis completed.

REM Open dashboard if it was generated
if "%DASHBOARD%"=="true" (
    echo Opening analysis dashboard...
    start "" "..\docs\analysis_dashboard.html"
)

REM Open tech debt report if it was generated
if "%TECH_DEBT%"=="true" (
    echo Opening technical debt report...
    start "" "..\docs\technical_debt_report.md"
)

goto :eof

:show_help
echo.
echo Usage: analyze.bat [options]
echo Options:
echo   --all        Run complete analysis with all features
echo   --rag        Update RAG knowledge base
echo   --ai         Generate AI insights
echo   --js         Analyze JavaScript files for Rust migration
echo   --dashboard  Generate visual dashboard
echo   --tech-debt  Analyze technical debt
echo   --quick      Run quick analysis (default if no options specified)
echo   --help       Show this help message
echo.
echo Example: analyze.bat --rag --js --dashboard
goto :eof
