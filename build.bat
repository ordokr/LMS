@echo off
setlocal enabledelayedexpansion

title LMS Build System
echo LMS Build System v1.0
echo =====================
echo.

REM Check if any command was provided
if "%1"=="" (
    goto :help
)

REM Process commands
if /i "%1"=="build" (
    echo Building project...
    call :run_command cargo build %2 %3 %4 %5 %6 %7 %8 %9
    exit /b %errorlevel%
)

if /i "%1"=="test" (
    echo Running tests...
    call :run_command cargo test %2 %3 %4 %5 %6 %7 %8 %9
    exit /b %errorlevel%
)

if /i "%1"=="update-audit" (
    echo Updating audit report...
    call :run_command cargo run --bin update_audit
    exit /b %errorlevel%
)

if /i "%1"=="run" (
    echo Running application...
    call :run_command cargo run -- %2 %3 %4 %5 %6 %7 %8 %9
    exit /b %errorlevel%
)

if /i "%1"=="doc" (
    echo Generating documentation...
    call :run_command cargo doc --no-deps --open
    exit /b %errorlevel%
)

if /i "%1"=="clean" (
    echo Cleaning project...
    call :run_command cargo clean
    exit /b %errorlevel%
)

if /i "%1"=="fix-modules" (
    echo Fixing module structure...
    
    REM Check which module files exist and remove duplicates
    if exist "src\components\layout.rs" (
        if exist "src\components\layout\mod.rs" (
            echo Removing duplicate layout module file...
            del "src\components\layout.rs"
        )
    )
    
    if exist "src\components\auth.rs" (
        if exist "src\components\auth\mod.rs" (
            echo Removing duplicate auth module file...
            del "src\components\auth.rs"
        )
    )
    
    REM Handle forum_categories module ambiguity
    if exist "src\components\forum\forum_categories.rs" (
        if exist "src\components\forum\forum_categories\mod.rs" (
            echo Removing duplicate forum_categories module file...
            del "src\components\forum\forum_categories.rs"
        ) else (
            echo Creating forum_categories directory module...
            mkdir "src\components\forum\forum_categories" 2>nul
            move "src\components\forum\forum_categories.rs" "src\components\forum\forum_categories\mod.rs" > nul
        )
    ) else (
        REM Create missing forum_categories module file
        if not exist "src\components\forum\forum_categories\mod.rs" (
            echo Creating missing forum_categories module...
            mkdir "src\components\forum\forum_categories" 2>nul
            echo // Forum categories module > "src\components\forum\forum_categories\mod.rs"
        )
    )
    
    echo Module structure fixes completed.
    exit /b 0
)

if /i "%1"=="fix-regex" (
    echo Fixing regex syntax in thread_detail.rs...
    
    REM Ensure the scripts folder exists and run the PowerShell script
    if not exist "scripts\fix_regex.ps1" (
        echo ERROR: scripts\fix_regex.ps1 not found.
        exit /b 1
    )
    
    powershell -ExecutionPolicy Bypass -File scripts\fix_regex.ps1
    if %errorlevel% neq 0 (
        echo Failed to fix regex in thread_detail.rs.
        exit /b %errorlevel%
    )
    
    echo Regex fixed.
    exit /b 0
)

if /i "%1"=="fix-unused" (
    echo Fixing unused variables and imports...
    call :run_command cargo fix --allow-dirty --allow-staged
    echo Unused code fixed.
    exit /b 0
)

if /i "%1"=="fix-all" (
    echo Fixing all issues...
    
    call build.bat fix-modules
    if %errorlevel% neq 0 (
        echo Module fix failed with error %errorlevel%.
        exit /b %errorlevel%
    )
    
    call build.bat fix-regex
    if %errorlevel% neq 0 (
        echo Regex fix failed with error %errorlevel%.
        exit /b %errorlevel%
    )
    
    call build.bat fix-unused
    if %errorlevel% neq 0 (
        echo Fix unused failed with error %errorlevel%.
        exit /b %errorlevel%
    )
    
    echo All fixes applied successfully.
    exit /b 0
)

if /i "%1"=="help" (
    goto :help
) else (
    echo Unknown command: %1
    echo.
    goto :help
)

:help
echo Available commands:
echo   build [args]        - Build the project with optional cargo arguments
echo   test [args]         - Run tests with optional test arguments
echo   update-audit        - Update the audit report
echo   run [args]          - Run the application with optional arguments
echo   doc                 - Generate and open documentation
echo   clean               - Clean the project
echo   fix-modules         - Fix module structure issues
echo   fix-regex           - Fix regex syntax in thread_detail.rs
echo   fix-unused          - Fix unused imports and variables
echo   fix-all             - Apply all fixes (modules, regex, and unused code)
echo   help                - Show this help message
exit /b 0

:run_command
echo ^> %*
%*
if %errorlevel% neq 0 (
    echo Command failed with exit code %errorlevel%
    exit /b %errorlevel%
)
echo Command completed successfully.
echo.
exit /b 0
