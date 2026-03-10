use thiserror::Error;

/// Result type alias for operations that can fail
///
/// This type alias provides a convenient way to use the Failure enum
/// as the error type for Result types throughout the codebase.
pub type Result<T> = std::result::Result<T, Failure>;
// Ensure Failure is Send + Sync for thread safety
unsafe impl Send for Failure {}
unsafe impl Sync for Failure {}

/// Comprehensive error type for CAD kernel operations
///
/// This enum covers a wide range of error types that can occur in
/// boundary representation operations.
///
/// # Error Handling Policy
/// - **Kernel operations**: Many helper functions panic for simplicity and performance
/// - **Public APIs**: Use Result<T> for recoverable errors
/// - **FFI boundaries**: Convert to appropriate C error codes
///
/// # Panic vs Error Return
/// - Use `panic!` for programming errors and invariants violations
/// - Use `Result<T>` for expected failures (e.g., invalid input, I/O errors)
/// - Helper macros like `raise_if!` are designed for kernel internal use
#[derive(Error, Debug)]
pub enum Failure {
    #[error("Domain error: {msg}")]
    DomainError { msg: String },
    #[error("Range error: {msg}")]
    RangeError { msg: String },
    #[error("Numeric error: {msg}")]
    NumericError { msg: String },
    #[error("Overflow error: {msg}")]
    OverflowError { msg: String },
    #[error("Underflow error: {msg}")]
    UnderflowError { msg: String },
    #[error("Divide by zero: {msg}")]
    DivideByZeroError { msg: String },
    #[error("Construction error: {msg}")]
    ConstructionError { msg: String },
    #[error("Not implemented: {msg}")]
    NotImplemented { msg: String },
    #[error("Runtime error: {msg}")]
    RuntimeError { msg: String },
    #[error("Unknown error: {msg}")]
    UnknownError { msg: String },
}

// Implement From conversions for common error types
impl From<std::io::Error> for Failure {
    fn from(err: std::io::Error) -> Self {
        Failure::RuntimeError { msg: err.to_string() }
    }
}

impl From<std::num::TryFromIntError> for Failure {
    fn from(err: std::num::TryFromIntError) -> Self {
        Failure::NumericError { msg: err.to_string() }
    }
}

impl From<std::num::ParseIntError> for Failure {
    fn from(err: std::num::ParseIntError) -> Self {
        Failure::NumericError { msg: err.to_string() }
    }
}

impl From<std::num::ParseFloatError> for Failure {
    fn from(err: std::num::ParseFloatError) -> Self {
        Failure::NumericError { msg: err.to_string() }
    }
}

impl Failure {
    #[inline]
    pub fn domain_error(msg: impl Into<String>) -> Self {
        Self::DomainError { msg: msg.into() }
    }

    #[inline]
    pub fn range_error(msg: impl Into<String>) -> Self {
        Self::RangeError { msg: msg.into() }
    }

    #[inline]
    pub fn numeric_error(msg: impl Into<String>) -> Self {
        Self::NumericError { msg: msg.into() }
    }

    #[inline]
    pub fn overflow_error(msg: impl Into<String>) -> Self {
        Self::OverflowError { msg: msg.into() }
    }

    #[inline]
    pub fn underflow_error(msg: impl Into<String>) -> Self {
        Self::UnderflowError { msg: msg.into() }
    }

    #[inline]
    pub fn divide_by_zero(msg: impl Into<String>) -> Self {
        Self::DivideByZeroError { msg: msg.into() }
    }

    #[inline]
    pub fn construction_error(msg: impl Into<String>) -> Self {
        Self::ConstructionError { msg: msg.into() }
    }

    #[inline]
    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented { msg: msg.into() }
    }

    #[inline]
    pub fn runtime_error(msg: impl Into<String>) -> Self {
        Self::RuntimeError { msg: msg.into() }
    }

    #[inline]
    pub fn unknown_error(msg: impl Into<String>) -> Self {
        Self::UnknownError { msg: msg.into() }
    }
}

/// Trait for ergonomic error handling on Option types
///
/// This trait provides a convenient way to convert None values
/// to panics with custom error messages.
pub trait RaiseIf {
    /// Convert None to a panic with the given error
    ///
    /// # Panics
    /// Panics if self is None
    fn raise_if<F>(self, error: F) -> Self
    where
        F: FnOnce() -> Failure;
}

impl<T> RaiseIf for Option<T> {
    #[inline]
    fn raise_if<F>(self, error: F) -> Self
    where
        F: FnOnce() -> Failure,
    {
        if self.is_none() {
            panic!("{}", error().to_string());
        }
        self
    }
}

/// Raise an error if condition is true
///
/// This function provides a convenient way to check conditions
/// and panic with custom error messages.
///
/// # Panics
/// Panics if condition is true
///
/// # Example
/// ```rust
/// use breprs::foundation::exception::raise_if;
/// use breprs::foundation::exception::Failure;
///
/// let value = 1;
/// raise_if(value < 0, || Failure::range_error("value must be non-negative"));
/// ```
#[inline]
pub fn raise_if<F>(condition: bool, error: F)
where
    F: FnOnce() -> Failure,
{
    if condition {
        panic!("{}", error().to_string());
    }
}

/// Raise a domain error and panic
///
/// # Panics
/// Always panics with the given domain error message
#[inline]
pub fn raise_domain_error(msg: impl Into<String>) -> ! {
    panic!("{}", Failure::domain_error(msg).to_string());
}

/// Raise a range error and panic
///
/// # Panics
/// Always panics with the given range error message
#[inline]
pub fn raise_range_error(msg: impl Into<String>) -> ! {
    panic!("{}", Failure::range_error(msg).to_string());
}

/// Raise a numeric error and panic
///
/// # Panics
/// Always panics with the given numeric error message
#[inline]
pub fn raise_numeric_error(msg: impl Into<String>) -> ! {
    panic!("{}", Failure::numeric_error(msg).to_string());
}

/// Raise a divide by zero error and panic
///
/// # Panics
/// Always panics with the given divide by zero error message
#[inline]
pub fn raise_divide_by_zero(msg: impl Into<String>) -> ! {
    panic!("{}", Failure::divide_by_zero(msg).to_string());
}

/// Raise a construction error and panic
///
/// # Panics
/// Always panics with the given construction error message
#[inline]
pub fn raise_construction_error(msg: impl Into<String>) -> ! {
    panic!("{}", Failure::construction_error(msg).to_string());
}

/// Macro for conditional error raising
///
/// This macro provides a convenient way to check conditions
/// and panic with custom error messages.
///
/// # Example
/// ```rust
/// use breprs::standard_raise_if;
/// use breprs::foundation::exception::Failure;
///
/// let value = 1;
/// standard_raise_if!(value < 0, Failure::range_error("value must be non-negative"));
/// ```
#[macro_export]
macro_rules! standard_raise_if {
    ($condition:expr, $error:expr) => {
        if $condition {
            panic!("{}", $error.to_string());
        }
    };
}

/// Macro for early return on error
///
/// This macro provides a convenient way to propagate errors
/// following Rust's ? operator pattern, but with custom error handling.
///
/// # Example
/// ```rust
/// use breprs::standard_try;
/// use breprs::foundation::exception::{Result, Failure};
///
/// fn some_operation() -> Result<i32> {
///     Ok(42)
/// }
///
/// fn example() -> Result<i32> {
///     let result = standard_try!(some_operation());
///     Ok(result)
/// }
/// ```
#[macro_export]
macro_rules! standard_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(e) => return Err(e),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Failure>();
        assert_send_sync::<Result<i32>>();
    }

    #[test]
    fn test_failure_variant_context_source_backtrace() {
        let domain = Failure::DomainError { msg: "domain".to_string() };
        assert!(matches!(domain, Failure::DomainError { .. }));

        let numeric = Failure::NumericError { msg: "numeric".to_string() };
        assert!(matches!(numeric, Failure::NumericError { .. }));

        let not_impl = Failure::NotImplemented { msg: "notimpl".to_string() };
        assert!(matches!(not_impl, Failure::NotImplemented { .. }));
    }

    #[test]
    fn test_standard_failure_creation() {
        let err = Failure::domain_error("test error");
        assert!(matches!(err, Failure::DomainError { .. }));

        let err = Failure::range_error("test");
        assert!(matches!(err, Failure::RangeError { .. }));

        let err = Failure::numeric_error("test");
        assert!(matches!(err, Failure::NumericError { .. }));

        let err = Failure::overflow_error("test");
        assert!(matches!(err, Failure::OverflowError { .. }));

        let err = Failure::underflow_error("test");
        assert!(matches!(err, Failure::UnderflowError { .. }));

        let err = Failure::divide_by_zero("test");
        assert!(matches!(err, Failure::DivideByZeroError { .. }));

        let err = Failure::construction_error("test");
        assert!(matches!(err, Failure::ConstructionError { .. }));

        let err = Failure::not_implemented("test");
        assert!(matches!(err, Failure::NotImplemented { .. }));

        let err = Failure::runtime_error("test");
        assert!(matches!(err, Failure::RuntimeError { .. }));

        let err = Failure::unknown_error("test");
        assert!(matches!(err, Failure::UnknownError { .. }));
    }

    #[test]
    fn test_standard_failure_display() {
        let err = Failure::domain_error("test error");
        assert_eq!(err.to_string(), "Domain error: test error");

        let err = Failure::range_error("range test");
        assert_eq!(err.to_string(), "Range error: range test");

        let err = Failure::numeric_error("numeric test");
        assert_eq!(err.to_string(), "Numeric error: numeric test");

        let err = Failure::not_implemented("feature");
        assert_eq!(err.to_string(), "Not implemented: feature");
    }

    #[test]
    fn test_raise_if() {
        let result = std::panic::catch_unwind(|| {
            raise_if(true, || Failure::domain_error("test"));
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_if_no_panic() {
        raise_if(false, || Failure::domain_error("test"));
    }

    #[test]
    fn test_standard_result() {
        let result: Result<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);

        let result: Result<i32> = Err(Failure::domain_error("test"));
        assert!(result.is_err());
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let failure: Failure = io_err.into();
        assert!(matches!(failure, Failure::RuntimeError { .. }));
        assert!(failure.to_string().contains("file not found"));
    }

    #[test]
    fn test_from_parse_int_error() {
        let result: std::result::Result<i32, _> = "not a number".parse();
        assert!(result.is_err());
        let parse_err = result.unwrap_err();
        let failure: Failure = parse_err.into();
        assert!(matches!(failure, Failure::NumericError { .. }));
    }

    #[test]
    fn test_from_parse_float_error() {
        let result: std::result::Result<f64, _> = "not a float".parse();
        assert!(result.is_err());
        let parse_err = result.unwrap_err();
        let failure: Failure = parse_err.into();
        assert!(matches!(failure, Failure::NumericError { .. }));
    }

    #[test]
    fn test_from_try_from_int_error() {
        let result: std::result::Result<i8, _> = 1000i64.try_into();
        assert!(result.is_err());
        let try_err = result.unwrap_err();
        let failure: Failure = try_err.into();
        assert!(matches!(failure, Failure::NumericError { .. }));
    }

    #[test]
    fn test_raise_if_trait() {
        let some: Option<i32> = Some(42);
        let result = some.raise_if(|| Failure::domain_error("should not panic"));
        assert_eq!(result, Some(42));

        let none: Option<i32> = None;
        let result = std::panic::catch_unwind(|| {
            none.raise_if(|| Failure::domain_error("should panic"));
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_domain_error() {
        let result = std::panic::catch_unwind(|| {
            raise_domain_error("domain error message");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_range_error() {
        let result = std::panic::catch_unwind(|| {
            raise_range_error("range error message");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_numeric_error() {
        let result = std::panic::catch_unwind(|| {
            raise_numeric_error("numeric error message");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_divide_by_zero() {
        let result = std::panic::catch_unwind(|| {
            raise_divide_by_zero("divide by zero");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_construction_error() {
        let result = std::panic::catch_unwind(|| {
            raise_construction_error("construction failed");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_standard_raise_if_macro() {
        // Test that macro does not panic when condition is false
        standard_raise_if!(false, Failure::domain_error("should not panic"));

        // Test that macro panics when condition is true
        let result = std::panic::catch_unwind(|| {
            standard_raise_if!(true, Failure::domain_error("should panic"));
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_standard_try_macro() {
        fn success_op() -> Result<i32> {
            Ok(42)
        }

        fn failure_op() -> Result<i32> {
            Err(Failure::domain_error("failed"))
        }

        fn test_success() -> Result<i32> {
            let val = standard_try!(success_op());
            Ok(val)
        }

        fn test_failure() -> Result<i32> {
            let _val = standard_try!(failure_op());
            Ok(0)
        }

        assert_eq!(test_success().unwrap(), 42);
        assert!(test_failure().is_err());
    }

    #[test]
    fn test_error_debug() {
        let err = Failure::domain_error("test");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("DomainError"));
    }
}
