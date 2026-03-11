//! Boundary layer mesh generation
//!
//! This module provides functionality for generating boundary layer meshes
//! (prism layers) for CFD simulations, which are essential for resolving
//! flow near solid walls.

use super::mesh_data::{Mesh3D, MeshPrism, MeshVertex};
use crate::geometry::{Point, Vector};
use std::collections::{HashMap, HashSet};

/// Boundary layer mesher error types
#[derive(Debug)]
pub enum BoundaryLayerMesherError {
    /// Invalid input mesh
    InvalidInputMesh,
    /// Meshing failed
    MeshingFailed,
    /// No boundary faces found
    NoBoundaryFaces,
    /// Invalid parameters
    InvalidParameters,
}

/// Boundary layer mesher parameters
#[derive(Debug, Clone)]
pub struct BoundaryLayerParams {
    /// Number of prism layers
    pub num_layers: usize,
    /// First layer thickness
    pub first_layer_thickness: f64,
    /// Growth rate between layers
    pub growth_rate: f64,
    /// Maximum layer thickness
    pub max_layer_thickness: f64,
    /// Use smooth transition
    pub smooth_transition: bool,
    /// Boundary face IDs to apply layers to
    pub boundary_face_ids: Option<HashSet<usize>>,
}

impl Default for BoundaryLayerParams {
    fn default() -> Self {
        Self {
            num_layers: 5,
            first_layer_thickness: 0.01,
            growth_rate: 1.2,
            max_layer_thickness: 0.1,
            smooth_transition: true,
            boundary_face_ids: None,
        }
    }
}

/// Boundary layer mesher
pub struct BoundaryLayerMesher {
    /// Mesher parameters
    params: BoundaryLayerParams,
    /// Input mesh
    input_mesh: Option<Mesh3D>,
    /// Output mesh with boundary layers
    output_mesh: Mesh3D,
    /// Boundary face to normal mapping
    boundary_normals: HashMap<usize, Vector>,
    /// Vertex to boundary normal mapping
    vertex_normals: HashMap<usize, Vector>,
}

impl BoundaryLayerMesher {
    /// Create a new boundary layer mesher
    pub fn new(params: BoundaryLayerParams) -> Self {
        Self {
            params,
            input_mesh: None,
            output_mesh: Mesh3D::new(),
            boundary_normals: HashMap::new(),
            vertex_normals: HashMap::new(),
        }
    }

    /// Set input mesh
    pub fn set_input_mesh(&mut self, mesh: Mesh3D) {
        self.input_mesh = Some(mesh);
    }

    /// Generate boundary layer mesh
    pub fn generate(&mut self) -> Result<Mesh3D, BoundaryLayerMesherError> {
        let input_mesh = self
            .input_mesh
            .take()
            .ok_or(BoundaryLayerMesherError::InvalidInputMesh)?;

        if input_mesh.faces.is_empty() {
            return Err(BoundaryLayerMesherError::InvalidInputMesh);
        }

        // Identify boundary faces
        let boundary_faces = self.identify_boundary_faces(&input_mesh);
        if boundary_faces.is_empty() {
            return Err(BoundaryLayerMesherError::NoBoundaryFaces);
        }

        // Calculate boundary normals
        self.calculate_boundary_normals(&input_mesh, &boundary_faces);

        // Calculate vertex normals
        self.calculate_vertex_normals(&input_mesh, &boundary_faces);

        // Generate boundary layers
        self.generate_boundary_layers(&input_mesh, &boundary_faces);

        // Add original mesh elements
        self.add_original_mesh_elements(&input_mesh);

        // Restore input mesh
        self.input_mesh = Some(input_mesh);

        Ok(self.output_mesh.clone())
    }

    /// Identify boundary faces
    fn identify_boundary_faces(&self, mesh: &Mesh3D) -> HashSet<usize> {
        let mut face_adjacency = HashMap::new();

        // Count face adjacencies
        for (face_id, face) in mesh.faces.iter().enumerate() {
            for i in 0..face.vertices.len() {
                let v0 = face.vertices[i];
                let v1 = face.vertices[(i + 1) % face.vertices.len()];
                let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                face_adjacency
                    .entry(edge)
                    .or_insert(Vec::new())
                    .push(face_id);
            }
        }

        // Identify boundary faces (faces with at least one edge that's only in one face)
        let mut boundary_faces = HashSet::new();
        for (edge, faces) in face_adjacency {
            if faces.len() == 1 {
                boundary_faces.insert(faces[0]);
            }
        }

        // Apply boundary face filter if specified
        if let Some(ref boundary_face_ids) = self.params.boundary_face_ids {
            boundary_faces = boundary_faces
                .intersection(boundary_face_ids)
                .cloned()
                .collect();
        }

        boundary_faces
    }

    /// Calculate boundary normals
    fn calculate_boundary_normals(&mut self, mesh: &Mesh3D, boundary_faces: &HashSet<usize>) {
        for &face_id in boundary_faces {
            let face = &mesh.faces[face_id];
            if let Some(normal) = face.normal {
                self.boundary_normals
                    .insert(face_id, Vector::new(normal[0], normal[1], normal[2]));
            } else {
                // Calculate normal if not provided
                let normal = self.calculate_face_normal(mesh, face);
                self.boundary_normals.insert(face_id, normal);
            }
        }
    }

    /// Calculate face normal
    fn calculate_face_normal(
        &self,
        mesh: &Mesh3D,
        face: &crate::mesh::mesh_data::MeshFace,
    ) -> Vector {
        if face.vertices.len() < 3 {
            return Vector::new(0.0, 0.0, 1.0);
        }

        let v0 = &mesh.vertices[face.vertices[0]].point;
        let v1 = &mesh.vertices[face.vertices[1]].point;
        let v2 = &mesh.vertices[face.vertices[2]].point;

        let v1v0 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
        let v2v0 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
        let normal = v1v0.cross(&v2v0);

        normal.normalized()
    }

    /// Calculate vertex normals
    fn calculate_vertex_normals(&mut self, mesh: &Mesh3D, boundary_faces: &HashSet<usize>) {
        // Initialize vertex normals
        for &face_id in boundary_faces {
            let face = &mesh.faces[face_id];
            let normal = self.boundary_normals[&face_id];

            for &vertex_id in &face.vertices {
                let entry = self
                    .vertex_normals
                    .entry(vertex_id)
                    .or_insert(Vector::new(0.0, 0.0, 0.0));
                *entry = entry.add(&normal);
            }
        }

        // Normalize vertex normals
        for (vertex_id, normal) in &mut self.vertex_normals {
            *normal = normal.normalized();
        }
    }

    /// Generate boundary layers
    fn generate_boundary_layers(&mut self, mesh: &Mesh3D, boundary_faces: &HashSet<usize>) {
        // Copy original vertices
        let mut vertex_map = HashMap::new();
        for (idx, vertex) in mesh.vertices.iter().enumerate() {
            let new_vertex_id = self.output_mesh.add_vertex(vertex.point.clone());
            vertex_map.insert(idx, new_vertex_id);
        }

        // Generate prism layers
        let layer_thicknesses = self.calculate_layer_thicknesses();

        // Create vertex layers
        let mut layer_vertices = vec![vertex_map.clone()];

        for (layer_idx, thickness) in layer_thicknesses.iter().enumerate() {
            let mut current_layer = HashMap::new();

            for &face_id in boundary_faces {
                let face = &mesh.faces[face_id];

                for &vertex_id in &face.vertices {
                    if let Some(normal) = self.vertex_normals.get(&vertex_id) {
                        let original_vertex = &mesh.vertices[vertex_id].point;
                        let offset = normal.scale(*thickness);
                        let new_point = Point::new(
                            original_vertex.x + offset.x,
                            original_vertex.y + offset.y,
                            original_vertex.z + offset.z,
                        );

                        let new_vertex_id = self.output_mesh.add_vertex(new_point);
                        current_layer.insert(vertex_id, new_vertex_id);
                    }
                }
            }

            layer_vertices.push(current_layer);
        }

        // Create prism elements
        for &face_id in boundary_faces {
            let face = &mesh.faces[face_id];

            for layer_idx in 0..self.params.num_layers {
                let layer1 = &layer_vertices[layer_idx];
                let layer2 = &layer_vertices[layer_idx + 1];

                // Create prism for each face
                let mut prism_vertices = Vec::new();

                for &vertex_id in &face.vertices {
                    if let Some(&v1) = layer1.get(&vertex_id) {
                        prism_vertices.push(v1);
                    }
                }

                for &vertex_id in face.vertices.iter().rev() {
                    if let Some(&v2) = layer2.get(&vertex_id) {
                        prism_vertices.push(v2);
                    }
                }

                if prism_vertices.len() == 6 {
                    self.output_mesh.add_prism(
                        prism_vertices[0],
                        prism_vertices[1],
                        prism_vertices[2],
                        prism_vertices[3],
                        prism_vertices[4],
                        prism_vertices[5],
                    );
                }
            }
        }
    }

    /// Calculate layer thicknesses
    fn calculate_layer_thicknesses(&self) -> Vec<f64> {
        let mut thicknesses = Vec::new();
        let mut current_thickness = 0.0;

        for i in 0..self.params.num_layers {
            let layer_thickness =
                self.params.first_layer_thickness * (self.params.growth_rate.powi(i as i32));
            let clamped_thickness = layer_thickness.min(self.params.max_layer_thickness);
            current_thickness += clamped_thickness;
            thicknesses.push(current_thickness);
        }

        thicknesses
    }

    /// Add original mesh elements
    fn add_original_mesh_elements(&mut self, mesh: &Mesh3D) {
        // Add original edges
        for edge in &mesh.edges {
            self.output_mesh
                .add_edge(edge.vertices[0], edge.vertices[1]);
        }

        // Add original faces
        for face in &mesh.faces {
            self.output_mesh.add_face(face.vertices.clone());
        }

        // Add original tetrahedrons
        for tetra in &mesh.tetrahedrons {
            self.output_mesh.add_tetrahedron(
                tetra.vertices[0],
                tetra.vertices[1],
                tetra.vertices[2],
                tetra.vertices[3],
            );
        }

        // Add original hexahedrons
        for hex in &mesh.hexahedrons {
            self.output_mesh.add_hexahedron(
                hex.vertices[0],
                hex.vertices[1],
                hex.vertices[2],
                hex.vertices[3],
                hex.vertices[4],
                hex.vertices[5],
                hex.vertices[6],
                hex.vertices[7],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_boundary_layer_mesher_creation() {
        let params = BoundaryLayerParams::default();
        let mesher = BoundaryLayerMesher::new(params);
        assert!(mesher.input_mesh.is_none());
    }

    #[test]
    fn test_calculate_layer_thicknesses() {
        let params = BoundaryLayerParams {
            num_layers: 3,
            first_layer_thickness: 0.01,
            growth_rate: 1.2,
            max_layer_thickness: 0.1,
            ..Default::default()
        };
        let mesher = BoundaryLayerMesher::new(params);
        let thicknesses = mesher.calculate_layer_thicknesses();
        assert_eq!(thicknesses.len(), 3);
        assert!(thicknesses[0] > 0.0);
        assert!(thicknesses[1] > thicknesses[0]);
        assert!(thicknesses[2] > thicknesses[1]);
    }

    #[test]
    fn test_generate_boundary_layers() {
        // Create a simple cube mesh
        let mut input_mesh = Mesh3D::new();
        let v0 = input_mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = input_mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = input_mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = input_mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v4 = input_mesh.add_vertex(Point::new(0.0, 0.0, 1.0));
        let v5 = input_mesh.add_vertex(Point::new(1.0, 0.0, 1.0));
        let v6 = input_mesh.add_vertex(Point::new(1.0, 1.0, 1.0));
        let v7 = input_mesh.add_vertex(Point::new(0.0, 1.0, 1.0));

        // Add faces (bottom face)
        input_mesh.add_face(vec![v0, v1, v2]);
        input_mesh.add_face(vec![v0, v2, v3]);

        let mut mesher = BoundaryLayerMesher::new(BoundaryLayerParams {
            num_layers: 2,
            first_layer_thickness: 0.01,
            ..Default::default()
        });
        mesher.set_input_mesh(input_mesh);

        let result = mesher.generate();
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.prisms.is_empty());
    }
}
