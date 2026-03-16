use crate::foundation::types::StandardReal;
use crate::geometry::Point;
use crate::topology::TopoDsFace;
use std::collections::{HashMap, HashSet};

/// Subdivision surface algorithm type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubdivisionType {
    /// Catmull-Clark subdivision for quad meshes
    CatmullClark,
    /// Loop subdivision for triangular meshes
    Loop,
}

/// Subdivision surface settings
#[derive(Debug, Clone, PartialEq)]
pub struct SubdivisionSettings {
    /// Subdivision algorithm type
    pub algorithm: SubdivisionType,
    /// Number of subdivision iterations
    pub iterations: u32,
    /// Crease sharpness (0.0-1.0)
    pub crease_sharpness: StandardReal,
    /// Whether to preserve boundary edges
    pub preserve_boundaries: bool,
}

impl Default for SubdivisionSettings {
    fn default() -> Self {
        Self {
            algorithm: SubdivisionType::CatmullClark,
            iterations: 2,
            crease_sharpness: 1.0,
            preserve_boundaries: true,
        }
    }
}

/// Subdivision surface implementation
#[derive(Debug, Clone)]
pub struct SubdivisionSurface {
    /// Original vertices
    vertices: Vec<Point>,
    /// Original faces (indices into vertices)
    faces: Vec<Vec<usize>>,
    /// Original edges (pairs of vertex indices)
    edges: Vec<(usize, usize)>,
    /// Subdivision settings
    settings: SubdivisionSettings,
}

impl SubdivisionSurface {
    /// Create a new subdivision surface from vertices and faces
    pub fn new(
        vertices: Vec<Point>,
        faces: Vec<Vec<usize>>,
        settings: SubdivisionSettings,
    ) -> Self {
        // Extract edges from faces
        let mut edges = Vec::new();
        let mut edge_set = HashSet::new();

        for face in &faces {
            for i in 0..face.len() {
                let j = (i + 1) % face.len();
                let edge = if face[i] < face[j] {
                    (face[i], face[j])
                } else {
                    (face[j], face[i])
                };

                if !edge_set.contains(&edge) {
                    edges.push(edge);
                    edge_set.insert(edge);
                }
            }
        }

        Self {
            vertices,
            faces,
            edges,
            settings,
        }
    }

    /// Create a subdivision surface from a TopoDsFace
    pub fn from_face(face: &TopoDsFace, settings: SubdivisionSettings) -> Self {
        // Extract vertices from the face
        let vertices: Vec<Point> = face.vertices().iter().map(|v| v.point().clone()).collect();

        // Extract edges from the face's wires
        let mut face_indices = Vec::new();
        if let Some(outer_wire) = face.outer_wire() {
            let edges = outer_wire.edges();
            for edge in edges {
                let start_vertex = edge.start_vertex();
                let end_vertex = edge.end_vertex();
                let start_point = start_vertex.point();
                let end_point = end_vertex.point();

                if let Some(start_idx) = vertices
                    .iter()
                    .position(|p| p.is_equal(&start_point, 0.001))
                {
                    if let Some(end_idx) =
                        vertices.iter().position(|p| p.is_equal(&end_point, 0.001))
                    {
                        face_indices.push(start_idx);
                        face_indices.push(end_idx);
                    }
                }
            }
        }

        let faces = vec![face_indices];

        Self::new(vertices, faces, settings)
    }

    /// Perform subdivision
    pub fn subdivide(&self) -> Self {
        match self.settings.algorithm {
            SubdivisionType::CatmullClark => self.catmull_clark_subdivision(),
            SubdivisionType::Loop => self.loop_subdivision(),
        }
    }

    /// Perform Catmull-Clark subdivision
    fn catmull_clark_subdivision(&self) -> Self {
        let mut new_vertices = self.vertices.clone();
        let mut new_faces = Vec::new();

        // Step 1: Compute face points
        let mut face_points = Vec::new();
        for face in &self.faces {
            let mut sum = Point::origin();
            for &idx in face {
                sum = sum + (self.vertices[idx] - Point::origin());
            }
            let face_point = Point::new(
                sum.x / face.len() as StandardReal,
                sum.y / face.len() as StandardReal,
                sum.z / face.len() as StandardReal,
            );
            face_points.push(face_point);
            new_vertices.push(face_point);
        }

        // Step 2: Compute edge points
        let mut edge_points = HashMap::new();
        for &(v1, v2) in self.edges.iter() {
            // Find all faces adjacent to this edge
            let mut adjacent_faces = Vec::new();
            for (face_idx, face) in self.faces.iter().enumerate() {
                if face.contains(&v1) && face.contains(&v2) {
                    adjacent_faces.push(face_idx);
                }
            }

            let edge_point = if adjacent_faces.len() == 2 {
                // Internal edge
                let face1 = face_points[adjacent_faces[0]];
                let face2 = face_points[adjacent_faces[1]];
                let v1_point = self.vertices[v1];
                let v2_point = self.vertices[v2];

                Point::new(
                    (face1.x + face2.x + v1_point.x + v2_point.x) / 4.0,
                    (face1.y + face2.y + v1_point.y + v2_point.y) / 4.0,
                    (face1.z + face2.z + v1_point.z + v2_point.z) / 4.0,
                )
            } else {
                // Boundary edge
                let v1_point = self.vertices[v1];
                let v2_point = self.vertices[v2];

                Point::new(
                    (v1_point.x + v2_point.x) / 2.0,
                    (v1_point.y + v2_point.y) / 2.0,
                    (v1_point.z + v2_point.z) / 2.0,
                )
            };

            edge_points.insert((v1, v2), edge_point);
            new_vertices.push(edge_point);
        }

        // Step 3: Update original vertices
        for (i, vertex) in self.vertices.iter().enumerate() {
            // Find all edges and faces adjacent to this vertex
            let mut adjacent_edges = Vec::new();
            let mut adjacent_faces = Vec::new();

            for (edge_idx, &(v1, v2)) in self.edges.iter().enumerate() {
                if v1 == i || v2 == i {
                    adjacent_edges.push(edge_idx);
                }
            }

            for (face_idx, face) in self.faces.iter().enumerate() {
                if face.contains(&i) {
                    adjacent_faces.push(face_idx);
                }
            }

            let n = adjacent_edges.len() as StandardReal;

            if n == 0.0 {
                // Isolated vertex, keep as is
                continue;
            }

            // Compute average of adjacent face points
            let mut face_avg = Point::origin();
            for &face_idx in &adjacent_faces {
                face_avg = face_avg + (face_points[face_idx] - Point::origin());
            }
            face_avg = Point::new(face_avg.x / n, face_avg.y / n, face_avg.z / n);

            // Compute average of adjacent edge midpoints
            let mut edge_avg = Point::origin();
            for &edge_idx in &adjacent_edges {
                let (v1, v2) = self.edges[edge_idx];
                let midpoint = Point::new(
                    (self.vertices[v1].x + self.vertices[v2].x) / 2.0,
                    (self.vertices[v1].y + self.vertices[v2].y) / 2.0,
                    (self.vertices[v1].z + self.vertices[v2].z) / 2.0,
                );
                edge_avg = edge_avg + (midpoint - Point::origin());
            }
            edge_avg = Point::new(edge_avg.x / n, edge_avg.y / n, edge_avg.z / n);

            // Update vertex position
            let new_x = (face_avg.x + 2.0 * edge_avg.x + (n - 3.0) * vertex.x) / n;
            let new_y = (face_avg.y + 2.0 * edge_avg.y + (n - 3.0) * vertex.y) / n;
            let new_z = (face_avg.z + 2.0 * edge_avg.z + (n - 3.0) * vertex.z) / n;

            new_vertices[i] = Point::new(new_x, new_y, new_z);
        }

        // Step 4: Create new faces
        let face_point_offset = self.vertices.len();
        let edge_point_offset = face_point_offset + face_points.len();

        for (face_idx, face) in self.faces.iter().enumerate() {
            let face_point_idx = face_point_offset + face_idx;

            for i in 0..face.len() {
                let j = (i + 1) % face.len();
                let v1 = face[i];
                let v2 = face[j];

                // Find edge point index
                let edge_key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                let edge_point_idx =
                    edge_point_offset + self.edges.iter().position(|&e| e == edge_key).unwrap();

                // Create new quad face
                new_faces.push(vec![v1, edge_point_idx, face_point_idx, edge_point_idx]);
            }
        }

        Self {
            vertices: new_vertices,
            faces: new_faces,
            edges: self.edges.clone(), // Edges will be recomputed in the new surface
            settings: self.settings.clone(),
        }
    }

    /// Perform Loop subdivision
    fn loop_subdivision(&self) -> Self {
        // TODO: Implement Loop subdivision for triangular meshes
        self.clone()
    }

    /// Get subdivided vertices
    pub fn vertices(&self) -> &[Point] {
        &self.vertices
    }

    /// Get subdivided faces
    pub fn faces(&self) -> &[Vec<usize>] {
        &self.faces
    }

    /// Get subdivision settings
    pub fn settings(&self) -> &SubdivisionSettings {
        &self.settings
    }

    /// Set subdivision settings
    pub fn set_settings(&mut self, settings: SubdivisionSettings) {
        self.settings = settings;
    }

    /// Perform multiple subdivision iterations
    pub fn subdivide_multiple(&self, iterations: u32) -> Self {
        let mut result = self.clone();
        for _ in 0..iterations {
            result = result.subdivide();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catmull_clark_subdivision() {
        // Create a simple cube
        let vertices = vec![
            Point::new(-1.0, -1.0, -1.0),
            Point::new(1.0, -1.0, -1.0),
            Point::new(1.0, 1.0, -1.0),
            Point::new(-1.0, 1.0, -1.0),
            Point::new(-1.0, -1.0, 1.0),
            Point::new(1.0, -1.0, 1.0),
            Point::new(1.0, 1.0, 1.0),
            Point::new(-1.0, 1.0, 1.0),
        ];

        let faces = vec![
            vec![0, 1, 2, 3], // Front face
            vec![1, 5, 6, 2], // Right face
            vec![5, 4, 7, 6], // Back face
            vec![4, 0, 3, 7], // Left face
            vec![3, 2, 6, 7], // Top face
            vec![4, 5, 1, 0], // Bottom face
        ];

        let settings = SubdivisionSettings::default();
        let surface = SubdivisionSurface::new(vertices, faces, settings);

        let subdivided = surface.subdivide();
        assert!(subdivided.vertices().len() > surface.vertices().len());
        assert!(subdivided.faces().len() > surface.faces().len());
    }

    #[test]
    fn test_subdivision_iterations() {
        let vertices = vec![
            Point::new(-1.0, -1.0, 0.0),
            Point::new(1.0, -1.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(-1.0, 1.0, 0.0),
        ];

        let faces = vec![vec![0, 1, 2, 3]];

        let settings = SubdivisionSettings::default();
        let surface = SubdivisionSurface::new(vertices, faces, settings);

        let subdivided = surface.subdivide_multiple(2);
        assert!(subdivided.vertices().len() > surface.vertices().len());
        assert!(subdivided.faces().len() > surface.faces().len());
    }
}
