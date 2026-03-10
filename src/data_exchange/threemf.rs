//! 3MF (3D Manufacturing Format) Support
//!
//! This module provides import and export functionality for 3MF format,
//! a modern file format developed by the 3MF Consortium for 3D printing.
//!
//! 3MF is designed to be the standard format for 3D printing, providing
//! a complete and unambiguous description of 3D models, including:
//! - Mesh geometry
//! - Materials and colors
//! - Print ticket information
//! - Multiple objects and build instructions

use std::io::{Read, Write};
use std::path::Path;

use crate::data_exchange::{DataExchangeError, DataExchangeResult};
use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::mesh::mesh_data::Mesh2D;
use crate::topology::topods_shape::TopoDsShape;

/// 3MF specification version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeMfVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
}

impl Default for ThreeMfVersion {
    fn default() -> Self {
        ThreeMfVersion::V1_3
    }
}

impl ThreeMfVersion {
    pub fn namespace(&self) -> &'static str {
        match self {
            ThreeMfVersion::V1_0 => "http://schemas.microsoft.com/3dmanufacturing/core/2015/02",
            ThreeMfVersion::V1_1 => "http://schemas.microsoft.com/3dmanufacturing/core/2015/02",
            ThreeMfVersion::V1_2 => "http://schemas.microsoft.com/3dmanufacturing/core/2015/02",
            ThreeMfVersion::V1_3 => "http://schemas.microsoft.com/3dmanufacturing/core/2015/02",
        }
    }
}

/// 3MF export options
#[derive(Debug, Clone)]
pub struct ThreeMfExportOptions {
    pub version: ThreeMfVersion,
    pub include_colors: bool,
    pub include_materials: bool,
    pub include_textures: bool,
    pub include_print_ticket: bool,
    pub unit: ThreeMfUnit,
    pub compression_level: u32,
}

impl Default for ThreeMfExportOptions {
    fn default() -> Self {
        Self {
            version: ThreeMfVersion::V1_3,
            include_colors: true,
            include_materials: true,
            include_textures: false,
            include_print_ticket: false,
            unit: ThreeMfUnit::Millimeter,
            compression_level: 6,
        }
    }
}

impl ThreeMfExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_unit(mut self, unit: ThreeMfUnit) -> Self {
        self.unit = unit;
        self
    }

    pub fn with_compression(mut self, level: u32) -> Self {
        self.compression_level = level.min(9);
        self
    }

    pub fn without_colors(mut self) -> Self {
        self.include_colors = false;
        self
    }
}

/// 3MF measurement units
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreeMfUnit {
    Micron,
    Millimeter,
    Centimeter,
    Inch,
    Foot,
    Meter,
}

impl ThreeMfUnit {
    pub fn to_millimeters(&self) -> f64 {
        match self {
            ThreeMfUnit::Micron => 0.001,
            ThreeMfUnit::Millimeter => 1.0,
            ThreeMfUnit::Centimeter => 10.0,
            ThreeMfUnit::Inch => 25.4,
            ThreeMfUnit::Foot => 304.8,
            ThreeMfUnit::Meter => 1000.0,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ThreeMfUnit::Micron => "micron",
            ThreeMfUnit::Millimeter => "millimeter",
            ThreeMfUnit::Centimeter => "centimeter",
            ThreeMfUnit::Inch => "inch",
            ThreeMfUnit::Foot => "foot",
            ThreeMfUnit::Meter => "meter",
        }
    }
}

/// 3MF import options
#[derive(Debug, Clone)]
pub struct ThreeMfImportOptions {
    pub load_colors: bool,
    pub load_materials: bool,
    pub load_textures: bool,
    pub load_print_ticket: bool,
    pub repair_meshes: bool,
}

impl Default for ThreeMfImportOptions {
    fn default() -> Self {
        Self {
            load_colors: true,
            load_materials: true,
            load_textures: true,
            load_print_ticket: false,
            repair_meshes: true,
        }
    }
}

/// 3MF color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreeMfColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ThreeMfColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
}

/// 3MF material definition
#[derive(Debug, Clone)]
pub struct ThreeMfMaterial {
    pub id: u32,
    pub name: String,
    pub color: ThreeMfColor,
}

/// 3MF object with metadata
#[derive(Debug, Clone)]
pub struct ThreeMfObject {
    pub id: u32,
    pub name: Option<String>,
    pub mesh: Mesh2D,
    pub color: Option<ThreeMfColor>,
    pub material_id: Option<u32>,
}

/// 3MF exporter
pub struct ThreeMfExporter {
    options: ThreeMfExportOptions,
    materials: Vec<ThreeMfMaterial>,
}

impl ThreeMfExporter {
    pub fn new() -> Self {
        Self {
            options: ThreeMfExportOptions::default(),
            materials: Vec::new(),
        }
    }

    pub fn with_options(options: ThreeMfExportOptions) -> Self {
        Self {
            options,
            materials: Vec::new(),
        }
    }

    /// Add a material for export
    pub fn add_material(&mut self, material: ThreeMfMaterial) {
        self.materials.push(material);
    }

    /// Export a single mesh to 3MF format
    pub fn export_mesh(&self, mesh: &Mesh2D, output_path: &Path) -> DataExchangeResult<()> {
        let object = ThreeMfObject {
            id: 1,
            name: Some("Object".to_string()),
            mesh: mesh.clone(),
            color: None,
            material_id: None,
        };
        self.export_objects(&[object], output_path)
    }

    /// Export multiple objects to 3MF format
    pub fn export_objects(
        &self,
        objects: &[ThreeMfObject],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        // Create ZIP archive (3MF is a ZIP-based format)
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(Vec::new()));

        let compression = match self.options.compression_level {
            0 => zip::CompressionMethod::Stored,
            _ => zip::CompressionMethod::Deflated,
        };

        let options = zip::write::FileOptions::default()
            .compression_method(compression)
            .compression_level(Some(self.options.compression_level as i32));

        // Add [Content_Types].xml
        zip.start_file("[Content_Types].xml", options)?;
        zip.write_all(self.generate_content_types().as_bytes())?;

        // Add _rels/.rels
        zip.start_file("_rels/.rels", options)?;
        zip.write_all(self.generate_rels().as_bytes())?;

        // Add 3D/3dmodel.model
        zip.start_file("3D/3dmodel.model", options)?;
        zip.write_all(self.generate_model(objects).as_bytes())?;

        // Add 3D/_rels/3dmodel.model.rels if needed
        if self.options.include_print_ticket {
            zip.start_file("3D/_rels/3dmodel.model.rels", options)?;
            zip.write_all(self.generate_model_rels().as_bytes())?;
        }

        // Finish ZIP
        let zip_data = zip.finish()?.into_inner();

        // Write to file
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(&zip_data)?;

        Ok(())
    }

    /// Export a shape to 3MF (requires mesh generation)
    pub fn export_shape(
        &self,
        shape: &Handle<TopoDsShape>,
        mesh_generator: &crate::mesh::MeshGenerator,
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let mesh = mesh_generator.generate(shape, 0.1, 0.5);
        self.export_mesh(&mesh, output_path)
    }

    fn generate_content_types(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(
            "<Types xmlns=\"http://schemas.openxmlformats.org/package/2006/content-types\">\n",
        );
        xml.push_str("  <Default Extension=\"rels\" ContentType=\"application/vnd.openxmlformats-package.relationships+xml\"/>\n");
        xml.push_str("  <Default Extension=\"model\" ContentType=\"application/vnd.ms-package.3dmanufacturing-3dmodel+xml\"/>\n");
        xml.push_str("</Types>\n");
        xml
    }

    fn generate_rels(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n");
        xml.push_str("  <Relationship Id=\"rel0\" Type=\"http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel\" Target=\"/3D/3dmodel.model\"/>\n");
        xml.push_str("</Relationships>\n");
        xml
    }

    fn generate_model_rels(&self) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">\n");
        // Add print ticket relationship if needed
        xml.push_str("</Relationships>\n");
        xml
    }

    fn generate_model(&self, objects: &[ThreeMfObject]) -> String {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str(&format!(
            "<model unit=\"{}\" xml:lang=\"en-US\" xmlns=\"{}\" xmlns:m=\"http://schemas.microsoft.com/3dmanufacturing/material/2015/02\">\n",
            self.options.unit.as_str(),
            self.options.version.namespace()
        ));

        // Generate metadata
        xml.push_str("  <metadata name=\"Title\">BrepRs Export</metadata>\n");
        xml.push_str("  <metadata name=\"Designer\">BrepRs</metadata>\n");
        xml.push_str("  <metadata name=\"Description\">Exported from BrepRs</metadata>\n");
        xml.push_str(&format!(
            "  <metadata name=\"CreationDate\">{}</metadata>\n",
            chrono::Local::now().format("%Y-%m-%d")
        ));

        // Generate resources
        xml.push_str("  <resources>\n");

        // Add materials
        if self.options.include_materials {
            for material in &self.materials {
                xml.push_str(&format!(
                    "    <m:color id=\"{}\" color=\"{}\"/>\n",
                    material.id,
                    material.color.to_hex()
                ));
            }
        }

        // Add objects
        for object in objects {
            xml.push_str(&format!(
                "    <object id=\"{}\" type=\"model\">\n",
                object.id
            ));

            if let Some(ref name) = object.name {
                xml.push_str(&format!(
                    "      <metadata name=\"name\">{}</metadata>\n",
                    name
                ));
            }

            xml.push_str("      <mesh>\n");

            // Vertices
            xml.push_str("        <vertices>\n");
            for i in 0..object.mesh.vertex_count() {
                if let Some(vertex) = object.mesh.vertex(i) {
                    xml.push_str(&format!(
                        "          <vertex x=\"{}\" y=\"{}\" z=\"{}\"/>\n",
                        vertex.point.x, vertex.point.y, vertex.point.z
                    ));
                }
            }
            xml.push_str("        </vertices>\n");

            // Triangles
            xml.push_str("        <triangles>\n");
            for i in 0..object.mesh.triangle_count() {
                if let Some(triangle) = object.mesh.triangle(i) {
                    let mut triangle_str = format!(
                        "          <triangle v1=\"{}\" v2=\"{}\" v3=\"{}\"",
                        triangle[0], triangle[1], triangle[2]
                    );
                    // Add color if available
                    if self.options.include_colors {
                        if let Some(color) = object.color {
                            triangle_str.push_str(&format!(" color=\"{}\"", color.to_hex()));
                        } else if let Some(material_id) = object.material_id {
                            triangle_str.push_str(&format!(
                                " pid=\"{}\" p1=\"{}\"",
                                material_id, material_id
                            ));
                        }
                    }
                    triangle_str.push_str("/>\n");
                    xml.push_str(&triangle_str);
                }
            }
            xml.push_str("        </triangles>\n");

            xml.push_str("      </mesh>\n");
            xml.push_str("    </object>\n");
        }

        xml.push_str("  </resources>\n");

        // Generate build instructions
        xml.push_str("  <build>\n");
        for object in objects {
            xml.push_str(&format!("    <item objectid=\"{}\"/>\n", object.id));
        }
        xml.push_str("  </build>\n");

        xml.push_str("</model>\n");
        xml
    }
}

impl Default for ThreeMfExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// 3MF importer
pub struct ThreeMfImporter {
    options: ThreeMfImportOptions,
}

impl ThreeMfImporter {
    pub fn new() -> Self {
        Self {
            options: ThreeMfImportOptions::default(),
        }
    }

    pub fn with_options(options: ThreeMfImportOptions) -> Self {
        Self { options }
    }

    /// Import objects from 3MF format
    pub fn import_objects(&self, input_path: &Path) -> DataExchangeResult<Vec<ThreeMfObject>> {
        // Read ZIP file
        let file = std::fs::File::open(input_path)?;
        let mut archive = zip::ZipArchive::new(file)?;

        // Find and read 3dmodel.model
        let mut model_content = String::new();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let name = file.name();

            if name.ends_with("3dmodel.model") {
                file.read_to_string(&mut model_content)?;
                break;
            }
        }

        if model_content.is_empty() {
            return Err(DataExchangeError::InvalidFormat(
                "3D model not found in 3MF archive".to_string(),
            ));
        }

        self.parse_model(&model_content)
    }

    /// Import a single mesh from 3MF format
    pub fn import_mesh(&self, input_path: &Path) -> DataExchangeResult<Mesh2D> {
        let objects = self.import_objects(input_path)?;

        if objects.is_empty() {
            return Err(DataExchangeError::InvalidFormat(
                "No objects found in 3MF file".to_string(),
            ));
        }

        // Return the first object's mesh
        Ok(objects[0].mesh.clone())
    }

    fn parse_model(&self, content: &str) -> DataExchangeResult<Vec<ThreeMfObject>> {
        let mut objects = Vec::new();
        let mut current_object: Option<ThreeMfObjectBuilder> = None;
        let mut current_mesh: Option<MeshBuilder> = None;

        // Very simplified XML parsing
        // In production, use a proper XML parsing library
        for line in content.lines() {
            let line = line.trim();

            // Parse object start
            if line.starts_with("<object ") {
                let id = self
                    .extract_attribute(line, "id")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);

                current_object = Some(ThreeMfObjectBuilder {
                    id,
                    name: self.extract_attribute(line, "name").map(|s| s.to_string()),
                    color: None,
                    material_id: None,
                });
            }

            // Parse mesh start
            if line == "<mesh>" {
                current_mesh = Some(MeshBuilder::new());
            }

            // Parse vertex
            if line.starts_with("<vertex ") {
                if let Some(ref mut mesh_builder) = current_mesh {
                    if let (Some(x), Some(y), Some(z)) = (
                        self.extract_attribute(line, "x")
                            .and_then(|s| s.parse().ok()),
                        self.extract_attribute(line, "y")
                            .and_then(|s| s.parse().ok()),
                        self.extract_attribute(line, "z")
                            .and_then(|s| s.parse().ok()),
                    ) {
                        mesh_builder.add_vertex(x, y, z);
                    }
                }
            }

            // Parse triangle
            if line.starts_with("<triangle ") {
                if let Some(ref mut mesh_builder) = current_mesh {
                    if let (Some(v1), Some(v2), Some(v3)) = (
                        self.extract_attribute(line, "v1")
                            .and_then(|s| s.parse().ok()),
                        self.extract_attribute(line, "v2")
                            .and_then(|s| s.parse().ok()),
                        self.extract_attribute(line, "v3")
                            .and_then(|s| s.parse().ok()),
                    ) {
                        mesh_builder.add_triangle(v1, v2, v3);
                    }
                }
            }

            // Parse mesh end
            if line == "</mesh>" {
                if let (Some(object_builder), Some(mesh_builder)) =
                    (current_object.take(), current_mesh.take())
                {
                    objects.push(object_builder.build(mesh_builder.build()));
                }
            }
        }

        Ok(objects)
    }

    fn extract_attribute<'a>(&self, line: &'a str, name: &str) -> Option<&'a str> {
        let pattern = format!("{}=\"", name);
        if let Some(start) = line.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = line[start..].find('"') {
                return Some(&line[start..start + end]);
            }
        }
        None
    }
}

impl Default for ThreeMfImporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for 3MF objects during parsing
struct ThreeMfObjectBuilder {
    id: u32,
    name: Option<String>,
    color: Option<ThreeMfColor>,
    material_id: Option<u32>,
}

impl ThreeMfObjectBuilder {
    fn build(self, mesh: Mesh2D) -> ThreeMfObject {
        ThreeMfObject {
            id: self.id,
            name: self.name,
            mesh,
            color: self.color,
            material_id: self.material_id,
        }
    }
}

/// Builder for meshes during parsing
struct MeshBuilder {
    vertices: Vec<(f64, f64, f64)>,
    triangles: Vec<(usize, usize, usize)>,
}

impl MeshBuilder {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
        }
    }

    fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push((x, y, z));
    }

    fn add_triangle(&mut self, v1: usize, v2: usize, v3: usize) {
        self.triangles.push((v1, v2, v3));
    }

    fn build(self) -> Mesh2D {
        let mut mesh = Mesh2D::new();

        // Add vertices
        for (x, y, z) in self.vertices {
            mesh.add_vertex(Point::new(x, y, z));
        }

        // Add triangles
        for (v1, v2, v3) in self.triangles {
            mesh.add_face(v1, v2, v3);
        }

        mesh
    }
}

/// Utility functions for 3MF
pub mod utils {
    use super::*;

    /// Check if a file is a valid 3MF file
    pub fn is_valid_3mf(path: &Path) -> bool {
        if let Ok(file) = std::fs::File::open(path) {
            if let Ok(mut archive) = zip::ZipArchive::new(file) {
                // Check for 3D/3dmodel.model
                for i in 0..archive.len() {
                    if let Ok(file) = archive.by_index(i) {
                        if file.name().ends_with("3dmodel.model") {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    /// Get 3MF file information
    pub fn get_3mf_info(path: &Path) -> Option<ThreeMfFileInfo> {
        if let Ok(file) = std::fs::File::open(path) {
            if let Ok(archive) = zip::ZipArchive::new(file) {
                return Some(ThreeMfFileInfo {
                    compressed_size: archive.len() as u64,
                    entry_count: archive.len(),
                });
            }
        }
        None
    }

    /// Convert mesh to 3MF format with validation
    pub fn convert_to_3mf(
        mesh: &Mesh2D,
        options: &ThreeMfExportOptions,
    ) -> DataExchangeResult<String> {
        let exporter = ThreeMfExporter::with_options(options.clone());
        let object = ThreeMfObject {
            id: 1,
            name: Some("Converted".to_string()),
            mesh: mesh.clone(),
            color: None,
            material_id: None,
        };
        Ok(exporter.generate_model(&[object]))
    }
}

/// 3MF file information
#[derive(Debug, Clone)]
pub struct ThreeMfFileInfo {
    pub compressed_size: u64,
    pub entry_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threemf_export_options_default() {
        let opts = ThreeMfExportOptions::default();
        assert!(opts.include_colors);
        assert_eq!(opts.unit, ThreeMfUnit::Millimeter);
        assert_eq!(opts.compression_level, 6);
    }

    #[test]
    fn test_threemf_exporter_new() {
        let exporter = ThreeMfExporter::new();
        assert!(exporter.options.include_colors);
    }

    #[test]
    fn test_threemf_unit_conversion() {
        assert!((ThreeMfUnit::Millimeter.to_millimeters() - 1.0).abs() < 1e-10);
        assert!((ThreeMfUnit::Centimeter.to_millimeters() - 10.0).abs() < 1e-10);
        assert!((ThreeMfUnit::Inch.to_millimeters() - 25.4).abs() < 1e-10);
    }

    #[test]
    fn test_threemf_color() {
        let color = ThreeMfColor::new(255, 128, 64);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
        assert_eq!(color.to_hex(), "#FF8040FF");
    }

    #[test]
    fn test_threemf_importer_new() {
        let importer = ThreeMfImporter::new();
        assert!(importer.options.load_colors);
        assert!(importer.options.repair_meshes);
    }

    #[test]
    fn test_threemf_version() {
        assert_eq!(ThreeMfVersion::V1_3, ThreeMfVersion::V1_3);
    }

    #[test]
    fn test_mesh_builder() {
        let mut builder = MeshBuilder::new();
        builder.add_vertex(0.0, 0.0, 0.0);
        builder.add_vertex(1.0, 0.0, 0.0);
        builder.add_vertex(0.5, 1.0, 0.0);
        builder.add_triangle(0, 1, 2);

        let mesh = builder.build();
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }
}
