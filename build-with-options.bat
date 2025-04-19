@echo off
echo Trying different build options for Ordo Quiz...

REM Try building with minimal features
if exist "src-tauri\Cargo.toml" (
    echo Building with minimal features from src-tauri directory...
    cd src-tauri
    
    echo Attempt 1: Building with --no-default-features
    cargo build --bin quiz-standalone --no-default-features
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful with --no-default-features!
        cd ..
        exit /b 0
    )
    
    echo Attempt 2: Building with -Z unstable-options
    cargo +nightly build --bin quiz-standalone -Z unstable-options
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful with nightly toolchain!
        cd ..
        exit /b 0
    )
    
    echo Attempt 3: Building with increased stack size
    set RUSTFLAGS=-C link-args=-Wl,--stack,4194304
    cargo build --bin quiz-standalone
    set RUSTFLAGS=
    
    if %ERRORLEVEL% EQU 0 (
        echo Build successful with increased stack size!
        cd ..
        exit /b 0
    )
    
    cd ..
)

echo All build attempts failed.
echo Please check the error messages above for more details.
pause
