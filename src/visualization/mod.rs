pub mod font;
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
pub mod gpu_memory;
pub mod gpu_buffer;
pub mod texture_stream;
pub mod gpu_compression;
pub mod lod;

pub use primitives::*;
pub use renderer::*;
pub use interactive::*;
pub use view::*;
pub use camera::*;
pub use light::*;
pub use material::{Material, MaterialType, Texture, TextureType, MaterialPresets};
pub use gpu_memory::*;
pub use gpu_buffer::*;
pub use texture_stream::*;
pub use gpu_compression::*;
pub use lod::*;
