# Setup Git Hooks Script
# This script sets up Git hooks to run the analyzers

# Define the pre-commit hook script
$preCommitHook = @"
#!/bin/sh
#
# Pre-commit hook to run quick analysis
#
# To skip this hook, use the --no-verify option when committing

echo "Running quick analysis before commit..."
./unified-analyze.bat --quick

# Check if the analysis was successful
if [ \$? -ne 0 ]; then
    echo "Analysis failed. Please fix the issues before committing."
    exit 1
fi

echo "Analysis completed successfully."
exit 0
"@

# Define the post-merge hook script
$postMergeHook = @"
#!/bin/sh
#
# Post-merge hook to run full analysis
#

echo "Running full analysis after merge..."
./unified-analyze.bat --full --tech-debt --code-quality --models

# Check if the analysis was successful
if [ \$? -ne 0 ]; then
    echo "Analysis failed. Please check the reports for issues."
    exit 1
fi

echo "Analysis completed successfully."
echo "Check the docs directory for generated reports."
exit 0
"@

# Create the hooks directory if it doesn't exist
if (-not (Test-Path ".git\hooks")) {
    New-Item -ItemType Directory -Path ".git\hooks" -Force | Out-Null
}

# Save the pre-commit hook
$preCommitHook | Out-File -FilePath ".git\hooks\pre-commit" -Encoding ASCII -NoNewline
# Make the pre-commit hook executable
if (Test-Path ".git\hooks\pre-commit") {
    # On Windows, we need to use Git's core.fileMode setting
    git config core.fileMode true
    git update-index --chmod=+x .git\hooks\pre-commit
    Write-Host "Pre-commit hook created successfully."
} else {
    Write-Host "Failed to create pre-commit hook."
}

# Save the post-merge hook
$postMergeHook | Out-File -FilePath ".git\hooks\post-merge" -Encoding ASCII -NoNewline
# Make the post-merge hook executable
if (Test-Path ".git\hooks\post-merge") {
    # On Windows, we need to use Git's core.fileMode setting
    git config core.fileMode true
    git update-index --chmod=+x .git\hooks\post-merge
    Write-Host "Post-merge hook created successfully."
} else {
    Write-Host "Failed to create post-merge hook."
}

Write-Host "Git hooks setup completed."
Write-Host "The analyzers will run automatically on commit and after merge."
