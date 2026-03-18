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
//!
//! # API Styles
//!
//! This library provides two API styles:
//!
//! 1. **Native Rust API** (default): Idiomatic Rust naming conventions
//!    - Types: `TopoDsShape`, `BrepBuilder`, `Point`
//!    - Methods: `new()`, `add_vertex()`, `compute_bounding_box()`
//!
//! 2. **OpenCASCADE Compatibility API**: Compatible with OpenCASCADE naming
//!    - Types: `TopoDS_Shape`, `BRep_Builder`, `gp_Pnt`
//!    - Methods: `MakeVertex()`, `Build()`, `Perform()`
//!
//! Use the native API for new Rust projects, and the compatibility API for
//! migrating existing OpenCASCADE code.

pub mod ai_ml;
pub mod api;
pub mod application;
pub mod benchmarking;
pub mod build;
pub mod cloud;
pub mod collections;
pub mod data_exchange;
pub mod foundation;
pub mod geometry;
pub mod gpu;
pub mod mesh;
pub mod modeling;
pub mod plugin;
pub mod plugins;
pub mod simulation;
pub mod surface;
pub mod topology;
pub mod visualization;

// Re-export ai_ml module as ai and ml for backward compatibility
pub mod ai {
    pub use crate::ai_ml::*;
}

pub mod ml {
    pub use crate::ai_ml::*;
}

/// OpenCASCADE API Compatibility Layer
///
/// This module provides OpenCASCADE-compatible naming conventions
/// for users migrating from OpenCASCADE or familiar with its API.
pub mod compat;

/// WebAssembly bindings module (requires `wasm` feature)
#[cfg(feature = "wasm")]
pub mod wasm;

/// Serialization support module (requires `serde` feature)
#[cfg(feature = "serde")]
pub mod serialization;

/// Parallel processing module (requires `rayon` feature)
#[cfg(feature = "rayon")]
pub mod parallel;

pub use foundation::*;
