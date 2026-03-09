use std::ffi::c_char;

pub type StandardInteger = i32;
pub type StandardReal = f64;
pub type StandardBoolean = bool;
pub type StandardCharacter = char;
pub type StandardExtCharacter = u32;
pub type StandardCString = *const c_char;
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
