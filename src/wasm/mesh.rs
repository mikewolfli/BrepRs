//! WebAssembly bindings for mesh generation

use wasm_bindgen::prelude::*;

/// Mesh types
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeshType {
    /// Triangular mesh
    Triangle,
    /// Quadrilateral mesh
    Quad,
    /// Mixed mesh
    Mixed,
}

/// Mesh quality metrics
#[wasm_bindgen]
pub struct MeshQuality {
    aspect_ratio: f64,
    min_angle: f64,
    max_angle: f64,
    area: f64,
}

#[wasm_bindgen]
impl MeshQuality {
    /// Create new mesh quality metrics
    #[wasm_bindgen(constructor)]
    pub fn new(aspect_ratio: f64, min_angle: f64, max_angle: f64, area: f64) -> Self {
        Self {
            aspect_ratio,
            min_angle,
            max_angle,
            area,
        }
    }

    /// Get aspect ratio
    #[wasm_bindgen(getter, js_name = aspectRatio)]
    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    /// Get minimum angle
    #[wasm_bindgen(getter, js_name = minAngle)]
    pub fn min_angle(&self) -> f64 {
        self.min_angle
    }

    /// Get maximum angle
    #[wasm_bindgen(getter, js_name = maxAngle)]
    pub fn max_angle(&self) -> f64 {
        self.max_angle
    }

    /// Get area
    #[wasm_bindgen(getter, js_name = area)]
    pub fn area(&self) -> f64 {
        self.area
    }

    /// Check if quality is acceptable
    #[wasm_bindgen(js_name = isAcceptable)]
    pub fn is_acceptable(&self, max_aspect_ratio: f64, min_angle_deg: f64, max_angle_deg: f64) -> bool {
        self.aspect_ratio <= max_aspect_ratio
            && self.min_angle >= min_angle_deg
            && self.max_angle <= max_angle_deg
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "MeshQuality(aspectRatio={}, minAngle={}, maxAngle={}, area={})",
            self.aspect_ratio, self.min_angle, self.max_angle, self.area
        )
    }
}

/// Mesh data structure
#[wasm_bindgen]
pub struct MeshData {
    vertices: Vec<f64>,
    indices: Vec<u32>,
    normals: Vec<f64>,
    uvs: Vec<f64>,
    mesh_type: MeshType,
}

#[wasm_bindgen]
impl MeshData {
    /// Create new mesh data
    #[wasm_bindgen(constructor)]
    pub fn new(mesh_type: MeshType) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            mesh_type,
        }
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

    /// Get UVs
    #[wasm_bindgen(getter, js_name = uvs)]
    pub fn uvs(&self) -> Vec<f64> {
        self.uvs.clone()
    }

    /// Get mesh type
    #[wasm_bindgen(getter, js_name = meshType)]
    pub fn mesh_type(&self) -> MeshType {
        self.mesh_type
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

    /// Add UV coordinate
    #[wasm_bindgen(js_name = addUV)]
    pub fn add_uv(&mut self, u: f64, v: f64) {
        self.uvs.push(u);
        self.uvs.push(v);
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

    /// Get triangle count
    #[wasm_bindgen(js_name = triangleCount)]
    pub fn triangle_count(&self) -> usize {
        match self.mesh_type {
            MeshType::Triangle => self.indices.len() / 3,
            MeshType::Quad => self.indices.len() / 4 * 2,
            MeshType::Mixed => self.indices.len() / 3,
        }
    }

    /// Clear all data
    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.normals.clear();
        self.uvs.clear();
    }

    /// Calculate bounding box
    #[wasm_bindgen(js_name = calculateBoundingBox)]
    pub fn calculate_bounding_box(&self) -> BoundingBox {
        if self.vertices.is_empty() {
            return BoundingBox::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let mut min_x = self.vertices[0];
        let mut min_y = self.vertices[1];
        let mut min_z = self.vertices[2];
        let mut max_x = self.vertices[0];
        let mut max_y = self.vertices[1];
        let mut max_z = self.vertices[2];

        for chunk in self.vertices.chunks(3) {
            min_x = min_x.min(chunk[0]);
            min_y = min_y.min(chunk[1]);
            min_z = min_z.min(chunk[2]);
            max_x = max_x.max(chunk[0]);
            max_y = max_y.max(chunk[1]);
            max_z = max_z.max(chunk[2]);
        }

        BoundingBox::new(min_x, min_y, min_z, max_x, max_y, max_z)
    }

    /// Calculate mesh quality
    #[wasm_bindgen(js_name = calculateQuality)]
    pub fn calculate_quality(&self) -> MeshQuality {
        let mut total_aspect_ratio = 0.0;
        let mut total_min_angle = 0.0;
        let mut total_max_angle = 0.0;
        let mut total_area = 0.0;
        let count = self.triangle_count();

        if count == 0 {
            return MeshQuality::new(0.0, 0.0, 0.0, 0.0);
        }

        for i in 0..count {
            let (aspect_ratio, min_angle, max_angle, area) = self.calculate_triangle_quality(i);
            total_aspect_ratio += aspect_ratio;
            total_min_angle += min_angle;
            total_max_angle += max_angle;
            total_area += area;
        }

        MeshQuality::new(
            total_aspect_ratio / count as f64,
            total_min_angle / count as f64,
            total_max_angle / count as f64,
            total_area,
        )
    }

    /// Calculate quality for a single triangle
    fn calculate_triangle_quality(&self, index: usize) -> (f64, f64, f64, f64) {
        let i0 = self.indices[index * 3] as usize;
        let i1 = self.indices[index * 3 + 1] as usize;
        let i2 = self.indices[index * 3 + 2] as usize;

        let v0 = [
            self.vertices[i0 * 3],
            self.vertices[i0 * 3 + 1],
            self.vertices[i0 * 3 + 2],
        ];
        let v1 = [
            self.vertices[i1 * 3],
            self.vertices[i1 * 3 + 1],
            self.vertices[i1 * 3 + 2],
        ];
        let v2 = [
            self.vertices[i2 * 3],
            self.vertices[i2 * 3 + 1],
            self.vertices[i2 * 3 + 2],
        ];

        let edge0 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
        let edge1 = [v2[0] - v1[0], v2[1] - v1[1], v2[2] - v1[2]];
        let edge2 = [v0[0] - v2[0], v0[1] - v2[1], v0[2] - v2[2]];

        let len0 = (edge0[0].powi(2) + edge0[1].powi(2) + edge0[2].powi(2)).sqrt();
        let len1 = (edge1[0].powi(2) + edge1[1].powi(2) + edge1[2].powi(2)).sqrt();
        let len2 = (edge2[0].powi(2) + edge2[1].powi(2) + edge2[2].powi(2)).sqrt();

        let max_len = len0.max(len1).max(len2);
        let min_len = len0.min(len1).min(len2);
        let aspect_ratio = if min_len > 0.0 { max_len / min_len } else { 0.0 };

        let cross = [
            edge0[1] * edge1[2] - edge0[2] * edge1[1],
            edge0[2] * edge1[0] - edge0[0] * edge1[2],
            edge0[0] * edge1[1] - edge0[1] * edge1[0],
        ];
        let area = 0.5 * (cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt();

        let dot01 = edge0[0] * edge1[0] + edge0[1] * edge1[1] + edge0[2] * edge1[2];
        let dot12 = edge1[0] * edge2[0] + edge1[1] * edge2[1] + edge1[2] * edge2[2];
        let dot20 = edge2[0] * edge0[0] + edge2[1] * edge0[1] + edge2[2] * edge0[2];

        let angle0 = (dot01 / (len0 * len1)).acos().to_degrees();
        let angle1 = (dot12 / (len1 * len2)).acos().to_degrees();
        let angle2 = (dot20 / (len2 * len0)).acos().to_degrees();

        let min_angle = angle0.min(angle1).min(angle2);
        let max_angle = angle0.max(angle1).max(angle2);

        (aspect_ratio, min_angle, max_angle, area)
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "MeshData(vertices={}, indices={}, normals={}, uvs={}, type={:?})",
            self.vertex_count(),
            self.index_count(),
            self.normals.len() / 3,
            self.uvs.len() / 2,
            self.mesh_type
        )
    }
}

/// Bounding box structure
#[wasm_bindgen]
pub struct BoundingBox {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

#[wasm_bindgen]
impl BoundingBox {
    /// Create new bounding box
    #[wasm_bindgen(constructor)]
    pub fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }

    /// Get minimum point
    #[wasm_bindgen(getter, js_name = min)]
    pub fn min(&self) -> Vec<f64> {
        vec![self.min_x, self.min_y, self.min_z]
    }

    /// Get maximum point
    #[wasm_bindgen(getter, js_name = max)]
    pub fn max(&self) -> Vec<f64> {
        vec![self.max_x, self.max_y, self.max_z]
    }

    /// Get center point
    #[wasm_bindgen(js_name = center)]
    pub fn center(&self) -> Vec<f64> {
        vec![
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
            (self.min_z + self.max_z) / 2.0,
        ]
    }

    /// Get size
    #[wasm_bindgen(js_name = size)]
    pub fn size(&self) -> Vec<f64> {
        vec![
            self.max_x - self.min_x,
            self.max_y - self.min_y,
            self.max_z - self.min_z,
        ]
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y) * (self.max_z - self.min_z)
    }

    /// Check if point is inside bounding box
    #[wasm_bindgen(js_name = containsPoint)]
    pub fn contains_point(&self, x: f64, y: f64, z: f64) -> bool {
        x >= self.min_x && x <= self.max_x
            && y >= self.min_y && y <= self.max_y
            && z >= self.min_z && z <= self.max_z
    }

    /// Expand bounding box
    #[wasm_bindgen(js_name = expand)]
    pub fn expand(&mut self, delta: f64) {
        self.min_x -= delta;
        self.min_y -= delta;
        self.min_z -= delta;
        self.max_x += delta;
        self.max_y += delta;
        self.max_z += delta;
    }

    /// Merge with another bounding box
    #[wasm_bindgen(js_name = merge)]
    pub fn merge(&self, other: &BoundingBox) -> BoundingBox {
        BoundingBox::new(
            self.min_x.min(other.min_x),
            self.min_y.min(other.min_y),
            self.min_z.min(other.min_z),
            self.max_x.max(other.max_x),
            self.max_y.max(other.max_y),
            self.max_z.max(other.max_z),
        )
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "BoundingBox(min=({}, {}, {}), max=({}, {}, {}))",
            self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z
        )
    }
}

/// Mesh generator
#[wasm_bindgen(js_name = MeshGenerator)]
pub struct MeshGenerator {
    mesh_type: MeshType,
    tolerance: f64,
}

#[wasm_bindgen(js_class = MeshGenerator)]
impl MeshGenerator {
    /// Create new mesh generator
    #[wasm_bindgen(constructor)]
    pub fn new(mesh_type: MeshType) -> Self {
        Self {
            mesh_type,
            tolerance: 1e-6,
        }
    }

    /// Create with tolerance
    #[wasm_bindgen(js_name = withTolerance)]
    pub fn with_tolerance(mesh_type: MeshType, tolerance: f64) -> Self {
        Self {
            mesh_type,
            tolerance,
        }
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get tolerance
    #[wasm_bindgen(getter, js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Generate simple box mesh
    #[wasm_bindgen(js_name = generateBoxMesh)]
    pub fn generate_box_mesh(&self, width: f64, height: f64, depth: f64) -> MeshData {
        let mut mesh = MeshData::new(self.mesh_type);

        let hw = width / 2.0;
        let hh = height / 2.0;
        let hd = depth / 2.0;

        let vertices = [
            [-hw, -hh, -hd],
            [hw, -hh, -hd],
            [hw, hh, -hd],
            [-hw, hh, -hd],
            [-hw, -hh, hd],
            [hw, -hh, hd],
            [hw, hh, hd],
            [-hw, hh, hd],
        ];

        for v in &vertices {
            mesh.add_vertex(v[0], v[1], v[2]);
        }

        let indices = [
            0, 1, 2, 0, 2, 3,
            4, 5, 6, 4, 6, 7,
            0, 1, 5, 0, 5, 4,
            2, 3, 7, 2, 7, 6,
            0, 3, 7, 0, 7, 4,
            1, 2, 6, 1, 6, 5,
        ];

        for i in &indices {
            mesh.add_index(*i as u32);
        }

        mesh
    }

    /// Generate simple sphere mesh
    #[wasm_bindgen(js_name = generateSphereMesh)]
    pub fn generate_sphere_mesh(&self, radius: f64, segments: u32) -> MeshData {
        let mut mesh = MeshData::new(self.mesh_type);

        let rings = segments;
        let sectors = segments;

        for r in 0..=rings {
            let theta = (r as f64) * std::f64::consts::PI / (rings as f64);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for s in 0..=sectors {
                let phi = (s as f64) * 2.0 * std::f64::consts::PI / (sectors as f64);
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = radius * sin_theta * cos_phi;
                let y = radius * cos_theta;
                let z = radius * sin_theta * sin_phi;

                mesh.add_vertex(x, y, z);
            }
        }

        for r in 0..rings {
            for s in 0..sectors {
                let first = r * (sectors + 1) + s;
                let second = first + sectors + 1;

                mesh.add_index(first as u32);
                mesh.add_index(second as u32);
                mesh.add_index((first + 1) as u32);

                mesh.add_index(second as u32);
                mesh.add_index((second + 1) as u32);
                mesh.add_index((first + 1) as u32);
            }
        }

        mesh
    }

    /// Generate simple cylinder mesh
    #[wasm_bindgen(js_name = generateCylinderMesh)]
    pub fn generate_cylinder_mesh(&self, radius: f64, height: f64, segments: u32) -> MeshData {
        let mut mesh = MeshData::new(self.mesh_type);

        let half_height = height / 2.0;

        for i in 0..=segments {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / (segments as f64);
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            mesh.add_vertex(x, -half_height, z);
            mesh.add_vertex(x, half_height, z);
        }

        for i in 0..segments {
            let next = (i + 1) % (segments + 1);
            let i0 = i * 2;
            let i1 = i0 + 1;
            let i2 = next * 2;
            let i3 = i2 + 1;

            mesh.add_index(i0 as u32);
            mesh.add_index(i1 as u32);
            mesh.add_index(i2 as u32);

            mesh.add_index(i1 as u32);
            mesh.add_index(i3 as u32);
            mesh.add_index(i2 as u32);
        }

        mesh
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "MeshGenerator(type={:?}, tolerance={})",
            self.mesh_type, self.tolerance
        )
    }
}

impl Default for MeshGenerator {
    fn default() -> Self {
        Self::new(MeshType::Triangle)
    }
}
