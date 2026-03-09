//! Mesh generation module
//!
//! This module provides functionality for mesh generation, including
//! mesh data structures, 2D triangle meshing, 3D tetrahedral meshing,
//! and mesh quality optimization.

pub mod mesh_data;
pub mod mesher2d;
pub mod mesher3d;
pub mod quality;

pub use mesh_data::*;
pub use mesher2d::*;
pub use mesher3d::*;
pub use quality::*;