//! Model Quality Module
//!
//! This module provides functionality for evaluating and repairing 3D models,
//! including detection of geometric errors and automated repair mechanisms.

use std::collections::{HashMap, HashSet};

use crate::ai_ml::protocol::{AiProtocolError, AiResult};
use crate::geometry::Point;
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

/// Model Quality Assessment Results
pub struct ModelQualityReport {
    pub is_valid: bool,
    pub error_count: usize,
    pub errors: Vec<ModelError>,
    pub repair_suggestions: Vec<String>,
    pub quality_score: f64,
}

/// Model Error Types
#[derive(Debug, Clone, PartialEq)]
pub enum ModelError {
    NonManifoldEdge(usize, usize),    // (vertex1, vertex2)
    NonManifoldVertex(usize),         // vertex index
    OverlappingFaces(usize, usize),   // (face1, face2)
    DegenerateFace(usize),            // face index
    SelfIntersection(usize, usize),   // (face1, face2)
    DuplicateVertices(Vec<usize>),    // list of duplicate vertex indices
    UnreferencedVertices(Vec<usize>), // list of unreferenced vertex indices
    EmptyFaces(Vec<usize>),           // list of empty face indices
}

/// Model Quality Evaluator
pub struct ModelQualityEvaluator {
    // Configuration parameters
    tolerance: f64,
    enable_self_intersection_check: bool,
}

impl ModelQualityEvaluator {
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
            enable_self_intersection_check: true,
        }
    }

    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    pub fn with_self_intersection_check(mut self, enable: bool) -> Self {
        self.enable_self_intersection_check = enable;
        self
    }

    /// Evaluate model quality
    pub fn evaluate(&self, mesh: &Mesh3D) -> ModelQualityReport {
        let mut errors = Vec::new();

        // Check for duplicate vertices
        self.check_duplicate_vertices(mesh, &mut errors);

        // Check for unreferenced vertices
        self.check_unreferenced_vertices(mesh, &mut errors);

        // Check for empty faces
        self.check_empty_faces(mesh, &mut errors);

        // Check for degenerate faces
        self.check_degenerate_faces(mesh, &mut errors);

        // Check for non-manifold edges
        self.check_non_manifold_edges(mesh, &mut errors);

        // Check for non-manifold vertices
        self.check_non_manifold_vertices(mesh, &mut errors);

        // Check for overlapping faces
        self.check_overlapping_faces(mesh, &mut errors);

        // Check for self-intersections
        if self.enable_self_intersection_check {
            self.check_self_intersections(mesh, &mut errors);
        }

        let is_valid = errors.is_empty();
        let error_count = errors.len();
        let repair_suggestions = self.generate_repair_suggestions(&errors);
        let quality_score = self.calculate_quality_score(mesh, error_count);

        ModelQualityReport {
            is_valid,
            error_count,
            errors,
            repair_suggestions,
            quality_score,
        }
    }

    /// Check for duplicate vertices
    fn check_duplicate_vertices(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        let mut vertex_map: HashMap<Point, Vec<usize>> = HashMap::new();

        for (i, vertex) in mesh.vertices.iter().enumerate() {
            // Round to tolerance to find duplicates
            let rounded_point = Point::new(
                (vertex.point.x / self.tolerance).round() * self.tolerance,
                (vertex.point.y / self.tolerance).round() * self.tolerance,
                (vertex.point.z / self.tolerance).round() * self.tolerance,
            );

            vertex_map.entry(rounded_point).or_default().push(i);
        }

        for (_, indices) in vertex_map {
            if indices.len() > 1 {
                errors.push(ModelError::DuplicateVertices(indices));
            }
        }
    }

    /// Check for unreferenced vertices
    fn check_unreferenced_vertices(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        let mut referenced = vec![false; mesh.vertices.len()];

        for face in &mesh.faces {
            for &vertex_id in &face.vertices {
                if vertex_id < referenced.len() {
                    referenced[vertex_id] = true;
                }
            }
        }

        let unreferenced: Vec<usize> = referenced
            .iter()
            .enumerate()
            .filter(|(_, &is_referenced)| !is_referenced)
            .map(|(i, _)| i)
            .collect();

        if !unreferenced.is_empty() {
            errors.push(ModelError::UnreferencedVertices(unreferenced));
        }
    }

    /// Check for empty faces
    fn check_empty_faces(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        let mut empty_faces = Vec::new();

        for (i, face) in mesh.faces.iter().enumerate() {
            if face.vertices.len() < 3 {
                empty_faces.push(i);
            }
        }

        if !empty_faces.is_empty() {
            errors.push(ModelError::EmptyFaces(empty_faces));
        }
    }

    /// Check for degenerate faces
    fn check_degenerate_faces(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        for (i, face) in mesh.faces.iter().enumerate() {
            if face.vertices.len() < 3 {
                continue;
            }

            // Check if all vertices are the same
            let first_vertex = &mesh.vertices[face.vertices[0]].point;
            let all_same = face.vertices.iter().all(|&vid| {
                let vertex = &mesh.vertices[vid].point;
                (vertex.x - first_vertex.x).abs() < self.tolerance
                    && (vertex.y - first_vertex.y).abs() < self.tolerance
                    && (vertex.z - first_vertex.z).abs() < self.tolerance
            });

            if all_same {
                errors.push(ModelError::DegenerateFace(i));
                continue;
            }

            // Check if face is flat or has zero area
            // Simplified check: if first three points are colinear
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;

                let vec1 = v1.clone() - v0.clone();
                let vec2 = v2.clone() - v0.clone();
                let cross = vec1.cross(&vec2);
                let area = cross.magnitude();

                if area < self.tolerance {
                    errors.push(ModelError::DegenerateFace(i));
                }
            }
        }
    }

    /// Check for non-manifold edges
    fn check_non_manifold_edges(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        // Edge is represented as a sorted pair of vertex indices
        let mut edge_count: HashMap<(usize, usize), usize> = HashMap::new();

        for face in &mesh.faces {
            let vertices = &face.vertices;
            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];
                let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                *edge_count.entry(edge).or_default() += 1;
            }
        }

        for ((v1, v2), count) in edge_count {
            if count != 2 {
                errors.push(ModelError::NonManifoldEdge(v1, v2));
            }
        }
    }

    /// Check for non-manifold vertices
    fn check_non_manifold_vertices(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        // Build adjacency list of edges for each vertex
        let mut vertex_edges: HashMap<usize, HashSet<usize>> = HashMap::new();

        for face in &mesh.faces {
            let vertices = &face.vertices;
            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];
                vertex_edges.entry(v1).or_default().insert(v2);
                vertex_edges.entry(v2).or_default().insert(v1);
            }
        }

        // For each vertex, check if edges form a single cycle
        for (vertex, neighbors) in vertex_edges {
            // Simplified check: if vertex has more than 2 edges, it might be non-manifold
            // This is a basic check and might produce false positives
            if neighbors.len() > 4 {
                errors.push(ModelError::NonManifoldVertex(vertex));
            }
        }
    }

    /// Check for overlapping faces
    fn check_overlapping_faces(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        // Simplified check: compare face centroids
        for i in 0..mesh.faces.len() {
            for j in i + 1..mesh.faces.len() {
                let centroid1 = self.calculate_face_centroid(mesh, i);
                let centroid2 = self.calculate_face_centroid(mesh, j);

                let distance = centroid1.distance(&centroid2);
                if distance < self.tolerance {
                    errors.push(ModelError::OverlappingFaces(i, j));
                }
            }
        }
    }

    /// Check for self-intersections
    fn check_self_intersections(&self, mesh: &Mesh3D, errors: &mut Vec<ModelError>) {
        // Simplified check: this is a computationally expensive operation
        // For simplicity, we'll skip detailed intersection checking
        // In a real implementation, you would use more sophisticated algorithms
    }

    /// Calculate face centroid
    fn calculate_face_centroid(&self, mesh: &Mesh3D, face_index: usize) -> Point {
        let face = &mesh.faces[face_index];
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for &vertex_id in &face.vertices {
            let vertex = &mesh.vertices[vertex_id];
            sum_x += vertex.point.x;
            sum_y += vertex.point.y;
            sum_z += vertex.point.z;
        }

        let count = face.vertices.len() as f64;
        Point::new(sum_x / count, sum_y / count, sum_z / count)
    }

    /// Generate repair suggestions
    fn generate_repair_suggestions(&self, errors: &[ModelError]) -> Vec<String> {
        let mut suggestions = Vec::new();

        for error in errors {
            match error {
                ModelError::NonManifoldEdge(v1, v2) => {
                    suggestions.push(format!(
                        "Fix non-manifold edge between vertices {} and {}",
                        v1, v2
                    ));
                }
                ModelError::NonManifoldVertex(v) => {
                    suggestions.push(format!("Fix non-manifold vertex {}", v));
                }
                ModelError::OverlappingFaces(f1, f2) => {
                    suggestions.push(format!(
                        "Remove or adjust overlapping faces {} and {}",
                        f1, f2
                    ));
                }
                ModelError::DegenerateFace(f) => {
                    suggestions.push(format!("Remove degenerate face {}", f));
                }
                ModelError::SelfIntersection(f1, f2) => {
                    suggestions.push(format!(
                        "Fix self-intersection between faces {} and {}",
                        f1, f2
                    ));
                }
                ModelError::DuplicateVertices(indices) => {
                    suggestions.push(format!("Merge duplicate vertices: {:?}", indices));
                }
                ModelError::UnreferencedVertices(indices) => {
                    suggestions.push(format!("Remove unreferenced vertices: {:?}", indices));
                }
                ModelError::EmptyFaces(indices) => {
                    suggestions.push(format!("Remove empty faces: {:?}", indices));
                }
            }
        }

        suggestions
    }

    /// Calculate quality score (0.0 to 1.0)
    fn calculate_quality_score(&self, mesh: &Mesh3D, error_count: usize) -> f64 {
        let total_elements = mesh.vertices.len() + mesh.faces.len();
        if total_elements == 0 {
            return 0.0;
        }

        let error_ratio = error_count as f64 / total_elements as f64;
        let score = 1.0 - error_ratio;
        score.max(0.0).min(1.0)
    }
}

/// Model Repair Tool
pub struct ModelRepairTool {
    evaluator: ModelQualityEvaluator,
}

impl ModelRepairTool {
    pub fn new() -> Self {
        Self {
            evaluator: ModelQualityEvaluator::new(),
        }
    }

    /// Repair model
    pub fn repair(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        let report = self.evaluator.evaluate(mesh);

        if report.is_valid {
            return Ok(mesh.clone());
        }

        let mut repaired_mesh = mesh.clone();

        // Apply repairs in order of priority
        self.repair_duplicate_vertices(&mut repaired_mesh)?;
        self.repair_empty_faces(&mut repaired_mesh);
        self.repair_degenerate_faces(&mut repaired_mesh);
        self.repair_unreferenced_vertices(&mut repaired_mesh);

        // Re-evaluate after repairs
        let final_report = self.evaluator.evaluate(&repaired_mesh);

        if !final_report.is_valid {
            return Err(AiProtocolError::ModelError(format!(
                "Could not fully repair model. Remaining errors: {}",
                final_report.error_count
            )));
        }

        Ok(repaired_mesh)
    }

    /// Repair duplicate vertices
    fn repair_duplicate_vertices(&self, mesh: &mut Mesh3D) -> AiResult<()> {
        let mut vertex_map: HashMap<Point, usize> = HashMap::new();
        let mut new_vertices = Vec::new();
        let mut vertex_mapping = Vec::new();

        // Round vertices to tolerance and build mapping
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            let rounded_point = Point::new(
                (vertex.point.x / self.evaluator.tolerance).round() * self.evaluator.tolerance,
                (vertex.point.y / self.evaluator.tolerance).round() * self.evaluator.tolerance,
                (vertex.point.z / self.evaluator.tolerance).round() * self.evaluator.tolerance,
            );

            if let Some(&existing_index) = vertex_map.get(&rounded_point) {
                vertex_mapping.push(existing_index);
            } else {
                let new_index = new_vertices.len();
                vertex_map.insert(rounded_point, new_index);
                new_vertices.push(vertex.clone());
                vertex_mapping.push(new_index);
            }
        }

        // Update faces with new vertex indices
        for face in &mut mesh.faces {
            for vertex_id in &mut face.vertices {
                *vertex_id = vertex_mapping[*vertex_id];
            }
        }

        mesh.vertices = new_vertices;
        Ok(())
    }

    /// Repair empty faces
    fn repair_empty_faces(&self, mesh: &mut Mesh3D) {
        mesh.faces.retain(|face| face.vertices.len() >= 3);
    }

    /// Repair degenerate faces
    fn repair_degenerate_faces(&self, mesh: &mut Mesh3D) {
        mesh.faces.retain(|face| {
            if face.vertices.len() < 3 {
                return false;
            }

            // Check if all vertices are the same
            let first_vertex = &mesh.vertices[face.vertices[0]].point;
            let all_same = face.vertices.iter().all(|&vid| {
                let vertex = &mesh.vertices[vid].point;
                (vertex.x - first_vertex.x).abs() < self.evaluator.tolerance
                    && (vertex.y - first_vertex.y).abs() < self.evaluator.tolerance
                    && (vertex.z - first_vertex.z).abs() < self.evaluator.tolerance
            });

            if all_same {
                return false;
            }

            // Check if face has zero area
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;

                let vec1 = v1.clone() - v0.clone();
                let vec2 = v2.clone() - v0.clone();
                let cross = vec1.cross(&vec2);
                let area = cross.magnitude();

                if area < self.evaluator.tolerance {
                    return false;
                }
            }

            true
        });
    }

    /// Repair unreferenced vertices
    fn repair_unreferenced_vertices(&self, mesh: &mut Mesh3D) {
        // Build set of referenced vertices
        let mut referenced = HashSet::new();
        for face in &mesh.faces {
            for &vertex_id in &face.vertices {
                referenced.insert(vertex_id);
            }
        }

        // Build new vertex list and mapping
        let mut new_vertices = Vec::new();
        let mut vertex_mapping = vec![0; mesh.vertices.len()];

        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if referenced.contains(&i) {
                vertex_mapping[i] = new_vertices.len();
                new_vertices.push(vertex.clone());
            }
        }

        // Update faces with new vertex indices
        for face in &mut mesh.faces {
            for vertex_id in &mut face.vertices {
                *vertex_id = vertex_mapping[*vertex_id];
            }
        }

        mesh.vertices = new_vertices;
    }
}

/// Extension methods for Mesh3D
pub trait MeshQualityExt {
    /// Evaluate model quality
    fn evaluate_quality(&self) -> ModelQualityReport;

    /// Repair model
    fn repair(&self) -> AiResult<Mesh3D>;
}

impl MeshQualityExt for Mesh3D {
    fn evaluate_quality(&self) -> ModelQualityReport {
        let evaluator = ModelQualityEvaluator::new();
        evaluator.evaluate(self)
    }

    fn repair(&self) -> AiResult<Mesh3D> {
        let repair_tool = ModelRepairTool::new();
        repair_tool.repair(self)
    }
}
