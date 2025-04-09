/**
 * Simple test script to directly import the WebAssembly module
 */

async function testWasmImport() {
  try {
    console.log('Attempting to import WebAssembly module...');
    const wasmModule = await import('./wasm/fs-utils/fs_utils_wasm_standalone.js');
    
    if (wasmModule) {
      console.log('✅ Successfully imported WebAssembly module!');
      console.log('Module exports:', Object.keys(wasmModule));
      
      if (wasmModule.FileSystemUtils) {
        console.log('✅ FileSystemUtils class found in the module');
        console.log('Creating instance...');
        
        const fsUtils = new wasmModule.FileSystemUtils('.');
        console.log('✅ Successfully created FileSystemUtils instance');
      } else {
        console.log('❌ FileSystemUtils class not found in the module');
      }
    } else {
      console.log('❌ Failed to import WebAssembly module');
    }
  } catch (error) {
    console.error('Error importing WebAssembly module:', error);
  }
}

// Run the test
testWasmImport();
