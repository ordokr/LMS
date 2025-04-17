@echo off
setlocal enabledelayedexpansion

echo ===== Unified Analyzer Deployment Script =====

REM Set paths
set SOURCE_DIR=C:\Users\Tim\Desktop\LMS\tools\unified-analyzer
set DEPLOY_DIR=C:\Users\Tim\Desktop\LMS\deployed\unified-analyzer
set BACKUP_DIR=C:\Users\Tim\Desktop\LMS\backups\unified-analyzer\%date:~-4,4%%date:~-7,2%%date:~-10,2%_%time:~0,2%%time:~3,2%%time:~6,2%
set BACKUP_DIR=%BACKUP_DIR: =0%

echo Source directory: %SOURCE_DIR%
echo Deploy directory: %DEPLOY_DIR%
echo Backup directory: %BACKUP_DIR%

REM Create backup directory
echo Creating backup directory...
mkdir "%BACKUP_DIR%" 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to create backup directory.
    exit /b 1
)

REM Backup existing deployment
if exist "%DEPLOY_DIR%" (
    echo Backing up existing deployment...
    xcopy /E /I /Y "%DEPLOY_DIR%" "%BACKUP_DIR%" >nul
    if %ERRORLEVEL% NEQ 0 (
        echo Failed to backup existing deployment.
        exit /b 1
    )
)

REM Create deploy directory if it doesn't exist
if not exist "%DEPLOY_DIR%" (
    echo Creating deploy directory...
    mkdir "%DEPLOY_DIR%" 2>nul
    if %ERRORLEVEL% NEQ 0 (
        echo Failed to create deploy directory.
        exit /b 1
    )
)

REM Build the project
echo Building the project...
cd "%SOURCE_DIR%"
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo Failed to build the project.
    exit /b 1
)

REM Deploy the executable
echo Deploying the executable...
copy /Y "%SOURCE_DIR%\target\release\unified-analyzer.exe" "%DEPLOY_DIR%\" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to deploy the executable.
    exit /b 1
)

REM Deploy the batch script
echo Deploying the batch script...
copy /Y "%SOURCE_DIR%\unified-analyze.bat" "%DEPLOY_DIR%\" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to deploy the batch script.
    exit /b 1
)

REM Deploy the configuration file
echo Deploying the configuration file...
copy /Y "%SOURCE_DIR%\config.toml" "%DEPLOY_DIR%\" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to deploy the configuration file.
    exit /b 1
)

REM Create docs directory
echo Creating docs directory...
mkdir "%DEPLOY_DIR%\docs" 2>nul

REM Deploy the documentation
echo Deploying the documentation...
xcopy /E /I /Y "%SOURCE_DIR%\docs" "%DEPLOY_DIR%\docs" >nul
if %ERRORLEVEL% NEQ 0 (
    echo Failed to deploy the documentation.
    exit /b 1
)

echo Deployment completed successfully!
echo The Unified Analyzer is now available at: %DEPLOY_DIR%

REM Create a shortcut on the desktop
echo Creating a shortcut on the desktop...
powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('C:\Users\Tim\Desktop\Unified Analyzer.lnk'); $Shortcut.TargetPath = '%DEPLOY_DIR%\unified-analyze.bat'; $Shortcut.WorkingDirectory = '%DEPLOY_DIR%'; $Shortcut.Save()"

echo Shortcut created on the desktop.
echo ===== Deployment Complete =====

pause
