$outputFile = "component_status.md"

# Create the output file with a header
Set-Content -Path $outputFile -Value "# LMS Component Status`n"
Add-Content -Path $outputFile -Value "Generated on: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')`n"

# Create a status table
Add-Content -Path $outputFile -Value "| Component | Status | Path |"
Add-Content -Path $outputFile -Value "|-----------|--------|------|"

$componentsToCheck = @(
    @("Auth API", "..\src-tauri\src\api\auth.rs"),
    @("User Repository", "..\src-tauri\src\database\repositories\user.rs"),
    @("Auth Service", "..\src-tauri\src\core\auth.rs"),
    @("Forum Repository", "..\src-tauri\src\database\repositories\forum.rs"),
    @("Course Repository", "..\src-tauri\src\database\repositories\course.rs"),
    @("Main Entry", "..\src-tauri\src\main.rs"),
    @("Login Component", "..\src\components\auth\login.rs"),
    @("Register Component", "..\src\components\auth\register.rs")
)

foreach ($component in $componentsToCheck) {
    $name = $component[0]
    $path = $component[1]
    
    $status = if (Test-Path $path) { "✅ Complete" } else { "❌ Missing" }
    Add-Content -Path $outputFile -Value "| $name | $status | ``$($path)`` |"
}

# Check directories
$directoriesToCheck = @(
    @("Forum Components", "..\src\components\forum"),
    @("Course Components", "..\src\components\courses")
)

foreach ($dir in $directoriesToCheck) {
    $name = $dir[0]
    $path = $dir[1]
    
    $status = if (Test-Path $path) { 
        $fileCount = (Get-ChildItem $path -Filter "*.rs" -Recurse).Count
        if ($fileCount -gt 0) {
            "✅ Complete ($fileCount files)"
        } else {
            "⚠️ Directory exists but no .rs files"
        }
    } else { 
        "❌ Missing" 
    }
    
    Add-Content -Path $outputFile -Value "| $name | $status | ``$($path)`` |"
}

Write-Host "Component status check created at: $outputFile"