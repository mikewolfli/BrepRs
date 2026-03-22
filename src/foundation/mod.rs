//! Foundation module
//!
//! This module provides foundational types and utilities that are used throughout the BrepRs library.
//! It includes core data structures, memory management, exception handling, and tolerance management.
//!
//! # Main Components
//!
//! - **Handle**: Reference-counted smart pointer for sharing data across the library
//! - **Transient**: Temporary objects with automatic cleanup
//! - **Exception**: Error handling and exception types
//! - **Memory**: Memory management utilities
//! - **Tolerance**: Geometric tolerance management
//! - **Types**: Core type definitions and constants
//!
//! # Example
//!
//! ```rust
//! use breprs::foundation::{Handle, StandardReal};
//!
//! let data = Handle::new(my_data);
//! ```

pub mod types;
pub mod transient;
pub mod handle;
pub mod exception;
pub mod memory;
pub mod tolerance;

pub use types::*;
pub use transient::*;
pub use handle::*;
pub use exception::*;
pub use memory::*;
pub use tolerance::*;
