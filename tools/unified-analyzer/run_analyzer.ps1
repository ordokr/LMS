# PowerShell script to run the project analyzer
Write-Host "Running Project Analyzer..."

# Change to the modules/analyzer directory
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Path
$analyzerPath = Join-Path -Path $scriptPath -ChildPath "..\..\modules\analyzer"
Set-Location -Path $analyzerPath

# Run the project analyzer
cargo run --bin project-analyzer

Write-Host "Project Analyzer completed."
Read-Host -Prompt "Press Enter to continue"
