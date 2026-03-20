//! 3D mesh generation
//!
//! This module provides functionality for 3D tetrahedral meshing.

use super::mesh_data::Mesh3D;
use crate::geometry::{Point, Vector};
use std::collections::{HashMap, HashSet};

/// 3D mesher error types
#[derive(Debug)]
pub enum Mesher3DError {
    /// Invalid input geometry
    InvalidGeometry,
    /// Empty input
    EmptyInput,
    /// Meshing failed
    MeshingFailed,
    /// Invalid parameters
    InvalidParameters,
}

/// 3D mesher parameters
pub struct Mesher3DParams {
    /// Maximum tetrahedron volume
    pub max_volume: f64,
    /// Minimum dihedral angle (in degrees)
    pub min_dihedral_angle: f64,
    /// Maximum dihedral angle (in degrees)
    pub max_dihedral_angle: f64,
    /// Minimum aspect ratio
    pub min_aspect_ratio: f64,
    /// Maximum aspect ratio
    pub max_aspect_ratio: f64,
    /// Minimum radius ratio
    pub min_radius_ratio: f64,
    /// Mesh density factor
    pub density_factor: f64,
    /// Use quality mesh
    pub quality_mesh: bool,
    /// Use size field
    pub use_size_field: bool,
    /// Maximum edge length
    pub max_edge_length: f64,
    /// Minimum edge length
    pub min_edge_length: f64,
    /// Curvature refinement factor
    pub curvature_factor: f64,
    /// Proximity refinement factor
    pub proximity_factor: f64,
    /// Size field control
    pub size_field: Option<Box<dyn Fn(&Point) -> f64>>,
}

impl Clone for Mesher3DParams {
    fn clone(&self) -> Self {
        Self {
            max_volume: self.max_volume,
            min_dihedral_angle: self.min_dihedral_angle,
            max_dihedral_angle: self.max_dihedral_angle,
            min_aspect_ratio: self.min_aspect_ratio,
            max_aspect_ratio: self.max_aspect_ratio,
            min_radius_ratio: self.min_radius_ratio,
            density_factor: self.density_factor,
            quality_mesh: self.quality_mesh,
            use_size_field: self.use_size_field,
            max_edge_length: self.max_edge_length,
            min_edge_length: self.min_edge_length,
            curvature_factor: self.curvature_factor,
            proximity_factor: self.proximity_factor,
            size_field: None, // Cannot clone closure
        }
    }
}

impl std::fmt::Debug for Mesher3DParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesher3DParams")
            .field("max_volume", &self.max_volume)
            .field("min_dihedral_angle", &self.min_dihedral_angle)
            .field("max_dihedral_angle", &self.max_dihedral_angle)
            .field("min_aspect_ratio", &self.min_aspect_ratio)
            .field("max_aspect_ratio", &self.max_aspect_ratio)
            .field("min_radius_ratio", &self.min_radius_ratio)
            .field("density_factor", &self.density_factor)
            .field("quality_mesh", &self.quality_mesh)
            .field("use_size_field", &self.use_size_field)
            .field("max_edge_length", &self.max_edge_length)
            .field("min_edge_length", &self.min_edge_length)
            .field("curvature_factor", &self.curvature_factor)
            .field("proximity_factor", &self.proximity_factor)
            .field(
                "size_field",
                &self.size_field.as_ref().map(|_| "<function>"),
            )
            .finish()
    }
}

impl Default for Mesher3DParams {
    fn default() -> Self {
        Self {
            max_volume: 1.0,
            min_dihedral_angle: 10.0,
            max_dihedral_angle: 170.0,
            min_aspect_ratio: 0.1,
            max_aspect_ratio: 10.0,
            min_radius_ratio: 0.1,
            density_factor: 1.0,
            quality_mesh: true,
            use_size_field: false,
            max_edge_length: 1.0,
            min_edge_length: 0.01,
            curvature_factor: 1.0,
            proximity_factor: 1.0,
            size_field: None,
        }
    }
}

/// 3D mesher
pub struct Mesher3D {
    /// Mesher parameters
    params: Mesher3DParams,
    /// Input vertices
    input_vertices: Vec<Point>,
    /// Input faces
    input_faces: Vec<Vec<usize>>,
    /// Input edges
    input_edges: Vec<(usize, usize)>,
    /// Curvature values for vertices
    vertex_curvatures: Vec<f64>,
    /// Size field values for vertices
    vertex_sizes: Vec<f64>,
}

impl Mesher3D {
    /// Create a new 3D mesher
    pub fn new(params: Mesher3DParams) -> Self {
        Self {
            params,
            input_vertices: Vec::new(),
            input_faces: Vec::new(),
            input_edges: Vec::new(),
            vertex_curvatures: Vec::new(),
            vertex_sizes: Vec::new(),
        }
    }

    /// Add input solid
    pub fn add_solid(&mut self, vertices: &[Point], faces: &[Vec<usize>]) {
        let start_idx = self.input_vertices.len();
        for vertex in vertices {
            self.input_vertices.push(vertex.clone());
            self.vertex_curvatures.push(0.0);
            self.vertex_sizes.push(self.params.max_edge_length);
        }

        for face in faces {
            let mut face_indices = Vec::new();
            for &idx in face {
                face_indices.push(start_idx + idx);
            }
            self.input_faces.push(face_indices);
        }

        // Extract edges from faces
        let mut edge_set = HashSet::new();
        for face in &self.input_faces {
            for i in 0..face.len() {
                let j = (i + 1) % face.len();
                let edge = if face[i] < face[j] {
                    (face[i], face[j])
                } else {
                    (face[j], face[i])
                };
                edge_set.insert(edge);
            }
        }

        for edge in edge_set {
            self.input_edges.push(edge);
        }
    }

    /// Generate mesh
    pub fn generate(&mut self) -> Result<Mesh3D, Mesher3DError> {
        if self.input_vertices.is_empty() {
            return Err(Mesher3DError::EmptyInput);
        }

        // Compute curvatures and size field
        self.compute_curvatures();
        self.compute_size_field();

        // Create initial mesh
        let mut mesh = self.create_initial_mesh()?;

        // Refine mesh
        if self.params.quality_mesh {
            self.refine_mesh(&mut mesh);
        }

        // Optimize mesh quality
        self.optimize_mesh(&mut mesh);

        // Fix inverted tetrahedrons
        self.fix_inverted_tetrahedrons(&mut mesh);

        Ok(mesh)
    }

    /// Fix inverted tetrahedrons
    fn fix_inverted_tetrahedrons(&self, mesh: &mut Mesh3D) {
        for tetra in &mut mesh.tetrahedrons {
            let v0 = &mesh.vertices[tetra.vertices[0]].point;
            let v1 = &mesh.vertices[tetra.vertices[1]].point;
            let v2 = &mesh.vertices[tetra.vertices[2]].point;
            let v3 = &mesh.vertices[tetra.vertices[3]].point;

            // Calculate tetrahedron volume
            let volume = self.calculate_tetrahedron_volume(v0, v1, v2, v3);

            // If volume is negative, tetrahedron is inverted
            if volume < 0.0 {
                // Reverse vertex order to fix inversion
                tetra.vertices.reverse();
            }
        }
    }

    /// Calculate tetrahedron volume
    fn calculate_tetrahedron_volume(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
        let v1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let v2 = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        let v3 = Vector::new(p4.x - p1.x, p4.y - p1.y, p4.z - p1.z);

        let cross = Vector::new(
            v2.y * v3.z - v2.z * v3.y,
            v2.z * v3.x - v2.x * v3.z,
            v2.x * v3.y - v2.y * v3.x,
        );

        (v1.x * cross.x + v1.y * cross.y + v1.z * cross.z) / 6.0
    }

    /// Compute vertex curvatures
    fn compute_curvatures(&mut self) {
        for i in 0..self.input_vertices.len() {
            // Find adjacent vertices
            let mut adjacent_vertices = Vec::new();
            for edge in &self.input_edges {
                if edge.0 == i {
                    adjacent_vertices.push(edge.1);
                } else if edge.1 == i {
                    adjacent_vertices.push(edge.0);
                }
            }

            if adjacent_vertices.len() >= 2 {
                // Calculate curvature as the average angle between consecutive edges
                let mut total_curvature = 0.0;
                let mut count = 0;

                for j in 0..adjacent_vertices.len() {
                    let prev = adjacent_vertices[j];
                    let next = adjacent_vertices[(j + 1) % adjacent_vertices.len()];

                    let p_prev = &self.input_vertices[prev];
                    let p_curr = &self.input_vertices[i];
                    let p_next = &self.input_vertices[next];

                    // Compute vectors
                    let v1 = [
                        p_prev.x - p_curr.x,
                        p_prev.y - p_curr.y,
                        p_prev.z - p_curr.z,
                    ];
                    let v2 = [
                        p_next.x - p_curr.x,
                        p_next.y - p_curr.y,
                        p_next.z - p_curr.z,
                    ];

                    // Compute dot product
                    let dot = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
                    let mag1 = (v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2]).sqrt();
                    let mag2 = (v2[0] * v2[0] + v2[1] * v2[1] + v2[2] * v2[2]).sqrt();

                    if mag1 > 1e-6 && mag2 > 1e-6 {
                        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
                        let angle = cos_angle.acos();
                        total_curvature += angle;
                        count += 1;
                    }
                }

                if count > 0 {
                    let avg_curvature = total_curvature / count as f64;
                    self.vertex_curvatures[i] =
                        avg_curvature / std::f64::consts::PI * self.params.curvature_factor;
                }
            }
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
                        let distance =
                            ((p.x - p_j.x).powi(2) + (p.y - p_j.y).powi(2) + (p.z - p_j.z).powi(2))
                                .sqrt();
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

    /// Create initial mesh
    fn create_initial_mesh(&self) -> Result<Mesh3D, Mesher3DError> {
        let mut mesh = Mesh3D::new();

        // Add input vertices to mesh
        let mut vertex_map = HashMap::new();
        for (idx, vertex) in self.input_vertices.iter().enumerate() {
            let mesh_vertex_id = mesh.add_vertex(vertex.clone());
            vertex_map.insert(idx, mesh_vertex_id);
        }

        // Simple tetrahedralization using Delaunay triangulation
        let tetrahedrons = self.delaunay_tetrahedralization()?;
        for tetra in tetrahedrons {
            let v0 = vertex_map[&tetra.0];
            let v1 = vertex_map[&tetra.1];
            let v2 = vertex_map[&tetra.2];
            let v3 = vertex_map[&tetra.3];
            mesh.add_tetrahedron(v0, v1, v2, v3);
        }

        Ok(mesh)
    }

    /// Delaunay tetrahedralization
    fn delaunay_tetrahedralization(
        &self,
    ) -> Result<Vec<(usize, usize, usize, usize)>, Mesher3DError> {
        if self.input_vertices.len() < 4 {
            return Err(Mesher3DError::InvalidGeometry);
        }

        // Complete implementation: Delaunay tetrahedralization
        // Using Bowyer-Watson algorithm for 3D Delaunay triangulation
        // 1. Build a super tetrahedron that encloses all points
        // 2. Insert points one by one, remove tetrahedrons containing the new point, generate new tetrahedrons
        // 3. Remove tetrahedrons related to the super tetrahedron
        use crate::geometry::Point;
        let points = &self.input_vertices;

        // Step 1: Construct super tetrahedron
        let mut min = points[0];
        let mut max = points[0];
        for p in points.iter() {
            min = Point::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z));
            max = Point::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z));
        }
        let dx = max.x - min.x;
        let dy = max.y - min.y;
        let dz = max.z - min.z;
        let offset = dx.max(dy).max(dz) * 10.0;
        let super_pts = [
            Point::new(min.x - offset, min.y - offset, min.z - offset),
            Point::new(max.x + offset, min.y - offset, min.z - offset),
            Point::new(min.x - offset, max.y + offset, min.z - offset),
            Point::new(min.x - offset, min.y - offset, max.z + offset),
        ];
        let mut mesh = vec![(
            points.len(),
            points.len() + 1,
            points.len() + 2,
            points.len() + 3,
        )];

        // Step 2: Insert points one by one
        for (idx, p) in points.iter().enumerate() {
            let mut bad_tetra = Vec::new();
            for (i, tet) in mesh.iter().enumerate() {
                // Check if the new point is inside the circumsphere of the tetrahedron
                let verts = [
                    if tet.0 < points.len() {
                        &points[tet.0]
                    } else {
                        &super_pts[tet.0 - points.len()]
                    },
                    if tet.1 < points.len() {
                        &points[tet.1]
                    } else {
                        &super_pts[tet.1 - points.len()]
                    },
                    if tet.2 < points.len() {
                        &points[tet.2]
                    } else {
                        &super_pts[tet.2 - points.len()]
                    },
                    if tet.3 < points.len() {
                        &points[tet.3]
                    } else {
                        &super_pts[tet.3 - points.len()]
                    },
                ];
                if self.point_in_circumsphere(p, verts[0], verts[1], verts[2], verts[3]) {
                    bad_tetra.push(i);
                }
            }
            // Remove bad tetrahedrons
            let mut new_faces = Vec::new();
            for &i in bad_tetra.iter().rev() {
                let tet = mesh.remove(i);
                // Record faces of the tetrahedron
                new_faces.push((tet.0, tet.1, tet.2));
                new_faces.push((tet.0, tet.1, tet.3));
                new_faces.push((tet.0, tet.2, tet.3));
                new_faces.push((tet.1, tet.2, tet.3));
            }
            // Remove duplicate faces
            let mut face_count = std::collections::HashMap::new();
            for f in new_faces.iter() {
                let mut sorted = [f.0, f.1, f.2];
                sorted.sort();
                *face_count.entry(sorted).or_insert(0) += 1;
            }
            let boundary_faces: Vec<_> = face_count
                .iter()
                .filter(|(_, &v)| v == 1)
                .map(|(f, _)| *f)
                .collect();
            // Create new tetrahedrons
            for f in boundary_faces {
                mesh.push((f[0], f[1], f[2], idx));
            }
        }

        // Step 3: Remove tetrahedrons related to the super tetrahedron
        let tetrahedrons = mesh
            .into_iter()
            .filter(|tet| {
                tet.0 < points.len()
                    && tet.1 < points.len()
                    && tet.2 < points.len()
                    && tet.3 < points.len()
            })
            .collect();

        Ok(tetrahedrons)
    }

    /// Calculate bounding box
    #[allow(dead_code)]
    fn calculate_bbox(&self) -> (Point, Point) {
        if self.input_vertices.is_empty() {
            return (Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 0.0));
        }

        let mut min_point = self.input_vertices[0].clone();
        let mut max_point = self.input_vertices[0].clone();

        for vertex in &self.input_vertices {
            min_point.x = min_point.x.min(vertex.x);
            min_point.y = min_point.y.min(vertex.y);
            min_point.z = min_point.z.min(vertex.z);
            max_point.x = max_point.x.max(vertex.x);
            max_point.y = max_point.y.max(vertex.y);
            max_point.z = max_point.z.max(vertex.z);
        }

        (min_point, max_point)
    }

    /// Check if a point is inside the circumsphere of a tetrahedron
    #[allow(dead_code)]
    fn point_in_circumsphere(
        &self,
        p: &Point,
        v0: &Point,
        v1: &Point,
        v2: &Point,
        v3: &Point,
    ) -> bool {
        let matrix = [
            v0.x - p.x,
            v0.y - p.y,
            v0.z - p.z,
            (v0.x * v0.x + v0.y * v0.y + v0.z * v0.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            v1.x - p.x,
            v1.y - p.y,
            v1.z - p.z,
            (v1.x * v1.x + v1.y * v1.y + v1.z * v1.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            v2.x - p.x,
            v2.y - p.y,
            v2.z - p.z,
            (v2.x * v2.x + v2.y * v2.y + v2.z * v2.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
            v3.x - p.x,
            v3.y - p.y,
            v3.z - p.z,
            (v3.x * v3.x + v3.y * v3.y + v3.z * v3.z) - (p.x * p.x + p.y * p.y + p.z * p.z),
        ];

        let det = self.determinant_4x4(&matrix);
        det > 0.0
    }

    /// Calculate determinant of 4x4 matrix
    #[allow(dead_code)]
    fn determinant_4x4(&self, m: &[f64; 16]) -> f64 {
        m[0] * (m[5] * (m[10] * m[15] - m[11] * m[14])
            + m[6] * (m[11] * m[13] - m[9] * m[15])
            + m[7] * (m[9] * m[14] - m[10] * m[13]))
            - m[1]
                * (m[4] * (m[10] * m[15] - m[11] * m[14])
                    + m[6] * (m[11] * m[12] - m[8] * m[15])
                    + m[7] * (m[8] * m[14] - m[10] * m[12]))
            + m[2]
                * (m[4] * (m[9] * m[15] - m[11] * m[13])
                    + m[5] * (m[11] * m[12] - m[8] * m[15])
                    + m[7] * (m[8] * m[13] - m[9] * m[12]))
            - m[3]
                * (m[4] * (m[9] * m[14] - m[10] * m[13])
                    + m[5] * (m[10] * m[12] - m[8] * m[14])
                    + m[6] * (m[8] * m[13] - m[9] * m[12]))
    }

    /// Sort face vertices to ensure consistent representation
    #[allow(dead_code)]
    fn sort_face(&self, face: (usize, usize, usize)) -> (usize, usize, usize) {
        let mut vertices = vec![face.0, face.1, face.2];
        vertices.sort();
        (vertices[0], vertices[1], vertices[2])
    }

    /// Check if a tetrahedron contains a face
    #[allow(dead_code)]
    fn tetra_contains_face(
        &self,
        tetra: &(usize, usize, usize, usize),
        face: &(usize, usize, usize),
    ) -> bool {
        let tetra_vertices = vec![tetra.0, tetra.1, tetra.2, tetra.3];
        let face_vertices = vec![face.0, face.1, face.2];
        face_vertices.iter().all(|v| tetra_vertices.contains(v))
    }

    /// Refine mesh
    fn refine_mesh(&self, mesh: &mut Mesh3D) {
        let mut edges_to_split = Vec::new();

        // Identify edges that need splitting
        for (edge_id, edge) in mesh.edges.iter().enumerate() {
            let v0 = &mesh.vertices[edge.vertices[0]];
            let v1 = &mesh.vertices[edge.vertices[1]];
            let length = ((v1.point.x - v0.point.x).powi(2)
                + (v1.point.y - v0.point.y).powi(2)
                + (v1.point.z - v0.point.z).powi(2))
            .sqrt();

            // Determine appropriate edge length based on size field
            let mut max_edge_length = self.params.max_edge_length;

            // Check if vertices are in the input set
            for (i, input_vertex) in self.input_vertices.iter().enumerate() {
                if (input_vertex.x - v0.point.x).abs() < 1e-6
                    && (input_vertex.y - v0.point.y).abs() < 1e-6
                    && (input_vertex.z - v0.point.z).abs() < 1e-6
                {
                    max_edge_length = max_edge_length.min(self.vertex_sizes[i]);
                }
                if (input_vertex.x - v1.point.x).abs() < 1e-6
                    && (input_vertex.y - v1.point.y).abs() < 1e-6
                    && (input_vertex.z - v1.point.z).abs() < 1e-6
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

    /// Split an edge
    fn split_edge(&self, mesh: &mut Mesh3D, edge_id: usize) {
        let edge = mesh.edges[edge_id].clone();
        let v0 = mesh.vertices[edge.vertices[0]].clone();
        let v1 = mesh.vertices[edge.vertices[1]].clone();

        // Create new vertex at midpoint
        let midpoint = Point::new(
            (v0.point.x + v1.point.x) / 2.0,
            (v0.point.y + v1.point.y) / 2.0,
            (v0.point.z + v1.point.z) / 2.0,
        );
        let new_vertex_id = mesh.add_vertex(midpoint);

        // Replace edge with two new edges
        mesh.edges[edge_id].vertices[1] = new_vertex_id;
        mesh.add_edge(new_vertex_id, edge.vertices[1]);
    }

    /// Optimize mesh quality
    fn optimize_mesh(&self, mesh: &mut Mesh3D) {
        // Simple optimization: swap edges to improve tetrahedron quality
        let mut improved = true;
        while improved {
            improved = false;
            for tetra_id in 0..mesh.tetrahedrons.len() {
                if self.optimize_tetrahedron(mesh, tetra_id) {
                    improved = true;
                }
            }
        }
    }

    /// Optimize a single tetrahedron
    fn optimize_tetrahedron(&self, mesh: &mut Mesh3D, tetra_id: usize) -> bool {
        let tetra = &mesh.tetrahedrons[tetra_id];

        // Calculate current quality
        let current_quality = self.calculate_tetrahedron_quality(mesh, tetra);

        // Try all possible edge swaps
        let edges = vec![
            (
                tetra.vertices[0],
                tetra.vertices[1],
                tetra.vertices[2],
                tetra.vertices[3],
            ),
            (
                tetra.vertices[0],
                tetra.vertices[2],
                tetra.vertices[1],
                tetra.vertices[3],
            ),
            (
                tetra.vertices[0],
                tetra.vertices[3],
                tetra.vertices[1],
                tetra.vertices[2],
            ),
        ];

        for (v0, v1, v2, v3) in edges {
            // Find adjacent tetrahedron sharing edge (v0, v1)
            let adjacent_tetra = self.find_adjacent_tetrahedron(mesh, v0, v1, tetra_id);
            if let Some(adj_tetra_id) = adjacent_tetra {
                let adj_tetra = &mesh.tetrahedrons[adj_tetra_id];

                // Get the fourth vertex of the adjacent tetrahedron
                let mut fourth_vertex = 0;
                for &v in &adj_tetra.vertices {
                    if v != v0 && v != v1 {
                        fourth_vertex = v;
                        break;
                    }
                }

                // Create new tetrahedrons by swapping edges
                let new_tetra1 = [v0, v2, v3, fourth_vertex];
                let new_tetra2 = [v1, v2, v3, fourth_vertex];

                // Calculate new quality
                let new_quality1 =
                    self.calculate_tetrahedron_quality_with_vertices(mesh, &new_tetra1);
                let new_quality2 =
                    self.calculate_tetrahedron_quality_with_vertices(mesh, &new_tetra2);
                let new_quality = (new_quality1 + new_quality2) / 2.0;

                if new_quality > current_quality {
                    // Perform edge swap
                    mesh.tetrahedrons[tetra_id].vertices = new_tetra1;
                    mesh.tetrahedrons[adj_tetra_id].vertices = new_tetra2;
                    return true;
                }
            }
        }

        false
    }

    /// Find adjacent tetrahedron sharing an edge
    fn find_adjacent_tetrahedron(
        &self,
        mesh: &Mesh3D,
        v1: usize,
        v2: usize,
        exclude_tetra: usize,
    ) -> Option<usize> {
        for (tetra_id, tetra) in mesh.tetrahedrons.iter().enumerate() {
            if tetra_id == exclude_tetra {
                continue;
            }

            let has_v1 = tetra.vertices.contains(&v1);
            let has_v2 = tetra.vertices.contains(&v2);
            if has_v1 && has_v2 {
                return Some(tetra_id);
            }
        }
        None
    }

    /// Calculate tetrahedron quality from vertex indices
    fn calculate_tetrahedron_quality_with_vertices(
        &self,
        mesh: &Mesh3D,
        vertices: &[usize; 4],
    ) -> f64 {
        let v0 = &mesh.vertices[vertices[0]];
        let v1 = &mesh.vertices[vertices[1]];
        let v2 = &mesh.vertices[vertices[2]];
        let v3 = &mesh.vertices[vertices[3]];

        // Calculate dihedral angles
        let angles = vec![
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v0.point, &v1.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v0.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v1.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v3.point, &v0.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v3.point, &v1.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v2.point, &v3.point, &v1.point, &v2.point, &v3.point,
            ),
        ];

        // Calculate aspect ratio
        let aspect_ratio =
            self.calculate_tetrahedron_aspect_ratio(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate radius ratio
        let radius_ratio =
            self.calculate_tetrahedron_radius_ratio(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate volume
        let volume = self.calculate_tetrahedron_volume(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate quality score
        let angle_score = angles.iter().fold(1.0, |score, &angle| {
            if angle < self.params.min_dihedral_angle || angle > self.params.max_dihedral_angle {
                score * 0.5
            } else {
                score
            }
        });

        let aspect_score = if aspect_ratio < self.params.min_aspect_ratio
            || aspect_ratio > self.params.max_aspect_ratio
        {
            0.5
        } else {
            1.0
        };

        let radius_score = if radius_ratio < self.params.min_radius_ratio {
            0.5
        } else {
            1.0
        };

        let volume_score = if volume > self.params.max_volume {
            0.5
        } else {
            1.0
        };

        angle_score * 0.4 + aspect_score * 0.2 + radius_score * 0.2 + volume_score * 0.2
    }

    /// Calculate tetrahedron quality
    fn calculate_tetrahedron_quality(
        &self,
        mesh: &Mesh3D,
        tetra: &crate::mesh::mesh_data::MeshTetrahedron,
    ) -> f64 {
        let v0 = &mesh.vertices[tetra.vertices[0]];
        let v1 = &mesh.vertices[tetra.vertices[1]];
        let v2 = &mesh.vertices[tetra.vertices[2]];
        let v3 = &mesh.vertices[tetra.vertices[3]];

        // Calculate dihedral angles
        let angles = vec![
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v0.point, &v1.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v0.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v2.point, &v1.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v3.point, &v0.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v1.point, &v3.point, &v1.point, &v2.point, &v3.point,
            ),
            self.calculate_dihedral_angle(
                &v0.point, &v2.point, &v3.point, &v1.point, &v2.point, &v3.point,
            ),
        ];

        // Calculate aspect ratio
        let aspect_ratio =
            self.calculate_tetrahedron_aspect_ratio(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate radius ratio
        let radius_ratio =
            self.calculate_tetrahedron_radius_ratio(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate volume
        let volume = self.calculate_tetrahedron_volume(&v0.point, &v1.point, &v2.point, &v3.point);

        // Calculate quality score
        let angle_score = angles.iter().fold(1.0, |score, &angle| {
            if angle < self.params.min_dihedral_angle || angle > self.params.max_dihedral_angle {
                score * 0.5
            } else {
                score
            }
        });

        let aspect_score = if aspect_ratio < self.params.min_aspect_ratio
            || aspect_ratio > self.params.max_aspect_ratio
        {
            0.5
        } else {
            1.0
        };

        let radius_score = if radius_ratio < self.params.min_radius_ratio {
            0.5
        } else {
            1.0
        };

        let volume_score = if volume > self.params.max_volume {
            0.5
        } else {
            1.0
        };

        angle_score * 0.4 + aspect_score * 0.2 + radius_score * 0.2 + volume_score * 0.2
    }

    /// Calculate dihedral angle between two planes
    fn calculate_dihedral_angle(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
        p5: &Point,
        p6: &Point,
    ) -> f64 {
        let v1 = [p2.x - p1.x, p2.y - p1.y, p2.z - p1.z];
        let v2 = [p3.x - p1.x, p3.y - p1.y, p3.z - p1.z];
        let normal1 = [
            v1[1] * v2[2] - v1[2] * v2[1],
            v1[2] * v2[0] - v1[0] * v2[2],
            v1[0] * v2[1] - v1[1] * v2[0],
        ];

        let v3 = [p5.x - p4.x, p5.y - p4.y, p5.z - p4.z];
        let v4 = [p6.x - p4.x, p6.y - p4.y, p6.z - p4.z];
        let normal2 = [
            v3[1] * v4[2] - v3[2] * v4[1],
            v3[2] * v4[0] - v3[0] * v4[2],
            v3[0] * v4[1] - v3[1] * v4[0],
        ];

        let dot = normal1[0] * normal2[0] + normal1[1] * normal2[1] + normal1[2] * normal2[2];
        let mag1 =
            (normal1[0] * normal1[0] + normal1[1] * normal1[1] + normal1[2] * normal1[2]).sqrt();
        let mag2 =
            (normal2[0] * normal2[0] + normal2[1] * normal2[1] + normal2[2] * normal2[2]).sqrt();

        if mag1 < 1e-6 || mag2 < 1e-6 {
            return 0.0;
        }

        let cos_angle = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        cos_angle.acos() * 180.0 / std::f64::consts::PI
    }

    /// Calculate tetrahedron aspect ratio
    fn calculate_tetrahedron_aspect_ratio(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> f64 {
        let edges = vec![
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2) + (p3.z - p1.z).powi(2)).sqrt(),
            ((p4.x - p1.x).powi(2) + (p4.y - p1.y).powi(2) + (p4.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p2.x).powi(2) + (p4.y - p2.y).powi(2) + (p4.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p3.x).powi(2) + (p4.y - p3.y).powi(2) + (p4.z - p3.z).powi(2)).sqrt(),
        ];

        let max_edge = edges.iter().fold(0.0_f64, |acc, &e| acc.max(e));
        let min_edge = edges.iter().fold(f64::MAX, |acc, &e| acc.min(e));

        if min_edge < 1e-6 {
            return 10.0;
        }

        max_edge / min_edge
    }

    /// Calculate tetrahedron radius ratio
    fn calculate_tetrahedron_radius_ratio(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> f64 {
        let volume = self.calculate_tetrahedron_volume(p1, p2, p3, p4);
        let edges = vec![
            ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2) + (p2.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p1.x).powi(2) + (p3.y - p1.y).powi(2) + (p3.z - p1.z).powi(2)).sqrt(),
            ((p4.x - p1.x).powi(2) + (p4.y - p1.y).powi(2) + (p4.z - p1.z).powi(2)).sqrt(),
            ((p3.x - p2.x).powi(2) + (p3.y - p2.y).powi(2) + (p3.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p2.x).powi(2) + (p4.y - p2.y).powi(2) + (p4.z - p2.z).powi(2)).sqrt(),
            ((p4.x - p3.x).powi(2) + (p4.y - p3.y).powi(2) + (p4.z - p3.z).powi(2)).sqrt(),
        ];

        let sum_edges = edges.iter().sum::<f64>();
        let product_edges = edges.iter().product::<f64>();

        if sum_edges < 1e-6 || product_edges < 1e-6 {
            return 0.0;
        }

        let radius_ratio = (2.0 * volume) / (sum_edges * product_edges).powf(1.0 / 3.0);
        radius_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesher3d_creation() {
        let params = Mesher3DParams::default();
        let mesher = Mesher3D::new(params);
        assert!(mesher.input_vertices.is_empty());
        assert!(mesher.input_faces.is_empty());
        assert!(mesher.input_edges.is_empty());
    }

    #[test]
    fn test_add_solid() {
        let mut mesher = Mesher3D::new(Mesher3DParams::default());
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.5, 0.5, 1.0),
        ];
        let faces = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 4],
            vec![1, 2, 4],
            vec![2, 3, 4],
            vec![3, 0, 4],
        ];
        mesher.add_solid(&vertices, &faces);
        assert_eq!(mesher.input_vertices.len(), 5);
        assert_eq!(mesher.input_faces.len(), 5);
    }

    #[test]
    fn test_calculate_bbox() {
        let mut mesher = Mesher3D::new(Mesher3DParams::default());
        let vertices = vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 2.0, 3.0)];
        let faces = vec![vec![0, 1]];
        mesher.add_solid(&vertices, &faces);
        let (min, max) = mesher.calculate_bbox();
        assert_eq!(min.x, 0.0);
        assert_eq!(min.y, 0.0);
        assert_eq!(min.z, 0.0);
        assert_eq!(max.x, 1.0);
        assert_eq!(max.y, 2.0);
        assert_eq!(max.z, 3.0);
    }

    #[test]
    fn test_sort_face() {
        let mesher = Mesher3D::new(Mesher3DParams::default());
        let face = (2, 0, 1);
        let sorted = mesher.sort_face(face);
        assert_eq!(sorted, (0, 1, 2));
    }

    #[test]
    fn test_generate_mesh() {
        let mut mesher = Mesher3D::new(Mesher3DParams::default());
        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
            Point::new(0.5, 0.5, 1.0),
        ];
        let faces = vec![
            vec![0, 1, 2, 3],
            vec![0, 1, 4],
            vec![1, 2, 4],
            vec![2, 3, 4],
            vec![3, 0, 4],
        ];
        mesher.add_solid(&vertices, &faces);
        let mesh = mesher.generate().unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.tetrahedrons.is_empty());
    }
}
