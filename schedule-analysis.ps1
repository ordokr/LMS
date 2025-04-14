# Schedule Analysis Script
# This script sets up a scheduled task to run the analyzers

# Define the task name
$taskName = "LMS-Project-Analysis"

# Define the task description
$taskDescription = "Run LMS Project Analyzers to generate reports"

# Define the task action
$taskAction = New-ScheduledTaskAction -Execute "cmd.exe" -Argument "/c cd /d C:\Users\Tim\Desktop\LMS && call unified-analyze.bat --full --tech-debt --code-quality --models --dashboard"

# Define the task trigger (daily at 3 AM)
$taskTrigger = New-ScheduledTaskTrigger -Daily -At 3am

# Define the task settings
$taskSettings = New-ScheduledTaskSettingsSet -RunOnlyIfIdle:$false -StartWhenAvailable -DontStopIfGoingOnBatteries -AllowStartIfOnBatteries

# Create the task
$task = Register-ScheduledTask -TaskName $taskName -Description $taskDescription -Action $taskAction -Trigger $taskTrigger -Settings $taskSettings -Force

# Output the result
if ($task) {
    Write-Host "Scheduled task '$taskName' created successfully."
    Write-Host "The analyzers will run daily at 3 AM."
} else {
    Write-Host "Failed to create scheduled task."
}

# Create a manual trigger script
$manualTriggerScript = @"
@echo off
REM Manual Trigger for Scheduled Analysis
REM This script manually triggers the scheduled analysis task

echo Triggering LMS Project Analysis...
schtasks /run /tn "$taskName"

echo.
echo Analysis task triggered.
echo Check the docs directory for generated reports.
"@

# Save the manual trigger script
$manualTriggerScript | Out-File -FilePath "trigger-analysis.bat" -Encoding ASCII

Write-Host "Created 'trigger-analysis.bat' for manually triggering the analysis."
