use wasm_bindgen::prelude::*;

// Re-export modules
pub mod utils;

// Export the wasm module from file_system_utils
#[cfg(target_arch = "wasm32")]
pub use utils::file_system_utils::wasm;
