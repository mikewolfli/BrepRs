//! BrepRs - Rust implementation of boundary representation (BRep) for CAD/CAE/CAM applications
//! 
//! This library provides a comprehensive implementation of boundary representation
//! for CAD/CAE/CAM applications, featuring:
//! - Foundation types and utilities
//! - Collection types
//! - Memory management
//! - Exception handling
//! - Smart pointers
//! - Geometry primitives and operations

pub mod foundation;
pub mod collections;
pub mod geometry;
pub mod topology;
pub mod modeling;
pub mod data_exchange;

pub use foundation::*;
