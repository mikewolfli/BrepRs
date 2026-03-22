//! Modeling algorithms module
//!
//! This module contains algorithms for creating and manipulating geometric shapes.

pub mod advanced_boolean;
pub mod bioreactor;
pub mod boolean_operations;
pub mod brep_builder;
pub mod bsp_tree;
pub mod constraint_solver;
pub mod cube;
pub mod electronics;
pub mod fillet_chamfer;
pub mod multi_resolution;
pub mod offset_operations;
pub mod parametric;
pub mod post_processing;
pub mod primitives;
pub mod feature_history;
pub mod shape_repair;
pub mod sketch;
pub mod solid_decomposition;
pub mod tissue;
pub use advanced_boolean::AdvancedBooleanOperations;
pub use bioreactor::*;
pub use boolean_operations::BooleanOperations;
pub use brep_builder::BrepBuilder;
pub use constraint_solver::ConstraintSolver;
pub use cube::Cube;
pub use electronics::*;
pub use fillet_chamfer::FilletChamfer;
pub use multi_resolution::*;
pub use offset_operations::OffsetOperations;
pub use offset_operations::{IntersectionType, JoinType};
pub use parametric::{
    Parameter, ParametricModel,
};
pub use post_processing::{
    BooleanOperation, MeshDecimator, MeshSubdivider, PostProcessing, SubdivisionScheme,
};
pub use primitives::*;
pub use feature_history::*;
pub use shape_repair::*;
pub use sketch::*;
pub use solid_decomposition::*;
pub use tissue::*;
