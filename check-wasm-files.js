const fs = require('fs');
const path = require('path');

// Just verify WebAssembly files exist
const wasmBinaryPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone_bg.wasm');
const jsBindingsPath = path.join(__dirname, 'wasm', 'fs-utils', 'fs_utils_wasm_standalone.js');

console.log('Checking WebAssembly files:');
console.log(`WASM binary path: ${wasmBinaryPath}`);
console.log(`WASM binary exists: ${fs.existsSync(wasmBinaryPath)}`);
console.log(`JS bindings path: ${jsBindingsPath}`);
console.log(`JS bindings exist: ${fs.existsSync(jsBindingsPath)}`);

// Write results to a file as a backup
try {
  fs.writeFileSync('wasm-files-check.txt', 
    `WASM binary exists: ${fs.existsSync(wasmBinaryPath)}\n` + 
    `JS bindings exist: ${fs.existsSync(jsBindingsPath)}\n`,
    'utf8');
} catch (err) {
  console.error('Error writing to file:', err);
}
