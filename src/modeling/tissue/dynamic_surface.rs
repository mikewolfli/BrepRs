use crate::foundation::{handle::Handle, StandardReal};
use crate::geometry::{Point, Transform, Vector};
use crate::topology::{TopoDsCompound, TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsVertex};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Vertex constraint type
#[derive(Debug, Clone, PartialEq)]
pub enum VertexConstraint {
    /// Fixed point constraint
    Fixed,
    /// Plane constraint
    Plane(TopoDsFace),
    /// Line constraint
    Line(TopoDsEdge),
    /// Surface constraint
    Surface(TopoDsFace),
}

/// FFD (Free Form Deformation) parameters
#[derive(Debug, Clone)]
pub struct FFD {
    /// Control points
    pub control_points: Vec<Point>,
    /// Resolution in u, v, w directions
    pub resolution: (usize, usize, usize),
    /// Bounding box
    pub bounding_box: (Point, Point),
}

/// Surface quality metrics
#[derive(Debug, Clone, Default)]
pub struct SurfaceQualityMetrics {
    /// Minimum edge length
    pub min_edge_length: StandardReal,
    /// Maximum edge length
    pub max_edge_length: StandardReal,
    /// Average edge length
    pub avg_edge_length: StandardReal,
    /// Minimum triangle aspect ratio
    pub min_aspect_ratio: StandardReal,
    /// Maximum triangle aspect ratio
    pub max_aspect_ratio: StandardReal,
    /// Average triangle aspect ratio
    pub avg_aspect_ratio: StandardReal,
    /// Number of degenerate faces
    pub degenerate_faces: usize,
    /// Number of flipped faces
    pub flipped_faces: usize,
}

/// Update parameters
#[derive(Debug, Clone, Default)]
pub struct UpdateParameters {
    /// Maximum displacement per iteration
    pub max_displacement: StandardReal,
    /// Smoothing factor
    pub smoothing_factor: StandardReal,
    /// Quality threshold for adaptive smoothing
    pub quality_threshold: StandardReal,
    /// Whether to preserve surface quality
    pub preserve_quality: bool,
    /// Whether to use FFD
    pub use_ffd: bool,
    /// Number of constraint solving iterations
    pub constraint_iterations: usize,
}

/// Dynamic surface that can update vertices while maintaining topology
#[derive(Debug, Clone)]
pub struct DynamicSurface {
    /// Original surface
    pub original_surface: TopoDsShape,
    /// Vertex positions
    pub vertices: Vec<Point>,
    /// Vertex indices map (original to current)
    pub vertex_map: HashMap<usize, usize>,
    /// Edge connectivity
    pub edges: Vec<(usize, usize)>,
    /// Face connectivity
    pub faces: Vec<Vec<usize>>,
    /// Vertex constraints
    pub constraints: Vec<VertexConstraint>,
    /// FFD for smooth deformation
    pub ffd: Option<FFD>,
    /// Surface normals for each face
    pub face_normals: Vec<Vector>,
    /// Vertex normals
    pub vertex_normals: Vec<Vector>,
    /// Quality metrics
    pub quality_metrics: SurfaceQualityMetrics,
}

impl DynamicSurface {
    /// Create a new dynamic surface from a TopoDsShape
    pub fn new(surface: TopoDsShape) -> Self {
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut faces = Vec::new();
        let mut vertex_map = HashMap::new();
        let mut vertex_index_map = HashMap::new();

        // Extract vertices, edges, and faces from the shape
        Self::extract_topology(
            &surface,
            &mut vertices,
            &mut edges,
            &mut faces,
            &mut vertex_map,
            &mut vertex_index_map,
        );

        // Compute initial normals
        let face_normals = Self::compute_face_normals(&vertices, &faces);
        let vertex_normals = Self::compute_vertex_normals(&vertices, &faces, &face_normals);

        // Compute initial quality metrics
        let quality_metrics = Self::compute_quality_metrics(&vertices, &faces);

        Self {
            original_surface: surface,
            vertices,
            vertex_map,
            edges,
            faces,
            constraints: Vec::new(),
            ffd: None,
            face_normals,
            vertex_normals,
            quality_metrics,
        }
    }

    /// Extract topology from a TopoDsShape
    pub fn extract_topology(
        shape: &TopoDsShape,
        vertices: &mut Vec<Point>,
        edges: &mut Vec<(usize, usize)>,
        faces: &mut Vec<Vec<usize>>,
        vertex_map: &mut HashMap<usize, usize>,
        vertex_index_map: &mut HashMap<Point, usize>,
    ) {
        // Real implementation: Extract actual topology from the shape
        // This implementation uses the shape's actual vertices, edges, and faces

        // Extract faces from the shape
        let shape_faces = shape.faces();

        for (face_idx, face) in shape_faces.iter().enumerate() {
            if let Some(face_ref) = face.get() {
                let mut face_vertices = Vec::new();

                // Extract actual vertices from the face
                let outer_wire = face_ref.outer_wire();
                if let Some(wire) = outer_wire {
                    if let Some(wire_ref) = wire.get() {
                        let wire_vertices = wire_ref.vertices();

                        for vertex in wire_vertices {
                            if let Some(vertex_ref) = vertex.get() {
                                let point = vertex_ref.point();
                                
                                // Get or create vertex index
                                let idx = if let Some(&idx) = vertex_index_map.get(point) {
                                    idx
                                } else {
                                    let idx = vertices.len();
                                    vertices.push(*point);
                                    vertex_index_map.insert(*point, idx);
                                    idx
                                };

                                if !face_vertices.contains(&idx) {
                                    face_vertices.push(idx);
                                }
                            }
                        }

                        // Extract edges from the wire
                        let wire_edges = wire_ref.edges();
                        for edge in wire_edges {
                            if let Some(edge_ref) = edge.get() {
                                let v1 = edge_ref.start_vertex();
                                let v2 = edge_ref.end_vertex();
                                
                                if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
                                    let p1 = v1_ref.point();
                                    let p2 = v2_ref.point();
                                    
                                    let idx1 = *vertex_index_map.get(p1).unwrap();
                                    let idx2 = *vertex_index_map.get(p2).unwrap();
                                    
                                    let edge = (idx1, idx2);
                                    if !edges.contains(&edge) && !edges.contains(&(edge.1, edge.0)) {
                                        edges.push(edge);
                                    }
                                }
                            }
                        }
                    }
                }

                // Add face if it has at least 3 vertices
                if face_vertices.len() >= 3 {
                    faces.push(face_vertices);
                    // Map original face index to current face index
                    vertex_map.insert(face_idx, faces.len() - 1);
                }
            }
        }
    }

    /// Compute face normals
    pub fn compute_face_normals(vertices: &[Point], faces: &[Vec<usize>]) -> Vec<Vector> {
        let mut normals = Vec::with_capacity(faces.len());

        for face in faces {
            if face.len() >= 3 {
                let v0 = vertices[face[0]];
                let v1 = vertices[face[1]];
                let v2 = vertices[face[2]];

                let vec1 = v1 - v0;
                let vec2 = v2 - v0;
                let normal = vec1.cross(&vec2).normalized();

                normals.push(normal);
            } else {
                normals.push(Vector::new(0.0, 0.0, 0.0));
            }
        }

        normals
    }

    /// Compute vertex normals by averaging adjacent face normals
    pub fn compute_vertex_normals(
        vertices: &[Point],
        faces: &[Vec<usize>],
        face_normals: &[Vector],
    ) -> Vec<Vector> {
        let mut vertex_normals = vec![Vector::new(0.0, 0.0, 0.0); vertices.len()];
        let mut vertex_face_counts = vec![0; vertices.len()];

        for (face_idx, face) in faces.iter().enumerate() {
            let face_normal = face_normals[face_idx];

            for &vertex_idx in face {
                vertex_normals[vertex_idx] = vertex_normals[vertex_idx] + face_normal;
                vertex_face_counts[vertex_idx] += 1;
            }
        }

        // Normalize vertex normals
        for (vertex_idx, normal) in vertex_normals.iter_mut().enumerate() {
            if vertex_face_counts[vertex_idx] > 0 {
                *normal = normal.normalized();
            }
        }

        vertex_normals
    }

    /// Compute surface quality metrics
    pub fn compute_quality_metrics(
        vertices: &[Point],
        faces: &[Vec<usize>],
    ) -> SurfaceQualityMetrics {
        let mut metrics = SurfaceQualityMetrics::default();

        if vertices.is_empty() || faces.is_empty() {
            return metrics;
        }

        let mut edge_lengths = Vec::new();
        let mut aspect_ratios = Vec::new();
        let mut degenerate_count = 0;
        let mut flipped_count = 0;

        for face in faces {
            if face.len() >= 3 {
                // Check for degenerate face
                let v0 = vertices[face[0]];
                let v1 = vertices[face[1]];
                let v2 = vertices[face[2]];

                let vec1 = v1 - v0;
                let vec2 = v2 - v0;
                let area = vec1.cross(&vec2).magnitude() / 2.0;

                if area < 1e-10 {
                    degenerate_count += 1;
                    continue;
                }

                // Check for flipped face
                let normal = vec1.cross(&vec2);
                if normal.z < 0.0 {
                    // Simple check for upward normal
                    flipped_count += 1;
                }

                // Compute edge lengths
                for i in 0..face.len() {
                    let j = (i + 1) % face.len();
                    let edge_length = (vertices[face[i]] - vertices[face[j]]).magnitude();
                    edge_lengths.push(edge_length);
                }

                // Compute aspect ratio (simplified)
                let edge1 = (v1 - v0).magnitude();
                let edge2 = (v2 - v1).magnitude();
                let edge3 = (v0 - v2).magnitude();
                let max_edge = edge1.max(edge2).max(edge3);
                let min_edge = edge1.min(edge2).min(edge3);
                let aspect_ratio = max_edge / min_edge;
                aspect_ratios.push(aspect_ratio);
            } else {
                degenerate_count += 1;
            }
        }

        // Update metrics
        if !edge_lengths.is_empty() {
            metrics.min_edge_length = *edge_lengths
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metrics.max_edge_length = *edge_lengths
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metrics.avg_edge_length =
                edge_lengths.iter().sum::<StandardReal>() / edge_lengths.len() as StandardReal;
        }

        if !aspect_ratios.is_empty() {
            metrics.min_aspect_ratio = *aspect_ratios
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metrics.max_aspect_ratio = *aspect_ratios
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            metrics.avg_aspect_ratio =
                aspect_ratios.iter().sum::<StandardReal>() / aspect_ratios.len() as StandardReal;
        }

        metrics.degenerate_faces = degenerate_count;
        metrics.flipped_faces = flipped_count;

        metrics
    }

    /// Apply vertex constraint
    pub fn apply_constraint(&self, vertex_idx: usize, new_position: &Point) -> Point {
        if vertex_idx >= self.constraints.len() {
            return *new_position;
        }

        match &self.constraints[vertex_idx] {
            VertexConstraint::Fixed => {
                // Keep original position
                self.vertices[vertex_idx]
            }
            VertexConstraint::Plane(plane) => {
                // Project to plane
                Self::project_to_plane(new_position, plane)
            }
            VertexConstraint::Line(line) => {
                // Project to line
                Self::project_to_line(new_position, line)
            }
            VertexConstraint::Surface(surface) => {
                // Project to surface
                Self::project_to_surface(new_position, surface)
            }
        }
    }

    /// Project point to plane
    pub fn project_to_plane(point: &Point, plane: &TopoDsFace) -> Point {
        // Simple plane projection
        if let Some(_surface_handle) = plane.surface() {
            if let Some(centroid) = plane.centroid() {
                let normal = Self::estimate_face_normal(plane);
                let vector = *point - centroid;
                let distance = vector.dot(&normal);
                let projected_vector = vector - (normal * distance);
                centroid + projected_vector
            } else {
                *point
            }
        } else {
            *point
        }
    }

    /// Project point to line
    pub fn project_to_line(point: &Point, line: &TopoDsEdge) -> Point {
        // Simple line projection
        let v1 = line.vertex1();
        let v2 = line.vertex2();
        let p1 = v1.point();
        let p2 = v2.point();

        let line_vec = *p2 - *p1;
        let point_vec = *point - *p1;

        let t = point_vec.dot(&line_vec) / line_vec.dot(&line_vec);
        let t_clamped = t.clamp(0.0, 1.0);

        let projected_vector = line_vec * t_clamped;
        *p1 + projected_vector
    }

    /// Project point to surface
    pub fn project_to_surface(point: &Point, surface: &TopoDsFace) -> Point {
        // Simple surface projection
        if let Some(_surface_handle) = surface.surface() {
            // For simplicity, use the centroid as the projection point
            if let Some(centroid) = surface.centroid() {
                centroid
            } else {
                *point
            }
        } else {
            *point
        }
    }

    /// Estimate face normal
    pub fn estimate_face_normal(face: &TopoDsFace) -> Vector {
        if let Some(outer_wire) = face.outer_wire() {
            let vertices = outer_wire.vertices();
            if vertices.len() >= 3 {
                let v0 = vertices[0].point();
                let v1 = vertices[1].point();
                let v2 = vertices[2].point();

                let vec1 = *v1 - *v0;
                let vec2 = *v2 - *v0;
                let normal = vec1.cross(&vec2).normalized();

                normal
            } else {
                Vector::new(0.0, 0.0, 1.0)
            }
        } else {
            Vector::new(0.0, 0.0, 1.0)
        }
    }

    /// Solve constraints iteratively
    pub fn solve_constraints(&mut self, params: &UpdateParameters) {
        for _ in 0..params.constraint_iterations {
            for (vertex_idx, constraint) in self.constraints.iter().enumerate() {
                if vertex_idx >= self.vertices.len() {
                    continue;
                }

                let current = self.vertices[vertex_idx];
                let new_position = match constraint {
                    VertexConstraint::Fixed => current,
                    VertexConstraint::Plane(plane) => Self::project_to_plane(&current, plane),
                    VertexConstraint::Line(line) => Self::project_to_line(&current, line),
                    VertexConstraint::Surface(surface) => {
                        Self::project_to_surface(&current, surface)
                    }
                };

                self.vertices[vertex_idx] = new_position;
            }
        }
    }

    /// Smooth the surface
    pub fn smooth_surface(&mut self, factor: StandardReal) {
        let mut new_vertices = self.vertices.clone();

        for (vertex_idx, vertex) in self.vertices.iter().enumerate() {
            // Find adjacent vertices
            let mut adjacent_vertices = Vec::new();

            for &(v1, v2) in &self.edges {
                if v1 == vertex_idx {
                    adjacent_vertices.push(v2);
                } else if v2 == vertex_idx {
                    adjacent_vertices.push(v1);
                }
            }

            if !adjacent_vertices.is_empty() {
                // Compute average position
                let mut avg_position = Point::new(0.0, 0.0, 0.0);
                for &adj_idx in &adjacent_vertices {
                    avg_position += self.vertices[adj_idx];
                }
                avg_position.x /= adjacent_vertices.len() as StandardReal;
                avg_position.y /= adjacent_vertices.len() as StandardReal;
                avg_position.z /= adjacent_vertices.len() as StandardReal;

                // Move vertex towards average
                let displacement = avg_position - *vertex;
                new_vertices[vertex_idx] = *vertex + displacement * factor;
            }
        }

        self.vertices = new_vertices;
    }

    /// Smooth surface with adaptive quality preservation
    pub fn smooth_surface_adaptive(
        &mut self,
        factor: StandardReal,
        quality_threshold: StandardReal,
    ) {
        let mut new_vertices = self.vertices.clone();

        for (vertex_idx, vertex) in self.vertices.iter().enumerate() {
            // Find adjacent vertices
            let mut adjacent_vertices = Vec::new();

            for &(v1, v2) in &self.edges {
                if v1 == vertex_idx {
                    adjacent_vertices.push(v2);
                } else if v2 == vertex_idx {
                    adjacent_vertices.push(v1);
                }
            }

            if !adjacent_vertices.is_empty() {
                // Compute average position
                let mut avg_position = Point::new(0.0, 0.0, 0.0);
                for &adj_idx in &adjacent_vertices {
                    avg_position += self.vertices[adj_idx];
                }
                avg_position.x /= adjacent_vertices.len() as StandardReal;
                avg_position.y /= adjacent_vertices.len() as StandardReal;
                avg_position.z /= adjacent_vertices.len() as StandardReal;

                // Calculate current quality around this vertex
                let local_quality = self.calculate_local_quality(vertex_idx);

                // Adjust smoothing factor based on quality
                let adaptive_factor = if local_quality < quality_threshold {
                    factor * 0.5 // Reduce smoothing for low-quality areas
                } else {
                    factor
                };

                // Move vertex towards average
                let displacement = avg_position - *vertex;
                new_vertices[vertex_idx] = *vertex + displacement * adaptive_factor;
            }
        }

        self.vertices = new_vertices;
    }

    /// Calculate local quality around a vertex
    pub fn calculate_local_quality(&self, vertex_idx: usize) -> StandardReal {
        let mut adjacent_faces = Vec::new();

        // Find adjacent faces
        for (face_idx, face) in self.faces.iter().enumerate() {
            if face.contains(&vertex_idx) {
                adjacent_faces.push(face_idx);
            }
        }

        if adjacent_faces.is_empty() {
            return 1.0;
        }

        // Calculate average aspect ratio of adjacent faces
        let mut aspect_ratios = Vec::new();

        for &face_idx in &adjacent_faces {
            let face = &self.faces[face_idx];
            if face.len() >= 3 {
                let v0 = self.vertices[face[0]];
                let v1 = self.vertices[face[1]];
                let v2 = self.vertices[face[2]];

                let edge1 = (v1 - v0).magnitude();
                let edge2 = (v2 - v1).magnitude();
                let edge3 = (v0 - v2).magnitude();

                let max_edge = edge1.max(edge2).max(edge3);
                let min_edge = edge1.min(edge2).min(edge3);
                let aspect_ratio = max_edge / min_edge;

                aspect_ratios.push(aspect_ratio);
            }
        }

        if aspect_ratios.is_empty() {
            return 1.0;
        }

        // Return inverse of average aspect ratio (higher is better)
        let avg_aspect_ratio =
            aspect_ratios.iter().sum::<StandardReal>() / aspect_ratios.len() as StandardReal;
        1.0 / avg_aspect_ratio
    }

    /// Apply FFD deformation
    pub fn apply_ffd(&mut self) {
        if let Some(ffd) = &self.ffd {
            for (_vertex_idx, vertex) in self.vertices.iter_mut().enumerate() {
                // Map vertex to FFD parameter space
                let (u, v, w) = Self::map_to_ffd_space(vertex, &ffd);

                // Evaluate FFD at parameters
                let deformed = Self::evaluate_ffd(u, v, w, &ffd);

                *vertex = deformed;
            }
        }
    }

    /// Map point to FFD parameter space
    pub fn map_to_ffd_space(
        point: &Point,
        ffd: &FFD,
    ) -> (StandardReal, StandardReal, StandardReal) {
        let (min, max) = ffd.bounding_box;
        let u = (point.x - min.x) / (max.x - min.x);
        let v = (point.y - min.y) / (max.y - min.y);
        let w = (point.z - min.z) / (max.z - min.z);

        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0), w.clamp(0.0, 1.0))
    }

    /// Evaluate FFD at parameters
    pub fn evaluate_ffd(u: StandardReal, v: StandardReal, w: StandardReal, ffd: &FFD) -> Point {
        let (res_u, res_v, res_w) = ffd.resolution;
        let mut result = Point::new(0.0, 0.0, 0.0);

        // Simple linear interpolation for demonstration
        for i in 0..res_u {
            for j in 0..res_v {
                for k in 0..res_w {
                    let weight = Self::ffd_weight(u, i, res_u)
                        * Self::ffd_weight(v, j, res_v)
                        * Self::ffd_weight(w, k, res_w);

                    let idx = i * res_v * res_w + j * res_w + k;
                    if idx < ffd.control_points.len() {
                        let cp = ffd.control_points[idx];
                        result.x += cp.x * weight;
                        result.y += cp.y * weight;
                        result.z += cp.z * weight;
                    }
                }
            }
        }

        result
    }

    /// FFD weight function
    pub fn ffd_weight(t: StandardReal, i: usize, resolution: usize) -> StandardReal {
        // Simple linear weight
        let step = 1.0 / (resolution - 1) as StandardReal;
        let pos = i as StandardReal * step;
        let distance = (t - pos).abs();
        (1.0 - distance / step).max(0.0)
    }

    /// Update vertex positions with full constraint solving
    pub fn update_vertices(&mut self, new_positions: &[(usize, Point)], params: &UpdateParameters) {
        // Apply new positions with constraints
        for (idx, new_pos) in new_positions {
            if *idx < self.vertices.len() {
                // Apply constraint
                let constrained_pos = self.apply_constraint(*idx, new_pos);

                // Limit displacement
                let current_pos = self.vertices[*idx];
                let displacement = constrained_pos - current_pos;
                let displacement_mag = displacement.magnitude();

                if displacement_mag > params.max_displacement {
                    let scaled_displacement = displacement.normalized() * params.max_displacement;
                    self.vertices[*idx] = current_pos + scaled_displacement;
                } else {
                    self.vertices[*idx] = constrained_pos;
                }
            }
        }

        // Iterative constraint solving for better accuracy
        self.solve_constraints(params);

        // Apply smoothing
        if params.smoothing_factor > 0.0 {
            if params.preserve_quality {
                self.smooth_surface_adaptive(params.smoothing_factor, params.quality_threshold);
            } else {
                self.smooth_surface(params.smoothing_factor);
            }
        }

        // Apply FFD if enabled
        if params.use_ffd && self.ffd.is_some() {
            self.apply_ffd();
        }

        // Update normals
        self.update_normals();

        // Update quality metrics
        self.quality_metrics = Self::compute_quality_metrics(&self.vertices, &self.faces);
    }

    /// Update normals
    pub fn update_normals(&mut self) {
        self.face_normals = Self::compute_face_normals(&self.vertices, &self.faces);
        self.vertex_normals =
            Self::compute_vertex_normals(&self.vertices, &self.faces, &self.face_normals);
    }

    /// Get surface point at UV coordinates
    pub fn point_at(&self, u: StandardReal, v: StandardReal) -> Point {
        // Simple UV mapping for demonstration
        if self.vertices.is_empty() {
            return Point::origin();
        }

        // Map UV to vertex indices
        let u_clamped = u.clamp(0.0, 1.0);
        let v_clamped = v.clamp(0.0, 1.0);

        let vertex_count = self.vertices.len();
        let idx1 = (u_clamped * (vertex_count - 1) as StandardReal) as usize;
        let idx2 = ((u_clamped + 0.1) * (vertex_count - 1) as StandardReal) as usize % vertex_count;

        let weight = v_clamped;
        let p1 = self.vertices[idx1];
        let p2 = self.vertices[idx2];

        Point::new(
            p1.x * (1.0 - weight) + p2.x * weight,
            p1.y * (1.0 - weight) + p2.y * weight,
            p1.z * (1.0 - weight) + p2.z * weight,
        )
    }

    /// Get surface normal at UV coordinates
    pub fn normal_at(&self, u: StandardReal, v: StandardReal) -> Vector {
        // Simple UV mapping for demonstration
        if self.vertex_normals.is_empty() {
            return Vector::new(0.0, 0.0, 1.0);
        }

        // Map UV to vertex indices
        let u_clamped = u.clamp(0.0, 1.0);
        let v_clamped = v.clamp(0.0, 1.0);

        let vertex_count = self.vertex_normals.len();
        let idx1 = (u_clamped * (vertex_count - 1) as StandardReal) as usize;
        let idx2 = ((u_clamped + 0.1) * (vertex_count - 1) as StandardReal) as usize % vertex_count;

        let weight = v_clamped;
        let n1 = self.vertex_normals[idx1];
        let n2 = self.vertex_normals[idx2];

        Vector::new(
            n1.x * (1.0 - weight) + n2.x * weight,
            n1.y * (1.0 - weight) + n2.y * weight,
            n1.z * (1.0 - weight) + n2.z * weight,
        )
        .normalized()
    }

    /// Get bounding box
    pub fn bounding_box(&self) -> (Point, Point) {
        if self.vertices.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_point = Point::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max_point = Point::new(f64::MIN, f64::MIN, f64::MIN);

        for vertex in &self.vertices {
            min_point.x = min_point.x.min(vertex.x);
            min_point.y = min_point.y.min(vertex.y);
            min_point.z = min_point.z.min(vertex.z);
            max_point.x = max_point.x.max(vertex.x);
            max_point.y = max_point.y.max(vertex.y);
            max_point.z = max_point.z.max(vertex.z);
        }

        (min_point, max_point)
    }

    /// Check if surface is valid
    pub fn is_valid(&self) -> bool {
        // Check for degenerate faces
        if self.quality_metrics.degenerate_faces > self.faces.len() / 2 {
            return false;
        }

        // Check for NaN or infinite values
        for vertex in &self.vertices {
            if !vertex.x.is_finite() || !vertex.y.is_finite() || !vertex.z.is_finite() {
                return false;
            }
        }

        true
    }

    /// Get surface area
    pub fn surface_area(&self) -> StandardReal {
        let mut area = 0.0;

        for face in &self.faces {
            if face.len() >= 3 {
                let v0 = self.vertices[face[0]];
                let v1 = self.vertices[face[1]];
                let v2 = self.vertices[face[2]];

                let vec1 = v1 - v0;
                let vec2 = v2 - v0;
                let face_area = vec1.cross(&vec2).magnitude() / 2.0;
                area += face_area;
            }
        }

        area
    }

    /// Get volume (for closed surfaces)
    pub fn volume(&self) -> StandardReal {
        let mut volume = 0.0;

        for face in &self.faces {
            if face.len() >= 3 {
                let v0 = self.vertices[face[0]];
                let v1 = self.vertices[face[1]];
                let v2 = self.vertices[face[2]];

                // Shoelace formula for volume
                volume += (v0.x * v1.y * v2.z + v1.x * v2.y * v0.z + v2.x * v0.y * v1.z
                    - v0.z * v1.y * v2.x
                    - v1.z * v2.y * v0.x
                    - v2.z * v0.y * v1.x)
                    / 6.0;
            }
        }

        volume.abs()
    }

    /// Apply transform to surface
    pub fn transform(&mut self, transform: &Transform) {
        for vertex in &mut self.vertices {
            *vertex = vertex.transformed(transform);
        }

        // Update normals
        self.update_normals();

        // Update quality metrics
        self.quality_metrics = Self::compute_quality_metrics(&self.vertices, &self.faces);
    }

    /// Set FFD for the surface
    pub fn set_ffd(&mut self, ffd: FFD) {
        self.ffd = Some(ffd);
    }

    /// Add vertex constraint
    pub fn add_constraint(&mut self, vertex_idx: usize, constraint: VertexConstraint) {
        if vertex_idx >= self.constraints.len() {
            self.constraints
                .resize(vertex_idx + 1, VertexConstraint::Fixed);
        }
        self.constraints[vertex_idx] = constraint;
    }

    /// Remove vertex constraint
    pub fn remove_constraint(&mut self, vertex_idx: usize) {
        if vertex_idx < self.constraints.len() {
            self.constraints[vertex_idx] = VertexConstraint::Fixed;
        }
    }

    /// Clear all constraints
    pub fn clear_constraints(&mut self) {
        self.constraints.clear();
    }

    /// Get vertex constraints
    pub fn get_constraints(&self) -> &[VertexConstraint] {
        &self.constraints
    }

    /// Get edge count
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Get face count
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Get vertex count
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Export surface to TopoDsShape
    pub fn to_shape(&self) -> TopoDsShape {
        let mut compound = TopoDsCompound::new();

        // Create faces from vertices
        for face in &self.faces {
            if face.len() >= 3 {
                // Create vertices
                let mut face_vertices = Vec::new();
                for &vertex_idx in face {
                    let point = self.vertices[vertex_idx];
                    let vertex = TopoDsVertex::new(point);
                    face_vertices.push(Handle::new(Arc::new(vertex)));
                }

                // Create edges
                let mut face_edges = Vec::new();
                for i in 0..face.len() {
                    let j = (i + 1) % face.len();
                    let edge = TopoDsEdge::new(face_vertices[i].clone(), face_vertices[j].clone());
                    face_edges.push(Handle::new(Arc::new(edge)));
                }

                // Create wire
                let mut wire = crate::topology::topods_wire::TopoDsWire::new();
                for edge in face_edges {
                    wire.add_edge(edge);
                }

                // Create face
                let face = TopoDsFace::with_outer_wire(wire);
                let face_shape = face.shape();
                compound.add_component(Handle::new(Arc::new(face_shape.clone())));
            }
        }

        compound.shape().clone()
    }

    /// Check surface topology stability
    pub fn check_topology_stability(&self) -> bool {
        // Check for edge consistency
        let mut edge_set = HashSet::new();
        for &(v1, v2) in &self.edges {
            let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            if !edge_set.insert(edge) {
                // Duplicate edge
                return false;
            }
        }

        // Check for face consistency
        for face in &self.faces {
            if face.len() < 3 {
                // Invalid face
                return false;
            }

            // Check for duplicate vertices in face
            let mut vertex_set = HashSet::new();
            for &vertex_idx in face {
                if !vertex_set.insert(vertex_idx) {
                    // Duplicate vertex in face
                    return false;
                }
            }
        }

        true
    }

    /// Remesh surface to improve quality
    pub fn remesh(&mut self, target_edge_length: StandardReal) {
        // Real implementation: Proper remeshing algorithm
        // 1. Split long edges
        // 2. Collapse short edges
        // 3. Update face connectivity
        
        // Step 1: Split long edges
        let mut new_vertices = self.vertices.clone();
        let mut new_edges = self.edges.clone();
        let mut new_faces = self.faces.clone();

        // Split long edges
        let mut edges_to_add: Vec<(usize, usize)> = Vec::new();
        let mut edges_to_remove: Vec<usize> = Vec::new();
        let mut face_updates: HashMap<usize, Vec<usize>> = HashMap::new();

        for (edge_idx, &(v1, v2)) in self.edges.iter().enumerate() {
            let edge_length = (self.vertices[v1] - self.vertices[v2]).magnitude();
            if edge_length > target_edge_length * 1.5 {
                // Split edge
                let midpoint = Point::new(
                    (self.vertices[v1].x + self.vertices[v2].x) / 2.0,
                    (self.vertices[v1].y + self.vertices[v2].y) / 2.0,
                    (self.vertices[v1].z + self.vertices[v2].z) / 2.0,
                );

                let new_vertex_idx = new_vertices.len();
                new_vertices.push(midpoint);

                edges_to_remove.push(edge_idx);
                edges_to_add.push((v1, new_vertex_idx));
                edges_to_add.push((new_vertex_idx, v2));

                // Update faces that contain this edge
                for (face_idx, face) in self.faces.iter().enumerate() {
                    if let Some(pos) = face.iter().position(|&vid| vid == v1) {
                        let next_pos = (pos + 1) % face.len();
                        if face[next_pos] == v2 {
                            let mut updated_face = face.clone();
                            updated_face.insert(next_pos, new_vertex_idx);
                            face_updates.insert(face_idx, updated_face);
                        }
                    }
                    if let Some(pos) = face.iter().position(|&vid| vid == v2) {
                        let next_pos = (pos + 1) % face.len();
                        if face[next_pos] == v1 {
                            let mut updated_face = face.clone();
                            updated_face.insert(next_pos, new_vertex_idx);
                            face_updates.insert(face_idx, updated_face);
                        }
                    }
                }
            }
        }

        // Update faces
        for (face_idx, updated_face) in face_updates {
            if face_idx < new_faces.len() {
                new_faces[face_idx] = updated_face;
            }
        }

        // Remove old edges and add new ones
        let mut filtered_edges = Vec::new();
        for (i, edge) in new_edges.iter().enumerate() {
            if !edges_to_remove.contains(&i) {
                filtered_edges.push(*edge);
            }
        }
        filtered_edges.extend(edges_to_add);
        new_edges = filtered_edges;

        // Step 2: Collapse short edges
        let mut edges_to_collapse: Vec<usize> = Vec::new();
        let mut vertex_map: HashMap<usize, usize> = HashMap::new();

        for (edge_idx, &(v1, v2)) in new_edges.iter().enumerate() {
            let edge_length = (new_vertices[v1] - new_vertices[v2]).magnitude();
            if edge_length < target_edge_length * 0.5 {
                edges_to_collapse.push(edge_idx);
                // Map v2 to v1
                vertex_map.insert(v2, v1);
            }
        }

        // Update vertices, edges, and faces after collapse
        let mut collapsed_vertices = new_vertices.clone();
        let mut collapsed_edges = Vec::new();
        let mut collapsed_faces = Vec::new();

        // Update edges
        for (i, &(v1, v2)) in new_edges.iter().enumerate() {
            if !edges_to_collapse.contains(&i) {
                let new_v1 = *vertex_map.get(&v1).unwrap_or(&v1);
                let new_v2 = *vertex_map.get(&v2).unwrap_or(&v2);
                if new_v1 != new_v2 {
                    collapsed_edges.push((new_v1, new_v2));
                }
            }
        }

        // Update faces
        for face in new_faces {
            let mut new_face = Vec::new();
            let mut seen = HashSet::new();
            for &vid in &face {
                let new_vid = *vertex_map.get(&vid).unwrap_or(&vid);
                if seen.insert(new_vid) {
                    new_face.push(new_vid);
                }
            }
            if new_face.len() >= 3 {
                collapsed_faces.push(new_face);
            }
        }

        // Step 3: Optimize face quality
        self.optimize_face_quality(&mut collapsed_vertices, &mut collapsed_faces, target_edge_length);

        // Update surface
        self.vertices = collapsed_vertices;
        self.edges = collapsed_edges;
        self.faces = collapsed_faces;

        // Update normals and quality metrics
        self.update_normals();
        self.quality_metrics = Self::compute_quality_metrics(&self.vertices, &self.faces);
    }
    
    /// Optimize face quality by adjusting vertex positions
    fn optimize_face_quality(&self, vertices: &mut Vec<Point>, faces: &mut Vec<Vec<usize>>, _target_edge_length: StandardReal) {
        // Simple Laplacian smoothing to improve face quality
        let mut new_vertices = vertices.clone();
        
        for (vertex_idx, _) in vertices.iter().enumerate() {
            // Find adjacent vertices
            let mut adjacent_vertices = Vec::new();
            
            for face in faces.iter() {
                if face.contains(&vertex_idx) {
                    for vid in face {
                        if *vid != vertex_idx {
                            adjacent_vertices.push(*vid);
                        }
                    }
                }
            }
            
            if !adjacent_vertices.is_empty() {
                // Compute average position
                let mut avg_position = Point::new(0.0, 0.0, 0.0);
                for &adj_idx in &adjacent_vertices {
                    avg_position += vertices[adj_idx];
                }
                avg_position.x /= adjacent_vertices.len() as StandardReal;
                avg_position.y /= adjacent_vertices.len() as StandardReal;
                avg_position.z /= adjacent_vertices.len() as StandardReal;
                
                // Move vertex towards average with a small factor
                let current = vertices[vertex_idx];
                let displacement = avg_position - current;
                new_vertices[vertex_idx] = current + displacement * 0.1;
            }
        }
        
        *vertices = new_vertices;
    }

    /// Merge close vertices
    pub fn merge_close_vertices(&mut self, tolerance: StandardReal) {
        // let _vertex_map: HashMap<usize, usize> = HashMap::new();
        let mut new_vertices: Vec<Point> = Vec::new();
        let mut new_vertex_indices = vec![0; self.vertices.len()];

        for (vertex_idx, vertex) in self.vertices.iter().enumerate() {
            let mut found = false;
            for (new_idx, new_vertex) in new_vertices.iter().enumerate() {
                if (vertex.x - new_vertex.x).abs() < tolerance
                    && (vertex.y - new_vertex.y).abs() < tolerance
                    && (vertex.z - new_vertex.z).abs() < tolerance
                {
                    new_vertex_indices[vertex_idx] = new_idx;
                    found = true;
                    break;
                }
            }

            if !found {
                new_vertex_indices[vertex_idx] = new_vertices.len();
                new_vertices.push(*vertex);
            }
        }

        // Update edges
        let mut new_edges = Vec::new();
        let mut edge_set = HashSet::new();

        for &(v1, v2) in &self.edges {
            let new_v1 = new_vertex_indices[v1];
            let new_v2 = new_vertex_indices[v2];

            if new_v1 != new_v2 {
                let edge = if new_v1 < new_v2 {
                    (new_v1, new_v2)
                } else {
                    (new_v2, new_v1)
                };

                if edge_set.insert(edge) {
                    new_edges.push(edge);
                }
            }
        }

        // Update faces
        let mut new_faces = Vec::new();

        for face in &self.faces {
            let mut new_face = Vec::new();
            let mut vertex_set = HashSet::new();

            for &vertex_idx in face {
                let new_vertex_idx = new_vertex_indices[vertex_idx];
                if vertex_set.insert(new_vertex_idx) {
                    new_face.push(new_vertex_idx);
                }
            }

            if new_face.len() >= 3 {
                new_faces.push(new_face);
            }
        }

        self.vertices = new_vertices;
        self.edges = new_edges;
        self.faces = new_faces;

        // Update normals and quality metrics
        self.update_normals();
        self.quality_metrics = Self::compute_quality_metrics(&self.vertices, &self.faces);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::topology::TopoDsShape;

    #[test]
    fn test_dynamic_surface_creation() {
        // Create a simple shape for testing
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);

        let surface = DynamicSurface::new(shape);

        assert!(!surface.vertices.is_empty());
        assert!(!surface.edges.is_empty());
        assert!(!surface.faces.is_empty());
        assert!(surface.is_valid());
    }

    #[test]
    fn test_surface_update() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let mut surface = DynamicSurface::new(shape);

        // Update a vertex
        let original_position = surface.vertices[0];
        let new_position = Point::new(0.5, 0.5, 0.5);

        let update_params = UpdateParameters::default();
        surface.update_vertices(&[(0, new_position)], &update_params);

        assert_ne!(surface.vertices[0], original_position);
        assert!(surface.is_valid());
    }

    #[test]
    fn test_surface_smoothing() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let mut surface = DynamicSurface::new(shape);
        let original_vertices = surface.vertices.clone();

        // Apply smoothing
        surface.smooth_surface(0.1);

        assert_ne!(surface.vertices, original_vertices);
        assert!(surface.is_valid());
    }

    #[test]
    fn test_surface_quality() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let surface = DynamicSurface::new(shape);

        assert!(surface.quality_metrics.degenerate_faces == 0);
        assert!(surface.quality_metrics.flipped_faces == 0);
        assert!(surface.quality_metrics.min_edge_length > 0.0);
    }

    #[test]
    fn test_bounding_box() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let surface = DynamicSurface::new(shape);
        let (min, max) = surface.bounding_box();

        assert!(min.x <= 0.5);
        assert!(min.y <= 0.5);
        assert!(min.z <= 0.0);
        assert!(max.x >= -0.5);
        assert!(max.y >= -0.5);
        assert!(max.z >= 0.0);
    }

    #[test]
    fn test_surface_area() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let surface = DynamicSurface::new(shape);
        let area = surface.surface_area();

        assert!(area > 0.0);
    }

    #[test]
    fn test_topology_stability() {
        let shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound);
        let surface = DynamicSurface::new(shape);
        assert!(surface.check_topology_stability());
    }
}
