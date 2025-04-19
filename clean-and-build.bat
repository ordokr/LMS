@echo off
echo Cleaning build directory and rebuilding Ordo Quiz...

REM Kill any running processes that might be locking files
taskkill /F /IM quiz-standalone.exe 2>NUL
taskkill /F /IM cargo.exe 2>NUL
taskkill /F /IM rustc.exe 2>NUL

REM Wait a moment for processes to terminate
timeout /t 2 /nobreak >NUL

REM Clean the build directory
if exist "src-tauri\target" (
    echo Cleaning src-tauri\target directory...
    rd /s /q "src-tauri\target"
)

if exist "target" (
    echo Cleaning root target directory...
    rd /s /q "target"
)

REM Wait a moment for file system operations to complete
timeout /t 2 /nobreak >NUL

REM Try to build from src-tauri directory
if exist "src-tauri\Cargo.toml" (
    echo Building from src-tauri directory...
    cd src-tauri
    cargo clean
    cargo build --bin quiz-standalone
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful, launching...
        start "" "..\target\debug\quiz-standalone.exe"
        cd ..
        exit /b 0
    ) else (
        echo Build failed in src-tauri directory. Trying root directory...
        cd ..
    )
)

REM Try to build from root directory
if exist "Cargo.toml" (
    echo Building from root directory...
    cargo clean
    cargo build --bin quiz-standalone
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful, launching...
        start "" "target\debug\quiz-standalone.exe"
        exit /b 0
    ) else (
        echo Build failed. Please check the error messages above.
    )
)

echo Could not build the Ordo Quiz module.
echo Please check your project structure and try again.
pause
