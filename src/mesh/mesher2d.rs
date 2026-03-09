//! 2D mesh generation
//!
//! This module provides functionality for 2D triangle meshing.

use super::mesh_data::Mesh2D;
use crate::geometry::{Point, Vector};

/// 2D mesher error types
#[derive(Debug)]
pub enum Mesher2DError {
    /// Invalid input geometry
    InvalidGeometry,
    /// Empty input
    EmptyInput,
    /// Meshing failed
    MeshingFailed,
    /// Invalid parameters
    InvalidParameters,
}

/// 2D mesher parameters
#[derive(Debug, Clone)]
pub struct Mesher2DParams {
    /// Maximum triangle area
    pub max_area: f64,
    /// Minimum triangle angle (in degrees)
    pub min_angle: f64,
    /// Mesh density factor
    pub density_factor: f64,
    /// Use quality mesh
    pub quality_mesh: bool,
}

impl Default for Mesher2DParams {
    fn default() -> Self {
        Self {
            max_area: 1.0,
            min_angle: 20.0,
            density_factor: 1.0,
            quality_mesh: true,
        }
    }
}

/// 2D mesher
pub struct Mesher2D {
    /// Mesher parameters
    params: Mesher2DParams,
    /// Input polygon vertices
    input_vertices: Vec<Point>,
    /// Input polygon edges
    input_edges: Vec<(usize, usize)>,
}

impl Mesher2D {
    /// Create a new 2D mesher
    pub fn new(params: Mesher2DParams) -> Self {
        Self {
            params,
            input_vertices: Vec::new(),
            input_edges: Vec::new(),
        }
    }

    /// Add input polygon
    pub fn add_polygon(&mut self, vertices: &[Point]) {
        if vertices.len() < 3 {
            return;
        }

        let start_idx = self.input_vertices.len();
        for vertex in vertices {
            self.input_vertices.push(vertex.clone());
        }

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            self.input_edges.push((start_idx + i, start_idx + j));
        }
    }

    /// Generate mesh
    pub fn generate(&mut self) -> Result<Mesh2D, Mesher2DError> {
        if self.input_vertices.is_empty() {
            return Err(Mesher2DError::EmptyInput);
        }

        // Create initial mesh
        let mut mesh = self.create_initial_mesh()?;

        // Refine mesh
        if self.params.quality_mesh {
            self.refine_mesh(&mut mesh);
        }

        // Optimize mesh quality
        self.optimize_mesh(&mut mesh);

        Ok(mesh)
    }

    /// Create initial mesh
    fn create_initial_mesh(&self) -> Result<Mesh2D, Mesher2DError> {
        let mut mesh = Mesh2D::new();

        // Add input vertices to mesh
        let mut vertex_map = HashMap::new();
        for (idx, vertex) in self.input_vertices.iter().enumerate() {
            let mesh_vertex_id = mesh.add_vertex(vertex.clone());
            vertex_map.insert(idx, mesh_vertex_id);
        }

        // Simple triangulation using ear clipping
        let triangles = self.ear_clipping()?;
        for triangle in triangles {
            let v0 = vertex_map[&triangle.0];
            let v1 = vertex_map[&triangle.1];
            let v2 = vertex_map[&triangle.2];
            mesh.add_face(v0, v1, v2);
        }

        Ok(mesh)
    }

    /// Ear clipping algorithm for triangulation
    fn ear_clipping(&self) -> Result<Vec<(usize, usize, usize)>, Mesher2DError> {
        if self.input_vertices.len() < 3 {
            return Err(Mesher2DError::InvalidGeometry);
        }

        let mut triangles = Vec::new();
        let mut polygon = (0..self.input_vertices.len()).collect::<Vec<_>>();

        while polygon.len() > 3 {
            let mut ear_found = false;
            for i in 0..polygon.len() {
                let prev = (i + polygon.len() - 1) % polygon.len();
                let curr = i;
                let next = (i + 1) % polygon.len();

                if self.is_ear(&polygon, prev, curr, next) {
                    triangles.push((polygon[prev], polygon[curr], polygon[next]));
                    polygon.remove(curr);
                    ear_found = true;
                    break;
                }
            }

            if !ear_found {
                return Err(Mesher2DError::MeshingFailed);
            }
        }

        if polygon.len() == 3 {
            triangles.push((polygon[0], polygon[1], polygon[2]));
        }

        Ok(triangles)
    }

    /// Check if a vertex is an ear
    fn is_ear(&self, polygon: &[usize], prev: usize, curr: usize, next: usize) -> bool {
        let p0 = &self.input_vertices[polygon[prev]];
        let p1 = &self.input_vertices[polygon[curr]];
        let p2 = &self.input_vertices[polygon[next]];

        // Check if the angle is convex
        if !self.is_convex(p0, p1, p2) {
            return false;
        }

        // Check if any other vertex is inside the triangle
        for i in 0..polygon.len() {
            if i == prev || i == curr || i == next {
                continue;
            }

            let p = &self.input_vertices[polygon[i]];
            if self.point_in_triangle(p, p0, p1, p2) {
                return false;
            }
        }

        true
    }

    /// Check if three points form a convex angle
    fn is_convex(&self, p0: &Point, p1: &Point, p2: &Point) -> bool {
        let v1 = Vector::new(p1.x - p0.x, p1.y - p0.y, 0.0);
        let v2 = Vector::new(p2.x - p1.x, p2.y - p1.y, 0.0);
        let cross = v1.x * v2.y - v1.y * v2.x;
        cross > 0.0
    }

    /// Check if a point is inside a triangle
    fn point_in_triangle(&self, p: &Point, p0: &Point, p1: &Point, p2: &Point) -> bool {
        let area =
            0.5 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
        let sign = if area < 0.0 { -1.0 } else { 1.0 };

        let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y) * sign;
        let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y) * sign;

        s > 0.0 && t > 0.0 && (s + t) < 2.0 * area.abs()
    }

    /// Refine mesh
    fn refine_mesh(&self, mesh: &mut Mesh2D) {
        let mut edges_to_split = Vec::new();

        // Identify edges that need splitting
        for (edge_id, edge) in mesh.edges.iter().enumerate() {
            let v0 = &mesh.vertices[edge.vertices[0]];
            let v1 = &mesh.vertices[edge.vertices[1]];
            let length =
                ((v1.point.x - v0.point.x).powi(2) + (v1.point.y - v0.point.y).powi(2)).sqrt();

            if length > self.params.max_area.sqrt() {
                edges_to_split.push(edge_id);
            }
        }

        // Split edges
        for edge_id in edges_to_split {
            self.split_edge(mesh, edge_id);
        }
    }

    /// Split an edge
    fn split_edge(&self, mesh: &mut Mesh2D, edge_id: usize) {
        let edge = mesh.edges[edge_id].clone();
        let v0 = mesh.vertices[edge.vertices[0]].clone();
        let v1 = mesh.vertices[edge.vertices[1]].clone();

        // Create new vertex at midpoint
        let midpoint = Point::new(
            (v0.point.x + v1.point.x) / 2.0,
            (v0.point.y + v1.point.y) / 2.0,
            0.0,
        );
        let new_vertex_id = mesh.add_vertex(midpoint);

        // Replace edge with two new edges
        mesh.edges[edge_id].vertices[1] = new_vertex_id;
        mesh.add_edge(new_vertex_id, edge.vertices[1]);

        // Update faces that use this edge
        for face in &mut mesh.faces {
            for i in 0..face.vertices.len() {
                let j = (i + 1) % face.vertices.len();
                if (face.vertices[i] == edge.vertices[0] && face.vertices[j] == edge.vertices[1])
                    || (face.vertices[i] == edge.vertices[1]
                        && face.vertices[j] == edge.vertices[0])
                {
                    face.vertices.insert(j, new_vertex_id);
                    break;
                }
            }
        }
    }

    /// Optimize mesh quality
    fn optimize_mesh(&self, mesh: &mut Mesh2D) {
        // Simple optimization: swap edges to improve triangle quality
        let mut improved = true;
        while improved {
            improved = false;
            for face_id in 0..mesh.faces.len() {
                if self.optimize_face(mesh, face_id) {
                    improved = true;
                }
            }
        }
    }

    /// Optimize a single face
    fn optimize_face(&self, mesh: &mut Mesh2D, face_id: usize) -> bool {
        let face = &mesh.faces[face_id];
        if face.vertices.len() != 3 {
            return false;
        }

        // Check if edge swap would improve quality
        // This is a simplified implementation
        false
    }
}

use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesher2d_creation() {
        let params = Mesher2DParams::default();
        let mesher = Mesher2D::new(params);
        assert!(mesher.input_vertices.is_empty());
        assert!(mesher.input_edges.is_empty());
    }

    #[test]
    fn test_add_polygon() {
        let mut mesher = Mesher2D::new(Mesher2DParams::default());
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        mesher.add_polygon(&vertices);
        assert_eq!(mesher.input_vertices.len(), 4);
        assert_eq!(mesher.input_edges.len(), 4);
    }

    #[test]
    fn test_ear_clipping() {
        let mut mesher = Mesher2D::new(Mesher2DParams::default());
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        mesher.add_polygon(&vertices);
        let triangles = mesher.ear_clipping().unwrap();
        assert_eq!(triangles.len(), 2);
    }

    #[test]
    fn test_is_convex() {
        let mesher = Mesher2D::new(Mesher2DParams::default());
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 0.0);
        assert!(mesher.is_convex(&p0, &p1, &p2));
    }

    #[test]
    fn test_point_in_triangle() {
        let mesher = Mesher2D::new(Mesher2DParams::default());
        let p0 = Point::new(0.0, 0.0, 0.0);
        let p1 = Point::new(1.0, 0.0, 0.0);
        let p2 = Point::new(0.0, 1.0, 0.0);
        let p = Point::new(0.25, 0.25, 0.0);
        assert!(mesher.point_in_triangle(&p, &p0, &p1, &p2));
    }

    #[test]
    fn test_generate_mesh() {
        let mut mesher = Mesher2D::new(Mesher2DParams::default());
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        mesher.add_polygon(&vertices);
        let mesh = mesher.generate().unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.faces.is_empty());
    }
}
