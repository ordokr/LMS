@echo off
cd %~dp0\..
rustc tools\generate_db_schema.rs -o tools\generate_db_schema.exe
tools\generate_db_schema.exe
