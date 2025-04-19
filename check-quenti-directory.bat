@echo off
echo Checking Quenti directory structure...

if exist "C:\Users\Tim\Desktop\quenti" (
    echo Directory exists: C:\Users\Tim\Desktop\quenti
    
    echo Listing contents:
    dir "C:\Users\Tim\Desktop\quenti"
    
    echo Searching for Cargo.toml files:
    dir /s /b "C:\Users\Tim\Desktop\quenti\Cargo.toml"
) else (
    echo Directory does not exist: C:\Users\Tim\Desktop\quenti
)
