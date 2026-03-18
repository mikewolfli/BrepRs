//! Geometric primitives module
//!
//! This module provides implementations for various geometric primitives,
//! including prism, pyramid, and polyhedron.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{topods_solid::TopoDsSolid, TopoDsShape};
use std::sync::Arc;

/// Prism primitive
#[derive(Debug, Clone)]
pub struct Prism {
    /// Base polygon vertices
    base_vertices: Vec<Point>,
    /// Height of the prism
    height: f64,
    /// Direction vector
    direction: Vector,
}

impl Prism {
    /// Create a new prism
    pub fn new(base_vertices: Vec<Point>, height: f64, direction: Vector) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }

        if height <= 0.0 {
            return Err("Height must be positive".to_string());
        }

        if direction.magnitude() < 1e-10 {
            return Err("Direction vector must be non-zero".to_string());
        }

        Ok(Self {
            base_vertices,
            height,
            direction: {
                let mut dir = direction;
                dir.normalize();
                dir
            },
        })
    }

    /// Create a prism with default direction (along Z-axis)
    pub fn new_with_default_direction(
        base_vertices: Vec<Point>,
        height: f64,
    ) -> Result<Self, String> {
        Self::new(base_vertices, height, Vector::new(0.0, 0.0, 1.0))
    }

    /// Build the prism as a solid
    pub fn build(&self) -> Arc<TopoDsSolid> {
        // Create base face
        let base_face = self.create_face(&self.base_vertices);

        // Create top face by translating base vertices
        let top_vertices: Vec<Point> = self
            .base_vertices
            .iter()
            .map(|v| {
                Point::new(
                    v.x + self.direction.x * self.height,
                    v.y + self.direction.y * self.height,
                    v.z + self.direction.z * self.height,
                )
            })
            .collect();
        let top_face = self.create_face(&top_vertices);

        // Create side faces
        let mut side_faces = Vec::new();
        for i in 0..self.base_vertices.len() {
            let v0 = self.base_vertices[i];
            let v1 = self.base_vertices[(i + 1) % self.base_vertices.len()];
            let v2 = top_vertices[(i + 1) % self.base_vertices.len()];
            let v3 = top_vertices[i];

            let side_face = self.create_face(&vec![v0, v1, v2, v3]);
            side_faces.push(side_face);
        }

        // Combine all faces into a solid
        self.create_solid(base_face, top_face, side_faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // Create face using TopoDsFace
        use crate::topology::{
            topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_vertex::TopoDsVertex,
            topods_wire::TopoDsWire,
        };

        if vertices.len() < 3 {
            return TopoDsShape::new(crate::topology::ShapeType::Face);
        }

        // Create vertices
        let mut vertex_handles = Vec::new();
        for vertex in vertices {
            let topo_vertex = TopoDsVertex::new(*vertex);
            vertex_handles.push(std::sync::Arc::new(topo_vertex));
        }

        // Create edges
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let v1 = vertex_handles[i].clone();
            let v2 = vertex_handles[j].clone();
            let edge = TopoDsEdge::new(v1, v2);
            edges.push(std::sync::Arc::new(edge));
        }

        // Create wire
        let mut wire = TopoDsWire::new();
        for edge in edges {
            wire.add_edge(std::sync::Arc::new(edge));
        }

        // Create face
        let face = TopoDsFace::with_wires(vec![std::sync::Arc::new(wire)]);
        face.shape().clone()
    }

    /// Create a solid from faces
    fn create_solid(
        &self,
        base_face: TopoDsShape,
        top_face: TopoDsShape,
        side_faces: Vec<TopoDsShape>,
    ) -> Arc<TopoDsSolid> {
        // Create solid by converting to mesh first
        use crate::topology::topods_face::TopoDsFace;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Extract vertices and faces from all faces
        let all_faces = vec![base_face, top_face];
        let mut all_faces = all_faces.into_iter();
        all_faces.extend(side_faces.into_iter());

        let mut vertex_map = std::collections::HashMap::new();
        let mut vertex_id = 0;

        for face_shape in all_faces {
            if let Some(face) = face_shape.as_face() {
                if let Some(outer_wire) = face.outer_wire() {
                    let wire_vertices = outer_wire.vertices();
                    let mut face_indices = Vec::new();

                    for vertex in wire_vertices {
                        let point = vertex.point();
                        let key = (point.x, point.y, point.z);

                        if let Some(&id) = vertex_map.get(&key) {
                            face_indices.push(id);
                        } else {
                            vertex_map.insert(key, vertex_id);
                            vertices.push(point);
                            face_indices.push(vertex_id);
                            vertex_id += 1;
                        }
                    }

                    if face_indices.len() >= 3 {
                        faces.push(face_indices);
                    }
                }
            }
        }

        // Create solid from mesh
        let solid = TopoDsSolid::from_mesh(vertices, faces);
        Arc::new(solid)
    }

    /// Get the base vertices
    pub fn base_vertices(&self) -> &[Point] {
        &self.base_vertices
    }

    /// Get the height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get the direction
    pub fn direction(&self) -> &Vector {
        &self.direction
    }
}

/// Pyramid primitive
#[derive(Debug, Clone)]
pub struct Pyramid {
    /// Base polygon vertices
    base_vertices: Vec<Point>,
    /// Apex point
    apex: Point,
}

impl Pyramid {
    /// Create a new pyramid
    pub fn new(base_vertices: Vec<Point>, apex: Point) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }

        Ok(Self {
            base_vertices,
            apex,
        })
    }

    /// Create a pyramid with specified height
    pub fn new_with_height(base_vertices: Vec<Point>, height: f64) -> Result<Self, String> {
        if base_vertices.len() < 3 {
            return Err("Base must have at least 3 vertices".to_string());
        }

        // Calculate centroid of base
        let (sum_x, sum_y, sum_z) = base_vertices
            .iter()
            .fold((0.0, 0.0, 0.0), |(sum_x, sum_y, sum_z), p| {
                (sum_x + p.x, sum_y + p.y, sum_z + p.z)
            });
        let count = base_vertices.len() as f64;
        let centroid = Point::new(sum_x / count, sum_y / count, sum_z / count);

        // Calculate apex point
        let apex = Point::new(centroid.x + 0.0, centroid.y + 0.0, centroid.z + height);

        Ok(Self {
            base_vertices,
            apex,
        })
    }

    /// Build the pyramid as a solid
    pub fn build(&self) -> Arc<TopoDsSolid> {
        // Create base face
        let base_face = self.create_face(&self.base_vertices);

        // Create side faces
        let mut side_faces = Vec::new();
        for i in 0..self.base_vertices.len() {
            let v0 = self.base_vertices[i];
            let v1 = self.base_vertices[(i + 1) % self.base_vertices.len()];

            let side_face = self.create_face(&vec![v0, v1, self.apex]);
            side_faces.push(side_face);
        }

        // Combine all faces into a solid
        self.create_solid(base_face, side_faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // Create face using TopoDsFace
        use crate::topology::{
            topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_vertex::TopoDsVertex,
            topods_wire::TopoDsWire,
        };

        if vertices.len() < 3 {
            return TopoDsShape::new(crate::topology::ShapeType::Face);
        }

        // Create vertices
        let mut vertex_handles = Vec::new();
        for vertex in vertices {
            let topo_vertex = TopoDsVertex::new(*vertex);
            vertex_handles.push(std::sync::Arc::new(topo_vertex));
        }

        // Create edges
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let v1 = vertex_handles[i].clone();
            let v2 = vertex_handles[j].clone();
            let edge = TopoDsEdge::new(v1, v2);
            edges.push(std::sync::Arc::new(edge));
        }

        // Create wire
        let mut wire = TopoDsWire::new();
        for edge in edges {
            wire.add_edge(std::sync::Arc::new(edge));
        }

        // Create face
        let face = TopoDsFace::with_wires(vec![std::sync::Arc::new(wire)]);
        face.shape().clone()
    }

    /// Create a solid from faces
    fn create_solid(
        &self,
        base_face: TopoDsShape,
        side_faces: Vec<TopoDsShape>,
    ) -> Arc<TopoDsSolid> {
        // Create solid by converting to mesh first
        use crate::topology::topods_face::TopoDsFace;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Extract vertices and faces from all faces
        let mut all_faces = vec![base_face];
        all_faces.extend(side_faces);

        let mut vertex_map = std::collections::HashMap::new();
        let mut vertex_id = 0;

        for face_shape in all_faces {
            if let Some(face) = face_shape.as_face() {
                if let Some(outer_wire) = face.outer_wire() {
                    let wire_vertices = outer_wire.vertices();
                    let mut face_indices = Vec::new();

                    for vertex in wire_vertices {
                        let point = vertex.point();
                        let key = (point.x, point.y, point.z);

                        if let Some(&id) = vertex_map.get(&key) {
                            face_indices.push(id);
                        } else {
                            vertex_map.insert(key, vertex_id);
                            vertices.push(point);
                            face_indices.push(vertex_id);
                            vertex_id += 1;
                        }
                    }

                    if face_indices.len() >= 3 {
                        faces.push(face_indices);
                    }
                }
            }
        }

        // Create solid from mesh
        let solid = TopoDsSolid::from_mesh(vertices, faces);
        Arc::new(solid)
    }

    /// Get the base vertices
    pub fn base_vertices(&self) -> &[Point] {
        &self.base_vertices
    }

    /// Get the apex point
    pub fn apex(&self) -> &Point {
        &self.apex
    }
}

/// Polyhedron primitive
#[derive(Debug, Clone)]
pub struct Polyhedron {
    /// Vertices of the polyhedron
    vertices: Vec<Point>,
    /// Faces of the polyhedron (each face is a list of vertex indices)
    faces: Vec<Vec<usize>>,
}

impl Polyhedron {
    /// Create a new polyhedron
    pub fn new(vertices: Vec<Point>, faces: Vec<Vec<usize>>) -> Result<Self, String> {
        if vertices.len() < 4 {
            return Err("Polyhedron must have at least 4 vertices".to_string());
        }

        if faces.len() < 4 {
            return Err("Polyhedron must have at least 4 faces".to_string());
        }

        // Validate face indices
        for face in &faces {
            if face.len() < 3 {
                return Err("Each face must have at least 3 vertices".to_string());
            }

            for &index in face {
                if index >= vertices.len() {
                    return Err("Invalid vertex index in face".to_string());
                }
            }
        }

        Ok(Self { vertices, faces })
    }

    /// Create a tetrahedron (simplest polyhedron)
    pub fn tetrahedron() -> Self {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
            Point::new(0.5, 0.5, 1.0),
        ];

        let faces = vec![vec![0, 1, 2], vec![0, 1, 3], vec![1, 2, 3], vec![2, 0, 3]];

        Self { vertices, faces }
    }

    /// Create a cube as a polyhedron
    pub fn cube() -> Self {
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.0, 0.0, 1.0),
            Point::new(1.0, 0.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(0.0, 1.0, 1.0),
        ];

        let faces = vec![
            vec![0, 1, 2, 3], // Bottom face
            vec![4, 5, 6, 7], // Top face
            vec![0, 1, 5, 4], // Front face
            vec![2, 3, 7, 6], // Back face
            vec![0, 3, 7, 4], // Left face
            vec![1, 2, 6, 5], // Right face
        ];

        Self { vertices, faces }
    }

    /// Build the polyhedron as a solid
    pub fn build(&self) -> Arc<TopoDsSolid> {
        // Create faces
        let mut faces = Vec::new();
        for face_indices in &self.faces {
            let face_vertices: Vec<Point> =
                face_indices.iter().map(|&i| self.vertices[i]).collect();
            let face = self.create_face(&face_vertices);
            faces.push(face);
        }

        // Combine all faces into a solid
        self.create_solid(faces)
    }

    /// Create a face from vertices
    fn create_face(&self, vertices: &[Point]) -> TopoDsShape {
        // Create face using TopoDsFace
        use crate::topology::{
            topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_vertex::TopoDsVertex,
            topods_wire::TopoDsWire,
        };

        if vertices.len() < 3 {
            return TopoDsShape::new(crate::topology::ShapeType::Face);
        }

        // Create vertices
        let mut vertex_handles = Vec::new();
        for vertex in vertices {
            let topo_vertex = TopoDsVertex::new(*vertex);
            vertex_handles.push(std::sync::Arc::new(topo_vertex));
        }

        // Create edges
        let mut edges = Vec::new();
        for i in 0..vertices.len() {
            let j = (i + 1) % vertices.len();
            let v1 = vertex_handles[i].clone();
            let v2 = vertex_handles[j].clone();
            let edge = TopoDsEdge::new(v1, v2);
            edges.push(std::sync::Arc::new(edge));
        }

        // Create wire
        let mut wire = TopoDsWire::new();
        for edge in edges {
            wire.add_edge(std::sync::Arc::new(edge));
        }

        // Create face
        let face = TopoDsFace::with_wires(vec![std::sync::Arc::new(wire)]);
        face.shape().clone()
    }

    /// Create a solid from faces
    fn create_solid(&self, faces: Vec<TopoDsShape>) -> Arc<TopoDsSolid> {
        // Create solid by converting to mesh first
        use crate::topology::topods_face::TopoDsFace;

        let mut vertices = Vec::new();
        let mut face_indices = Vec::new();

        // Extract vertices and faces from all faces
        let mut vertex_map = std::collections::HashMap::new();
        let mut vertex_id = 0;

        for face_shape in faces {
            if let Some(face) = face_shape.as_face() {
                if let Some(outer_wire) = face.outer_wire() {
                    let wire_vertices = outer_wire.vertices();
                    let mut indices = Vec::new();

                    for vertex in wire_vertices {
                        let point = vertex.point();
                        let key = (point.x, point.y, point.z);

                        if let Some(&id) = vertex_map.get(&key) {
                            indices.push(id);
                        } else {
                            vertex_map.insert(key, vertex_id);
                            vertices.push(point);
                            indices.push(vertex_id);
                            vertex_id += 1;
                        }
                    }

                    if indices.len() >= 3 {
                        face_indices.push(indices);
                    }
                }
            }
        }

        // Create solid from mesh
        let solid = TopoDsSolid::from_mesh(vertices, face_indices);
        Arc::new(solid)
    }

    /// Get the vertices
    pub fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    /// Get the faces
    pub fn faces(&self) -> &[Vec<usize>] {
        &self.faces
    }

    /// Calculate the volume of the polyhedron
    pub fn volume(&self) -> f64 {
        // Use the divergence theorem to calculate volume
        let mut volume = 0.0;

        for face in &self.faces {
            // Calculate centroid of the face
            let (sum_x, sum_y, sum_z) =
                face.iter()
                    .fold((0.0, 0.0, 0.0), |(sum_x, sum_y, sum_z), &i| {
                        let p = self.vertices[i];
                        (sum_x + p.x, sum_y + p.y, sum_z + p.z)
                    });
            let count = face.len() as f64;
            let centroid = Point::new(sum_x / count, sum_y / count, sum_z / count);

            // Calculate area vector of the face
            let area_vector = self.calculate_face_area_vector(face);

            // Add contribution to volume
            volume += centroid.dot(&area_vector) / 3.0;
        }

        f64::abs(volume)
    }

    /// Calculate the area vector of a face
    fn calculate_face_area_vector(&self, face: &[usize]) -> Vector {
        let mut area_vector = Vector::zero();

        for i in 0..face.len() {
            let v0 = self.vertices[face[i]];
            let v1 = self.vertices[face[(i + 1) % face.len()]];

            area_vector += v0.cross(&v1);
        }

        area_vector * 0.5
    }
}

/// Create a box (rectangular prism) solid
pub fn make_box(dx: f64, dy: f64, dz: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let half_dx = dx / 2.0;
    let half_dy = dy / 2.0;
    let half_dz = dz / 2.0;

    let vertices = vec![
        Point::new(center.x - half_dx, center.y - half_dy, center.z - half_dz),
        Point::new(center.x + half_dx, center.y - half_dy, center.z - half_dz),
        Point::new(center.x + half_dx, center.y + half_dy, center.z - half_dz),
        Point::new(center.x - half_dx, center.y + half_dy, center.z - half_dz),
        Point::new(center.x - half_dx, center.y - half_dy, center.z + half_dz),
        Point::new(center.x + half_dx, center.y - half_dy, center.z + half_dz),
        Point::new(center.x + half_dx, center.y + half_dy, center.z + half_dz),
        Point::new(center.x - half_dx, center.y + half_dy, center.z + half_dz),
    ];

    let faces = vec![
        vec![0, 1, 2, 3],
        vec![4, 5, 6, 7],
        vec![0, 1, 5, 4],
        vec![2, 3, 7, 6],
        vec![0, 3, 7, 4],
        vec![1, 2, 6, 5],
    ];

    let polyhedron = Polyhedron::new(vertices, faces).unwrap();
    let solid_arc = polyhedron.build();
    (*solid_arc).clone()
}

/// Create a cylinder solid
pub fn make_cylinder(radius: f64, height: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let half_height = height / 2.0;

    let segments = 32;
    let mut vertices = Vec::new();

    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Point::new(x, y, center.z - half_height));
    }

    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Point::new(x, y, center.z + half_height));
    }

    let mut faces = Vec::new();

    for i in 0..segments {
        let next = (i + 1) % segments;
        faces.push(vec![i, next, next + segments, i + segments]);
    }

    let bottom_center_idx = vertices.len();
    vertices.push(Point::new(center.x, center.y, center.z - half_height));
    let top_center_idx = vertices.len();
    vertices.push(Point::new(center.x, center.y, center.z + half_height));

    for i in 0..segments {
        let next = (i + 1) % segments;
        faces.push(vec![bottom_center_idx, next, i]);
    }

    for i in 0..segments {
        let next = (i + 1) % segments;
        faces.push(vec![top_center_idx, i + segments, next + segments]);
    }

    let polyhedron = Polyhedron::new(vertices, faces).unwrap();
    let solid_arc = polyhedron.build();
    (*solid_arc).clone()
}

/// Create a sphere solid
pub fn make_sphere(radius: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let segments = 32;
    let rings = 16;
    let mut vertices = Vec::new();

    for ring in 0..rings {
        let phi = std::f64::consts::PI * ring as f64 / rings as f64;
        let z = radius * phi.cos();
        let r = radius * phi.sin();

        for seg in 0..segments {
            let theta = 2.0 * std::f64::consts::PI * seg as f64 / segments as f64;
            let x = center.x + r * theta.cos();
            let y = center.y + r * theta.sin();
            vertices.push(Point::new(x, y, center.z + z));
        }
    }

    let mut faces = Vec::new();

    for ring in 0..rings {
        for seg in 0..segments {
            let next_seg = (seg + 1) % segments;
            let current_ring_start = ring * segments;
            let next_ring_start = (ring + 1) * segments;
            faces.push(vec![
                current_ring_start + seg,
                current_ring_start + next_seg,
                next_ring_start + next_seg,
            ]);
            faces.push(vec![
                current_ring_start + seg,
                next_ring_start + next_seg,
                next_ring_start + seg,
            ]);
        }
    }

    let polyhedron = Polyhedron::new(vertices, faces).unwrap();
    let solid_arc = polyhedron.build();
    (*solid_arc).clone()
}

/// Create a cone solid
pub fn make_cone(radius: f64, height: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let segments = 32;
    let half_height = height / 2.0;
    let mut vertices = Vec::new();

    let apex_idx = vertices.len();
    vertices.push(Point::new(center.x, center.y, center.z + half_height));

    for i in 0..segments {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / segments as f64;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        vertices.push(Point::new(x, y, center.z - half_height));
    }

    let mut faces = Vec::new();

    for i in 0..segments {
        let next = (i + 1) % segments;
        faces.push(vec![apex_idx, 1 + i, 1 + next]);
    }

    let polyhedron = Polyhedron::new(vertices, faces).unwrap();
    let solid_arc = polyhedron.build();
    (*solid_arc).clone()
}

/// Create a torus solid
pub fn make_torus(major_radius: f64, minor_radius: f64, center: Option<Point>) -> TopoDsSolid {
    let center = center.unwrap_or(Point::origin());
    let major_segments = 32;
    let minor_segments = 16;
    let mut vertices = Vec::new();

    for i in 0..major_segments {
        let major_angle = 2.0 * std::f64::consts::PI * i as f64 / major_segments as f64;
        let major_x = major_radius * major_angle.cos();
        let major_y = major_radius * major_angle.sin();

        for j in 0..minor_segments {
            let minor_angle = 2.0 * std::f64::consts::PI * j as f64 / minor_segments as f64;
            let minor_x = minor_radius * minor_angle.cos();
            let minor_y = minor_radius * minor_angle.sin();
            let x = center.x + major_x + minor_x * major_angle.cos();
            let y = center.y + major_y + minor_y * major_angle.sin();
            let z = center.z + minor_y * major_angle.sin();
            vertices.push(Point::new(x, y, z));
        }
    }

    let mut faces = Vec::new();

    for i in 0..major_segments {
        for j in 0..minor_segments {
            let next_i = (i + 1) % major_segments;
            let next_j = (j + 1) % minor_segments;
            let idx = i * minor_segments + j;
            let next_idx = next_i * minor_segments + j;
            let idx_next_j = i * minor_segments + next_j;
            let next_idx_next_j = next_i * minor_segments + next_j;
            faces.push(vec![idx, next_idx, next_idx_next_j]);
            faces.push(vec![idx, next_idx_next_j, idx_next_j]);
        }
    }

    let polyhedron = Polyhedron::new(vertices, faces).unwrap();
    let solid_arc = polyhedron.build();
    (*solid_arc).clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_prism_creation() {
        let base_vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
        ];

        let result = Prism::new_with_default_direction(base_vertices, 1.0);
        assert!(result.is_ok());

        let prism = result.unwrap();
        assert_eq!(prism.base_vertices().len(), 3);
        assert!((prism.height() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_pyramid_creation() {
        let base_vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];

        let result = Pyramid::new_with_height(base_vertices, 1.0);
        assert!(result.is_ok());

        let pyramid = result.unwrap();
        assert_eq!(pyramid.base_vertices().len(), 4);
        assert!((pyramid.apex().z - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_polyhedron_tetrahedron() {
        let tetrahedron = Polyhedron::tetrahedron();
        assert_eq!(tetrahedron.vertices().len(), 4);
        assert_eq!(tetrahedron.faces().len(), 4);

        let volume = tetrahedron.volume();
        assert!((volume - 1.0 / 6.0).abs() < 1e-10); // Volume of a regular tetrahedron with edge length sqrt(2)
    }

    #[test]
    fn test_polyhedron_cube() {
        let cube = Polyhedron::cube();
        assert_eq!(cube.vertices().len(), 8);
        assert_eq!(cube.faces().len(), 6);

        let volume = cube.volume();
        assert!((volume - 1.0).abs() < 1e-10); // Volume of a unit cube
    }
}
