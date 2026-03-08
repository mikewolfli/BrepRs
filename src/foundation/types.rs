use std::ffi::c_char;

pub type Standard_Integer = i32;
pub type Standard_Real = f64;
pub type Standard_Boolean = bool;
pub type Standard_Character = char;
pub type Standard_ExtCharacter = u32;
pub type Standard_CString = *const c_char;
pub type Standard_Size = usize;

pub const STANDARD_TRUE: Standard_Boolean = true;
pub const STANDARD_FALSE: Standard_Boolean = false;

pub const STANDARD_INTEGER_MAX: Standard_Integer = i32::MAX;
pub const STANDARD_INTEGER_MIN: Standard_Integer = i32::MIN;

pub const STANDARD_REAL_MAX: Standard_Real = f64::MAX;
pub const STANDARD_REAL_MIN: Standard_Real = f64::MIN;
pub const STANDARD_REAL_EPSILON: Standard_Real = f64::EPSILON;

#[inline]
pub fn standard_is_nan(value: Standard_Real) -> Standard_Boolean {
    value.is_nan()
}

#[inline]
pub fn standard_is_infinite(value: Standard_Real) -> Standard_Boolean {
    value.is_infinite()
}

#[inline]
pub fn standard_is_finite(value: Standard_Real) -> Standard_Boolean {
    value.is_finite()
}

#[inline]
pub fn standard_approximate(value1: Standard_Real, value2: Standard_Real, tolerance: Standard_Real) -> Standard_Boolean {
    (value1 - value2).abs() <= tolerance
}

#[inline]
pub fn standard_real_hash(value: Standard_Real) -> usize {
    value.to_bits() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_types() {
        let int_val: Standard_Integer = 42;
        assert_eq!(int_val, 42);

        let real_val: Standard_Real = 3.14159;
        assert!((real_val - 3.14159).abs() < 1e-10);

        let bool_val: Standard_Boolean = true;
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
