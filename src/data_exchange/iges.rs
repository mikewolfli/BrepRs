use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// IGES file format error types
#[derive(Debug)]
pub enum IgesError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid IGES file format
    InvalidFormat,
    /// Invalid IGES entity
    InvalidEntity,
    /// Unsupported IGES entity type
    UnsupportedEntityType,
    /// Parsing error
    ParsingError,
    /// Not implemented
    NotImplemented,
}

impl std::fmt::Display for IgesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IgesError::IoError(e) => write!(f, "IO error: {}", e),
            IgesError::InvalidFormat => write!(f, "Invalid IGES file format"),
            IgesError::InvalidEntity => write!(f, "Invalid IGES entity"),
            IgesError::UnsupportedEntityType => write!(f, "Unsupported IGES entity type"),
            IgesError::ParsingError => write!(f, "Parsing error"),
            IgesError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl From<std::io::Error> for IgesError {
    fn from(err: std::io::Error) -> Self {
        IgesError::IoError(err)
    }
}

/// IGES entity type codes
#[derive(Copy, Clone, Debug)]
pub enum IgesEntityType {
    /// Point
    Point = 116,
    /// Line
    Line = 110,
    /// Circle
    Circle = 100,
    /// Circular Arc
    CircularArc = 104,
    /// Ellipse
    Ellipse = 106,
    /// Elliptical Arc
    EllipticalArc = 108,
    /// Spline Curve
    SplineCurve = 126,
    /// Composite Curve
    CompositeCurve = 102,
    /// Plane
    Plane = 109,
    /// Cylinder
    Cylinder = 118,
    /// Cone
    Cone = 120,
    /// Sphere
    Sphere = 112,
    /// Torus
    Torus = 128,
    /// Surface of Revolution
    SurfaceOfRevolution = 122,
    /// Tabulated Cylinder
    TabulatedCylinder = 124,
    /// B-spline Surface
    BSplineSurface = 129,
    /// Trimmed Surface
    TrimmedSurface = 144,
    /// Face
    Face = 142,
    /// Edge
    Edge = 103,
    /// Vertex
    Vertex = 117,
    /// Loop
    Loop = 105,
    /// Shell
    Shell = 143,
    /// Solid
    Solid = 190,
    /// Assembly
    Assembly = 184,
    /// Instance
    Instance = 402,
}

/// IGES reader for reading IGES files
pub struct IgesReader {
    filename: String,
    tolerance: f64,
    read_colors: bool,
}

impl IgesReader {
    /// Create a new IGES reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            tolerance: 1e-6,
            read_colors: false,
        }
    }

    /// Set the tolerance for geometry operations
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set whether to read colors
    pub fn set_read_colors(&mut self, read_colors: bool) {
        self.read_colors = read_colors;
    }

    /// Get whether colors are read
    pub fn read_colors(&self) -> bool {
        self.read_colors
    }

    /// Read an IGES file and return a shape
    pub fn read(&self) -> Result<TopoDsShape, IgesError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read and parse the IGES file
        self.parse_iges_file(&mut reader)
    }

    /// Parse an IGES file
    fn parse_iges_file(&self, reader: &mut BufReader<File>) -> Result<TopoDsShape, IgesError> {
        // Read the header section
        let _header = self.read_header(reader)?;

        // Read the directory section
        let directory = self.read_directory_section(reader)?;

        // Read the parameter section
        let parameters = self.read_parameter_section(reader)?;

        // Read the trailer section
        self.read_trailer_section(reader)?;

        // Create shape from parsed data
        let shape = self.create_shape_from_data(&directory, &parameters)?;

        Ok(shape)
    }

    /// Read the IGES file header section
    fn read_header(&self, reader: &mut BufReader<File>) -> Result<String, IgesError> {
        let mut header = String::new();
        let mut line = String::new();

        // Read the header section (first 80 lines)
        for _ in 0..80 {
            if reader.read_line(&mut line)? == 0 {
                break;
            }
            header.push_str(&line);
            line.clear();
        }

        Ok(header)
    }

    /// Read the IGES file directory section
    fn read_directory_section(
        &self,
        reader: &mut BufReader<File>,
    ) -> Result<Vec<String>, IgesError> {
        let mut directory = Vec::new();
        let mut line = String::new();

        // Read directory entries until we reach the parameter section
        while reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                line.clear();
                continue;
            }

            // Check for the start of the parameter section
            if trimmed.starts_with("S") {
                break;
            }

            directory.push(line.clone());
            line.clear();
        }

        Ok(directory)
    }

    /// Read the IGES file parameter section
    fn read_parameter_section(
        &self,
        reader: &mut BufReader<File>,
    ) -> Result<Vec<String>, IgesError> {
        let mut parameters = Vec::new();
        let mut line = String::new();

        // Read parameter entries until we reach the trailer section
        while reader.read_line(&mut line)? > 0 {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                line.clear();
                continue;
            }

            // Check for the start of the trailer section
            if trimmed.starts_with("T") {
                break;
            }

            parameters.push(line.clone());
            line.clear();
        }

        Ok(parameters)
    }

    /// Read the IGES file trailer section
    fn read_trailer_section(&self, reader: &mut BufReader<File>) -> Result<(), IgesError> {
        let mut line = String::new();

        // Read the trailer section
        while reader.read_line(&mut line)? > 0 {
            line.clear();
        }

        Ok(())
    }

    /// Create a shape from parsed IGES data
    fn create_shape_from_data(
        &self,
        _directory: &[String],
        _parameters: &[String],
    ) -> Result<TopoDsShape, IgesError> {
        // This is a placeholder implementation
        // In a real implementation, we would create shapes from the parsed data
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }

    /// Validate an IGES file
    pub fn validate(&self) -> Result<(), IgesError> {
        // Just check if the file can be read
        let _ = self.read()?;
        Ok(())
    }
}

/// IGES writer for writing IGES files
pub struct IgesWriter {
    filename: String,
    precision: usize,
    write_colors: bool,
}

impl IgesWriter {
    /// Create a new IGES writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            precision: 6,
            write_colors: false,
        }
    }

    /// Set the precision for numeric values
    pub fn set_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    /// Get the precision
    pub fn precision(&self) -> usize {
        self.precision
    }

    /// Set whether to write colors
    pub fn set_write_colors(&mut self, write_colors: bool) {
        self.write_colors = write_colors;
    }

    /// Get whether colors are written
    pub fn write_colors(&self) -> bool {
        self.write_colors
    }

    /// Write a shape to an IGES file
    pub fn write(&self, shape: &TopoDsShape) -> Result<(), IgesError> {
        let path = Path::new(&self.filename);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);

        // Write the header section
        self.write_header(&mut writer)?;

        // Write the directory section
        self.write_directory_section(&mut writer, shape)?;

        // Write the parameter section
        self.write_parameter_section(&mut writer, shape)?;

        // Write the trailer section
        self.write_trailer_section(&mut writer)?;

        Ok(())
    }

    /// Write the IGES file header section
    fn write_header(&self, writer: &mut BufWriter<File>) -> Result<(), IgesError> {
        // Write header lines
        writeln!(writer, "{:80}", "BrepRs IGES Export")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;
        writeln!(writer, "{:80}", "")?;

        // Write more header lines (total 80 lines)
        for _ in 8..80 {
            writeln!(writer, "{:80}", "")?;
        }

        Ok(())
    }

    /// Write the IGES file directory section
    fn write_directory_section(
        &self,
        writer: &mut BufWriter<File>,
        shape: &TopoDsShape,
    ) -> Result<(), IgesError> {
        let shape_type = shape.shape_type();
        let entity_id = 1;

        match shape_type {
            ShapeType::Solid => {
                self.write_solid_directory(writer, entity_id)?;
            }
            ShapeType::Face => {
                self.write_face_directory(writer, entity_id)?;
            }
            ShapeType::Edge => {
                self.write_edge_directory(writer, entity_id)?;
            }
            ShapeType::Wire => {
                self.write_wire_directory(writer, entity_id)?;
            }
            ShapeType::Compound => {
                self.write_compound_directory(writer, entity_id)?;
            }
            _ => {
                return Err(IgesError::NotImplemented);
            }
        }

        Ok(())
    }

    /// Write solid directory entry
    fn write_solid_directory(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        let line = format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}",
            entity_id, 190, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
        writeln!(writer, "{}", line)?;
        Ok(())
    }

    /// Write face directory entry
    fn write_face_directory(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        let line = format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}",
            entity_id, 142, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
        writeln!(writer, "{}", line)?;
        Ok(())
    }

    /// Write edge directory entry
    fn write_edge_directory(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        let line = format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}",
            entity_id, 142, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
        writeln!(writer, "{}", line)?;
        Ok(())
    }

    /// Write wire directory entry
    fn write_wire_directory(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        let line = format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}",
            entity_id, 102, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
        writeln!(writer, "{}", line)?;
        Ok(())
    }

    /// Write compound directory entry
    fn write_compound_directory(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        let line = format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}{:<8}",
            entity_id, 184, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
        writeln!(writer, "{}", line)?;
        Ok(())
    }

    /// Write the IGES file parameter section
    fn write_parameter_section(
        &self,
        writer: &mut BufWriter<File>,
        shape: &TopoDsShape,
    ) -> Result<(), IgesError> {
        let shape_type = shape.shape_type();
        let entity_id = 1;

        match shape_type {
            ShapeType::Solid => {
                self.write_solid_parameters(writer, entity_id)?;
            }
            ShapeType::Face => {
                self.write_face_parameters(writer, entity_id)?;
            }
            ShapeType::Edge => {
                self.write_edge_parameters(writer, entity_id)?;
            }
            ShapeType::Wire => {
                self.write_wire_parameters(writer, entity_id)?;
            }
            ShapeType::Compound => {
                self.write_compound_parameters(writer, entity_id)?;
            }
            _ => {
                return Err(IgesError::NotImplemented);
            }
        }

        Ok(())
    }

    /// Write solid parameters
    fn write_solid_parameters(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        writeln!(writer, "{:64},{}H,", entity_id, 1)?;
        writeln!(writer, "{:64},{}H,", entity_id, 2)?;
        writeln!(writer, "{:64},{}H,", entity_id, 3)?;
        writeln!(writer, "{:64},{}H,", entity_id, 4)?;
        writeln!(writer, "{:64},{}H,", entity_id, 5)?;
        writeln!(writer, "{:64},{}H,", entity_id, 6)?;
        writeln!(writer, "{:64},{}H,", entity_id, 7)?;
        writeln!(writer, "{:64},{}H,", entity_id, 8)?;
        writeln!(writer, "{:64},{}H,", entity_id, 9)?;
        writeln!(writer, "{:64},{}H,", entity_id, 10)?;
        writeln!(writer, "{:64},{}H,", entity_id, 11)?;
        writeln!(writer, "{:64},{}H,", entity_id, 12)?;
        writeln!(writer, "{:64},{}H,", entity_id, 13)?;
        writeln!(writer, "{:64},{}H,", entity_id, 14)?;
        writeln!(writer, "{:64},{}H,", entity_id, 15)?;
        writeln!(writer, "{:64},{}H,", entity_id, 16)?;
        writeln!(writer, "{:64},{}H,", entity_id, 17)?;
        writeln!(writer, "{:64},{}H,", entity_id, 18)?;
        writeln!(writer, "{:64},{}H,", entity_id, 19)?;
        writeln!(writer, "{:64},{}H,", entity_id, 20)?;
        Ok(())
    }

    /// Write face parameters
    fn write_face_parameters(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        writeln!(writer, "{:64},{}H,", entity_id, 1)?;
        writeln!(writer, "{:64},{}H,", entity_id, 2)?;
        writeln!(writer, "{:64},{}H,", entity_id, 3)?;
        writeln!(writer, "{:64},{}H,", entity_id, 4)?;
        writeln!(writer, "{:64},{}H,", entity_id, 5)?;
        writeln!(writer, "{:64},{}H,", entity_id, 6)?;
        writeln!(writer, "{:64},{}H,", entity_id, 7)?;
        writeln!(writer, "{:64},{}H,", entity_id, 8)?;
        writeln!(writer, "{:64},{}H,", entity_id, 9)?;
        writeln!(writer, "{:64},{}H,", entity_id, 10)?;
        writeln!(writer, "{:64},{}H,", entity_id, 11)?;
        writeln!(writer, "{:64},{}H,", entity_id, 12)?;
        writeln!(writer, "{:64},{}H,", entity_id, 13)?;
        writeln!(writer, "{:64},{}H,", entity_id, 14)?;
        writeln!(writer, "{:64},{}H,", entity_id, 15)?;
        writeln!(writer, "{:64},{}H,", entity_id, 16)?;
        writeln!(writer, "{:64},{}H,", entity_id, 17)?;
        writeln!(writer, "{:64},{}H,", entity_id, 18)?;
        writeln!(writer, "{:64},{}H,", entity_id, 19)?;
        writeln!(writer, "{:64},{}H,", entity_id, 20)?;
        Ok(())
    }

    /// Write edge parameters
    fn write_edge_parameters(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        writeln!(writer, "{:64},{}H,", entity_id, 1)?;
        writeln!(writer, "{:64},{}H,", entity_id, 2)?;
        writeln!(writer, "{:64},{}H,", entity_id, 3)?;
        writeln!(writer, "{:64},{}H,", entity_id, 4)?;
        writeln!(writer, "{:64},{}H,", entity_id, 5)?;
        writeln!(writer, "{:64},{}H,", entity_id, 6)?;
        writeln!(writer, "{:64},{}H,", entity_id, 7)?;
        writeln!(writer, "{:64},{}H,", entity_id, 8)?;
        writeln!(writer, "{:64},{}H,", entity_id, 9)?;
        writeln!(writer, "{:64},{}H,", entity_id, 10)?;
        writeln!(writer, "{:64},{}H,", entity_id, 11)?;
        writeln!(writer, "{:64},{}H,", entity_id, 12)?;
        writeln!(writer, "{:64},{}H,", entity_id, 13)?;
        writeln!(writer, "{:64},{}H,", entity_id, 14)?;
        writeln!(writer, "{:64},{}H,", entity_id, 15)?;
        writeln!(writer, "{:64},{}H,", entity_id, 16)?;
        writeln!(writer, "{:64},{}H,", entity_id, 17)?;
        writeln!(writer, "{:64},{}H,", entity_id, 18)?;
        writeln!(writer, "{:64},{}H,", entity_id, 19)?;
        writeln!(writer, "{:64},{}H,", entity_id, 20)?;
        Ok(())
    }

    /// Write wire parameters
    fn write_wire_parameters(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        writeln!(writer, "{:64},{}H,", entity_id, 1)?;
        writeln!(writer, "{:64},{}H,", entity_id, 2)?;
        writeln!(writer, "{:64},{}H,", entity_id, 3)?;
        writeln!(writer, "{:64},{}H,", entity_id, 4)?;
        writeln!(writer, "{:64},{}H,", entity_id, 5)?;
        writeln!(writer, "{:64},{}H,", entity_id, 6)?;
        writeln!(writer, "{:64},{}H,", entity_id, 7)?;
        writeln!(writer, "{:64},{}H,", entity_id, 8)?;
        writeln!(writer, "{:64},{}H,", entity_id, 9)?;
        writeln!(writer, "{:64},{}H,", entity_id, 10)?;
        writeln!(writer, "{:64},{}H,", entity_id, 11)?;
        writeln!(writer, "{:64},{}H,", entity_id, 12)?;
        writeln!(writer, "{:64},{}H,", entity_id, 13)?;
        writeln!(writer, "{:64},{}H,", entity_id, 14)?;
        writeln!(writer, "{:64},{}H,", entity_id, 15)?;
        writeln!(writer, "{:64},{}H,", entity_id, 16)?;
        writeln!(writer, "{:64},{}H,", entity_id, 17)?;
        writeln!(writer, "{:64},{}H,", entity_id, 18)?;
        writeln!(writer, "{:64},{}H,", entity_id, 19)?;
        writeln!(writer, "{:64},{}H,", entity_id, 20)?;
        Ok(())
    }

    /// Write compound parameters
    fn write_compound_parameters(
        &self,
        writer: &mut BufWriter<File>,
        entity_id: usize,
    ) -> Result<(), IgesError> {
        writeln!(writer, "{:64},{}H,", entity_id, 1)?;
        writeln!(writer, "{:64},{}H,", entity_id, 2)?;
        writeln!(writer, "{:64},{}H,", entity_id, 3)?;
        writeln!(writer, "{:64},{}H,", entity_id, 4)?;
        writeln!(writer, "{:64},{}H,", entity_id, 5)?;
        writeln!(writer, "{:64},{}H,", entity_id, 6)?;
        writeln!(writer, "{:64},{}H,", entity_id, 7)?;
        writeln!(writer, "{:64},{}H,", entity_id, 8)?;
        writeln!(writer, "{:64},{}H,", entity_id, 9)?;
        writeln!(writer, "{:64},{}H,", entity_id, 10)?;
        writeln!(writer, "{:64},{}H,", entity_id, 11)?;
        writeln!(writer, "{:64},{}H,", entity_id, 12)?;
        writeln!(writer, "{:64},{}H,", entity_id, 13)?;
        writeln!(writer, "{:64},{}H,", entity_id, 14)?;
        writeln!(writer, "{:64},{}H,", entity_id, 15)?;
        writeln!(writer, "{:64},{}H,", entity_id, 16)?;
        writeln!(writer, "{:64},{}H,", entity_id, 17)?;
        writeln!(writer, "{:64},{}H,", entity_id, 18)?;
        writeln!(writer, "{:64},{}H,", entity_id, 19)?;
        writeln!(writer, "{:64},{}H,", entity_id, 20)?;
        Ok(())
    }

    /// Write the IGES file trailer section
    fn write_trailer_section(&self, writer: &mut BufWriter<File>) -> Result<(), IgesError> {
        writeln!(writer, "T{:79}", "")?;
        writeln!(writer, "T{:79}", "")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_iges_reader_creation() {
        let reader = IgesReader::new("test.iges");
        assert_eq!(reader.tolerance(), 1e-6);
        assert!(!reader.read_colors());
    }

    #[test]
    fn test_iges_reader_settings() {
        let mut reader = IgesReader::new("test.iges");

        reader.set_tolerance(1e-4);
        assert_eq!(reader.tolerance(), 1e-4);

        reader.set_read_colors(true);
        assert!(reader.read_colors());
    }

    #[test]
    fn test_iges_writer_creation() {
        let writer = IgesWriter::new("test.iges");
        assert_eq!(writer.precision(), 6);
        assert!(!writer.write_colors());
    }

    #[test]
    fn test_iges_writer_settings() {
        let mut writer = IgesWriter::new("test.iges");

        writer.set_precision(10);
        assert_eq!(writer.precision(), 10);

        writer.set_write_colors(true);
        assert!(writer.write_colors());
    }

    #[test]
    fn test_iges_validate() {
        // This is a placeholder test
        // In a real implementation, we would create a test IGES file and validate it
        let reader = IgesReader::new("test.iges");
        let validate_result = reader.validate();
        // The validate operation should fail for a non-existent file
        assert!(validate_result.is_err());
    }

    #[test]
    fn test_iges_read_write_cycle() {
        // This is a placeholder test
        // In a real implementation, we would test reading and writing an IGES file
        let shape = TopoDsShape::new(ShapeType::Compound);

        let writer = IgesWriter::new("test_iges_cycle.iges");
        let write_result = writer.write(&shape);
        assert!(write_result.is_ok());

        let reader = IgesReader::new("test_iges_cycle.iges");
        let read_result = reader.read();

        // Clean up
        if Path::new("test_iges_cycle.iges").exists() {
            let _ = fs::remove_file("test_iges_cycle.iges");
        }

        // The read operation should succeed (even with placeholder implementation)
        assert!(read_result.is_ok());
    }
}
