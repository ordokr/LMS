@echo off
REM Unified Project Analyzer Script
REM This script runs the consolidated codebase analysis tool

echo LMS Project Analysis Tool
echo =======================

set "COMMAND=quick"
set "TARGET_DIRS="
set "EXCLUDE_PATTERNS="
set "OUTPUT_DIR="
set "UPDATE_RAG=false"
set "GENERATE_AI=false"
set "ANALYZE_JS=false"
set "DASHBOARD=false"
set "TECH_DEBT=false"
set "CODE_QUALITY=false"
set "MODELS=false"

REM Parse command line arguments
:parse_args
if "%~1"=="" goto :after_args
if /i "%~1"=="--full" set "COMMAND=full" & goto :next_arg
if /i "%~1"=="--quick" set "COMMAND=quick" & goto :next_arg
if /i "%~1"=="--update-hub" set "COMMAND=update-hub" & goto :next_arg
if /i "%~1"=="--summary" set "COMMAND=summary" & goto :next_arg
if /i "%~1"=="--update-rag" set "COMMAND=update-rag" & goto :next_arg
if /i "%~1"=="--target-dirs" set "TARGET_DIRS=%~2" & shift & goto :next_arg
if /i "%~1"=="--exclude" set "EXCLUDE_PATTERNS=%~2" & shift & goto :next_arg
if /i "%~1"=="--output" set "OUTPUT_DIR=%~2" & shift & goto :next_arg
if /i "%~1"=="--rag" set "UPDATE_RAG=true" & goto :next_arg
if /i "%~1"=="--ai" set "GENERATE_AI=true" & goto :next_arg
if /i "%~1"=="--js" set "ANALYZE_JS=true" & goto :next_arg
if /i "%~1"=="--dashboard" set "DASHBOARD=true" & goto :next_arg
if /i "%~1"=="--tech-debt" set "TECH_DEBT=true" & goto :next_arg
if /i "%~1"=="--code-quality" set "CODE_QUALITY=true" & goto :next_arg
if /i "%~1"=="--models" set "MODELS=true" & goto :next_arg
if /i "%~1"=="--help" goto :show_help
echo Unknown option: %~1
goto :show_help

:next_arg
shift
goto :parse_args

:after_args
echo Running analysis with command: %COMMAND%

REM Build options string for cargo run
set "OPTIONS="

if "%COMMAND%"=="full" (
    set "OPTIONS=full"
    if not "%TARGET_DIRS%"=="" set "OPTIONS=%OPTIONS% --target-dirs %TARGET_DIRS%"
    if not "%EXCLUDE_PATTERNS%"=="" set "OPTIONS=%OPTIONS% --exclude %EXCLUDE_PATTERNS%"
    if not "%OUTPUT_DIR%"=="" set "OPTIONS=%OPTIONS% --output %OUTPUT_DIR%"
    if "%UPDATE_RAG%"=="true" set "OPTIONS=%OPTIONS% --update-rag"
    if "%GENERATE_AI%"=="true" set "OPTIONS=%OPTIONS% --generate-insights"
    if "%ANALYZE_JS%"=="true" set "OPTIONS=%OPTIONS% --analyze-js"
    if "%DASHBOARD%"=="true" set "OPTIONS=%OPTIONS% --dashboard"
    if "%TECH_DEBT%"=="true" set "OPTIONS=%OPTIONS% --tech-debt"
    if "%CODE_QUALITY%"=="true" set "OPTIONS=%OPTIONS% --code-quality"
    if "%MODELS%"=="true" set "OPTIONS=%OPTIONS% --models"
) else (
    set "OPTIONS=%COMMAND%"
)

REM Run the analyzer
echo.
echo Starting analysis...
echo.

cd modules\analyzer
cargo run --bin analyze %OPTIONS%

echo.
echo Analysis completed.

REM Open dashboard if it was generated
if "%DASHBOARD%"=="true" (
    echo Opening analysis dashboard...
    start "" "..\..\docs\dashboard.html"
)

goto :eof

:show_help
echo.
echo Usage: unified-analyze.bat [command] [options]
echo.
echo Commands:
echo   --full        Run complete analysis
echo   --quick       Run quick analysis (default)
echo   --update-hub  Update central reference hub
echo   --summary     Generate summary report
echo   --update-rag  Update RAG knowledge base
echo.
echo Options for --full command:
echo   --target-dirs DIR1,DIR2,...  Target directories to analyze
echo   --exclude PAT1,PAT2,...      Patterns to exclude
echo   --output DIR                 Output directory for documentation
echo   --rag                        Update RAG knowledge base
echo   --ai                         Generate AI insights
echo   --js                         Analyze JavaScript files for Rust migration
echo   --dashboard                  Generate visual dashboard
echo   --tech-debt                  Analyze technical debt
echo   --code-quality                Analyze code quality
echo   --models                      Analyze data models
echo   --help                       Show this help message
echo.
echo Example: unified-analyze.bat --full --rag --js --dashboard --tech-debt --code-quality --models
goto :eof
