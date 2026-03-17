//! 2D mesh generation
//!
//! This module provides functionality for 2D triangle meshing.

use super::mesh_data::{Mesh2D, MeshVertex};
use crate::geometry::{Point, Vector};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

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
pub struct Mesher2DParams {
    /// Maximum triangle area
    pub max_area: f64,
    /// Minimum triangle angle (in degrees)
    pub min_angle: f64,
    /// Mesh density factor
    pub density_factor: f64,
    /// Use quality mesh
    pub quality_mesh: bool,
    /// Curvature refinement factor
    pub curvature_factor: f64,
    /// Proximity refinement factor
    pub proximity_factor: f64,
    /// Size field control
    pub size_field: Option<Box<dyn Fn(&Point) -> f64 + Send + Sync>>,
    /// Maximum edge length
    pub max_edge_length: f64,
    /// Minimum edge length
    pub min_edge_length: f64,
}

impl Clone for Mesher2DParams {
    fn clone(&self) -> Self {
        Self {
            max_area: self.max_area,
            min_angle: self.min_angle,
            density_factor: self.density_factor,
            quality_mesh: self.quality_mesh,
            curvature_factor: self.curvature_factor,
            proximity_factor: self.proximity_factor,
            size_field: None, // Cannot clone closure
            max_edge_length: self.max_edge_length,
            min_edge_length: self.min_edge_length,
        }
    }
}

impl std::fmt::Debug for Mesher2DParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesher2DParams")
            .field("max_area", &self.max_area)
            .field("min_angle", &self.min_angle)
            .field("density_factor", &self.density_factor)
            .field("quality_mesh", &self.quality_mesh)
            .field("curvature_factor", &self.curvature_factor)
            .field("proximity_factor", &self.proximity_factor)
            .field(
                "size_field",
                &self.size_field.as_ref().map(|_| "<function>"),
            )
            .field("max_edge_length", &self.max_edge_length)
            .field("min_edge_length", &self.min_edge_length)
            .finish()
    }
}

impl Default for Mesher2DParams {
    fn default() -> Self {
        Self {
            max_area: 1.0,
            min_angle: 20.0,
            density_factor: 1.0,
            quality_mesh: true,
            curvature_factor: 1.0,
            proximity_factor: 1.0,
            size_field: None,
            max_edge_length: 1.0,
            min_edge_length: 0.01,
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
    /// Curvature values for vertices
    vertex_curvatures: Vec<f64>,
    /// Size field values for vertices
    vertex_sizes: Vec<f64>,
}

impl Mesher2D {
    /// Create a new 2D mesher
    pub fn new(params: Mesher2DParams) -> Self {
        Self {
            params,
            input_vertices: Vec::new(),
            input_edges: Vec::new(),
            vertex_curvatures: Vec::new(),
            vertex_sizes: Vec::new(),
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
            self.vertex_curvatures.push(0.0);
            self.vertex_sizes.push(self.params.max_edge_length);
        }

        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            self.input_edges.push((start_idx + i, start_idx + j));
        }
    }

    /// Compute vertex curvatures
    fn compute_curvatures(&mut self) {
        for i in 0..self.input_vertices.len() {
            let prev = (i + self.input_vertices.len() - 1) % self.input_vertices.len();
            let next = (i + 1) % self.input_vertices.len();

            let p_prev = &self.input_vertices[prev];
            let p_curr = &self.input_vertices[i];
            let p_next = &self.input_vertices[next];

            // Compute curvature as the angle between consecutive edges
            let v1 = Vector::new(p_curr.x - p_prev.x, p_curr.y - p_prev.y, 0.0);
            let v2 = Vector::new(p_next.x - p_curr.x, p_next.y - p_curr.y, 0.0);

            let dot = v1.x * v2.x + v1.y * v2.y;
            let mag1 = (v1.x * v1.x + v1.y * v1.y).sqrt();
            let mag2 = (v2.x * v2.x + v2.y * v2.y).sqrt();

            let curvature = if mag1 > 1e-6 && mag2 > 1e-6 {
                let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
                let angle = cos_angle.acos();
                angle / std::f64::consts::PI
            } else {
                0.0
            };

            self.vertex_curvatures[i] = curvature * self.params.curvature_factor;
        }
    }

    /// Compute size field
    fn compute_size_field(&mut self) {
        for i in 0..self.input_vertices.len() {
            let p = &self.input_vertices[i];

            // Use custom size field if provided
            let size = if let Some(ref size_field) = self.params.size_field {
                size_field(p)
            } else {
                // Default size field based on curvature and proximity
                let curvature = self.vertex_curvatures[i];
                let base_size = self.params.max_edge_length;
                let curvature_size = base_size * (1.0 - curvature * 0.8);

                // Proximity to other vertices
                let mut min_distance = f64::MAX;
                for j in 0..self.input_vertices.len() {
                    if i != j {
                        let p_j = &self.input_vertices[j];
                        let distance = ((p.x - p_j.x).powi(2) + (p.y - p_j.y).powi(2)).sqrt();
                        min_distance = min_distance.min(distance);
                    }
                }

                let proximity_size = min_distance * 0.5 * self.params.proximity_factor;
                curvature_size
                    .min(proximity_size)
                    .max(self.params.min_edge_length)
            };

            self.vertex_sizes[i] = size;
        }
    }

    /// Generate mesh
    pub fn generate(&mut self) -> Result<Mesh2D, Mesher2DError> {
        if self.input_vertices.is_empty() {
            return Err(Mesher2DError::EmptyInput);
        }

        // Compute curvatures and size field
        self.compute_curvatures();
        self.compute_size_field();

        // Create initial mesh
        let mut mesh = self.create_initial_mesh()?;

        // Refine mesh based on curvature and size field
        if self.params.quality_mesh {
            self.refine_mesh(&mut mesh);
        }

        // Optimize mesh quality
        self.optimize_mesh(&mut mesh);

        // Fix inverted triangles
        self.fix_inverted_triangles(&mut mesh);

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
        // Identify edges that need splitting
        #[cfg(feature = "rayon")]
        {
            let edges_to_split: Vec<usize> = mesh
                .edges
                .par_iter()
                .enumerate()
                .filter_map(|(edge_id, edge)| {
                    let v0 = &mesh.vertices[edge.vertices[0]];
                    let v1 = &mesh.vertices[edge.vertices[1]];
                    let length = ((v1.point.x - v0.point.x).powi(2)
                        + (v1.point.y - v0.point.y).powi(2))
                    .sqrt();

                    // Determine appropriate edge length based on vertices
                    let mut max_edge_length = self.params.max_edge_length;

                    // Check if vertices are in the input polygon
                    for (i, input_vertex) in self.input_vertices.iter().enumerate() {
                        if (input_vertex.x - v0.point.x).abs() < 1e-6
                            && (input_vertex.y - v0.point.y).abs() < 1e-6
                        {
                            max_edge_length = max_edge_length.min(self.vertex_sizes[i]);
                        }
                        if (input_vertex.x - v1.point.x).abs() < 1e-6
                            && (input_vertex.y - v1.point.y).abs() < 1e-6
                        {
                            max_edge_length = max_edge_length.min(self.vertex_sizes[i]);
                        }
                    }

                    if length > max_edge_length {
                        Some(edge_id)
                    } else {
                        None
                    }
                })
                .collect();

            // Split edges
            for edge_id in edges_to_split {
                self.split_edge(mesh, edge_id);
            }
        }

        #[cfg(not(feature = "rayon"))]
        {
            let mut edges_to_split = Vec::new();

            for (edge_id, edge) in mesh.edges.iter().enumerate() {
                let v0 = &mesh.vertices[edge.vertices[0]];
                let v1 = &mesh.vertices[edge.vertices[1]];
                let length =
                    ((v1.point.x - v0.point.x).powi(2) + (v1.point.y - v0.point.y).powi(2)).sqrt();

                // Determine appropriate edge length based on vertices
                let mut max_edge_length = self.params.max_edge_length;

                // Check if vertices are in the input polygon
                for (i, input_vertex) in self.input_vertices.iter().enumerate() {
                    if (input_vertex.x - v0.point.x).abs() < 1e-6
                        && (input_vertex.y - v0.point.y).abs() < 1e-6
                    {
                        max_edge_length = max_edge_length.min(self.vertex_sizes[i]);
                    }
                    if (input_vertex.x - v1.point.x).abs() < 1e-6
                        && (input_vertex.y - v1.point.y).abs() < 1e-6
                    {
                        max_edge_length = max_edge_length.min(self.vertex_sizes[i]);
                    }
                }

                if length > max_edge_length {
                    edges_to_split.push(edge_id);
                }
            }

            // Split edges
            for edge_id in edges_to_split {
                self.split_edge(mesh, edge_id);
            }
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
        let v0 = &mesh.vertices[face.vertices[0]];
        let v1 = &mesh.vertices[face.vertices[1]];
        let v2 = &mesh.vertices[face.vertices[2]];

        // Calculate current quality
        let current_quality = self.calculate_triangle_quality(v0, v1, v2);

        // Try edge swaps
        for i in 0..3 {
            let j = (i + 1) % 3;
            let k = (i + 2) % 3;

            // Find adjacent face
            let adjacent_face = self.find_adjacent_face(mesh, face.vertices[i], face.vertices[j]);
            if let Some(adj_face_id) = adjacent_face {
                let adj_face = &mesh.faces[adj_face_id];
                if adj_face.vertices.len() == 3 {
                    // Get the fourth vertex
                    let mut fourth_vertex = 0;
                    for &v in &adj_face.vertices {
                        if v != face.vertices[i] && v != face.vertices[j] {
                            fourth_vertex = v;
                            break;
                        }
                    }

                    // Calculate new quality if we swap edges
                    let new_quality1 = self.calculate_triangle_quality(
                        &mesh.vertices[face.vertices[i]],
                        &mesh.vertices[fourth_vertex],
                        &mesh.vertices[face.vertices[k]],
                    );

                    let new_quality2 = self.calculate_triangle_quality(
                        &mesh.vertices[face.vertices[j]],
                        &mesh.vertices[fourth_vertex],
                        &mesh.vertices[face.vertices[k]],
                    );

                    let new_quality = (new_quality1 + new_quality2) / 2.0;
                    if new_quality > current_quality {
                        // Perform edge swap
                        self.swap_edges(
                            mesh,
                            face_id,
                            adj_face_id,
                            face.vertices[i],
                            face.vertices[j],
                            fourth_vertex,
                            face.vertices[k],
                        );
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Calculate triangle quality based on minimum angle
    fn calculate_triangle_quality(&self, v0: &MeshVertex, v1: &MeshVertex, v2: &MeshVertex) -> f64 {
        let angle1 = self.calculate_angle(&v1.point, &v0.point, &v2.point);
        let angle2 = self.calculate_angle(&v0.point, &v1.point, &v2.point);
        let angle3 = self.calculate_angle(&v0.point, &v2.point, &v1.point);

        let min_angle = angle1.min(angle2).min(angle3);
        min_angle / 60.0 // Normalize to [0, 1] where 1 is ideal (60 degrees)
    }

    /// Calculate angle between three points
    fn calculate_angle(&self, p1: &Point, p2: &Point, p3: &Point) -> f64 {
        let v1 = Vector::new(p1.x - p2.x, p1.y - p2.y, 0.0);
        let v2 = Vector::new(p3.x - p2.x, p3.y - p2.y, 0.0);

        let dot = v1.x * v2.x + v1.y * v2.y;
        let mag1 = (v1.x * v1.x + v1.y * v1.y).sqrt();
        let mag2 = (v2.x * v2.x + v2.y * v2.y).sqrt();

        if mag1 < 1e-6 || mag2 < 1e-6 {
            return 0.0;
        }

        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        cos_angle.acos() * 180.0 / std::f64::consts::PI
    }

    /// Find adjacent face sharing an edge
    fn find_adjacent_face(&self, mesh: &Mesh2D, v1: usize, v2: usize) -> Option<usize> {
        for (face_id, face) in mesh.faces.iter().enumerate() {
            if face.vertices.len() == 3 {
                let has_v1 = face.vertices.contains(&v1);
                let has_v2 = face.vertices.contains(&v2);
                if has_v1 && has_v2 {
                    return Some(face_id);
                }
            }
        }
        None
    }

    /// Swap edges between two triangles
    fn swap_edges(
        &self,
        mesh: &mut Mesh2D,
        face1_id: usize,
        face2_id: usize,
        v1: usize,
        v2: usize,
        v3: usize,
        v4: usize,
    ) {
        // Update first face
        mesh.faces[face1_id].vertices = vec![v1, v3, v4];

        // Update second face
        mesh.faces[face2_id].vertices = vec![v2, v3, v4];
    }

    /// Ensure no inverted triangles
    fn fix_inverted_triangles(&self, mesh: &mut Mesh2D) {
        for face in &mut mesh.faces {
            if face.vertices.len() == 3 {
                let v0 = &mesh.vertices[face.vertices[0]];
                let v1 = &mesh.vertices[face.vertices[1]];
                let v2 = &mesh.vertices[face.vertices[2]];

                // Calculate normal
                let vec1 = Vector::new(v1.point.x - v0.point.x, v1.point.y - v0.point.y, 0.0);
                let vec2 = Vector::new(v2.point.x - v0.point.x, v2.point.y - v0.point.y, 0.0);
                let cross = vec1.x * vec2.y - vec1.y * vec2.x;

                // If normal is negative, reverse the face
                if cross < 0.0 {
                    face.vertices.reverse();
                }
            }
        }
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
