@echo off
setlocal enabledelayedexpansion

REM Parse command line arguments
set OPTIONS=
set COMMAND=redundancy

:parse_args
if "%~1"=="" goto :done_parsing
set OPTIONS=%OPTIONS% %1
shift
goto :parse_args

:done_parsing

cd %~dp0
cargo run --bin unified-analyzer -- %COMMAND% %OPTIONS%

endlocal
