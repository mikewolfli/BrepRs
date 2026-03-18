#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE Cloud Compatibility Module
//!
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for cloud-native functionality.

// Re-export cloud types with OpenCASCADE naming
pub use crate::cloud::{
    CloudStorageManager as CLOUD_StorageManager,
    CloudStorageInterface as CLOUD_StorageInterface,
    CrdtManager as CLOUD_CrdtManager,
    CrdtDocument as CLOUD_Document,
};
