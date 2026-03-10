//! Modeling algorithms module
//!
//! This module contains algorithms for creating and manipulating geometric shapes.

pub mod boolean_operations;
pub mod brep_builder;
pub mod bsp_tree;
pub mod fillet_chamfer;
pub mod offset_operations;
pub mod post_processing;
pub mod primitives;

pub use boolean_operations::BooleanOperations;
pub use brep_builder::BrepBuilder;
pub use fillet_chamfer::FilletChamfer;
pub use offset_operations::OffsetOperations;
pub use offset_operations::{IntersectionType, JoinType};
pub use post_processing::{
    BooleanOperation, MeshDecimator, MeshSubdivider, PostProcessing, SubdivisionScheme,
};
pub use primitives::*;
