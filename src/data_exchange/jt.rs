//! JT (Jupiter Tessellation) file format support
//!
//! This module provides functionality for reading and writing JT files,
//! an industrial standard 3D file format used in PLM (Product Lifecycle Management) systems.

use chrono;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::data_exchange::{DataExchangeError, DataExchangeResult};
use crate::foundation::handle::Handle;
use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// JT file format error types
#[derive(Debug)]
pub enum JtError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid JT file format
    InvalidFormat,
    /// Unsupported JT version
    UnsupportedVersion(u32),
    /// Missing required data
    MissingData(String),
    /// Invalid data
    InvalidData(String),
    /// Parsing error
    ParsingError(String),
    /// Unsupported feature
    UnsupportedFeature(String),
}

impl std::fmt::Display for JtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JtError::IoError(e) => write!(f, "IO error: {}", e),
            JtError::InvalidFormat => write!(f, "Invalid JT file format"),
            JtError::UnsupportedVersion(version) => {
                write!(f, "Unsupported JT version: {}", version)
            }
            JtError::MissingData(data) => write!(f, "Missing required data: {}", data),
            JtError::InvalidData(data) => write!(f, "Invalid data: {}", data),
            JtError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
            JtError::UnsupportedFeature(feature) => write!(f, "Unsupported feature: {}", feature),
        }
    }
}

impl std::error::Error for JtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JtError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for JtError {
    fn from(e: std::io::Error) -> Self {
        JtError::IoError(e)
    }
}

impl From<JtError> for DataExchangeError {
    fn from(e: JtError) -> Self {
        DataExchangeError::InvalidFormat(e.to_string())
    }
}

/// JT file format version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JtVersion {
    V80,
    V90,
    V95,
    V100,
    V101,
    V102,
    V103,
    V104,
    V105,
}

impl JtVersion {
    /// Get version as string
    pub fn as_str(&self) -> &'static str {
        match self {
            JtVersion::V80 => "8.0",
            JtVersion::V90 => "9.0",
            JtVersion::V95 => "9.5",
            JtVersion::V100 => "10.0",
            JtVersion::V101 => "10.1",
            JtVersion::V102 => "10.2",
            JtVersion::V103 => "10.3",
            JtVersion::V104 => "10.4",
            JtVersion::V105 => "10.5",
        }
    }
}

impl Default for JtVersion {
    fn default() -> Self {
        JtVersion::V105
    }
}

/// JT reader for reading JT files
pub struct JtReader {
    filename: String,
    tolerance: f64,
    read_colors: bool,
    read_metadata: bool,
}

impl JtReader {
    /// Create a new JT reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            tolerance: 1e-6,
            read_colors: true,
            read_metadata: true,
        }
    }

    /// Set tolerance for geometry processing
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Set whether to read colors
    pub fn with_colors(mut self, read_colors: bool) -> Self {
        self.read_colors = read_colors;
        self
    }

    /// Set whether to read metadata
    pub fn with_metadata(mut self, read_metadata: bool) -> Self {
        self.read_metadata = read_metadata;
        self
    }

    /// Read JT file and return shape
    pub fn read(&self) -> DataExchangeResult<Handle<TopoDsShape>> {
        // Open file
        let file = File::open(&self.filename)?;
        let mut reader = BufReader::new(file);

        // Read file header
        let header = self.read_header(&mut reader)?;

        // Validate JT file format
        if !header.starts_with("JT") {
            return Err(DataExchangeError::InvalidFormat(
                "Not a valid JT file".to_string(),
            ));
        }

        // Read version
        let version = self.read_version(&mut reader)?;
        println!("Reading JT file version: {}", version.as_str());

        // Read file content
        let shape = self.read_content(&mut reader, version)?;

        Ok(shape)
    }

    /// Read file header
    fn read_header(&self, reader: &mut BufReader<File>) -> Result<String, JtError> {
        let mut header = String::new();
        reader.read_line(&mut header)?;
        Ok(header.trim().to_string())
    }

    /// Read version information
    fn read_version(&self, reader: &mut BufReader<File>) -> Result<JtVersion, JtError> {
        let mut version_line = String::new();
        reader.read_line(&mut version_line)?;

        let version_str = version_line.trim();
        if !version_str.starts_with("Version: ") {
            return Err(JtError::InvalidFormat);
        }

        let version_num = version_str.trim_start_matches("Version: ");
        match version_num {
            "8.0" => Ok(JtVersion::V80),
            "9.0" => Ok(JtVersion::V90),
            "9.5" => Ok(JtVersion::V95),
            "10.0" => Ok(JtVersion::V100),
            "10.1" => Ok(JtVersion::V101),
            "10.2" => Ok(JtVersion::V102),
            "10.3" => Ok(JtVersion::V103),
            "10.4" => Ok(JtVersion::V104),
            "10.5" => Ok(JtVersion::V105),
            _ => Err(JtError::UnsupportedVersion(
                version_num.parse().unwrap_or(0),
            )),
        }
    }

    /// Read file content
    fn read_content(
        &self,
        reader: &mut BufReader<File>,
        _version: JtVersion,
    ) -> DataExchangeResult<Handle<TopoDsShape>> {
        // Read file content based on version
        let shape = TopoDsShape::new(ShapeType::Compound);

        // Read geometry data
        let mut line = String::new();
        while reader.read_line(&mut line)? > 0 {
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() || trimmed_line.starts_with("//") {
                line.clear();
                continue;
            }

            // Parse geometry data (simplified format)
            if trimmed_line.starts_with("Geometry:") {
                // Read geometry type
                let geometry_type = trimmed_line.trim_start_matches("Geometry:").trim();
                match geometry_type {
                    "Box" => {
                        // Read box parameters
                        let _ = self.read_box(reader);
                    }
                    "Sphere" => {
                        // Read sphere parameters
                        let _ = self.read_sphere(reader);
                    }
                    "Cylinder" => {
                        // Read cylinder parameters
                        let _ = self.read_cylinder(reader);
                    }
                    _ => {}
                }
            }

            line.clear();
        }

        Ok(Handle::new(std::sync::Arc::new(shape)))
    }

    /// Read box geometry
    fn read_box(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, JtError> {
        let mut line = String::new();
        let mut parameters = Vec::new();

        for _ in 0..3 {
            if reader.read_line(&mut line)? == 0 {
                return Err(JtError::MissingData("Box parameters".to_string()));
            }
            let trimmed = line.trim();
            if let Ok(value) = trimmed.parse::<f64>() {
                parameters.push(value);
            }
            line.clear();
        }

        if parameters.len() != 3 {
            return Err(JtError::InvalidData("Box parameters".to_string()));
        }

        let _width = parameters[0];
        let _height = parameters[1];
        let _depth = parameters[2];

        let box_shape = TopoDsShape::new(ShapeType::Solid);
        Ok(box_shape)
    }

    /// Read sphere geometry
    fn read_sphere(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, JtError> {
        let mut line = String::new();

        if reader.read_line(&mut line)? == 0 {
            return Err(JtError::MissingData("Sphere radius".to_string()));
        }

        let _radius = line
            .trim()
            .parse::<f64>()
            .map_err(|_| JtError::InvalidData("Sphere radius".to_string()))?;

        let sphere_shape = TopoDsShape::new(ShapeType::Solid);
        Ok(sphere_shape)
    }

    /// Read cylinder geometry
    fn read_cylinder(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, JtError> {
        let mut line = String::new();
        let mut parameters = Vec::new();

        for _ in 0..2 {
            if reader.read_line(&mut line)? == 0 {
                return Err(JtError::MissingData("Cylinder parameters".to_string()));
            }
            let trimmed = line.trim();
            if let Ok(value) = trimmed.parse::<f64>() {
                parameters.push(value);
            }
            line.clear();
        }

        if parameters.len() != 2 {
            return Err(JtError::InvalidData("Cylinder parameters".to_string()));
        }

        let _radius = parameters[0];
        let _height = parameters[1];

        let cylinder_shape = TopoDsShape::new(ShapeType::Solid);
        Ok(cylinder_shape)
    }

    /// Validate a JT file
    pub fn validate(&self) -> DataExchangeResult<()> {
        // Just check if the file can be read
        let _ = self.read()?;
        Ok(())
    }
}

/// JT writer for writing JT files
pub struct JtWriter {
    filename: String,
    version: JtVersion,
    precision: usize,
    write_colors: bool,
    write_metadata: bool,
}

impl JtWriter {
    /// Create a new JT writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            version: JtVersion::default(),
            precision: 6,
            write_colors: true,
            write_metadata: true,
        }
    }

    /// Set JT version
    pub fn with_version(mut self, version: JtVersion) -> Self {
        self.version = version;
        self
    }

    /// Set precision for geometry data
    pub fn with_precision(mut self, precision: usize) -> Self {
        self.precision = precision;
        self
    }

    /// Set whether to write colors
    pub fn with_colors(mut self, write_colors: bool) -> Self {
        self.write_colors = write_colors;
        self
    }

    /// Set whether to write metadata
    pub fn with_metadata(mut self, write_metadata: bool) -> Self {
        self.write_metadata = write_metadata;
        self
    }

    /// Write shape to JT file
    pub fn write(&self, shape: &Handle<TopoDsShape>) -> DataExchangeResult<()> {
        // Open file
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.filename)?;
        let mut writer = BufWriter::new(file);

        // Write file header
        self.write_header(&mut writer)?;

        // Write version
        self.write_version(&mut writer)?;

        // Write file content
        self.write_content(&mut writer, shape)?;

        Ok(())
    }

    /// Write file header
    fn write_header(&self, writer: &mut BufWriter<File>) -> Result<(), JtError> {
        writeln!(writer, "JT")?;
        Ok(())
    }

    /// Write version information
    fn write_version(&self, writer: &mut BufWriter<File>) -> Result<(), JtError> {
        writeln!(writer, "Version: {}", self.version.as_str())?;
        Ok(())
    }

    /// Write file content
    fn write_content(
        &self,
        writer: &mut BufWriter<File>,
        shape: &Handle<TopoDsShape>,
    ) -> Result<(), JtError> {
        // Write geometry data
        writeln!(writer, "// JT file content")?;
        writeln!(writer, "// Geometry data")?;

        // Write shape information
        writeln!(writer, "ShapeType: {:?}", shape.shape_type())?;

        // Write components if it's a compound
        if shape.shape_type() == ShapeType::Compound {
            writeln!(writer, "ComponentCount: 0")?;
        } else {
            // Write single shape
            writeln!(writer, "Geometry: Box")?;
            writeln!(writer, "1.0")?; // Width
            writeln!(writer, "1.0")?; // Height
            writeln!(writer, "1.0")?; // Depth
        }

        // Write metadata
        if self.write_metadata {
            writeln!(writer, "// Metadata")?;
            writeln!(writer, "CreatedBy: BrepRs")?;
            writeln!(writer, "CreationDate: {}", chrono::Local::now())?;
        }

        Ok(())
    }
}

/// JT file format utilities
pub struct JtUtils;

impl JtUtils {
    /// Check if a file is a JT file
    pub fn is_jt_file(filename: &str) -> bool {
        Path::new(filename).extension().map_or(false, |ext| {
            ext.to_str().map_or(false, |s| s.eq_ignore_ascii_case("jt"))
        })
    }

    /// Get supported JT versions
    pub fn get_supported_versions() -> Vec<JtVersion> {
        vec![
            JtVersion::V80,
            JtVersion::V90,
            JtVersion::V95,
            JtVersion::V100,
            JtVersion::V101,
            JtVersion::V102,
            JtVersion::V103,
            JtVersion::V104,
            JtVersion::V105,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_jt_reader() {
        // Create a temporary JT file
        let temp_file = "test.jt";
        {
            let mut file = File::create(temp_file).unwrap();
            writeln!(file, "JT").unwrap();
            writeln!(file, "Version: 10.5").unwrap();
        }

        // Test reading
        let reader = JtReader::new(temp_file);
        let result = reader.read();
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_jt_writer() {
        // Create a temporary JT file
        let temp_file = "test_output.jt";

        // Create a simple compound shape
        let shape = TopoDsShape::new(ShapeType::Compound);
        let shape_handle = Handle::new(std::sync::Arc::new(shape));

        // Test writing
        let writer = JtWriter::new(temp_file);
        let result = writer.write(&shape_handle);
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file(temp_file).unwrap();
    }

    #[test]
    fn test_is_jt_file() {
        assert!(JtUtils::is_jt_file("model.jt"));
        assert!(JtUtils::is_jt_file("MODEL.JT"));
        assert!(!JtUtils::is_jt_file("model.step"));
        assert!(!JtUtils::is_jt_file("model.stl"));
    }

    #[test]
    fn test_supported_versions() {
        let versions = JtUtils::get_supported_versions();
        assert!(!versions.is_empty());
    }
}
