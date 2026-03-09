//! 3MF (3D Manufacturing Format) file format support
//!
//! This module provides functionality for reading and writing 3MF files,
//! which are OPC (Open Packaging Conventions) packages containing XML data.

// ...existing code...
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use crate::geometry::Point;
use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// 3MF file format error types
#[derive(Debug)]
pub enum ThreemfError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid 3MF file format
    InvalidFormat,
    /// Invalid XML parsing
    XmlError(String),
    /// Invalid OPC package
    InvalidOpcPackage,
    /// Missing required part
    MissingPart(String),
    /// Invalid mesh data
    InvalidMesh,
    /// Invalid vertex data
    InvalidVertex,
    /// Invalid triangle data
    InvalidTriangle,
    /// Unsupported extension
    UnsupportedExtension(String),
}

impl From<std::io::Error> for ThreemfError {
    fn from(err: std::io::Error) -> Self {
        ThreemfError::IoError(err)
    }
}

/// 3MF vertex
#[derive(Debug, Clone, Default)]
pub struct ThreemfVertex {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl ThreemfVertex {
    /// Create a new vertex
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Convert to Point
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
}

/// 3MF triangle
#[derive(Debug, Clone, Default)]
pub struct ThreemfTriangle {
    /// First vertex index
    pub v1: u32,
    /// Second vertex index
    pub v2: u32,
    /// Third vertex index
    pub v3: u32,
}

impl ThreemfTriangle {
    /// Create a new triangle
    pub fn new(v1: u32, v2: u32, v3: u32) -> Self {
        Self { v1, v2, v3 }
    }
}

/// 3MF mesh
#[derive(Debug, Clone, Default)]
pub struct ThreemfMesh {
    /// Vertices
    pub vertices: Vec<ThreemfVertex>,
    /// Triangles
    pub triangles: Vec<ThreemfTriangle>,
}

impl ThreemfMesh {
    /// Create a new mesh
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, vertex: ThreemfVertex) -> u32 {
        let index = self.vertices.len() as u32;
        self.vertices.push(vertex);
        index
    }

    /// Add a triangle
    pub fn add_triangle(&mut self, triangle: ThreemfTriangle) {
        self.triangles.push(triangle);
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get triangle count
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
}

/// 3MF component
#[derive(Debug, Clone)]
pub struct ThreemfComponent {
    /// Object ID reference
    pub object_id: u32,
    /// Transform matrix (optional)
    pub transform: Option<[f64; 12]>,
}

impl ThreemfComponent {
    /// Create a new component
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            transform: None,
        }
    }

    /// Create with transform
    pub fn with_transform(object_id: u32, transform: [f64; 12]) -> Self {
        Self {
            object_id,
            transform: Some(transform),
        }
    }
}

/// 3MF object type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreemfObjectType {
    /// Model object
    Model,
    /// Support object
    Support,
    /// Solid support object
    SolidSupport,
    /// Other
    Other,
}

impl Default for ThreemfObjectType {
    fn default() -> Self {
        ThreemfObjectType::Model
    }
}

impl ThreemfObjectType {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "model" => ThreemfObjectType::Model,
            "support" => ThreemfObjectType::Support,
            "solidsupport" => ThreemfObjectType::SolidSupport,
            _ => ThreemfObjectType::Other,
        }
    }

    /// Convert to string
    pub fn to_str(&self) -> &'static str {
        match self {
            ThreemfObjectType::Model => "model",
            ThreemfObjectType::Support => "support",
            ThreemfObjectType::SolidSupport => "solidsupport",
            ThreemfObjectType::Other => "other",
        }
    }
}

/// 3MF object
#[derive(Debug, Clone)]
pub struct ThreemfObject {
    /// Object ID
    pub id: u32,
    /// Object type
    pub object_type: ThreemfObjectType,
    /// Object name (optional)
    pub name: Option<String>,
    /// Part number (optional)
    pub part_number: Option<String>,
    /// Mesh data (if mesh object)
    pub mesh: Option<ThreemfMesh>,
    /// Components (if assembly object)
    pub components: Vec<ThreemfComponent>,
}

impl ThreemfObject {
    /// Create a new mesh object
    pub fn new_mesh(id: u32) -> Self {
        Self {
            id,
            object_type: ThreemfObjectType::Model,
            name: None,
            part_number: None,
            mesh: Some(ThreemfMesh::new()),
            components: Vec::new(),
        }
    }

    /// Create a new component object
    pub fn new_components(id: u32) -> Self {
        Self {
            id,
            object_type: ThreemfObjectType::Model,
            name: None,
            part_number: None,
            mesh: None,
            components: Vec::new(),
        }
    }

    /// Add a component
    pub fn add_component(&mut self, component: ThreemfComponent) {
        self.components.push(component);
    }

    /// Check if this is a mesh object
    pub fn is_mesh(&self) -> bool {
        self.mesh.is_some()
    }

    /// Check if this is a component object
    pub fn is_components(&self) -> bool {
        self.mesh.is_none() && !self.components.is_empty()
    }
}

/// 3MF build item
#[derive(Debug, Clone)]
pub struct ThreemfBuildItem {
    /// Object ID reference
    pub object_id: u32,
    /// Transform matrix (optional)
    pub transform: Option<[f64; 12]>,
    /// Part number (optional)
    pub part_number: Option<String>,
}

impl ThreemfBuildItem {
    /// Create a new build item
    pub fn new(object_id: u32) -> Self {
        Self {
            object_id,
            transform: None,
            part_number: None,
        }
    }

    /// Create with transform
    pub fn with_transform(object_id: u32, transform: [f64; 12]) -> Self {
        Self {
            object_id,
            transform: Some(transform),
            part_number: None,
        }
    }
}

/// 3MF color
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreemfColor {
    /// Red (0-255)
    pub r: u8,
    /// Green (0-255)
    pub g: u8,
    /// Blue (0-255)
    pub b: u8,
    /// Alpha (0-255)
    pub a: u8,
}

impl ThreemfColor {
    /// Create a new color
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create with alpha
    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create from RGBA hex value
    pub fn from_rgba(rgba: u32) -> Self {
        Self {
            r: ((rgba >> 24) & 0xFF) as u8,
            g: ((rgba >> 16) & 0xFF) as u8,
            b: ((rgba >> 8) & 0xFF) as u8,
            a: (rgba & 0xFF) as u8,
        }
    }

    /// Convert to RGBA hex value
    pub fn to_rgba(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}

/// 3MF material
#[derive(Debug, Clone)]
pub struct ThreemfMaterial {
    /// Material ID
    pub id: u32,
    /// Material name
    pub name: Option<String>,
    /// Color
    pub color: Option<ThreemfColor>,
}

impl ThreemfMaterial {
    /// Create a new material
    pub fn new(id: u32) -> Self {
        Self {
            id,
            name: None,
            color: None,
        }
    }

    /// Create with color
    pub fn with_color(id: u32, color: ThreemfColor) -> Self {
        Self {
            id,
            name: None,
            color: Some(color),
        }
    }
}

/// 3MF unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreemfUnit {
    /// Microns
    Micron,
    /// Millimeters
    Millimeter,
    /// Centimeters
    Centimeter,
    /// Inches
    Inch,
    /// Feet
    Foot,
    /// Meters
    Meter,
}

impl Default for ThreemfUnit {
    fn default() -> Self {
        ThreemfUnit::Millimeter
    }
}

impl ThreemfUnit {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "micron" => ThreemfUnit::Micron,
            "millimeter" | "mm" => ThreemfUnit::Millimeter,
            "centimeter" | "cm" => ThreemfUnit::Centimeter,
            "inch" | "in" => ThreemfUnit::Inch,
            "foot" | "ft" => ThreemfUnit::Foot,
            "meter" | "m" => ThreemfUnit::Meter,
            _ => ThreemfUnit::Millimeter,
        }
    }

    /// Convert to string
    pub fn to_str(&self) -> &'static str {
        match self {
            ThreemfUnit::Micron => "micron",
            ThreemfUnit::Millimeter => "millimeter",
            ThreemfUnit::Centimeter => "centimeter",
            ThreemfUnit::Inch => "inch",
            ThreemfUnit::Foot => "foot",
            ThreemfUnit::Meter => "meter",
        }
    }

    /// Get conversion factor to millimeters
    pub fn to_mm_factor(&self) -> f64 {
        match self {
            ThreemfUnit::Micron => 0.001,
            ThreemfUnit::Millimeter => 1.0,
            ThreemfUnit::Centimeter => 10.0,
            ThreemfUnit::Inch => 25.4,
            ThreemfUnit::Foot => 304.8,
            ThreemfUnit::Meter => 1000.0,
        }
    }
}

/// 3MF model
#[derive(Debug, Clone, Default)]
pub struct ThreemfModel {
    /// Unit
    pub unit: ThreemfUnit,
    /// XML language
    pub xml_lang: Option<String>,
    /// Required extensions
    pub required_extensions: Vec<String>,
    /// Objects
    pub objects: Vec<ThreemfObject>,
    /// Build items
    pub build_items: Vec<ThreemfBuildItem>,
    /// Materials
    pub materials: Vec<ThreemfMaterial>,
}

impl ThreemfModel {
    /// Create a new model
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an object
    pub fn add_object(&mut self, object: ThreemfObject) -> u32 {
        let id = object.id;
        self.objects.push(object);
        id
    }

    /// Add a build item
    pub fn add_build_item(&mut self, item: ThreemfBuildItem) {
        self.build_items.push(item);
    }

    /// Add a material
    pub fn add_material(&mut self, material: ThreemfMaterial) -> u32 {
        let id = material.id;
        self.materials.push(material);
        id
    }

    /// Get object by ID
    pub fn get_object(&self, id: u32) -> Option<&ThreemfObject> {
        self.objects.iter().find(|o| o.id == id)
    }

    /// Get total triangle count
    pub fn total_triangle_count(&self) -> usize {
        self.objects
            .iter()
            .filter_map(|o| o.mesh.as_ref())
            .map(|m| m.triangle_count())
            .sum()
    }

    /// Get total vertex count
    pub fn total_vertex_count(&self) -> usize {
        self.objects
            .iter()
            .filter_map(|o| o.mesh.as_ref())
            .map(|m| m.vertex_count())
            .sum()
    }
}

/// 3MF reader for reading 3MF files
pub struct ThreemfReader {
    filename: String,
    model: ThreemfModel,
}

impl ThreemfReader {
    /// Create a new 3MF reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            model: ThreemfModel::new(),
        }
    }

    /// Read a 3MF file
    pub fn read(&mut self) -> Result<&ThreemfModel, ThreemfError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read ZIP archive (3MF is a ZIP/OPC package)
        // For now, we'll implement a simplified version
        // that reads the 3D Model part directly

        // In a full implementation, we would:
        // 1. Parse the ZIP structure
        // 2. Read [Content_Types].xml
        // 3. Read _rels/.rels
        // 4. Read 3D/3dmodel.model
        // 5. Read any additional parts (textures, materials, etc.)

        // Simplified: read as raw XML for now
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        self.parse_model_xml(&content)?;

        Ok(&self.model)
    }

    /// Parse 3MF model XML
    fn parse_model_xml(&mut self, _xml: &str) -> Result<(), ThreemfError> {
        // Placeholder implementation
        // In a full implementation, we would use an XML parser
        // to extract objects, meshes, materials, etc.

        Ok(())
    }

    /// Get the model
    pub fn model(&self) -> &ThreemfModel {
        &self.model
    }

    /// Convert to TopoDsShape
    pub fn to_shape(&self) -> Result<TopoDsShape, ThreemfError> {
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }
}

/// 3MF writer for writing 3MF files
pub struct ThreemfWriter {
    filename: String,
    model: ThreemfModel,
}

impl ThreemfWriter {
    /// Create a new 3MF writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            model: ThreemfModel::new(),
        }
    }

    /// Get the model
    pub fn model(&mut self) -> &mut ThreemfModel {
        &mut self.model
    }

    /// Write 3MF file
    pub fn write(&self) -> Result<(), ThreemfError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Write ZIP archive structure
        // For now, we'll write a simplified version

        // In a full implementation, we would:
        // 1. Create ZIP structure
        // 2. Write [Content_Types].xml
        // 3. Write _rels/.rels
        // 4. Write 3D/3dmodel.model
        // 5. Write any additional parts

        // Simplified: write XML content directly
        let xml = self.generate_model_xml();
        writer.write_all(xml.as_bytes())?;

        Ok(())
    }

    /// Generate 3MF model XML
    fn generate_model_xml(&self) -> String {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<model unit=\"");
        xml.push_str(self.model.unit.to_str());
        xml.push_str("\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n");

        // Write resources
        xml.push_str("  <resources>\n");

        // Write objects
        for object in &self.model.objects {
            xml.push_str(&self.generate_object_xml(object));
        }

        // Write materials
        if !self.model.materials.is_empty() {
            xml.push_str("    <basematerials id=\"1\">\n");
            for material in &self.model.materials {
                xml.push_str("      <base");
                if let Some(ref name) = material.name {
                    xml.push_str(&format!(" name=\"{}\"", name));
                }
                if let Some(color) = material.color {
                    xml.push_str(&format!(
                        " color=\"#{:02X}{:02X}{:02X}{:02X}\"",
                        color.r, color.g, color.b, color.a
                    ));
                }
                xml.push_str("/>\n");
            }
            xml.push_str("    </basematerials>\n");
        }

        xml.push_str("  </resources>\n");

        // Write build
        xml.push_str("  <build>\n");
        for item in &self.model.build_items {
            xml.push_str(&format!("    <item objectid=\"{}\"", item.object_id));
            if let Some(ref transform) = item.transform {
                xml.push_str(&format!(
                    " transform=\"{} {} {} {} {} {} {} {} {} {} {} {}\"",
                    transform[0],
                    transform[1],
                    transform[2],
                    transform[3],
                    transform[4],
                    transform[5],
                    transform[6],
                    transform[7],
                    transform[8],
                    transform[9],
                    transform[10],
                    transform[11]
                ));
            }
            xml.push_str("/>\n");
        }
        xml.push_str("  </build>\n");

        xml.push_str("</model>\n");

        xml
    }

    /// Generate object XML
    fn generate_object_xml(&self, object: &ThreemfObject) -> String {
        let mut xml = String::new();

        xml.push_str(&format!(
            "    <object id=\"{}\" type=\"{}\">\n",
            object.id,
            object.object_type.to_str()
        ));

        if let Some(ref name) = object.name {
            xml.push_str(&format!("      <name>{}</name>\n", name));
        }

        if let Some(ref mesh) = object.mesh {
            xml.push_str(&self.generate_mesh_xml(mesh));
        }

        if !object.components.is_empty() {
            xml.push_str("      <components>\n");
            for component in &object.components {
                xml.push_str(&format!(
                    "        <component objectid=\"{}\"",
                    component.object_id
                ));
                if let Some(ref transform) = component.transform {
                    xml.push_str(&format!(
                        " transform=\"{} {} {} {} {} {} {} {} {} {} {} {}\"",
                        transform[0],
                        transform[1],
                        transform[2],
                        transform[3],
                        transform[4],
                        transform[5],
                        transform[6],
                        transform[7],
                        transform[8],
                        transform[9],
                        transform[10],
                        transform[11]
                    ));
                }
                xml.push_str("/>\n");
            }
            xml.push_str("      </components>\n");
        }

        xml.push_str("    </object>\n");

        xml
    }

    /// Generate mesh XML
    fn generate_mesh_xml(&self, mesh: &ThreemfMesh) -> String {
        let mut xml = String::new();

        xml.push_str("      <mesh>\n");

        // Write vertices
        xml.push_str("        <vertices>\n");
        for vertex in &mesh.vertices {
            xml.push_str(&format!(
                "          <vertex x=\"{}\" y=\"{}\" z=\"{}\"/>\n",
                vertex.x, vertex.y, vertex.z
            ));
        }
        xml.push_str("        </vertices>\n");

        // Write triangles
        xml.push_str("        <triangles>\n");
        for triangle in &mesh.triangles {
            xml.push_str(&format!(
                "          <triangle v1=\"{}\" v2=\"{}\" v3=\"{}\"/>\n",
                triangle.v1, triangle.v2, triangle.v3
            ));
        }
        xml.push_str("        </triangles>\n");

        xml.push_str("      </mesh>\n");

        xml
    }

    /// Add a mesh object
    pub fn add_mesh_object(&mut self, id: u32) -> &mut ThreemfMesh {
        let object = ThreemfObject::new_mesh(id);
        self.model.add_object(object);
        self.model
            .objects
            .last_mut()
            .unwrap()
            .mesh
            .as_mut()
            .unwrap()
    }

    /// Add a build item
    pub fn add_build_item(&mut self, object_id: u32) {
        self.model.add_build_item(ThreemfBuildItem::new(object_id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threemf_reader_creation() {
        let reader = ThreemfReader::new("test.3mf");
        assert_eq!(reader.filename, "test.3mf");
    }

    #[test]
    fn test_threemf_writer_creation() {
        let writer = ThreemfWriter::new("test.3mf");
        assert_eq!(writer.filename, "test.3mf");
    }

    #[test]
    fn test_vertex_creation() {
        let vertex = ThreemfVertex::new(1.0, 2.0, 3.0);
        assert_eq!(vertex.x, 1.0);
        assert_eq!(vertex.y, 2.0);
        assert_eq!(vertex.z, 3.0);
    }

    #[test]
    fn test_triangle_creation() {
        let triangle = ThreemfTriangle::new(0, 1, 2);
        assert_eq!(triangle.v1, 0);
        assert_eq!(triangle.v2, 1);
        assert_eq!(triangle.v3, 2);
    }

    #[test]
    fn test_mesh_operations() {
        let mut mesh = ThreemfMesh::new();
        let v1 = mesh.add_vertex(ThreemfVertex::new(0.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(ThreemfVertex::new(1.0, 0.0, 0.0));
        let v3 = mesh.add_vertex(ThreemfVertex::new(0.0, 1.0, 0.0));

        mesh.add_triangle(ThreemfTriangle::new(v1, v2, v3));

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }

    #[test]
    fn test_object_type_parsing() {
        assert_eq!(
            ThreemfObjectType::from_str("model"),
            ThreemfObjectType::Model
        );
        assert_eq!(
            ThreemfObjectType::from_str("support"),
            ThreemfObjectType::Support
        );
        assert_eq!(
            ThreemfObjectType::from_str("solidsupport"),
            ThreemfObjectType::SolidSupport
        );
    }

    #[test]
    fn test_unit_parsing() {
        assert_eq!(ThreemfUnit::from_str("millimeter"), ThreemfUnit::Millimeter);
        assert_eq!(ThreemfUnit::from_str("inch"), ThreemfUnit::Inch);
        assert_eq!(ThreemfUnit::from_str("meter"), ThreemfUnit::Meter);
    }

    #[test]
    fn test_unit_conversion() {
        assert_eq!(ThreemfUnit::Millimeter.to_mm_factor(), 1.0);
        assert_eq!(ThreemfUnit::Inch.to_mm_factor(), 25.4);
        assert_eq!(ThreemfUnit::Meter.to_mm_factor(), 1000.0);
    }

    #[test]
    fn test_color_creation() {
        let color = ThreemfColor::new(255, 128, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_color_rgba() {
        let color = ThreemfColor::from_rgba(0xFF8000FF);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_model_operations() {
        let mut model = ThreemfModel::new();

        let object = ThreemfObject::new_mesh(1);
        model.add_object(object);

        model.add_build_item(ThreemfBuildItem::new(1));

        assert_eq!(model.objects.len(), 1);
        assert_eq!(model.build_items.len(), 1);
    }

    #[test]
    fn test_writer_add_mesh() {
        let mut writer = ThreemfWriter::new("test.3mf");
        let mesh = writer.add_mesh_object(1);

        mesh.add_vertex(ThreemfVertex::new(0.0, 0.0, 0.0));
        mesh.add_vertex(ThreemfVertex::new(1.0, 0.0, 0.0));
        mesh.add_vertex(ThreemfVertex::new(0.0, 1.0, 0.0));

        assert_eq!(mesh.vertex_count(), 3);
    }
}
