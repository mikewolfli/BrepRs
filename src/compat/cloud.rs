#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE Cloud Compatibility Module
//! 
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for cloud-native functionality.

// Re-export cloud types with OpenCASCADE naming
pub use crate::cloud::{ 
    WebRtcStreamer as CLOUD_WebRtcStreamer,
    CloudStorage as CLOUD_Storage,
    CrdtManager as CLOUD_CrdtManager,
    CloudDocument as CLOUD_Document,
};
