#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE API Compatibility Module
//! 
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for next-generation API functionality.

// Re-export API types with OpenCASCADE naming
pub use crate::api::optimized::{ 
    OptimizedPoint as API_OptimizedPoint,
    OptimizedMesh as API_OptimizedMesh,
    OptimizedShape as API_OptimizedShape,
    OptimizedVertex as API_OptimizedVertex,
    OptimizedEdge as API_OptimizedEdge,
    OptimizedFace as API_OptimizedFace,
};

// Re-export incremental compilation types
pub use crate::api::incremental::{ 
    HotReloadManager as API_HotReloadManager,
    IncrementalMeshBuilder as API_IncrementalMeshBuilder,
};

// Re-export documentation types
pub use crate::api::documentation::{ 
    ApiDocGenerator as API_DocGenerator,
};
