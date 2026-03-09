//! OBJ (Wavefront OBJ) file format support
//!
//! This module provides functionality for reading and writing OBJ files,
//! including MTL material file support.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use crate::geometry::Point;
use crate::topology::{shape_enum::ShapeType, topods_shape::TopoDsShape};

/// OBJ file format error types
#[derive(Debug)]
pub enum ObjError {
    /// File I/O error
    IoError(std::io::Error),
    /// Invalid OBJ file format
    InvalidFormat,
    /// Invalid vertex data
    InvalidVertex,
    /// Invalid face data
    InvalidFace,
    /// Invalid normal data
    InvalidNormal,
    /// Invalid texture coordinate
    InvalidTexCoord,
    /// Invalid material reference
    InvalidMaterial,
    /// MTL file error
    MtlError(String),
    /// Parsing error
    ParsingError(String),
}

impl From<std::io::Error> for ObjError {
    fn from(err: std::io::Error) -> Self {
        ObjError::IoError(err)
    }
}

/// OBJ vertex
#[derive(Debug, Clone, Default)]
pub struct ObjVertex {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
    /// W coordinate (optional, default 1.0)
    pub w: f64,
}

impl ObjVertex {
    /// Create a new vertex
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Create a vertex with W coordinate
    pub fn with_w(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Convert to Point
    pub fn to_point(&self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
}

/// OBJ texture coordinate
#[derive(Debug, Clone, Default)]
pub struct ObjTexCoord {
    /// U coordinate
    pub u: f64,
    /// V coordinate (optional)
    pub v: Option<f64>,
    /// W coordinate (optional)
    pub w: Option<f64>,
}

impl ObjTexCoord {
    /// Create a new texture coordinate
    pub fn new(u: f64) -> Self {
        Self {
            u,
            v: None,
            w: None,
        }
    }

    /// Create a texture coordinate with V
    pub fn with_v(u: f64, v: f64) -> Self {
        Self {
            u,
            v: Some(v),
            w: None,
        }
    }

    /// Create a texture coordinate with V and W
    pub fn with_w(u: f64, v: f64, w: f64) -> Self {
        Self {
            u,
            v: Some(v),
            w: Some(w),
        }
    }
}

/// OBJ normal vector
#[derive(Debug, Clone, Default)]
pub struct ObjNormal {
    /// X component
    pub x: f64,
    /// Y component
    pub y: f64,
    /// Z component
    pub z: f64,
}

impl ObjNormal {
    /// Create a new normal
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Normalize the normal
    pub fn normalize(&self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Self {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            }
        }
    }
}

/// OBJ face vertex reference
#[derive(Debug, Clone, Default)]
pub struct ObjFaceVertex {
    /// Vertex index (1-based in OBJ, converted to 0-based)
    pub vertex_index: usize,
    /// Texture coordinate index (optional)
    pub texcoord_index: Option<usize>,
    /// Normal index (optional)
    pub normal_index: Option<usize>,
}

impl ObjFaceVertex {
    /// Create a new face vertex reference
    pub fn new(vertex_index: usize) -> Self {
        Self {
            vertex_index,
            texcoord_index: None,
            normal_index: None,
        }
    }

    /// Create with texture coordinate
    pub fn with_texcoord(vertex_index: usize, texcoord_index: usize) -> Self {
        Self {
            vertex_index,
            texcoord_index: Some(texcoord_index),
            normal_index: None,
        }
    }

    /// Create with texture coordinate and normal
    pub fn with_texcoord_and_normal(
        vertex_index: usize,
        texcoord_index: usize,
        normal_index: usize,
    ) -> Self {
        Self {
            vertex_index,
            texcoord_index: Some(texcoord_index),
            normal_index: Some(normal_index),
        }
    }

    /// Create with normal only
    pub fn with_normal(vertex_index: usize, normal_index: usize) -> Self {
        Self {
            vertex_index,
            texcoord_index: None,
            normal_index: Some(normal_index),
        }
    }
}

/// OBJ face
#[derive(Debug, Clone, Default)]
pub struct ObjFace {
    /// Vertex references
    pub vertices: Vec<ObjFaceVertex>,
    /// Material name
    pub material: Option<String>,
    /// Smoothing group
    pub smoothing_group: Option<u32>,
}

impl ObjFace {
    /// Create a new face
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex reference
    pub fn add_vertex(&mut self, vertex: ObjFaceVertex) {
        self.vertices.push(vertex);
    }

    /// Check if the face is a triangle
    pub fn is_triangle(&self) -> bool {
        self.vertices.len() == 3
    }

    /// Check if the face is a quad
    pub fn is_quad(&self) -> bool {
        self.vertices.len() == 4
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Triangulate the face (fan triangulation)
    pub fn triangulate(&self) -> Vec<ObjFace> {
        if self.vertices.len() <= 3 {
            return vec![self.clone()];
        }

        let mut triangles = Vec::new();
        for i in 1..self.vertices.len() - 1 {
            let mut tri = ObjFace::new();
            tri.vertices.push(self.vertices[0].clone());
            tri.vertices.push(self.vertices[i].clone());
            tri.vertices.push(self.vertices[i + 1].clone());
            tri.material = self.material.clone();
            tri.smoothing_group = self.smoothing_group;
            triangles.push(tri);
        }
        triangles
    }
}

/// OBJ line element
#[derive(Debug, Clone, Default)]
pub struct ObjLine {
    /// Vertex indices
    pub vertices: Vec<usize>,
}

/// OBJ point element
#[derive(Debug, Clone, Default)]
pub struct ObjPoint {
    /// Vertex indices
    pub vertices: Vec<usize>,
}

/// OBJ group
#[derive(Debug, Clone, Default)]
pub struct ObjGroup {
    /// Group name
    pub name: String,
    /// Faces in this group
    pub faces: Vec<ObjFace>,
    /// Lines in this group
    pub lines: Vec<ObjLine>,
    /// Points in this group
    pub points: Vec<ObjPoint>,
}

impl ObjGroup {
    /// Create a new group
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Add a face
    pub fn add_face(&mut self, face: ObjFace) {
        self.faces.push(face);
    }
}

/// OBJ object
#[derive(Debug, Clone, Default)]
pub struct ObjObject {
    /// Object name
    pub name: String,
    /// Groups in this object
    pub groups: Vec<ObjGroup>,
}

impl ObjObject {
    /// Create a new object
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
}

/// MTL material
#[derive(Debug, Clone, Default)]
pub struct MtlMaterial {
    /// Material name
    pub name: String,
    /// Ambient color
    pub ka: [f64; 3],
    /// Diffuse color
    pub kd: [f64; 3],
    /// Specular color
    pub ks: [f64; 3],
    /// Emissive color
    pub ke: [f64; 3],
    /// Specular exponent (shininess)
    pub ns: f64,
    /// Dissolve (transparency)
    pub d: f64,
    /// Transparency (inverse of dissolve)
    pub tr: f64,
    /// Illumination model
    pub illum: u32,
    /// Optical density (refraction index)
    pub ni: f64,
    /// Ambient texture map
    pub map_ka: Option<String>,
    /// Diffuse texture map
    pub map_kd: Option<String>,
    /// Specular texture map
    pub map_ks: Option<String>,
    /// Emissive texture map
    pub map_ke: Option<String>,
    /// Specular exponent texture map
    pub map_ns: Option<String>,
    /// Normal/bump map
    pub map_bump: Option<String>,
    /// Normal map (alternative)
    pub norm: Option<String>,
    /// Displacement map
    pub disp: Option<String>,
    /// Roughness (PBR extension)
    pub roughness: Option<f64>,
    /// Metallic (PBR extension)
    pub metallic: Option<f64>,
    /// Roughness texture map (PBR extension)
    pub map_pr: Option<String>,
    /// Metallic texture map (PBR extension)
    pub map_pm: Option<String>,
}

impl MtlMaterial {
    /// Create a new material
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            d: 1.0,
            tr: 0.0,
            ns: 0.0,
            ni: 1.0,
            illum: 0,
            ..Default::default()
        }
    }
}

/// OBJ document
#[derive(Debug, Clone, Default)]
pub struct ObjDocument {
    /// Vertices
    pub vertices: Vec<ObjVertex>,
    /// Texture coordinates
    pub texcoords: Vec<ObjTexCoord>,
    /// Normals
    pub normals: Vec<ObjNormal>,
    /// Objects
    pub objects: Vec<ObjObject>,
    /// Groups (not in any object)
    pub groups: Vec<ObjGroup>,
    /// Materials
    pub materials: HashMap<String, MtlMaterial>,
    /// Current smoothing group
    pub current_smoothing_group: Option<u32>,
    /// Current material
    pub current_material: Option<String>,
    /// Default group name
    pub default_group_name: String,
}

impl ObjDocument {
    /// Create a new OBJ document
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, vertex: ObjVertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    /// Add a texture coordinate
    pub fn add_texcoord(&mut self, texcoord: ObjTexCoord) -> usize {
        let index = self.texcoords.len();
        self.texcoords.push(texcoord);
        index
    }

    /// Add a normal
    pub fn add_normal(&mut self, normal: ObjNormal) -> usize {
        let index = self.normals.len();
        self.normals.push(normal);
        index
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Get face count
    pub fn face_count(&self) -> usize {
        let mut count = 0;
        for object in &self.objects {
            for group in &object.groups {
                count += group.faces.len();
            }
        }
        for group in &self.groups {
            count += group.faces.len();
        }
        count
    }
}

/// OBJ reader for reading OBJ files
pub struct ObjReader {
    filename: String,
    document: ObjDocument,
    mtl_search_paths: Vec<String>,
}

impl ObjReader {
    /// Create a new OBJ reader
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: ObjDocument::new(),
            mtl_search_paths: Vec::new(),
        }
    }

    /// Add MTL search path
    pub fn add_mtl_search_path(&mut self, path: &str) {
        self.mtl_search_paths.push(path.to_string());
    }

    /// Read an OBJ file
    pub fn read(&mut self) -> Result<&ObjDocument, ObjError> {
        let path = Path::new(&self.filename);
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Initialize default group
        self.document.default_group_name = "default".to_string();
        let default_group = ObjGroup::new(&self.document.default_group_name);
        self.document.groups.push(default_group);

        let mut current_object_name = String::new();
        let mut current_group_name = self.document.default_group_name.clone();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => self.parse_vertex(&parts[1..])?,
                "vt" => self.parse_texcoord(&parts[1..])?,
                "vn" => self.parse_normal(&parts[1..])?,
                "f" => self.parse_face(&parts[1..], &current_group_name)?,
                "l" => self.parse_line(&parts[1..])?,
                "p" => self.parse_point(&parts[1..])?,
                "o" => {
                    current_object_name = if parts.len() > 1 {
                        parts[1..].join(" ")
                    } else {
                        String::new()
                    };
                    self.add_object(&current_object_name);
                }
                "g" => {
                    current_group_name = if parts.len() > 1 {
                        parts[1..].join(" ")
                    } else {
                        self.document.default_group_name.clone()
                    };
                    self.add_group(&current_group_name, &current_object_name);
                }
                "s" => {
                    self.document.current_smoothing_group = if parts.len() > 1 {
                        if parts[1] == "off" || parts[1] == "0" {
                            None
                        } else {
                            parts[1].parse().ok()
                        }
                    } else {
                        None
                    };
                }
                "usemtl" => {
                    self.document.current_material = if parts.len() > 1 {
                        Some(parts[1..].join(" "))
                    } else {
                        None
                    };
                }
                "mtllib" => {
                    if parts.len() > 1 {
                        let mtl_filename = parts[1..].join(" ");
                        let _ = self.load_mtl(&mtl_filename);
                    }
                }
                _ => {}
            }
        }

        Ok(&self.document)
    }

    /// Parse vertex
    fn parse_vertex(&mut self, parts: &[&str]) -> Result<(), ObjError> {
        if parts.len() < 3 {
            return Err(ObjError::InvalidVertex);
        }

        let x = parts[0].parse().map_err(|_| ObjError::InvalidVertex)?;
        let y = parts[1].parse().map_err(|_| ObjError::InvalidVertex)?;
        let z = parts[2].parse().map_err(|_| ObjError::InvalidVertex)?;
        let w = if parts.len() > 3 {
            parts[3].parse().unwrap_or(1.0)
        } else {
            1.0
        };

        self.document.add_vertex(ObjVertex::with_w(x, y, z, w));
        Ok(())
    }

    /// Parse texture coordinate
    fn parse_texcoord(&mut self, parts: &[&str]) -> Result<(), ObjError> {
        if parts.is_empty() {
            return Err(ObjError::InvalidTexCoord);
        }

        let u = parts[0].parse().map_err(|_| ObjError::InvalidTexCoord)?;
        let v = parts.get(1).and_then(|s| s.parse().ok());
        let w = parts.get(2).and_then(|s| s.parse().ok());

        let texcoord = match (v, w) {
            (Some(v), Some(w)) => ObjTexCoord::with_w(u, v, w),
            (Some(v), None) => ObjTexCoord::with_v(u, v),
            _ => ObjTexCoord::new(u),
        };

        self.document.add_texcoord(texcoord);
        Ok(())
    }

    /// Parse normal
    fn parse_normal(&mut self, parts: &[&str]) -> Result<(), ObjError> {
        if parts.len() < 3 {
            return Err(ObjError::InvalidNormal);
        }

        let x = parts[0].parse().map_err(|_| ObjError::InvalidNormal)?;
        let y = parts[1].parse().map_err(|_| ObjError::InvalidNormal)?;
        let z = parts[2].parse().map_err(|_| ObjError::InvalidNormal)?;

        self.document.add_normal(ObjNormal::new(x, y, z));
        Ok(())
    }

    /// Parse face
    fn parse_face(&mut self, parts: &[&str], group_name: &str) -> Result<(), ObjError> {
        if parts.len() < 3 {
            return Err(ObjError::InvalidFace);
        }

        let mut face = ObjFace::new();
        face.material = self.document.current_material.clone();
        face.smoothing_group = self.document.current_smoothing_group;

        for part in parts {
            let vertex = self.parse_face_vertex(part)?;
            face.add_vertex(vertex);
        }

        // Add face to current group
        if let Some(group) = self.find_group_mut(group_name) {
            group.add_face(face);
        }

        Ok(())
    }

    /// Parse face vertex reference
    fn parse_face_vertex(&self, part: &str) -> Result<ObjFaceVertex, ObjError> {
        let components: Vec<&str> = part.split('/').collect();

        let vertex_index = components[0]
            .parse::<isize>()
            .map_err(|_| ObjError::InvalidFace)?;

        // Convert to 0-based index (OBJ uses 1-based, negative indices are relative)
        let vertex_index = if vertex_index < 0 {
            (self.document.vertices.len() as isize + vertex_index) as usize
        } else {
            (vertex_index - 1) as usize
        };

        let texcoord_index = if components.len() > 1 && !components[1].is_empty() {
            let idx = components[1]
                .parse::<isize>()
                .map_err(|_| ObjError::InvalidFace)?;
            Some(if idx < 0 {
                (self.document.texcoords.len() as isize + idx) as usize
            } else {
                (idx - 1) as usize
            })
        } else {
            None
        };

        let normal_index = if components.len() > 2 && !components[2].is_empty() {
            let idx = components[2]
                .parse::<isize>()
                .map_err(|_| ObjError::InvalidFace)?;
            Some(if idx < 0 {
                (self.document.normals.len() as isize + idx) as usize
            } else {
                (idx - 1) as usize
            })
        } else {
            None
        };

        Ok(ObjFaceVertex {
            vertex_index,
            texcoord_index,
            normal_index,
        })
    }

    /// Parse line element
    fn parse_line(&mut self, _parts: &[&str]) -> Result<(), ObjError> {
        Ok(())
    }

    /// Parse point element
    fn parse_point(&mut self, _parts: &[&str]) -> Result<(), ObjError> {
        Ok(())
    }

    /// Add object
    fn add_object(&mut self, name: &str) {
        if !self.document.objects.iter().any(|o| o.name == name) {
            self.document.objects.push(ObjObject::new(name));
        }
    }

    /// Add group
    fn add_group(&mut self, name: &str, _object_name: &str) {
        if !self.document.groups.iter().any(|g| g.name == name) {
            self.document.groups.push(ObjGroup::new(name));
        }
    }

    /// Find group by name (mutable)
    fn find_group_mut(&mut self, name: &str) -> Option<&mut ObjGroup> {
        self.document.groups.iter_mut().find(|g| g.name == name)
    }

    /// Load MTL file
    fn load_mtl(&mut self, filename: &str) -> Result<(), ObjError> {
        let base_path = Path::new(&self.filename).parent().unwrap_or(Path::new("."));
        let mtl_path = base_path.join(filename);

        let file = File::open(&mtl_path).map_err(|e| ObjError::MtlError(e.to_string()))?;
        let reader = BufReader::new(file);

        let mut current_material: Option<MtlMaterial> = None;

        for line in reader.lines() {
            let line = line.map_err(|e| ObjError::MtlError(e.to_string()))?;
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "newmtl" => {
                    if let Some(mat) = current_material.take() {
                        self.document.materials.insert(mat.name.clone(), mat);
                    }
                    let name = if parts.len() > 1 {
                        parts[1..].join(" ")
                    } else {
                        "unnamed".to_string()
                    };
                    current_material = Some(MtlMaterial::new(&name));
                }
                "Ka" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.ka = [
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            ];
                        }
                    }
                }
                "Kd" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.kd = [
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            ];
                        }
                    }
                }
                "Ks" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.ks = [
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            ];
                        }
                    }
                }
                "Ke" => {
                    if let Some(ref mut mat) = current_material {
                        if parts.len() >= 4 {
                            mat.ke = [
                                parts[1].parse().unwrap_or(0.0),
                                parts[2].parse().unwrap_or(0.0),
                                parts[3].parse().unwrap_or(0.0),
                            ];
                        }
                    }
                }
                "Ns" => {
                    if let Some(ref mut mat) = current_material {
                        mat.ns = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                    }
                }
                "d" => {
                    if let Some(ref mut mat) = current_material {
                        mat.d = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                    }
                }
                "Tr" => {
                    if let Some(ref mut mat) = current_material {
                        mat.tr = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                    }
                }
                "illum" => {
                    if let Some(ref mut mat) = current_material {
                        mat.illum = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                    }
                }
                "Ni" => {
                    if let Some(ref mut mat) = current_material {
                        mat.ni = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(1.0);
                    }
                }
                "map_Ka" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_ka = Some(parts[1..].join(" "));
                    }
                }
                "map_Kd" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_kd = Some(parts[1..].join(" "));
                    }
                }
                "map_Ks" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_ks = Some(parts[1..].join(" "));
                    }
                }
                "map_Ke" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_ke = Some(parts[1..].join(" "));
                    }
                }
                "map_Ns" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_ns = Some(parts[1..].join(" "));
                    }
                }
                "map_Bump" | "bump" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_bump = Some(parts[1..].join(" "));
                    }
                }
                "norm" => {
                    if let Some(ref mut mat) = current_material {
                        mat.norm = Some(parts[1..].join(" "));
                    }
                }
                "disp" => {
                    if let Some(ref mut mat) = current_material {
                        mat.disp = Some(parts[1..].join(" "));
                    }
                }
                "Pr" => {
                    if let Some(ref mut mat) = current_material {
                        mat.roughness = parts.get(1).and_then(|s| s.parse().ok());
                    }
                }
                "Pm" => {
                    if let Some(ref mut mat) = current_material {
                        mat.metallic = parts.get(1).and_then(|s| s.parse().ok());
                    }
                }
                "map_Pr" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_pr = Some(parts[1..].join(" "));
                    }
                }
                "map_Pm" => {
                    if let Some(ref mut mat) = current_material {
                        mat.map_pm = Some(parts[1..].join(" "));
                    }
                }
                _ => {}
            }
        }

        if let Some(mat) = current_material {
            self.document.materials.insert(mat.name.clone(), mat);
        }

        Ok(())
    }

    /// Get the document
    pub fn document(&self) -> &ObjDocument {
        &self.document
    }

    /// Convert to TopoDsShape
    pub fn to_shape(&self) -> Result<TopoDsShape, ObjError> {
        let shape = TopoDsShape::new(ShapeType::Compound);
        Ok(shape)
    }
}

/// OBJ writer for writing OBJ files
pub struct ObjWriter {
    filename: String,
    document: ObjDocument,
    write_mtl: bool,
    mtl_filename: Option<String>,
}

impl ObjWriter {
    /// Create a new OBJ writer
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            document: ObjDocument::new(),
            write_mtl: true,
            mtl_filename: None,
        }
    }

    /// Set whether to write MTL file
    pub fn set_write_mtl(&mut self, write_mtl: bool) {
        self.write_mtl = write_mtl;
    }

    /// Set MTL filename
    pub fn set_mtl_filename(&mut self, filename: &str) {
        self.mtl_filename = Some(filename.to_string());
    }

    /// Get the document
    pub fn document(&mut self) -> &mut ObjDocument {
        &mut self.document
    }

    /// Write OBJ file
    pub fn write(&self) -> Result<(), ObjError> {
        let path = Path::new(&self.filename);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);

        // Write header comment
        writeln!(writer, "# OBJ file exported by BrepRs")?;
        writeln!(writer, "# File: {}", self.filename)?;
        writeln!(writer)?;

        // Write MTL library reference
        if self.write_mtl && !self.document.materials.is_empty() {
            let mtl_name = self
                .mtl_filename
                .as_ref()
                .map(|s| s.clone())
                .unwrap_or_else(|| {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("materials");
                    format!("{}.mtl", stem)
                });
            writeln!(writer, "mtllib {}", mtl_name)?;
            writeln!(writer)?;
        }

        // Write vertices
        writeln!(writer, "# {} vertices", self.document.vertices.len())?;
        for vertex in &self.document.vertices {
            writeln!(writer, "v {} {} {}", vertex.x, vertex.y, vertex.z)?;
        }
        writeln!(writer)?;

        // Write texture coordinates
        if !self.document.texcoords.is_empty() {
            writeln!(
                writer,
                "# {} texture coordinates",
                self.document.texcoords.len()
            )?;
            for texcoord in &self.document.texcoords {
                match (texcoord.v, texcoord.w) {
                    (Some(v), Some(w)) => writeln!(writer, "vt {} {} {}", texcoord.u, v, w)?,
                    (Some(v), None) => writeln!(writer, "vt {} {}", texcoord.u, v)?,
                    _ => writeln!(writer, "vt {}", texcoord.u)?,
                }
            }
            writeln!(writer)?;
        }

        // Write normals
        if !self.document.normals.is_empty() {
            writeln!(writer, "# {} normals", self.document.normals.len())?;
            for normal in &self.document.normals {
                writeln!(writer, "vn {} {} {}", normal.x, normal.y, normal.z)?;
            }
            writeln!(writer)?;
        }

        // Write groups and faces
        let mut current_material: Option<&str> = None;
        let mut current_smoothing: Option<u32> = None;

        for group in &self.document.groups {
            writeln!(writer, "g {}", group.name)?;

            for face in &group.faces {
                // Update material if changed
                if face.material.as_deref() != current_material {
                    if let Some(ref mat_name) = face.material {
                        writeln!(writer, "usemtl {}", mat_name)?;
                        current_material = Some(mat_name.as_str());
                    } else {
                        current_material = None;
                    }
                }

                // Update smoothing group if changed
                if face.smoothing_group != current_smoothing {
                    if let Some(sg) = face.smoothing_group {
                        writeln!(writer, "s {}", sg)?;
                    } else {
                        writeln!(writer, "s off")?;
                    }
                    current_smoothing = face.smoothing_group;
                }

                // Write face
                write!(writer, "f")?;
                for vertex in &face.vertices {
                    match (vertex.texcoord_index, vertex.normal_index) {
                        (Some(ti), Some(ni)) => {
                            write!(writer, " {}/{}/{}", vertex.vertex_index + 1, ti + 1, ni + 1)?
                        }
                        (Some(ti), None) => {
                            write!(writer, " {}/{}", vertex.vertex_index + 1, ti + 1)?
                        }
                        (None, Some(ni)) => {
                            write!(writer, " {}//{}", vertex.vertex_index + 1, ni + 1)?
                        }
                        (None, None) => write!(writer, " {}", vertex.vertex_index + 1)?,
                    }
                }
                writeln!(writer)?;
            }
        }

        // Write MTL file if needed
        if self.write_mtl && !self.document.materials.is_empty() {
            let mtl_name = self
                .mtl_filename
                .as_ref()
                .map(|s| s.clone())
                .unwrap_or_else(|| {
                    let stem = path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("materials");
                    format!("{}.mtl", stem)
                });
            let mtl_path = path.parent().unwrap_or(Path::new(".")).join(&mtl_name);
            self.write_mtl_file(&mtl_path)?;
        }

        Ok(())
    }

    /// Write MTL file
    fn write_mtl_file(&self, path: &Path) -> Result<(), ObjError> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "# MTL file exported by BrepRs")?;
        writeln!(writer)?;

        for (_, material) in &self.document.materials {
            writeln!(writer, "newmtl {}", material.name)?;
            writeln!(
                writer,
                "Ka {} {} {}",
                material.ka[0], material.ka[1], material.ka[2]
            )?;
            writeln!(
                writer,
                "Kd {} {} {}",
                material.kd[0], material.kd[1], material.kd[2]
            )?;
            writeln!(
                writer,
                "Ks {} {} {}",
                material.ks[0], material.ks[1], material.ks[2]
            )?;

            if material.ke != [0.0; 3] {
                writeln!(
                    writer,
                    "Ke {} {} {}",
                    material.ke[0], material.ke[1], material.ke[2]
                )?;
            }

            if material.ns != 0.0 {
                writeln!(writer, "Ns {}", material.ns)?;
            }

            if material.d != 1.0 {
                writeln!(writer, "d {}", material.d)?;
            }

            if material.tr != 0.0 {
                writeln!(writer, "Tr {}", material.tr)?;
            }

            writeln!(writer, "illum {}", material.illum)?;

            if material.ni != 1.0 {
                writeln!(writer, "Ni {}", material.ni)?;
            }

            if let Some(ref map) = material.map_ka {
                writeln!(writer, "map_Ka {}", map)?;
            }
            if let Some(ref map) = material.map_kd {
                writeln!(writer, "map_Kd {}", map)?;
            }
            if let Some(ref map) = material.map_ks {
                writeln!(writer, "map_Ks {}", map)?;
            }
            if let Some(ref map) = material.map_bump {
                writeln!(writer, "map_Bump {}", map)?;
            }

            writeln!(writer)?;
        }

        Ok(())
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.document.add_vertex(ObjVertex::new(x, y, z))
    }

    /// Add a normal
    pub fn add_normal(&mut self, x: f64, y: f64, z: f64) -> usize {
        self.document.add_normal(ObjNormal::new(x, y, z))
    }

    /// Add a texture coordinate
    pub fn add_texcoord(&mut self, u: f64, v: f64) -> usize {
        self.document.add_texcoord(ObjTexCoord::with_v(u, v))
    }

    /// Add a material
    pub fn add_material(&mut self, material: MtlMaterial) {
        self.document
            .materials
            .insert(material.name.clone(), material);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obj_reader_creation() {
        let reader = ObjReader::new("test.obj");
        assert_eq!(reader.filename, "test.obj");
    }

    #[test]
    fn test_obj_writer_creation() {
        let writer = ObjWriter::new("test.obj");
        assert!(writer.write_mtl);
    }

    #[test]
    fn test_vertex_creation() {
        let vertex = ObjVertex::new(1.0, 2.0, 3.0);
        assert_eq!(vertex.x, 1.0);
        assert_eq!(vertex.y, 2.0);
        assert_eq!(vertex.z, 3.0);
        assert_eq!(vertex.w, 1.0);
    }

    #[test]
    fn test_vertex_with_w() {
        let vertex = ObjVertex::with_w(1.0, 2.0, 3.0, 0.5);
        assert_eq!(vertex.w, 0.5);
    }

    #[test]
    fn test_normal_normalize() {
        let normal = ObjNormal::new(0.0, 0.0, 2.0);
        let normalized = normal.normalize();
        assert!((normalized.z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_face_vertex_creation() {
        let fv = ObjFaceVertex::new(0);
        assert_eq!(fv.vertex_index, 0);
        assert!(fv.texcoord_index.is_none());
        assert!(fv.normal_index.is_none());
    }

    #[test]
    fn test_face_vertex_with_texcoord() {
        let fv = ObjFaceVertex::with_texcoord(0, 1);
        assert_eq!(fv.vertex_index, 0);
        assert_eq!(fv.texcoord_index, Some(1));
    }

    #[test]
    fn test_face_vertex_with_all() {
        let fv = ObjFaceVertex::with_texcoord_and_normal(0, 1, 2);
        assert_eq!(fv.vertex_index, 0);
        assert_eq!(fv.texcoord_index, Some(1));
        assert_eq!(fv.normal_index, Some(2));
    }

    #[test]
    fn test_face_triangulate() {
        let mut face = ObjFace::new();
        face.add_vertex(ObjFaceVertex::new(0));
        face.add_vertex(ObjFaceVertex::new(1));
        face.add_vertex(ObjFaceVertex::new(2));
        face.add_vertex(ObjFaceVertex::new(3));

        let triangles = face.triangulate();
        assert_eq!(triangles.len(), 2);
        assert!(triangles[0].is_triangle());
        assert!(triangles[1].is_triangle());
    }

    #[test]
    fn test_face_is_triangle() {
        let mut face = ObjFace::new();
        face.add_vertex(ObjFaceVertex::new(0));
        face.add_vertex(ObjFaceVertex::new(1));
        face.add_vertex(ObjFaceVertex::new(2));
        assert!(face.is_triangle());
    }

    #[test]
    fn test_face_is_quad() {
        let mut face = ObjFace::new();
        face.add_vertex(ObjFaceVertex::new(0));
        face.add_vertex(ObjFaceVertex::new(1));
        face.add_vertex(ObjFaceVertex::new(2));
        face.add_vertex(ObjFaceVertex::new(3));
        assert!(face.is_quad());
    }

    #[test]
    fn test_mtl_material_creation() {
        let mat = MtlMaterial::new("test_material");
        assert_eq!(mat.name, "test_material");
        assert_eq!(mat.d, 1.0);
        assert_eq!(mat.illum, 0);
    }

    #[test]
    fn test_obj_document() {
        let mut doc = ObjDocument::new();
        let idx = doc.add_vertex(ObjVertex::new(0.0, 0.0, 0.0));
        assert_eq!(idx, 0);
        assert_eq!(doc.vertex_count(), 1);
    }

    #[test]
    fn test_writer_add_vertex() {
        let mut writer = ObjWriter::new("test.obj");
        let idx = writer.add_vertex(1.0, 2.0, 3.0);
        assert_eq!(idx, 0);
        assert_eq!(writer.document.vertex_count(), 1);
    }
}
