//! Quad-dominant mesh generation
//!
//! This module provides functionality for generating quad-dominant meshes
//! from triangular meshes or directly from geometry.

use super::mesh_data::{Mesh2D, MeshFace};
use crate::geometry::Point;
use std::collections::{HashMap, HashSet};

/// Quad-dominant mesher error types
#[derive(Debug)]
pub enum QuadMesherError {
    /// Invalid input mesh
    InvalidInputMesh,
    /// Meshing failed
    MeshingFailed,
    /// No triangles to process
    NoTriangles,
}

/// Quad-dominant mesher parameters
#[derive(Debug, Clone)]
pub struct QuadMesherParams {
    /// Target quad size
    pub target_quad_size: f64,
    /// Minimum quad angle (in degrees)
    pub min_quad_angle: f64,
    /// Maximum quad aspect ratio
    pub max_aspect_ratio: f64,
    /// Preserve boundaries
    pub preserve_boundaries: bool,
    /// Use triangle to quad conversion
    pub convert_triangles: bool,
}

impl Default for QuadMesherParams {
    fn default() -> Self {
        Self {
            target_quad_size: 1.0,
            min_quad_angle: 20.0,
            max_aspect_ratio: 5.0,
            preserve_boundaries: true,
            convert_triangles: true,
        }
    }
}

/// Quad-dominant mesher
pub struct QuadMesher {
    /// Mesher parameters
    params: QuadMesherParams,
    /// Input triangular mesh
    input_mesh: Option<Mesh2D>,
    /// Quad mesh
    quad_mesh: Mesh2D,
    /// Edge to face mapping
    edge_to_faces: HashMap<(usize, usize), Vec<usize>>,
    /// Face adjacency
    face_adjacency: Vec<Vec<usize>>,
}

/// Quad candidate
struct QuadCandidate {
    face1: usize,
    face2: usize,
    vertices: [usize; 4],
    quality: f64,
}

impl QuadMesher {
    /// Create a new quad-dominant mesher
    pub fn new(params: QuadMesherParams) -> Self {
        Self {
            params,
            input_mesh: None,
            quad_mesh: Mesh2D::new(),
            edge_to_faces: HashMap::new(),
            face_adjacency: Vec::new(),
        }
    }

    /// Set input mesh
    pub fn set_input_mesh(&mut self, mesh: Mesh2D) {
        self.input_mesh = Some(mesh);
    }

    /// Generate quad-dominant mesh
    pub fn generate(&mut self) -> Result<Mesh2D, QuadMesherError> {
        // Take ownership of input mesh temporarily
        let input_mesh = match self.input_mesh.take() {
            Some(mesh) => mesh,
            None => return Err(QuadMesherError::InvalidInputMesh),
        };

        if input_mesh.faces.is_empty() {
            self.input_mesh = Some(input_mesh);
            return Err(QuadMesherError::NoTriangles);
        }

        // Initialize data structures
        self.initialize_data_structures(&input_mesh);

        // Build quad mesh
        self.build_quad_mesh(&input_mesh);

        // Optimize quad mesh
        self.optimize_quad_mesh();

        // Restore input mesh
        self.input_mesh = Some(input_mesh);

        Ok(self.quad_mesh.clone())
    }

    /// Initialize data structures
    fn initialize_data_structures(&mut self, mesh: &Mesh2D) {
        // Clear existing data
        self.edge_to_faces.clear();
        self.face_adjacency.clear();
        self.quad_mesh = Mesh2D::new();

        // Build edge to face mapping
        for (face_id, face) in mesh.faces.iter().enumerate() {
            for i in 0..face.vertices.len() {
                let v0 = face.vertices[i];
                let v1 = face.vertices[(i + 1) % face.vertices.len()];
                let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                self.edge_to_faces.entry(edge).or_default().push(face_id);
            }
        }

        // Build face adjacency
        self.face_adjacency.resize(mesh.faces.len(), Vec::new());
        for (edge, faces) in &self.edge_to_faces {
            if faces.len() == 2 {
                self.face_adjacency[faces[0]].push(faces[1]);
                self.face_adjacency[faces[1]].push(faces[0]);
            }
        }
    }

    /// Build quad mesh from triangular mesh
    fn build_quad_mesh(&mut self, input_mesh: &Mesh2D) {
        // Copy vertices
        for vertex in &input_mesh.vertices {
            self.quad_mesh.add_vertex(vertex.point.clone());
        }

        // Identify quad candidates
        let quad_candidates = self.identify_quad_candidates(input_mesh);

        // Create quads from candidates
        let mut processed_faces = HashSet::new();
        for candidate in quad_candidates {
            if !processed_faces.contains(&candidate.face1)
                && !processed_faces.contains(&candidate.face2)
            {
                self.create_quad(input_mesh, candidate, &mut processed_faces);
            }
        }

        // Handle remaining triangles
        if self.params.convert_triangles {
            self.handle_remaining_triangles(input_mesh, &processed_faces);
        }
    }

    /// Identify quad candidates
    fn identify_quad_candidates(&self, mesh: &Mesh2D) -> Vec<QuadCandidate> {
        let mut candidates = Vec::new();

        for (face_id, adj_faces) in self.face_adjacency.iter().enumerate() {
            for &adj_face_id in adj_faces {
                if face_id < adj_face_id {
                    // Avoid duplicate pairs
                    if let Some(quad) = self.try_form_quad(mesh, face_id, adj_face_id) {
                        candidates.push(quad);
                    }
                }
            }
        }

        // Sort candidates by quality
        candidates.sort_by(|a, b| b.quality.partial_cmp(&a.quality).unwrap());
        candidates
    }

    /// Try to form a quad from two adjacent triangles
    fn try_form_quad(
        &self,
        mesh: &Mesh2D,
        face1_id: usize,
        face2_id: usize,
    ) -> Option<QuadCandidate> {
        let face1 = &mesh.faces[face1_id];
        let face2 = &mesh.faces[face2_id];

        // Find common edge
        let mut common_vertices = Vec::new();
        for v1 in &face1.vertices {
            for v2 in &face2.vertices {
                if v1 == v2 {
                    common_vertices.push(*v1);
                }
            }
        }

        if common_vertices.len() != 2 {
            return None;
        }

        // Get quad vertices
        let mut quad_vertices = Vec::new();
        for v in &face1.vertices {
            if !common_vertices.contains(v) {
                quad_vertices.push(*v);
            }
        }
        for v in &face2.vertices {
            if !common_vertices.contains(v) {
                quad_vertices.push(*v);
            }
        }
        quad_vertices.extend(&common_vertices);

        if quad_vertices.len() != 4 {
            return None;
        }

        // Calculate quad quality
        let quality = self.calculate_quad_quality(mesh, &quad_vertices);

        Some(QuadCandidate {
            face1: face1_id,
            face2: face2_id,
            vertices: [
                quad_vertices[0],
                quad_vertices[1],
                quad_vertices[2],
                quad_vertices[3],
            ],
            quality,
        })
    }

    /// Calculate quad quality
    fn calculate_quad_quality(&self, mesh: &Mesh2D, vertices: &[usize]) -> f64 {
        Self::calculate_quad_quality_static(mesh, vertices)
    }

    /// Calculate quad quality (static version)
    fn calculate_quad_quality_static(mesh: &Mesh2D, vertices: &[usize]) -> f64 {
        if vertices.len() != 4 {
            return 0.0;
        }

        let p0 = &mesh.vertices[vertices[0]].point;
        let p1 = &mesh.vertices[vertices[1]].point;
        let p2 = &mesh.vertices[vertices[2]].point;
        let p3 = &mesh.vertices[vertices[3]].point;

        // Calculate edge lengths
        let edges = vec![
            ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2)).sqrt(),
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2)).sqrt(),
            ((p0.x - p3.x).powi(2) + (p0.y - p3.y).powi(2)).sqrt(),
        ];

        // Calculate aspect ratio
        let max_edge = edges.iter().fold(0.0_f64, |max, &e| max.max(e));
        let min_edge = edges.iter().fold(f64::MAX, |min, &e| min.min(e));
        let aspect_ratio = if min_edge > 0.0 {
            max_edge / min_edge
        } else {
            10.0
        };

        // Calculate angles
        let angles = vec![
            Self::calculate_angle_static(p3, p0, p1),
            Self::calculate_angle_static(p0, p1, p2),
            Self::calculate_angle_static(p1, p2, p3),
            Self::calculate_angle_static(p2, p3, p0),
        ];

        let min_angle = angles.iter().fold(180.0_f64, |min, &a| min.min(a));
        let max_angle = angles.iter().fold(0.0_f64, |max, &a| max.max(a));

        // Calculate quality score
        let aspect_score = 1.0 / aspect_ratio;
        let angle_score = if min_angle > 0.0 && max_angle < 180.0 {
            min_angle / 90.0 * (180.0 - max_angle) / 90.0
        } else {
            0.0
        };

        0.5 * aspect_score + 0.5 * angle_score
    }

    /// Calculate angle between three points
    fn calculate_angle(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        Self::calculate_angle_static(p1, p2, p3)
    }

    /// Calculate angle between three points (static version)
    fn calculate_angle_static(p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let v1x = p1.x - p2.x;
        let v1y = p1.y - p2.y;
        let v2x = p3.x - p2.x;
        let v2y = p3.y - p2.y;

        let dot = v1x * v2x + v1y * v2y;
        let mag1 = (v1x * v1x + v1y * v1y).sqrt();
        let mag2 = (v2x * v2x + v2y * v2y).sqrt();

        if mag1 < 1e-6 || mag2 < 1e-6 {
            return 0.0;
        }

        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        cos_angle.acos() * 180.0 / std::f64::consts::PI
    }

    /// Create a quad from two triangles
    fn create_quad(
        &mut self,
        mesh: &Mesh2D,
        candidate: QuadCandidate,
        processed_faces: &mut HashSet<usize>,
    ) {
        let [v0, v1, v2, v3] = candidate.vertices;

        // Add quad face
        let mut quad_face = MeshFace::new(self.quad_mesh.faces.len(), vec![v0, v1, v2, v3]);

        // Calculate normal
        let p0 = &mesh.vertices[v0].point;
        let p1 = &mesh.vertices[v1].point;
        let p2 = &mesh.vertices[v2].point;
        let p3 = &mesh.vertices[v3].point;

        let v1 = [p1.x - p0.x, p1.y - p0.y, 0.0];
        let v2 = [p2.x - p0.x, p2.y - p0.y, 0.0];
        let normal = [0.0, 0.0, v1[0] * v2[1] - v1[1] * v2[0]];
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();

        if length > 1e-6 {
            let normalized_normal = [normal[0] / length, normal[1] / length, normal[2] / length];
            quad_face.set_normal(normalized_normal);
        }

        self.quad_mesh.faces.push(quad_face);
        processed_faces.insert(candidate.face1);
        processed_faces.insert(candidate.face2);
    }

    /// Handle remaining triangles
    fn handle_remaining_triangles(&mut self, mesh: &Mesh2D, processed_faces: &HashSet<usize>) {
        for (face_id, face) in mesh.faces.iter().enumerate() {
            if !processed_faces.contains(&face_id) {
                // Add as triangle
                let mut tri_face = MeshFace::new(self.quad_mesh.faces.len(), face.vertices.clone());
                if let Some(normal) = face.normal {
                    tri_face.set_normal(normal);
                }
                self.quad_mesh.faces.push(tri_face);
            }
        }
    }

    /// Optimize quad mesh
    fn optimize_quad_mesh(&mut self) {
        // Edge swapping to improve quad quality
        let mut improved = true;
        while improved {
            improved = false;
            let quad_mesh = &mut self.quad_mesh;
            for i in 0..quad_mesh.faces.len() {
                if Self::optimize_quad_static(quad_mesh, i) {
                    improved = true;
                }
            }
        }
    }

    /// Optimize a single quad
    fn optimize_quad(&self, mesh: &mut Mesh2D, face_id: usize) -> bool {
        Self::optimize_quad_static(mesh, face_id)
    }

    /// Optimize a single quad (static version)
    fn optimize_quad_static(mesh: &mut Mesh2D, face_id: usize) -> bool {
        let face = &mesh.faces[face_id];
        if face.vertices.len() != 4 {
            return false;
        }

        // Try swapping diagonal edges
        let v0 = face.vertices[0];
        let v1 = face.vertices[1];
        let v2 = face.vertices[2];
        let v3 = face.vertices[3];

        // Calculate current quality
        let current_quality = Self::calculate_quad_quality_static(mesh, &[v0, v1, v2, v3]);

        // Try swapping diagonal v0-v2
        let new_quality = Self::calculate_quad_quality_static(mesh, &[v0, v1, v3, v2]);
        if new_quality > current_quality {
            // Swap diagonal
            mesh.faces[face_id].vertices = vec![v0, v1, v3, v2];
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_quad_mesher_creation() {
        let params = QuadMesherParams::default();
        let mesher = QuadMesher::new(params);
        assert!(mesher.input_mesh.is_none());
    }

    #[test]
    fn test_quad_mesher_with_simple_mesh() {
        // Create a simple quad mesh as two triangles
        let mut input_mesh = Mesh2D::new();
        let v0 = input_mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = input_mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = input_mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = input_mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        input_mesh.add_face(v0, v1, v2);
        input_mesh.add_face(v0, v2, v3);

        let mut mesher = QuadMesher::new(QuadMesherParams::default());
        mesher.set_input_mesh(input_mesh);

        let result = mesher.generate();
        assert!(result.is_ok());

        let quad_mesh = result.unwrap();
        assert!(!quad_mesh.faces.is_empty());
    }
}
