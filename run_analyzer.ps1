# Script to run the project analyzer for JavaScript to Rust migration

Write-Host "Starting JavaScript to Rust migration analysis..." -ForegroundColor Cyan

# Store the current location
$originalLocation = Get-Location

# Calculate the project root (assuming script is in the project root)
$projectRoot = $originalLocation.Path

# Navigate to the tools/project-analyzer directory
$analyzerPath = Join-Path -Path $projectRoot -ChildPath "tools\project-analyzer"
Set-Location -Path $analyzerPath

# Build the project analyzer
Write-Host "Building project analyzer..." -ForegroundColor Yellow
cargo build --release

# Check if build succeeded
if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed with exit code $LASTEXITCODE" -ForegroundColor Red
    Set-Location -Path $originalLocation
    exit $LASTEXITCODE
}

# Calculate the executable path
$exePath = Join-Path -Path $analyzerPath -ChildPath "target\release\project-analyzer.exe"

# Run the analyzer
Write-Host "Running analysis..." -ForegroundColor Yellow
Write-Host "Using project root: $projectRoot" -ForegroundColor Yellow

# Run the analyzer with the update option
& $exePath --project-root="$projectRoot" --update

# Check if analyzer succeeded
if ($LASTEXITCODE -ne 0) {
    Write-Host "Analysis failed with exit code $LASTEXITCODE" -ForegroundColor Red
    Set-Location -Path $originalLocation
    exit $LASTEXITCODE
}

Write-Host "Analysis completed successfully!" -ForegroundColor Green

# Return to the original directory
Set-Location -Path $originalLocation