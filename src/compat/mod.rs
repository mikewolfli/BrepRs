//! OpenCASCADE API Compatibility Layer
//!
//! This module provides OpenCASCADE-compatible naming conventions
//! for users migrating from OpenCASCADE or familiar with its API.
//!
//! # Usage
//!
//! ```rust
//! use breprs::compat::{TopoDS_Shape, BRep_Builder, gp_Pnt};
//!
//! let shape = TopoDS_Shape::new(ShapeType::Vertex);
//! ```
//!
//! For idiomatic Rust code, use the native API instead:
//!
//! ```rust
//! use breprs::topology::TopoDsShape;
//! use breprs::geometry::Point;
//!
//! let shape = TopoDsShape::new(ShapeType::Vertex);
//! ```

pub mod topology;
pub mod modeling;
pub mod geometry;

// Re-export commonly used types
pub use topology::*;
pub use modeling::*;
pub use geometry::*;
