//! glTF 2.0 (GL Transmission Format) file format support
//!
//! This module provides functionality for reading and writing glTF files,
//! both in JSON (.gltf) and binary (.glb) formats.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

// ...existing code...
use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// glTF file format error types
#[derive(Debug)]
pub enum GltfError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid glTF file format
    InvalidFormat,
    /// Invalid JSON parsing
    JsonError(serde_json::Error),
    /// Invalid binary GLB format
    InvalidGlb,
    /// Unsupported glTF version
    UnsupportedVersion(u32),
    /// Missing required field
    MissingField(String),
    /// Invalid buffer data
    InvalidBuffer,
    /// Invalid accessor
    InvalidAccessor,
    /// Unsupported primitive mode
    UnsupportedPrimitiveMode(u32),
    /// Texture loading error
    TextureError(String),
    /// Animation error
    AnimationError(String),
    /// Skin/skeleton error
    SkinError(String),
}

impl From<std::io::Error> for GltfError {
    fn from(err: std::io::Error) -> Self {
        GltfError::IoError(err)
    }
}

impl From<serde_json::Error> for GltfError {
    fn from(err: serde_json::Error) -> Self {
        GltfError::JsonError(err)
    }
}

/// glTF primitive mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GltfPrimitiveMode {
    /// Points
    Points = 0,
    /// Lines
    Lines = 1,
    /// Line loop
    LineLoop = 2,
    /// Line strip
    LineStrip = 3,
    /// Triangles
    #[default]
    Triangles = 4,
    /// Triangle strip
    TriangleStrip = 5,
    /// Triangle fan
    TriangleFan = 6,
}

impl GltfPrimitiveMode {
    /// Create from glTF mode value
    pub fn from_value(value: u32) -> Self {
        match value {
            0 => GltfPrimitiveMode::Points,
            1 => GltfPrimitiveMode::Lines,
            2 => GltfPrimitiveMode::LineLoop,
            3 => GltfPrimitiveMode::LineStrip,
            4 => GltfPrimitiveMode::Triangles,
            5 => GltfPrimitiveMode::TriangleStrip,
            6 => GltfPrimitiveMode::TriangleFan,
            _ => GltfPrimitiveMode::Triangles,
        }
    }
}

/// glTF component type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GltfComponentType {
    /// Byte (signed 8-bit integer)
    Byte = 5120,
    /// Unsigned byte
    UnsignedByte = 5121,
    /// Short (signed 16-bit integer)
    Short = 5122,
    /// Unsigned short
    UnsignedShort = 5123,
    /// Unsigned int (32-bit)
    UnsignedInt = 5125,
    /// Float (32-bit)
    Float = 5126,
}

impl GltfComponentType {
    /// Get the size in bytes
    pub fn size(&self) -> usize {
        match self {
            GltfComponentType::Byte => 1,
            GltfComponentType::UnsignedByte => 1,
            GltfComponentType::Short => 2,
            GltfComponentType::UnsignedShort => 2,
            GltfComponentType::UnsignedInt => 4,
            GltfComponentType::Float => 4,
        }
    }

    /// Create from glTF component type value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            5120 => Some(GltfComponentType::Byte),
            5121 => Some(GltfComponentType::UnsignedByte),
            5122 => Some(GltfComponentType::Short),
            5123 => Some(GltfComponentType::UnsignedShort),
            5125 => Some(GltfComponentType::UnsignedInt),
            5126 => Some(GltfComponentType::Float),
            _ => None,
        }
    }
}

/// glTF accessor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GltfAccessorType {
    /// Scalar
    Scalar,
    /// Vec2
    Vec2,
    /// Vec3
    Vec3,
    /// Vec4
    Vec4,
    /// Mat2
    Mat2,
    /// Mat3
    Mat3,
    /// Mat4
    Mat4,
}

impl GltfAccessorType {
    /// Get the number of components
    pub fn component_count(&self) -> usize {
        match self {
            GltfAccessorType::Scalar => 1,
            GltfAccessorType::Vec2 => 2,
            GltfAccessorType::Vec3 => 3,
            GltfAccessorType::Vec4 => 4,
            GltfAccessorType::Mat2 => 4,
            GltfAccessorType::Mat3 => 9,
            GltfAccessorType::Mat4 => 16,
        }
    }

    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "SCALAR" => Some(GltfAccessorType::Scalar),
            "VEC2" => Some(GltfAccessorType::Vec2),
            "VEC3" => Some(GltfAccessorType::Vec3),
            "VEC4" => Some(GltfAccessorType::Vec4),
            "MAT2" => Some(GltfAccessorType::Mat2),
            "MAT3" => Some(GltfAccessorType::Mat3),
            "MAT4" => Some(GltfAccessorType::Mat4),
            _ => None,
        }
    }
}

/// glTF buffer view target
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GltfBufferViewTarget {
    /// Array buffer
    ArrayBuffer = 34962,
    /// Element array buffer
    ElementArrayBuffer = 34963,
}

/// glTF texture filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GltfTextureFilter {
    /// Nearest
    Nearest = 9728,
    /// Linear
    Linear = 9729,
    /// Nearest mipmap nearest
    NearestMipmapNearest = 9984,
    /// Linear mipmap nearest
    LinearMipmapNearest = 9985,
    /// Nearest mipmap linear
    NearestMipmapLinear = 9986,
    /// Linear mipmap linear
    LinearMipmapLinear = 9987,
}

/// glTF texture wrap mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GltfWrapMode {
    /// Clamp to edge
    ClampToEdge = 33071,
    /// Mirrored repeat
    MirroredRepeat = 33648,
    /// Repeat
    #[default]
    Repeat = 10497,
}

/// glTF alpha mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GltfAlphaMode {
    /// Opaque
    #[default]
    Opaque,
    /// Mask
    Mask,
    /// Blend
    Blend,
}

impl GltfAlphaMode {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "MASK" => GltfAlphaMode::Mask,
            "BLEND" => GltfAlphaMode::Blend,
            _ => GltfAlphaMode::Opaque,
        }
    }
}

/// glTF PBR metallic-roughness material
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfPbrMetallicRoughness {
    /// Base color factor (RGBA)
    pub base_color_factor: [f64; 4],
    /// Base color texture index
    pub base_color_texture: Option<u32>,
    /// Metallic factor
    pub metallic_factor: f64,
    /// Roughness factor
    pub roughness_factor: f64,
    /// Metallic-roughness texture index
    pub metallic_roughness_texture: Option<u32>,
}

/// glTF material
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfMaterial {
    /// Material name
    pub name: Option<String>,
    /// PBR metallic-roughness properties
    pub pbr_metallic_roughness: GltfPbrMetallicRoughness,
    /// Normal texture index
    pub normal_texture: Option<u32>,
    /// Occlusion texture index
    pub occlusion_texture: Option<u32>,
    /// Emissive texture index
    pub emissive_texture: Option<u32>,
    /// Emissive factor (RGB)
    pub emissive_factor: [f64; 3],
    /// Alpha mode
    pub alpha_mode: GltfAlphaMode,
    /// Alpha cutoff
    pub alpha_cutoff: f64,
    /// Double sided
    pub double_sided: bool,
}

/// glTF primitive attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GltfAttribute {
    /// Position
    Position,
    /// Normal
    Normal,
    /// Tangent
    Tangent,
    /// Texture coordinate 0
    TexCoord0,
    /// Texture coordinate 1
    TexCoord1,
    /// Color 0
    Color0,
    /// Joint indices 0
    Joints0,
    /// Joint weights 0
    Weights0,
    /// Custom attribute
    Custom(String),
}

impl GltfAttribute {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "POSITION" => GltfAttribute::Position,
            "NORMAL" => GltfAttribute::Normal,
            "TANGENT" => GltfAttribute::Tangent,
            "TEXCOORD_0" => GltfAttribute::TexCoord0,
            "TEXCOORD_1" => GltfAttribute::TexCoord1,
            "COLOR_0" => GltfAttribute::Color0,
            "JOINTS_0" => GltfAttribute::Joints0,
            "WEIGHTS_0" => GltfAttribute::Weights0,
            _ => GltfAttribute::Custom(s.to_string()),
        }
    }
}

/// glTF primitive
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfPrimitive {
    /// Attributes (attribute -> accessor index)
    pub attributes: HashMap<GltfAttribute, u32>,
    /// Indices accessor index
    pub indices: Option<u32>,
    /// Material index
    pub material: Option<u32>,
    /// Primitive mode
    pub mode: GltfPrimitiveMode,
    /// Morph targets
    pub targets: Vec<HashMap<GltfAttribute, u32>>,
}

/// glTF mesh
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfMesh {
    /// Mesh name
    pub name: Option<String>,
    /// Primitives
    pub primitives: Vec<GltfPrimitive>,
    /// Morph target weights
    pub weights: Vec<f64>,
}

/// glTF camera type
#[derive(Debug, Clone)]
pub enum GltfCameraType {
    /// Perspective camera
    Perspective {
        /// Aspect ratio
        aspect_ratio: Option<f64>,
        /// Vertical field of view in radians
        yfov: f64,
        /// Near plane
        zfar: Option<f64>,
        /// Far plane
        znear: f64,
    },
    /// Orthographic camera
    Orthographic {
        /// Horizontal magnification
        xmag: f64,
        /// Vertical magnification
        ymag: f64,
        /// Far plane
        zfar: f64,
        /// Near plane
        znear: f64,
    },
}

/// glTF camera
#[derive(Debug, Clone)]
pub struct GltfCamera {
    /// Camera name
    pub name: Option<String>,
    /// Camera type
    pub camera_type: GltfCameraType,
}

/// glTF light type
#[derive(Debug, Clone)]
pub enum GltfLightType {
    /// Directional light
    Directional,
    /// Point light
    Point,
    /// Spot light
    Spot {
        /// Inner cone angle in radians
        inner_cone_angle: f64,
        /// Outer cone angle in radians
        outer_cone_angle: f64,
    },
}

/// glTF light (KHR_lights_punctual extension)
#[derive(Debug, Clone)]
pub struct GltfLight {
    /// Light name
    pub name: Option<String>,
    /// Light type
    pub light_type: GltfLightType,
    /// Light color (RGB)
    pub color: [f64; 3],
    /// Intensity in candela (point/spot) or lux (directional)
    pub intensity: f64,
    /// Range (for point/spot lights)
    pub range: Option<f64>,
}

/// glTF transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfTransform {
    /// Translation
    #[serde(default = "default_translation")]
    pub translation: [f64; 3],
    /// Rotation (quaternion: x, y, z, w)
    #[serde(default = "default_rotation")]
    pub rotation: [f64; 4],
    /// Scale
    #[serde(default = "default_scale")]
    pub scale: [f64; 3],
}

fn default_translation() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}
fn default_rotation() -> [f64; 4] {
    [0.0, 0.0, 0.0, 1.0]
}
fn default_scale() -> [f64; 3] {
    [1.0, 1.0, 1.0]
}

impl Default for GltfTransform {
    fn default() -> Self {
        Self {
            translation: default_translation(),
            rotation: default_rotation(),
            scale: default_scale(),
        }
    }
}

/// glTF node
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfNode {
    /// Node name
    pub name: Option<String>,
    /// Child node indices
    pub children: Vec<u32>,
    /// Transformation
    pub transform: GltfTransform,
    /// Matrix (if provided instead of TRS)
    pub matrix: Option<[f64; 16]>,
    /// Mesh index
    pub mesh: Option<u32>,
    /// Camera index
    pub camera: Option<u32>,
    /// Light index (KHR_lights_punctual)
    pub light: Option<u32>,
    /// Skin index
    pub skin: Option<u32>,
}

/// glTF scene
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GltfScene {
    /// Scene name
    pub name: Option<String>,
    /// Root node indices
    pub nodes: Vec<u32>,
}

/// glTF animation sampler interpolation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GltfInterpolation {
    /// Linear
    Linear,
    /// Step
    Step,
    /// Cubic spline
    CubicSpline,
}

impl GltfInterpolation {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "STEP" => GltfInterpolation::Step,
            "CUBICSPLINE" => GltfInterpolation::CubicSpline,
            _ => GltfInterpolation::Linear,
        }
    }
}

/// glTF animation channel target path
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GltfAnimationPath {
    /// Translation
    Translation,
    /// Rotation
    Rotation,
    /// Scale
    Scale,
    /// Weights (morph targets)
    Weights,
}

impl GltfAnimationPath {
    /// Parse from string
    pub fn from_str(s: &str) -> Self {
        match s {
            "translation" => GltfAnimationPath::Translation,
            "rotation" => GltfAnimationPath::Rotation,
            "scale" => GltfAnimationPath::Scale,
            "weights" => GltfAnimationPath::Weights,
            _ => GltfAnimationPath::Translation,
        }
    }
}

/// glTF animation channel
#[derive(Debug, Clone)]
pub struct GltfAnimationChannel {
    /// Sampler index
    pub sampler: u32,
    /// Target node index
    pub target_node: u32,
    /// Target path
    pub target_path: GltfAnimationPath,
}

/// glTF animation sampler
#[derive(Debug, Clone)]
pub struct GltfAnimationSampler {
    /// Input accessor (time)
    pub input: u32,
    /// Output accessor
    pub output: u32,
    /// Interpolation method
    pub interpolation: GltfInterpolation,
}

/// glTF animation
#[derive(Debug, Clone)]
pub struct GltfAnimation {
    /// Animation name
    pub name: Option<String>,
    /// Channels
    pub channels: Vec<GltfAnimationChannel>,
    /// Samplers
    pub samplers: Vec<GltfAnimationSampler>,
}

/// glTF skin
#[derive(Debug, Clone)]
pub struct GltfSkin {
    /// Skin name
    pub name: Option<String>,
    /// Inverse bind matrices accessor
    pub inverse_bind_matrices: Option<u32>,
    /// Skeleton root node
    pub skeleton: Option<u32>,
    /// Joint node indices
    pub joints: Vec<u32>,
}

/// glTF buffer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfBuffer {
    /// Buffer name
    pub name: Option<String>,
    /// Byte length
    pub byte_length: usize,
    /// URI (for .gltf format)
    pub uri: Option<String>,
    /// Data (loaded from file)
    pub data: Option<Vec<u8>>,
}

/// glTF buffer view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfBufferView {
    /// Buffer view name
    pub name: Option<String>,
    /// Buffer index
    pub buffer: u32,
    /// Byte offset
    pub byte_offset: usize,
    /// Byte length
    pub byte_length: usize,
    /// Byte stride
    pub byte_stride: Option<usize>,
    /// Target
    pub target: Option<GltfBufferViewTarget>,
}

/// glTF accessor sparse indices
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfSparseIndices {
    /// Buffer view
    pub buffer_view: u32,
    /// Byte offset
    pub byte_offset: usize,
    /// Component type
    pub component_type: GltfComponentType,
}

/// glTF accessor sparse values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfSparseValues {
    /// Buffer view
    pub buffer_view: u32,
    /// Byte offset
    pub byte_offset: usize,
}

/// glTF accessor sparse data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfSparse {
    /// Count
    pub count: usize,
    /// Indices
    pub indices: GltfSparseIndices,
    /// Values
    pub values: GltfSparseValues,
}

/// glTF accessor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GltfAccessor {
    /// Accessor name
    pub name: Option<String>,
    /// Buffer view
    pub buffer_view: Option<u32>,
    /// Byte offset
    pub byte_offset: usize,
    /// Component type
    pub component_type: GltfComponentType,
    /// Count
    pub count: usize,
    /// Type
    pub accessor_type: GltfAccessorType,
    /// Min values
    pub min: Option<Vec<f64>>,
    /// Max values
    pub max: Option<Vec<f64>>,
    /// Sparse data
    pub sparse: Option<GltfSparse>,
}

/// glTF image
#[derive(Debug, Clone)]
pub struct GltfImage {
    /// Image name
    pub name: Option<String>,
    /// URI
    pub uri: Option<String>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Buffer view (for embedded images)
    pub buffer_view: Option<u32>,
}

/// glTF texture sampler
#[derive(Debug, Clone, Default)]
pub struct GltfSampler {
    /// Sampler name
    pub name: Option<String>,
    /// Mag filter
    pub mag_filter: Option<GltfTextureFilter>,
    /// Min filter
    pub min_filter: Option<GltfTextureFilter>,
    /// Wrap S
    pub wrap_s: GltfWrapMode,
    /// Wrap T
    pub wrap_t: GltfWrapMode,
}

/// glTF texture
#[derive(Debug, Clone)]
pub struct GltfTexture {
    /// Texture name
    pub name: Option<String>,
    /// Sampler index
    pub sampler: Option<u32>,
    /// Source image index
    pub source: Option<u32>,
}

/// glTF asset metadata
#[derive(Debug, Clone, Default)]
pub struct GltfAsset {
    /// Copyright
    pub copyright: Option<String>,
    /// Generator
    pub generator: Option<String>,
    /// glTF version
    pub version: String,
    /// Min version
    pub min_version: Option<String>,
    /// Extensions used
    pub extensions_used: Vec<String>,
    /// Extensions required
    pub extensions_required: Vec<String>,
}

/// glTF document
#[derive(Debug, Clone, Default)]
pub struct GltfDocument {
    /// Asset metadata
    pub asset: GltfAsset,
    /// Default scene index
    pub scene: Option<u32>,
    /// Scenes
    pub scenes: Vec<GltfScene>,
    /// Nodes
    pub nodes: Vec<GltfNode>,
    /// Meshes
    pub meshes: Vec<GltfMesh>,
    /// Materials
    pub materials: Vec<GltfMaterial>,
    /// Textures
    pub textures: Vec<GltfTexture>,
    /// Images
    pub images: Vec<GltfImage>,
    /// Samplers
    pub samplers: Vec<GltfSampler>,
    /// Accessors
    pub accessors: Vec<GltfAccessor>,
    /// Buffer views
    pub buffer_views: Vec<GltfBufferView>,
    /// Buffers
    pub buffers: Vec<GltfBuffer>,
    /// Animations
    pub animations: Vec<GltfAnimation>,
    /// Skins
    pub skins: Vec<GltfSkin>,
    /// Cameras
    pub cameras: Vec<GltfCamera>,
    /// Lights (KHR_lights_punctual)
    pub lights: Vec<GltfLight>,
}

/// glTF reader for reading glTF/GLB files
pub struct GltfReader {
    filename: String,
    document: GltfDocument,
    binary_buffer: Option<Vec<u8>>,
}

impl GltfReader {
    /// Create a new glTF reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: GltfDocument::default(),
            binary_buffer: None,
        }
    }

    /// Read a glTF/GLB file and return the document
    pub fn read(&mut self) -> Result<&GltfDocument, GltfError> {
        let path = Path::new(&self.filename);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        if extension == "glb" {
            self.read_glb()
        } else {
            self.read_gltf()
        }
    }

    /// Read a GLB binary file
    fn read_glb(&mut self) -> Result<&GltfDocument, GltfError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        // Read GLB header (12 bytes)
        let mut header = [0u8; 12];
        reader.read_exact(&mut header)?;

        // Check magic number (glTF)
        let magic = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        if magic != 0x46546C67 {
            return Err(GltfError::InvalidGlb);
        }

        // Check version (must be 2)
        let version = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);
        if version != 2 {
            return Err(GltfError::UnsupportedVersion(version));
        }

        // Total file length
        let _total_length = u32::from_le_bytes([header[8], header[9], header[10], header[11]]);

        // Read JSON chunk
        let mut chunk_header = [0u8; 8];
        reader.read_exact(&mut chunk_header)?;
        let chunk_length = u32::from_le_bytes([
            chunk_header[0],
            chunk_header[1],
            chunk_header[2],
            chunk_header[3],
        ]) as usize;
        let chunk_type = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]);

        if chunk_type != 0x4E4F534A {
            return Err(GltfError::InvalidGlb);
        }

        let mut json_data = vec![0u8; chunk_length];
        reader.read_exact(&mut json_data)?;

        // Parse JSON
        let json_str = String::from_utf8_lossy(&json_data);
        self.parse_gltf_json(&json_str)?;

        // Read binary chunk (if present)
        let mut chunk_header = [0u8; 8];
        if reader.read_exact(&mut chunk_header).is_ok() {
            let chunk_length = u32::from_le_bytes([
                chunk_header[0],
                chunk_header[1],
                chunk_header[2],
                chunk_header[3],
            ]) as usize;
            let chunk_type = u32::from_le_bytes([
                chunk_header[4],
                chunk_header[5],
                chunk_header[6],
                chunk_header[7],
            ]);

            if chunk_type == 0x004E4942 {
                let mut binary_data = vec![0u8; chunk_length];
                reader.read_exact(&mut binary_data)?;
                self.binary_buffer = Some(binary_data);
            }
        }

        Ok(&self.document)
    }

    /// Read a glTF JSON file
    fn read_gltf(&mut self) -> Result<&GltfDocument, GltfError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut json_str = String::new();
        reader.read_to_string(&mut json_str)?;

        self.parse_gltf_json(&json_str)?;

        // Load external buffers
        self.load_external_buffers()?;

        Ok(&self.document)
    }

    /// Parse glTF JSON content
    fn parse_gltf_json(&mut self, json_str: &str) -> Result<(), GltfError> {
        let json: serde_json::Value = serde_json::from_str(json_str)?;

        // Parse asset
        if let Some(asset) = json.get("asset") {
            self.document.asset = self.parse_asset(asset);
        } else {
            return Err(GltfError::MissingField("asset".to_string()));
        }

        // Parse scenes
        if let Some(scenes) = json.get("scenes").and_then(|s| s.as_array()) {
            for scene in scenes {
                self.document.scenes.push(self.parse_scene(scene));
            }
        }

        // Parse nodes
        if let Some(nodes) = json.get("nodes").and_then(|n| n.as_array()) {
            for node in nodes {
                self.document.nodes.push(self.parse_node(node));
            }
        }

        // Parse meshes
        if let Some(meshes) = json.get("meshes").and_then(|m| m.as_array()) {
            for mesh in meshes {
                self.document.meshes.push(self.parse_mesh(mesh));
            }
        }

        // Parse materials
        if let Some(materials) = json.get("materials").and_then(|m| m.as_array()) {
            for material in materials {
                self.document.materials.push(self.parse_material(material));
            }
        }

        // Parse textures
        if let Some(textures) = json.get("textures").and_then(|t| t.as_array()) {
            for texture in textures {
                self.document.textures.push(self.parse_texture(texture));
            }
        }

        // Parse images
        if let Some(images) = json.get("images").and_then(|i| i.as_array()) {
            for image in images {
                self.document.images.push(self.parse_image(image));
            }
        }

        // Parse samplers
        if let Some(samplers) = json.get("samplers").and_then(|s| s.as_array()) {
            for sampler in samplers {
                self.document.samplers.push(self.parse_sampler(sampler));
            }
        }

        // Parse accessors
        if let Some(accessors) = json.get("accessors").and_then(|a| a.as_array()) {
            for accessor in accessors {
                self.document.accessors.push(self.parse_accessor(accessor));
            }
        }

        // Parse buffer views
        if let Some(buffer_views) = json.get("bufferViews").and_then(|b| b.as_array()) {
            for buffer_view in buffer_views {
                self.document
                    .buffer_views
                    .push(self.parse_buffer_view(buffer_view));
            }
        }

        // Parse buffers
        if let Some(buffers) = json.get("buffers").and_then(|b| b.as_array()) {
            for buffer in buffers {
                self.document.buffers.push(self.parse_buffer(buffer));
            }
        }

        // Parse animations
        if let Some(animations) = json.get("animations").and_then(|a| a.as_array()) {
            for animation in animations {
                self.document
                    .animations
                    .push(self.parse_animation(animation));
            }
        }

        // Parse skins
        if let Some(skins) = json.get("skins").and_then(|s| s.as_array()) {
            for skin in skins {
                self.document.skins.push(self.parse_skin(skin));
            }
        }

        // Parse cameras
        if let Some(cameras) = json.get("cameras").and_then(|c| c.as_array()) {
            for camera in cameras {
                self.document.cameras.push(self.parse_camera(camera));
            }
        }

        // Parse default scene
        self.document.scene = json.get("scene").and_then(|s| s.as_u64()).map(|s| s as u32);

        Ok(())
    }

    /// Parse asset metadata
    fn parse_asset(&self, json: &serde_json::Value) -> GltfAsset {
        GltfAsset {
            copyright: json
                .get("copyright")
                .and_then(|c| c.as_str())
                .map(String::from),
            generator: json
                .get("generator")
                .and_then(|g| g.as_str())
                .map(String::from),
            version: json
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("2.0")
                .to_string(),
            min_version: json
                .get("minVersion")
                .and_then(|m| m.as_str())
                .map(String::from),
            extensions_used: json
                .get("extensionsUsed")
                .and_then(|e| e.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            extensions_required: json
                .get("extensionsRequired")
                .and_then(|e| e.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    /// Parse scene
    fn parse_scene(&self, json: &serde_json::Value) -> GltfScene {
        GltfScene {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            nodes: json
                .get("nodes")
                .and_then(|n| n.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_u64().map(|i| i as u32))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    /// Parse node
    fn parse_node(&self, json: &serde_json::Value) -> GltfNode {
        let mut node = GltfNode::default();

        node.name = json.get("name").and_then(|n| n.as_str()).map(String::from);
        node.children = json
            .get("children")
            .and_then(|c| c.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_u64().map(|i| i as u32))
                    .collect()
            })
            .unwrap_or_default();
        node.mesh = json.get("mesh").and_then(|m| m.as_u64()).map(|m| m as u32);
        node.camera = json
            .get("camera")
            .and_then(|c| c.as_u64())
            .map(|c| c as u32);
        node.skin = json.get("skin").and_then(|s| s.as_u64()).map(|s| s as u32);

        // Parse transformation
        if let Some(matrix) = json.get("matrix").and_then(|m| m.as_array()) {
            let m: Vec<f64> = matrix.iter().filter_map(|v| v.as_f64()).collect();
            if m.len() == 16 {
                let mut arr = [0.0; 16];
                arr.copy_from_slice(&m);
                node.matrix = Some(arr);
            }
        } else {
            if let Some(translation) = json.get("translation").and_then(|t| t.as_array()) {
                let t: Vec<f64> = translation.iter().filter_map(|v| v.as_f64()).collect();
                if t.len() == 3 {
                    node.transform.translation = [t[0], t[1], t[2]];
                }
            }
            if let Some(rotation) = json.get("rotation").and_then(|r| r.as_array()) {
                let r: Vec<f64> = rotation.iter().filter_map(|v| v.as_f64()).collect();
                if r.len() == 4 {
                    node.transform.rotation = [r[0], r[1], r[2], r[3]];
                }
            }
            if let Some(scale) = json.get("scale").and_then(|s| s.as_array()) {
                let s: Vec<f64> = scale.iter().filter_map(|v| v.as_f64()).collect();
                if s.len() == 3 {
                    node.transform.scale = [s[0], s[1], s[2]];
                }
            }
        }

        node
    }

    /// Parse mesh
    fn parse_mesh(&self, json: &serde_json::Value) -> GltfMesh {
        let mut mesh = GltfMesh::default();

        mesh.name = json.get("name").and_then(|n| n.as_str()).map(String::from);

        if let Some(primitives) = json.get("primitives").and_then(|p| p.as_array()) {
            for primitive in primitives {
                mesh.primitives.push(self.parse_primitive(primitive));
            }
        }

        if let Some(weights) = json.get("weights").and_then(|w| w.as_array()) {
            mesh.weights = weights.iter().filter_map(|v| v.as_f64()).collect();
        }

        mesh
    }

    /// Parse primitive
    fn parse_primitive(&self, json: &serde_json::Value) -> GltfPrimitive {
        let mut primitive = GltfPrimitive::default();

        if let Some(attributes) = json.get("attributes").and_then(|a| a.as_object()) {
            for (key, value) in attributes {
                if let Some(accessor_index) = value.as_u64() {
                    let attr = GltfAttribute::from_str(key);
                    primitive.attributes.insert(attr, accessor_index as u32);
                }
            }
        }

        primitive.indices = json
            .get("indices")
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);
        primitive.material = json
            .get("material")
            .and_then(|m| m.as_u64())
            .map(|m| m as u32);
        primitive.mode = json
            .get("mode")
            .and_then(|m| m.as_u64())
            .map(|m| GltfPrimitiveMode::from_value(m as u32))
            .unwrap_or_default();

        primitive
    }

    /// Parse material
    fn parse_material(&self, json: &serde_json::Value) -> GltfMaterial {
        let mut material = GltfMaterial::default();

        material.name = json.get("name").and_then(|n| n.as_str()).map(String::from);

        if let Some(pbr) = json.get("pbrMetallicRoughness") {
            material.pbr_metallic_roughness = self.parse_pbr_metallic_roughness(pbr);
        }

        material.normal_texture = json
            .get("normalTexture")
            .and_then(|t| t.get("index"))
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);

        material.occlusion_texture = json
            .get("occlusionTexture")
            .and_then(|t| t.get("index"))
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);

        material.emissive_texture = json
            .get("emissiveTexture")
            .and_then(|t| t.get("index"))
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);

        if let Some(emissive) = json.get("emissiveFactor").and_then(|e| e.as_array()) {
            let e: Vec<f64> = emissive.iter().filter_map(|v| v.as_f64()).collect();
            if e.len() == 3 {
                material.emissive_factor = [e[0], e[1], e[2]];
            }
        }

        material.alpha_mode = json
            .get("alphaMode")
            .and_then(|a| a.as_str())
            .map(GltfAlphaMode::from_str)
            .unwrap_or_default();

        material.alpha_cutoff = json
            .get("alphaCutoff")
            .and_then(|a| a.as_f64())
            .unwrap_or(0.5);

        material.double_sided = json
            .get("doubleSided")
            .and_then(|d| d.as_bool())
            .unwrap_or(false);

        material
    }

    /// Parse PBR metallic-roughness
    fn parse_pbr_metallic_roughness(&self, json: &serde_json::Value) -> GltfPbrMetallicRoughness {
        let mut pbr = GltfPbrMetallicRoughness::default();

        if let Some(base_color) = json.get("baseColorFactor").and_then(|b| b.as_array()) {
            let bc: Vec<f64> = base_color.iter().filter_map(|v| v.as_f64()).collect();
            if bc.len() == 4 {
                pbr.base_color_factor = [bc[0], bc[1], bc[2], bc[3]];
            }
        }

        pbr.base_color_texture = json
            .get("baseColorTexture")
            .and_then(|t| t.get("index"))
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);

        pbr.metallic_factor = json
            .get("metallicFactor")
            .and_then(|m| m.as_f64())
            .unwrap_or(1.0);

        pbr.roughness_factor = json
            .get("roughnessFactor")
            .and_then(|r| r.as_f64())
            .unwrap_or(1.0);

        pbr.metallic_roughness_texture = json
            .get("metallicRoughnessTexture")
            .and_then(|t| t.get("index"))
            .and_then(|i| i.as_u64())
            .map(|i| i as u32);

        pbr
    }

    /// Parse texture
    fn parse_texture(&self, json: &serde_json::Value) -> GltfTexture {
        GltfTexture {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            sampler: json
                .get("sampler")
                .and_then(|s| s.as_u64())
                .map(|s| s as u32),
            source: json
                .get("source")
                .and_then(|s| s.as_u64())
                .map(|s| s as u32),
        }
    }

    /// Parse image
    fn parse_image(&self, json: &serde_json::Value) -> GltfImage {
        GltfImage {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            uri: json.get("uri").and_then(|u| u.as_str()).map(String::from),
            mime_type: json
                .get("mimeType")
                .and_then(|m| m.as_str())
                .map(String::from),
            buffer_view: json
                .get("bufferView")
                .and_then(|b| b.as_u64())
                .map(|b| b as u32),
        }
    }

    /// Parse sampler
    fn parse_sampler(&self, json: &serde_json::Value) -> GltfSampler {
        GltfSampler {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            mag_filter: json
                .get("magFilter")
                .and_then(|f| f.as_u64())
                .and_then(|f| GltfTextureFilter::from_value(f as u32)),
            min_filter: json
                .get("minFilter")
                .and_then(|f| f.as_u64())
                .and_then(|f| GltfTextureFilter::from_value(f as u32)),
            wrap_s: json
                .get("wrapS")
                .and_then(|w| w.as_u64())
                .map(|w| GltfWrapMode::from_value(w as u32))
                .unwrap_or(GltfWrapMode::Repeat),
            wrap_t: json
                .get("wrapT")
                .and_then(|w| w.as_u64())
                .map(|w| GltfWrapMode::from_value(w as u32))
                .unwrap_or(GltfWrapMode::Repeat),
        }
    }

    /// Parse accessor
    fn parse_accessor(&self, json: &serde_json::Value) -> GltfAccessor {
        GltfAccessor {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            buffer_view: json
                .get("bufferView")
                .and_then(|b| b.as_u64())
                .map(|b| b as u32),
            byte_offset: json.get("byteOffset").and_then(|o| o.as_u64()).unwrap_or(0) as usize,
            component_type: json
                .get("componentType")
                .and_then(|c| c.as_u64())
                .and_then(|c| GltfComponentType::from_value(c as u32))
                .unwrap_or(GltfComponentType::Float),
            count: json.get("count").and_then(|c| c.as_u64()).unwrap_or(0) as usize,
            accessor_type: json
                .get("type")
                .and_then(|t| t.as_str())
                .and_then(GltfAccessorType::from_str)
                .unwrap_or(GltfAccessorType::Scalar),
            min: json
                .get("min")
                .and_then(|m| m.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_f64()).collect()),
            max: json
                .get("max")
                .and_then(|m| m.as_array())
                .map(|a| a.iter().filter_map(|v| v.as_f64()).collect()),
            sparse: None, // TODO: Parse sparse
        }
    }

    /// Parse buffer view
    fn parse_buffer_view(&self, json: &serde_json::Value) -> GltfBufferView {
        GltfBufferView {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            buffer: json.get("buffer").and_then(|b| b.as_u64()).unwrap_or(0) as u32,
            byte_offset: json.get("byteOffset").and_then(|o| o.as_u64()).unwrap_or(0) as usize,
            byte_length: json.get("byteLength").and_then(|l| l.as_u64()).unwrap_or(0) as usize,
            byte_stride: json
                .get("byteStride")
                .and_then(|s| s.as_u64())
                .map(|s| s as usize),
            target: json
                .get("target")
                .and_then(|t| t.as_u64())
                .and_then(|t| match t {
                    34962 => Some(GltfBufferViewTarget::ArrayBuffer),
                    34963 => Some(GltfBufferViewTarget::ElementArrayBuffer),
                    _ => None,
                }),
        }
    }

    /// Parse buffer
    fn parse_buffer(&self, json: &serde_json::Value) -> GltfBuffer {
        GltfBuffer {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            byte_length: json.get("byteLength").and_then(|l| l.as_u64()).unwrap_or(0) as usize,
            uri: json.get("uri").and_then(|u| u.as_str()).map(String::from),
            data: None,
        }
    }

    /// Parse animation
    fn parse_animation(&self, json: &serde_json::Value) -> GltfAnimation {
        let mut animation = GltfAnimation {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            channels: Vec::new(),
            samplers: Vec::new(),
        };

        if let Some(channels) = json.get("channels").and_then(|c| c.as_array()) {
            for channel in channels {
                animation
                    .channels
                    .push(self.parse_animation_channel(channel));
            }
        }

        if let Some(samplers) = json.get("samplers").and_then(|s| s.as_array()) {
            for sampler in samplers {
                animation
                    .samplers
                    .push(self.parse_animation_sampler(sampler));
            }
        }

        animation
    }

    /// Parse animation channel
    fn parse_animation_channel(&self, json: &serde_json::Value) -> GltfAnimationChannel {
        GltfAnimationChannel {
            sampler: json.get("sampler").and_then(|s| s.as_u64()).unwrap_or(0) as u32,
            target_node: json
                .get("target")
                .and_then(|t| t.get("node"))
                .and_then(|n| n.as_u64())
                .unwrap_or(0) as u32,
            target_path: json
                .get("target")
                .and_then(|t| t.get("path"))
                .and_then(|p| p.as_str())
                .map(GltfAnimationPath::from_str)
                .unwrap_or(GltfAnimationPath::Translation),
        }
    }

    /// Parse animation sampler
    fn parse_animation_sampler(&self, json: &serde_json::Value) -> GltfAnimationSampler {
        GltfAnimationSampler {
            input: json.get("input").and_then(|i| i.as_u64()).unwrap_or(0) as u32,
            output: json.get("output").and_then(|o| o.as_u64()).unwrap_or(0) as u32,
            interpolation: json
                .get("interpolation")
                .and_then(|i| i.as_str())
                .map(GltfInterpolation::from_str)
                .unwrap_or(GltfInterpolation::Linear),
        }
    }

    /// Parse skin
    fn parse_skin(&self, json: &serde_json::Value) -> GltfSkin {
        GltfSkin {
            name: json.get("name").and_then(|n| n.as_str()).map(String::from),
            inverse_bind_matrices: json
                .get("inverseBindMatrices")
                .and_then(|i| i.as_u64())
                .map(|i| i as u32),
            skeleton: json
                .get("skeleton")
                .and_then(|s| s.as_u64())
                .map(|s| s as u32),
            joints: json
                .get("joints")
                .and_then(|j| j.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_u64().map(|i| i as u32))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }

    /// Parse camera
    fn parse_camera(&self, json: &serde_json::Value) -> GltfCamera {
        let name = json.get("name").and_then(|n| n.as_str()).map(String::from);

        let camera_type = if let Some(persp) = json.get("perspective") {
            GltfCameraType::Perspective {
                aspect_ratio: persp.get("aspectRatio").and_then(|a| a.as_f64()),
                yfov: persp.get("yfov").and_then(|f| f.as_f64()).unwrap_or(1.0),
                zfar: persp.get("zfar").and_then(|f| f.as_f64()),
                znear: persp.get("znear").and_then(|n| n.as_f64()).unwrap_or(0.1),
            }
        } else if let Some(ortho) = json.get("orthographic") {
            GltfCameraType::Orthographic {
                xmag: ortho.get("xmag").and_then(|x| x.as_f64()).unwrap_or(1.0),
                ymag: ortho.get("ymag").and_then(|y| y.as_f64()).unwrap_or(1.0),
                zfar: ortho.get("zfar").and_then(|f| f.as_f64()).unwrap_or(100.0),
                znear: ortho.get("znear").and_then(|n| n.as_f64()).unwrap_or(0.1),
            }
        } else {
            GltfCameraType::Perspective {
                aspect_ratio: None,
                yfov: 1.0,
                zfar: None,
                znear: 0.1,
            }
        };

        GltfCamera { name, camera_type }
    }

    /// Load external buffers for .gltf format
    fn load_external_buffers(&mut self) -> Result<(), GltfError> {
        let base_path = Path::new(&self.filename).parent().unwrap_or(Path::new("."));

        for buffer in &mut self.document.buffers {
            if let Some(uri) = &buffer.uri {
                if uri.starts_with("data:") {
                    // Base64 encoded data URI
                    // TODO: Implement data URI parsing
                } else {
                    // External file
                    let buffer_path = base_path.join(uri);
                    let mut file = File::open(&buffer_path)?;
                    let mut data = Vec::new();
                    file.read_to_end(&mut data)?;
                    buffer.data = Some(data);
                }
            }
        }

        Ok(())
    }

    /// Get the document
    pub fn document(&self) -> &GltfDocument {
        &self.document
    }

    /// Get binary buffer (for GLB format)
    pub fn binary_buffer(&self) -> Option<&[u8]> {
        self.binary_buffer.as_deref()
    }

    /// Get accessor data
    pub fn get_accessor_data(&self, accessor_index: u32) -> Option<Vec<u8>> {
        let accessor = self.document.accessors.get(accessor_index as usize)?;
        let buffer_view_index = accessor.buffer_view?;
        let buffer_view = self.document.buffer_views.get(buffer_view_index as usize)?;
        let buffer = self.document.buffers.get(buffer_view.buffer as usize)?;

        let data = buffer.data.as_ref().or(self.binary_buffer.as_ref())?;

        let start = buffer_view.byte_offset + accessor.byte_offset;
        let end = start
            + accessor.count
                * accessor.accessor_type.component_count()
                * accessor.component_type.size();

        if end <= data.len() {
            Some(data[start..end].to_vec())
        } else {
            None
        }
    }

    /// Convert to TopoDsShape
    pub fn to_shape(&self) -> Result<TopoDsShape, GltfError> {
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }
}

impl GltfWrapMode {
    /// Create from value
    pub fn from_value(value: u32) -> Self {
        match value {
            33071 => GltfWrapMode::ClampToEdge,
            33648 => GltfWrapMode::MirroredRepeat,
            _ => GltfWrapMode::Repeat,
        }
    }
}

impl GltfTextureFilter {
    /// Create from value
    pub fn from_value(value: u32) -> Option<Self> {
        match value {
            9728 => Some(GltfTextureFilter::Nearest),
            9729 => Some(GltfTextureFilter::Linear),
            9984 => Some(GltfTextureFilter::NearestMipmapNearest),
            9985 => Some(GltfTextureFilter::LinearMipmapNearest),
            9986 => Some(GltfTextureFilter::NearestMipmapLinear),
            9987 => Some(GltfTextureFilter::LinearMipmapLinear),
            _ => None,
        }
    }
}

/// glTF writer for writing glTF/GLB files
pub struct GltfWriter {
    filename: String,
    document: GltfDocument,
    binary: bool,
}

impl GltfWriter {
    /// Create a new glTF writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: GltfDocument::default(),
            binary: false,
        }
    }

    /// Create a new GLB binary writer
    pub fn new_binary(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: GltfDocument::default(),
            binary: true,
        }
    }

    /// Set whether to use binary format
    pub fn set_binary(&mut self, binary: bool) {
        self.binary = binary;
    }

    /// Get the document
    pub fn document(&mut self) -> &mut GltfDocument {
        &mut self.document
    }

    /// Write to file
    pub fn write(&self) -> Result<(), GltfError> {
        if self.binary {
            self.write_glb()
        } else {
            self.write_gltf()
        }
    }

    /// Write GLB binary format
    fn write_glb(&self) -> Result<(), GltfError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        // Generate JSON
        let json = self.generate_json();
        let json_bytes = json.as_bytes();

        // Pad JSON to 4-byte alignment
        let json_padded_len = (json_bytes.len() + 3) & !3;
        let mut json_padded = json_bytes.to_vec();
        json_padded.resize(json_padded_len, b' ');

        // Generate binary buffer
        let binary_data = self.generate_binary_buffer();
        let binary_padded_len = (binary_data.len() + 3) & !3;
        let mut binary_padded = binary_data;
        binary_padded.resize(binary_padded_len, 0);

        // Calculate total file length
        let header_len = 12;
        let json_chunk_header_len = 8;
        let binary_chunk_header_len = if binary_padded.is_empty() { 0 } else { 8 };
        let total_len = header_len
            + json_chunk_header_len
            + json_padded_len
            + binary_chunk_header_len
            + binary_padded_len;

        // Write GLB header
        writer.write_all(&0x46546C67u32.to_le_bytes())?; // glTF magic
        writer.write_all(&2u32.to_le_bytes())?; // version
        writer.write_all(&(total_len as u32).to_le_bytes())?; // total length

        // Write JSON chunk
        writer.write_all(&(json_padded_len as u32).to_le_bytes())?;
        writer.write_all(&0x4E4F534Au32.to_le_bytes())?; // JSON
        writer.write_all(&json_padded)?;

        // Write binary chunk (if present)
        if !binary_padded.is_empty() {
            writer.write_all(&(binary_padded_len as u32).to_le_bytes())?;
            writer.write_all(&0x004E4942u32.to_le_bytes())?; // BIN
            writer.write_all(&binary_padded)?;
        }

        Ok(())
    }

    /// Write glTF JSON format
    fn write_gltf(&self) -> Result<(), GltfError> {
        let path = Path::new(&self.filename);
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        let json = self.generate_json();
        writer.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Generate JSON content
    fn generate_json(&self) -> String {
        let mut json = serde_json::json!({
            "asset": {
                "version": "2.0",
                "generator": "BrepRs glTF Export"
            }
        });

        if !self.document.scenes.is_empty() {
            json["scenes"] = serde_json::to_value(&self.document.scenes).unwrap();
        }

        if !self.document.nodes.is_empty() {
            json["nodes"] = serde_json::to_value(&self.document.nodes).unwrap();
        }

        if !self.document.meshes.is_empty() {
            json["meshes"] = serde_json::to_value(&self.document.meshes).unwrap();
        }

        if !self.document.materials.is_empty() {
            json["materials"] = serde_json::to_value(&self.document.materials).unwrap();
        }

        if !self.document.accessors.is_empty() {
            json["accessors"] = serde_json::to_value(&self.document.accessors).unwrap();
        }

        if !self.document.buffer_views.is_empty() {
            json["bufferViews"] = serde_json::to_value(&self.document.buffer_views).unwrap();
        }

        if !self.document.buffers.is_empty() {
            json["buffers"] = serde_json::to_value(&self.document.buffers).unwrap();
        }

        if let Some(scene) = self.document.scene {
            json["scene"] = serde_json::to_value(scene).unwrap();
        }

        serde_json::to_string_pretty(&json).unwrap_or_default()
    }

    /// Generate binary buffer
    fn generate_binary_buffer(&self) -> Vec<u8> {
        Vec::new()
    }

    /// Add a mesh to the document
    pub fn add_mesh(&mut self, mesh: GltfMesh) -> u32 {
        let index = self.document.meshes.len() as u32;
        self.document.meshes.push(mesh);
        index
    }

    /// Add a node to the document
    pub fn add_node(&mut self, node: GltfNode) -> u32 {
        let index = self.document.nodes.len() as u32;
        self.document.nodes.push(node);
        index
    }

    /// Add a scene to the document
    pub fn add_scene(&mut self, scene: GltfScene) -> u32 {
        let index = self.document.scenes.len() as u32;
        self.document.scenes.push(scene);
        index
    }

    /// Add a material to the document
    pub fn add_material(&mut self, material: GltfMaterial) -> u32 {
        let index = self.document.materials.len() as u32;
        self.document.materials.push(material);
        index
    }

    /// Set default scene
    pub fn set_default_scene(&mut self, scene_index: u32) {
        self.document.scene = Some(scene_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gltf_reader_creation() {
        let reader = GltfReader::new("test.gltf");
        assert_eq!(reader.filename, "test.gltf");
    }

    #[test]
    fn test_gltf_writer_creation() {
        let writer = GltfWriter::new("test.gltf");
        assert!(!writer.binary);
    }

    #[test]
    fn test_gltf_writer_binary_creation() {
        let writer = GltfWriter::new_binary("test.glb");
        assert!(writer.binary);
    }

    #[test]
    fn test_primitive_mode_default() {
        let mode = GltfPrimitiveMode::default();
        assert_eq!(mode, GltfPrimitiveMode::Triangles);
    }

    #[test]
    fn test_primitive_mode_from_value() {
        assert_eq!(
            GltfPrimitiveMode::from_value(4),
            GltfPrimitiveMode::Triangles
        );
        assert_eq!(GltfPrimitiveMode::from_value(0), GltfPrimitiveMode::Points);
        assert_eq!(GltfPrimitiveMode::from_value(1), GltfPrimitiveMode::Lines);
    }

    #[test]
    fn test_component_type_size() {
        assert_eq!(GltfComponentType::Byte.size(), 1);
        assert_eq!(GltfComponentType::UnsignedByte.size(), 1);
        assert_eq!(GltfComponentType::Short.size(), 2);
        assert_eq!(GltfComponentType::UnsignedShort.size(), 2);
        assert_eq!(GltfComponentType::UnsignedInt.size(), 4);
        assert_eq!(GltfComponentType::Float.size(), 4);
    }

    #[test]
    fn test_accessor_type_component_count() {
        assert_eq!(GltfAccessorType::Scalar.component_count(), 1);
        assert_eq!(GltfAccessorType::Vec2.component_count(), 2);
        assert_eq!(GltfAccessorType::Vec3.component_count(), 3);
        assert_eq!(GltfAccessorType::Vec4.component_count(), 4);
        assert_eq!(GltfAccessorType::Mat4.component_count(), 16);
    }

    #[test]
    fn test_transform_default() {
        let transform = GltfTransform::default();
        assert_eq!(transform.translation, [0.0, 0.0, 0.0]);
        assert_eq!(transform.rotation, [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_alpha_mode_from_str() {
        assert_eq!(GltfAlphaMode::from_str("OPAQUE"), GltfAlphaMode::Opaque);
        assert_eq!(GltfAlphaMode::from_str("MASK"), GltfAlphaMode::Mask);
        assert_eq!(GltfAlphaMode::from_str("BLEND"), GltfAlphaMode::Blend);
    }

    #[test]
    fn test_attribute_from_str() {
        assert_eq!(GltfAttribute::from_str("POSITION"), GltfAttribute::Position);
        assert_eq!(GltfAttribute::from_str("NORMAL"), GltfAttribute::Normal);
        assert_eq!(
            GltfAttribute::from_str("TEXCOORD_0"),
            GltfAttribute::TexCoord0
        );
    }

    #[test]
    fn test_writer_add_mesh() {
        let mut writer = GltfWriter::new("test.gltf");
        let mesh = GltfMesh {
            name: Some("TestMesh".to_string()),
            ..Default::default()
        };
        let index = writer.add_mesh(mesh);
        assert_eq!(index, 0);
        assert_eq!(writer.document.meshes.len(), 1);
    }

    #[test]
    fn test_writer_add_node() {
        let mut writer = GltfWriter::new("test.gltf");
        let node = GltfNode {
            name: Some("TestNode".to_string()),
            ..Default::default()
        };
        let index = writer.add_node(node);
        assert_eq!(index, 0);
        assert_eq!(writer.document.nodes.len(), 1);
    }

    #[test]
    fn test_writer_add_scene() {
        let mut writer = GltfWriter::new("test.gltf");
        let scene = GltfScene {
            name: Some("TestScene".to_string()),
            ..Default::default()
        };
        let index = writer.add_scene(scene);
        assert_eq!(index, 0);
        assert_eq!(writer.document.scenes.len(), 1);
    }
}
