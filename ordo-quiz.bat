@echo off
setlocal enabledelayedexpansion

echo Ordo Quiz Module Utility Script
echo ===============================
echo.

if "%1"=="" (
    goto :menu
) else (
    goto :%1
)

:menu
echo Please select an option:
echo 1. Build Ordo Quiz (Release)
echo 2. Build Ordo Quiz (Debug)
echo 3. Launch Ordo Quiz
echo 4. Clean temporary files
echo 5. Exit
echo.

set /p choice="Enter your choice (1-5): "

if "%choice%"=="1" goto :build_release
if "%choice%"=="2" goto :build_debug
if "%choice%"=="3" goto :launch
if "%choice%"=="4" goto :clean
if "%choice%"=="5" goto :exit

echo Invalid choice. Please try again.
goto :menu

:build_release
echo Building Ordo Quiz (Release)...
echo.

cd src-tauri
cargo build --bin quiz-standalone --release
if %ERRORLEVEL% neq 0 (
    echo.
    echo Build failed. Please check the error messages above.
    pause
    goto :menu
)
cd ..

echo.
echo Build completed successfully.
echo The binary is available at: src-tauri\target\release\quiz-standalone.exe
echo.
pause
goto :menu

:build_debug
echo Building Ordo Quiz (Debug)...
echo.

cd src-tauri
cargo build --bin quiz-standalone
if %ERRORLEVEL% neq 0 (
    echo.
    echo Build failed. Please check the error messages above.
    pause
    goto :menu
)
cd ..

echo.
echo Build completed successfully.
echo The binary is available at: src-tauri\target\debug\quiz-standalone.exe
echo.
pause
goto :menu

:launch
echo Launching Ordo Quiz...
echo.

set "binary_found=false"

if exist "src-tauri\target\release\quiz-standalone.exe" (
    echo Found release binary, launching...
    start "" "src-tauri\target\release\quiz-standalone.exe"
    set "binary_found=true"
    goto :launch_done
)

if exist "src-tauri\target\debug\quiz-standalone.exe" (
    echo Found debug binary, launching...
    start "" "src-tauri\target\debug\quiz-standalone.exe"
    set "binary_found=true"
    goto :launch_done
)

if exist "target\release\quiz-standalone.exe" (
    echo Found release binary in root target, launching...
    start "" "target\release\quiz-standalone.exe"
    set "binary_found=true"
    goto :launch_done
)

if exist "target\debug\quiz-standalone.exe" (
    echo Found debug binary in root target, launching...
    start "" "target\debug\quiz-standalone.exe"
    set "binary_found=true"
    goto :launch_done
)

:launch_done
if "%binary_found%"=="false" (
    echo Binary not found. Would you like to build it now?
    set /p build_choice="Enter 'y' to build or any other key to return to menu: "
    
    if /i "%build_choice%"=="y" (
        goto :build_debug
    ) else (
        goto :menu
    )
)

echo.
echo Ordo Quiz has been launched.
echo.
pause
goto :menu

:clean
echo Cleaning temporary files...
echo.

if exist "ordo_quiz.db" (
    del /f /q "ordo_quiz.db"
    echo Removed ordo_quiz.db
)

if exist "ordo_quiz_test.db" (
    del /f /q "ordo_quiz_test.db"
    echo Removed ordo_quiz_test.db
)

if exist "quenti.db" (
    del /f /q "quenti.db"
    echo Removed quenti.db
)

if exist "ordo_quiz_data" (
    rmdir /s /q "ordo_quiz_data"
    echo Removed ordo_quiz_data directory
)

if exist "quenti_data" (
    rmdir /s /q "quenti_data"
    echo Removed quenti_data directory
)

if exist "test_data" (
    rmdir /s /q "test_data"
    echo Removed test_data directory
)

echo.
echo Cleanup completed.
echo.
pause
goto :menu

:exit
echo Exiting...
exit /b 0
