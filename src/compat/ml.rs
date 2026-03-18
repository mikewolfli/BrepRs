#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_variables
)]
//! OpenCASCADE ML Compatibility Module
//!
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for machine learning functionality (now integrated into AI_ML module).

// Re-export ML types with OpenCASCADE naming from AI_ML
pub use crate::ai_ml::{
    models::{
        FeatureRecognitionModel as ML_FeatureRecognitionModel,
        MeshGenerationModel as ML_MeshGenerationModel, ModelRepairModel as ML_ModelRepairModel,
    },
    utils::AiMlUtils as ML_Utils,
};

// Re-export PyTorch integration if feature is enabled
#[cfg(feature = "pytorch")]
pub use crate::ai_ml::frameworks::pytorch as ML_PyTorch;

// Re-export TensorFlow integration if feature is enabled
#[cfg(feature = "tensorflow")]
pub use crate::ai_ml::frameworks::tensorflow as ML_TensorFlow;
