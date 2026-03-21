//! WebAssembly Bindings Module
//!
//! This module provides WebAssembly bindings for the BrepRs library using wasm-bindgen.
//! It enables web applications to use the CAD kernel functionality in browsers.
//!
//! # Example JavaScript Usage
//!
//! ```javascript
//! import init, { Box, Sphere, BooleanOperations } from './breprs.js';
//!
//! async function main() {
//!     await init();
//!     
//!     // Create a box
//!     const box = new Box(10.0, 10.0, 10.0);
//!     console.log(`Box volume: ${box.volume()}`);
//!     
//!     // Create a sphere
//!     const sphere = new Sphere(5.0);
//!     console.log(`Sphere volume: ${sphere.volume()}`);
//! }
//! ```

use wasm_bindgen::prelude::*;

pub mod geometry;
pub mod topology;
pub mod primitives;
pub mod modeling;


/// Initialize the WebAssembly module
#[wasm_bindgen(start)]
pub fn start() {
    // Set panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
}

/// Get library version
#[wasm_bindgen(js_name = version)]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Set global tolerance
#[wasm_bindgen(js_name = setTolerance)]
pub fn set_tolerance(tol: f64) {
    // Set global tolerance using the foundation module
    crate::foundation::tolerance::set_global_tolerance(tol);
}

/// Get global tolerance
#[wasm_bindgen(js_name = getTolerance)]
pub fn get_tolerance() -> f64 {
    1e-6
}

/// Initialize logging for WebAssembly
#[wasm_bindgen]
pub fn init_logging() {
    console_log::init_with_level(log::Level::Debug).ok();
}

/// Test function to verify WASM is working
#[wasm_bindgen(js_name = testWasm)]
pub fn test_wasm() -> String {
    "WebAssembly module loaded successfully!".to_string()
}
