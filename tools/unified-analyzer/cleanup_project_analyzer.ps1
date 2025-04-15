# Script to remove the original project_analyzer.rs files
Write-Host "Removing original project_analyzer.rs files..."

# Files to remove
$files = @(
    "../../src/bin/project_analyzer.rs",
    "../../src-tauri/src/bin/project_analyzer.rs"
)

# Check if the binary is still defined in the root Cargo.toml
$cargoToml = Get-Content "../../Cargo.toml" -Raw
if ($cargoToml -match '\[\[bin\]\]\s*name\s*=\s*"project-analyzer"') {
    Write-Host "Warning: Binary 'project-analyzer' is still defined in the root Cargo.toml"
    Write-Host "It has been commented out, but you may want to remove it completely"
}

# Verify that the binary is properly defined in the modules/analyzer/Cargo.toml
$analyzerCargoToml = Get-Content "../../modules/analyzer/Cargo.toml" -Raw
if (-not ($analyzerCargoToml -match '\[\[bin\]\]\s*name\s*=\s*"project-analyzer"')) {
    Write-Host "Warning: Binary 'project-analyzer' is not defined in the modules/analyzer/Cargo.toml"
    Write-Host "Please ensure it is properly defined"
}

# Remove each file
foreach ($file in $files) {
    if (Test-Path $file) {
        Write-Host "Removing $file..."
        Remove-Item -Path $file -Force
    } else {
        Write-Host "Warning: $file not found"
    }
}

Write-Host "Cleanup completed successfully."
