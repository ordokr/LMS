# Script to migrate analyzers to the unified analyzer
Write-Host "Migrating analyzers to the unified analyzer..."

# Create directories for the migrated analyzers
$targetDir = "src/analyzers/modules"
if (-not (Test-Path $targetDir)) {
    New-Item -Path $targetDir -ItemType Directory -Force
}

# Copy analyzers from src-tauri/src/analyzers
$srcTauriAnalyzers = @(
    "ast_analyzer.rs",
    "blockchain_analyzer.rs",
    "db_schema_analyzer.rs",
    "js_migration_analyzer.rs",
    "tech_debt_analyzer.rs",
    "trend_analyzer.rs",
    "unified_analyzer.rs"
)

foreach ($analyzer in $srcTauriAnalyzers) {
    $sourcePath = "../LMS/src-tauri/src/analyzers/$analyzer"
    $targetPath = "$targetDir/$analyzer"

    if (Test-Path $sourcePath) {
        Write-Host "Copying $analyzer from src-tauri/src/analyzers..."
        Copy-Item -Path $sourcePath -Destination $targetPath -Force
    } else {
        Write-Host "Warning: $sourcePath not found"
    }
}

# Copy analyzers from tools/project-analyzer/src
$projectAnalyzers = @(
    "ast_analyzer.rs",
    "conflict_analyzer.rs",
    "js_migration_analyzer.rs",
    "unified_project_analyzer.rs"
)

foreach ($analyzer in $projectAnalyzers) {
    $sourcePath = "../LMS/tools/project-analyzer/src/$analyzer"
    $targetPath = "$targetDir/$analyzer"

    if (Test-Path $sourcePath) {
        Write-Host "Copying $analyzer from tools/project-analyzer/src..."
        Copy-Item -Path $sourcePath -Destination $targetPath -Force
    } else {
        Write-Host "Warning: $sourcePath not found"
    }
}

# Create a modules.rs file to export all the modules
$modulesContent = "// Migrated analyzer modules
"

foreach ($analyzer in ($srcTauriAnalyzers + $projectAnalyzers | Sort-Object -Unique)) {
    $moduleName = $analyzer -replace "\.rs$", ""
    $modulesContent += "pub mod $moduleName;`n"
}

Set-Content -Path "$targetDir/mod.rs" -Value $modulesContent

Write-Host "Migration complete. Please update the unified analyzer to use the migrated modules."
