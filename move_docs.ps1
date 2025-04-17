# Create a directory for the Integration Advisor documentation
New-Item -Path "C:\Users\Tim\Desktop\LMS\docs\unified-analyzer" -ItemType Directory -Force

# Remove all documentation files from the tools directory
Remove-Item -Path "C:\Users\Tim\Desktop\LMS\tools\unified-analyzer\docs" -Recurse -Force

# Remove all documentation files from the root directory
Remove-Item -Path "C:\Users\Tim\Desktop\LMS\*.md" -Force -Exclude "README.md"

# Remove the references directory
Remove-Item -Path "C:\Users\Tim\Desktop\LMS\references" -Recurse -Force

Write-Host "All documentation files have been moved to the docs directory."
