@echo off
REM Build the Rust FileSystemUtils module for WebAssembly

echo Building Rust FileSystemUtils for WebAssembly...

REM Check if wasm-pack is installed
where wasm-pack >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo wasm-pack is not installed. Installing...
    cargo install wasm-pack
)

REM Build the Rust code targeting WebAssembly
wasm-pack build --target web --out-dir pkg

echo Build completed. WebAssembly module is available in ./pkg directory
