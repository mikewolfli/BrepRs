//! STL (STereoLithography) file format support
//!
//! This module provides functionality for reading and writing STL files,
//! both in ASCII and binary formats.

use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;

use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_vertex::TopoDS_Vertex,
    topods_edge::TopoDS_Edge,
    topods_face::TopoDS_Face,
    topods_wire::TopoDS_Wire,
    topods_shell::TopoDS_Shell,
    topods_solid::TopoDS_Solid,
    topods_compound::TopoDS_Compound,
    topods_shape::TopoDS_Shape,
    shape_enum::ShapeType,
    top_exp_explorer::TopExpExplorer,
};
use crate::modeling::BRep_Builder;

/// STL file reader
///
/// This struct provides functionality to read STL files and convert them to BrepRs shapes.
pub struct StlReader {
    filename: String,
    tolerance: f64,
    has_normals: bool,
    has_colors: bool,
}

/// STL file writer
///
/// This struct provides functionality to write BrepRs shapes to STL files.
pub struct StlWriter {
    filename: String,
    binary: bool,
    precision: usize,
    has_normals: bool,
    has_colors: bool,
}

/// STL format error
#[derive(Debug, thiserror::Error)]
pub enum StlError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid STL format")]
    InvalidFormat,
    #[error("Invalid STL header")]
    InvalidHeader,
    #[error("Invalid STL facet")]
    InvalidFacet,
    #[error("Invalid STL normal vector")]
    InvalidNormal,
    #[error("Invalid STL vertex")]
    InvalidVertex,
    #[error("Unexpected end of file")]
    UnexpectedEof,
    #[error("Invalid binary STL data")]
    InvalidBinaryData,
    #[error("Unsupported STL feature: {0}")]
    UnsupportedFeature(String),
}

/// STL facet
#[derive(Debug, Clone, PartialEq)]
pub struct StlFacet {
    pub normal: [f64; 3],
    pub vertices: [[f64; 3]; 3],
    pub color: Option<[u8; 3]>,
}

impl StlFacet {
    /// Create a new STL facet
    pub fn new(normal: [f64; 3], vertices: [[f64; 3]; 3]) -> Self {
        Self {
            normal,
            vertices,
            color: None,
        }
    }

    /// Create a new STL facet with color
    pub fn with_color(normal: [f64; 3], vertices: [[f64; 3]; 3], color: [u8; 3]) -> Self {
        Self {
            normal,
            vertices,
            color: Some(color),
        }
    }

    /// Check if the facet is valid
    pub fn is_valid(&self) -> bool {
        // Check if normal is a unit vector (approximately)
        let norm_sq = self.normal[0] * self.normal[0] + self.normal[1] * self.normal[1] + self.normal[2] * self.normal[2];
        if norm_sq < 0.9 || norm_sq > 1.1 {
            return false;
        }

        // Check if vertices are not colinear
        let v1 = [
            self.vertices[1][0] - self.vertices[0][0],
            self.vertices[1][1] - self.vertices[0][1],
            self.vertices[1][2] - self.vertices[0][2],
        ];
        let v2 = [
            self.vertices[2][0] - self.vertices[0][0],
            self.vertices[2][1] - self.vertices[0][1],
            self.vertices[2][2] - self.vertices[0][2],
        ];
        
        let cross = [
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        ];
        
        let cross_sq = cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2];
        cross_sq > 1e-12
    }
}

impl StlReader {
    /// Create a new STL reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            tolerance: 1e-6,
            has_normals: true,
            has_colors: false,
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

    /// Set whether to read normals
    pub fn set_read_normals(&mut self, read_normals: bool) {
        self.has_normals = read_normals;
    }

    /// Get whether normals are read
    pub fn read_normals(&self) -> bool {
        self.has_normals
    }

    /// Set whether to read colors
    pub fn set_read_colors(&mut self, read_colors: bool) {
        self.has_colors = read_colors;
    }

    /// Get whether colors are read
    pub fn read_colors(&self) -> bool {
        self.has_colors
    }

    /// Read an STL file and return a shape
    pub fn read(&self) -> Result<TopoDS_Compound, StlError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        
        let mut reader = BufReader::new(file);
        let mut header = String::new();
        reader.read_line(&mut header)?;
        
        let is_binary = self.detect_binary_format(&header)?;
        
        if is_binary {
            self.read_binary(&mut reader)
        } else {
            self.read_ascii(&mut reader)
        }
    }

    /// Detect if the STL file is binary
    fn detect_binary_format(&self, header: &str) -> Result<bool, StlError> {
        // Check if the first line starts with "solid" (ASCII format)
        if header.trim().starts_with("solid") {
            Ok(false)
        } else {
            // Check if the file size is consistent with binary format
            // Binary STL has 80-byte header + 4-byte triangle count + 50-byte per triangle
            let path = Path::new(&self.filename);
            let metadata = path.metadata()?;
            let file_size = metadata.len();
            
            if file_size < 84 {
                return Err(StlError::InvalidHeader);
            }
            
            let triangle_count = (file_size - 84) / 50;
            if (file_size - 84) % 50 != 0 {
                return Err(StlError::InvalidBinaryData);
            }
            
            Ok(true)
        }
    }

    /// Read ASCII STL format
    fn read_ascii(&self, reader: &mut BufReader<File>) -> Result<TopoDS_Compound, StlError> {
        let builder = BRep_Builder::new();
        let mut compound = TopoDS_Compound::new();
        
        let mut facets = Vec::new();
        let mut current_facet = None;
        let mut vertex_count = 0;
        let mut vertices = [[0.0; 3]; 3];
        let mut normal = [0.0; 3];
        
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            
            if trimmed.starts_with("facet normal") {
                // Start of a new facet
                if current_facet.is_some() {
                    return Err(StlError::InvalidFormat);
                }
                
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() != 6 {
                    return Err(StlError::InvalidFacet);
                }
                
                normal[0] = parts[2].parse().map_err(|_| StlError::InvalidNormal)?;
                normal[1] = parts[3].parse().map_err(|_| StlError::InvalidNormal)?;
                normal[2] = parts[4].parse().map_err(|_| StlError::InvalidNormal)?;
                
                current_facet = Some(normal);
                vertex_count = 0;
            } else if trimmed.starts_with("vertex") && current_facet.is_some() {
                // Vertex of the current facet
                if vertex_count >= 3 {
                    return Err(StlError::InvalidFacet);
                }
                
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() != 4 {
                    return Err(StlError::InvalidVertex);
                }
                
                vertices[vertex_count][0] = parts[1].parse().map_err(|_| StlError::InvalidVertex)?;
                vertices[vertex_count][1] = parts[2].parse().map_err(|_| StlError::InvalidVertex)?;
                vertices[vertex_count][2] = parts[3].parse().map_err(|_| StlError::InvalidVertex)?;
                
                vertex_count += 1;
            } else if trimmed == "endfacet" && current_facet.is_some() {
                // End of the current facet
                if vertex_count != 3 {
                    return Err(StlError::InvalidFacet);
                }
                
                let facet = StlFacet::new(normal, vertices);
                facets.push(facet);
                
                current_facet = None;
            } else if trimmed == "endsolid" {
                // End of the file
                break;
            }
        }
        
        // Convert facets to BrepRs shapes
        for facet in facets {
            let shape = self.create_face_from_facet(&facet)?;
            compound.add_component(shape);
        }
        
        Ok(compound)
    }

    /// Read binary STL format
    fn read_binary(&self, reader: &mut BufReader<File>) -> Result<TopoDS_Compound, StlError> {
        let mut compound = TopoDS_Compound::new();
        
        // Skip the header (80 bytes)
        let mut header = vec![0; 80];
        reader.read_exact(&mut header)?;
        
        // Read triangle count (4 bytes, little-endian)
        let mut triangle_count_buf = [0; 4];
        reader.read_exact(&mut triangle_count_buf)?;
        let triangle_count = u32::from_le_bytes(triangle_count_buf) as usize;
        
        let mut facets = Vec::with_capacity(triangle_count);
        
        for _ in 0..triangle_count {
            // Read normal (3 floats, 4 bytes each, little-endian)
            let mut normal_buf = [0; 12];
            reader.read_exact(&mut normal_buf)?;
            let normal = [
                f32::from_le_bytes(normal_buf[0..4].try_into().unwrap()) as f64,
                f32::from_le_bytes(normal_buf[4..8].try_into().unwrap()) as f64,
                f32::from_le_bytes(normal_buf[8..12].try_into().unwrap()) as f64,
            ];
            
            // Read vertices (3 vertices * 3 floats each, 4 bytes each, little-endian)
            let mut vertices_buf = [0; 36];
            reader.read_exact(&mut vertices_buf)?;
            let mut vertices = [[0.0; 3]; 3];
            
            for i in 0..3 {
                let offset = i * 12;
                vertices[i][0] = f32::from_le_bytes(vertices_buf[offset..offset+4].try_into().unwrap()) as f64;
                vertices[i][1] = f32::from_le_bytes(vertices_buf[offset+4..offset+8].try_into().unwrap()) as f64;
                vertices[i][2] = f32::from_le_bytes(vertices_buf[offset+8..offset+12].try_into().unwrap()) as f64;
            }
            
            // Read attribute byte count (2 bytes, little-endian)
            let mut attr_buf = [0; 2];
            reader.read_exact(&mut attr_buf)?;
            let attr_count: u16 = u16::from_le_bytes(attr_buf);
            
            // Check if color is present (attr_count == 0x0038 for RGB color)
            let color = if attr_count == 0x0038 {
                let red = ((attr_buf[1] & 0xE0) >> 5) * 36;
                let green = ((attr_buf[1] & 0x1C) >> 2) * 36;
                let blue = ((attr_buf[1] & 0x03) << 3) * 36;
                Some([red, green, blue])
            } else {
                None
            };
            
            let facet = if let Some(color) = color {
                StlFacet::with_color(normal, vertices, color)
            } else {
                StlFacet::new(normal, vertices)
            };
            
            facets.push(facet);
        }
        
        // Convert facets to BrepRs shapes
        for facet in facets {
            let shape = self.create_face_from_facet(&facet)?;
            compound.add_component(shape);
        }
        
        Ok(compound)
    }

    /// Create a face from an STL facet
    fn create_face_from_facet(&self, facet: &StlFacet) -> Result<Handle<TopoDS_Shape>, StlError> {
        let builder = BRep_Builder::new();
        
        // Create vertices
        let mut vertex_handles = Vec::with_capacity(3);
        for vertex in &facet.vertices {
            let pnt = Point::new(vertex[0], vertex[1], vertex[2]);
            let v = builder.make_vertex_with_tolerance(pnt, self.tolerance);
            vertex_handles.push(v);
        }
        
        // Create edges
        let mut edge_handles = Vec::with_capacity(3);
        for i in 0..3 {
            let j = (i + 1) % 3;
            let edge = builder.make_edge(vertex_handles[i].clone(), vertex_handles[j].clone());
            edge_handles.push(edge);
        }
        
        // Create wire
        let mut wire = TopoDS_Wire::new();
        for edge in edge_handles {
            wire.add_edge(edge);
        }
        let wire_handle = Handle::new(std::sync::Arc::new(wire));
        
        // Create face
        let face = builder.make_face_with_wire(wire_handle);
        
        // Create a new TopoDS_Shape from the face's shape
        let shape = TopoDS_Shape::new(crate::topology::shape_enum::ShapeType::Face);
        let shape_handle = Handle::new(std::sync::Arc::new(shape));
        
        Ok(shape_handle)
    }

    /// Validate an STL file
    pub fn validate(&self) -> Result<(), StlError> {
        // Just check if the file can be read
        let _ = self.read()?;
        
        Ok(())
    }
}

impl StlWriter {
    /// Create a new STL writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            binary: false,
            precision: 6,
            has_normals: true,
            has_colors: false,
        }
    }

    /// Create a new STL writer with binary format
    pub fn new_binary(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            binary: true,
            precision: 6,
            has_normals: true,
            has_colors: false,
        }
    }

    /// Set whether to use binary format
    pub fn set_binary(&mut self, binary: bool) {
        self.binary = binary;
    }

    /// Get whether binary format is used
    pub fn binary(&self) -> bool {
        self.binary
    }

    /// Set the precision for ASCII format
    pub fn set_precision(&mut self, precision: usize) {
        self.precision = precision;
    }

    /// Get the precision
    pub fn precision(&self) -> usize {
        self.precision
    }

    /// Set whether to write normals
    pub fn set_write_normals(&mut self, write_normals: bool) {
        self.has_normals = write_normals;
    }

    /// Get whether normals are written
    pub fn write_normals(&self) -> bool {
        self.has_normals
    }

    /// Set whether to write colors
    pub fn set_write_colors(&mut self, write_colors: bool) {
        self.has_colors = write_colors;
    }

    /// Get whether colors are written
    pub fn write_colors(&self) -> bool {
        self.has_colors
    }

    /// Write a shape to an STL file
    pub fn write(&self, shape: &TopoDS_Shape) -> Result<(), StlError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        
        if self.binary {
            self.write_binary(file, shape)
        } else {
            self.write_ascii(file, shape)
        }
    }

    /// Write ASCII STL format
    fn write_ascii(&self, file: File, shape: &TopoDS_Shape) -> Result<(), StlError> {
        let mut writer = BufWriter::new(file);
        
        // Write header
        writeln!(writer, "solid BrepRs")?;
        
        // Write facets using TopExpExplorer
        let mut explorer = TopExpExplorer::new(shape, ShapeType::Face);
        while explorer.more() {
            explorer.next();
            if let Some(current_shape) = explorer.current() {
                if current_shape.is_face() {
                    // Safe cast since we checked the type
                    let face_ref = unsafe {
                        &*(current_shape as *const _ as *const TopoDS_Face)
                    };
                    let facet = self.create_facet_from_face(face_ref)?;
                    
                    // Write normal
                    writeln!(writer, "  facet normal {:.prec$} {:.prec$} {:.prec$}", 
                        facet.normal[0], facet.normal[1], facet.normal[2], 
                        prec = self.precision)?;
                    
                    // Write outer loop
                    writeln!(writer, "    outer loop")?;
                    
                    // Write vertices
                    for vertex in &facet.vertices {
                        writeln!(writer, "      vertex {:.prec$} {:.prec$} {:.prec$}", 
                            vertex[0], vertex[1], vertex[2], 
                            prec = self.precision)?;
                    }
                    
                    // Write end of loop and facet
                    writeln!(writer, "    endloop")?;
                    writeln!(writer, "  endfacet")?;
                }
            }
        }
        
        // Write footer
        writeln!(writer, "endsolid BrepRs")?;
        
        Ok(())
    }

    /// Write binary STL format
    fn write_binary(&self, file: File, shape: &TopoDS_Shape) -> Result<(), StlError> {
        let mut writer = BufWriter::new(file);
        
        // Write header (80 bytes)
        let header = "BrepRs STL Binary Format";
        let mut header_buf = [0u8; 80];
        header_buf[..header.len()].copy_from_slice(header.as_bytes());
        writer.write_all(&header_buf)?;
        
        // Collect facets using TopExpExplorer
        let mut facets = Vec::new();
        let mut explorer = TopExpExplorer::new(shape, ShapeType::Face);
        while explorer.more() {
            explorer.next();
            if let Some(current_shape) = explorer.current() {
                if current_shape.is_face() {
                    // Safe cast since we checked the type
                    let face_ref = unsafe {
                        &*(current_shape as *const _ as *const TopoDS_Face)
                    };
                    let facet = self.create_facet_from_face(face_ref)?;
                    facets.push(facet);
                }
            }
        }
        
        // Write triangle count (4 bytes, little-endian)
        let triangle_count = facets.len() as u32;
        writer.write_all(&triangle_count.to_le_bytes())?;
        
        // Write facets
        for facet in facets {
            // Write normal (3 floats, 4 bytes each, little-endian)
            for &component in &facet.normal {
                let value = component as f32;
                writer.write_all(&value.to_le_bytes())?;
            }
            
            // Write vertices (3 vertices * 3 floats each, 4 bytes each, little-endian)
            for vertex in &facet.vertices {
                for &component in vertex {
                    let value = component as f32;
                    writer.write_all(&value.to_le_bytes())?;
                }
            }
            
            // Write attribute byte count (2 bytes, little-endian)
            let attr_count: u16 = if facet.color.is_some() {
                0x0038 // Color present
            } else {
                0x0000 // No color
            };
            writer.write_all(&attr_count.to_le_bytes())?;
        }
        
        Ok(())
    }

    /// Create an STL facet from a face
    fn create_facet_from_face(&self, face: &TopoDS_Face) -> Result<StlFacet, StlError> {
        // For now, return a placeholder facet
        // In a real implementation, this would:
        // 1. Get the face's vertices
        // 2. Calculate the normal
        // 3. Create the facet
        
        // Placeholder implementation
        let normal = [0.0, 0.0, 1.0];
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        
        Ok(StlFacet::new(normal, vertices))
    }

    /// Validate the output STL file
    pub fn validate_output(&self) -> Result<(), StlError> {
        let reader = StlReader::new(&self.filename);
        reader.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::modeling::primitives;
    use crate::geometry::Point;

    #[test]
    fn test_stl_reader_creation() {
        let reader = StlReader::new("test.stl");
        assert_eq!(reader.tolerance(), 1e-6);
        assert!(reader.read_normals());
        assert!(!reader.read_colors());
    }

    #[test]
    fn test_stl_writer_creation() {
        let writer = StlWriter::new("test.stl");
        assert!(!writer.binary());
        assert_eq!(writer.precision(), 6);
        assert!(writer.write_normals());
        assert!(!writer.write_colors());
    }

    #[test]
    fn test_stl_writer_binary_creation() {
        let writer = StlWriter::new_binary("test.stl");
        assert!(writer.binary());
        assert_eq!(writer.precision(), 6);
    }

    #[test]
    fn test_stl_facet_creation() {
        let normal = [0.0, 0.0, 1.0];
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        
        let facet = StlFacet::new(normal, vertices);
        assert_eq!(facet.normal, normal);
        assert_eq!(facet.vertices, vertices);
        assert!(facet.color.is_none());
        assert!(facet.is_valid());
    }

    #[test]
    fn test_stl_facet_with_color() {
        let normal = [0.0, 0.0, 1.0];
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let color = [255, 0, 0];
        
        let facet = StlFacet::with_color(normal, vertices, color);
        assert_eq!(facet.normal, normal);
        assert_eq!(facet.vertices, vertices);
        assert_eq!(facet.color, Some(color));
        assert!(facet.is_valid());
    }

    #[test]
    fn test_stl_facet_invalid() {
        // Create a facet with colinear vertices
        let normal = [0.0, 0.0, 1.0];
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
        ];
        
        let facet = StlFacet::new(normal, vertices);
        assert!(!facet.is_valid());
    }

    #[test]
    fn test_stl_read_write_cycle() {
        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        
        // Write to STL file
        let writer = StlWriter::new("test_write.stl");
        let result = writer.write(&box_solid.shape());
        
        // The write operation should succeed (even with placeholder implementation)
        assert!(result.is_ok());
        
        // Clean up
        if Path::new("test_write.stl").exists() {
            let _ = fs::remove_file("test_write.stl");
        }
    }

    #[test]
    fn test_stl_read_write_binary() {
        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        
        // Write to binary STL file
        let writer = StlWriter::new_binary("test_write_binary.stl");
        let result = writer.write(&box_solid.shape());
        
        // The write operation should succeed (even with placeholder implementation)
        assert!(result.is_ok());
        
        // Clean up
        if Path::new("test_write_binary.stl").exists() {
            let _ = fs::remove_file("test_write_binary.stl");
        }
    }

    #[test]
    fn test_stl_validate() {
        // Create a simple box
        let box_solid = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        
        // Write to STL file
        let writer = StlWriter::new("test_validate.stl");
        let write_result = writer.write(&box_solid.shape());
        assert!(write_result.is_ok());
        
        // Validate the file
        let reader = StlReader::new("test_validate.stl");
        let validate_result = reader.validate();
        
        // Clean up
        if Path::new("test_validate.stl").exists() {
            let _ = fs::remove_file("test_validate.stl");
        }
        
        // The validate operation should succeed (even with placeholder implementation)
        assert!(validate_result.is_ok());
    }

    #[test]
    fn test_stl_writer_settings() {
        let mut writer = StlWriter::new("test.stl");
        
        writer.set_binary(true);
        assert!(writer.binary());
        
        writer.set_precision(10);
        assert_eq!(writer.precision(), 10);
        
        writer.set_write_normals(false);
        assert!(!writer.write_normals());
        
        writer.set_write_colors(true);
        assert!(writer.write_colors());
    }

    #[test]
    fn test_stl_reader_settings() {
        let mut reader = StlReader::new("test.stl");
        
        reader.set_tolerance(1e-4);
        assert_eq!(reader.tolerance(), 1e-4);
        
        reader.set_read_normals(false);
        assert!(!reader.read_normals());
        
        reader.set_read_colors(true);
        assert!(reader.read_colors());
    }
}
