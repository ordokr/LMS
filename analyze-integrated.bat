@echo off
cd %~dp0
cd tools\unified-analyzer
if not exist "target" (
    echo Building analyzer...
    cargo build --release
)
target\release\unified-analyzer --integrated %*
