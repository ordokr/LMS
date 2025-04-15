# Script to remove obsolete files
Write-Host "Removing obsolete files..."

# Analyzers to remove
$analyzers = @(
    "ast_analyzer.rs",
    "blockchain_analyzer.rs",
    "db_schema_analyzer.rs",
    "js_migration_analyzer.rs",
    "tech_debt_analyzer.rs",
    "trend_analyzer.rs",
    "unified_analyzer.rs"
)

# Remove analyzers from src-tauri/src/analyzers
foreach ($analyzer in $analyzers) {
    $path = "../../src-tauri/src/analyzers/$analyzer"
    if (Test-Path $path) {
        Write-Host "Removing $analyzer from src-tauri/src/analyzers..."
        Remove-Item -Path $path -Force
    } else {
        Write-Host "Warning: $path not found"
    }
}

Write-Host "Obsolete files removed successfully."
