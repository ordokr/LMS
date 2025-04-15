# Run the unified analyzer tests
$ErrorActionPreference = "Stop"

# Get the directory of the script
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# Change to the script directory
Set-Location $scriptDir

# Run the tests
Write-Host ""
Write-Host "Running tests..." -ForegroundColor Cyan
Write-Host ""

try {
    cargo test -- --nocapture
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "Error: Tests failed with exit code $LASTEXITCODE" -ForegroundColor Red
        Write-Host ""
        exit $LASTEXITCODE
    }
    
    Write-Host ""
    Write-Host "===================================" -ForegroundColor Green
    Write-Host "All tests passed!" -ForegroundColor Green
    Write-Host "===================================" -ForegroundColor Green
    Write-Host ""
} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
