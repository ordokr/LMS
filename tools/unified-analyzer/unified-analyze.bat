@echo off
setlocal enabledelayedexpansion

set COMMAND=quick
set OPTIONS=

:parse_args
if "%~1"=="" goto run
if "%~1"=="--full" (
    set COMMAND=full
    shift
    goto parse_args
)
if "%~1"=="--quick" (
    set COMMAND=quick
    shift
    goto parse_args
)
if "%~1"=="--update-hub" (
    set COMMAND=update-hub
    shift
    goto parse_args
)
if "%~1"=="--summary" (
    set COMMAND=summary
    shift
    goto parse_args
)
if "%~1"=="--update-rag" (
    set COMMAND=update-rag
    shift
    goto parse_args
)
if "%~1"=="--project" (
    set COMMAND=project
    shift
    goto parse_args
)
if "%~1"=="--roadmap" (
    set COMMAND=roadmap
    shift
    goto parse_args
)
if "%~1"=="--component-tree" (
    set COMMAND=component-tree
    shift
    goto parse_args
)
if "%~1"=="--api-map" (
    set COMMAND=api-map
    shift
    goto parse_args
)
if "%~1"=="--viz" (
    set COMMAND=viz
    shift
    goto parse_args
)

set OPTIONS=%OPTIONS% %1
shift
goto parse_args

:run
echo Running Unified Analyzer with command: %COMMAND% %OPTIONS%

cd %~dp0
if "%COMMAND%"=="full" (
    cargo run --bin unified-analyzer -- full %OPTIONS%
) else if "%COMMAND%"=="quick" (
    cargo run --bin unified-analyzer -- quick %OPTIONS%
) else if "%COMMAND%"=="update-hub" (
    cargo run --bin unified-analyzer -- update-hub %OPTIONS%
) else if "%COMMAND%"=="summary" (
    cargo run --bin unified-analyzer -- summary %OPTIONS%
) else if "%COMMAND%"=="update-rag" (
    cargo run --bin unified-analyzer -- update-rag %OPTIONS%
) else if "%COMMAND%"=="project" (
    cargo run --bin project-analyzer %OPTIONS%
) else if "%COMMAND%"=="roadmap" (
    cargo run --bin unified-analyzer -- roadmap %OPTIONS%
) else if "%COMMAND%"=="component-tree" (
    cargo run --bin unified-analyzer -- component-tree %OPTIONS%
) else if "%COMMAND%"=="api-map" (
    cargo run --bin unified-analyzer -- api-map %OPTIONS%
) else if "%COMMAND%"=="viz" (
    cargo run --bin unified-analyzer -- viz %OPTIONS%
) else (
    echo Unknown command: %COMMAND%
    echo Available commands:
    echo   --full            Run full analysis
    echo   --quick           Run quick analysis
    echo   --update-hub      Update central reference hub
    echo   --summary         Generate summary report
    echo   --update-rag      Update RAG knowledge base
    echo   --project         Run project analyzer
    echo   --roadmap         Generate migration roadmap
    echo   --component-tree  Generate component tree visualization
    echo   --api-map         Generate API map visualization
    echo   --viz             Generate all visualizations
    exit /b 1
)

echo Unified Analyzer completed.
pause
