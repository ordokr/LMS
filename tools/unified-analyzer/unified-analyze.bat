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
) else (
    echo Unknown command: %COMMAND%
    exit /b 1
)

echo Unified Analyzer completed.
pause
