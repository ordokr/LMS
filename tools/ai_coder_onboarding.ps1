# AI Coder Onboarding Script for Ordo Project
# This script provides a quick orientation for AI coding assistants

# Define colors for better readability
$headerColor = "Cyan"
$subHeaderColor = "Yellow"
$highlightColor = "Green"
$warningColor = "Red"

# Clear the screen
Clear-Host

# Display welcome message
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host "       WELCOME TO THE ORDO PROJECT - AI CODER GUIDE      " -ForegroundColor $headerColor
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host ""

# Display orientation information
Write-Host "This script will help you get oriented with the Ordo project structure," -ForegroundColor $subHeaderColor
Write-Host "architecture, and implementation details." -ForegroundColor $subHeaderColor
Write-Host ""

# Get the base directory
$baseDir = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$docsDir = Join-Path $baseDir "docs"
$toolsDir = Join-Path $baseDir "tools"

# Check if orientation guide exists
$orientationPath = Join-Path $docsDir "ai_coder_orientation.md"
if (-not (Test-Path $orientationPath)) {
    Write-Host "ERROR: Orientation guide not found at $orientationPath" -ForegroundColor $warningColor
    Write-Host "Please run the unified-analyzer to generate documentation first." -ForegroundColor $warningColor
    exit 1
}

# Display key documents and their locations
Write-Host "KEY DOCUMENTS FOR AI CODERS:" -ForegroundColor $headerColor
Write-Host ""

Write-Host "1. AI Coder Orientation Guide" -ForegroundColor $highlightColor
Write-Host "   Location: $orientationPath"
Write-Host "   Description: Start here for a comprehensive overview of the project"
Write-Host ""

$centralHubPath = Join-Path $docsDir "central_reference_hub.md"
Write-Host "2. Central Reference Hub" -ForegroundColor $highlightColor
Write-Host "   Location: $centralHubPath"
Write-Host "   Description: The main source of truth for the entire project"
Write-Host ""

$nextStepsPath = Join-Path $docsDir "integration-advisor\next_steps.md"
Write-Host "3. Next Steps Document" -ForegroundColor $highlightColor
Write-Host "   Location: $nextStepsPath"
Write-Host "   Description: Current implementation priorities and roadmap"
Write-Host ""

$recommendationsPath = Join-Path $toolsDir "unified-analyzer\docs\integration-advisor\reports\recommendations.md"
Write-Host "4. Migration Recommendations" -ForegroundColor $highlightColor
Write-Host "   Location: $recommendationsPath"
Write-Host "   Description: Strategies for migrating functionality from Canvas/Discourse to Rust"
Write-Host ""

# Display project structure
Write-Host "PROJECT STRUCTURE OVERVIEW:" -ForegroundColor $headerColor
Write-Host ""
Write-Host "Ordo/" -ForegroundColor $subHeaderColor
Write-Host "├── src-tauri/         # Rust backend code" -ForegroundColor $subHeaderColor
Write-Host "│   └── src/" -ForegroundColor $subHeaderColor
Write-Host "│       ├── api/       # API endpoints" -ForegroundColor $subHeaderColor
Write-Host "│       ├── core/      # Core business logic" -ForegroundColor $subHeaderColor
Write-Host "│       ├── db/        # Database interactions" -ForegroundColor $subHeaderColor
Write-Host "│       ├── models/    # Data models" -ForegroundColor $subHeaderColor
Write-Host "│       ├── sync/      # Synchronization engine" -ForegroundColor $subHeaderColor
Write-Host "│       └── modules/   # App-like modules" -ForegroundColor $subHeaderColor
Write-Host "├── src/               # Frontend code (Leptos)" -ForegroundColor $subHeaderColor
Write-Host "│   ├── components/    # Reusable UI components" -ForegroundColor $subHeaderColor
Write-Host "│   ├── pages/         # Application pages" -ForegroundColor $subHeaderColor
Write-Host "│   └── services/      # Frontend services" -ForegroundColor $subHeaderColor
Write-Host "├── docs/              # Documentation" -ForegroundColor $subHeaderColor
Write-Host "└── tools/             # Development tools" -ForegroundColor $subHeaderColor
Write-Host ""

# Display technology stack
Write-Host "TECHNOLOGY STACK:" -ForegroundColor $headerColor
Write-Host ""
Write-Host "- Frontend: Leptos (Rust-based reactive framework)" -ForegroundColor $subHeaderColor
Write-Host "- Desktop Shell: Tauri" -ForegroundColor $subHeaderColor
Write-Host "- Backend: Rust and Haskell" -ForegroundColor $subHeaderColor
Write-Host "- Database: Hybrid SQLite/Redb approach" -ForegroundColor $subHeaderColor
Write-Host "- ORM: SQLx for type-safe SQL" -ForegroundColor $subHeaderColor
Write-Host ""

# Display core principles
Write-Host "CORE PRINCIPLES:" -ForegroundColor $headerColor
Write-Host ""
Write-Host "1. Offline-First: All core functionality works without an internet connection" -ForegroundColor $subHeaderColor
Write-Host "2. Integrated Experience: Seamless integration between LMS and forum components" -ForegroundColor $subHeaderColor
Write-Host "3. Performance: Fast, responsive experience even on lower-end hardware" -ForegroundColor $subHeaderColor
Write-Host "4. Security: Strong data protection and privacy controls" -ForegroundColor $subHeaderColor
Write-Host "5. Extensibility: Modular architecture that allows for customization" -ForegroundColor $subHeaderColor
Write-Host ""

# Offer to open key documents
Write-Host "Would you like to open any of these documents?" -ForegroundColor $headerColor
Write-Host "1. AI Coder Orientation Guide" -ForegroundColor $highlightColor
Write-Host "2. Central Reference Hub" -ForegroundColor $highlightColor
Write-Host "3. Next Steps Document" -ForegroundColor $highlightColor
Write-Host "4. Migration Recommendations" -ForegroundColor $highlightColor
Write-Host "5. All of the above" -ForegroundColor $highlightColor
Write-Host "0. Exit" -ForegroundColor $highlightColor
Write-Host ""

$choice = Read-Host "Enter your choice (0-5)"

switch ($choice) {
    "1" { Invoke-Item $orientationPath }
    "2" { Invoke-Item $centralHubPath }
    "3" { Invoke-Item $nextStepsPath }
    "4" { Invoke-Item $recommendationsPath }
    "5" {
        Invoke-Item $orientationPath
        Invoke-Item $centralHubPath
        Invoke-Item $nextStepsPath
        Invoke-Item $recommendationsPath
    }
    "0" { Write-Host "Exiting..." -ForegroundColor $subHeaderColor }
    default { Write-Host "Invalid choice. Exiting..." -ForegroundColor $warningColor }
}

Write-Host ""
Write-Host "=========================================================" -ForegroundColor $headerColor
Write-Host "       HAPPY CODING WITH THE ORDO PROJECT!               " -ForegroundColor $headerColor
Write-Host "=========================================================" -ForegroundColor $headerColor
