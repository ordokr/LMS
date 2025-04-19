@echo off
echo Launching Ordo Quiz Standalone...

REM Check if the binary exists in the target directory
if exist "target\release\quiz-standalone.exe" (
    echo Found release binary, launching...
    start "" "target\release\quiz-standalone.exe"
    exit /b 0
)

if exist "target\debug\quiz-standalone.exe" (
    echo Found debug binary, launching...
    start "" "target\debug\quiz-standalone.exe"
    exit /b 0
)

if exist "src-tauri\target\debug\quiz-standalone.exe" (
    echo Found debug binary in src-tauri, launching...
    start "" "src-tauri\target\debug\quiz-standalone.exe"
    exit /b 0
)

REM If binary not found, try to build the minimal version
echo Binary not found, attempting to build minimal version...

REM Run the minimal build script
call build-minimal-ordo-quiz.bat

REM Check if the build was successful
if exist "src-tauri\target\debug\quiz-standalone.exe" (
    echo Build successful, launching...
    start "" "src-tauri\target\debug\quiz-standalone.exe"
    exit /b 0
)

echo Could not find or build the Ordo Quiz module.
echo Please check the error messages above.
pause
