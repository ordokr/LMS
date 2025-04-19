@echo off
echo Building Ordo Quiz Standalone...

REM Try to build from src-tauri directory
if exist "src-tauri\Cargo.toml" (
    echo Building from src-tauri directory...
    cd src-tauri
    cargo build --release --bin quiz-standalone

    if %ERRORLEVEL% EQU 0 (
        echo Build successful!
        echo The standalone executable is located at:
        echo %CD%\..\target\release\quiz-standalone.exe

        echo Creating shortcut on desktop...
        powershell "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Ordo Quiz.lnk'); $Shortcut.TargetPath = '%CD%\..\target\release\quiz-standalone.exe'; $Shortcut.Save()"

        echo Done! You can now launch Ordo Quiz by double-clicking the "Ordo Quiz" shortcut on your desktop.
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
    cargo build --release --bin quiz-standalone

    if %ERRORLEVEL% EQU 0 (
        echo Build successful!
        echo The standalone executable is located at:
        echo %CD%\target\release\quiz-standalone.exe

        echo Creating shortcut on desktop...
        powershell "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Ordo Quiz.lnk'); $Shortcut.TargetPath = '%CD%\target\release\quiz-standalone.exe'; $Shortcut.Save()"

        echo Done! You can now launch Ordo Quiz by double-clicking the "Ordo Quiz" shortcut on your desktop.
        exit /b 0
    ) else (
        echo Build failed. Please check the error messages above.
    )
)

echo Could not find or build the Ordo Quiz module.
echo Please make sure you have the necessary Cargo.toml files in your project.
pause
