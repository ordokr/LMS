#!/usr/bin/env pwsh
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir
Set-Location tools\unified-analyzer

if (-not (Test-Path "target")) {
    Write-Host "Building analyzer..."
    cargo build --release
}

& .\target\release\unified-analyzer.exe --integrated $args
