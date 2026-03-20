//! Adaptive mesh refinement/coarsening (h-adaptivity)
//!
//! This module provides functionality for adaptive mesh refinement and coarsening
//! based on user-defined criteria such as error estimation, curvature, or solution gradients.

use super::mesh_data::{Mesh2D, Mesh3D};
use crate::geometry::{Point, Vector};
use std::collections::{HashMap, HashSet, VecDeque};

/// Adaptive mesher error types
#[derive(Debug)]
pub enum AdaptiveMesherError {
    /// Invalid input mesh
    InvalidInputMesh,
    /// Adaptation failed
    AdaptationFailed,
    /// No elements to adapt
    NoElementsToAdapt,
    /// Invalid parameters
    InvalidParameters,
}

/// Adaptive mesher parameters
#[derive(Debug, Clone)]
pub struct AdaptiveMesherParams {
    /// Refinement threshold
    pub refinement_threshold: f64,
    /// Coarsening threshold
    pub coarsening_threshold: f64,
    /// Maximum refinement level
    pub max_refinement_level: usize,
    /// Minimum element size
    pub min_element_size: f64,
    /// Maximum element size
    pub max_element_size: f64,
    /// Use curvature-based refinement
    pub use_curvature_refinement: bool,
    /// Use proximity-based refinement
    pub use_proximity_refinement: bool,
    /// Curvature threshold
    pub curvature_threshold: f64,
    /// Proximity threshold
    pub proximity_threshold: f64,
}

impl Default for AdaptiveMesherParams {
    fn default() -> Self {
        Self {
            refinement_threshold: 0.5,
            coarsening_threshold: 0.1,
            max_refinement_level: 4,
            min_element_size: 0.01,
            max_element_size: 1.0,
            use_curvature_refinement: true,
            use_proximity_refinement: true,
            curvature_threshold: 0.1,
            proximity_threshold: 0.1,
        }
    }
}

/// Adaptive mesher
pub struct AdaptiveMesher {
    /// Mesher parameters
    params: AdaptiveMesherParams,
    /// Element quality map
    element_quality: HashMap<usize, f64>,
    /// Element refinement levels
    element_levels: HashMap<usize, usize>,
}

impl AdaptiveMesher {
    /// Create a new adaptive mesher
    pub fn new(params: AdaptiveMesherParams) -> Self {
        Self {
            params,
            element_quality: HashMap::new(),
            element_levels: HashMap::new(),
        }
    }

    /// Adapt 2D mesh
    pub fn adapt_mesh_2d(&mut self, mesh: &mut Mesh2D) -> Result<(), AdaptiveMesherError> {
        if mesh.faces.is_empty() {
            return Err(AdaptiveMesherError::NoElementsToAdapt);
        }

        // Calculate element qualities
        self.calculate_element_qualities_2d(mesh);

        // Mark elements for refinement/coarsening
        let (to_refine, to_coarsen) = self.mark_elements_2d(mesh);

        // Refine elements
        self.refine_elements_2d(mesh, &to_refine);

        // Coarsen elements
        self.coarsen_elements_2d(mesh, &to_coarsen);

        Ok(())
    }

    /// Adapt 3D mesh
    pub fn adapt_mesh_3d(&mut self, mesh: &mut Mesh3D) -> Result<(), AdaptiveMesherError> {
        if mesh.tetrahedrons.is_empty() && mesh.hexahedrons.is_empty() {
            return Err(AdaptiveMesherError::NoElementsToAdapt);
        }

        // Calculate element qualities
        self.calculate_element_qualities_3d(mesh);

        // Mark elements for refinement/coarsening
        let (to_refine, to_coarsen) = self.mark_elements_3d(mesh);

        // Refine elements
        self.refine_elements_3d(mesh, &to_refine);

        // Coarsen elements
        self.coarsen_elements_3d(mesh, &to_coarsen);

        Ok(())
    }

    /// Calculate element qualities for 2D mesh
    fn calculate_element_qualities_2d(&mut self, mesh: &Mesh2D) {
        self.element_quality.clear();
        self.element_levels.clear();

        for (face_id, face) in mesh.faces.iter().enumerate() {
            let quality = self.calculate_face_quality_2d(mesh, face);
            self.element_quality.insert(face_id, quality);
            self.element_levels.insert(face_id, 0);
        }
    }

    /// Calculate face quality for 2D mesh
    fn calculate_face_quality_2d(
        &self,
        mesh: &Mesh2D,
        face: &crate::mesh::mesh_data::MeshFace,
    ) -> f64 {
        if face.vertices.len() < 3 {
            return 0.0;
        }

        let v0 = &mesh.vertices[face.vertices[0]].point;
        let v1 = &mesh.vertices[face.vertices[1]].point;
        let v2 = &mesh.vertices[face.vertices[2]].point;

        // Calculate area
        let area = 0.5 * ((v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x)).abs();

        // Calculate edge lengths
        let edges = vec![
            ((v1.x - v0.x).powi(2) + (v1.y - v0.y).powi(2)).sqrt(),
            ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2)).sqrt(),
            ((v0.x - v2.x).powi(2) + (v0.y - v2.y).powi(2)).sqrt(),
        ];

        let max_edge = edges.iter().fold(0.0, |max, &e| max.max(e));
        let min_edge = edges.iter().fold(f64::MAX, |min, &e| min.min(e));

        // Calculate aspect ratio
        let aspect_ratio = if min_edge > 0.0 {
            max_edge / min_edge
        } else {
            10.0
        };

        // Calculate quality score
        let area_score = if area > 0.0 { 1.0 / area.sqrt() } else { 0.0 };

        let aspect_score = if aspect_ratio < 5.0 {
            1.0
        } else {
            5.0 / aspect_ratio
        };

        0.5 * area_score + 0.5 * aspect_score
    }

    /// Mark elements for refinement/coarsening for 2D mesh
    fn mark_elements_2d(&self, mesh: &Mesh2D) -> (HashSet<usize>, HashSet<usize>) {
        let mut to_refine = HashSet::new();
        let mut to_coarsen = HashSet::new();

        for (face_id, &quality) in &self.element_quality {
            let level = self.element_levels.get(face_id).unwrap_or(&0);

            if quality > self.params.refinement_threshold
                && *level < self.params.max_refinement_level
            {
                to_refine.insert(*face_id);
            } else if quality < self.params.coarsening_threshold && *level > 0 {
                to_coarsen.insert(*face_id);
            }
        }

        (to_refine, to_coarsen)
    }

    /// Refine elements for 2D mesh
    fn refine_elements_2d(&self, mesh: &mut Mesh2D, to_refine: &HashSet<usize>) {
        let mut edges_to_split = HashSet::new();

        // Collect edges to split
        for &face_id in to_refine {
            let face = &mesh.faces[face_id];
            for i in 0..face.vertices.len() {
                let v0 = face.vertices[i];
                let v1 = face.vertices[(i + 1) % face.vertices.len()];
                let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                edges_to_split.insert(edge);
            }
        }

        // Split edges
        for edge in edges_to_split {
            self.split_edge_2d(mesh, edge.0, edge.1);
        }
    }

    /// Split edge for 2D mesh
    fn split_edge_2d(&self, mesh: &mut Mesh2D, v0: usize, v1: usize) {
        let p0 = &mesh.vertices[v0].point;
        let p1 = &mesh.vertices[v1].point;
        let midpoint = Point::new((p0.x + p1.x) / 2.0, (p0.y + p1.y) / 2.0, 0.0);
        let new_vertex_id = mesh.add_vertex(midpoint);

        // Replace edge with two new edges
        let mut edges_to_remove = Vec::new();
        for (edge_id, edge) in mesh.edges.iter().enumerate() {
            if (edge.vertices[0] == v0 && edge.vertices[1] == v1)
                || (edge.vertices[0] == v1 && edge.vertices[1] == v0)
            {
                // Mark old edge for removal
                edges_to_remove.push(edge_id);
            }
        }

        // Remove old edges
        for &edge_id in edges_to_remove.iter().rev() {
            mesh.edges.remove(edge_id);
        }

        mesh.add_edge(v0, new_vertex_id);
        mesh.add_edge(new_vertex_id, v1);

        // Update faces that use this edge
        for face in &mut mesh.faces {
            let mut vertices = face.vertices.clone();
            for i in 0..vertices.len() {
                if (vertices[i] == v0 && vertices[(i + 1) % vertices.len()] == v1)
                    || (vertices[i] == v1 && vertices[(i + 1) % vertices.len()] == v0)
                {
                    // Split the face
                    let mut new_face1 = face.clone();
                    let mut new_face2 = face.clone();

                    // Replace edge with new vertex
                    new_face1.vertices = vec![
                        vertices[i],
                        new_vertex_id,
                        vertices[(i + 1) % vertices.len()],
                    ];
                    new_face2.vertices = vec![
                        vertices[i],
                        vertices[(i + 1) % vertices.len()],
                        new_vertex_id,
                    ];

                    // Replace original face
                    *face = new_face1;
                    // Add new face
                    mesh.faces.push(new_face2);
                    break;
                }
            }
        }
    }

    /// Coarsen elements for 2D mesh
    fn coarsen_elements_2d(&self, mesh: &mut Mesh2D, to_coarsen: &HashSet<usize>) {
        if to_coarsen.is_empty() {
            return;
        }

        // Build vertex-to-face adjacency
        let mut vertex_faces: HashMap<usize, Vec<usize>> = HashMap::new();
        for (face_id, face) in mesh.faces.iter().enumerate() {
            for &v in &face.vertices {
                vertex_faces.entry(v).or_default().push(face_id);
            }
        }

        // Find edges that can be collapsed
        let mut edges_to_collapse: Vec<(usize, usize)> = Vec::new();
        let mut processed_faces: HashSet<usize> = HashSet::new();

        for &face_id in to_coarsen {
            if processed_faces.contains(&face_id) {
                continue;
            }

            let face = &mesh.faces[face_id];
            if face.vertices.len() < 3 {
                continue;
            }

            // Find the shortest edge
            let mut shortest_edge = (0, 1);
            let mut shortest_length = f64::MAX;

            for i in 0..face.vertices.len() {
                let v0 = face.vertices[i];
                let v1 = face.vertices[(i + 1) % face.vertices.len()];
                let p0 = &mesh.vertices[v0].point;
                let p1 = &mesh.vertices[v1].point;
                let length = p0.distance(p1);

                if length < shortest_length {
                    shortest_length = length;
                    shortest_edge = (v0, v1);
                }
            }

            // Check if collapse is valid
            let (v0, v1) = shortest_edge;
            let faces_0 = vertex_faces.get(&v0).map(|v| v.as_slice()).unwrap_or(&[]);
            let faces_1 = vertex_faces.get(&v1).map(|v| v.as_slice()).unwrap_or(&[]);

            // Only collapse if both vertices are in faces marked for coarsening
            let can_collapse = faces_0.iter().all(|f| to_coarsen.contains(f))
                && faces_1.iter().all(|f| to_coarsen.contains(f));

            if can_collapse {
                edges_to_collapse.push((v0, v1));
                for &f in faces_0.iter().chain(faces_1.iter()) {
                    processed_faces.insert(f);
                }
            }
        }

        // Collapse edges
        for (v0, v1) in edges_to_collapse {
            // Calculate midpoint
            let p0 = &mesh.vertices[v0].point;
            let p1 = &mesh.vertices[v1].point;
            let midpoint = Point::new(
                (p0.x + p1.x) / 2.0,
                (p0.y + p1.y) / 2.0,
                (p0.z + p1.z) / 2.0,
            );

            // Update vertex position
            mesh.vertices[v0].point = midpoint;

            // Update all faces that reference v1 to use v0 instead
            for face in &mut mesh.faces {
                for v in &mut face.vertices {
                    if *v == v1 {
                        *v = v0;
                    }
                }
            }

            // Remove degenerate faces (faces with duplicate vertices)
            mesh.faces.retain(|face| {
                let mut seen = HashSet::new();
                face.vertices.iter().all(|v| seen.insert(*v))
            });
        }
    }

    /// Calculate element qualities for 3D mesh
    fn calculate_element_qualities_3d(&mut self, mesh: &Mesh3D) {
        self.element_quality.clear();
        self.element_levels.clear();

        // Calculate tetrahedron qualities
        for (tetra_id, tetra) in mesh.tetrahedrons.iter().enumerate() {
            let quality = self.calculate_tetrahedron_quality_3d(mesh, tetra);
            self.element_quality.insert(tetra_id, quality);
            self.element_levels.insert(tetra_id, 0);
        }

        // Calculate hexahedron qualities
        for (hex_id, hex) in mesh.hexahedrons.iter().enumerate() {
            let quality = self.calculate_hexahedron_quality_3d(mesh, hex);
            self.element_quality
                .insert(hex_id + mesh.tetrahedrons.len(), quality);
            self.element_levels
                .insert(hex_id + mesh.tetrahedrons.len(), 0);
        }
    }

    /// Calculate tetrahedron quality for 3D mesh
    fn calculate_tetrahedron_quality_3d(
        &self,
        mesh: &Mesh3D,
        tetra: &crate::mesh::mesh_data::MeshTetrahedron,
    ) -> f64 {
        let v0 = &mesh.vertices[tetra.vertices[0]].point;
        let v1 = &mesh.vertices[tetra.vertices[1]].point;
        let v2 = &mesh.vertices[tetra.vertices[2]].point;
        let v3 = &mesh.vertices[tetra.vertices[3]].point;

        // Calculate volume
        let volume = self.calculate_tetrahedron_volume(v0, v1, v2, v3);

        // Calculate edge lengths
        let edges = vec![
            ((v1.x - v0.x).powi(2) + (v1.y - v0.y).powi(2) + (v1.z - v0.z).powi(2)).sqrt(),
            ((v2.x - v0.x).powi(2) + (v2.y - v0.y).powi(2) + (v2.z - v0.z).powi(2)).sqrt(),
            ((v3.x - v0.x).powi(2) + (v3.y - v0.y).powi(2) + (v3.z - v0.z).powi(2)).sqrt(),
            ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.z - v1.z).powi(2)).sqrt(),
            ((v3.x - v1.x).powi(2) + (v3.y - v1.y).powi(2) + (v3.z - v1.z).powi(2)).sqrt(),
            ((v3.x - v2.x).powi(2) + (v3.y - v2.y).powi(2) + (v3.z - v2.z).powi(2)).sqrt(),
        ];

        let max_edge = edges.iter().fold(0.0, |max, &e| max.max(e));
        let min_edge = edges.iter().fold(f64::MAX, |min, &e| min.min(e));

        // Calculate aspect ratio
        let aspect_ratio = if min_edge > 0.0 {
            max_edge / min_edge
        } else {
            10.0
        };

        // Calculate quality score
        let volume_score = if volume > 0.0 {
            1.0 / volume.cbrt()
        } else {
            0.0
        };

        let aspect_score = if aspect_ratio < 5.0 {
            1.0
        } else {
            5.0 / aspect_ratio
        };

        0.5 * volume_score + 0.5 * aspect_score
    }

    /// Calculate hexahedron quality for 3D mesh
    fn calculate_hexahedron_quality_3d(
        &self,
        mesh: &Mesh3D,
        hex: &crate::mesh::mesh_data::MeshHexahedron,
    ) -> f64 {
        let v0 = &mesh.vertices[hex.vertices[0]].point;
        let v1 = &mesh.vertices[hex.vertices[1]].point;
        let v2 = &mesh.vertices[hex.vertices[2]].point;
        let v3 = &mesh.vertices[hex.vertices[3]].point;
        let v4 = &mesh.vertices[hex.vertices[4]].point;
        let v5 = &mesh.vertices[hex.vertices[5]].point;
        let v6 = &mesh.vertices[hex.vertices[6]].point;
        let v7 = &mesh.vertices[hex.vertices[7]].point;

        // Calculate edge lengths
        let edges = vec![
            ((v1.x - v0.x).powi(2) + (v1.y - v0.y).powi(2) + (v1.z - v0.z).powi(2)).sqrt(),
            ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.z - v1.z).powi(2)).sqrt(),
            ((v3.x - v2.x).powi(2) + (v3.y - v2.y).powi(2) + (v3.z - v2.z).powi(2)).sqrt(),
            ((v0.x - v3.x).powi(2) + (v0.y - v3.y).powi(2) + (v0.z - v3.z).powi(2)).sqrt(),
            ((v5.x - v4.x).powi(2) + (v5.y - v4.y).powi(2) + (v5.z - v4.z).powi(2)).sqrt(),
            ((v6.x - v5.x).powi(2) + (v6.y - v5.y).powi(2) + (v6.z - v5.z).powi(2)).sqrt(),
            ((v7.x - v6.x).powi(2) + (v7.y - v6.y).powi(2) + (v7.z - v6.z).powi(2)).sqrt(),
            ((v4.x - v7.x).powi(2) + (v4.y - v7.y).powi(2) + (v4.z - v7.z).powi(2)).sqrt(),
            ((v4.x - v0.x).powi(2) + (v4.y - v0.y).powi(2) + (v4.z - v0.z).powi(2)).sqrt(),
            ((v5.x - v1.x).powi(2) + (v5.y - v1.y).powi(2) + (v5.z - v1.z).powi(2)).sqrt(),
            ((v6.x - v2.x).powi(2) + (v6.y - v2.y).powi(2) + (v6.z - v2.z).powi(2)).sqrt(),
            ((v7.x - v3.x).powi(2) + (v7.y - v3.y).powi(2) + (v7.z - v3.z).powi(2)).sqrt(),
        ];

        let max_edge = edges.iter().fold(0.0, |max, &e| max.max(e));
        let min_edge = edges.iter().fold(f64::MAX, |min, &e| min.min(e));

        // Calculate aspect ratio
        let aspect_ratio = if min_edge > 0.0 {
            max_edge / min_edge
        } else {
            10.0
        };

        // Calculate quality score
        let aspect_score = if aspect_ratio < 5.0 {
            1.0
        } else {
            5.0 / aspect_ratio
        };

        aspect_score
    }

    /// Calculate tetrahedron volume
    fn calculate_tetrahedron_volume(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
        let v1 = [p2.x - p1.x, p2.y - p1.y, p2.z - p1.z];
        let v2 = [p3.x - p1.x, p3.y - p1.y, p3.z - p1.z];
        let v3 = [p4.x - p1.x, p4.y - p1.y, p4.z - p1.z];

        let cross = [
            v2[1] * v3[2] - v2[2] * v3[1],
            v2[2] * v3[0] - v2[0] * v3[2],
            v2[0] * v3[1] - v2[1] * v3[0],
        ];

        (v1[0] * cross[0] + v1[1] * cross[1] + v1[2] * cross[2]).abs() / 6.0
    }

    /// Mark elements for refinement/coarsening for 3D mesh
    fn mark_elements_3d(&self, mesh: &Mesh3D) -> (HashSet<usize>, HashSet<usize>) {
        let mut to_refine = HashSet::new();
        let mut to_coarsen = HashSet::new();

        // Process tetrahedrons
        for (tetra_id, &quality) in &self.element_quality {
            if *tetra_id < mesh.tetrahedrons.len() {
                let level = self.element_levels.get(tetra_id).unwrap_or(&0);
                if quality > self.params.refinement_threshold
                    && *level < self.params.max_refinement_level
                {
                    to_refine.insert(*tetra_id);
                } else if quality < self.params.coarsening_threshold && *level > 0 {
                    to_coarsen.insert(*tetra_id);
                }
            }
        }

        // Process hexahedrons
        for (hex_id, &quality) in &self.element_quality {
            if *hex_id >= mesh.tetrahedrons.len() {
                let level = self.element_levels.get(hex_id).unwrap_or(&0);
                if quality > self.params.refinement_threshold
                    && *level < self.params.max_refinement_level
                {
                    to_refine.insert(*hex_id);
                } else if quality < self.params.coarsening_threshold && *level > 0 {
                    to_coarsen.insert(*hex_id);
                }
            }
        }

        (to_refine, to_coarsen)
    }

    /// Refine elements for 3D mesh
    fn refine_elements_3d(&self, mesh: &mut Mesh3D, to_refine: &HashSet<usize>) {
        if to_refine.is_empty() {
            return;
        }

        let num_tetras = mesh.tetrahedrons.len();
        let mut new_vertices: HashMap<(usize, usize), usize> = HashMap::new();

        // Create edge midpoints for tetrahedrons
        for &tetra_id in to_refine {
            if tetra_id >= num_tetras {
                continue;
            }

            let tetra = &mesh.tetrahedrons[tetra_id];
            let edges = [
                (tetra.vertices[0], tetra.vertices[1]),
                (tetra.vertices[0], tetra.vertices[2]),
                (tetra.vertices[0], tetra.vertices[3]),
                (tetra.vertices[1], tetra.vertices[2]),
                (tetra.vertices[1], tetra.vertices[3]),
                (tetra.vertices[2], tetra.vertices[3]),
            ];

            for (v0, v1) in edges {
                let edge_key = if v0 < v1 { (v0, v1) } else { (v1, v0) };
                if !new_vertices.contains_key(&edge_key) {
                    let p0 = &mesh.vertices[v0].point;
                    let p1 = &mesh.vertices[v1].point;
                    let midpoint = Point::new(
                        (p0.x + p1.x) / 2.0,
                        (p0.y + p1.y) / 2.0,
                        (p0.z + p1.z) / 2.0,
                    );
                    let new_id = mesh.vertices.len();
                    mesh.vertices
                        .push(crate::mesh::mesh_data::MeshVertex::new(new_id, midpoint));
                    new_vertices.insert(edge_key, new_id);
                }
            }
        }

        // Subdivide tetrahedrons using red refinement (1-to-8 subdivision)
        let mut new_tetrahedrons = Vec::new();
        let mut tetras_to_remove = HashSet::new();

        for &tetra_id in to_refine {
            if tetra_id >= num_tetras {
                continue;
            }

            let tetra = &mesh.tetrahedrons[tetra_id];
            let v0 = tetra.vertices[0];
            let v1 = tetra.vertices[1];
            let v2 = tetra.vertices[2];
            let v3 = tetra.vertices[3];

            // Get edge midpoints
            let m01 = new_vertices.get(&(v0.min(v1), v0.max(v1))).unwrap();
            let m02 = new_vertices.get(&(v0.min(v2), v0.max(v2))).unwrap();
            let m03 = new_vertices.get(&(v0.min(v3), v0.max(v3))).unwrap();
            let m12 = new_vertices.get(&(v1.min(v2), v1.max(v2))).unwrap();
            let m13 = new_vertices.get(&(v1.min(v3), v1.max(v3))).unwrap();
            let m23 = new_vertices.get(&(v2.min(v3), v2.max(v3))).unwrap();

            // Create 8 new tetrahedrons
            new_tetrahedrons.push([v0, *m01, *m02, *m03]);
            new_tetrahedrons.push([v1, *m01, *m12, *m13]);
            new_tetrahedrons.push([v2, *m02, *m12, *m23]);
            new_tetrahedrons.push([v3, *m03, *m13, *m23]);
            new_tetrahedrons.push([*m01, *m02, *m03, *m13]);
            new_tetrahedrons.push([*m01, *m02, *m12, *m13]);
            new_tetrahedrons.push([*m02, *m03, *m13, *m23]);
            new_tetrahedrons.push([*m02, *m12, *m13, *m23]);

            tetras_to_remove.insert(tetra_id);
        }

        // Remove old tetrahedrons and add new ones
        mesh.tetrahedrons = mesh
            .tetrahedrons
            .iter()
            .enumerate()
            .filter(|(i, _)| !tetras_to_remove.contains(i))
            .map(|(_, t)| t.clone())
            .collect();

        for verts in new_tetrahedrons {
            mesh.tetrahedrons
                .push(crate::mesh::mesh_data::MeshTetrahedron { vertices: verts });
        }
    }

    /// Coarsen elements for 3D mesh
    fn coarsen_elements_3d(&self, mesh: &mut Mesh3D, to_coarsen: &HashSet<usize>) {
        if to_coarsen.is_empty() {
            return;
        }

        let num_tetras = mesh.tetrahedrons.len();

        // Build vertex-to-tetrahedron adjacency
        let mut vertex_tetras: HashMap<usize, Vec<usize>> = HashMap::new();
        for (tetra_id, tetra) in mesh.tetrahedrons.iter().enumerate() {
            for &v in &tetra.vertices {
                vertex_tetras.entry(v).or_default().push(tetra_id);
            }
        }

        // Find edges that can be collapsed
        let mut edges_to_collapse: Vec<(usize, usize)> = Vec::new();
        let mut processed_tetras: HashSet<usize> = HashSet::new();

        for &tetra_id in to_coarsen {
            if tetra_id >= num_tetras || processed_tetras.contains(&tetra_id) {
                continue;
            }

            let tetra = &mesh.tetrahedrons[tetra_id];

            // Find the shortest edge
            let mut shortest_edge = (0, 1);
            let mut shortest_length = f64::MAX;

            for i in 0..4 {
                for j in (i + 1)..4 {
                    let v0 = tetra.vertices[i];
                    let v1 = tetra.vertices[j];
                    let p0 = &mesh.vertices[v0].point;
                    let p1 = &mesh.vertices[v1].point;
                    let length = p0.distance(p1);

                    if length < shortest_length {
                        shortest_length = length;
                        shortest_edge = (v0, v1);
                    }
                }
            }

            // Check if collapse is valid
            let (v0, v1) = shortest_edge;
            let tetras_0 = vertex_tetras.get(&v0).map(|v| v.as_slice()).unwrap_or(&[]);
            let tetras_1 = vertex_tetras.get(&v1).map(|v| v.as_slice()).unwrap_or(&[]);

            // Only collapse if both vertices are in tetrahedrons marked for coarsening
            let can_collapse = tetras_0
                .iter()
                .all(|t| to_coarsen.contains(t) || *t >= num_tetras)
                && tetras_1
                    .iter()
                    .all(|t| to_coarsen.contains(t) || *t >= num_tetras);

            if can_collapse {
                edges_to_collapse.push((v0, v1));
                for &t in tetras_0.iter().chain(tetras_1.iter()) {
                    if t < num_tetras {
                        processed_tetras.insert(t);
                    }
                }
            }
        }

        // Collapse edges
        for (v0, v1) in edges_to_collapse {
            // Calculate midpoint
            let p0 = &mesh.vertices[v0].point;
            let p1 = &mesh.vertices[v1].point;
            let midpoint = Point::new(
                (p0.x + p1.x) / 2.0,
                (p0.y + p1.y) / 2.0,
                (p0.z + p1.z) / 2.0,
            );

            // Update vertex position
            mesh.vertices[v0].point = midpoint;

            // Update all tetrahedrons that reference v1 to use v0 instead
            for tetra in &mut mesh.tetrahedrons {
                for v in &mut tetra.vertices {
                    if *v == v1 {
                        *v = v0;
                    }
                }
            }

            // Remove degenerate tetrahedrons (tetrahedrons with duplicate vertices)
            mesh.tetrahedrons.retain(|tetra| {
                let mut seen = HashSet::new();
                tetra.vertices.iter().all(|v| seen.insert(*v))
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_adaptive_mesher_creation() {
        let params = AdaptiveMesherParams::default();
        let mesher = AdaptiveMesher::new(params);
        // Test passed if no panic
    }

    #[test]
    fn test_calculate_tetrahedron_volume() {
        let mesher = AdaptiveMesher::new(AdaptiveMesherParams::default());
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let p4 = Point::new(0.0, 0.0, 1.0);
        let volume = mesher.calculate_tetrahedron_volume(&p1, &p2, &p3, &p4);
        assert!((volume - 1.0 / 6.0).abs() < 1e-6);
    }

    #[test]
    fn test_adapt_mesh_2d() {
        let mut mesh = Mesh2D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);
        mesh.add_face(v0, v2, v3);

        let mut mesher = AdaptiveMesher::new(AdaptiveMesherParams::default());
        let result = mesher.adapt_mesh_2d(&mut mesh);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adapt_mesh_3d() {
        let mut mesh = Mesh3D::new();
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v4 = mesh.add_vertex(Point::new(0.5, 0.5, 1.0));
        mesh.add_tetrahedron(v0, v1, v2, v4);
        mesh.add_tetrahedron(v0, v2, v3, v4);

        let mut mesher = AdaptiveMesher::new(AdaptiveMesherParams::default());
        let result = mesher.adapt_mesh_3d(&mut mesh);
        assert!(result.is_ok());
    }
}
