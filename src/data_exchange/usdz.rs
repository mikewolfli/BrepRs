//! USD (Universal Scene Description) Support
//!
//! This module provides import and export functionality for USD format,
//! developed by Pixar for efficient interchange of 3D graphics data.
//!
//! USD provides a rich, common language for defining, packaging, assembling,
//! and editing 3D data, facilitating the use of multiple digital content
//! creation applications.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;
use zip;

use crate::data_exchange::{DataExchangeError, DataExchangeResult};
use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::mesh::Mesh;
use crate::topology::topods_shape::TopoDsShape;

/// USD format version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdVersion {
    V21_02,
    V22_03,
    V23_02,
}

impl Default for UsdVersion {
    fn default() -> Self {
        UsdVersion::V23_02
    }
}

/// USD export options
#[derive(Debug, Clone)]
pub struct UsdExportOptions {
    pub version: UsdVersion,
    pub format: UsdFormat,
    pub include_normals: bool,
    pub include_uvs: bool,
    pub include_materials: bool,
    pub include_cameras: bool,
    pub include_lights: bool,
    pub up_axis: UpAxis,
    pub units: Units,
}

impl Default for UsdExportOptions {
    fn default() -> Self {
        Self {
            version: UsdVersion::V23_02,
            format: UsdFormat::Usdc,
            include_normals: true,
            include_uvs: true,
            include_materials: true,
            include_cameras: false,
            include_lights: false,
            up_axis: UpAxis::Y,
            units: Units::Meters,
        }
    }
}

impl UsdExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_format(mut self, format: UsdFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_up_axis(mut self, axis: UpAxis) -> Self {
        self.up_axis = axis;
        self
    }

    pub fn with_units(mut self, units: Units) -> Self {
        self.units = units;
        self
    }
}
/// USD file format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdFormat {
    /// ASCII format (human-readable)
    Usda,
    /// Binary format (compact, faster)
    Usdc,
    /// Zipped binary format
    Usdz,
}

/// Coordinate system up axis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpAxis {
    Y,
    Z,
}

/// Scene units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Units {
    Meters,
    Centimeters,
    Millimeters,
    Inches,
    Feet,
}

impl Units {
    pub fn to_meters(&self) -> f64 {
        match self {
            Units::Meters => 1.0,
            Units::Centimeters => 0.01,
            Units::Millimeters => 0.001,
            Units::Inches => 0.0254,
            Units::Feet => 0.3048,
        }
    }
}

/// USD import options
#[derive(Debug, Clone)]
pub struct UsdImportOptions {
    pub load_materials: bool,
    pub load_cameras: bool,
    pub load_lights: bool,
    pub load_skeletons: bool,
    pub load_animations: bool,
    pub time_code: Option<f64>,
}

impl Default for UsdImportOptions {
    fn default() -> Self {
        Self {
            load_materials: true,
            load_cameras: false,
            load_lights: false,
            load_skeletons: false,
            load_animations: false,
            time_code: None,
        }
    }
}

/// USD exporter
pub struct UsdExporter {
    options: UsdExportOptions,
}

impl UsdExporter {
    pub fn new() -> Self {
        Self {
            options: UsdExportOptions::default(),
        }
    }

    pub fn with_options(options: UsdExportOptions) -> Self {
        Self { options }
    }

    /// Export a single mesh to USD format
    pub fn export_mesh(&self, mesh: &Mesh, output_path: &Path) -> DataExchangeResult<()> {
        match self.options.format {
            UsdFormat::Usda => self.export_usda(mesh, output_path),
            UsdFormat::Usdc => self.export_usdc(mesh, output_path),
            UsdFormat::Usdz => self.export_usdz(mesh, output_path),
        }
    }

    /// Export multiple meshes to USD format
    pub fn export_meshes(
        &self,
        meshes: &[(String, Mesh)],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        match self.options.format {
            UsdFormat::Usda => self.export_usda_multi(meshes, output_path),
            UsdFormat::Usdc => self.export_usdc_multi(meshes, output_path),
            UsdFormat::Usdz => self.export_usdz_multi(meshes, output_path),
        }
    }

    /// Export a shape to USD (requires mesh generation)
    pub fn export_shape(
        &self,
        shape: &Handle<TopoDsShape>,
        mesh_generator: &crate::mesh::MeshGenerator,
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let mesh = mesh_generator.generate(shape, 0.1, 0.5);
        self.export_mesh(&mesh, output_path)
    }

    fn export_usda(&self, mesh: &Mesh, output_path: &Path) -> DataExchangeResult<()> {
        let content = self.generate_usda_content(mesh, "Mesh")?;
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn export_usda_multi(
        &self,
        meshes: &[(String, Mesh)],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let content = self.generate_usda_content_multi(meshes)?;
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn export_usdc(&self, mesh: &Mesh, output_path: &Path) -> DataExchangeResult<()> {
        // Binary format - simplified implementation
        // In production, use the official USD library or a proper crate
        let usda_content = self.generate_usda_content(mesh, "Mesh")?;

        // For now, just write the ASCII version with .usdc extension
        // A real implementation would convert to binary format
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(usda_content.as_bytes())?;
        Ok(())
    }

    fn export_usdc_multi(
        &self,
        meshes: &[(String, Mesh)],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let usda_content = self.generate_usda_content_multi(meshes)?;
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(usda_content.as_bytes())?;
        Ok(())
    }

    fn export_usdz(&self, mesh: &Mesh, output_path: &Path) -> DataExchangeResult<()> {
        use std::io::Cursor;

        // Create a ZIP archive containing the USD file
        let usda_content = self.generate_usda_content(mesh, "Mesh")?;

        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("mesh.usda", options)?;
        zip.write_all(usda_content.as_bytes())?;

        let zip_data = zip.finish()?.into_inner();

        let mut file = std::fs::File::create(output_path)?;
        file.write_all(&zip_data)?;

        Ok(())
    }

    fn export_usdz_multi(
        &self,
        meshes: &[(String, Mesh)],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        use std::io::Cursor;

        let usda_content = self.generate_usda_content_multi(meshes)?;

        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file("scene.usda", options)?;
        zip.write_all(usda_content.as_bytes())?;

        let zip_data = zip.finish()?.into_inner();

        let mut file = std::fs::File::create(output_path)?;
        file.write_all(&zip_data)?;

        Ok(())
    }

    fn generate_usda_content(&self, mesh: &Mesh, name: &str) -> DataExchangeResult<String> {
        let mut content = String::new();

        // Header
        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str(&format!("    defaultPrim = \"{}\"\n", name));
        content.push_str(&format!(
            "    metersPerUnit = {}\n",
            self.options.units.to_meters()
        ));
        content.push_str(&format!(
            "    upAxis = \"{}\"\n",
            match self.options.up_axis {
                UpAxis::Y => "Y",
                UpAxis::Z => "Z",
            }
        ));
        content.push_str(")\n\n");

        // Mesh prim
        content.push_str(&format!("def Mesh \"{}\"\n", name));
        content.push_str("{\n");

        // Points (vertices)
        content.push_str("    point3f[] points = [\n");
        for i in 0..mesh.vertex_count() {
            if let Some(vertex) = mesh.vertex(i) {
                if i > 0 {
                    content.push_str(",\n");
                }
                content.push_str(&format!(
                    "        ({}, {}, {})",
                    vertex.point.x, vertex.point.y, vertex.point.z
                ));
            }
        }
        content.push_str("\n    ]\n");

        // Face vertex counts
        content.push_str("    int[] faceVertexCounts = [\n");
        for i in 0..mesh.triangle_count() {
            if i > 0 {
                content.push_str(", ");
            }
            content.push_str("3");
        }
        content.push_str("\n    ]\n");

        // Face vertex indices
        content.push_str("    int[] faceVertexIndices = [\n");
        for i in 0..mesh.triangle_count() {
            if let Some(triangle) = mesh.triangle(i) {
                if i > 0 {
                    content.push_str(",\n");
                }
                content.push_str(&format!(
                    "        {}, {}, {}",
                    triangle[0], triangle[1], triangle[2]
                ));
            }
        }
        content.push_str("\n    ]\n");

        // Normals
        if self.options.include_normals {
            content.push_str("    normal3f[] normals = [\n");
            for i in 0..mesh.vertex_count() {
                if let Some(vertex) = mesh.vertex(i) {
                    if i > 0 {
                        content.push_str(",\n");
                    }
                    let normal = vertex.normal.unwrap_or([0.0, 0.0, 1.0]);
                    content.push_str(&format!(
                        "        ({}, {}, {})",
                        normal[0], normal[1], normal[2]
                    ));
                }
            }
            content.push_str("\n    ]\n");
            content.push_str("    uniform token interpolation = \"vertex\"\n");

            if self.options.include_uvs {
                content.push_str("    float2[] primvars:st = [\n");
                for i in 0..mesh.vertex_count() {
                    if let Some(vertex) = mesh.vertex(i) {
                        if i > 0 {
                            content.push_str(",\n");
                        }
                        if let Some(uv) = vertex.uv {
                            content.push_str(&format!("        ({}, {})", uv[0], uv[1]));
                        }
                    }
                }
                content.push_str("\n    ]\n");
                content.push_str("    uniform token primvars:st:interpolation = \"vertex\"\n");
            }
        }

        // Subdivision scheme
        content.push_str("    uniform token subdivisionScheme = \"none\"\n");

        content.push_str("}\n");

        Ok(content)
    }

    fn generate_usda_content_multi(&self, meshes: &[(String, Mesh)]) -> DataExchangeResult<String> {
        let mut content = String::new();

        // Header
        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str("    defaultPrim = \"World\"\n");
        content.push_str(&format!(
            "    metersPerUnit = {}\n",
            self.options.units.to_meters()
        ));
        content.push_str(&format!(
            "    upAxis = \"{}\"\n",
            match self.options.up_axis {
                UpAxis::Y => "Y",
                UpAxis::Z => "Z",
            }
        ));
        content.push_str(")\n\n");

        // World scope
        content.push_str("def Xform \"World\"\n");
        content.push_str("{\n");

        // Add each mesh as a child
        for (name, mesh) in meshes {
            content.push_str(&format!("    def Mesh \"{}\"\n", name));
            content.push_str("    {\n");

            // Points
            content.push_str("        point3f[] points = [\n");
            for i in 0..mesh.vertex_count() {
                if let Some(vertex) = mesh.vertex(i) {
                    if i > 0 {
                        content.push_str(",\n");
                    }
                    content.push_str(&format!(
                        "            ({}, {}, {})",
                        vertex.point.x, vertex.point.y, vertex.point.z
                    ));
                }
            }
            content.push_str("\n        ]\n");

            // Face vertex counts
            content.push_str("        int[] faceVertexCounts = [\n");
            for i in 0..mesh.triangle_count() {
                if i > 0 {
                    content.push_str(", ");
                }
                content.push_str("3");
            }
            content.push_str("\n        ]\n");

            // Face vertex indices
            content.push_str("        int[] faceVertexIndices = [\n");
            for i in 0..mesh.triangle_count() {
                if let Some(triangle) = mesh.triangle(i) {
                    if i > 0 {
                        content.push_str(",\n");
                    }
                    content.push_str(&format!(
                        "            {}, {}, {}",
                        triangle[0], triangle[1], triangle[2]
                    ));
                }
            }
            content.push_str("\n        ]\n");

            // Normals
            if self.options.include_normals {
                content.push_str("        normal3f[] normals = [\n");
                for i in 0..mesh.vertex_count() {
                    if let Some(vertex) = mesh.vertex(i) {
                        if i > 0 {
                            content.push_str(",\n");
                        }
                        let normal = vertex.normal.unwrap_or([0.0, 0.0, 1.0]);
                        content.push_str(&format!(
                            "            ({}, {}, {})",
                            normal[0], normal[1], normal[2]
                        ));
                    }
                }
                content.push_str("\n        ]\n");
            }

            content.push_str("        uniform token subdivisionScheme = \"none\"\n");
            content.push_str("    }\n\n");
        }

        content.push_str("}\n");

        Ok(content)
    }
}

impl Default for UsdExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// USD importer
pub struct UsdImporter {
    options: UsdImportOptions,
}

impl UsdImporter {
    pub fn new() -> Self {
        Self {
            options: UsdImportOptions::default(),
        }
    }

    pub fn with_options(options: UsdImportOptions) -> Self {
        Self { options }
    }

    /// Import a mesh from USD format
    pub fn import_mesh(&self, input_path: &Path) -> DataExchangeResult<Mesh> {
        let content = std::fs::read_to_string(input_path)?;
        self.parse_usda(&content)
    }

    /// Import meshes from USD format
    pub fn import_meshes(&self, input_path: &Path) -> DataExchangeResult<Vec<(String, Mesh)>> {
        let content = std::fs::read_to_string(input_path)?;
        self.parse_usda_multi(&content)
    }

    fn parse_usda(&self, content: &str) -> DataExchangeResult<Mesh> {
        // Simplified USDA parser
        // In production, use a proper USD parsing library

        let mut mesh = Mesh::new();

        // Very basic parsing - look for points array
        if let Some(points_start) = content.find("point3f[] points = [") {
            let points_section = &content[points_start..];
            if let Some(points_end) = points_section.find("]") {
                let points_str = &points_section[20..points_end];

                // Parse points (simplified)
                for line in points_str.lines() {
                    let line = line.trim();
                    if line.starts_with('(') && line.ends_with(')') {
                        let coords: Vec<&str> = line[1..line.len() - 1].split(',').collect();
                        if coords.len() == 3 {
                            if let (Ok(x), Ok(y), Ok(z)) = (
                                coords[0].trim().parse::<f64>(),
                                coords[1].trim().parse::<f64>(),
                                coords[2].trim().parse::<f64>(),
                            ) {
                                mesh.add_vertex(Point::new(x, y, z));
                            }
                        }
                    }
                }
            }
        }

        // Parse face indices
        if let Some(indices_start) = content.find("int[] faceVertexIndices = [") {
            let indices_section = &content[indices_start..];
            if let Some(indices_end) = indices_section.find("]") {
                let indices_str = &indices_section[27..indices_end];

                let indices: Vec<usize> = indices_str
                    .split(|c| c == ',' || c == '\n')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                // Group into triangles
                for chunk in indices.chunks(3) {
                    if chunk.len() == 3 {
                        mesh.add_face(chunk[0], chunk[1], chunk[2]);
                    }
                }
            }
        }

        Ok(mesh)
    }

    fn parse_usda_multi(&self, _content: &str) -> DataExchangeResult<Vec<(String, Mesh)>> {
        // Simplified multi-mesh parsing
        // In production, this would properly parse the USD stage structure
        let meshes = Vec::new();
        Ok(meshes)
    }
}

impl Default for UsdImporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_export_options_default() {
        let opts = UsdExportOptions::default();
        assert!(opts.include_normals);
        assert!(opts.include_uvs);
        assert_eq!(opts.up_axis, UpAxis::Y);
    }

    #[test]
    fn test_usd_exporter_new() {
        let exporter = UsdExporter::new();
        assert!(exporter.options.include_normals);
    }

    #[test]
    fn test_units_conversion() {
        assert!((Units::Meters.to_meters() - 1.0).abs() < 1e-10);
        assert!((Units::Centimeters.to_meters() - 0.01).abs() < 1e-10);
        assert!((Units::Millimeters.to_meters() - 0.001).abs() < 1e-10);
    }

    #[test]
    fn test_up_axis() {
        assert_eq!(UpAxis::Y, UpAxis::Y);
        assert_ne!(UpAxis::Y, UpAxis::Z);
    }

    #[test]
    fn test_usd_importer_new() {
        let importer = UsdImporter::new();
        assert!(importer.options.load_materials);
    }

    #[test]
    fn test_usd_version() {
        assert_eq!(UsdVersion::V23_02, UsdVersion::V23_02);
    }
}
