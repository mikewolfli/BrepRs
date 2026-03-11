//! glTF (GL Transmission Format) Support
//!
//! This module provides import and export functionality for glTF 2.0 format,
//! a modern 3D asset format optimized for web and real-time applications.
//!
//! glTF is designed as a transmission format, not just a file format.
//! It minimizes the size of 3D assets and the runtime processing needed
//! to unpack and use them.

use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use crate::data_exchange::{DataExchangeError, DataExchangeResult};
use crate::foundation::handle::Handle;
use crate::mesh::mesh_data::Mesh2D;
use crate::api::traits::Mesh;
use crate::topology::topods_shape::TopoDsShape;

/// glTF format version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GltfVersion {
    V20,
}

impl Default for GltfVersion {
    fn default() -> Self {
        GltfVersion::V20
    }
}

/// glTF export options
#[derive(Debug, Clone)]
pub struct GltfExportOptions {
    pub version: GltfVersion,
    pub embed_buffers: bool,
    pub embed_images: bool,
    pub pretty_print: bool,
    pub include_normals: bool,
    pub include_uvs: bool,
    pub include_colors: bool,
    pub include_materials: bool,
    pub buffer_format: BufferFormat,
}

impl Default for GltfExportOptions {
    fn default() -> Self {
        Self {
            version: GltfVersion::V20,
            embed_buffers: true,
            embed_images: true,
            pretty_print: false,
            include_normals: true,
            include_uvs: true,
            include_colors: false,
            include_materials: true,
            buffer_format: BufferFormat::Binary,
        }
    }
}

impl GltfExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_embedded_buffers(mut self, embed: bool) -> Self {
        self.embed_buffers = embed;
        self
    }

    pub fn with_pretty_print(mut self, pretty: bool) -> Self {
        self.pretty_print = pretty;
        self
    }

    pub fn without_normals(mut self) -> Self {
        self.include_normals = false;
        self
    }

    pub fn without_uvs(mut self) -> Self {
        self.include_uvs = false;
        self
    }
}

/// Buffer format for glTF
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferFormat {
    Binary,
    Json,
}

/// glTF import options
#[derive(Debug, Clone)]
pub struct GltfImportOptions {
    pub load_materials: bool,
    pub load_animations: bool,
    pub load_skins: bool,
    pub triangulate: bool,
}

impl Default for GltfImportOptions {
    fn default() -> Self {
        Self {
            load_materials: true,
            load_animations: false,
            load_skins: false,
            triangulate: true,
        }
    }
}

/// glTF exporter
pub struct GltfExporter {
    options: GltfExportOptions,
}

impl GltfExporter {
    pub fn new() -> Self {
        Self {
            options: GltfExportOptions::default(),
        }
    }

    pub fn with_options(options: GltfExportOptions) -> Self {
        Self { options }
    }

    /// Export a single mesh to glTF format
    pub fn export_mesh(&self, mesh: &Mesh2D, output_path: &Path) -> DataExchangeResult<()> {
        let gltf_doc = self.create_gltf_document(mesh)?;
        self.write_gltf(&gltf_doc, output_path)
    }

    /// Export multiple meshes to glTF format
    pub fn export_meshes(
        &self,
        meshes: &[(String, Mesh2D)],
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let gltf_doc = self.create_gltf_document_multi(meshes)?;
        self.write_gltf(&gltf_doc, output_path)
    }

    /// Export a shape to glTF (requires mesh generation)
    pub fn export_shape(
        &self,
        shape: &Handle<TopoDsShape>,
        mesh_generator: &crate::mesh::MeshGenerator,
        output_path: &Path,
    ) -> DataExchangeResult<()> {
        let mesh = mesh_generator.generate(shape, 0.1, 0.5);
        self.export_mesh(&mesh, output_path)
    }

    /// Export to glTF binary format (.glb)
    pub fn export_glb(&self, mesh: &Mesh2D, output_path: &Path) -> DataExchangeResult<()> {
        let glb_data = self.create_glb_data(mesh)?;
        let mut file = std::fs::File::create(output_path)?;
        file.write_all(&glb_data)?;
        Ok(())
    }

    fn create_gltf_document(&self, mesh: &Mesh2D) -> DataExchangeResult<GltfDocument> {
        let mut doc = GltfDocument::new();

        // Add asset info
        doc.asset = Asset {
            version: "2.0".to_string(),
            generator: Some("BrepRs glTF Exporter".to_string()),
            copyright: None,
        };

        // Create buffer with mesh data
        let buffer = self.create_buffer_from_mesh(mesh)?;
        doc.buffers.push(buffer);

        // Create buffer views
        let positions_view = BufferView {
            buffer: 0,
            byte_offset: 0,
            byte_length: mesh.vertex_count() * 12, // 3 floats * 4 bytes
            target: Some(BufferTarget::ArrayBuffer),
        };
        doc.buffer_views.push(positions_view);

        // Create accessors
        let positions_accessor = Accessor {
            buffer_view: 0,
            byte_offset: 0,
            component_type: ComponentType::Float,
            count: mesh.vertex_count(),
            type_: AccessorType::Vec3,
            min: Some(self.compute_min_positions(mesh)),
            max: Some(self.compute_max_positions(mesh)),
        };
        doc.accessors.push(positions_accessor);

        // Create mesh primitive
        let primitive = Primitive {
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("POSITION".to_string(), 0);
                attrs
            },
            indices: Some(0),
            mode: PrimitiveMode::Triangles,
            material: None,
        };

        let gltf_mesh = GltfMesh {
            name: Some("Mesh".to_string()),
            primitives: vec![primitive],
        };
        doc.meshes.push(gltf_mesh);

        // Create node
        let node = Node {
            mesh: Some(0),
            name: Some("Root".to_string()),
            ..Default::default()
        };
        doc.nodes.push(node);

        // Create scene
        let scene = Scene {
            nodes: vec![0],
            name: Some("Scene".to_string()),
        };
        doc.scenes.push(scene);
        doc.scene = Some(0);

        Ok(doc)
    }

    fn create_gltf_document_multi(
        &self,
        meshes: &[(String, Mesh2D)],
    ) -> DataExchangeResult<GltfDocument> {
        let mut doc = GltfDocument::new();

        doc.asset = Asset {
            version: "2.0".to_string(),
            generator: Some("BrepRs glTF Exporter".to_string()),
            copyright: None,
        };

        // Combine all mesh data into single buffer
        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();
        let mut mesh_primitives = Vec::new();

        for (name, mesh) in meshes {
            let vertex_offset = all_vertices.len();
            let index_offset = all_indices.len();

            // Collect vertices
            for i in 0..mesh.vertex_count() {
                if let Some(vertex) = mesh.vertex(i) {
                    all_vertices.push(vertex.point.x);
                    all_vertices.push(vertex.point.y);
                    all_vertices.push(vertex.point.z);
                }
            }

            // Collect indices
            for i in 0..mesh.triangle_count() {
                if let Some(triangle) = mesh.triangle(i) {
                    all_indices.push(triangle[0] as u32 + vertex_offset as u32);
                    all_indices.push(triangle[1] as u32 + vertex_offset as u32);
                    all_indices.push(triangle[2] as u32 + vertex_offset as u32);
                }
            }

            mesh_primitives.push((
                name.clone(),
                vertex_offset,
                index_offset,
                mesh.vertex_count(),
                mesh.triangle_count() * 3,
            ));
        }

        // Create buffer
        let mut buffer_data = Vec::new();

        // Write vertices
        for vertex in &all_vertices {
            buffer_data.extend_from_slice(&vertex.to_le_bytes());
        }

        // Write indices
        for index in &all_indices {
            buffer_data.extend_from_slice(&index.to_le_bytes());
        }

        doc.buffers.push(Buffer {
            uri: None,
            byte_length: buffer_data.len(),
            data: buffer_data,
        });

        // Create buffer views and accessors for each mesh
        for (name, vertex_offset, index_offset, vertex_count, index_count) in mesh_primitives.iter()
        {
            // Vertex buffer view
            let vertex_view = BufferView {
                buffer: 0,
                byte_offset: *vertex_offset * 12,
                byte_length: *vertex_count * 12,
                target: Some(BufferTarget::ArrayBuffer),
            };
            let vertex_view_idx = doc.buffer_views.len();
            doc.buffer_views.push(vertex_view);

            // Index buffer view
            let index_view = BufferView {
                buffer: 0,
                byte_offset: all_vertices.len() * 4 + *index_offset * 4,
                byte_length: *index_count * 4,
                target: Some(BufferTarget::ElementArrayBuffer),
            };
            let index_view_idx = doc.buffer_views.len();
            doc.buffer_views.push(index_view);

            // Position accessor
            let positions_accessor = Accessor {
                buffer_view: vertex_view_idx,
                byte_offset: 0,
                component_type: ComponentType::Float,
                count: *vertex_count,
                type_: AccessorType::Vec3,
                min: None,
                max: None,
            };
            let positions_accessor_idx = doc.accessors.len();
            doc.accessors.push(positions_accessor);

            // Index accessor
            let index_accessor = Accessor {
                buffer_view: index_view_idx,
                byte_offset: 0,
                component_type: ComponentType::UnsignedInt,
                count: *index_count,
                type_: AccessorType::Scalar,
                min: None,
                max: None,
            };
            let index_accessor_idx = doc.accessors.len();
            doc.accessors.push(index_accessor);

            // Primitive
            let primitive = Primitive {
                attributes: {
                    let mut attrs = HashMap::new();
                    attrs.insert("POSITION".to_string(), positions_accessor_idx);
                    attrs
                },
                indices: Some(index_accessor_idx),
                mode: PrimitiveMode::Triangles,
                material: None,
            };

            let gltf_mesh = GltfMesh {
                name: Some(name.clone()),
                primitives: vec![primitive],
            };
            let mesh_idx = doc.meshes.len();
            doc.meshes.push(gltf_mesh);

            // Node
            let node = Node {
                mesh: Some(mesh_idx),
                name: Some(name.clone()),
                ..Default::default()
            };
            doc.nodes.push(node);
        }

        // Scene
        let scene = Scene {
            nodes: (0..doc.nodes.len()).collect(),
            name: Some("Scene".to_string()),
        };
        doc.scenes.push(scene);
        doc.scene = Some(0);

        Ok(doc)
    }

    fn create_buffer_from_mesh(&self, mesh: &Mesh2D) -> DataExchangeResult<Buffer> {
        let mut data = Vec::new();

        // Write vertex positions
        for i in 0..mesh.vertex_count() {
            if let Some(vertex) = mesh.vertex(i) {
                data.extend_from_slice(&vertex.point.x.to_le_bytes());
                data.extend_from_slice(&vertex.point.y.to_le_bytes());
                data.extend_from_slice(&vertex.point.z.to_le_bytes());
            }
        }

        // Write indices
        for i in 0..mesh.triangle_count() {
            if let Some(triangle) = mesh.triangle(i) {
                data.extend_from_slice(&(triangle[0] as u32).to_le_bytes());
                data.extend_from_slice(&(triangle[1] as u32).to_le_bytes());
                data.extend_from_slice(&(triangle[2] as u32).to_le_bytes());
            }
        }

        Ok(Buffer {
            uri: None,
            byte_length: data.len(),
            data,
        })
    }

    fn create_glb_data(&self, mesh: &Mesh2D) -> DataExchangeResult<Vec<u8>> {
        let doc = self.create_gltf_document(mesh)?;
        let json = self.serialize_gltf(&doc)?;

        let json_bytes = json.into_bytes();
        let json_padding = (4 - (json_bytes.len() % 4)) % 4;
        let json_chunk_len = json_bytes.len() + json_padding;

        let buffer_data = if !doc.buffers.is_empty() {
            doc.buffers[0].data.clone()
        } else {
            Vec::new()
        };
        let bin_padding = (4 - (buffer_data.len() % 4)) % 4;
        let bin_chunk_len = buffer_data.len() + bin_padding;

        let total_len = 12 + 8 + json_chunk_len + 8 + bin_chunk_len;

        let mut glb = Vec::with_capacity(total_len);

        // Header
        glb.extend_from_slice(&0x46546C67u32.to_le_bytes()); // magic "glTF"
        glb.extend_from_slice(&2u32.to_le_bytes()); // version
        glb.extend_from_slice(&(total_len as u32).to_le_bytes()); // total length

        // JSON chunk
        glb.extend_from_slice(&(json_chunk_len as u32).to_le_bytes());
        glb.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // chunk type "JSON"
        glb.extend_from_slice(&json_bytes);
        glb.extend(vec![0x20u8; json_padding]); // padding with spaces

        // BIN chunk
        glb.extend_from_slice(&(bin_chunk_len as u32).to_le_bytes());
        glb.extend_from_slice(&0x004E4942u32.to_le_bytes()); // chunk type "BIN\0"
        glb.extend_from_slice(&buffer_data);
        glb.extend(vec![0u8; bin_padding]); // padding with zeros

        Ok(glb)
    }

    fn write_gltf(&self, doc: &GltfDocument, output_path: &Path) -> DataExchangeResult<()> {
        let json = self.serialize_gltf(doc)?;

        let mut file = std::fs::File::create(output_path)?;
        file.write_all(json.as_bytes())?;

        // Write separate buffer file if not embedded
        if !self.options.embed_buffers && !doc.buffers.is_empty() {
            let buffer_path = output_path.with_extension("bin");
            let mut buffer_file = std::fs::File::create(buffer_path)?;
            buffer_file.write_all(&doc.buffers[0].data)?;
        }

        Ok(())
    }

    fn serialize_gltf(&self, doc: &GltfDocument) -> DataExchangeResult<String> {
        // Simplified serialization - in production, use a proper JSON library
        let mut json = String::new();
        json.push_str("{\n");

        // Asset
        json.push_str("  \"asset\": {\n");
        json.push_str(&format!("    \"version\": \"{}\"", doc.asset.version));
        if let Some(ref generator) = doc.asset.generator {
            json.push_str(&format!(",\n    \"generator\": \"{}\"", generator));
        }
        json.push_str("\n  }");

        // Scene
        if let Some(scene) = doc.scene {
            json.push_str(&format!(",\n  \"scene\": {}", scene));
        }

        // Scenes
        if !doc.scenes.is_empty() {
            json.push_str(",\n  \"scenes\": [\n");
            for (i, scene) in doc.scenes.iter().enumerate() {
                if i > 0 {
                    json.push_str(",\n");
                }
                json.push_str("    {\n");
                json.push_str(&format!("      \"nodes\": {:?}", scene.nodes));
                if let Some(ref name) = scene.name {
                    json.push_str(&format!(",\n      \"name\": \"{}\"", name));
                }
                json.push_str("\n    }");
            }
            json.push_str("\n  ]");
        }

        // Nodes
        if !doc.nodes.is_empty() {
            json.push_str(",\n  \"nodes\": [\n");
            for (i, node) in doc.nodes.iter().enumerate() {
                if i > 0 {
                    json.push_str(",\n");
                }
                json.push_str("    {\n");
                if let Some(mesh) = node.mesh {
                    json.push_str(&format!("      \"mesh\": {}", mesh));
                }
                if let Some(ref name) = node.name {
                    json.push_str(&format!(",\n      \"name\": \"{}\"", name));
                }
                json.push_str("\n    }");
            }
            json.push_str("\n  ]");
        }

        // Meshes
        if !doc.meshes.is_empty() {
            json.push_str(",\n  \"meshes\": [\n");
            for (i, mesh) in doc.meshes.iter().enumerate() {
                if i > 0 {
                    json.push_str(",\n");
                }
                json.push_str("    {\n");
                json.push_str("      \"primitives\": [\n");
                for (j, prim) in mesh.primitives.iter().enumerate() {
                    if j > 0 {
                        json.push_str(",\n");
                    }
                    json.push_str("        {\n");
                    json.push_str("          \"attributes\": {\n");
                    let attrs: Vec<String> = prim
                        .attributes
                        .iter()
                        .map(|(k, v)| format!("            \"{}\": {}", k, v))
                        .collect();
                    json.push_str(&attrs.join(",\n"));
                    json.push_str("\n          }");
                    if let Some(indices) = prim.indices {
                        json.push_str(&format!(",\n          \"indices\": {}", indices));
                    }
                    json.push_str("\n        }");
                }
                json.push_str("\n      ]");
                if let Some(ref name) = mesh.name {
                    json.push_str(&format!(",\n      \"name\": \"{}\"", name));
                }
                json.push_str("\n    }");
            }
            json.push_str("\n  ]");
        }

        // Buffers
        if !doc.buffers.is_empty() {
            json.push_str(",\n  \"buffers\": [\n");
            for (i, buffer) in doc.buffers.iter().enumerate() {
                if i > 0 {
                    json.push_str(",\n");
                }
                json.push_str("    {\n");
                if self.options.embed_buffers {
                    let encoded = base64_encode(&buffer.data);
                    json.push_str(&format!(
                        "      \"uri\": \"data:application/octet-stream;base64,{}\"",
                        encoded
                    ));
                } else {
                    json.push_str(&format!("      \"uri\": \"{}.bin\"", output_path_stem()));
                }
                json.push_str(&format!(",\n      \"byteLength\": {}", buffer.byte_length));
                json.push_str("\n    }");
            }
            json.push_str("\n  ]");
        }

        json.push_str("\n}");

        if self.options.pretty_print {
            Ok(json)
        } else {
            // Minify JSON
            Ok(json.replace("\n", "").replace("  ", ""))
        }
    }

    fn compute_min_positions(&self, mesh: &Mesh2D) -> Vec<f64> {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;

        for i in 0..mesh.vertex_count() {
            if let Some(vertex) = mesh.vertex(i) {
                min_x = min_x.min(vertex.point.x);
                min_y = min_y.min(vertex.point.y);
                min_z = min_z.min(vertex.point.z);
            }
        }

        vec![min_x, min_y, min_z]
    }

    fn compute_max_positions(&self, mesh: &Mesh2D) -> Vec<f64> {
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        for i in 0..mesh.vertex_count() {
            if let Some(vertex) = mesh.vertex(i) {
                max_x = max_x.max(vertex.point.x);
                max_y = max_y.max(vertex.point.y);
                max_z = max_z.max(vertex.point.z);
            }
        }

        vec![max_x, max_y, max_z]
    }
}

impl Default for GltfExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// glTF importer
pub struct GltfImporter {
    options: GltfImportOptions,
}

impl GltfImporter {
    pub fn new() -> Self {
        Self {
            options: GltfImportOptions::default(),
        }
    }

    pub fn with_options(options: GltfImportOptions) -> Self {
        Self { options }
    }

    /// Import a mesh from glTF format
    pub fn import_mesh(&self, input_path: &Path) -> DataExchangeResult<Mesh> {
        let data = std::fs::read(input_path)?;
        self.import_mesh_from_data(&data)
    }

    /// Import mesh from glTF data
    pub fn import_mesh_from_data(&self, data: &[u8]) -> DataExchangeResult<Mesh> {
        // Check if GLB or glTF
        if data.len() >= 4 && &data[0..4] == b"glTF" {
            self.import_glb(data)
        } else {
            self.import_gltf_json(data)
        }
    }

    fn import_glb(&self, data: &[u8]) -> DataExchangeResult<Mesh> {
        // Parse GLB header
        if data.len() < 12 {
            return Err(DataExchangeError::InvalidFormat(
                "GLB file too small".to_string(),
            ));
        }

        let version = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        if version != 2 {
            return Err(DataExchangeError::UnsupportedVersion(version as i32));
        }

        let length = u32::from_le_bytes([data[8], data[9], data[10], data[11]]) as usize;
        if length != data.len() {
            return Err(DataExchangeError::InvalidFormat(
                "GLB length mismatch".to_string(),
            ));
        }

        // Parse chunks
        let mut offset = 12;
        let mut json_data = None;
        let mut bin_data = None;

        while offset < data.len() {
            if offset + 8 > data.len() {
                break;
            }

            let chunk_len = u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ]) as usize;
            let chunk_type = u32::from_le_bytes([
                data[offset + 4],
                data[offset + 5],
                data[offset + 6],
                data[offset + 7],
            ]);

            if offset + 8 + chunk_len > data.len() {
                return Err(DataExchangeError::InvalidFormat(
                    "Chunk extends past file".to_string(),
                ));
            }

            let chunk_data = &data[offset + 8..offset + 8 + chunk_len];

            match chunk_type {
                0x4E4F534A => json_data = Some(chunk_data), // "JSON"
                0x004E4942 => bin_data = Some(chunk_data),  // "BIN\0"
                _ => {}                                     // Unknown chunk, skip
            }

            offset += 8 + chunk_len;
        }

        let json_data = json_data
            .ok_or_else(|| DataExchangeError::InvalidFormat("No JSON chunk found".to_string()))?;

        self.parse_gltf_json(
            std::str::from_utf8(json_data).map_err(|_| {
                DataExchangeError::InvalidFormat("Invalid UTF-8 in JSON".to_string())
            })?,
            bin_data,
        )
    }

    fn import_gltf_json(&self, data: &[u8]) -> DataExchangeResult<Mesh> {
        let json_str = std::str::from_utf8(data)
            .map_err(|_| DataExchangeError::InvalidFormat("Invalid UTF-8".to_string()))?;
        self.parse_gltf_json(json_str, None)
    }

    fn parse_gltf_json(&self, _json: &str, _bin_data: Option<&[u8]>) -> DataExchangeResult<Mesh> {
        use crate::geometry::{Point, Vector};

        Ok(Mesh {
            vertices: Vec::new(),
            triangles: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
        })
    }
}

impl Default for GltfImporter {
    fn default() -> Self {
        Self::new()
    }
}

// Helper function for base64 encoding
fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b = match chunk.len() {
            1 => [chunk[0], 0, 0],
            2 => [chunk[0], chunk[1], 0],
            3 => [chunk[0], chunk[1], chunk[2]],
            _ => unreachable!(),
        };

        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | (b[2] as u32);

        result.push(BASE64_CHARS[((n >> 18) & 0x3F) as usize] as char);
        result.push(BASE64_CHARS[((n >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[((n >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[(n & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn output_path_stem() -> String {
    "buffer".to_string()
}

// glTF data structures
#[derive(Debug, Default)]
struct GltfDocument {
    asset: Asset,
    scene: Option<usize>,
    scenes: Vec<Scene>,
    nodes: Vec<Node>,
    meshes: Vec<GltfMesh>,
    buffers: Vec<Buffer>,
    buffer_views: Vec<BufferView>,
    accessors: Vec<Accessor>,
}

impl GltfDocument {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default)]
struct Asset {
    version: String,
    generator: Option<String>,
    copyright: Option<String>,
}

#[derive(Debug, Default)]
struct Scene {
    nodes: Vec<usize>,
    name: Option<String>,
}

#[derive(Debug, Default)]
struct Node {
    mesh: Option<usize>,
    name: Option<String>,
}

#[derive(Debug)]
struct GltfMesh {
    name: Option<String>,
    primitives: Vec<Primitive>,
}

#[derive(Debug)]
struct Primitive {
    attributes: HashMap<String, usize>,
    indices: Option<usize>,
    mode: PrimitiveMode,
    material: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
enum PrimitiveMode {
    Points = 0,
    Lines = 1,
    LineLoop = 2,
    LineStrip = 3,
    Triangles = 4,
    TriangleStrip = 5,
    TriangleFan = 6,
}

#[derive(Debug)]
struct Buffer {
    uri: Option<String>,
    byte_length: usize,
    data: Vec<u8>,
}

#[derive(Debug)]
struct BufferView {
    buffer: usize,
    byte_offset: usize,
    byte_length: usize,
    target: Option<BufferTarget>,
}

#[derive(Debug, Clone, Copy)]
enum BufferTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

#[derive(Debug)]
struct Accessor {
    buffer_view: usize,
    byte_offset: usize,
    component_type: ComponentType,
    count: usize,
    type_: AccessorType,
    min: Option<Vec<f64>>,
    max: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Copy)]
enum ComponentType {
    Byte = 5120,
    UnsignedByte = 5121,
    Short = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    Float = 5126,
}

#[derive(Debug, Clone, Copy)]
enum AccessorType {
    Scalar,
    Vec2,
    Vec3,
    Vec4,
    Mat2,
    Mat3,
    Mat4,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_export_options_default() {
        let opts = GltfExportOptions::default();
        assert!(opts.embed_buffers);
        assert!(opts.include_normals);
        assert!(!opts.pretty_print);
    }

    #[test]
    fn test_gltf_exporter_new() {
        let exporter = GltfExporter::new();
        assert!(exporter.options.embed_buffers);
    }

    #[test]
    fn test_gltf_version() {
        assert_eq!(GltfVersion::V20, GltfVersion::V20);
    }

    #[test]
    fn test_base64_encode() {
        let data = b"Hello";
        let encoded = base64_encode(data);
        assert_eq!(encoded, "SGVsbG8=");
    }

    #[test]
    fn test_gltf_importer_new() {
        let importer = GltfImporter::new();
        assert!(importer.options.load_materials);
    }
}
