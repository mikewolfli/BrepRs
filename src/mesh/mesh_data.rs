//! Mesh data structures
//!
//! This module provides basic mesh data structures for 2D and 3D meshes.

use crate::geometry::Point;
use std::collections::HashMap;

/// Mesh vertex
#[derive(Debug, Clone, PartialEq)]
pub struct MeshVertex {
    /// Vertex ID
    pub id: usize,
    /// Vertex coordinates
    pub point: Point,
    /// Optional normal vector
    pub normal: Option<[f64; 3]>,
    /// Optional texture coordinates
    pub uv: Option<[f64; 2]>,
}

impl MeshVertex {
    /// Create a new mesh vertex
    pub fn new(id: usize, point: Point) -> Self {
        Self {
            id,
            point,
            normal: None,
            uv: None,
        }
    }

    /// Set normal vector
    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = Some(normal);
    }

    /// Set texture coordinates
    pub fn set_uv(&mut self, uv: [f64; 2]) {
        self.uv = Some(uv);
    }
}

/// Mesh edge
#[derive(Debug, Clone, PartialEq)]
pub struct MeshEdge {
    /// Edge ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 2],
    /// Optional edge data
    pub data: HashMap<String, f64>,
}

impl MeshEdge {
    /// Create a new mesh edge
    pub fn new(id: usize, v1: usize, v2: usize) -> Self {
        Self {
            id,
            vertices: [v1, v2],
            data: HashMap::new(),
        }
    }

    /// Add edge data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

/// Mesh face
#[derive(Debug, Clone, PartialEq)]
pub struct MeshFace {
    /// Face ID
    pub id: usize,
    /// Vertex indices
    pub vertices: Vec<usize>,
    /// Edge indices
    pub edges: Vec<usize>,
    /// Optional face normal
    pub normal: Option<[f64; 3]>,
    /// Optional material ID
    pub material_id: Option<usize>,
    /// Optional face data
    pub data: HashMap<String, f64>,
}

impl MeshFace {
    /// Create a new mesh face
    pub fn new(id: usize, vertices: Vec<usize>) -> Self {
        Self {
            id,
            vertices,
            edges: Vec::new(),
            normal: None,
            material_id: None,
            data: HashMap::new(),
        }
    }

    /// Set face normal
    pub fn set_normal(&mut self, normal: [f64; 3]) {
        self.normal = Some(normal);
    }

    /// Set material ID
    pub fn set_material_id(&mut self, material_id: usize) {
        self.material_id = Some(material_id);
    }

    /// Add face data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

/// 2D mesh
#[derive(Debug, Clone)]
pub struct Mesh2D {
    /// Vertices
    pub vertices: Vec<MeshVertex>,
    /// Edges
    pub edges: Vec<MeshEdge>,
    /// Faces (triangles)
    pub faces: Vec<MeshFace>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
}

impl Mesh2D {
    /// Create a new 2D mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            bbox: (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0)),
            quality: HashMap::new(),
        }
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, point: Point) -> usize {
        let id = self.vertices.len();
        self.vertices.push(MeshVertex::new(id, point));
        self.update_bbox();
        id
    }

    /// Add an edge
    pub fn add_edge(&mut self, v1: usize, v2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(MeshEdge::new(id, v1, v2));
        id
    }

    /// Add a face (triangle)
    pub fn add_face(&mut self, v1: usize, v2: usize, v3: usize) -> usize {
        let id = self.faces.len();
        self.faces.push(MeshFace::new(id, vec![v1, v2, v3]));
        id
    }

    /// Update bounding box
    fn update_bbox(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min_point = self.vertices[0].point.clone();
        let mut max_point = self.vertices[0].point.clone();

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.point.x);
            min_point.y = min_point.y.min(vertex.point.y);
            max_point.x = max_point.x.max(vertex.point.x);
            max_point.y = max_point.y.max(vertex.point.y);
        }

        self.bbox = (min_point, max_point);
    }

    /// Calculate face normal
    pub fn calculate_face_normal(&mut self, face_id: usize) {
        if face_id >= self.faces.len() {
            return;
        }

        let face = &self.faces[face_id];
        if face.vertices.len() < 3 {
            return;
        }

        let v0 = &self.vertices[face.vertices[0]];
        let v1 = &self.vertices[face.vertices[1]];
        let v2 = &self.vertices[face.vertices[2]];

        let vec1 = [v1.point.x - v0.point.x, v1.point.y - v0.point.y, 0.0];
        let vec2 = [v2.point.x - v0.point.x, v2.point.y - v0.point.y, 0.0];

        let normal = [0.0, 0.0, vec1[0] * vec2[1] - vec1[1] * vec2[0]];

        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > 1e-6 {
            let normalized_normal = [normal[0] / length, normal[1] / length, normal[2] / length];
            self.faces[face_id].set_normal(normalized_normal);
        }
    }

    /// Calculate all face normals
    pub fn calculate_normals(&mut self) {
        for i in 0..self.faces.len() {
            self.calculate_face_normal(i);
        }
    }
}

/// 3D mesh
#[derive(Debug, Clone)]
pub struct Mesh3D {
    /// Vertices
    pub vertices: Vec<MeshVertex>,
    /// Edges
    pub edges: Vec<MeshEdge>,
    /// Faces
    pub faces: Vec<MeshFace>,
    /// Tetrahedrons
    pub tetrahedrons: Vec<MeshTetrahedron>,
    /// Bounding box
    pub bbox: (Point, Point),
    /// Mesh quality metrics
    pub quality: HashMap<String, f64>,
}

impl Mesh3D {
    /// Create a new 3D mesh
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            tetrahedrons: Vec::new(),
            bbox: (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0)),
            quality: HashMap::new(),
        }
    }

    /// Add a vertex
    pub fn add_vertex(&mut self, point: Point) -> usize {
        let id = self.vertices.len();
        self.vertices.push(MeshVertex::new(id, point));
        self.update_bbox();
        id
    }

    /// Add an edge
    pub fn add_edge(&mut self, v1: usize, v2: usize) -> usize {
        let id = self.edges.len();
        self.edges.push(MeshEdge::new(id, v1, v2));
        id
    }

    /// Add a face
    pub fn add_face(&mut self, vertices: Vec<usize>) -> usize {
        let id = self.faces.len();
        self.faces.push(MeshFace::new(id, vertices));
        id
    }

    /// Add a tetrahedron
    pub fn add_tetrahedron(&mut self, v1: usize, v2: usize, v3: usize, v4: usize) -> usize {
        let id = self.tetrahedrons.len();
        self.tetrahedrons
            .push(MeshTetrahedron::new(id, v1, v2, v3, v4));
        id
    }

    /// Update bounding box
    fn update_bbox(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        let mut min_point = self.vertices[0].point.clone();
        let mut max_point = self.vertices[0].point.clone();

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.point.x);
            min_point.y = min_point.y.min(vertex.point.y);
            min_point.z = min_point.z.min(vertex.point.z);
            max_point.x = max_point.x.max(vertex.point.x);
            max_point.y = max_point.y.max(vertex.point.y);
            max_point.z = max_point.z.max(vertex.point.z);
        }

        self.bbox = (min_point, max_point);
    }
}

/// Mesh tetrahedron
#[derive(Debug, Clone, PartialEq)]
pub struct MeshTetrahedron {
    /// Tetrahedron ID
    pub id: usize,
    /// Vertex indices
    pub vertices: [usize; 4],
    /// Face indices
    pub faces: [usize; 4],
    /// Optional tetrahedron data
    pub data: HashMap<String, f64>,
}

impl MeshTetrahedron {
    /// Create a new mesh tetrahedron
    pub fn new(id: usize, v1: usize, v2: usize, v3: usize, v4: usize) -> Self {
        Self {
            id,
            vertices: [v1, v2, v3, v4],
            faces: [0, 0, 0, 0], // Will be filled later
            data: HashMap::new(),
        }
    }

    /// Add tetrahedron data
    pub fn add_data(&mut self, key: &str, value: f64) {
        self.data.insert(key.to_string(), value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_vertex_creation() {
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = MeshVertex::new(0, point);
        assert_eq!(vertex.id, 0);
        assert_eq!(vertex.point.x, 1.0);
        assert_eq!(vertex.point.y, 2.0);
        assert_eq!(vertex.point.z, 3.0);
        assert!(vertex.normal.is_none());
        assert!(vertex.uv.is_none());
    }

    #[test]
    fn test_mesh_edge_creation() {
        let edge = MeshEdge::new(0, 0, 1);
        assert_eq!(edge.id, 0);
        assert_eq!(edge.vertices[0], 0);
        assert_eq!(edge.vertices[1], 1);
        assert!(edge.data.is_empty());
    }

    #[test]
    fn test_mesh_face_creation() {
        let face = MeshFace::new(0, vec![0, 1, 2]);
        assert_eq!(face.id, 0);
        assert_eq!(face.vertices, vec![0, 1, 2]);
        assert!(face.edges.is_empty());
        assert!(face.normal.is_none());
        assert!(face.material_id.is_none());
        assert!(face.data.is_empty());
    }

    #[test]
    fn test_mesh2d_creation() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.faces.len(), 1);
    }

    #[test]
    fn test_mesh3d_creation() {
        let mut mesh = Mesh3D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0));
        mesh.add_tetrahedron(v0, v1, v2, v3);

        assert_eq!(mesh.vertices.len(), 4);
        assert_eq!(mesh.tetrahedrons.len(), 1);
    }

    #[test]
    fn test_mesh_tetrahedron_creation() {
        let tetra = MeshTetrahedron::new(0, 0, 1, 2, 3);
        assert_eq!(tetra.id, 0);
        assert_eq!(tetra.vertices, [0, 1, 2, 3]);
        assert!(tetra.data.is_empty());
    }

    #[test]
    fn test_calculate_face_normal() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let face_id = mesh.add_face(v0, v1, v2);

        mesh.calculate_face_normal(face_id);
        let normal = mesh.faces[face_id].normal.unwrap();
        assert!((normal[0] - 0.0).abs() < 1e-6);
        assert!((normal[1] - 0.0).abs() < 1e-6);
        assert!((normal[2] - 1.0).abs() < 1e-6);
    }
}
