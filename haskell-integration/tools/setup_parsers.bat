@echo off
echo Setting up Alex and Happy lexer/parser generators...

:: Check if cabal is available
where cabal >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Cabal not found in PATH.
    echo Please install GHCup from https://www.haskell.org/ghcup/
    exit /b 1
)

:: Install Alex and Happy
echo Installing Alex (lexer generator)...
cabal install alex --overwrite-policy=always

echo Installing Happy (parser generator)...
cabal install happy --overwrite-policy=always

echo Adding tools to PATH if needed...
setx PATH "%APPDATA%\cabal\bin;%PATH%"

echo Setup complete!