# Run the unified analyzer
# IMPORTANT: The unified analyzer is for source-to-source/code/feature porting only. It does NOT support or perform data migration, user import, or live system integration. All analysis and recommendations are for codebase transformation and feature parity only.
$ErrorActionPreference = "Stop"

# Get the directory of the script
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Change to the script directory
Set-Location $scriptDir

Write-Host "============================================================" -ForegroundColor Yellow
Write-Host "Unified Analyzer: Source-to-Source Porting Only" -ForegroundColor Yellow
Write-Host "This tool does NOT perform data migration or live system integration." -ForegroundColor Yellow
Write-Host "All analysis is for code, schema, and feature porting only." -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Yellow

# Check if a path was provided
if ($args.Count -gt 0) {
    $targetDir = $args[0]
    Write-Host "Analyzing directory: $targetDir"
} else {
    $targetDir = Get-Location
    Write-Host "No path provided. Analyzing current directory: $targetDir"
}

# Check if the config file exists
if (Test-Path "config.toml") {
    Write-Host "Using configuration from config.toml"
} else {
    Write-Host "Warning: config.toml not found. Using default configuration." -ForegroundColor Yellow
}

# Run the analyzer
Write-Host ""
Write-Host "Running analyzer..." -ForegroundColor Cyan
Write-Host ""

try {
    cargo run -- "$targetDir"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "Error: Analyzer failed with exit code $LASTEXITCODE" -ForegroundColor Red
        Write-Host ""
        exit $LASTEXITCODE
    }
    
    Write-Host ""
    Write-Host "===================================" -ForegroundColor Green
    Write-Host "Analyzer completed successfully!" -ForegroundColor Green
    Write-Host "===================================" -ForegroundColor Green
    Write-Host ""
    Write-Host "Documentation generated in the docs directory."
    Write-Host ""
    
    # Ask if the user wants to open the central reference hub
    $openHub = Read-Host "Would you like to open the central reference hub? (Y/N)"
    
    if ($openHub -eq "Y" -or $openHub -eq "y") {
        if (Test-Path "docs\central_reference_hub.md") {
            Start-Process "docs\central_reference_hub.md"
        } else {
            Write-Host "Error: Central reference hub not found." -ForegroundColor Red
        }
    }
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
