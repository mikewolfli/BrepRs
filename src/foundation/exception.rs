use std::fmt;
use thiserror::Error;

pub type Standard_Result<T> = Result<T, Standard_Failure>;

#[derive(Error, Debug)]
pub enum Standard_Failure {
    #[error("Domain error: {0}")]
    DomainError(String),

    #[error("Range error: {0}")]
    RangeError(String),

    #[error("Numeric error: {0}")]
    NumericError(String),

    #[error("Overflow error: {0}")]
    OverflowError(String),

    #[error("Underflow error: {0}")]
    UnderflowError(String),

    #[error("Divide by zero: {0}")]
    DivideByZeroError(String),

    #[error("Construction error: {0}")]
    ConstructionError(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

impl Standard_Failure {
    pub fn domain_error(msg: impl Into<String>) -> Self {
        Self::DomainError(msg.into())
    }

    pub fn range_error(msg: impl Into<String>) -> Self {
        Self::RangeError(msg.into())
    }

    pub fn numeric_error(msg: impl Into<String>) -> Self {
        Self::NumericError(msg.into())
    }

    pub fn overflow_error(msg: impl Into<String>) -> Self {
        Self::OverflowError(msg.into())
    }

    pub fn underflow_error(msg: impl Into<String>) -> Self {
        Self::UnderflowError(msg.into())
    }

    pub fn divide_by_zero(msg: impl Into<String>) -> Self {
        Self::DivideByZeroError(msg.into())
    }

    pub fn construction_error(msg: impl Into<String>) -> Self {
        Self::ConstructionError(msg.into())
    }

    pub fn not_implemented(msg: impl Into<String>) -> Self {
        Self::NotImplemented(msg.into())
    }

    pub fn runtime_error(msg: impl Into<String>) -> Self {
        Self::RuntimeError(msg.into())
    }

    pub fn unknown_error(msg: impl Into<String>) -> Self {
        Self::UnknownError(msg.into())
    }
}

pub trait Standard_RaiseIf {
    fn raise_if<F>(self, error: F) -> Self
    where
        F: FnOnce() -> Standard_Failure;
}

impl<T> Standard_RaiseIf for Option<T> {
    fn raise_if<F>(self, error: F) -> Self
    where
        F: FnOnce() -> Standard_Failure,
    {
        if self.is_none() {
            panic!("{}", error().to_string());
        }
        self
    }
}

pub fn raise_if<F>(condition: bool, error: F)
where
    F: FnOnce() -> Standard_Failure,
{
    if condition {
        panic!("{}", error().to_string());
    }
}

pub fn raise_domain_error(msg: impl Into<String>) -> ! {
    panic!("{}", Standard_Failure::domain_error(msg).to_string());
}

pub fn raise_range_error(msg: impl Into<String>) -> ! {
    panic!("{}", Standard_Failure::range_error(msg).to_string());
}

pub fn raise_numeric_error(msg: impl Into<String>) -> ! {
    panic!("{}", Standard_Failure::numeric_error(msg).to_string());
}

pub fn raise_divide_by_zero(msg: impl Into<String>) -> ! {
    panic!("{}", Standard_Failure::divide_by_zero(msg).to_string());
}

pub fn raise_construction_error(msg: impl Into<String>) -> ! {
    panic!("{}", Standard_Failure::construction_error(msg).to_string());
}

#[macro_export]
macro_rules! standard_raise_if {
    ($condition:expr, $error:expr) => {
        if $condition {
            panic!("{}", $error.to_string());
        }
    };
}

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
    fn test_standard_failure_creation() {
        let err = Standard_Failure::domain_error("test error");
        assert!(matches!(err, Standard_Failure::DomainError(_)));
    }

    #[test]
    fn test_standard_failure_display() {
        let err = Standard_Failure::domain_error("test error");
        assert_eq!(err.to_string(), "Domain error: test error");
    }

    #[test]
    fn test_raise_if() {
        let result = std::panic::catch_unwind(|| {
            raise_if(true, || Standard_Failure::domain_error("test"));
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_raise_if_no_panic() {
        raise_if(false, || Standard_Failure::domain_error("test"));
    }

    #[test]
    fn test_standard_result() {
        let result: Standard_Result<i32> = Ok(42);
        assert_eq!(result.unwrap(), 42);

        let result: Standard_Result<i32> = Err(Standard_Failure::domain_error("test"));
        assert!(result.is_err());
    }
}
