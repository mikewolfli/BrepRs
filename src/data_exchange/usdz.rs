//! USD (Universal Scene Description) Support
//!
//! This module provides import and export functionality for USD format,
//! developed by Pixar for efficient interchange of 3D graphics data.
//!
//! USD provides a rich, common language for defining, packaging, assembling,
//! and editing 3D data, facilitating the use of multiple digital content
//! creation applications.

use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::path::Path;
use zip;

use crate::data_exchange::DataExchangeResult;
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

impl UsdVersion {
    fn as_str(&self) -> &'static str {
        match self {
            UsdVersion::V21_02 => "21.02",
            UsdVersion::V22_03 => "22.03",
            UsdVersion::V23_02 => "23.02",
        }
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

    pub fn from_meters(meters: f64) -> Self {
        if meters >= 1.0 {
            Units::Meters
        } else if meters >= 0.01 {
            Units::Centimeters
        } else if meters >= 0.001 {
            Units::Millimeters
        } else if meters >= 0.0254 {
            Units::Inches
        } else {
            Units::Feet
        }
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
    pub compression_level: Option<u8>,
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
            compression_level: Some(6),
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

    pub fn with_compression(mut self, level: u8) -> Self {
        self.compression_level = Some(level);
        self
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
    pub apply_transforms: bool,
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
            apply_transforms: true,
        }
    }
}

/// USD material definition
#[derive(Debug, Clone)]
pub struct UsdMaterial {
    pub name: String,
    pub diffuse_color: Option<(f32, f32, f32)>,
    pub metallic: Option<f32>,
    pub roughness: Option<f32>,
    pub opacity: Option<f32>,
    pub emissive_color: Option<(f32, f32, f32)>,
    pub texture_paths: HashMap<String, String>,
}

impl Default for UsdMaterial {
    fn default() -> Self {
        Self {
            name: "default_material".to_string(),
            diffuse_color: Some((0.8, 0.8, 0.8)),
            metallic: Some(0.0),
            roughness: Some(0.5),
            opacity: Some(1.0),
            emissive_color: Some((0.0, 0.0, 0.0)),
            texture_paths: HashMap::new(),
        }
    }
}

/// USD transform
#[derive(Debug, Clone)]
pub struct UsdTransform {
    pub translation: (f64, f64, f64),
    pub rotation: (f64, f64, f64), // Euler angles in radians
    pub scale: (f64, f64, f64),
}

impl Default for UsdTransform {
    fn default() -> Self {
        Self {
            translation: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            scale: (1.0, 1.0, 1.0),
        }
    }
}

/// USD exporter
pub struct UsdExporter {
    options: UsdExportOptions,
    materials: Vec<UsdMaterial>,
}

impl UsdExporter {
    pub fn new() -> Self {
        Self {
            options: UsdExportOptions::default(),
            materials: Vec::new(),
        }
    }

    pub fn with_options(options: UsdExportOptions) -> Self {
        Self {
            options,
            materials: Vec::new(),
        }
    }

    /// Add material to export
    pub fn add_material(&mut self, material: UsdMaterial) {
        self.materials.push(material);
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
        let usda_content = self.generate_usda_content(mesh, "Mesh")?;
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
        let usda_content = self.generate_usda_content(mesh, "Mesh")?;

        let compression = self.options.compression_level.unwrap_or(6);
        let compression_method = if compression == 0 {
            zip::CompressionMethod::Stored
        } else {
            zip::CompressionMethod::Deflated
        };

        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::FileOptions::default()
            .compression_method(compression_method);

        zip.start_file("model.usda", options)?;
        zip.write_all(usda_content.as_bytes())?;

        if self.options.include_materials {
            for material in &self.materials {
                let material_content = self.generate_material_content(material)?;
                let mat_name = format!("materials/{}.usda", material.name);
                zip.start_file(&mat_name, options)?;
                zip.write_all(material_content.as_bytes())?;
            }
        }

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
        let usda_content = self.generate_usda_content_multi(meshes)?;

        let compression = self.options.compression_level.unwrap_or(6);
        let compression_method = if compression == 0 {
            zip::CompressionMethod::Stored
        } else {
            zip::CompressionMethod::Deflated
        };

        let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
        let options = zip::write::FileOptions::default()
            .compression_method(compression_method);

        zip.start_file("scene.usda", options)?;
        zip.write_all(usda_content.as_bytes())?;

        if self.options.include_materials {
            for material in &self.materials {
                let material_content = self.generate_material_content(material)?;
                let mat_name = format!("materials/{}.usda", material.name);
                zip.start_file(&mat_name, options)?;
                zip.write_all(material_content.as_bytes())?;
            }
        }

        let zip_data = zip.finish()?.into_inner();

        let mut file = std::fs::File::create(output_path)?;
        file.write_all(&zip_data)?;

        Ok(())
    }

    fn generate_usda_content(&self, mesh: &Mesh, name: &str) -> DataExchangeResult<String> {
        let mut content = String::new();

        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str(&format!("    doc = \"BrepRs USD Export v{}\"\n", self.options.version.as_str()));
        content.push_str(&format!("    defaultPrim = \"{}\"\n", name));
        content.push_str(&format!("    metersPerUnit = {}\n", self.options.units.to_meters()));
        content.push_str(&format!(
            "    upAxis = \"{}\"\n",
            match self.options.up_axis {
                UpAxis::Y => "Y",
                UpAxis::Z => "Z",
            }
        ));
        content.push_str(")\n\n");

        content.push_str(&format!("def Mesh \"{}\"\n", name));
        content.push_str("{\n");

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

        content.push_str("    int[] faceVertexCounts = [\n");
        for i in 0..mesh.triangle_count() {
            if i > 0 {
                content.push_str(", ");
            }
            content.push_str("3");
        }
        content.push_str("\n    ]\n");

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
        }

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

        content.push_str("    uniform token subdivisionScheme = \"none\"\n");
        content.push_str("}\n");

        Ok(content)
    }

    fn generate_usda_content_multi(&self, meshes: &[(String, Mesh)]) -> DataExchangeResult<String> {
        let mut content = String::new();

        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str(&format!("    doc = \"BrepRs USD Export v{}\"\n", self.options.version.as_str()));
        content.push_str("    defaultPrim = \"World\"\n");
        content.push_str(&format!("    metersPerUnit = {}\n", self.options.units.to_meters()));
        content.push_str(&format!(
            "    upAxis = \"{}\"\n",
            match self.options.up_axis {
                UpAxis::Y => "Y",
                UpAxis::Z => "Z",
            }
        ));
        content.push_str(")\n\n");

        content.push_str("def Xform \"World\"\n");
        content.push_str("{\n");

        for (name, mesh) in meshes {
            content.push_str(&format!("    def Mesh \"{}\"\n", name));
            content.push_str("    {\n");

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

            content.push_str("        int[] faceVertexCounts = [\n");
            for i in 0..mesh.triangle_count() {
                if i > 0 {
                    content.push_str(", ");
                }
                content.push_str("3");
            }
            content.push_str("\n        ]\n");

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

    fn generate_material_content(&self, material: &UsdMaterial) -> DataExchangeResult<String> {
        let mut content = String::new();

        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str("    defaultPrim = \"Material\"\n");
        content.push_str(")\n\n");

        content.push_str(&format!("def Material \"{}\"\n", material.name));
        content.push_str("{\n");

        if let Some(color) = material.diffuse_color {
            content.push_str("    def Shader \"PreviewSurface\"\n");
            content.push_str("    {\n");
            content.push_str(&format!(
                "        color3f inputs:diffuseColor = ({}, {}, {})\n",
                color.0, color.1, color.2
            ));
            if let Some(metallic) = material.metallic {
                content.push_str(&format!("        float inputs:metallic = {}\n", metallic));
            }
            if let Some(roughness) = material.roughness {
                content.push_str(&format!("        float inputs:roughness = {}\n", roughness));
            }
            if let Some(opacity) = material.opacity {
                content.push_str(&format!("        float inputs:opacity = {}\n", opacity));
            }
            content.push_str("        token outputs:surface.connect = </Material.outputs:surface>\n");
            content.push_str("    }\n");
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
    materials: HashMap<String, UsdMaterial>,
    transforms: HashMap<String, UsdTransform>,
}

impl UsdImporter {
    pub fn new() -> Self {
        Self {
            options: UsdImportOptions::default(),
            materials: HashMap::new(),
            transforms: HashMap::new(),
        }
    }

    pub fn with_options(options: UsdImportOptions) -> Self {
        Self {
            options,
            materials: HashMap::new(),
            transforms: HashMap::new(),
        }
    }

    /// Get import options
    pub fn options(&self) -> &UsdImportOptions {
        &self.options
    }

    /// Set import options
    pub fn set_options(&mut self, options: UsdImportOptions) {
        self.options = options;
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

    /// Get imported materials
    pub fn materials(&self) -> &HashMap<String, UsdMaterial> {
        &self.materials
    }

    /// Get imported transforms
    pub fn transforms(&self) -> &HashMap<String, UsdTransform> {
        &self.transforms
    }

    fn parse_usda(&self, content: &str) -> DataExchangeResult<Mesh> {
        let mut mesh = Mesh::new();

        let points = self.parse_points_array(content)?;
        let indices = self.parse_face_indices_array(content)?;
        let normals = if self.options.load_materials {
            self.parse_normals_array(content)?
        } else {
            Vec::new()
        };

        for point in points {
            mesh.add_vertex(point);
        }

        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                mesh.add_face(chunk[0], chunk[1], chunk[2]);
            }
        }

        if !normals.is_empty() {
            for (i, normal) in normals.iter().enumerate() {
                if i < mesh.vertices.len() {
                    mesh.vertices[i].normal = Some([normal.x, normal.y, normal.z]);
                }
            }
        }

        Ok(mesh)
    }

    fn parse_usda_multi(&self, content: &str) -> DataExchangeResult<Vec<(String, Mesh)>> {
        let mut meshes = Vec::new();

        let mesh_blocks = self.extract_mesh_blocks(content)?;

        for (name, block_content) in mesh_blocks {
            let mesh = self.parse_single_mesh_block(&block_content)?;
            meshes.push((name, mesh));
        }

        Ok(meshes)
    }

    fn parse_points_array(&self, content: &str) -> DataExchangeResult<Vec<Point>> {
        let mut points = Vec::new();

        if let Some(start) = content.find("point3f[] points = [") {
            let section = &content[start..];
            if let Some(end) = section.find("]") {
                let array_content = &section[21..end];

                for line in array_content.lines() {
                    let line = line.trim();
                    if line.starts_with('(') && line.ends_with(')') {
                        let coords: Vec<&str> = line[1..line.len() - 1].split(',').collect();
                        if coords.len() == 3 {
                            if let (Ok(x), Ok(y), Ok(z)) = (
                                coords[0].trim().parse::<f64>(),
                                coords[1].trim().parse::<f64>(),
                                coords[2].trim().parse::<f64>(),
                            ) {
                                points.push(Point::new(x, y, z));
                            }
                        }
                    }
                }
            }
        }

        Ok(points)
    }

    fn parse_face_indices_array(&self, content: &str) -> DataExchangeResult<Vec<usize>> {
        let mut indices = Vec::new();

        if let Some(start) = content.find("int[] faceVertexIndices = [") {
            let section = &content[start..];
            if let Some(end) = section.find("]") {
                let array_content = &section[27..end];

                for token in array_content.split(|c| c == ',' || c == '\n') {
                    let token = token.trim();
                    if !token.is_empty() {
                        if let Ok(index) = token.parse::<usize>() {
                            indices.push(index);
                        }
                    }
                }
            }
        }

        Ok(indices)
    }

    fn parse_normals_array(&self, content: &str) -> DataExchangeResult<Vec<Vector>> {
        let mut normals = Vec::new();

        if let Some(start) = content.find("normal3f[] normals = [") {
            let section = &content[start..];
            if let Some(end) = section.find("]") {
                let array_content = &section[23..end];

                for line in array_content.lines() {
                    let line = line.trim();
                    if line.starts_with('(') && line.ends_with(')') {
                        let coords: Vec<&str> = line[1..line.len() - 1].split(',').collect();
                        if coords.len() == 3 {
                            if let (Ok(x), Ok(y), Ok(z)) = (
                                coords[0].trim().parse::<f64>(),
                                coords[1].trim().parse::<f64>(),
                                coords[2].trim().parse::<f64>(),
                            ) {
                                normals.push(Vector::new(x, y, z));
                            }
                        }
                    }
                }
            }
        }

        Ok(normals)
    }

    fn extract_mesh_blocks(&self, content: &str) -> DataExchangeResult<Vec<(String, String)>> {
        let mut blocks = Vec::new();

        let mut depth = 0;
        let mut current_block_start = 0;
        let mut current_name = String::new();
        let mut in_mesh = false;

        for (i, c) in content.char_indices() {
            match c {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 && in_mesh {
                        let block_content = content[current_block_start..=i].to_string();
                        blocks.push((current_name.clone(), block_content));
                        in_mesh = false;
                    }
                }
                _ => {}
            }

            if depth == 0 && content[i..].starts_with("def Mesh \"") {
                let mesh_start = i + "def Mesh \"".len();
                if let Some(end) = content[mesh_start..].find('"') {
                    current_name = content[mesh_start..mesh_start + end].to_string();
                    current_block_start = i;
                    in_mesh = true;
                }
            }
        }

        Ok(blocks)
    }

    fn parse_single_mesh_block(&self, block_content: &str) -> DataExchangeResult<Mesh> {
        let mut mesh = Mesh::new();

        let points = self.parse_points_array(block_content)?;
        let indices = self.parse_face_indices_array(block_content)?;

        for point in points {
            mesh.add_vertex(point);
        }

        for chunk in indices.chunks(3) {
            if chunk.len() == 3 {
                mesh.add_face(chunk[0], chunk[1], chunk[2]);
            }
        }

        Ok(mesh)
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

    #[test]
    fn test_usd_material_default() {
        let material = UsdMaterial::default();
        assert_eq!(material.name, "default_material");
        assert!(material.diffuse_color.is_some());
    }

    #[test]
    fn test_usd_transform_default() {
        let transform = UsdTransform::default();
        assert_eq!(transform.translation, (0.0, 0.0, 0.0));
        assert_eq!(transform.scale, (1.0, 1.0, 1.0));
    }
}
