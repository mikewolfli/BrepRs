//! Application framework module
//! 
//! This module provides the application framework for BrepRs, including
//! data framework, document management, standard attributes, and topological
//! naming and history functionality.

pub mod data_framework;
pub mod document;
pub mod attributes;
pub mod topological_naming;

pub use data_framework::*;
pub use document::*;
pub use attributes::*;
pub use topological_naming::*;