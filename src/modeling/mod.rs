//! Modeling algorithms module
//! 
//! This module contains algorithms for creating and manipulating geometric shapes.

pub mod primitives;
pub mod brep_builder;
pub mod boolean_operations;
pub mod fillet_chamfer;
pub mod offset_operations;

pub use primitives::*;
pub use brep_builder::BrepBuilder;
pub use boolean_operations::BooleanOperations;
pub use fillet_chamfer::FilletChamfer;
pub use offset_operations::OffsetOperations;
pub use offset_operations::{JoinType, IntersectionType};
