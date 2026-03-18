//! Mesh subdivision module
//!
//! This module provides various mesh subdivision algorithms,
//! including Loop subdivision, Catmull-Clark subdivision, and others.

use super::mesh_data::{Mesh2D, Mesh3D, MeshVertex};
use crate::geometry::{Point, Vector};

/// Subdivision algorithm type
#[derive(Debug, Clone, PartialEq)]
pub enum SubdivisionAlgorithm {
    /// Loop subdivision for triangular meshes
    Loop,
    /// Catmull-Clark subdivision for quadrilateral meshes
    CatmullClark,
    /// Doo-Sabin subdivision for quadrilateral meshes
    DooSabin,
    /// Butterfly subdivision for triangular meshes
    Butterfly,
}

/// Subdivision parameters
#[derive(Debug, Clone)]
pub struct SubdivisionParams {
    /// Number of subdivision iterations
    pub iterations: usize,
    /// Crease angle threshold (in degrees)
    pub crease_angle: f64,
    /// Boundary interpolation method
    pub boundary_interpolation: BoundaryInterpolation,
}

/// Boundary interpolation method
#[derive(Debug, Clone, PartialEq)]
pub enum BoundaryInterpolation {
    /// Interpolate boundary vertices
    Interpolate,
    /// Approximate boundary vertices
    Approximate,
}

impl Default for SubdivisionParams {
    fn default() -> Self {
        Self {
            iterations: 1,
            crease_angle: 30.0,
            boundary_interpolation: BoundaryInterpolation::Interpolate,
        }
    }
}

/// Mesh subdivider
pub struct MeshSubdivider {
    /// Subdivision parameters
    params: SubdivisionParams,
}

impl MeshSubdivider {
    /// Create a new mesh subdivider
    pub fn new(params: SubdivisionParams) -> Self {
        Self { params }
    }

    /// Subdivide a 2D mesh
    pub fn subdivide_2d(&self, mesh: &Mesh2D) -> Mesh2D {
        let mut result = mesh.clone();
        
        for _ in 0..self.params.iterations {
            result = match self.params.boundary_interpolation {
                BoundaryInterpolation::Interpolate => {
                    self.loop_subdivision_2d(&result)
                }
                BoundaryInterpolation::Approximate => {
                    self.butterfly_subdivision_2d(&result)
                }
            };
        }
        
        result
    }

    /// Subdivide a 3D mesh
    pub fn subdivide_3d(&self, mesh: &Mesh3D) -> Mesh3D {
        let mut result = mesh.clone();
        
        for _ in 0..self.params.iterations {
            result = self.catmull_clark_subdivision_3d(&result);
        }
        
        result
    }

    /// Loop subdivision for 2D triangular meshes
    fn loop_subdivision_2d(&self, mesh: &Mesh2D) -> Mesh2D {
        let mut new_mesh = Mesh2D::new();
        
        // Step 1: Calculate new positions for existing vertices
        let mut vertex_positions = Vec::new();
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let new_position = self.calculate_loop_vertex_position(mesh, i);
            vertex_positions.push(new_position);
        }
        
        // Step 2: Create new vertices at edge midpoints
        let mut edge_midpoints = std::collections::HashMap::new();
        
        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                for i in 0..3 {
                    let v0 = face.vertices[i];
                    let v1 = face.vertices[(i + 1) % 3];
                    let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                    
                    if !edge_midpoints.contains_key(&edge) {
                        let midpoint = self.calculate_edge_midpoint(mesh, v0, v1);
                        let midpoint_idx = new_mesh.add_vertex(midpoint, Vector::zero());
                        edge_midpoints.insert(edge, midpoint_idx);
                    }
                }
            }
        }
        
        // Step 3: Create new faces
        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                let v0 = face.vertices[0];
                let v1 = face.vertices[1];
                let v2 = face.vertices[2];
                
                let e0 = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                let e1 = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                let e2 = if v2 < v0 { (v2, v0) } else { (v0, v2) };
                
                let m0 = edge_midpoints[&e0];
                let m1 = edge_midpoints[&e1];
                let m2 = edge_midpoints[&e2];
                
                // Add new vertices with updated positions
                let v0_idx = new_mesh.add_vertex(vertex_positions[v0], Vector::zero());
                let v1_idx = new_mesh.add_vertex(vertex_positions[v1], Vector::zero());
                let v2_idx = new_mesh.add_vertex(vertex_positions[v2], Vector::zero());
                
                // Create four new faces
                new_mesh.add_face(v0_idx, m0, m2);
                new_mesh.add_face(m0, v1_idx, m1);
                new_mesh.add_face(m2, m1, v2_idx);
                new_mesh.add_face(m0, m1, m2);
            }
        }
        
        new_mesh
    }

    /// Calculate new position for a vertex using Loop subdivision
    fn calculate_loop_vertex_position(&self, mesh: &Mesh2D, vertex_idx: usize) -> Point {
        // Find adjacent vertices
        let mut adjacent_vertices = Vec::new();
        
        for face in &mesh.faces {
            if face.vertices.contains(&vertex_idx) {
                for &v in &face.vertices {
                    if v != vertex_idx && !adjacent_vertices.contains(&v) {
                        adjacent_vertices.push(v);
                    }
                }
            }
        }
        
        let n = adjacent_vertices.len() as f64;
        
        if n == 0 {
            return mesh.vertices[vertex_idx].point;
        }
        
        // Calculate weights
        let weight = if n == 3 {
            3.0 / 16.0
        } else {
            3.0 / (8.0 * n)
        };
        
        let mut new_position = mesh.vertices[vertex_idx].point * (1.0 - n * weight);
        
        for &v in &adjacent_vertices {
            new_position += mesh.vertices[v].point * weight;
        }
        
        new_position
    }

    /// Calculate midpoint of an edge
    fn calculate_edge_midpoint(&self, mesh: &Mesh2D, v0: usize, v1: usize) -> Point {
        // Find all faces that contain this edge
        let mut adjacent_faces = Vec::new();
        
        for face in &mesh.faces {
            if face.vertices.contains(&v0) && face.vertices.contains(&v1) {
                adjacent_faces.push(face);
            }
        }
        
        if adjacent_faces.len() == 2 {
            // Edge is internal
            let v0_pos = mesh.vertices[v0].point;
            let v1_pos = mesh.vertices[v1].point;
            
            // Find the other vertices in each face
            let mut other_vertices = Vec::new();
            
            for face in &adjacent_faces {
                for &v in &face.vertices {
                    if v != v0 && v != v1 {
                        other_vertices.push(v);
                        break;
                    }
                }
            }
            
            if other_vertices.len() == 2 {
                let v2_pos = mesh.vertices[other_vertices[0]].point;
                let v3_pos = mesh.vertices[other_vertices[1]].point;
                
                return (v0_pos + v1_pos) * 0.5 + (v2_pos + v3_pos) * 0.125;
            }
        }
        
        // Edge is on boundary or has less than two adjacent faces
        (mesh.vertices[v0].point + mesh.vertices[v1].point) * 0.5
    }

    /// Butterfly subdivision for 2D triangular meshes
    fn butterfly_subdivision_2d(&self, mesh: &Mesh2D) -> Mesh2D {
        let mut new_mesh = Mesh2D::new();
        
        // Step 1: Create new vertices at edge midpoints
        let mut edge_midpoints = std::collections::HashMap::new();
        
        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                for i in 0..3 {
                    let v0 = face.vertices[i];
                    let v1 = face.vertices[(i + 1) % 3];
                    let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                    
                    if !edge_midpoints.contains_key(&edge) {
                        let midpoint = self.calculate_butterfly_midpoint(mesh, v0, v1);
                        let midpoint_idx = new_mesh.add_vertex(midpoint, Vector::zero());
                        edge_midpoints.insert(edge, midpoint_idx);
                    }
                }
            }
        }
        
        // Step 2: Create new faces
        for face in &mesh.faces {
            if face.vertices.len() == 3 {
                let v0 = face.vertices[0];
                let v1 = face.vertices[1];
                let v2 = face.vertices[2];
                
                let e0 = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                let e1 = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                let e2 = if v2 < v0 { (v2, v0) } else { (v0, v2) };
                
                let m0 = edge_midpoints[&e0];
                let m1 = edge_midpoints[&e1];
                let m2 = edge_midpoints[&e2];
                
                // Add original vertices
                let v0_idx = new_mesh.add_vertex(mesh.vertices[v0].point, Vector::zero());
                let v1_idx = new_mesh.add_vertex(mesh.vertices[v1].point, Vector::zero());
                let v2_idx = new_mesh.add_vertex(mesh.vertices[v2].point, Vector::zero());
                
                // Create four new faces
                new_mesh.add_face(v0_idx, m0, m2);
                new_mesh.add_face(m0, v1_idx, m1);
                new_mesh.add_face(m2, m1, v2_idx);
                new_mesh.add_face(m0, m1, m2);
            }
        }
        
        new_mesh
    }

    /// Calculate midpoint of an edge using butterfly subdivision
    fn calculate_butterfly_midpoint(&self, mesh: &Mesh2D, v0: usize, v1: usize) -> Point {
        // Find all faces that contain this edge
        let mut adjacent_faces = Vec::new();
        
        for face in &mesh.faces {
            if face.vertices.contains(&v0) && face.vertices.contains(&v1) {
                adjacent_faces.push(face);
            }
        }
        
        let v0_pos = mesh.vertices[v0].point;
        let v1_pos = mesh.vertices[v1].point;
        
        if adjacent_faces.len() == 2 {
            // Edge is internal
            let mut other_vertices = Vec::new();
            
            for face in &adjacent_faces {
                for &v in &face.vertices {
                    if v != v0 && v != v1 {
                        other_vertices.push(v);
                        break;
                    }
                }
            }
            
            if other_vertices.len() == 2 {
                let v2_pos = mesh.vertices[other_vertices[0]].point;
                let v3_pos = mesh.vertices[other_vertices[1]].point;
                
                // Butterfly weights
                return v0_pos * 0.5 + v1_pos * 0.5 + (v2_pos + v3_pos) * (-1.0 / 16.0);
            }
        }
        
        // Edge is on boundary or has less than two adjacent faces
        (v0_pos + v1_pos) * 0.5
    }

    /// Catmull-Clark subdivision for 3D meshes
    fn catmull_clark_subdivision_3d(&self, mesh: &Mesh3D) -> Mesh3D {
        let mut new_mesh = Mesh3D::new();
        
        // Step 1: Calculate face points
        let mut face_points = Vec::new();
        for tetra in &mesh.tetrahedrons {
            let v0 = &mesh.vertices[tetra.vertices[0]].point;
            let v1 = &mesh.vertices[tetra.vertices[1]].point;
            let v2 = &mesh.vertices[tetra.vertices[2]].point;
            let v3 = &mesh.vertices[tetra.vertices[3]].point;
            
            let face_point = (v0 + v1 + v2 + v3) * 0.25;
            face_points.push(face_point);
        }
        
        // Step 2: Calculate edge points
        let mut edge_points = std::collections::HashMap::new();
        
        for (tetra_idx, tetra) in mesh.tetrahedrons.iter().enumerate() {
            let edges = [
                (tetra.vertices[0], tetra.vertices[1]),
                (tetra.vertices[1], tetra.vertices[2]),
                (tetra.vertices[2], tetra.vertices[0]),
                (tetra.vertices[0], tetra.vertices[3]),
                (tetra.vertices[1], tetra.vertices[3]),
                (tetra.vertices[2], tetra.vertices[3]),
            ];
            
            for edge in edges {
                let edge_key = if edge.0 < edge.1 { edge } else { (edge.1, edge.0) };
                
                if !edge_points.contains_key(&edge_key) {
                    let v0 = &mesh.vertices[edge_key.0].point;
                    let v1 = &mesh.vertices[edge_key.1].point;
                    let face_point = face_points[tetra_idx];
                    
                    let edge_point = (v0 + v1 + face_point * 2.0) * 0.25;
                    let edge_point_idx = new_mesh.add_vertex(edge_point, Vector::zero());
                    edge_points.insert(edge_key, edge_point_idx);
                }
            }
        }
        
        // Step 3: Calculate new vertex positions
        let mut vertex_positions = Vec::new();
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            // Find adjacent edges and faces
            let mut adjacent_edges = std::collections::HashSet::new();
            let mut adjacent_faces = Vec::new();
            
            for (tetra_idx, tetra) in mesh.tetrahedrons.iter().enumerate() {
                if tetra.vertices.contains(&i) {
                    adjacent_faces.push(face_points[tetra_idx]);
                    
                    let edges = [
                        (tetra.vertices[0], tetra.vertices[1]),
                        (tetra.vertices[1], tetra.vertices[2]),
                        (tetra.vertices[2], tetra.vertices[0]),
                        (tetra.vertices[0], tetra.vertices[3]),
                        (tetra.vertices[1], tetra.vertices[3]),
                        (tetra.vertices[2], tetra.vertices[3]),
                    ];
                    
                    for edge in edges {
                        if edge.0 == i || edge.1 == i {
                            let edge_key = if edge.0 < edge.1 { edge } else { (edge.1, edge.0) };
                            adjacent_edges.insert(edge_key);
                        }
                    }
                }
            }
            
            let n = adjacent_edges.len() as f64;
            
            if n == 0 {
                vertex_positions.push(vertex.point);
                continue;
            }
            
            // Calculate new position
            let k = 3.0 / (8.0 * n);
            let beta = if n == 3 {
                3.0 / 16.0
            } else {
                k
            };
            
            let mut new_position = vertex.point * (1.0 - n * beta);
            
            // Add contributions from adjacent edge points
            for edge_key in &adjacent_edges {
                if let Some(&edge_point_idx) = edge_points.get(edge_key) {
                    let edge_point = new_mesh.vertices[edge_point_idx].point;
                    new_position += edge_point * beta;
                }
            }
            
            // Add contribution from adjacent face points
            if !adjacent_faces.is_empty() {
                let face_avg = adjacent_faces.iter().fold(Point::origin(), |sum, &p| sum + p) / adjacent_faces.len() as f64;
                new_position += face_avg * (1.0 / n);
            }
            
            vertex_positions.push(new_position);
        }
        
        // Step 4: Create new tetrahedrons
        for (tetra_idx, tetra) in mesh.tetrahedrons.iter().enumerate() {
            let v0 = tetra.vertices[0];
            let v1 = tetra.vertices[1];
            let v2 = tetra.vertices[2];
            let v3 = tetra.vertices[3];
            
            let e0 = if v0 < v1 { (v0, v1) } else { (v1, v0) };
            let e1 = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            let e2 = if v2 < v0 { (v2, v0) } else { (v0, v2) };
            let e3 = if v0 < v3 { (v0, v3) } else { (v3, v0) };
            let e4 = if v1 < v3 { (v1, v3) } else { (v3, v1) };
            let e5 = if v2 < v3 { (v2, v3) } else { (v3, v2) };
            
            let m0 = edge_points[&e0];
            let m1 = edge_points[&e1];
            let m2 = edge_points[&e2];
            let m3 = edge_points[&e3];
            let m4 = edge_points[&e4];
            let m5 = edge_points[&e5];
            
            let f = new_mesh.add_vertex(face_points[tetra_idx], Vector::zero());
            
            // Add new vertices with updated positions
            let v0_idx = new_mesh.add_vertex(vertex_positions[v0], Vector::zero());
            let v1_idx = new_mesh.add_vertex(vertex_positions[v1], Vector::zero());
            let v2_idx = new_mesh.add_vertex(vertex_positions[v2], Vector::zero());
            let v3_idx = new_mesh.add_vertex(vertex_positions[v3], Vector::zero());
            
            // Create new tetrahedrons
            new_mesh.add_tetrahedron(v0_idx, m0, m2, m3);
            new_mesh.add_tetrahedron(m0, v1_idx, m1, m4);
            new_mesh.add_tetrahedron(m2, m1, v2_idx, m5);
            new_mesh.add_tetrahedron(m3, m4, m5, v3_idx);
            new_mesh.add_tetrahedron(m0, m2, m1, f);
            new_mesh.add_tetrahedron(m0, m3, m4, f);
            new_mesh.add_tetrahedron(m2, m5, m3, f);
            new_mesh.add_tetrahedron(m1, m5, m4, f);
        }
        
        new_mesh
    }
}

impl Default for MeshSubdivider {
    fn default() -> Self {
        Self::new(SubdivisionParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_loop_subdivision() {
        // Create a simple triangular mesh
        let mut mesh = Mesh2D::new();
        
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.5, 1.0, 0.0), Vector::zero());
        
        mesh.add_face(v0, v1, v2);
        
        let subdivider = MeshSubdivider::default();
        let subdivided = subdivider.subdivide_2d(&mesh);
        
        // After one iteration, we should have 4 faces
        assert_eq!(subdivided.faces.len(), 4);
    }

    #[test]
    fn test_catmull_clark_subdivision() {
        // Create a simple tetrahedral mesh
        let mut mesh = Mesh3D::new();
        
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0), Vector::zero());
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0), Vector::zero());
        let v2 = mesh.add_vertex(Point::new(0.5, 1.0, 0.0), Vector::zero());
        let v3 = mesh.add_vertex(Point::new(0.5, 0.5, 1.0), Vector::zero());
        
        mesh.add_tetrahedron(v0, v1, v2, v3);
        
        let subdivider = MeshSubdivider::default();
        let subdivided = subdivider.subdivide_3d(&mesh);
        
        // After one iteration, we should have 8 tetrahedrons
        assert_eq!(subdivided.tetrahedrons.len(), 8);
    }
}
