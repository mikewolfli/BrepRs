#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE ML Compatibility Module
//! 
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for machine learning functionality.

// Re-export ML types with OpenCASCADE naming
pub use crate::ml::{ 
    MlUtils as ML_Utils,
    FeatureRecognitionModel as ML_FeatureRecognitionModel,
    ModelRepairModel as ML_ModelRepairModel,
};

// Re-export PyTorch integration if feature is enabled
#[cfg(feature = "pytorch")]
pub use crate::ml::pytorch as ML_PyTorch;

// Re-export TensorFlow integration if feature is enabled
#[cfg(feature = "tensorflow")]
pub use crate::ml::tensorflow as ML_TensorFlow;
