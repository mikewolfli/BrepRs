#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE LOD Compatibility Module
//! 
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for LOD (Level of Detail) functionality.

// Re-export LOD types with OpenCASCADE naming
pub use crate::visualization::lod::{ 
    LodLevel as LOD_Level,
    LodQualityMetrics as LOD_QualityMetrics,
    LodSystem as LOD_System,
    LodParams as LOD_Params,
    LodTransitionManager as LOD_TransitionManager,
    LodTransitionParams as LOD_TransitionParams,
    LodTransition as LOD_Transition,
    LodCollisionDetector as LOD_CollisionDetector,
    CollisionParams as LOD_CollisionParams,
    LodDebugger as LOD_Debugger,
    DebugParams as LOD_DebugParams,
};

// Re-export LOD memory manager
pub use crate::foundation::memory::LodMemoryManager as LOD_MemoryManager;
