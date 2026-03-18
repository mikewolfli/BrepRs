//! MessagePack Serialization Support
//!
//! This module provides MessagePack serialization/deserialization support.
//! MessagePack is an efficient binary serialization format that is more compact
//! than JSON and faster to parse.

use serde::{Deserialize, Serialize};

/// MessagePack serialization options
#[derive(Debug, Clone)]
pub struct MsgPackOptions {
    pub use_compact_format: bool,
    pub serialize_enums_as_ints: bool,
}

impl Default for MsgPackOptions {
    fn default() -> Self {
        Self {
            use_compact_format: true,
            serialize_enums_as_ints: true,
        }
    }
}

/// Serialize to MessagePack bytes
pub fn to_msgpack<T: Serialize>(_value: &T) -> Result<Vec<u8>, MsgPackError> {
    // Real implementation using rmp-serde
    rmp_serde::to_vec(_value).map_err(|e| MsgPackError::EncodingError(e.to_string()))
}

/// Deserialize from MessagePack bytes
pub fn from_msgpack<T: for<'de> Deserialize<'de>>(_bytes: &[u8]) -> Result<T, MsgPackError> {
    // Real implementation using rmp-serde
    rmp_serde::from_slice(_bytes).map_err(|e| MsgPackError::DecodingError(e.to_string()))
}

/// Serialize to MessagePack with custom options
pub fn to_msgpack_with_options<T: Serialize>(
    _value: &T,
    _options: &MsgPackOptions,
) -> Result<Vec<u8>, MsgPackError> {
    // Streaming and compact encoding
    use rmp_serde::encode::Serializer;
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);
    _value.serialize(&mut serializer).map_err(|e| MsgPackError::EncodingError(e.to_string()))?;
    Ok(buf)
}

/// Stream MessagePack serialization
pub fn stream_msgpack<T: Serialize, W: std::io::Write>(value: &T, writer: W) -> Result<(), MsgPackError> {
    use rmp_serde::encode::Serializer;
    let mut serializer = Serializer::new(writer);
    value.serialize(&mut serializer).map_err(|e| MsgPackError::EncodingError(e.to_string()))
}

/// MessagePack error types
#[derive(Debug, thiserror::Error)]
pub enum MsgPackError {
    #[error("MessagePack encoding error: {0}")]
    EncodingError(String),

    #[error("MessagePack decoding error: {0}")]
    DecodingError(String),

    #[error("Invalid MessagePack data: {0}")]
    InvalidData(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// MessagePack format types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgPackFormat {
    PositiveFixInt(u8),
    FixMap(u8),
    FixArray(u8),
    FixStr(u8),
    Null,
    False,
    True,
    Bin8,
    Bin16,
    Bin32,
    Ext8,
    Ext16,
    Ext32,
    Float32,
    Float64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    FixExt1,
    FixExt2,
    FixExt4,
    FixExt8,
    FixExt16,
    Str8,
    Str16,
    Str32,
    Array16,
    Array32,
    Map16,
    Map32,
    NegativeFixInt(i8),
}

impl MsgPackFormat {
    /// Parse a format byte
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            0x00..=0x7f => MsgPackFormat::PositiveFixInt(byte),
            0x80..=0x8f => MsgPackFormat::FixMap(byte & 0x0f),
            0x90..=0x9f => MsgPackFormat::FixArray(byte & 0x0f),
            0xa0..=0xbf => MsgPackFormat::FixStr(byte & 0x1f),
            0xc0 => MsgPackFormat::Null,
            0xc1 => MsgPackFormat::Null,
            0xc2 => MsgPackFormat::False,
            0xc3 => MsgPackFormat::True,
            0xc4 => MsgPackFormat::Bin8,
            0xc5 => MsgPackFormat::Bin16,
            0xc6 => MsgPackFormat::Bin32,
            0xc7 => MsgPackFormat::Ext8,
            0xc8 => MsgPackFormat::Ext16,
            0xc9 => MsgPackFormat::Ext32,
            0xca => MsgPackFormat::Float32,
            0xcb => MsgPackFormat::Float64,
            0xcc => MsgPackFormat::UInt8,
            0xcd => MsgPackFormat::UInt16,
            0xce => MsgPackFormat::UInt32,
            0xcf => MsgPackFormat::UInt64,
            0xd0 => MsgPackFormat::Int8,
            0xd1 => MsgPackFormat::Int16,
            0xd2 => MsgPackFormat::Int32,
            0xd3 => MsgPackFormat::Int64,
            0xd4 => MsgPackFormat::FixExt1,
            0xd5 => MsgPackFormat::FixExt2,
            0xd6 => MsgPackFormat::FixExt4,
            0xd7 => MsgPackFormat::FixExt8,
            0xd8 => MsgPackFormat::FixExt16,
            0xd9 => MsgPackFormat::Str8,
            0xda => MsgPackFormat::Str16,
            0xdb => MsgPackFormat::Str32,
            0xdc => MsgPackFormat::Array16,
            0xdd => MsgPackFormat::Array32,
            0xde => MsgPackFormat::Map16,
            0xdf => MsgPackFormat::Map32,
            0xe0..=0xff => MsgPackFormat::NegativeFixInt((byte as i16 - 256) as i8),
        }
    }

    /// Get the byte representation
    pub fn to_byte(&self) -> u8 {
        match *self {
            MsgPackFormat::PositiveFixInt(v) => v,
            MsgPackFormat::FixMap(v) => 0x80 | v,
            MsgPackFormat::FixArray(v) => 0x90 | v,
            MsgPackFormat::FixStr(v) => 0xa0 | v,
            MsgPackFormat::Null => 0xc0,
            MsgPackFormat::False => 0xc2,
            MsgPackFormat::True => 0xc3,
            MsgPackFormat::Bin8 => 0xc4,
            MsgPackFormat::Bin16 => 0xc5,
            MsgPackFormat::Bin32 => 0xc6,
            MsgPackFormat::Ext8 => 0xc7,
            MsgPackFormat::Ext16 => 0xc8,
            MsgPackFormat::Ext32 => 0xc9,
            MsgPackFormat::Float32 => 0xca,
            MsgPackFormat::Float64 => 0xcb,
            MsgPackFormat::UInt8 => 0xcc,
            MsgPackFormat::UInt16 => 0xcd,
            MsgPackFormat::UInt32 => 0xce,
            MsgPackFormat::UInt64 => 0xcf,
            MsgPackFormat::Int8 => 0xd0,
            MsgPackFormat::Int16 => 0xd1,
            MsgPackFormat::Int32 => 0xd2,
            MsgPackFormat::Int64 => 0xd3,
            MsgPackFormat::FixExt1 => 0xd4,
            MsgPackFormat::FixExt2 => 0xd5,
            MsgPackFormat::FixExt4 => 0xd6,
            MsgPackFormat::FixExt8 => 0xd7,
            MsgPackFormat::FixExt16 => 0xd8,
            MsgPackFormat::Str8 => 0xd9,
            MsgPackFormat::Str16 => 0xda,
            MsgPackFormat::Str32 => 0xdb,
            MsgPackFormat::Array16 => 0xdc,
            MsgPackFormat::Array32 => 0xdd,
            MsgPackFormat::Map16 => 0xde,
            MsgPackFormat::Map32 => 0xdf,
            MsgPackFormat::NegativeFixInt(v) => v as u8,
        }
    }
}

/// MessagePack writer for streaming serialization
pub struct MsgPackWriter<W: std::io::Write> {
    writer: W,
}

impl<W: std::io::Write> MsgPackWriter<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn write_nil(&mut self) -> std::io::Result<()> {
        self.writer.write_all(&[0xc0])
    }

    pub fn write_bool(&mut self, value: bool) -> std::io::Result<()> {
        self.writer.write_all(&[if value { 0xc3 } else { 0xc2 }])
    }

    pub fn write_u64(&mut self, value: u64) -> std::io::Result<()> {
        if value <= 0x7f {
            self.writer.write_all(&[value as u8])
        } else if value <= 0xff {
            self.writer.write_all(&[0xcc, value as u8])
        } else if value <= 0xffff {
            self.writer
                .write_all(&[0xcd, (value >> 8) as u8, value as u8])
        } else if value <= 0xffffffff {
            self.writer.write_all(&[
                0xce,
                (value >> 24) as u8,
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ])
        } else {
            self.writer.write_all(&[
                0xcf,
                (value >> 56) as u8,
                (value >> 48) as u8,
                (value >> 40) as u8,
                (value >> 32) as u8,
                (value >> 24) as u8,
                (value >> 16) as u8,
                (value >> 8) as u8,
                value as u8,
            ])
        }
    }

    pub fn write_f64(&mut self, value: f64) -> std::io::Result<()> {
        let bytes = value.to_be_bytes();
        self.writer.write_all(&[0xcb])?;
        self.writer.write_all(&bytes)
    }

    pub fn write_str(&mut self, s: &str) -> std::io::Result<()> {
        let len = s.len();
        if len <= 31 {
            self.writer.write_all(&[0xa0 | len as u8])?;
        } else if len <= 0xff {
            self.writer.write_all(&[0xd9, len as u8])?;
        } else if len <= 0xffff {
            self.writer
                .write_all(&[0xda, (len >> 8) as u8, len as u8])?;
        } else {
            self.writer.write_all(&[
                0xdb,
                (len >> 24) as u8,
                (len >> 16) as u8,
                (len >> 8) as u8,
                len as u8,
            ])?;
        }
        self.writer.write_all(s.as_bytes())
    }

    pub fn write_array_len(&mut self, len: u32) -> std::io::Result<()> {
        if len <= 15 {
            self.writer.write_all(&[0x90 | len as u8])
        } else if len <= 0xffff {
            self.writer.write_all(&[0xdc, (len >> 8) as u8, len as u8])
        } else {
            self.writer.write_all(&[
                0xdd,
                (len >> 24) as u8,
                (len >> 16) as u8,
                (len >> 8) as u8,
                len as u8,
            ])
        }
    }

    pub fn write_map_len(&mut self, len: u32) -> std::io::Result<()> {
        if len <= 15 {
            self.writer.write_all(&[0x80 | len as u8])
        } else if len <= 0xffff {
            self.writer.write_all(&[0xde, (len >> 8) as u8, len as u8])
        } else {
            self.writer.write_all(&[
                0xdf,
                (len >> 24) as u8,
                (len >> 16) as u8,
                (len >> 8) as u8,
                len as u8,
            ])
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

/// MessagePack reader for streaming deserialization
pub struct MsgPackReader<R: std::io::Read> {
    reader: R,
}

impl<R: std::io::Read> MsgPackReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn read_format(&mut self) -> std::io::Result<MsgPackFormat> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(MsgPackFormat::from_byte(buf[0]))
    }

    pub fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_u16(&mut self) -> std::io::Result<u16> {
        let mut buf = [0u8; 2];
        self.reader.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    pub fn read_u32(&mut self) -> std::io::Result<u32> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    pub fn read_u64(&mut self) -> std::io::Result<u64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    pub fn read_f64(&mut self) -> std::io::Result<f64> {
        let mut buf = [0u8; 8];
        self.reader.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msgpack_options_default() {
        let options = MsgPackOptions::default();
        assert!(options.use_compact_format);
        assert!(options.serialize_enums_as_ints);
    }

    #[test]
    fn test_msgpack_format_from_byte() {
        assert!(matches!(
            MsgPackFormat::from_byte(0x00),
            MsgPackFormat::PositiveFixInt(0)
        ));
        assert!(matches!(
            MsgPackFormat::from_byte(0x7f),
            MsgPackFormat::PositiveFixInt(127)
        ));
        assert!(matches!(
            MsgPackFormat::from_byte(0xc0),
            MsgPackFormat::Null
        ));
        assert!(matches!(
            MsgPackFormat::from_byte(0xc2),
            MsgPackFormat::False
        ));
        assert!(matches!(
            MsgPackFormat::from_byte(0xc3),
            MsgPackFormat::True
        ));
        assert!(matches!(
            MsgPackFormat::from_byte(0xcb),
            MsgPackFormat::Float64
        ));
    }

    #[test]
    fn test_msgpack_writer() {
        let mut buf = Vec::new();
        {
            let mut writer = MsgPackWriter::new(&mut buf);
            writer.write_nil().unwrap();
            writer.write_bool(true).unwrap();
            writer.write_u64(42).unwrap();
            writer.write_f64(3.14).unwrap();
            writer.write_str("hello").unwrap();
        }

        assert!(!buf.is_empty());
    }
}
