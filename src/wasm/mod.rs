//! WebAssembly Bindings Module
//!
//! This module provides WebAssembly bindings for BrepRs library using wasm-bindgen.
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

#[cfg(feature = "serde-wasm-bindgen")]
use serde_wasm_bindgen;

pub mod geometry;
pub mod topology;
pub mod primitives;
pub mod modeling;
pub mod io;
pub mod i18n;
pub mod mesh;

use crate::foundation::tolerance;

/// Global state for WASM module
static mut GLOBAL_TOLERANCE: f64 = 1e-6;

/// Initialize the WebAssembly module
#[wasm_bindgen(js_name = init)]
pub fn wasm_init() {
    // Set panic hook for better error messages in browser console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    // Initialize logging
    #[cfg(feature = "console_log")]
    console_log::init_with_level(log::Level::Info).ok();
    
    // Set default tolerance
    unsafe {
        GLOBAL_TOLERANCE = 1e-6;
    }
}

/// Get library version
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Get library name
#[wasm_bindgen(js_name = getLibraryName)]
pub fn get_library_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

/// Set global tolerance
#[wasm_bindgen(js_name = setTolerance)]
pub fn set_tolerance(tol: f64) {
    if tol > 0.0 {
        unsafe {
            GLOBAL_TOLERANCE = tol;
        }
        tolerance::set_global_tolerance(tol);
    }
}

/// Get global tolerance
#[wasm_bindgen(js_name = getTolerance)]
pub fn get_tolerance() -> f64 {
    unsafe { GLOBAL_TOLERANCE }
}

/// Initialize logging for WebAssembly
#[wasm_bindgen(js_name = initLogging)]
pub fn init_logging(_level: Option<String>) {
    #[cfg(feature = "console_log")]
    {
        use log::Level;
        let log_level = match _level.as_deref() {
            Some("trace") => Level::Trace,
            Some("debug") => Level::Debug,
            Some("info") => Level::Info,
            Some("warn") => Level::Warn,
            Some("error") => Level::Error,
            _ => Level::Info,
        };
        console_log::init_with_level(log_level).ok();
    }
}

/// Test function to verify WASM is working
#[wasm_bindgen(js_name = testWasm)]
pub fn test_wasm() -> String {
    "WebAssembly module loaded successfully!".to_string()
}

/// Get memory usage statistics
#[wasm_bindgen(js_name = getMemoryStats)]
pub fn get_memory_stats() -> JsValue {
    let mut stats = std::collections::HashMap::new();
    stats.insert("heap_size", 0usize);
    #[cfg(feature = "serde-wasm-bindgen")]
    {
        serde_wasm_bindgen::to_value(&stats).unwrap_or(JsValue::NULL)
    }
    #[cfg(not(feature = "serde-wasm-bindgen"))]
    {
        JsValue::NULL
    }
}

/// Check if a feature is enabled
#[wasm_bindgen(js_name = hasFeature)]
pub fn has_feature(feature: String) -> bool {
    match feature.as_str() {
        "console_error_panic_hook" => cfg!(feature = "console_error_panic_hook"),
        "console_log" => cfg!(feature = "console_log"),
        _ => false,
    }
}

/// Get supported features list
#[wasm_bindgen(js_name = getSupportedFeatures)]
pub fn get_supported_features() -> Vec<String> {
    let features = Vec::new();
    features
}

/// Clear all caches and reset state
#[wasm_bindgen(js_name = reset)]
pub fn reset() {
    unsafe {
        GLOBAL_TOLERANCE = 1e-6;
    }
    tolerance::set_global_tolerance(1e-6);
}

/// Get platform information
#[wasm_bindgen(js_name = getPlatformInfo)]
pub fn get_platform_info() -> JsValue {
    let mut info = std::collections::HashMap::new();
    info.insert("name", "WebAssembly".to_string());
    info.insert("architecture", "wasm32".to_string());
    info.insert("rust_version", env!("CARGO_PKG_RUST_VERSION").to_string());
    info.insert("library_version", env!("CARGO_PKG_VERSION").to_string());
    #[cfg(feature = "serde-wasm-bindgen")]
    {
        serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL)
    }
    #[cfg(not(feature = "serde-wasm-bindgen"))]
    {
        JsValue::NULL
    }
}

/// Error type for WASM operations
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WasmError {
    message: String,
    code: u32,
}

#[wasm_bindgen]
impl WasmError {
    /// Create a new error
    #[wasm_bindgen(constructor)]
    pub fn new(message: String, code: u32) -> Self {
        Self { message, code }
    }

    /// Get error message
    #[wasm_bindgen(getter, js_name = message)]
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Get error code
    #[wasm_bindgen(getter, js_name = code)]
    pub fn code(&self) -> u32 {
        self.code
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Error {}: {}", self.code, self.message)
    }
}

/// Result type for WASM operations
#[wasm_bindgen]
pub struct WasmResult {
    success: bool,
    data: Option<JsValue>,
    error: Option<WasmError>,
}

#[wasm_bindgen]
impl WasmResult {
    /// Create a successful result
    #[wasm_bindgen(js_name = success)]
    pub fn success(data: JsValue) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create a failed result
    #[wasm_bindgen(js_name = failure)]
    pub fn failure(message: String, code: u32) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(WasmError::new(message, code)),
        }
    }

    /// Check if result is successful
    #[wasm_bindgen(getter, js_name = isSuccess)]
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get result data
    #[wasm_bindgen(getter, js_name = data)]
    pub fn data(&self) -> Option<JsValue> {
        self.data.clone()
    }

    /// Get error
    #[wasm_bindgen(getter, js_name = error)]
    pub fn error(&self) -> Option<WasmError> {
        self.error.clone()
    }
}

/// Utility functions for common operations
#[wasm_bindgen]
pub struct Utils;

#[wasm_bindgen]
impl Utils {
    /// Convert degrees to radians
    #[wasm_bindgen(js_name = degreesToRadians)]
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    /// Convert radians to degrees
    #[wasm_bindgen(js_name = radiansToDegrees)]
    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    /// Clamp a value between min and max
    #[wasm_bindgen(js_name = clamp)]
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        value.max(min).min(max)
    }

    /// Linear interpolation between two values
    #[wasm_bindgen(js_name = lerp)]
    pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    /// Check if two floats are approximately equal
    #[wasm_bindgen(js_name = approximatelyEqual)]
    pub fn approximately_equal(a: f64, b: f64, tolerance: Option<f64>) -> bool {
        let tol = tolerance.unwrap_or_else(|| get_tolerance());
        (a - b).abs() < tol
    }
}
