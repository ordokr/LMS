@echo off
echo Building Quenti Standalone...

cd C:\Users\Tim\Desktop\quenti
cargo build --release --bin quiz-standalone

if %ERRORLEVEL% EQU 0 (
    echo Build successful!
    echo The standalone executable is located at:
    echo C:\Users\Tim\Desktop\quenti\target\release\quiz-standalone.exe
    
    echo Creating shortcut on desktop...
    powershell "$WshShell = New-Object -comObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut('%USERPROFILE%\Desktop\Quenti Quiz.lnk'); $Shortcut.TargetPath = 'C:\Users\Tim\Desktop\quenti\target\release\quiz-standalone.exe'; $Shortcut.Save()"
    
    echo Done! You can now launch Quenti by double-clicking the "Quenti Quiz" shortcut on your desktop.
) else (
    echo Build failed. Please check the error messages above.
    pause
)
