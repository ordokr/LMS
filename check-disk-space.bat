@echo off
echo Checking disk space...

REM Check disk space on the system drive
echo System drive:
wmic logicaldisk where "DeviceID='C:'" get FreeSpace,Size /format:list

REM Check if temp directory exists and has write permissions
echo.
echo Checking temp directory permissions...
set TEMP_TEST_FILE=%TEMP%\test_write_permission.txt
echo Test > "%TEMP_TEST_FILE%" 2>NUL
if exist "%TEMP_TEST_FILE%" (
    echo Temp directory is writable.
    del "%TEMP_TEST_FILE%"
) else (
    echo WARNING: Cannot write to temp directory. This may cause build issues.
)

REM Check if target directory has write permissions
echo.
echo Checking target directory permissions...
if exist "src-tauri" (
    set TARGET_TEST_FILE=src-tauri\target\test_write_permission.txt
    mkdir "src-tauri\target" 2>NUL
    echo Test > "%TARGET_TEST_FILE%" 2>NUL
    if exist "%TARGET_TEST_FILE%" (
        echo Target directory is writable.
        del "%TARGET_TEST_FILE%"
    ) else (
        echo WARNING: Cannot write to target directory. This may cause build issues.
    )
)

echo.
echo Checking for antivirus interference...
echo This is a common cause of file locking issues during builds.
echo Consider temporarily disabling real-time scanning for the project directory.

pause
