//! Visualization module
//!
//! This module provides functionality for 3D visualization, including
//! graphics primitives, rendering, interactive objects, and view control.
//! Compatible with OpenCASCADE Open API design.

pub mod primitives;
pub mod renderer;
pub mod interactive;
pub mod view;
pub mod camera;
pub mod light;
pub mod material;

pub use primitives::*;
pub use renderer::*;
pub use interactive::*;
pub use view::*;
pub use camera::*;
pub use light::*;
pub use material::*;
