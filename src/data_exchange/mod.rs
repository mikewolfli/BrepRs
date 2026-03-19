// Data exchange module
//
// This module provides functionality for reading and writing various 3D file formats.

use thiserror::Error;

pub mod fbx;
pub mod gltf;
pub mod iges;
pub mod modern_formats;
pub mod obj;
pub mod ode;
pub mod ply;
pub mod step;
pub mod stl;
pub mod threemf;
pub mod usd;
pub mod usdz;
pub mod utils;
pub mod vtk;

pub use fbx::*;
pub use gltf::*;
pub use iges::*;
pub use modern_formats::*;
pub use obj::*;
pub use ode::*;
pub use ply::*;
pub use step::*;
pub use stl::*;
pub use threemf::*;
pub use usd::*;
pub use usdz::*;
pub use utils::*;
pub use vtk::*;

#[derive(Error, Debug)]
pub enum DataExchangeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(i32),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Conversion error: {0}")]
    ConversionError(String),
}

pub type DataExchangeResult<T> = Result<T, DataExchangeError>;

// Conversion from ZipError
impl From<zip::result::ZipError> for DataExchangeError {
    fn from(err: zip::result::ZipError) -> Self {
        DataExchangeError::ParseError(err.to_string())
    }
}
// Conversion from IgesError
impl From<crate::data_exchange::iges::IgesError> for DataExchangeError {
    fn from(err: crate::data_exchange::iges::IgesError) -> Self {
        DataExchangeError::ParseError(err.to_string())
    }
}
// Conversion from StlError
impl From<crate::data_exchange::stl::StlError> for DataExchangeError {
    fn from(err: crate::data_exchange::stl::StlError) -> Self {
        DataExchangeError::ParseError(err.to_string())
    }
}

// Conversion from StepError
impl From<crate::data_exchange::step::StepError> for DataExchangeError {
    fn from(err: crate::data_exchange::step::StepError) -> Self {
        DataExchangeError::ParseError(err.to_string())
    }
}
