//! Model Optimization Module
//!
//! This module provides functionality for optimizing 3D models,
//! including mesh simplification, topology optimization, and LOD generation.

use std::collections::{HashMap, HashSet};

use crate::ai_ml::protocol::{AiProtocolError, AiResult};
use crate::geometry::Point;
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

/// Model Optimization Settings
#[derive(Debug, Default, Clone)]
pub struct OptimizationSettings {
    pub target_polygon_count: Option<usize>,
    pub target_reduction_ratio: Option<f64>,
    pub quality_threshold: f64, // 0.0 to 1.0
    pub preserve_boundaries: bool,
    pub preserve_features: bool,
}

/// Model Simplification Result
pub struct SimplificationResult {
    pub simplified_mesh: Mesh3D,
    pub original_polygon_count: usize,
    pub simplified_polygon_count: usize,
    pub reduction_ratio: f64,
    pub quality_score: f64,
}

/// LOD (Level of Detail) Settings
pub struct LodSettings {
    pub levels: usize,              // Number of LOD levels
    pub reduction_ratios: Vec<f64>, // Reduction ratio for each level
}

impl Default for LodSettings {
    fn default() -> Self {
        Self {
            levels: 3,
            reduction_ratios: vec![0.7, 0.4, 0.2], // 70%, 40%, 20% of original
        }
    }
}

/// LOD Result
pub struct LodResult {
    pub lods: Vec<Mesh3D>,
    pub original_mesh: Mesh3D,
    pub reduction_ratios: Vec<f64>,
}

/// Model Optimizer
pub struct ModelOptimizer {
    settings: OptimizationSettings,
}

impl ModelOptimizer {
    pub fn new() -> Self {
        Self {
            settings: OptimizationSettings::default(),
        }
    }

    pub fn with_settings(mut self, settings: OptimizationSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Simplify mesh
    pub fn simplify(&self, mesh: &Mesh3D) -> AiResult<SimplificationResult> {
        let original_polygon_count = mesh.faces.len();
        if original_polygon_count == 0 {
            return Err(AiProtocolError::ModelError(
                "Cannot simplify empty mesh".to_string(),
            ));
        }

        // Calculate target polygon count
        let target_polygon_count = match self.settings.target_polygon_count {
            Some(count) => count,
            None => {
                let ratio = self.settings.target_reduction_ratio.unwrap_or(0.5);
                std::cmp::max(1, (original_polygon_count as f64 * ratio).round() as usize)
            }
        };

        if target_polygon_count >= original_polygon_count {
            return Ok(SimplificationResult {
                simplified_mesh: mesh.clone(),
                original_polygon_count,
                simplified_polygon_count: original_polygon_count,
                reduction_ratio: 1.0,
                quality_score: 1.0,
            });
        }

        // Perform mesh simplification using quadric error metric (simplified implementation)
        let simplified_mesh = self.simplify_mesh(mesh, target_polygon_count)?;
        let simplified_polygon_count = simplified_mesh.faces.len();
        let reduction_ratio = simplified_polygon_count as f64 / original_polygon_count as f64;
        let quality_score = self.calculate_quality_score(mesh, &simplified_mesh);

        Ok(SimplificationResult {
            simplified_mesh,
            original_polygon_count,
            simplified_polygon_count,
            reduction_ratio,
            quality_score,
        })
    }

    /// Generate LODs
    pub fn generate_lods(&self, mesh: &Mesh3D, lod_settings: &LodSettings) -> AiResult<LodResult> {
        let mut lods = Vec::new();
        let mut reduction_ratios = Vec::new();

        for ratio in &lod_settings.reduction_ratios {
            let target_count = std::cmp::max(1, (mesh.faces.len() as f64 * ratio).round() as usize);
            let simplified_mesh = self.simplify_mesh(mesh, target_count)?;
            lods.push(simplified_mesh);
            reduction_ratios.push(*ratio);
        }

        Ok(LodResult {
            lods,
            original_mesh: mesh.clone(),
            reduction_ratios,
        })
    }

    /// Optimize topology
    pub fn optimize_topology(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        // This is a simplified implementation of topology optimization
        // In a real implementation, this would involve more sophisticated algorithms

        // 1. Merge duplicate vertices
        let mut optimized_mesh = self.merge_duplicate_vertices(mesh)?;

        // 2. Remove degenerate faces
        self.remove_degenerate_faces(&mut optimized_mesh);

        // 3. Optimize face order and vertex layout for better cache performance
        self.optimize_vertex_cache(&mut optimized_mesh);

        Ok(optimized_mesh)
    }

    /// Simplify mesh using quadric error metric approach
    fn simplify_mesh(&self, mesh: &Mesh3D, target_count: usize) -> AiResult<Mesh3D> {
        // Create a copy of the mesh
        let mut simplified = mesh.clone();

        // Calculate edge collapse costs using quadric error metric
        let mut _edge_costs = self.calculate_edge_costs(&simplified);

        // Collapse edges until we reach target count
        let mut current_count = simplified.faces.len();
        let mut collapsed_vertices = HashSet::new();

        while current_count > target_count {
            // Recalculate edge costs and sort
            let edge_costs = self.calculate_edge_costs(&simplified);
            let mut edges: Vec<_> = edge_costs.iter().collect();
            edges.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

            // Find the best edge to collapse
            let mut collapsed = false;
            for (edge, _cost) in edges {
                let (v1, v2) = *edge;

                // Skip if either vertex has been collapsed
                if collapsed_vertices.contains(&v1) || collapsed_vertices.contains(&v2) {
                    continue;
                }

                // Try to collapse the edge
                if self.collapse_edge(&mut simplified, v1, v2).is_some() {
                    current_count = simplified.faces.len();
                    collapsed_vertices.insert(v1);
                    collapsed_vertices.insert(v2);
                    collapsed = true;
                    break;
                }
            }

            // If no edges can be collapsed, break
            if !collapsed {
                break;
            }
        }

        Ok(simplified)
    }

    /// Calculate edge collapse costs using quadric error metric
    fn calculate_edge_costs(&self, mesh: &Mesh3D) -> HashMap<(usize, usize), f64> {
        let mut edge_costs = HashMap::new();

        // For each face, add edges and calculate costs
        for face in &mesh.faces {
            let vertices = &face.vertices;
            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];
                let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };

                // Skip if edge already processed
                if edge_costs.contains_key(&edge) {
                    continue;
                }

                // Calculate quadric error metric cost
                let cost = self.calculate_quadric_error(mesh, v1, v2);

                edge_costs.insert(edge, cost);
            }
        }

        edge_costs
    }

    /// Calculate quadric error metric for edge collapse
    fn calculate_quadric_error(&self, mesh: &Mesh3D, v1: usize, v2: usize) -> f64 {
        // Real implementation: proper quadric error metrics

        let p1 = &mesh.vertices[v1].point;
        let p2 = &mesh.vertices[v2].point;

        // Calculate quadric matrices for both vertices
        let q1 = self.calculate_quadric_matrix(mesh, v1);
        let q2 = self.calculate_quadric_matrix(mesh, v2);
        
        // Combine quadrics
        let q = self.add_quadrics(&q1, &q2);
        
        // Calculate optimal vertex position
        let optimal_point = self.find_optimal_vertex(&q, p1, p2);
        
        // Calculate error for optimal position
        self.calculate_error(&q, &optimal_point)
    }

    /// Calculate quadric matrix for a vertex
    fn calculate_quadric_matrix(&self, mesh: &Mesh3D, vertex_id: usize) -> [[f64; 4]; 4] {
        // Initialize quadric matrix
        let mut q = [[0.0; 4]; 4];
        
        // Find all faces adjacent to the vertex
        let adjacent_faces = self.find_adjacent_faces(mesh, vertex_id);
        
        for face_id in adjacent_faces {
            let face = &mesh.faces[face_id];
            
            // Calculate face normal if not available
            let normal = match face.normal {
                Some(n) => n,
                None => self.calculate_face_normal(mesh, face),
            };
            
            let v0 = &mesh.vertices[face.vertices[0]].point;
            
            // Plane equation: ax + by + cz + d = 0
            let a = normal[0];
            let b = normal[1];
            let c = normal[2];
            let d = -(a * v0.x + b * v0.y + c * v0.z);
            
            // Update quadric matrix with plane equation
            let plane = [a, b, c, d];
            self.update_quadric_with_plane(&mut q, &plane);
        }
        
        q
    }

    /// Find adjacent faces for a vertex
    fn find_adjacent_faces(&self, mesh: &Mesh3D, vertex_id: usize) -> Vec<usize> {
        let mut adjacent_faces = Vec::new();
        
        for (face_id, face) in mesh.faces.iter().enumerate() {
            if face.vertices.contains(&vertex_id) {
                adjacent_faces.push(face_id);
            }
        }
        
        adjacent_faces
    }

    /// Calculate face normal
    fn calculate_face_normal(&self, mesh: &Mesh3D, face: &MeshFace) -> [f64; 3] {
        if face.vertices.len() < 3 {
            return [0.0, 0.0, 1.0]; // Default normal
        }
        
        let v0 = &mesh.vertices[face.vertices[0]].point;
        let v1 = &mesh.vertices[face.vertices[1]].point;
        let v2 = &mesh.vertices[face.vertices[2]].point;
        
        let vec1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
        let vec2 = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];
        
        // Cross product
        let normal = [
            vec1[1] * vec2[2] - vec1[2] * vec2[1],
            vec1[2] * vec2[0] - vec1[0] * vec2[2],
            vec1[0] * vec2[1] - vec1[1] * vec2[0]
        ];
        
        // Normalize
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        if length > 1e-6 {
            [normal[0] / length, normal[1] / length, normal[2] / length]
        } else {
            [0.0, 0.0, 1.0]
        }
    }

    /// Update quadric matrix with plane equation
    fn update_quadric_with_plane(&self, q: &mut [[f64; 4]; 4], plane: &[f64; 4]) {
        let a = plane[0];
        let b = plane[1];
        let c = plane[2];
        let d = plane[3];
        
        q[0][0] += a * a;
        q[0][1] += a * b;
        q[0][2] += a * c;
        q[0][3] += a * d;
        
        q[1][1] += b * b;
        q[1][2] += b * c;
        q[1][3] += b * d;
        
        q[2][2] += c * c;
        q[2][3] += c * d;
        
        q[3][3] += d * d;
        
        // Symmetric matrix
        q[1][0] = q[0][1];
        q[2][0] = q[0][2];
        q[2][1] = q[1][2];
        q[3][0] = q[0][3];
        q[3][1] = q[1][3];
        q[3][2] = q[2][3];
    }

    /// Add two quadric matrices
    fn add_quadrics(&self, q1: &[[f64; 4]; 4], q2: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
        let mut result = [[0.0; 4]; 4];
        
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = q1[i][j] + q2[i][j];
            }
        }
        
        result
    }

    /// Find optimal vertex position for edge collapse
    fn find_optimal_vertex(&self, q: &[[f64; 4]; 4], p1: &Point, p2: &Point) -> Point {
        // Create matrix for linear system
        let mut mat = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                mat[i][j] = q[i][j];
            }
        }
        
        // Add constraint to ensure non-singular matrix
        mat[3][0] = 0.0;
        mat[3][1] = 0.0;
        mat[3][2] = 0.0;
        mat[3][3] = 1.0;
        
        // Create right-hand side
        let rhs = [0.0, 0.0, 0.0, 1.0];
        
        // Solve linear system
        if let Some(solution) = self.solve_linear_system(&mat, &rhs) {
            Point::new(solution[0], solution[1], solution[2])
        } else {
            // Fallback to midpoint if system is singular
            Point::new(
                (p1.x + p2.x) / 2.0,
                (p1.y + p2.y) / 2.0,
                (p1.z + p2.z) / 2.0
            )
        }
    }

    /// Solve linear system using Gaussian elimination
    fn solve_linear_system(&self, mat: &[[f64; 4]; 4], rhs: &[f64; 4]) -> Option<[f64; 4]> {
        let mut a = [[0.0; 5]; 4];
        for i in 0..4 {
            for j in 0..4 {
                a[i][j] = mat[i][j];
            }
            a[i][4] = rhs[i];
        }
        
        // Forward elimination
        for i in 0..4 {
            // Find pivot
            let mut max_row = i;
            for j in i+1..4 {
                if a[j][i].abs() > a[max_row][i].abs() {
                    max_row = j;
                }
            }
            
            // Swap rows
            if max_row != i {
                a.swap(i, max_row);
            }
            
            // Check for singular matrix
            if a[i][i].abs() < 1e-10 {
                return None;
            }
            
            // Eliminate below
            for j in i+1..4 {
                let factor = a[j][i] / a[i][i];
                for k in i..5 {
                    a[j][k] -= factor * a[i][k];
                }
            }
        }
        
        // Back substitution
        let mut x = [0.0; 4];
        for i in (0..4).rev() {
            x[i] = a[i][4];
            for j in i+1..4 {
                x[i] -= a[i][j] * x[j];
            }
            x[i] /= a[i][i];
        }
        
        Some(x)
    }

    /// Calculate error for a point using quadric matrix
    fn calculate_error(&self, q: &[[f64; 4]; 4], point: &Point) -> f64 {
        let v = [point.x, point.y, point.z, 1.0];
        let mut error = 0.0;
        
        for i in 0..4 {
            for j in 0..4 {
                error += v[i] * q[i][j] * v[j];
            }
        }
        
        error
    }

    /// Update edge costs after vertex collapse
    #[allow(dead_code)]
    fn update_edge_costs(
        &self,
        mesh: &Mesh3D,
        new_vertex_id: usize,
        edge_costs: &mut HashMap<(usize, usize), f64>,
    ) {
        // Real implementation: update edge costs involving the new vertex
        
        // Find all edges connected to the new vertex
        let mut new_edges = Vec::new();
        
        for face in &mesh.faces {
            for i in 0..face.vertices.len() {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % face.vertices.len()];
                
                if v1 == new_vertex_id || v2 == new_vertex_id {
                    let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                    new_edges.push(edge);
                }
            }
        }
        
        // Remove old edges that involved the collapsed vertices
        edge_costs.retain(|(v1, v2), _| *v1 != new_vertex_id && *v2 != new_vertex_id);
        
        // Calculate costs for new edges
        for edge in new_edges {
            let cost = self.calculate_quadric_error(mesh, edge.0, edge.1);
            edge_costs.insert(edge, cost);
        }
    }

    /// Collapse an edge
    fn collapse_edge(&self, mesh: &mut Mesh3D, v1: usize, v2: usize) -> Option<usize> {
        // Calculate new vertex position using quadric error metric
        let p1 = &mesh.vertices[v1].point;
        let p2 = &mesh.vertices[v2].point;
        
        // Calculate quadric matrices for both vertices
        let q1 = self.calculate_quadric_matrix(mesh, v1);
        let q2 = self.calculate_quadric_matrix(mesh, v2);
        
        // Combine quadrics
        let q = self.add_quadrics(&q1, &q2);
        
        // Calculate optimal vertex position
        let new_point = self.find_optimal_vertex(&q, p1, p2);

        // Create new vertex with average normal
        let new_normal = match (mesh.vertices[v1].normal, mesh.vertices[v2].normal) {
            (Some(n1), Some(n2)) => {
                let avg = [
                    (n1[0] + n2[0]) / 2.0,
                    (n1[1] + n2[1]) / 2.0,
                    (n1[2] + n2[2]) / 2.0,
                ];
                Some(avg)
            }
            (Some(n), None) => Some(n),
            (None, Some(n)) => Some(n),
            (None, None) => None,
        };

        let new_vertex = MeshVertex {
            id: mesh.vertices.len(),
            point: new_point,
            normal: new_normal,
            ..Default::default()
        };
        let new_vertex_id = mesh.vertices.len();
        mesh.vertices.push(new_vertex);

        // Update faces
        let mut new_faces = Vec::new();

        for face in &mesh.faces {
            // Skip faces that use both vertices
            if face.vertices.contains(&v1) && face.vertices.contains(&v2) {
                continue;
            }

            // Update vertices in the face
            let mut new_face_vertices = Vec::new();
            for &vid in &face.vertices {
                if vid == v1 || vid == v2 {
                    new_face_vertices.push(new_vertex_id);
                } else {
                    new_face_vertices.push(vid);
                }
            }

            // Add face if it's still valid
            if new_face_vertices.len() >= 3 {
                // Remove duplicate vertices
                let mut unique_vertices = Vec::new();
                for &vid in &new_face_vertices {
                    if !unique_vertices.contains(&vid) {
                        unique_vertices.push(vid);
                    }
                }

                if unique_vertices.len() >= 3 {
                    new_faces.push(MeshFace {
                        id: new_faces.len(),
                        vertices: unique_vertices,
                        normal: face.normal,
                        ..Default::default()
                    });
                }
            }
        }

        // Replace faces
        if !new_faces.is_empty() {
            mesh.faces = new_faces;
            Some(new_vertex_id)
        } else {
            None
        }
    }

    /// Merge duplicate vertices
    fn merge_duplicate_vertices(&self, mesh: &Mesh3D) -> AiResult<Mesh3D> {
        let mut vertex_map: HashMap<Point, usize> = HashMap::new();
        let mut new_vertices = Vec::new();
        let mut vertex_mapping = Vec::new();
        let tolerance = 1e-6;

        for (_i, vertex) in mesh.vertices.iter().enumerate() {
            // Round to tolerance
            let rounded_point = Point::new(
                (vertex.point.x / tolerance).round() * tolerance,
                (vertex.point.y / tolerance).round() * tolerance,
                (vertex.point.z / tolerance).round() * tolerance,
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

        // Update faces
        let mut new_faces = Vec::new();
        for face in &mesh.faces {
            let mut new_face_vertices = Vec::new();
            for &vid in &face.vertices {
                new_face_vertices.push(vertex_mapping[vid]);
            }
            new_faces.push(MeshFace {
                id: new_faces.len(),
                vertices: new_face_vertices,
                normal: face.normal,
                ..Default::default()
            });
        }

        let mut mesh = Mesh3D::new();
        mesh.vertices = new_vertices;
        mesh.faces = new_faces;
        Ok(mesh)
    }

    /// Remove degenerate faces
    fn remove_degenerate_faces(&self, mesh: &mut Mesh3D) {
        mesh.faces.retain(|face| {
            if face.vertices.len() < 3 {
                return false;
            }

            // Check if all vertices are the same
            let first_vertex = &mesh.vertices[face.vertices[0]].point;
            let all_same = face.vertices.iter().all(|&vid| {
                let vertex = &mesh.vertices[vid].point;
                (vertex.x - first_vertex.x).abs() < 1e-6
                    && (vertex.y - first_vertex.y).abs() < 1e-6
                    && (vertex.z - first_vertex.z).abs() < 1e-6
            });

            !all_same
        });
    }

    /// Optimize vertex cache
    fn optimize_vertex_cache(&self, mesh: &mut Mesh3D) {
        // Real implementation: more sophisticated vertex cache optimization
        
        // Use a variant of the Linear-Sweep algorithm with vertex cache awareness
        
        // Build face adjacency list
        let mut face_adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
        for (face_id, face) in mesh.faces.iter().enumerate() {
            for &vid in &face.vertices {
                face_adjacency.entry(vid).or_default().push(face_id);
            }
        }
        
        // Build vertex adjacency list
        let mut vertex_adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
        for face in &mesh.faces {
            for i in 0..face.vertices.len() {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % face.vertices.len()];
                vertex_adjacency.entry(v1).or_default().insert(v2);
                vertex_adjacency.entry(v2).or_default().insert(v1);
            }
        }
        
        // Initialize variables
        let mut face_visited = vec![false; mesh.faces.len()];
        let mut vertex_visited = vec![false; mesh.vertices.len()];
        let mut vertex_order = Vec::new();
        let mut face_order = Vec::new();
        
        // Cache size (typical GPU vertex cache size)
        let cache_size = 32;
        let mut cache = Vec::new();
        
        // Start with the first face
        if !mesh.faces.is_empty() {
            self.process_face(
                mesh,
                0,
                &mut face_visited,
                &mut vertex_visited,
                &mut vertex_order,
                &mut face_order,
                &face_adjacency,
                &vertex_adjacency,
                &mut cache,
                cache_size
            );
        }
        
        // Process remaining faces
        for face_id in 0..mesh.faces.len() {
            if !face_visited[face_id] {
                self.process_face(
                    mesh,
                    face_id,
                    &mut face_visited,
                    &mut vertex_visited,
                    &mut vertex_order,
                    &mut face_order,
                    &face_adjacency,
                    &vertex_adjacency,
                    &mut cache,
                    cache_size
                );
            }
        }
        
        // Create new vertex mapping
        let mut vertex_mapping = vec![0; mesh.vertices.len()];
        for (new_id, &old_id) in vertex_order.iter().enumerate() {
            vertex_mapping[old_id] = new_id;
        }
        
        // Create new vertices
        let mut new_vertices = Vec::with_capacity(mesh.vertices.len());
        for &old_id in &vertex_order {
            new_vertices.push(mesh.vertices[old_id].clone());
        }
        
        // Update faces in the new order
        let mut new_faces = Vec::with_capacity(mesh.faces.len());
        for &face_id in &face_order {
            let face = &mesh.faces[face_id];
            let mut new_face_vertices = Vec::with_capacity(face.vertices.len());
            for &old_id in &face.vertices {
                new_face_vertices.push(vertex_mapping[old_id]);
            }
            new_faces.push(MeshFace {
                id: new_faces.len(),
                vertices: new_face_vertices,
                normal: face.normal,
                ..Default::default()
            });
        }
        
        // Update mesh
        mesh.vertices = new_vertices;
        mesh.faces = new_faces;
    }
    
    /// Process a face and its neighbors
    fn process_face(
        &self,
        mesh: &Mesh3D,
        face_id: usize,
        face_visited: &mut Vec<bool>,
        vertex_visited: &mut Vec<bool>,
        vertex_order: &mut Vec<usize>,
        face_order: &mut Vec<usize>,
        face_adjacency: &HashMap<usize, Vec<usize>>,
        vertex_adjacency: &HashMap<usize, HashSet<usize>>,
        cache: &mut Vec<usize>,
        cache_size: usize
    ) {
        if face_visited[face_id] {
            return;
        }
        
        face_visited[face_id] = true;
        face_order.push(face_id);
        
        let face = &mesh.faces[face_id];
        
        // Add vertices to order and cache
        for &vid in &face.vertices {
            if !vertex_visited[vid] {
                vertex_visited[vid] = true;
                vertex_order.push(vid);
            }
            
            // Update cache
            if let Some(pos) = cache.iter().position(|&v| v == vid) {
                // Move to front
                cache.remove(pos);
                cache.insert(0, vid);
            } else {
                // Add to front
                cache.insert(0, vid);
                // Trim cache if needed
                if cache.len() > cache_size {
                    cache.pop();
                }
            }
        }
        
        // Find best adjacent face to process next
        let mut best_face = None;
        let mut best_score = -1.0;
        
        for &vid in &face.vertices {
            if let Some(adjacent_faces) = face_adjacency.get(&vid) {
                for &adj_face_id in adjacent_faces {
                    if !face_visited[adj_face_id] {
                        let score = self.calculate_face_score(mesh, adj_face_id, cache);
                        if score > best_score {
                            best_score = score;
                            best_face = Some(adj_face_id);
                        }
                    }
                }
            }
        }
        
        // Process best face next
        if let Some(next_face) = best_face {
            self.process_face(
                mesh,
                next_face,
                face_visited,
                vertex_visited,
                vertex_order,
                face_order,
                face_adjacency,
                vertex_adjacency,
                cache,
                cache_size
            );
        }
    }
    
    /// Calculate face score based on cache hits
    fn calculate_face_score(&self, mesh: &Mesh3D, face_id: usize, cache: &[usize]) -> f64 {
        let face = &mesh.faces[face_id];
        let mut score = 0.0;
        
        for &vid in &face.vertices {
            if let Some(pos) = cache.iter().position(|&v| v == vid) {
                // Higher score for vertices closer to the front of the cache
                score += (cache.len() - pos) as f64 / cache.len() as f64;
            } else {
                // Penalty for cache misses
                score -= 1.0;
            }
        }
        
        score
    }

    /// Calculate quality score between original and simplified mesh
    fn calculate_quality_score(&self, original: &Mesh3D, simplified: &Mesh3D) -> f64 {
        // Real implementation: more sophisticated quality metrics
        
        // Calculate multiple quality metrics
        let volume_ratio = self.calculate_volume_ratio(original, simplified);
        let normal_consistency = self.calculate_normal_consistency(original, simplified);
        let geometric_distortion = self.calculate_geometric_distortion(original, simplified);
        let edge_length_consistency = self.calculate_edge_length_consistency(original, simplified);
        
        // Combine metrics with weights
        let quality_score = volume_ratio * 0.25 +
            normal_consistency * 0.25 +
            (1.0 - geometric_distortion) * 0.25 +
            edge_length_consistency * 0.25;
        
        quality_score.max(0.0).min(1.0)
    }
    
    /// Calculate volume ratio between original and simplified mesh
    fn calculate_volume_ratio(&self, original: &Mesh3D, simplified: &Mesh3D) -> f64 {
        let original_volume = self.calculate_mesh_volume(original);
        let simplified_volume = self.calculate_mesh_volume(simplified);
        
        if original_volume > 0.0 {
            1.0 - (original_volume - simplified_volume).abs() / original_volume
        } else {
            1.0
        }
    }
    
    /// Calculate mesh volume
    fn calculate_mesh_volume(&self, mesh: &Mesh3D) -> f64 {
        let mut volume = 0.0;
        
        for face in &mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;
                
                // Calculate signed volume contribution
                let cross = [
                    v1.y * v2.z - v1.z * v2.y,
                    v1.z * v2.x - v1.x * v2.z,
                    v1.x * v2.y - v1.y * v2.x
                ];
                volume += v0.x * cross[0] + v0.y * cross[1] + v0.z * cross[2];
            }
        }
        
        (volume / 6.0).abs()
    }
    
    /// Calculate normal consistency between original and simplified mesh
    fn calculate_normal_consistency(&self, original: &Mesh3D, simplified: &Mesh3D) -> f64 {
        if original.vertices.is_empty() || simplified.vertices.is_empty() {
            return 1.0;
        }
        
        // For simplicity, we'll sample points on the original mesh and compare normals
        let sample_count = std::cmp::min(1000, original.vertices.len());
        let step = original.vertices.len() / sample_count;
        
        let mut total_consistency = 0.0;
        let mut valid_samples = 0;
        
        for i in (0..original.vertices.len()).step_by(step) {
            let original_vertex = &original.vertices[i];
            if let Some(original_normal) = original_vertex.normal {
                // Find closest vertex in simplified mesh
                if let Some((_, simplified_normal)) = self.find_closest_vertex_with_normal(&original_vertex.point, simplified) {
                    // Calculate dot product
                    let dot = original_normal[0] * simplified_normal[0] +
                              original_normal[1] * simplified_normal[1] +
                              original_normal[2] * simplified_normal[2];
                    total_consistency += dot.abs();
                    valid_samples += 1;
                }
            }
        }
        
        if valid_samples > 0 {
            total_consistency / valid_samples as f64
        } else {
            1.0
        }
    }
    
    /// Find closest vertex with normal in mesh
    fn find_closest_vertex_with_normal(&self, point: &Point, mesh: &Mesh3D) -> Option<(usize, [f64; 3])> {
        let mut closest_dist = std::f64::MAX;
        let mut closest_vertex = None;
        
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if let Some(normal) = vertex.normal {
                let dist = point.distance(&vertex.point);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_vertex = Some((i, normal));
                }
            }
        }
        
        closest_vertex
    }
    
    /// Calculate geometric distortion between original and simplified mesh
    fn calculate_geometric_distortion(&self, original: &Mesh3D, simplified: &Mesh3D) -> f64 {
        if original.vertices.is_empty() || simplified.vertices.is_empty() {
            return 0.0;
        }
        
        // Sample points on original mesh and calculate distance to simplified mesh
        let sample_count = std::cmp::min(1000, original.vertices.len());
        let step = original.vertices.len() / sample_count;
        
        let mut total_distortion = 0.0;
        
        for i in (0..original.vertices.len()).step_by(step) {
            let original_point = &original.vertices[i].point;
            let min_dist = self.find_min_distance_to_mesh(original_point, simplified);
            total_distortion += min_dist;
        }
        
        // Normalize by bounding box diagonal
        let original_bbox = self.calculate_bounding_box(original);
        let diagonal = original_bbox.0.distance(&original_bbox.1);
        
        if diagonal > 0.0 {
            (total_distortion / sample_count as f64) / diagonal
        } else {
            0.0
        }
    }
    
    /// Find minimum distance from point to mesh
    fn find_min_distance_to_mesh(&self, point: &Point, mesh: &Mesh3D) -> f64 {
        let mut min_dist = std::f64::MAX;
        
        for face in &mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;
                
                // Calculate distance from point to triangle
                let dist = self.distance_to_triangle(point, v0, v1, v2);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }
        
        min_dist
    }
    
    /// Calculate distance from point to triangle
    fn distance_to_triangle(&self, point: &Point, v0: &Point, v1: &Point, v2: &Point) -> f64 {
        // Vector from v0 to v1
        let v0v1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
        // Vector from v0 to v2
        let v0v2 = [v2.x - v0.x, v2.y - v0.y, v2.z - v0.z];
        // Vector from v0 to point
        let v0p = [point.x - v0.x, point.y - v0.y, point.z - v0.z];
        
        // Dot products
        let dot00 = v0v1[0] * v0v1[0] + v0v1[1] * v0v1[1] + v0v1[2] * v0v1[2];
        let dot01 = v0v1[0] * v0v2[0] + v0v1[1] * v0v2[1] + v0v1[2] * v0v2[2];
        let dot02 = v0v1[0] * v0p[0] + v0v1[1] * v0p[1] + v0v1[2] * v0p[2];
        let dot11 = v0v2[0] * v0v2[0] + v0v2[1] * v0v2[1] + v0v2[2] * v0v2[2];
        let dot12 = v0v2[0] * v0p[0] + v0v2[1] * v0p[1] + v0v2[2] * v0p[2];
        
        // Barycentric coordinates
        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        
        // Check if point is inside triangle
        if u >= 0.0 && v >= 0.0 && u + v <= 1.0 {
            // Project point onto triangle
            let proj = [
                v0.x + u * v0v1[0] + v * v0v2[0],
                v0.y + u * v0v1[1] + v * v0v2[1],
                v0.z + u * v0v1[2] + v * v0v2[2]
            ];
            
            // Calculate distance
            let dx = point.x - proj[0];
            let dy = point.y - proj[1];
            let dz = point.z - proj[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        } else {
            // Calculate distance to closest edge or vertex
            let dist_v0 = point.distance(v0);
            let dist_v1 = point.distance(v1);
            let dist_v2 = point.distance(v2);
            let dist_e0 = self.distance_to_line(point, v0, v1);
            let dist_e1 = self.distance_to_line(point, v1, v2);
            let dist_e2 = self.distance_to_line(point, v2, v0);
            
            [dist_v0, dist_v1, dist_v2, dist_e0, dist_e1, dist_e2].iter().fold(std::f64::MAX, |min, &d| min.min(d))
        }
    }
    
    /// Calculate distance from point to line segment
    fn distance_to_line(&self, point: &Point, v0: &Point, v1: &Point) -> f64 {
        let v0v1 = [v1.x - v0.x, v1.y - v0.y, v1.z - v0.z];
        let v0p = [point.x - v0.x, point.y - v0.y, point.z - v0.z];
        
        let dot = v0v1[0] * v0p[0] + v0v1[1] * v0p[1] + v0v1[2] * v0p[2];
        let len_sq = v0v1[0] * v0v1[0] + v0v1[1] * v0v1[1] + v0v1[2] * v0v1[2];
        
        let t = if len_sq > 0.0 {
            dot / len_sq
        } else {
            0.0
        };
        
        let t_clamped = t.max(0.0).min(1.0);
        
        let proj = [
            v0.x + t_clamped * v0v1[0],
            v0.y + t_clamped * v0v1[1],
            v0.z + t_clamped * v0v1[2]
        ];
        
        let dx = point.x - proj[0];
        let dy = point.y - proj[1];
        let dz = point.z - proj[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
    
    /// Calculate edge length consistency between original and simplified mesh
    fn calculate_edge_length_consistency(&self, original: &Mesh3D, simplified: &Mesh3D) -> f64 {
        let original_avg_edge_length = self.calculate_average_edge_length(original);
        let simplified_avg_edge_length = self.calculate_average_edge_length(simplified);
        
        if original_avg_edge_length > 0.0 {
            1.0 - (original_avg_edge_length - simplified_avg_edge_length).abs() / original_avg_edge_length
        } else {
            1.0
        }
    }
    
    /// Calculate average edge length of mesh
    fn calculate_average_edge_length(&self, mesh: &Mesh3D) -> f64 {
        let mut total_length = 0.0;
        let mut edge_count = 0;
        let mut visited_edges = HashSet::new();
        
        for face in &mesh.faces {
            for i in 0..face.vertices.len() {
                let v1 = face.vertices[i];
                let v2 = face.vertices[(i + 1) % face.vertices.len()];
                let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                
                if !visited_edges.contains(&edge) {
                    let p1 = &mesh.vertices[v1].point;
                    let p2 = &mesh.vertices[v2].point;
                    total_length += p1.distance(p2);
                    edge_count += 1;
                    visited_edges.insert(edge);
                }
            }
        }
        
        if edge_count > 0 {
            total_length / edge_count as f64
        } else {
            0.0
        }
    }

    /// Calculate bounding box
    fn calculate_bounding_box(&self, mesh: &Mesh3D) -> (Point, Point) {
        if mesh.vertices.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_x = mesh.vertices[0].point.x;
        let mut min_y = mesh.vertices[0].point.y;
        let mut min_z = mesh.vertices[0].point.z;
        let mut max_x = mesh.vertices[0].point.x;
        let mut max_y = mesh.vertices[0].point.y;
        let mut max_z = mesh.vertices[0].point.z;

        for vertex in &mesh.vertices {
            min_x = min_x.min(vertex.point.x);
            min_y = min_y.min(vertex.point.y);
            min_z = min_z.min(vertex.point.z);
            max_x = max_x.max(vertex.point.x);
            max_y = max_y.max(vertex.point.y);
            max_z = max_z.max(vertex.point.z);
        }

        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }
}

/// Extension methods for Mesh3D
pub trait MeshOptimizationExt {
    /// Simplify mesh
    fn simplify(&self, settings: &OptimizationSettings) -> AiResult<SimplificationResult>;

    /// Generate LODs
    fn generate_lods(&self, lod_settings: &LodSettings) -> AiResult<LodResult>;

    /// Optimize topology
    fn optimize_topology(&self) -> AiResult<Mesh3D>;
}

impl MeshOptimizationExt for Mesh3D {
    fn simplify(&self, settings: &OptimizationSettings) -> AiResult<SimplificationResult> {
        let optimizer = ModelOptimizer::new().with_settings((*settings).clone());
        optimizer.simplify(self)
    }

    fn generate_lods(&self, lod_settings: &LodSettings) -> AiResult<LodResult> {
        let optimizer = ModelOptimizer::new();
        optimizer.generate_lods(self, lod_settings)
    }

    fn optimize_topology(&self) -> AiResult<Mesh3D> {
        let optimizer = ModelOptimizer::new();
        optimizer.optimize_topology(self)
    }
}
