//! Tolerance Module
//! 
//! This module provides functionality for managing global tolerance values used throughout the library.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

const DEFAULT_TOLERANCE: f64 = 1e-6;
const DEFAULT_ANGULAR_TOLERANCE: f64 = 1e-4;

static GLOBAL_TOLERANCE: AtomicU64 = AtomicU64::new(DEFAULT_TOLERANCE.to_bits());
static GLOBAL_ANGULAR_TOLERANCE: AtomicU64 = AtomicU64::new(DEFAULT_ANGULAR_TOLERANCE.to_bits());
static TOLERANCE_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize global tolerance values
pub fn initialize_tolerance() {
    if !TOLERANCE_INITIALIZED.load(Ordering::SeqCst) {
        GLOBAL_TOLERANCE.store(DEFAULT_TOLERANCE.to_bits(), Ordering::SeqCst);
        GLOBAL_ANGULAR_TOLERANCE.store(DEFAULT_ANGULAR_TOLERANCE.to_bits(), Ordering::SeqCst);
        TOLERANCE_INITIALIZED.store(true, Ordering::SeqCst);
    }
}

/// Set global linear tolerance
pub fn set_global_tolerance(tolerance: f64) {
    if tolerance > 0.0 {
        GLOBAL_TOLERANCE.store(tolerance.to_bits(), Ordering::SeqCst);
    }
}

/// Get global linear tolerance
pub fn global_tolerance() -> f64 {
    f64::from_bits(GLOBAL_TOLERANCE.load(Ordering::SeqCst))
}

/// Set global angular tolerance
pub fn set_global_angular_tolerance(tolerance: f64) {
    if tolerance > 0.0 {
        GLOBAL_ANGULAR_TOLERANCE.store(tolerance.to_bits(), Ordering::SeqCst);
    }
}

/// Get global angular tolerance
pub fn global_angular_tolerance() -> f64 {
    f64::from_bits(GLOBAL_ANGULAR_TOLERANCE.load(Ordering::SeqCst))
}

/// Check if two values are equal within tolerance
pub fn is_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= global_tolerance()
}

/// Check if a value is zero within tolerance
pub fn is_zero(value: f64) -> bool {
    value.abs() <= global_tolerance()
}

/// Check if a value is positive within tolerance
pub fn is_positive(value: f64) -> bool {
    value > global_tolerance()
}

/// Check if a value is negative within tolerance
pub fn is_negative(value: f64) -> bool {
    value < -global_tolerance()
}

/// Clamp a value to zero if it's within tolerance
pub fn clamp_to_zero(value: f64) -> f64 {
    if is_zero(value) {
        0.0
    } else {
        value
    }
}

/// Round a value to the nearest multiple of tolerance
pub fn round_to_tolerance(value: f64) -> f64 {
    let tol = global_tolerance();
    (value / tol).round() * tol
}
