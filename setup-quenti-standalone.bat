@echo off
echo Setting up environment for Quenti Module Standalone...

echo Installing Tauri CLI...
cargo install tauri-cli

echo Installing nightly toolchain...
rustup install nightly
rustup default nightly

echo Installing required dependencies...
npm install

echo Setup complete. You can now run the application using run-quenti-standalone.bat
