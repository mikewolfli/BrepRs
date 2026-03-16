//! Serialization Support Module
//!
//! This module provides serialization/deserialization support for BrepRs types
//! using Serde library. It supports multiple formats including JSON, BSON,
//! and MessagePack.

#[cfg(feature = "serde")]
pub mod bson;
#[cfg(feature = "serde")]
pub mod json;
#[cfg(feature = "serde")]
pub mod msgpack;

#[cfg(feature = "serde")]
pub use bson::*;
#[cfg(feature = "serde")]
pub use json::*;
#[cfg(feature = "serde")]
pub use msgpack::*;

/// Trait for types that can be serialized and deserialized
#[cfg(feature = "serde")]
pub trait Serializable: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Serialize to JSON string
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from JSON string
    fn from_json(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized,
    {
        serde_json::from_str(json)
    }

    /// Serialize to JSON bytes
    fn to_json_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    /// Deserialize from JSON bytes
    fn from_json_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error>
    where
        Self: Sized,
    {
        serde_json::from_slice(bytes)
    }
}

/// Serialize a value to JSON string
#[cfg(feature = "serde")]
pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(value)
}

/// Deserialize a value from JSON string
#[cfg(feature = "serde")]
pub fn from_json<T: for<'de> serde::Deserialize<'de>>(json: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

/// Serialize a value to JSON bytes
#[cfg(feature = "serde")]
pub fn to_json_bytes<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec(value)
}

/// Deserialize a value from JSON bytes
#[cfg(feature = "serde")]
pub fn from_json_bytes<T: for<'de> serde::Deserialize<'de>>(
    bytes: &[u8],
) -> Result<T, serde_json::Error> {
    serde_json::from_slice(bytes)
}

/// Serialize a value to compact JSON (no whitespace)
#[cfg(feature = "serde")]
pub fn to_json_compact<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Serialization format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Json,
    Bson,
    MessagePack,
}

impl SerializationFormat {
    /// Get the file extension for this format
    pub fn file_extension(&self) -> &'static str {
        match self {
            SerializationFormat::Json => "json",
            SerializationFormat::Bson => "bson",
            SerializationFormat::MessagePack => "msgpack",
        }
    }

    /// Get the MIME type for this format
    pub fn mime_type(&self) -> &'static str {
        match self {
            SerializationFormat::Json => "application/json",
            SerializationFormat::Bson => "application/bson",
            SerializationFormat::MessagePack => "application/msgpack",
        }
    }
}

/// Serialization options
#[derive(Debug, Clone)]
pub struct SerializationOptions {
    pub format: SerializationFormat,
    pub pretty: bool,
    pub include_metadata: bool,
}

impl Default for SerializationOptions {
    fn default() -> Self {
        Self {
            format: SerializationFormat::Json,
            pretty: true,
            include_metadata: true,
        }
    }
}

/// Serialization metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializationMetadata {
    pub version: String,
    pub created_at: String,
    pub format_version: u32,
}

impl Default for SerializationMetadata {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            format_version: 1,
        }
    }
}

/// Wrapper for serialized data with metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SerializedData<T> {
    pub metadata: SerializationMetadata,
    pub data: T,
}

impl<T> SerializedData<T> {
    pub fn new(data: T) -> Self {
        Self {
            metadata: SerializationMetadata::default(),
            data,
        }
    }

    pub fn with_metadata(data: T, metadata: SerializationMetadata) -> Self {
        Self { metadata, data }
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + for<'de> serde::Deserialize<'de>> Serializable for SerializedData<T> {}

/// Serialization error types
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    // Removed unexpected cfg feature blocks
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// Result type for serialization operations
pub type SerializationResult<T> = Result<T, SerializationError>;

/// Serialize to file
#[cfg(feature = "serde")]
pub fn serialize_to_file<T: serde::Serialize>(
    value: &T,
    path: &std::path::Path,
    format: SerializationFormat,
) -> SerializationResult<()> {
    use std::io::Write;

    let data = match format {
        SerializationFormat::Json => serde_json::to_vec_pretty(value)?,
        SerializationFormat::Bson => {
            return Err(SerializationError::UnsupportedFormat("BSON".to_string()))
        }
        SerializationFormat::MessagePack => {
            return Err(SerializationError::UnsupportedFormat(
                "MessagePack".to_string(),
            ))
        }
    };

    let mut file = std::fs::File::create(path)?;
    file.write_all(&data)?;
    Ok(())
}

/// Deserialize from file
#[cfg(feature = "serde")]
pub fn deserialize_from_file<T: for<'de> serde::Deserialize<'de>>(
    path: &std::path::Path,
    format: SerializationFormat,
) -> SerializationResult<T> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    match format {
        SerializationFormat::Json => Ok(serde_json::from_slice(&data)?),
        SerializationFormat::Bson => Err(SerializationError::UnsupportedFormat("BSON".to_string())),
        SerializationFormat::MessagePack => Err(SerializationError::UnsupportedFormat(
            "MessagePack".to_string(),
        )),
    }
}

/// Serialize to string with specified format
#[cfg(feature = "serde")]
pub fn serialize_to_string<T: serde::Serialize>(
    value: &T,
    format: SerializationFormat,
) -> SerializationResult<String> {
    match format {
        SerializationFormat::Json => Ok(serde_json::to_string_pretty(value)?),
        SerializationFormat::Bson => Err(SerializationError::UnsupportedFormat("BSON".to_string())),
        SerializationFormat::MessagePack => Err(SerializationError::UnsupportedFormat(
            "MessagePack".to_string(),
        )),
    }
}

/// Deserialize from string with specified format
#[cfg(feature = "serde")]
pub fn deserialize_from_string<T: for<'de> serde::Deserialize<'de>>(
    data: &str,
    format: SerializationFormat,
) -> SerializationResult<T> {
    match format {
        SerializationFormat::Json => Ok(serde_json::from_str(data)?),
        SerializationFormat::Bson => Err(SerializationError::UnsupportedFormat("BSON".to_string())),
        SerializationFormat::MessagePack => Err(SerializationError::UnsupportedFormat(
            "MessagePack".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "serde")]
    fn test_json_serialization() {
        let point = crate::geometry::Point::new(1.0, 2.0, 3.0);
        let json = to_json(&point).unwrap();
        let deserialized: crate::geometry::Point = from_json(&json).unwrap();

        assert!((deserialized.x() - 1.0).abs() < 1e-10);
        assert!((deserialized.y() - 2.0).abs() < 1e-10);
        assert!((deserialized.z() - 3.0).abs() < 1e-10);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialized_data_wrapper() {
        let point = crate::geometry::Point::new(1.0, 2.0, 3.0);
        let wrapped = SerializedData::new(point);

        let json = to_json(&wrapped).unwrap();
        let deserialized: SerializedData<crate::geometry::Point> = from_json(&json).unwrap();

        assert!((deserialized.data.x() - 1.0).abs() < 1e-10);
        assert_eq!(deserialized.metadata.format_version, 1);
    }
}
