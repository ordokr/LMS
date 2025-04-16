@echo off
cd C:\Users\Tim\Desktop\LMS
echo Compiling and running test generators...
rustc -o test_generators.exe test_generators.rs
if %ERRORLEVEL% NEQ 0 (
    echo Failed to compile test_generators.rs
    exit /b 1
)
echo Running test generators...
.\test_generators.exe
echo Done!
pause
