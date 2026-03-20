use crate::topology::TopoDsShape;
use std::collections::HashMap;
use std::io::Read;

/// Modern file format types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModernFormat {
    /// glTF format
    Gltf,
    /// USD format
    Usd,
    /// 3MF format
    ThreeMF,
    /// Other modern format
    Other(String),
}

/// glTF asset information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfAsset {
    pub version: String,
    pub generator: String,
    pub copyright: Option<String>,
    pub min_version: Option<String>,
}

/// glTF node
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfNode {
    pub name: Option<String>,
    pub mesh: Option<usize>,
    pub children: Vec<usize>,
    pub matrix: Option<[f32; 16]>,
    pub translation: Option<[f32; 3]>,
    pub rotation: Option<[f32; 4]>,
    pub scale: Option<[f32; 3]>,
}

/// glTF mesh
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfMesh {
    pub name: Option<String>,
    pub primitives: Vec<GltfPrimitive>,
}

/// glTF primitive
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfPrimitive {
    pub mode: u32,
    pub attributes: HashMap<String, usize>,
    pub indices: Option<usize>,
    pub material: Option<usize>,
    pub targets: Option<HashMap<String, usize>>,
}

/// glTF material
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfMaterial {
    pub name: Option<String>,
    pub pbr_metallic_roughness: Option<GltfPbrMetallicRoughness>,
    pub emissive_factor: Option<[f32; 3]>,
    pub emissive_texture: Option<GltfTextureInfo>,
    pub alpha_mode: Option<String>,
    pub alpha_cutoff: Option<f32>,
    pub double_sided: Option<bool>,
}

/// glTF PBR metallic roughness
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfPbrMetallicRoughness {
    pub base_color_factor: Option<[f32; 4]>,
    pub base_color_texture: Option<GltfTextureInfo>,
    pub metallic_factor: Option<f32>,
    pub roughness_factor: Option<f32>,
    pub metallic_roughness_texture: Option<GltfTextureInfo>,
}

/// glTF texture info
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfTextureInfo {
    pub index: usize,
    pub tex_coord: Option<u32>,
}

/// glTF texture
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfTexture {
    pub name: Option<String>,
    pub source: Option<usize>,
    pub sampler: Option<usize>,
}

/// glTF image
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfImage {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub buffer_view: Option<usize>,
    pub mime_type: Option<String>,
}

/// glTF buffer view
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfBufferView {
    pub buffer: usize,
    pub byte_offset: Option<u32>,
    pub byte_length: u32,
    pub byte_stride: Option<u32>,
    pub target: Option<u32>,
}

/// glTF buffer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfBuffer {
    pub name: Option<String>,
    pub uri: Option<String>,
    pub byte_length: u32,
    pub data: Option<Vec<u8>>,
}

/// glTF accessor
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfAccessor {
    pub name: Option<String>,
    pub buffer_view: Option<usize>,
    pub byte_offset: Option<u32>,
    pub component_type: u32,
    pub count: u32,
    pub type_: String,
    pub max: Option<Vec<f32>>,
    pub min: Option<Vec<f32>>,
    pub normalized: Option<bool>,
    pub sparse: Option<GltfSparse>,
}

/// glTF sparse
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfSparse {
    pub count: u32,
    pub indices: GltfSparseIndices,
    pub values: GltfSparseValues,
}

/// glTF sparse indices
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfSparseIndices {
    pub buffer_view: usize,
    pub byte_offset: Option<u32>,
    pub component_type: u32,
}

/// glTF sparse values
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfSparseValues {
    pub buffer_view: usize,
    pub byte_offset: Option<u32>,
}

/// glTF sampler
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfSampler {
    pub name: Option<String>,
    pub mag_filter: Option<u32>,
    pub min_filter: Option<u32>,
    pub wrap_s: Option<u32>,
    pub wrap_t: Option<u32>,
}

/// glTF document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfDocument {
    pub asset: GltfAsset,
    pub scenes: Vec<GltfScene>,
    pub nodes: Vec<GltfNode>,
    pub meshes: Vec<GltfMesh>,
    pub materials: Vec<GltfMaterial>,
    pub textures: Vec<GltfTexture>,
    pub images: Vec<GltfImage>,
    pub samplers: Vec<GltfSampler>,
    pub buffer_views: Vec<GltfBufferView>,
    pub buffers: Vec<GltfBuffer>,
    pub accessors: Vec<GltfAccessor>,
    pub extensions_used: Option<Vec<String>>,
    pub extensions_required: Option<Vec<String>>,
}

/// glTF scene
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GltfScene {
    pub name: Option<String>,
    pub nodes: Vec<usize>,
}

/// USD asset information
pub struct UsdAsset {
    pub name: String,
    pub identifier: String,
    pub version: String,
    pub metadata: HashMap<String, String>,
}

/// USD prim
pub struct UsdPrim {
    pub name: String,
    pub type_: String,
    pub properties: HashMap<String, UsdProperty>,
    pub children: Vec<UsdPrim>,
}

/// USD property
pub enum UsdProperty {
    /// Float property
    Float(f32),
    /// Double property
    Double(f64),
    /// Integer property
    Integer(i32),
    /// Boolean property
    Boolean(bool),
    /// String property
    String(String),
    /// Vector2f property
    Vector2f([f32; 2]),
    /// Vector3f property
    Vector3f([f32; 3]),
    /// Vector4f property
    Vector4f([f32; 4]),
    /// Matrix4f property
    Matrix4f([f32; 16]),
    /// Array property
    Array(Vec<UsdProperty>),
}

/// USD document
pub struct UsdDocument {
    pub asset: UsdAsset,
    pub root_prim: UsdPrim,
}

/// 3MF model
pub struct ThreeMFModel {
    pub name: String,
    pub resources: ThreeMFResources,
    pub build: ThreeMFBuild,
    pub metadata: HashMap<String, String>,
}

/// 3MF resources
pub struct ThreeMFResources {
    pub objects: Vec<ThreeMFObject>,
    pub materials: Vec<ThreeMFMaterial>,
}

/// 3MF object
pub struct ThreeMFObject {
    pub id: usize,
    pub name: Option<String>,
    pub type_: String,
    pub mesh: Option<ThreeMFMesh>,
}

/// 3MF mesh
pub struct ThreeMFMesh {
    pub vertices: Vec<[f32; 3]>,
    pub triangles: Vec<[u32; 3]>,
}

/// 3MF material
pub struct ThreeMFMaterial {
    pub id: String,
    pub name: Option<String>,
    pub color: Option<[f32; 4]>,
}

/// 3MF build
pub struct ThreeMFBuild {
    pub items: Vec<ThreeMFItem>,
}

/// 3MF item
pub struct ThreeMFItem {
    pub object_id: usize,
    pub transform: Option<[f32; 16]>,
}

/// Modern format reader
pub trait ModernFormatReader {
    /// Read from file
    fn read_from_file(&mut self, path: &str) -> Result<TopoDsShape, String>;

    /// Read from buffer
    fn read_from_buffer(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String>;

    /// Get format
    fn format(&self) -> ModernFormat;
}

/// Modern format writer
pub trait ModernFormatWriter {
    /// Write to file
    fn write_to_file(&mut self, shape: &TopoDsShape, path: &str) -> Result<(), String>;

    /// Write to buffer
    fn write_to_buffer(&mut self, shape: &TopoDsShape) -> Result<Vec<u8>, String>;

    /// Get format
    fn format(&self) -> ModernFormat;
}

/// glTF reader
pub struct GltfReader {
    pub document: Option<GltfDocument>,
    pub options: GltfReaderOptions,
}

/// glTF reader options
pub struct GltfReaderOptions {
    pub load_textures: bool,
    pub load_materials: bool,
    pub triangulate: bool,
    pub compute_normals: bool,
}

impl Default for GltfReaderOptions {
    fn default() -> Self {
        Self {
            load_textures: true,
            load_materials: true,
            triangulate: true,
            compute_normals: true,
        }
    }
}

impl GltfReader {
    /// Create a new glTF reader
    pub fn new() -> Self {
        Self {
            document: None,
            options: GltfReaderOptions::default(),
        }
    }

    /// Create a new glTF reader with options
    pub fn with_options(options: GltfReaderOptions) -> Self {
        Self {
            document: None,
            options,
        }
    }
}

impl ModernFormatReader for GltfReader {
    fn read_from_file(&mut self, path: &str) -> Result<TopoDsShape, String> {
        // Read file into buffer
        let buffer = std::fs::read(path).map_err(|e| e.to_string())?;
        self.read_from_buffer(&buffer)
    }

    fn read_from_buffer(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Check if it's a binary glTF file (GLB)
        if buffer.len() >= 12 && &buffer[0..4] == b"glTF" {
            self.read_glb(buffer)
        } else {
            // Assume it's a JSON glTF file
            self.read_gltf_json(buffer)
        }
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::Gltf
    }
}

impl GltfReader {
    /// Read GLB (binary glTF) file
    fn read_glb(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Parse GLB header
        if buffer.len() < 12 {
            return Err("Invalid GLB file: too short".to_string());
        }

        // Check magic number
        if &buffer[0..4] != b"glTF" {
            return Err("Invalid GLB file: wrong magic number".to_string());
        }

        // Read version and length
        let version = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        if version != 2 {
            return Err(format!("Unsupported GLB version: {}", version));
        }

        let length = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        if length as usize > buffer.len() {
            return Err("Invalid GLB file: length mismatch".to_string());
        }

        // Read JSON chunk
        let mut offset = 12;
        let json_length = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let json_type = &buffer[offset..offset + 4];
        if json_type != b"JSON" {
            return Err("Invalid GLB file: expected JSON chunk".to_string());
        }
        offset += 4;

        let json_data = &buffer[offset..offset + json_length as usize];
        offset += json_length as usize;
        // Align to 4-byte boundary
        offset = (offset + 3) & !3;

        // Parse JSON
        let document: GltfDocument =
            serde_json::from_slice(json_data).map_err(|e| e.to_string())?;
        self.document = Some(document);

        // Read BIN chunk if present
        if offset + 8 <= buffer.len() {
            let bin_length = u32::from_le_bytes(buffer[offset..offset + 4].try_into().unwrap());
            offset += 4;

            let bin_type = &buffer[offset..offset + 4];
            if bin_type == b"BIN " {
                offset += 4;
                let bin_data = &buffer[offset..offset + bin_length as usize];
                // Store binary data in the first buffer
                if let Some(doc) = &mut self.document {
                    if !doc.buffers.is_empty() {
                        doc.buffers[0].data = Some(bin_data.to_vec());
                    }
                }
            }
        }

        // Convert to TopoDsShape
        self.convert_to_shape()
    }

    /// Read JSON glTF file
    fn read_gltf_json(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Parse JSON
        let document: GltfDocument = serde_json::from_slice(buffer).map_err(|e| e.to_string())?;
        self.document = Some(document);

        // Load external buffers and textures
        self.load_external_resources()?;

        // Convert to TopoDsShape
        self.convert_to_shape()
    }

    /// Load external resources
    fn load_external_resources(&mut self) -> Result<(), String> {
        // Implementation for loading external buffers and textures
        // For now, we'll just return Ok
        Ok(())
    }

    /// Convert glTF document to TopoDsShape
    fn convert_to_shape(&self) -> Result<TopoDsShape, String> {
        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();

        if let Some(doc) = &self.document {
            for scene in &doc.scenes {
                for node_idx in &scene.nodes {
                    if let Some(node) = doc.nodes.get(*node_idx) {
                        self.process_node(node, &mut compound, doc)?;
                    }
                }
            }
        }

        Ok(compound.shape().clone())
    }

    /// Process a glTF node
    fn process_node(
        &self,
        node: &GltfNode,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
        doc: &GltfDocument,
    ) -> Result<(), String> {
        if let Some(mesh_idx) = node.mesh {
            if let Some(mesh) = doc.meshes.get(mesh_idx) {
                self.process_mesh(mesh, compound)?;
            }
        }

        for child_idx in &node.children {
            if let Some(child) = doc.nodes.get(*child_idx) {
                self.process_node(child, compound, doc)?;
            }
        }

        Ok(())
    }

    /// Process a glTF mesh
    fn process_mesh(
        &self,
        mesh: &GltfMesh,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        for primitive in &mesh.primitives {
            self.convert_primitive(primitive, compound)?;
        }

        Ok(())
    }

    /// Process a glTF primitive
    fn convert_primitive(
        &self,
        primitive: &GltfPrimitive,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        let doc = match &self.document {
            Some(d) => d,
            None => return Err("No document loaded".to_string()),
        };

        let position_accessor_idx = match primitive.attributes.get("POSITION") {
            Some(&idx) => idx,
            None => return Err("Primitive has no POSITION attribute".to_string()),
        };

        let (vertices, _min, _max) = self.read_accessor_data_f32(doc, position_accessor_idx)?;

        let points: Vec<crate::geometry::Point> = vertices
            .chunks_exact(3)
            .map(|chunk| {
                crate::geometry::Point::new(chunk[0] as f64, chunk[1] as f64, chunk[2] as f64)
            })
            .collect();

        let faces: Vec<Vec<usize>> = if let Some(indices_accessor_idx) = primitive.indices {
            let (indices, _, _) = self.read_accessor_data_u32(doc, indices_accessor_idx)?;

            let mode = primitive.mode;
            match mode {
                0 => indices
                    .chunks_exact(1)
                    .map(|chunk| vec![chunk[0] as usize])
                    .collect(),
                1 => indices
                    .chunks_exact(2)
                    .map(|chunk| vec![chunk[0] as usize, chunk[1] as usize])
                    .collect(),
                2 => indices
                    .chunks_exact(3)
                    .map(|chunk| vec![chunk[0] as usize, chunk[1] as usize, chunk[2] as usize])
                    .collect(),
                3 => {
                    let mut result = Vec::new();
                    let mut i = 0;
                    while i + 2 < indices.len() {
                        result.push(vec![
                            indices[i] as usize,
                            indices[i + 1] as usize,
                            indices[i + 2] as usize,
                        ]);
                        i += 3;
                    }
                    result
                }
                4 => {
                    let mut result = Vec::new();
                    let mut i = 0;
                    while i + 2 < indices.len() {
                        result.push(vec![
                            indices[i] as usize,
                            indices[i + 1] as usize,
                            indices[i + 2] as usize,
                        ]);
                        i += 1;
                    }
                    result
                }
                5 => {
                    let mut result = Vec::new();
                    let n = indices.len();
                    if n >= 3 {
                        for i in 1..n - 1 {
                            result.push(vec![
                                indices[0] as usize,
                                indices[i] as usize,
                                indices[i + 1] as usize,
                            ]);
                        }
                    }
                    result
                }
                6 => {
                    let mut result = Vec::new();
                    let mut i = 0;
                    while i + 2 < indices.len() {
                        result.push(vec![
                            indices[i] as usize,
                            indices[i + 1] as usize,
                            indices[i + 2] as usize,
                        ]);
                        i += 1;
                    }
                    result
                }
                _ => indices
                    .chunks_exact(3)
                    .map(|chunk| vec![chunk[0] as usize, chunk[1] as usize, chunk[2] as usize])
                    .collect(),
            }
        } else {
            let vertex_count = points.len();
            let mode = primitive.mode;
            match mode {
                0..=2 => {
                    let mut result = Vec::new();
                    for i in (0..vertex_count).step_by(3) {
                        if i + 2 < vertex_count {
                            result.push(vec![i, i + 1, i + 2]);
                        }
                    }
                    result
                }
                _ => {
                    let mut result = Vec::new();
                    for i in (0..vertex_count).step_by(3) {
                        if i + 2 < vertex_count {
                            result.push(vec![i, i + 1, i + 2]);
                        }
                    }
                    result
                }
            }
        };

        if points.is_empty() || faces.is_empty() {
            return Ok(());
        }

        let solid = crate::topology::topods_solid::TopoDsSolid::from_mesh(points, faces);
        compound.add_component(crate::foundation::handle::Handle::new(std::sync::Arc::new(
            solid.shape().clone(),
        )));

        Ok(())
    }

    /// Read accessor data as f32
    fn read_accessor_data_f32(
        &self,
        doc: &GltfDocument,
        accessor_idx: usize,
    ) -> Result<(Vec<f32>, Option<Vec<f32>>, Option<Vec<f32>>), String> {
        let accessor = doc
            .accessors
            .get(accessor_idx)
            .ok_or_else(|| format!("Accessor {} not found", accessor_idx))?;

        let buffer_view_idx = accessor
            .buffer_view
            .ok_or_else(|| format!("Accessor {} has no buffer view", accessor_idx))?;

        let buffer_view = doc
            .buffer_views
            .get(buffer_view_idx)
            .ok_or_else(|| format!("Buffer view {} not found", buffer_view_idx))?;

        let buffer = doc
            .buffers
            .get(buffer_view.buffer)
            .ok_or_else(|| format!("Buffer {} not found", buffer_view.buffer))?;

        let data = buffer
            .data
            .as_ref()
            .ok_or_else(|| format!("Buffer {} has no data", buffer_view.buffer))?;

        let byte_offset = accessor.byte_offset.unwrap_or(0) as usize;
        let view_offset = buffer_view.byte_offset.unwrap_or(0) as usize;
        let total_offset = view_offset + byte_offset;

        let component_size = match accessor.component_type {
            5120 => 1,
            5121 => 1,
            5122 => 2,
            5123 => 2,
            5125 => 4,
            5126 => 4,
            _ => {
                return Err(format!(
                    "Unknown component type: {}",
                    accessor.component_type
                ))
            }
        };

        let num_components = match accessor.type_.as_str() {
            "SCALAR" => 1,
            "VEC2" => 2,
            "VEC3" => 3,
            "VEC4" => 4,
            "MAT2" => 4,
            "MAT3" => 9,
            "MAT4" => 16,
            _ => return Err(format!("Unknown accessor type: {}", accessor.type_)),
        };

        let stride = buffer_view
            .byte_stride
            .unwrap_or((component_size * num_components) as u32) as usize;
        let _element_size = component_size * num_components;
        let count = accessor.count as usize;

        let mut result = Vec::with_capacity(count * num_components);

        for i in 0..count {
            let offset = total_offset + i * stride;
            for j in 0..num_components {
                let component_offset = offset + j * component_size;
                if component_offset + component_size > data.len() {
                    return Err("Buffer data out of bounds".to_string());
                }

                let value = match accessor.component_type {
                    5120 => {
                        let bytes = [data[component_offset]];
                        f32::from(i8::from_le_bytes(bytes)) as f32
                    }
                    5121 => {
                        let bytes = [data[component_offset]];
                        f32::from(u8::from_le_bytes(bytes))
                    }
                    5122 => {
                        let bytes = [data[component_offset], data[component_offset + 1]];
                        f32::from(i16::from_le_bytes(bytes))
                    }
                    5123 => {
                        let bytes = [data[component_offset], data[component_offset + 1]];
                        f32::from(u16::from_le_bytes(bytes))
                    }
                    5125 => {
                        let bytes = [
                            data[component_offset],
                            data[component_offset + 1],
                            data[component_offset + 2],
                            data[component_offset + 3],
                        ];
                        f32::from_bits(u32::from_le_bytes(bytes))
                    }
                    5126 => {
                        let bytes = [
                            data[component_offset],
                            data[component_offset + 1],
                            data[component_offset + 2],
                            data[component_offset + 3],
                        ];
                        f32::from_bits(u32::from_le_bytes(bytes))
                    }
                    _ => 0.0,
                };
                result.push(value);
            }
        }

        Ok((result, accessor.min.clone(), accessor.max.clone()))
    }

    /// Read accessor data as u32
    fn read_accessor_data_u32(
        &self,
        doc: &GltfDocument,
        accessor_idx: usize,
    ) -> Result<(Vec<u32>, Option<Vec<f32>>, Option<Vec<f32>>), String> {
        let accessor = doc
            .accessors
            .get(accessor_idx)
            .ok_or_else(|| format!("Accessor {} not found", accessor_idx))?;

        let buffer_view_idx = accessor
            .buffer_view
            .ok_or_else(|| format!("Accessor {} has no buffer view", accessor_idx))?;

        let buffer_view = doc
            .buffer_views
            .get(buffer_view_idx)
            .ok_or_else(|| format!("Buffer view {} not found", buffer_view_idx))?;

        let buffer = doc
            .buffers
            .get(buffer_view.buffer)
            .ok_or_else(|| format!("Buffer {} not found", buffer_view.buffer))?;

        let data = buffer
            .data
            .as_ref()
            .ok_or_else(|| format!("Buffer {} has no data", buffer_view.buffer))?;

        let byte_offset = accessor.byte_offset.unwrap_or(0) as usize;
        let view_offset = buffer_view.byte_offset.unwrap_or(0) as usize;
        let total_offset = view_offset + byte_offset;

        let component_size = match accessor.component_type {
            5121 => 1,
            5123 => 2,
            5125 => 4,
            _ => {
                return Err(format!(
                    "Unsupported index component type: {}",
                    accessor.component_type
                ))
            }
        };

        let count = accessor.count as usize;
        let mut result = Vec::with_capacity(count);

        for i in 0..count {
            let offset = total_offset + i * component_size;
            if offset + component_size > data.len() {
                return Err("Buffer data out of bounds".to_string());
            }

            let value = match accessor.component_type {
                5121 => data[offset] as u32,
                5123 => {
                    let bytes = [data[offset], data[offset + 1]];
                    u16::from_le_bytes(bytes) as u32
                }
                5125 => {
                    let bytes = [
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ];
                    u32::from_le_bytes(bytes)
                }
                _ => 0,
            };
            result.push(value);
        }

        Ok((result, accessor.min.clone(), accessor.max.clone()))
    }
}

/// glTF writer
pub struct GltfWriter {
    pub document: GltfDocument,
    pub options: GltfWriterOptions,
}

/// glTF writer options
pub struct GltfWriterOptions {
    pub embed_buffers: bool,
    pub embed_textures: bool,
    pub use_draco_compression: bool,
    pub export_materials: bool,
    pub export_textures: bool,
    pub export_normals: bool,
    pub export_uvs: bool,
}

impl Default for GltfWriterOptions {
    fn default() -> Self {
        Self {
            embed_buffers: true,
            embed_textures: true,
            use_draco_compression: false,
            export_materials: true,
            export_textures: true,
            export_normals: true,
            export_uvs: true,
        }
    }
}

impl GltfWriter {
    /// Create a new glTF writer
    pub fn new() -> Self {
        Self {
            document: GltfDocument {
                asset: GltfAsset {
                    version: "2.0".to_string(),
                    generator: "BrepRs".to_string(),
                    copyright: None,
                    min_version: None,
                },
                scenes: Vec::new(),
                nodes: Vec::new(),
                meshes: Vec::new(),
                materials: Vec::new(),
                textures: Vec::new(),
                images: Vec::new(),
                samplers: Vec::new(),
                buffer_views: Vec::new(),
                buffers: Vec::new(),
                accessors: Vec::new(),
                extensions_used: None,
                extensions_required: None,
            },
            options: GltfWriterOptions::default(),
        }
    }

    /// Create a new glTF writer with options
    pub fn with_options(options: GltfWriterOptions) -> Self {
        Self {
            document: GltfDocument {
                asset: GltfAsset {
                    version: "2.0".to_string(),
                    generator: "BrepRs".to_string(),
                    copyright: None,
                    min_version: None,
                },
                scenes: Vec::new(),
                nodes: Vec::new(),
                meshes: Vec::new(),
                materials: Vec::new(),
                textures: Vec::new(),
                images: Vec::new(),
                samplers: Vec::new(),
                buffer_views: Vec::new(),
                buffers: Vec::new(),
                accessors: Vec::new(),
                extensions_used: None,
                extensions_required: None,
            },
            options,
        }
    }
}

impl ModernFormatWriter for GltfWriter {
    fn write_to_file(&mut self, shape: &TopoDsShape, path: &str) -> Result<(), String> {
        // Convert shape to glTF document
        self.convert_from_shape(shape)?;

        // Check if we should write as GLB
        let is_glb = path.ends_with(".glb");

        if is_glb {
            let buffer = self.write_to_glb()?;
            std::fs::write(path, buffer).map_err(|e| e.to_string())
        } else {
            let buffer = self.write_to_gltf_json()?;
            std::fs::write(path, buffer).map_err(|e| e.to_string())
        }
    }

    fn write_to_buffer(&mut self, shape: &TopoDsShape) -> Result<Vec<u8>, String> {
        // Convert shape to glTF document
        self.convert_from_shape(shape)?;

        // Write as GLB by default
        self.write_to_glb()
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::Gltf
    }
}

impl GltfWriter {
    /// Convert TopoDsShape to glTF document
    fn convert_from_shape(&mut self, _shape: &TopoDsShape) -> Result<(), String> {
        // Implementation for converting TopoDsShape to glTF document
        // For now, we'll just return Ok
        Ok(())
    }

    /// Write glTF as JSON
    fn write_to_gltf_json(&self) -> Result<Vec<u8>, String> {
        // Serialize to JSON
        let json = serde_json::to_vec_pretty(&self.document).map_err(|e| e.to_string())?;
        Ok(json)
    }

    /// Write glTF as GLB (binary format)
    fn write_to_glb(&self) -> Result<Vec<u8>, String> {
        // Serialize JSON
        let json = serde_json::to_vec(&self.document).map_err(|e| e.to_string())?;

        // Calculate buffer sizes
        let json_length = json.len();
        let json_padded = (json_length + 3) & !3;

        // Get binary data
        let bin_data = self
            .document
            .buffers
            .first()
            .and_then(|buf| buf.data.clone())
            .unwrap_or_default();
        let bin_length = bin_data.len();
        let bin_padded = (bin_length + 3) & !3;

        // Calculate total length
        let total_length = 12 + 8 + json_padded + 8 + bin_padded;

        // Create GLB buffer
        let mut buffer = Vec::with_capacity(total_length);

        // Write GLB header
        buffer.extend_from_slice(b"glTF");
        buffer.extend_from_slice(&2u32.to_le_bytes());
        buffer.extend_from_slice(&total_length.to_le_bytes());

        // Write JSON chunk
        buffer.extend_from_slice(&json_length.to_le_bytes());
        buffer.extend_from_slice(b"JSON");
        buffer.extend_from_slice(&json);
        // Pad to 4-byte boundary
        while buffer.len() % 4 != 0 {
            buffer.push(0);
        }

        // Write BIN chunk if there's data
        if !bin_data.is_empty() {
            buffer.extend_from_slice(&bin_length.to_le_bytes());
            buffer.extend_from_slice(b"BIN ");
            buffer.extend_from_slice(&bin_data);
            // Pad to 4-byte boundary
            while buffer.len() % 4 != 0 {
                buffer.push(0);
            }
        }

        Ok(buffer)
    }
}

/// USD reader
pub struct UsdReader {
    pub document: Option<UsdDocument>,
    pub options: UsdReaderOptions,
}

/// USD reader options
pub struct UsdReaderOptions {
    pub load_materials: bool,
    pub load_textures: bool,
    pub triangulate: bool,
    pub compute_normals: bool,
}

impl Default for UsdReaderOptions {
    fn default() -> Self {
        Self {
            load_materials: true,
            load_textures: true,
            triangulate: true,
            compute_normals: true,
        }
    }
}

impl UsdReader {
    /// Create a new USD reader
    pub fn new() -> Self {
        Self {
            document: None,
            options: UsdReaderOptions::default(),
        }
    }

    /// Create a new USD reader with options
    pub fn with_options(options: UsdReaderOptions) -> Self {
        Self {
            document: None,
            options,
        }
    }
}

impl ModernFormatReader for UsdReader {
    fn read_from_file(&mut self, path: &str) -> Result<TopoDsShape, String> {
        // Read file into buffer
        let buffer = std::fs::read(path).map_err(|e| e.to_string())?;
        self.read_from_buffer(&buffer)
    }

    fn read_from_buffer(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Check if it's a binary USD file (USDC)
        if buffer.len() >= 4 && &buffer[0..4] == b"usdc" {
            self.read_usdc(buffer)
        } else {
            // Assume it's an ASCII USD file (USDA)
            self.read_usda(buffer)
        }
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::Usd
    }
}

impl UsdReader {
    /// Read USDC (binary USD) file
    fn read_usdc(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        if buffer.len() < 8 {
            return Err("USDC file too short".to_string());
        }

        if &buffer[0..4] != b"usdc" {
            return Err("Invalid USDC magic number".to_string());
        }

        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();

        let mut offset = 8;
        while offset + 12 < buffer.len() {
            let path_length = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize;
            offset += 4;

            let num_fields = u32::from_le_bytes([
                buffer[offset],
                buffer[offset + 1],
                buffer[offset + 2],
                buffer[offset + 3],
            ]) as usize;
            offset += 4;

            let _spec_type = buffer[offset];
            offset += 1;

            if path_length > 0 && offset + path_length <= buffer.len() {
                offset += path_length;
            }

            for _ in 0..num_fields {
                if offset + 8 > buffer.len() {
                    break;
                }
                let field_name_len = u32::from_le_bytes([
                    buffer[offset],
                    buffer[offset + 1],
                    buffer[offset + 2],
                    buffer[offset + 3],
                ]) as usize;
                offset += 4;
                let value_type = u32::from_le_bytes([
                    buffer[offset],
                    buffer[offset + 1],
                    buffer[offset + 2],
                    buffer[offset + 3],
                ]);
                offset += 4;

                if field_name_len > 0 && offset + field_name_len <= buffer.len() {
                    offset += field_name_len;
                }

                match value_type {
                    0 => {
                        if offset + 4 <= buffer.len() {
                            offset += 4;
                        }
                    }
                    1 => {
                        if offset + 8 <= buffer.len() {
                            offset += 8;
                        }
                    }
                    2 => {
                        if offset + 4 <= buffer.len() {
                            let array_len = u32::from_le_bytes([
                                buffer[offset],
                                buffer[offset + 1],
                                buffer[offset + 2],
                                buffer[offset + 3],
                            ]) as usize;
                            offset += 4;
                            if offset + array_len * 12 <= buffer.len() {
                                let mut points = Vec::new();
                                for i in 0..array_len {
                                    let x = f32::from_le_bytes([
                                        buffer[offset + i * 12],
                                        buffer[offset + i * 12 + 1],
                                        buffer[offset + i * 12 + 2],
                                        buffer[offset + i * 12 + 3],
                                    ]);
                                    let y = f32::from_le_bytes([
                                        buffer[offset + i * 12 + 4],
                                        buffer[offset + i * 12 + 5],
                                        buffer[offset + i * 12 + 6],
                                        buffer[offset + i * 12 + 7],
                                    ]);
                                    let z = f32::from_le_bytes([
                                        buffer[offset + i * 12 + 8],
                                        buffer[offset + i * 12 + 9],
                                        buffer[offset + i * 12 + 10],
                                        buffer[offset + i * 12 + 11],
                                    ]);
                                    points.push(crate::geometry::Point::new(
                                        x as f64, y as f64, z as f64,
                                    ));
                                }
                                if points.len() >= 3 {
                                    let mut faces = Vec::new();
                                    for i in (0..points.len()).step_by(3) {
                                        if i + 2 < points.len() {
                                            faces.push(vec![i, i + 1, i + 2]);
                                        }
                                    }
                                    let solid =
                                        crate::topology::topods_solid::TopoDsSolid::from_mesh(
                                            points, faces,
                                        );
                                    compound.add_component(crate::foundation::handle::Handle::new(
                                        std::sync::Arc::new(solid.shape().clone()),
                                    ));
                                }
                                offset += array_len * 12;
                            }
                        }
                    }
                    3 => {
                        if offset + 4 <= buffer.len() {
                            let array_len = u32::from_le_bytes([
                                buffer[offset],
                                buffer[offset + 1],
                                buffer[offset + 2],
                                buffer[offset + 3],
                            ]) as usize;
                            offset += 4 + array_len * 4;
                        }
                    }
                    _ => {
                        offset += 4;
                    }
                }
            }
        }

        Ok(compound.shape().clone())
    }

    /// Read USDA (ASCII USD) file
    fn read_usda(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Convert buffer to string
        let content = String::from_utf8_lossy(buffer);

        // Parse USDA content
        let document = self.parse_usda(&content)?;
        self.document = Some(document);

        // Convert to TopoDsShape
        self.convert_to_shape()
    }

    /// Parse USDA content
    fn parse_usda(&self, content: &str) -> Result<UsdDocument, String> {
        let asset = UsdAsset {
            name: "BrepRs Import".to_string(),
            identifier: "BrepRs".to_string(),
            version: "0.8.0".to_string(),
            metadata: HashMap::new(),
        };

        let mut root_prim = UsdPrim {
            name: "Root".to_string(),
            type_: "Xform".to_string(),
            properties: HashMap::new(),
            children: Vec::new(),
        };

        let mut prim_stack: Vec<UsdPrim> = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Parse asset information
            if trimmed.starts_with("customData") || trimmed.starts_with("doc") {
                continue;
            }

            // Parse metadata
            if trimmed.contains("assetInfo") || trimmed.contains("defaultPrim") {
                continue;
            }

            // Parse prim definitions
            if trimmed.starts_with("def ") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 {
                    root_prim.type_ = parts[1].to_string();
                    root_prim.name = parts[2].trim_matches('"').to_string();
                }
            }

            // Parse nested prims
            if trimmed.starts_with('{') {
                prim_stack.push(std::mem::replace(
                    &mut root_prim,
                    UsdPrim {
                        name: "Child".to_string(),
                        type_: "Xform".to_string(),
                        properties: HashMap::new(),
                        children: Vec::new(),
                    },
                ));
            }

            if trimmed.starts_with('}') {
                if let Some(parent) = prim_stack.pop() {
                    let child = std::mem::replace(&mut root_prim, parent);
                    root_prim.children.push(child);
                }
            }

            // Parse properties
            if trimmed.contains('=') {
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let prop_name = parts[0].trim();
                    let prop_value = parts[1].trim();

                    // Try to parse different property types
                    if let Ok(value) = prop_value.parse::<f32>() {
                        root_prim
                            .properties
                            .insert(prop_name.to_string(), UsdProperty::Float(value));
                    } else if let Ok(value) = prop_value.parse::<f64>() {
                        root_prim
                            .properties
                            .insert(prop_name.to_string(), UsdProperty::Double(value));
                    } else if let Ok(value) = prop_value.parse::<i32>() {
                        root_prim
                            .properties
                            .insert(prop_name.to_string(), UsdProperty::Integer(value));
                    } else if let Ok(value) = prop_value.parse::<bool>() {
                        root_prim
                            .properties
                            .insert(prop_name.to_string(), UsdProperty::Boolean(value));
                    } else if prop_value.starts_with('"') {
                        let value = prop_value.trim_matches('"').to_string();
                        root_prim
                            .properties
                            .insert(prop_name.to_string(), UsdProperty::String(value));
                    } else if prop_value.starts_with('(') && prop_value.ends_with(')') {
                        // Parse vector
                        let inner = prop_value.trim_matches('(').trim_matches(')');
                        let coords: Vec<f32> = inner
                            .split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect();

                        if coords.len() == 2 {
                            root_prim.properties.insert(
                                prop_name.to_string(),
                                UsdProperty::Vector2f([coords[0], coords[1]]),
                            );
                        } else if coords.len() == 3 {
                            root_prim.properties.insert(
                                prop_name.to_string(),
                                UsdProperty::Vector3f([coords[0], coords[1], coords[2]]),
                            );
                        } else if coords.len() == 4 {
                            root_prim.properties.insert(
                                prop_name.to_string(),
                                UsdProperty::Vector4f([coords[0], coords[1], coords[2], coords[3]]),
                            );
                        }
                    }
                }
            }
        }

        Ok(UsdDocument { asset, root_prim })
    }

    /// Convert USD document to TopoDsShape
    fn convert_to_shape(&self) -> Result<TopoDsShape, String> {
        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();

        if let Some(document) = &self.document {
            self.process_usd_document(document, &mut compound)?;
        }

        Ok(compound.shape().clone())
    }

    /// Process USD document
    fn process_usd_document(
        &self,
        document: &UsdDocument,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        self.process_usd_prim(&document.root_prim, compound)?;

        Ok(())
    }

    /// Process USD prim
    fn process_usd_prim(
        &self,
        prim: &UsdPrim,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        match prim.type_.as_str() {
            "Mesh" | "GeomMesh" => {
                let mut points = Vec::new();
                let mut face_vertex_counts = Vec::new();
                let mut face_vertex_indices = Vec::new();

                if let Some(UsdProperty::Array(arr)) = prim.properties.get("points") {
                    for prop in arr {
                        if let UsdProperty::Vector3f(v) = prop {
                            points.push(crate::geometry::Point::new(
                                v[0] as f64,
                                v[1] as f64,
                                v[2] as f64,
                            ));
                        }
                    }
                }

                if let Some(UsdProperty::Array(arr)) = prim.properties.get("faceVertexCounts") {
                    for prop in arr {
                        if let UsdProperty::Integer(v) = prop {
                            face_vertex_counts.push(*v as usize);
                        }
                    }
                }

                if let Some(UsdProperty::Array(arr)) = prim.properties.get("faceVertexIndices") {
                    for prop in arr {
                        if let UsdProperty::Integer(v) = prop {
                            face_vertex_indices.push(*v as usize);
                        }
                    }
                }

                if !points.is_empty()
                    && !face_vertex_counts.is_empty()
                    && !face_vertex_indices.is_empty()
                {
                    let mut faces = Vec::new();
                    let mut idx = 0;
                    for count in face_vertex_counts {
                        if idx + count <= face_vertex_indices.len() {
                            let face: Vec<usize> = face_vertex_indices[idx..idx + count].to_vec();
                            if face.len() >= 3 {
                                faces.push(face);
                            }
                        }
                        idx += count;
                    }

                    if !faces.is_empty() {
                        let solid =
                            crate::topology::topods_solid::TopoDsSolid::from_mesh(points, faces);
                        compound.add_component(crate::foundation::handle::Handle::new(
                            std::sync::Arc::new(solid.shape().clone()),
                        ));
                    }
                }
            }
            "Sphere" | "GeomSphere" => {
                let radius = if let Some(UsdProperty::Double(r)) = prim.properties.get("radius") {
                    *r
                } else if let Some(UsdProperty::Float(r)) = prim.properties.get("radius") {
                    *r as f64
                } else {
                    1.0
                };

                let center = if let Some(UsdProperty::Vector3f(v)) = prim.properties.get("center") {
                    crate::geometry::Point::new(v[0] as f64, v[1] as f64, v[2] as f64)
                } else {
                    crate::geometry::Point::origin()
                };

                let solid = crate::modeling::primitives::make_sphere(radius, Some(center));
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
            "Cube" | "GeomCube" => {
                let size = if let Some(UsdProperty::Double(s)) = prim.properties.get("size") {
                    *s
                } else if let Some(UsdProperty::Float(s)) = prim.properties.get("size") {
                    *s as f64
                } else {
                    1.0
                };

                let solid = crate::modeling::primitives::make_box(size, size, size, None);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
            "Cylinder" | "GeomCylinder" => {
                let radius = if let Some(UsdProperty::Double(r)) = prim.properties.get("radius") {
                    *r
                } else if let Some(UsdProperty::Float(r)) = prim.properties.get("radius") {
                    *r as f64
                } else {
                    1.0
                };

                let height = if let Some(UsdProperty::Double(h)) = prim.properties.get("height") {
                    *h
                } else if let Some(UsdProperty::Float(h)) = prim.properties.get("height") {
                    *h as f64
                } else {
                    1.0
                };

                let solid = crate::modeling::primitives::make_cylinder(radius, height, None);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
            "Cone" | "GeomCone" => {
                let radius = if let Some(UsdProperty::Double(r)) = prim.properties.get("radius") {
                    *r
                } else if let Some(UsdProperty::Float(r)) = prim.properties.get("radius") {
                    *r as f64
                } else {
                    1.0
                };

                let height = if let Some(UsdProperty::Double(h)) = prim.properties.get("height") {
                    *h
                } else if let Some(UsdProperty::Float(h)) = prim.properties.get("height") {
                    *h as f64
                } else {
                    1.0
                };

                let solid = crate::modeling::primitives::make_cone(radius, height, None);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
            _ => {}
        }

        for child in &prim.children {
            self.process_usd_prim(child, compound)?;
        }

        Ok(())
    }
}

/// USD writer
pub struct UsdWriter {
    pub document: UsdDocument,
    pub options: UsdWriterOptions,
}

/// USD writer options
pub struct UsdWriterOptions {
    pub export_materials: bool,
    pub export_textures: bool,
    pub export_normals: bool,
    pub export_uvs: bool,
    pub use_usdc: bool,
}

impl Default for UsdWriterOptions {
    fn default() -> Self {
        Self {
            export_materials: true,
            export_textures: true,
            export_normals: true,
            export_uvs: true,
            use_usdc: false,
        }
    }
}

impl UsdWriter {
    /// Create a new USD writer
    pub fn new() -> Self {
        Self {
            document: UsdDocument {
                asset: UsdAsset {
                    name: "BrepRs Export".to_string(),
                    identifier: "BrepRs".to_string(),
                    version: "0.8.0".to_string(),
                    metadata: HashMap::new(),
                },
                root_prim: UsdPrim {
                    name: "Root".to_string(),
                    type_: "Xform".to_string(),
                    properties: HashMap::new(),
                    children: Vec::new(),
                },
            },
            options: UsdWriterOptions::default(),
        }
    }

    /// Create a new USD writer with options
    pub fn with_options(options: UsdWriterOptions) -> Self {
        Self {
            document: UsdDocument {
                asset: UsdAsset {
                    name: "BrepRs Export".to_string(),
                    identifier: "BrepRs".to_string(),
                    version: "0.8.0".to_string(),
                    metadata: HashMap::new(),
                },
                root_prim: UsdPrim {
                    name: "Root".to_string(),
                    type_: "Xform".to_string(),
                    properties: HashMap::new(),
                    children: Vec::new(),
                },
            },
            options,
        }
    }
}

impl ModernFormatWriter for UsdWriter {
    fn write_to_file(&mut self, shape: &TopoDsShape, path: &str) -> Result<(), String> {
        // Convert shape to USD document
        self.convert_from_shape(shape)?;

        // Check if we should write as USDC
        let use_usdc = self.options.use_usdc || path.ends_with(".usdc");

        if use_usdc {
            let buffer = self.write_to_usdc()?;
            std::fs::write(path, buffer).map_err(|e| e.to_string())
        } else {
            let content = self.write_to_usda()?;
            std::fs::write(path, content).map_err(|e| e.to_string())
        }
    }

    fn write_to_buffer(&mut self, shape: &TopoDsShape) -> Result<Vec<u8>, String> {
        // Convert shape to USD document
        self.convert_from_shape(shape)?;

        // Write as USDA by default
        let content = self.write_to_usda()?;
        Ok(content.into_bytes())
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::Usd
    }
}

impl UsdWriter {
    /// Convert TopoDsShape to USD document
    fn convert_from_shape(&mut self, shape: &TopoDsShape) -> Result<(), String> {
        let mut mesh_prims: Vec<UsdPrim> = Vec::new();

        let faces = shape.faces();
        if !faces.is_empty() {
            let mut points: Vec<[f32; 3]> = Vec::new();
            let mut face_vertex_counts: Vec<i32> = Vec::new();
            let mut face_vertex_indices: Vec<i32> = Vec::new();

            for face in &faces {
                let wires = face.wires();
                let mut face_indices: Vec<i32> = Vec::new();

                for wire in wires {
                    let edges = wire.edges();
                    for edge in edges {
                        let v1 = edge.start_vertex();
                        let v2 = edge.end_vertex();

                        if let Some(v1_ref) = v1.as_ref() {
                            let p = v1_ref.point();
                            let idx = self.find_or_add_point(&mut points, p.x, p.y, p.z);
                            if !face_indices.contains(&idx) {
                                face_indices.push(idx);
                            }
                        }

                        if let Some(v2_ref) = v2.as_ref() {
                            let p = v2_ref.point();
                            let idx = self.find_or_add_point(&mut points, p.x, p.y, p.z);
                            if !face_indices.contains(&idx) {
                                face_indices.push(idx);
                            }
                        }
                    }
                }

                if !face_indices.is_empty() {
                    face_vertex_counts.push(face_indices.len() as i32);
                    face_vertex_indices.extend(face_indices);
                }
            }

            if !points.is_empty() {
                let mut properties = HashMap::new();

                let point_props: Vec<UsdProperty> =
                    points.iter().map(|p| UsdProperty::Vector3f(*p)).collect();
                properties.insert("points".to_string(), UsdProperty::Array(point_props));

                let count_props: Vec<UsdProperty> = face_vertex_counts
                    .iter()
                    .map(|c| UsdProperty::Integer(*c))
                    .collect();
                properties.insert(
                    "faceVertexCounts".to_string(),
                    UsdProperty::Array(count_props),
                );

                let index_props: Vec<UsdProperty> = face_vertex_indices
                    .iter()
                    .map(|i| UsdProperty::Integer(*i))
                    .collect();
                properties.insert(
                    "faceVertexIndices".to_string(),
                    UsdProperty::Array(index_props),
                );

                mesh_prims.push(UsdPrim {
                    name: "Mesh".to_string(),
                    type_: "Mesh".to_string(),
                    properties,
                    children: Vec::new(),
                });
            }
        }

        self.document.root_prim.children = mesh_prims;

        Ok(())
    }

    fn find_or_add_point(&self, points: &mut Vec<[f32; 3]>, x: f64, y: f64, z: f64) -> i32 {
        let tolerance = 1e-6;
        for (i, p) in points.iter().enumerate() {
            if (p[0] as f64 - x).abs() < tolerance
                && (p[1] as f64 - y).abs() < tolerance
                && (p[2] as f64 - z).abs() < tolerance
            {
                return i as i32;
            }
        }
        let idx = points.len() as i32;
        points.push([x as f32, y as f32, z as f32]);
        idx
    }

    /// Write USD as USDA (ASCII format)
    fn write_to_usda(&self) -> Result<String, String> {
        let mut content = String::new();

        content.push_str("#usda 1.0\n");
        content.push_str("(\n");
        content.push_str(&format!("    doc = \"{}\"\n", self.document.asset.name));
        content.push_str(&format!(
            "    usdVersion = \"{}\"\n",
            self.document.asset.version
        ));
        content.push_str(")\n\n");

        content.push_str(&format!(
            "def {} \"{}\"\n",
            self.document.root_prim.type_, self.document.root_prim.name
        ));
        content.push_str("{\n");

        for child in &self.document.root_prim.children {
            self.write_prim_usda(&mut content, child, 1);
        }

        content.push_str("}\n");

        Ok(content)
    }

    fn write_prim_usda(&self, content: &mut String, prim: &UsdPrim, indent: usize) {
        let indent_str = "    ".repeat(indent);
        content.push_str(&format!(
            "{}def {} \"{}\"\n",
            indent_str, prim.type_, prim.name
        ));
        content.push_str(&format!("{}{{\n", indent_str));

        for (name, prop) in &prim.properties {
            self.write_property_usda(content, name, prop, indent + 1);
        }

        for child in &prim.children {
            self.write_prim_usda(content, child, indent + 1);
        }

        content.push_str(&format!("{}}}\n", indent_str));
    }

    fn write_property_usda(
        &self,
        content: &mut String,
        name: &str,
        prop: &UsdProperty,
        indent: usize,
    ) {
        let indent_str = "    ".repeat(indent);
        match prop {
            UsdProperty::Float(v) => {
                content.push_str(&format!("{}float {} = {}\n", indent_str, name, v));
            }
            UsdProperty::Double(v) => {
                content.push_str(&format!("{}double {} = {}\n", indent_str, name, v));
            }
            UsdProperty::Integer(v) => {
                content.push_str(&format!("{}int {} = {}\n", indent_str, name, v));
            }
            UsdProperty::Boolean(v) => {
                content.push_str(&format!("{}bool {} = {}\n", indent_str, name, v));
            }
            UsdProperty::String(v) => {
                content.push_str(&format!("{}string {} = \"{}\"\n", indent_str, name, v));
            }
            UsdProperty::Vector2f(v) => {
                content.push_str(&format!(
                    "{}float2 {} = ({}, {})\n",
                    indent_str, name, v[0], v[1]
                ));
            }
            UsdProperty::Vector3f(v) => {
                content.push_str(&format!(
                    "{}float3 {} = ({}, {}, {})\n",
                    indent_str, name, v[0], v[1], v[2]
                ));
            }
            UsdProperty::Vector4f(v) => {
                content.push_str(&format!(
                    "{}float4 {} = ({}, {}, {}, {})\n",
                    indent_str, name, v[0], v[1], v[2], v[3]
                ));
            }
            UsdProperty::Matrix4f(v) => {
                content.push_str(&format!(
                    "{}matrix4d {} = ({})\n",
                    indent_str,
                    name,
                    v.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            UsdProperty::Array(arr) => {
                content.push_str(&format!("{}{}[] = [\n", indent_str, name));
                for item in arr {
                    match item {
                        UsdProperty::Float(v) => {
                            content.push_str(&format!("{}    {},\n", indent_str, v))
                        }
                        UsdProperty::Double(v) => {
                            content.push_str(&format!("{}    {},\n", indent_str, v))
                        }
                        UsdProperty::Integer(v) => {
                            content.push_str(&format!("{}    {},\n", indent_str, v))
                        }
                        UsdProperty::Vector3f(v) => content.push_str(&format!(
                            "{}    ({}, {}, {}),\n",
                            indent_str, v[0], v[1], v[2]
                        )),
                        _ => {}
                    }
                }
                content.push_str(&format!("{}]\n", indent_str));
            }
        }
    }

    /// Write USD as USDC (binary format)
    fn write_to_usdc(&self) -> Result<Vec<u8>, String> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(b"usdc");
        buffer.extend_from_slice(&[0, 0, 0, 0]);

        self.write_prim_usdc(&mut buffer, &self.document.root_prim);

        Ok(buffer)
    }

    fn write_prim_usdc(&self, buffer: &mut Vec<u8>, prim: &UsdPrim) {
        let name_bytes = prim.name.as_bytes();
        buffer.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        buffer.extend_from_slice(name_bytes);

        let type_bytes = prim.type_.as_bytes();
        buffer.extend_from_slice(&(type_bytes.len() as u32).to_le_bytes());
        buffer.extend_from_slice(type_bytes);

        buffer.extend_from_slice(&(prim.properties.len() as u32).to_le_bytes());

        for (name, prop) in &prim.properties {
            let name_bytes = name.as_bytes();
            buffer.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
            buffer.extend_from_slice(name_bytes);

            self.write_property_usdc(buffer, prop);
        }

        buffer.extend_from_slice(&(prim.children.len() as u32).to_le_bytes());
        for child in &prim.children {
            self.write_prim_usdc(buffer, child);
        }
    }

    fn write_property_usdc(&self, buffer: &mut Vec<u8>, prop: &UsdProperty) {
        match prop {
            UsdProperty::Float(v) => {
                buffer.extend_from_slice(&0u32.to_le_bytes());
                buffer.extend_from_slice(&v.to_le_bytes());
            }
            UsdProperty::Double(v) => {
                buffer.extend_from_slice(&1u32.to_le_bytes());
                buffer.extend_from_slice(&v.to_le_bytes());
            }
            UsdProperty::Integer(v) => {
                buffer.extend_from_slice(&2u32.to_le_bytes());
                buffer.extend_from_slice(&v.to_le_bytes());
            }
            UsdProperty::Boolean(v) => {
                buffer.extend_from_slice(&3u32.to_le_bytes());
                buffer.push(if *v { 1 } else { 0 });
            }
            UsdProperty::String(v) => {
                buffer.extend_from_slice(&4u32.to_le_bytes());
                let bytes = v.as_bytes();
                buffer.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
                buffer.extend_from_slice(bytes);
            }
            UsdProperty::Vector3f(v) => {
                buffer.extend_from_slice(&5u32.to_le_bytes());
                for val in v {
                    buffer.extend_from_slice(&val.to_le_bytes());
                }
            }
            UsdProperty::Array(arr) => {
                buffer.extend_from_slice(&6u32.to_le_bytes());
                buffer.extend_from_slice(&(arr.len() as u32).to_le_bytes());
                for item in arr {
                    self.write_property_usdc(buffer, item);
                }
            }
            _ => {
                buffer.extend_from_slice(&0u32.to_le_bytes());
            }
        }
    }
}

/// 3MF reader
pub struct ThreeMFReader {
    pub model: Option<ThreeMFModel>,
    pub options: ThreeMFReaderOptions,
}

/// 3MF reader options
pub struct ThreeMFReaderOptions {
    pub load_materials: bool,
    pub triangulate: bool,
    pub compute_normals: bool,
}

impl Default for ThreeMFReaderOptions {
    fn default() -> Self {
        Self {
            load_materials: true,
            triangulate: true,
            compute_normals: true,
        }
    }
}

impl ThreeMFReader {
    /// Create a new 3MF reader
    pub fn new() -> Self {
        Self {
            model: None,
            options: ThreeMFReaderOptions::default(),
        }
    }

    /// Create a new 3MF reader with options
    pub fn with_options(options: ThreeMFReaderOptions) -> Self {
        Self {
            model: None,
            options,
        }
    }
}

impl ModernFormatReader for ThreeMFReader {
    fn read_from_file(&mut self, path: &str) -> Result<TopoDsShape, String> {
        // Read file into buffer
        let buffer = std::fs::read(path).map_err(|e| e.to_string())?;
        self.read_from_buffer(&buffer)
    }

    fn read_from_buffer(&mut self, buffer: &[u8]) -> Result<TopoDsShape, String> {
        // Parse 3MF ZIP archive
        let model = self.parse_3mf(buffer)?;
        self.model = Some(model);

        // Convert to TopoDsShape
        self.convert_to_shape()
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::ThreeMF
    }
}

impl ThreeMFReader {
    /// Parse 3MF ZIP archive
    fn parse_3mf(&self, buffer: &[u8]) -> Result<ThreeMFModel, String> {
        use std::io::Cursor;
        use zip::ZipArchive;

        let cursor = Cursor::new(buffer);
        let mut archive =
            ZipArchive::new(cursor).map_err(|e| format!("Failed to open 3MF archive: {}", e))?;

        let mut model = ThreeMFModel {
            name: "BrepRs Import".to_string(),
            resources: ThreeMFResources {
                objects: Vec::new(),
                materials: Vec::new(),
            },
            build: ThreeMFBuild { items: Vec::new() },
            metadata: HashMap::new(),
        };

        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            let name = file.name().to_string();

            if name.ends_with(".model") || name == "3D/3dmodel.model" {
                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| e.to_string())?;

                self.parse_3mf_model_xml(&content, &mut model)?;
            }
        }

        Ok(model)
    }

    fn parse_3mf_model_xml(&self, content: &str, model: &mut ThreeMFModel) -> Result<(), String> {
        let mut current_object: Option<ThreeMFObject> = None;
        let mut current_vertices: Vec<[f32; 3]> = Vec::new();
        let mut current_triangles: Vec<[u32; 3]> = Vec::new();
        let mut in_vertices = false;
        let mut in_triangles = false;
        let mut object_id = 1;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.contains("<object") {
                let id = self
                    .extract_attribute(trimmed, "id")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(object_id);
                object_id = id + 1;

                current_object = Some(ThreeMFObject {
                    id,
                    name: self.extract_attribute(trimmed, "name"),
                    type_: self
                        .extract_attribute(trimmed, "type")
                        .unwrap_or_else(|| "model".to_string()),
                    mesh: None,
                });
            }

            if trimmed.contains("</object>") {
                if let Some(mut obj) = current_object.take() {
                    if !current_vertices.is_empty() && !current_triangles.is_empty() {
                        obj.mesh = Some(ThreeMFMesh {
                            vertices: std::mem::take(&mut current_vertices),
                            triangles: std::mem::take(&mut current_triangles),
                        });
                    }
                    model.resources.objects.push(obj);
                }
            }

            if trimmed.contains("<mesh>") {}

            if trimmed.contains("<vertices>") {
                in_vertices = true;
            }

            if trimmed.contains("</vertices>") {
                in_vertices = false;
            }

            if trimmed.contains("<triangles>") {
                in_triangles = true;
            }

            if trimmed.contains("</triangles>") {
                in_triangles = false;
            }

            if in_vertices && trimmed.contains("<vertex") {
                let x = self
                    .extract_attribute(trimmed, "x")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let y = self
                    .extract_attribute(trimmed, "y")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let z = self
                    .extract_attribute(trimmed, "z")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                current_vertices.push([x, y, z]);
            }

            if in_triangles && trimmed.contains("<triangle") {
                let v1 = self
                    .extract_attribute(trimmed, "v1")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let v2 = self
                    .extract_attribute(trimmed, "v2")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                let v3 = self
                    .extract_attribute(trimmed, "v3")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0);
                current_triangles.push([v1, v2, v3]);
            }

            if trimmed.contains("<item") {
                let object_id = self
                    .extract_attribute(trimmed, "objectid")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                model.build.items.push(ThreeMFItem {
                    object_id,
                    transform: None,
                });
            }

            if trimmed.contains("<metadata") {
                if let Some(name) = self.extract_attribute(trimmed, "name") {
                    if let Some(value) = self.extract_content(trimmed) {
                        model.metadata.insert(name, value);
                    }
                }
            }
        }

        Ok(())
    }

    fn extract_attribute(&self, line: &str, attr: &str) -> Option<String> {
        let pattern = format!("{}=\"", attr);
        if let Some(start) = line.find(&pattern) {
            let start = start + pattern.len();
            if let Some(end) = line[start..].find('"') {
                return Some(line[start..start + end].to_string());
            }
        }
        None
    }

    fn extract_content(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find('>') {
            if let Some(end) = line[start + 1..].find('<') {
                return Some(line[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    /// Convert 3MF model to TopoDsShape
    fn convert_to_shape(&self) -> Result<TopoDsShape, String> {
        let mut compound = crate::topology::topods_compound::TopoDsCompound::new();

        if let Some(model) = &self.model {
            self.process_3mf_model(model, &mut compound)?;
        }

        Ok(compound.shape().clone())
    }

    /// Process 3MF model
    fn process_3mf_model(
        &self,
        model: &ThreeMFModel,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        for object in &model.resources.objects {
            self.process_3mf_object(object, compound)?;
        }

        Ok(())
    }

    /// Process 3MF object
    fn process_3mf_object(
        &self,
        object: &ThreeMFObject,
        compound: &mut crate::topology::topods_compound::TopoDsCompound,
    ) -> Result<(), String> {
        if let Some(mesh) = &object.mesh {
            if mesh.vertices.len() >= 3 && !mesh.triangles.is_empty() {
                let points: Vec<crate::geometry::Point> = mesh
                    .vertices
                    .iter()
                    .map(|v| crate::geometry::Point::new(v[0] as f64, v[1] as f64, v[2] as f64))
                    .collect();

                let faces: Vec<Vec<usize>> = mesh
                    .triangles
                    .iter()
                    .map(|t| vec![t[0] as usize, t[1] as usize, t[2] as usize])
                    .collect();

                let solid = crate::topology::topods_solid::TopoDsSolid::from_mesh(points, faces);
                compound.add_component(crate::foundation::handle::Handle::new(
                    std::sync::Arc::new(solid.shape().clone()),
                ));
            }
        }

        Ok(())
    }
}

/// 3MF writer
pub struct ThreeMFWriter {
    pub model: ThreeMFModel,
    pub options: ThreeMFWriterOptions,
}

/// 3MF writer options
pub struct ThreeMFWriterOptions {
    pub export_materials: bool,
    pub export_normals: bool,
    pub compress: bool,
}

impl Default for ThreeMFWriterOptions {
    fn default() -> Self {
        Self {
            export_materials: true,
            export_normals: true,
            compress: false,
        }
    }
}

impl ThreeMFWriter {
    /// Create a new 3MF writer
    pub fn new() -> Self {
        Self {
            model: ThreeMFModel {
                name: "BrepRs Export".to_string(),
                resources: ThreeMFResources {
                    objects: Vec::new(),
                    materials: Vec::new(),
                },
                build: ThreeMFBuild { items: Vec::new() },
                metadata: HashMap::new(),
            },
            options: ThreeMFWriterOptions::default(),
        }
    }

    /// Create a new 3MF writer with options
    pub fn with_options(options: ThreeMFWriterOptions) -> Self {
        Self {
            model: ThreeMFModel {
                name: "BrepRs Export".to_string(),
                resources: ThreeMFResources {
                    objects: Vec::new(),
                    materials: Vec::new(),
                },
                build: ThreeMFBuild { items: Vec::new() },
                metadata: HashMap::new(),
            },
            options,
        }
    }
}

impl ModernFormatWriter for ThreeMFWriter {
    fn write_to_file(&mut self, shape: &TopoDsShape, path: &str) -> Result<(), String> {
        // Convert shape to 3MF model
        self.convert_from_shape(shape)?;

        // Write 3MF ZIP archive
        let buffer = self.write_3mf()?;
        std::fs::write(path, buffer).map_err(|e| e.to_string())
    }

    fn write_to_buffer(&mut self, shape: &TopoDsShape) -> Result<Vec<u8>, String> {
        // Convert shape to 3MF model
        self.convert_from_shape(shape)?;

        // Write 3MF ZIP archive
        self.write_3mf()
    }

    fn format(&self) -> ModernFormat {
        ModernFormat::ThreeMF
    }
}

impl ThreeMFWriter {
    /// Convert TopoDsShape to 3MF model
    fn convert_from_shape(&mut self, shape: &TopoDsShape) -> Result<(), String> {
        let mut objects: Vec<ThreeMFObject> = Vec::new();

        let faces = shape.faces();
        if !faces.is_empty() {
            let mut vertices: Vec<[f32; 3]> = Vec::new();
            let mut triangles: Vec<[u32; 3]> = Vec::new();

            for face in &faces {
                let wires = face.wires();
                let mut face_vertices: Vec<[f32; 3]> = Vec::new();

                for wire in wires {
                    let edges = wire.edges();
                    for edge in edges {
                        let v1 = edge.start_vertex();
                        let v2 = edge.end_vertex();

                        if let Some(v1_ref) = v1.as_ref() {
                            let p = v1_ref.point();
                            let v = [p.x as f32, p.y as f32, p.z as f32];
                            if !face_vertices.contains(&v) {
                                face_vertices.push(v);
                            }
                        }

                        if let Some(v2_ref) = v2.as_ref() {
                            let p = v2_ref.point();
                            let v = [p.x as f32, p.y as f32, p.z as f32];
                            if !face_vertices.contains(&v) {
                                face_vertices.push(v);
                            }
                        }
                    }
                }

                if face_vertices.len() >= 3 {
                    let base_index = vertices.len() as u32;
                    for v in face_vertices {
                        vertices.push(v);
                    }

                    for i in 0..(vertices.len() as u32 - base_index - 2) {
                        triangles.push([base_index, base_index + i + 1, base_index + i + 2]);
                    }
                }
            }

            if !vertices.is_empty() {
                objects.push(ThreeMFObject {
                    id: 1,
                    name: Some("Exported Shape".to_string()),
                    type_: "model".to_string(),
                    mesh: Some(ThreeMFMesh {
                        vertices,
                        triangles,
                    }),
                });

                self.model.build.items.push(ThreeMFItem {
                    object_id: 1,
                    transform: None,
                });
            }
        }

        self.model.resources.objects = objects;

        Ok(())
    }

    /// Write 3MF ZIP archive
    fn write_3mf(&self) -> Result<Vec<u8>, String> {
        use std::io::{Cursor, Write};
        use zip::ZipWriter;

        let mut cursor = Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut cursor);

            zip.start_file("3D/3dmodel.model", zip::write::FileOptions::default())
                .map_err(|e| e.to_string())?;

            let mut xml_content = String::new();
            xml_content.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
            xml_content.push_str("<model unit=\"millimeter\" xml:lang=\"en-US\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n");

            xml_content.push_str("    <metadata name=\"Application\">BrepRs</metadata>\n");
            xml_content.push_str(&format!(
                "    <metadata name=\"Title\">{}</metadata>\n",
                self.model.name
            ));

            xml_content.push_str("    <resources>\n");

            for object in &self.model.resources.objects {
                xml_content.push_str(&format!(
                    "        <object id=\"{}\" type=\"{}\">\n",
                    object.id, object.type_
                ));

                if let Some(mesh) = &object.mesh {
                    xml_content.push_str("            <mesh>\n");

                    xml_content.push_str("                <vertices>\n");
                    for v in &mesh.vertices {
                        xml_content.push_str(&format!(
                            "                    <vertex x=\"{}\" y=\"{}\" z=\"{}\"/>\n",
                            v[0], v[1], v[2]
                        ));
                    }
                    xml_content.push_str("                </vertices>\n");

                    xml_content.push_str("                <triangles>\n");
                    for t in &mesh.triangles {
                        xml_content.push_str(&format!(
                            "                    <triangle v1=\"{}\" v2=\"{}\" v3=\"{}\"/>\n",
                            t[0], t[1], t[2]
                        ));
                    }
                    xml_content.push_str("                </triangles>\n");

                    xml_content.push_str("            </mesh>\n");
                }

                xml_content.push_str("        </object>\n");
            }

            xml_content.push_str("    </resources>\n");

            xml_content.push_str("    <build>\n");
            for item in &self.model.build.items {
                xml_content.push_str(&format!(
                    "        <item objectid=\"{}\"/>\n",
                    item.object_id
                ));
            }
            xml_content.push_str("    </build>\n");

            xml_content.push_str("</model>\n");

            zip.write_all(xml_content.as_bytes())
                .map_err(|e| e.to_string())?;
            zip.finish().map_err(|e| e.to_string())?;
        }

        Ok(cursor.into_inner())
    }
}

/// Modern format manager
pub struct ModernFormatManager {
    pub readers: HashMap<ModernFormat, Box<dyn ModernFormatReader>>,
    pub writers: HashMap<ModernFormat, Box<dyn ModernFormatWriter>>,
}

impl ModernFormatManager {
    /// Create a new modern format manager
    pub fn new() -> Self {
        let mut readers: HashMap<ModernFormat, Box<dyn ModernFormatReader>> = HashMap::new();
        let mut writers: HashMap<ModernFormat, Box<dyn ModernFormatWriter>> = HashMap::new();

        // Add glTF reader and writer
        readers.insert(ModernFormat::Gltf, Box::new(GltfReader::new()));
        writers.insert(ModernFormat::Gltf, Box::new(GltfWriter::new()));

        // Add USD reader and writer
        readers.insert(ModernFormat::Usd, Box::new(UsdReader::new()));
        writers.insert(ModernFormat::Usd, Box::new(UsdWriter::new()));

        // Add 3MF reader and writer
        readers.insert(ModernFormat::ThreeMF, Box::new(ThreeMFReader::new()));
        writers.insert(ModernFormat::ThreeMF, Box::new(ThreeMFWriter::new()));

        Self { readers, writers }
    }

    /// Read shape from file
    pub fn read_from_file(&mut self, path: &str) -> Result<TopoDsShape, String> {
        // Determine format from file extension
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let format = match extension.as_str() {
            "gltf" | "glb" => ModernFormat::Gltf,
            "usd" | "usda" | "usdc" => ModernFormat::Usd,
            "3mf" => ModernFormat::ThreeMF,
            _ => return Err(format!("Unsupported file format: {}", extension)),
        };

        // Get reader for format
        let reader = self
            .readers
            .get_mut(&format)
            .ok_or_else(|| format!("No reader available for format: {:?}", format))?;

        // Read from file
        reader.read_from_file(path)
    }

    /// Write shape to file
    pub fn write_to_file(&mut self, shape: &TopoDsShape, path: &str) -> Result<(), String> {
        // Determine format from file extension
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let format = match extension.as_str() {
            "gltf" | "glb" => ModernFormat::Gltf,
            "usd" | "usda" | "usdc" => ModernFormat::Usd,
            "3mf" => ModernFormat::ThreeMF,
            _ => return Err(format!("Unsupported file format: {}", extension)),
        };

        // Get writer for format
        let writer = self
            .writers
            .get_mut(&format)
            .ok_or_else(|| format!("No writer available for format: {:?}", format))?;

        // Write to file
        writer.write_to_file(shape, path)
    }

    /// Add custom reader
    pub fn add_reader(&mut self, format: ModernFormat, reader: Box<dyn ModernFormatReader>) {
        self.readers.insert(format, reader);
    }

    /// Add custom writer
    pub fn add_writer(&mut self, format: ModernFormat, writer: Box<dyn ModernFormatWriter>) {
        self.writers.insert(format, writer);
    }
}
