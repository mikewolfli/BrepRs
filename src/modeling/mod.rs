//! Modeling algorithms module
//!
//! This module contains algorithms for creating and manipulating geometric shapes.

pub mod bioreactor;
pub mod boolean_operations;
pub mod brep_builder;
pub mod bsp_tree;
pub mod constraint_solver;
pub mod cube;
pub mod electronics;
pub mod fillet_chamfer;
pub mod offset_operations;
pub mod parametric;
pub mod post_processing;
pub mod primitives;
pub mod tissue;
pub use bioreactor::*;
pub use boolean_operations::BooleanOperations;
pub use brep_builder::BrepBuilder;
pub use constraint_solver::ConstraintSolver;
pub use cube::Cube;
pub use electronics::*;
pub use fillet_chamfer::FilletChamfer;
pub use offset_operations::OffsetOperations;
pub use offset_operations::{IntersectionType, JoinType};
pub use parametric::{
    Parameter, ParameterManager, ParametricCube, ParametricCylinder, ParametricShape,
};
pub use post_processing::{
    BooleanOperation, MeshDecimator, MeshSubdivider, PostProcessing, SubdivisionScheme,
};
pub use primitives::*;
pub use tissue::*;
