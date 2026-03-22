//! Surface module
//!
//! This module provides advanced surface operations and algorithms for 3D modeling.
//! It includes surface analysis, editing, deformation, matching, and reconstruction capabilities.
//!
//! # Main Components
//!
//! - **Surface Analysis**: Curvature analysis, fitting analysis, and quality assessment
//! - **Surface Editing**: Surface modification and manipulation tools
//! - **Surface Deformation**: Free-form deformation (FFD) and other deformation techniques
//! - **Surface Matching**: Surface alignment and registration algorithms
//! - **Surface Reconstruction**: Point cloud to surface reconstruction
//! - **UV Parameterization**: Surface parameterization for texture mapping
//! - **Implicit Surfaces**: Implicit surface representation and operations
//! - **Clipping**: Surface clipping and trimming operations
//!
//! # Example
//!
//! ```rust
//! use breprs::surface::{SurfaceAnalysis, SurfaceEditing};
//!
//! // Analyze surface curvature
//! let curvature = SurfaceAnalysis::curvature(&surface);
//!
//! // Edit surface
//! let edited = SurfaceEditing::smooth(&surface, 0.5);
//! ```

pub mod clipping;
pub mod implicit;
pub mod reconstruction;
pub mod uv_parameterization;
pub mod fitting_analysis;
pub mod surface_editing;
pub mod surface_matching;
pub mod surface_analysis;
pub mod surface_deformation;

pub use clipping::*;
pub use fitting_analysis::*;
pub use implicit::*;
pub use reconstruction::*;
pub use uv_parameterization::*;
pub use surface_editing::*;
pub use surface_matching::*;
pub use surface_analysis::*;
pub use surface_deformation::*;
