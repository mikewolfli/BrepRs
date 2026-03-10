//! OpenCASCADE API Compatibility Layer
//!
//! This module provides OpenCASCADE-compatible naming conventions
//! for users migrating from OpenCASCADE or familiar with its API.
//!
//! # Usage
//!
//! ```rust
//! use breprs::compat::{TopoDS_Shape, BRep_Builder, gp_Pnt, ShapeType};
//!
//! let shape = TopoDS_Shape::new(ShapeType::Vertex);
//! ```
//!
//! For idiomatic Rust code, use the native API instead:
//!
//! ```rust
//! use breprs::topology::{TopoDsShape, ShapeType};
//! use breprs::geometry::Point;
//!
//! let shape = TopoDsShape::new(ShapeType::Vertex);
//! ```

pub mod api;
pub mod cloud;
pub mod geometry;
pub mod lod;
pub mod ml;
pub mod modeling;
pub mod topexp;
pub mod topology;
pub mod toptools;

// Re-export commonly used types
pub use api::*;
pub use cloud::*;
pub use geometry::*;
pub use lod::*;
pub use ml::*;
pub use modeling::*;
pub use topexp::*;
pub use topology::*;
pub use toptools::*;

// Re-export ShapeType for convenience
pub use topology::ShapeType;
