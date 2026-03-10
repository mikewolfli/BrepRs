//! Advanced GPU Features Module
//!
//! This module provides GPU-accelerated features for BrepRs using WGPU,
//! including compute shaders, ray tracing, and advanced rendering techniques.

#[cfg(feature = "gpu")]
pub mod compute;
#[cfg(feature = "gpu")]
pub mod illumination;
#[cfg(feature = "gpu")]
pub mod mesh_shaders;
#[cfg(feature = "gpu")]
pub mod multi_gpu;
#[cfg(feature = "gpu")]
pub mod neural;
#[cfg(feature = "gpu")]
pub mod ray_tracing;
#[cfg(feature = "gpu")]
pub mod rendering;
#[cfg(feature = "gpu")]
pub mod virtual_texture;

#[cfg(feature = "gpu")]
pub use compute::*;
#[cfg(feature = "gpu")]
pub use illumination::*;
#[cfg(feature = "gpu")]
pub use mesh_shaders::*;
#[cfg(feature = "gpu")]
pub use multi_gpu::*;
#[cfg(feature = "gpu")]
pub use neural::*;
#[cfg(feature = "gpu")]
pub use ray_tracing::*;
#[cfg(feature = "gpu")]
pub use rendering::*;
#[cfg(feature = "gpu")]
pub use virtual_texture::*;
