@echo off
echo Launching Ordo Quiz Module Standalone...

rem Change to the src-tauri directory
cd src-tauri

rem Check if we're in development or production mode
if exist "Cargo.toml" (
    echo Development environment detected
    
    rem Try to build and run with Tauri
    echo Building and running with Tauri...
    where cargo >nul 2>nul
    if %ERRORLEVEL% equ 0 (
        cargo tauri dev
        if %ERRORLEVEL% equ 0 (
            echo Ordo Quiz launched successfully with Tauri
            exit /b 0
        ) else (
            echo Failed to launch with Tauri, trying direct binary...
        )
    ) else (
        echo Cargo not found, trying direct binary...
    )
    
    rem Try to run the binary directly
    if exist "target\debug\quiz-standalone.exe" (
        echo Running debug binary...
        target\debug\quiz-standalone.exe
        exit /b %ERRORLEVEL%
    ) else if exist "target\release\quiz-standalone.exe" (
        echo Running release binary...
        target\release\quiz-standalone.exe
        exit /b %ERRORLEVEL%
    ) else (
        echo Building binary...
        cargo build --bin quiz-standalone
        if %ERRORLEVEL% equ 0 (
            echo Running newly built binary...
            target\debug\quiz-standalone.exe
            exit /b %ERRORLEVEL%
        ) else (
            echo Failed to build binary
            exit /b 1
        )
    )
) else (
    echo Production environment detected
    
    rem Try to find and run the binary
    if exist "quiz-standalone.exe" (
        quiz-standalone.exe
        exit /b %ERRORLEVEL%
    ) else if exist "bin\quiz-standalone.exe" (
        bin\quiz-standalone.exe
        exit /b %ERRORLEVEL%
    ) else (
        echo Could not find Ordo Quiz binary
        exit /b 1
    )
)

echo Failed to launch Ordo Quiz Module
exit /b 1
