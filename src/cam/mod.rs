//! Computer-Aided Manufacturing (CAM) Module
//!
//! This module provides comprehensive CAM functionality for generating toolpaths,
//! G-code, and machining operations from 3D models.
//!
//! # Main Components
//!
//! - **Toolpath Generation**: Generate efficient toolpaths from 3D models
//! - **G-Code Generation**: Convert toolpaths to standard G-code format
//! - **Machining Operations**: Various machining strategies (roughing, finishing, etc.)
//! - **Simulation**: Visualize and verify machining operations
//!
//! # Example
//!
//! ```rust
//! use breprs::cam::{Toolpath, GCodeGenerator, MachiningStrategy};
//!
//! // Generate toolpath
//! let toolpath = Toolpath::generate(&model, MachiningStrategy::Roughing);
//!
//! // Convert to G-code
//! let gcode = GCodeGenerator::generate(&toolpath);
//! ```

pub mod toolpath;
pub mod gcode;
pub mod machining;
pub mod simulation;

pub use toolpath::*;
pub use gcode::*;
pub use machining::*;
pub use simulation::*;
