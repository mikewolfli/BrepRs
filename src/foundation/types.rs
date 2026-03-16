use std::ffi::c_char;

/// Standard integer type for CAD operations
///
/// This type alias provides a 32-bit signed integer type.
/// Used for indices, counts, and other discrete values in the CAD kernel.
///
/// # FFI Compatibility
/// - This type is guaranteed to be 32-bit signed integer
/// - Compatible with C's `int32_t` and C++'s `std::int32_t`
/// - Safe for cross-platform FFI operations
pub type StandardInteger = i32;

/// Standard real number type for CAD operations
///
/// This type alias provides a 64-bit floating-point number (double precision).
/// Used for coordinates, distances, angles, and other continuous values.
///
/// # FFI Compatibility
/// - This type is guaranteed to be 64-bit IEEE 754 double precision
/// - Compatible with C's `double` and C++'s `double`
/// - Safe for cross-platform FFI operations
/// - Note: floating-point representation may vary slightly across platforms
pub type StandardReal = f64;

/// Standard boolean type for CAD operations
///
/// This type alias provides a boolean type.
/// Used for flags, conditions, and logical operations.
///
/// # FFI Compatibility
/// - Rust's `bool` is guaranteed to be 1 byte (8 bits)
/// - Compatible with C99's `_Bool` and C++'s `bool`
/// - Use `StandardInteger` (0/1) for C89 compatibility
pub type StandardBoolean = bool;

/// Standard character type for CAD operations
///
/// This type alias provides a character type.
/// Used for single character values and string operations.
pub type StandardCharacter = char;

/// Standard extended character type for CAD operations
///
/// This type alias provides a 32-bit Unicode character. Used for internationalization
/// and extended character set support.
pub type StandardExtCharacter = u32;

/// Standard C string type for FFI operations
///
/// This type alias provides a C string type.
/// Used for FFI (Foreign Function Interface) with C libraries and
/// interoperability with external C APIs.
///
/// # Safety
/// - This is a raw pointer type and must be used with caution
/// - The pointer must be valid and null-terminated
/// - Ownership and lifetime must be carefully managed
/// - Prefer using Rust's String or &str when possible
///
/// # FFI Compatibility
/// - Uses `c_char` which maps to C's `char` type
/// - Note: `char` size varies by platform (1 byte on most systems, but may be different on some embedded platforms)
/// - For maximum portability, use byte arrays (`*const u8`) instead
/// - Always check for null before dereferencing
pub type StandardCString = *const c_char;

/// Standard size type for CAD operations
///
/// This type alias provides a size type.
/// Used for sizes of collections, memory allocations, and indexing.
///
/// # FFI Compatibility
/// - Rust's `usize` is platform-dependent (32-bit on 32-bit platforms, 64-bit on 64-bit platforms)
/// - For FFI, prefer using fixed-size types like `StandardInteger` or `StandardReal`
/// - When passing to C, use `size_t` which matches platform's pointer size
/// - For cross-platform compatibility, consider using `u64` for sizes in FFI
pub type StandardSize = usize;

pub const STANDARD_TRUE: StandardBoolean = true;
pub const STANDARD_FALSE: StandardBoolean = false;

pub const STANDARD_INTEGER_MAX: StandardInteger = i32::MAX;
pub const STANDARD_INTEGER_MIN: StandardInteger = i32::MIN;

pub const STANDARD_REAL_MAX: StandardReal = f64::MAX;
pub const STANDARD_REAL_MIN: StandardReal = f64::MIN;
pub const STANDARD_REAL_EPSILON: StandardReal = f64::EPSILON;

#[inline]
pub fn standard_is_nan(value: StandardReal) -> StandardBoolean {
    value.is_nan()
}

#[inline]
pub fn standard_is_infinite(value: StandardReal) -> StandardBoolean {
    value.is_infinite()
}

#[inline]
pub fn standard_is_finite(value: StandardReal) -> StandardBoolean {
    value.is_finite()
}

#[inline]
pub fn standard_approximate(
    value1: StandardReal,
    value2: StandardReal,
    tolerance: StandardReal,
) -> StandardBoolean {
    (value1 - value2).abs() <= tolerance
}

#[inline]
pub fn standard_real_hash(value: StandardReal) -> usize {
    value.to_bits() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_types() {
        let int_val: StandardInteger = 42;
        assert_eq!(int_val, 42);

        let real_val: StandardReal = 3.14159;
        assert!((real_val - 3.14159).abs() < 1e-10);

        let bool_val: StandardBoolean = true;
        assert_eq!(bool_val, STANDARD_TRUE);
    }

    #[test]
    fn test_standard_constants() {
        assert_eq!(STANDARD_TRUE, true);
        assert_eq!(STANDARD_FALSE, false);
    }

    #[test]
    fn test_standard_is_nan() {
        assert!(standard_is_nan(f64::NAN));
        assert!(!standard_is_nan(1.0));
    }

    #[test]
    fn test_standard_is_infinite() {
        assert!(standard_is_infinite(f64::INFINITY));
        assert!(!standard_is_infinite(1.0));
    }

    #[test]
    fn test_standard_is_finite() {
        assert!(standard_is_finite(1.0));
        assert!(!standard_is_finite(f64::INFINITY));
    }

    #[test]
    fn test_standard_approximate() {
        assert!(standard_approximate(1.0, 1.0001, 0.001));
        assert!(!standard_approximate(1.0, 2.0, 0.001));
    }
}
