:: filepath: c:/Users/Tim/Desktop/LMS/haskell-integration/tools/profile_hybrid.bat
@echo off
setlocal enabledelayedexpansion

echo Setting up profiling environment...
cd %~dp0\..

:: Capture baseline metrics
echo Capturing baseline metrics...
cargo bench --bench sync_benchmark -- --profile-time=10 > rust_baseline.txt

:: Run integrated profiling with GHC RTS options
echo Running integrated profiling...
set GHCRTS=+RTS -p -h -l -xt -RTS
cargo bench --bench hybrid_benchmark -- --profile-time=10

:: Process heap profiles if tools are available
echo Analyzing GC behavior...
where hp2ps >nul 2>nul
if %ERRORLEVEL% equ 0 (
    hp2ps -e8in -c *.hp
    echo GC heap profiles generated as .ps files
) else (
    echo hp2ps not found.
    echo Raw heap profiles available as .hp files
)

:: Use Windows built-in performance tools as alternative
echo Running Windows ETW tracing...
cargo bench --bench hybrid_benchmark -- --profile-time=5 2>NUL 1>NUL

:: Summarize
echo Profiling complete. Results available in:
echo - *.hp.ps (heap profile, if hp2ps was available)
echo - *.hp (raw heap profile data)
echo - *.prof (time profile data)
echo - rust_baseline.txt (baseline metrics)

endlocal