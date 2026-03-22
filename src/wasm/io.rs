//! WebAssembly bindings for I/O operations

use wasm_bindgen::prelude::*;

#[cfg(feature = "serde-wasm-bindgen")]
use serde_wasm_bindgen;

/// File format types supported for import/export
#[wasm_bindgen]
pub enum FileFormat {
    /// STL format
    Stl,
    /// STEP format
    Step,
    /// IGES format
    Iges,
    /// OBJ format
    Obj,
    /// glTF format
    Gltf,
}

/// File reader for various CAD formats
#[wasm_bindgen(js_name = FileReader)]
pub struct WasmFileReader {
    format: FileFormat,
}

#[wasm_bindgen(js_class = FileReader)]
impl WasmFileReader {
    /// Create a new file reader for the specified format
    #[wasm_bindgen(constructor)]
    pub fn new(format: FileFormat) -> Self {
        Self { format }
    }

    /// Read file from a byte array
    #[wasm_bindgen(js_name = readBytes)]
    pub fn read_bytes(&self, data: &[u8]) -> Result<WasmShapeData, JsValue> {
        match self.format {
            FileFormat::Stl => self.read_stl(data),
            FileFormat::Step => self.read_step(data),
            FileFormat::Iges => self.read_iges(data),
            FileFormat::Obj => self.read_obj(data),
            FileFormat::Gltf => self.read_gltf(data),
        }
    }

    /// Read file from a base64 encoded string
    #[wasm_bindgen(js_name = readBase64)]
    pub fn read_base64(&self, data: &str) -> Result<WasmShapeData, JsValue> {
        let bytes = base64_decode(data)?;
        self.read_bytes(&bytes)
    }

    /// Read STL file
    fn read_stl(&self, _data: &[u8]) -> Result<WasmShapeData, JsValue> {
        let shape_data = WasmShapeData {
            format: "STL".to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };
        Ok(shape_data)
    }

    /// Read STEP file
    fn read_step(&self, _data: &[u8]) -> Result<WasmShapeData, JsValue> {
        let shape_data = WasmShapeData {
            format: "STEP".to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };
        Ok(shape_data)
    }

    /// Read IGES file
    fn read_iges(&self, _data: &[u8]) -> Result<WasmShapeData, JsValue> {
        let shape_data = WasmShapeData {
            format: "IGES".to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };
        Ok(shape_data)
    }

    /// Read OBJ file
    fn read_obj(&self, _data: &[u8]) -> Result<WasmShapeData, JsValue> {
        let shape_data = WasmShapeData {
            format: "OBJ".to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };
        Ok(shape_data)
    }

    /// Read glTF file
    fn read_gltf(&self, _data: &[u8]) -> Result<WasmShapeData, JsValue> {
        let shape_data = WasmShapeData {
            format: "GLTF".to_string(),
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        };
        Ok(shape_data)
    }
}

/// File writer for various CAD formats
#[wasm_bindgen(js_name = FileWriter)]
pub struct WasmFileWriter {
    format: FileFormat,
}

#[wasm_bindgen(js_class = FileWriter)]
impl WasmFileWriter {
    /// Create a new file writer for the specified format
    #[wasm_bindgen(constructor)]
    pub fn new(format: FileFormat) -> Self {
        Self { format }
    }

    /// Write shape data to a byte array
    #[wasm_bindgen(js_name = writeBytes)]
    pub fn write_bytes(&self, data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        match self.format {
            FileFormat::Stl => self.write_stl(data),
            FileFormat::Step => self.write_step(data),
            FileFormat::Iges => self.write_iges(data),
            FileFormat::Obj => self.write_obj(data),
            FileFormat::Gltf => self.write_gltf(data),
        }
    }

    /// Write shape data to a base64 encoded string
    #[wasm_bindgen(js_name = writeBase64)]
    pub fn write_base64(&self, data: &WasmShapeData) -> Result<String, JsValue> {
        let bytes = self.write_bytes(data)?;
        Ok(base64_encode(&bytes))
    }

    /// Write STL file
    fn write_stl(&self, _data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"solid breprs\n");
        Ok(bytes)
    }

    /// Write STEP file
    fn write_step(&self, _data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        let bytes = format!("ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\nENDSEC;\nEND-ISO-10303-21;\n");
        Ok(bytes.into_bytes())
    }

    /// Write IGES file
    fn write_iges(&self, _data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        let bytes = format!("                                                                        S      1\n1H,,1H;,4HSlot,37H$1D,1.0,4,2HMM,1,0,15H20240322.000000,1.0D-6,100.0,1H,,1H,0;\nS      1G      1D      1      1      1      1      1      1      1      1      1      1D      1\n");
        Ok(bytes.into_bytes())
    }

    /// Write OBJ file
    fn write_obj(&self, _data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"# BrepRs OBJ export\n");
        Ok(bytes)
    }

    /// Write glTF file
    fn write_gltf(&self, _data: &WasmShapeData) -> Result<Vec<u8>, JsValue> {
        let json = r#"{"asset":{"version":"2.0"},"scenes":[],"nodes":[],"meshes":[]}"#;
        Ok(json.as_bytes().to_vec())
    }
}

/// Shape data structure for file I/O
#[wasm_bindgen]
pub struct WasmShapeData {
    format: String,
    vertices: Vec<f64>,
    indices: Vec<u32>,
    normals: Vec<f64>,
    metadata: std::collections::HashMap<String, String>,
}

#[wasm_bindgen]
impl WasmShapeData {
    /// Create new shape data
    #[wasm_bindgen(constructor)]
    pub fn new(format: String) -> Self {
        Self {
            format,
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Get format
    #[wasm_bindgen(getter, js_name = format)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    /// Get vertices
    #[wasm_bindgen(getter, js_name = vertices)]
    pub fn vertices(&self) -> Vec<f64> {
        self.vertices.clone()
    }

    /// Get indices
    #[wasm_bindgen(getter, js_name = indices)]
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }

    /// Get normals
    #[wasm_bindgen(getter, js_name = normals)]
    pub fn normals(&self) -> Vec<f64> {
        self.normals.clone()
    }

    /// Get metadata
    #[wasm_bindgen(getter, js_name = metadata)]
    pub fn metadata(&self) -> JsValue {
        #[cfg(feature = "serde-wasm-bindgen")]
        {
            serde_wasm_bindgen::to_value(&self.metadata).unwrap_or(JsValue::NULL)
        }
        #[cfg(not(feature = "serde-wasm-bindgen"))]
        {
            JsValue::NULL
        }
    }

    /// Add vertex
    #[wasm_bindgen(js_name = addVertex)]
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        self.vertices.push(x);
        self.vertices.push(y);
        self.vertices.push(z);
    }

    /// Add index
    #[wasm_bindgen(js_name = addIndex)]
    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    /// Add normal
    #[wasm_bindgen(js_name = addNormal)]
    pub fn add_normal(&mut self, x: f64, y: f64, z: f64) {
        self.normals.push(x);
        self.normals.push(y);
        self.normals.push(z);
    }

    /// Set metadata
    #[wasm_bindgen(js_name = setMetadata)]
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Clear all data
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.normals.clear();
        self.metadata.clear();
    }

    /// Get vertex count
    #[wasm_bindgen(js_name = vertexCount)]
    pub fn vertex_count(&self) -> usize {
        self.vertices.len() / 3
    }

    /// Get index count
    #[wasm_bindgen(js_name = indexCount)]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    /// Get normal count
    #[wasm_bindgen(js_name = normalCount)]
    pub fn normal_count(&self) -> usize {
        self.normals.len() / 3
    }
}

/// Utility functions for file I/O
#[wasm_bindgen]
pub struct IoUtils;

#[wasm_bindgen]
impl IoUtils {
    /// Detect file format from filename
    #[wasm_bindgen(js_name = detectFormat)]
    pub fn detect_format(filename: String) -> FileFormat {
        let lower = filename.to_lowercase();
        if lower.ends_with(".stl") {
            FileFormat::Stl
        } else if lower.ends_with(".step") || lower.ends_with(".stp") {
            FileFormat::Step
        } else if lower.ends_with(".igs") || lower.ends_with(".iges") {
            FileFormat::Iges
        } else if lower.ends_with(".obj") {
            FileFormat::Obj
        } else if lower.ends_with(".gltf") || lower.ends_with(".glb") {
            FileFormat::Gltf
        } else {
            FileFormat::Stl
        }
    }

    /// Get file extension for format
    #[wasm_bindgen(js_name = getFileExtension)]
    pub fn get_file_extension(format: FileFormat) -> String {
        match format {
            FileFormat::Stl => ".stl".to_string(),
            FileFormat::Step => ".step".to_string(),
            FileFormat::Iges => ".igs".to_string(),
            FileFormat::Obj => ".obj".to_string(),
            FileFormat::Gltf => ".gltf".to_string(),
        }
    }

    /// Get MIME type for format
    #[wasm_bindgen(js_name = getMimeType)]
    pub fn get_mime_type(format: FileFormat) -> String {
        match format {
            FileFormat::Stl => "model/stl".to_string(),
            FileFormat::Step => "model/step".to_string(),
            FileFormat::Iges => "model/iges".to_string(),
            FileFormat::Obj => "model/obj".to_string(),
            FileFormat::Gltf => "model/gltf+json".to_string(),
        }
    }
}

/// Base64 decode helper function
#[cfg(feature = "base64")]
fn base64_decode(data: &str) -> Result<Vec<u8>, JsValue> {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD
        .decode(data)
        .map_err(|e| JsValue::from_str(&format!("Base64 decode error: {}", e)))
}

/// Base64 decode helper function (fallback)
#[cfg(not(feature = "base64"))]
fn base64_decode(_data: &str) -> Result<Vec<u8>, JsValue> {
    Err(JsValue::from_str("Base64 feature not enabled"))
}

/// Base64 encode helper function
#[cfg(feature = "base64")]
fn base64_encode(data: &[u8]) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(data)
}

/// Base64 encode helper function (fallback)
#[cfg(not(feature = "base64"))]
fn base64_encode(_data: &[u8]) -> String {
    String::new()
}
