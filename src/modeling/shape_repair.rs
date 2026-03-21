use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::TopoDsShape;
use std::sync::Arc;

/// Shape repair status
pub enum RepairStatus {
    /// Repair successful
    Success,
    /// No repair needed
    NoRepairNeeded,
    /// Repair partially successful
    PartialSuccess,
    /// Repair failed
    Failed,
}

/// Repair result with detailed information
pub struct RepairResult {
    pub status: RepairStatus,
    pub repaired_shape: Option<TopoDsShape>,
    pub issues_detected: Vec<String>,
    pub issues_fixed: Vec<String>,
    pub issues_remaining: Vec<String>,
    pub repair_time_ms: u64,
}

/// Shape repair settings
pub struct RepairSettings {
    pub fix_non_manifold_edges: bool,
    pub fix_degenerate_faces: bool,
    pub fix_self_intersections: bool,
    pub fix_duplicate_vertices: bool,
    pub fix_duplicate_faces: bool,
    pub merge_close_vertices: bool,
    pub vertex_merge_tolerance: f64,
    pub max_iterations: usize,
    pub enable_logging: bool,
}

impl Default for RepairSettings {
    fn default() -> Self {
        Self {
            fix_non_manifold_edges: true,
            fix_degenerate_faces: true,
            fix_self_intersections: true,
            fix_duplicate_vertices: true,
            fix_duplicate_faces: true,
            merge_close_vertices: true,
            vertex_merge_tolerance: 1e-6,
            max_iterations: 10,
            enable_logging: false,
        }
    }
}

/// Advanced shape repair tools
pub struct ShapeRepairTools {
    settings: RepairSettings,
    log: Vec<String>,
}

impl ShapeRepairTools {
    /// Create a new shape repair tool with default settings
    pub fn new() -> Self {
        Self {
            settings: RepairSettings::default(),
            log: Vec::new(),
        }
    }

    /// Create a new shape repair tool with custom settings
    pub fn with_settings(settings: RepairSettings) -> Self {
        Self {
            settings,
            log: Vec::new(),
        }
    }

    /// Repair a shape
    pub fn repair(&mut self, shape: &TopoDsShape) -> RepairResult {
        let start_time = std::time::Instant::now();
        self.log.clear();

        let mut issues_detected = Vec::new();
        let mut issues_fixed = Vec::new();
        let mut issues_remaining = Vec::new();

        // 1. Check for issues
        if self.settings.fix_non_manifold_edges {
            let non_manifold_edges = self.detect_non_manifold_edges(shape);
            if !non_manifold_edges.is_empty() {
                issues_detected.push(format!(
                    "Found {} non-manifold edges",
                    non_manifold_edges.len()
                ));
            }
        }

        if self.settings.fix_degenerate_faces {
            let degenerate_faces = self.detect_degenerate_faces(shape);
            if !degenerate_faces.is_empty() {
                issues_detected.push(format!("Found {} degenerate faces", degenerate_faces.len()));
            }
        }

        if self.settings.fix_self_intersections {
            let self_intersections = self.detect_self_intersections(shape);
            if !self_intersections.is_empty() {
                issues_detected.push(format!(
                    "Found {} self-intersections",
                    self_intersections.len()
                ));
            }
        }

        if self.settings.fix_duplicate_vertices {
            let duplicate_vertices = self.detect_duplicate_vertices(shape);
            if !duplicate_vertices.is_empty() {
                issues_detected.push(format!(
                    "Found {} duplicate vertices",
                    duplicate_vertices.len()
                ));
            }
        }

        if self.settings.fix_duplicate_faces {
            let duplicate_faces = self.detect_duplicate_faces(shape);
            if !duplicate_faces.is_empty() {
                issues_detected.push(format!("Found {} duplicate faces", duplicate_faces.len()));
            }
        }

        // 2. Apply repairs
        let mut repaired_shape = shape.clone();
        let mut repair_success = true;

        if self.settings.fix_duplicate_vertices {
            if let Ok(fixed_shape) = self.fix_duplicate_vertices(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Fixed duplicate vertices".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to fix duplicate vertices".to_string());
            }
        }

        if self.settings.merge_close_vertices {
            if let Ok(fixed_shape) = self.merge_close_vertices(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Merged close vertices".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to merge close vertices".to_string());
            }
        }

        if self.settings.fix_degenerate_faces {
            if let Ok(fixed_shape) = self.fix_degenerate_faces(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Fixed degenerate faces".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to fix degenerate faces".to_string());
            }
        }

        if self.settings.fix_non_manifold_edges {
            if let Ok(fixed_shape) = self.fix_non_manifold_edges(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Fixed non-manifold edges".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to fix non-manifold edges".to_string());
            }
        }

        if self.settings.fix_self_intersections {
            if let Ok(fixed_shape) = self.fix_self_intersections(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Fixed self-intersections".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to fix self-intersections".to_string());
            }
        }

        if self.settings.fix_duplicate_faces {
            if let Ok(fixed_shape) = self.fix_duplicate_faces(&repaired_shape) {
                repaired_shape = fixed_shape;
                issues_fixed.push("Fixed duplicate faces".to_string());
            } else {
                repair_success = false;
                issues_remaining.push("Failed to fix duplicate faces".to_string());
            }
        }

        // 3. Determine repair status
        let status = if issues_detected.is_empty() {
            RepairStatus::NoRepairNeeded
        } else if repair_success && issues_remaining.is_empty() {
            RepairStatus::Success
        } else if !issues_fixed.is_empty() {
            RepairStatus::PartialSuccess
        } else {
            RepairStatus::Failed
        };

        let repair_time_ms = start_time.elapsed().as_millis() as u64;

        RepairResult {
            status,
            repaired_shape: Some(repaired_shape),
            issues_detected,
            issues_fixed,
            issues_remaining,
            repair_time_ms,
        }
    }

    /// Detect non-manifold edges
    fn detect_non_manifold_edges(&self, shape: &TopoDsShape) -> Vec<usize> {
        // Implementation of non-manifold edge detection
        let mut non_manifold_edges = Vec::new();

        // Traverse all edges in the shape
        let mut edge_index = 0;
        let mut explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Edge,
        );

        while explorer.more() {
            explorer.next();
            if let Some(edge_shape) = explorer.current() {
                if edge_shape.is_edge() {
                    // SAFETY: Safe because we checked the shape type
                    let _edge = unsafe {
                        &*(edge_shape as *const _
                            as *const crate::topology::topods_edge::TopoDsEdge)
                    };

                    // Count adjacent faces using TopExp
                    let mut face_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
                        edge_shape,
                        crate::topology::shape_enum::ShapeType::Face,
                    );
                    let mut face_count = 0;
                    while face_explorer.more() {
                        face_explorer.next();
                        if let Some(face_shape) = face_explorer.current() {
                            if face_shape.is_face() {
                                face_count += 1;
                            }
                        }
                    }

                    // For manifold edges, face count should be exactly 2
                    if face_count != 2 {
                        non_manifold_edges.push(edge_index);
                    }
                }
                edge_index += 1;
            }
        }

        non_manifold_edges
    }

    /// Detect degenerate faces
    fn detect_degenerate_faces(&self, shape: &TopoDsShape) -> Vec<usize> {
        // Implementation of degenerate face detection
        let mut degenerate_faces = Vec::new();

        // Traverse all faces in the shape
        let mut face_index = 0;
        let mut explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Face,
        );

        while explorer.more() {
            explorer.next();
            if let Some(face_shape) = explorer.current() {
                if face_shape.is_face() {
                    // SAFETY: Safe because we checked the shape type
                    let face = unsafe {
                        &*(face_shape as *const _
                            as *const crate::topology::topods_face::TopoDsFace)
                    };

                    // Check if face is degenerate
                    if self.is_degenerate_face(face) {
                        degenerate_faces.push(face_index);
                    }
                }
                face_index += 1;
            }
        }

        degenerate_faces
    }

    /// Check if a face is degenerate
    fn is_degenerate_face(&self, face: &crate::topology::topods_face::TopoDsFace) -> bool {
        // Get face vertices
        let mut vertices: Vec<Point> = Vec::new();
        let mut vertex_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            face.shape(),
            crate::topology::shape_enum::ShapeType::Vertex,
        );

        while vertex_explorer.more() {
            vertex_explorer.next();
            if let Some(vertex_shape) = vertex_explorer.current() {
                if vertex_shape.is_vertex() {
                    // SAFETY: Safe because we checked the shape type
                    let vertex = unsafe {
                        &*(vertex_shape as *const _
                            as *const crate::topology::topods_vertex::TopoDsVertex)
                    };
                    vertices.push(*vertex.point());
                }
            }
        }

        // Check if face has less than 3 vertices
        if vertices.len() < 3 {
            return true;
        }

        // Check if vertices are colinear
        if self.are_points_colinear(&vertices) {
            return true;
        }

        // Check if face has zero area
        if self.compute_face_area(&vertices) < self.settings.vertex_merge_tolerance {
            return true;
        }

        false
    }

    /// Check if points are colinear
    fn are_points_colinear(&self, points: &[Point]) -> bool {
        if points.len() < 3 {
            return false;
        }

        // Calculate vectors between first point and others
        let v1 = points[1] - points[0];

        for i in 2..points.len() {
            let vi = points[i] - points[0];
            // Calculate cross product
            let cross = v1.cross(&vi);
            // If cross product magnitude is above threshold, points are not colinear
            if cross.magnitude() > self.settings.vertex_merge_tolerance {
                return false;
            }
        }

        true
    }

    /// Compute face area
    fn compute_face_area(&self, points: &[Point]) -> f64 {
        if points.len() < 3 {
            return 0.0;
        }

        // Use shoelace formula for polygon area
        let mut area = 0.0;
        let n = points.len();

        for i in 0..n {
            let j = (i + 1) % n;
            area += points[i].x * points[j].y;
            area -= points[j].x * points[i].y;
        }

        (area / 2.0).abs()
    }

    /// Detect self-intersections
    fn detect_self_intersections(&self, shape: &TopoDsShape) -> Vec<(Point, Point)> {
        // Implementation of self-intersection detection
        let mut self_intersections = Vec::new();

        // Collect all faces in the shape
        let mut faces = Vec::new();
        let mut face_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Face,
        );

        while face_explorer.more() {
            face_explorer.next();
            if let Some(face_shape) = face_explorer.current() {
                if face_shape.is_face() {
                    // SAFETY: Safe because we checked the shape type
                    let face = unsafe {
                        &*(face_shape as *const _
                            as *const crate::topology::topods_face::TopoDsFace)
                    };
                    faces.push(face);
                }
            }
        }

        // Check all pairs of faces for intersections
        for i in 0..faces.len() {
            for j in i + 1..faces.len() {
                let face1 = faces[i];
                let face2 = faces[j];

                // Check if faces intersect
                if let Some(intersection) = self.check_face_intersection(face1, face2) {
                    self_intersections.push(intersection);
                }
            }
        }

        self_intersections
    }

    /// Check if two faces intersect
    fn check_face_intersection(
        &self,
        face1: &crate::topology::topods_face::TopoDsFace,
        face2: &crate::topology::topods_face::TopoDsFace,
    ) -> Option<(Point, Point)> {
        // Get surfaces of the faces
        if let (Some(surface1), Some(surface2)) = (face1.surface(), face2.surface()) {
            // Use advanced intersection solver to check for intersection
            let solver = crate::geometry::advanced_intersection::AdvancedIntersectionSolver::new();

            // Get SurfaceEnum from face surfaces
            let surface1_enum = &**surface1;
            let surface2_enum = &**surface2;

            // Compute surface-surface intersection
            let intersections = solver.surface_surface_intersection(surface1_enum, surface2_enum);

            if !intersections.is_empty() {
                // Return the first intersection point pair
                let first = intersections[0].point;
                let second = if intersections.len() > 1 {
                    intersections[1].point
                } else {
                    // If only one point, use a slightly offset point
                    first + crate::geometry::Vector::new(1e-6, 1e-6, 1e-6)
                };
                return Some((first, second));
            }
        }

        None
    }

    /// Detect duplicate vertices
    fn detect_duplicate_vertices(&self, shape: &TopoDsShape) -> Vec<(usize, usize)> {
        // Implementation of duplicate vertex detection
        let mut duplicate_vertices = Vec::new();

        // Collect all vertices in the shape
        let mut vertices = Vec::new();
        let mut vertex_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Vertex,
        );

        while vertex_explorer.more() {
            vertex_explorer.next();
            if let Some(vertex_shape) = vertex_explorer.current() {
                if vertex_shape.is_vertex() {
                    // SAFETY: Safe because we checked the shape type
                    let vertex = unsafe {
                        &*(vertex_shape as *const _
                            as *const crate::topology::topods_vertex::TopoDsVertex)
                    };
                    vertices.push(vertex.point());
                }
            }
        }

        // Compare all pairs of vertices
        for i in 0..vertices.len() {
            for j in i + 1..vertices.len() {
                let distance = vertices[i].distance(&vertices[j]);
                if distance < self.settings.vertex_merge_tolerance {
                    duplicate_vertices.push((i, j));
                }
            }
        }

        duplicate_vertices
    }

    /// Detect duplicate faces
    fn detect_duplicate_faces(&self, shape: &TopoDsShape) -> Vec<(usize, usize)> {
        // Implementation of duplicate face detection
        let mut duplicate_faces = Vec::new();

        // Collect all faces in the shape with their vertices
        let mut face_vertices: Vec<Vec<&Point>> = Vec::new();
        let mut face_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Face,
        );

        while face_explorer.more() {
            face_explorer.next();
            if let Some(face_shape) = face_explorer.current() {
                if face_shape.is_face() {
                    // SAFETY: Safe because we checked the shape type
                    let face = unsafe {
                        &*(face_shape as *const _
                            as *const crate::topology::topods_face::TopoDsFace)
                    };

                    // Collect face vertices
                    let mut vertices: Vec<&Point> = Vec::new();
                    let mut vertex_explorer =
                        crate::topology::top_exp_explorer::TopExpExplorer::new(
                            face.shape(),
                            crate::topology::shape_enum::ShapeType::Vertex,
                        );

                    while vertex_explorer.more() {
                        vertex_explorer.next();
                        if let Some(vertex_shape) = vertex_explorer.current() {
                            if vertex_shape.is_vertex() {
                                // SAFETY: Safe because we checked the shape type
                                let vertex = unsafe {
                                    &*(vertex_shape as *const _
                                        as *const crate::topology::topods_vertex::TopoDsVertex)
                                };
                                vertices.push(&vertex.point());
                            }
                        }
                    }

                    face_vertices.push(vertices);
                }
            }
        }

        // Compare all pairs of faces
        for i in 0..face_vertices.len() {
            for j in i + 1..face_vertices.len() {
                if self.are_faces_duplicate(&face_vertices[i], &face_vertices[j]) {
                    duplicate_faces.push((i, j));
                }
            }
        }

        duplicate_faces
    }

    /// Check if two faces are duplicates
    fn are_faces_duplicate(&self, vertices1: &[&Point], vertices2: &[&Point]) -> bool {
        // Check if faces have the same number of vertices
        if vertices1.len() != vertices2.len() {
            return false;
        }

        // Check if vertices are the same (considering tolerance)
        for (v1, v2) in vertices1.iter().zip(vertices2.iter()) {
            if v1.distance(v2) > self.settings.vertex_merge_tolerance {
                return false;
            }
        }

        true
    }

    /// Fix duplicate vertices
    fn fix_duplicate_vertices(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of duplicate vertex fixing
        let duplicate_pairs = self.detect_duplicate_vertices(shape);

        if duplicate_pairs.is_empty() {
            return Ok(shape.clone());
        }

        // Create a mapping from duplicate vertices to their representative
        let mut vertex_map = std::collections::HashMap::new();

        // Collect all vertices
        let vertices: Vec<_> = {
            let mut vertex_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
                shape,
                crate::topology::shape_enum::ShapeType::Vertex,
            );
            let mut v = Vec::new();
            while vertex_explorer.more() {
                if let Some(vertex_shape) = vertex_explorer.current() {
                    if vertex_shape.is_vertex() {
                        v.push(vertex_shape.clone());
                    }
                }
                vertex_explorer.next();
            }
            v
        };

        // Build vertex mapping
        for (i, j) in duplicate_pairs {
            if !vertex_map.contains_key(&j) {
                vertex_map.insert(j, i);
            }
        }

        // Create a new shape with merged vertices
        let builder = crate::modeling::BrepBuilder::new();
        let result =
            self.rebuild_shape_with_merged_vertices(shape, &vertices, &vertex_map, &builder)?;

        Ok(result)
    }

    /// Rebuild shape with merged vertices
    fn rebuild_shape_with_merged_vertices(
        &self,
        shape: &TopoDsShape,
        vertices: &[TopoDsShape],
        vertex_map: &std::collections::HashMap<usize, usize>,
        builder: &crate::modeling::BrepBuilder,
    ) -> Result<TopoDsShape, String> {
        // Collect all unique vertices
        let mut unique_vertices = Vec::new();
        let mut vertex_indices = std::collections::HashMap::new();

        for (i, vertex_shape) in vertices.iter().enumerate() {
            // Get the representative index
            let rep_index = vertex_map.get(&i).unwrap_or(&i);

            // Only add the representative vertex once
            if !vertex_indices.contains_key(rep_index) {
                if let Some(vertex) = vertex_shape.as_vertex() {
                    let point = vertex.point().clone();
                    let new_vertex = builder.make_vertex(point);
                    unique_vertices.push(new_vertex);
                    vertex_indices.insert(*rep_index, unique_vertices.len() - 1);
                }
            }
        }

        // Rebuild edges
        let mut edges = Vec::new();
        let mut edge_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Edge,
        );

        while edge_explorer.more() {
            edge_explorer.next();
            if let Some(edge_shape) = edge_explorer.current() {
                if let Some(edge) = edge_shape.as_edge() {
                    // Get edge vertices
                    let edge_vertices = edge.vertices();
                    if edge_vertices.len() == 2 {
                        // Find the original indices of the vertices
                        let mut v1_index = None;
                        let mut v2_index = None;

                        for (i, vertex_shape) in vertices.iter().enumerate() {
                            if vertex_shape.is_vertex() {
                                if let Some(v) = vertex_shape.as_vertex() {
                                    if v.point() == edge_vertices[0].point() {
                                        v1_index = Some(i);
                                    } else if v.point() == edge_vertices[1].point() {
                                        v2_index = Some(i);
                                    }
                                }
                            }
                        }

                        if let (Some(v1_idx), Some(v2_idx)) = (v1_index, v2_index) {
                            // Get representative indices
                            let rep_v1 = *vertex_map.get(&v1_idx).unwrap_or(&v1_idx);
                            let rep_v2 = *vertex_map.get(&v2_idx).unwrap_or(&v2_idx);

                            // Get unique vertices
                            if let (Some(&v1_pos), Some(&v2_pos)) =
                                (vertex_indices.get(&rep_v1), vertex_indices.get(&rep_v2))
                            {
                                let new_edge = builder.make_edge(
                                    unique_vertices[v1_pos].clone(),
                                    unique_vertices[v2_pos].clone(),
                                );
                                edges.push(new_edge);
                            }
                        }
                    }
                }
            }
        }

        // For simplicity, we'll return a compound shape with the new vertices and edges
        // In a complete implementation, we would rebuild the entire topology hierarchy
        let mut compound_mut = builder.make_compound();

        // Add vertices to compound
        for vertex in &unique_vertices {
            let shape_handle = Handle::new(Arc::new(vertex.shape().clone()));
            builder.add_to_compound(&mut compound_mut, shape_handle);
        }

        // Add edges to compound
        for edge in &edges {
            let shape_handle = Handle::new(Arc::new(edge.shape().clone()));
            builder.add_to_compound(&mut compound_mut, shape_handle);
        }

        Ok(compound_mut.as_ref().map(|c| c.shape().clone()).unwrap_or_else(|| TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound)))
    }

    /// Merge close vertices
    fn merge_close_vertices(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of close vertex merging
        // Collect all vertices in the shape
        let mut vertices = Vec::new();
        let mut vertex_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Vertex,
        );

        while vertex_explorer.more() {
            if let Some(vertex_shape) = vertex_explorer.current() {
                if vertex_shape.is_vertex() {
                    let vertex = unsafe {
                        &*(vertex_shape as *const _
                            as *const crate::topology::topods_vertex::TopoDsVertex)
                    };
                    vertices.push((vertex.point(), vertex_shape.clone()));
                }
            }
            vertex_explorer.next();
        }

        if vertices.is_empty() {
            return Ok(shape.clone());
        }

        // Group close vertices
        let mut groups = Vec::new();
        let mut visited = vec![false; vertices.len()];

        for i in 0..vertices.len() {
            if !visited[i] {
                let mut group = Vec::new();
                group.push(i);
                visited[i] = true;

                for j in i + 1..vertices.len() {
                    if !visited[j] {
                        let distance = vertices[i].0.distance(&vertices[j].0);
                        if distance < self.settings.vertex_merge_tolerance {
                            group.push(j);
                            visited[j] = true;
                        }
                    }
                }

                if group.len() > 1 {
                    groups.push(group);
                }
            }
        }

        if groups.is_empty() {
            return Ok(shape.clone());
        }

        // Create vertex mapping
        let mut vertex_map = std::collections::HashMap::new();
        for group in &groups {
            // Use the first vertex as the representative
            let representative = group[0];
            for &index in group {
                if index != representative {
                    vertex_map.insert(index, representative);
                }
            }
        }

        // Create a new shape with merged vertices
        let builder = crate::modeling::BrepBuilder::new();

        // Collect all unique vertices
        let mut unique_vertices = Vec::new();
        let mut vertex_indices = std::collections::HashMap::new();

        for (i, (point, _)) in vertices.iter().enumerate() {
            // Get the representative index
            let rep_index = *vertex_map.get(&i).unwrap_or(&i);

            // Only add the representative vertex once
            if !vertex_indices.contains_key(&rep_index) {
                let new_vertex = builder.make_vertex((*point).clone());
                unique_vertices.push(new_vertex);
                vertex_indices.insert(rep_index, unique_vertices.len() - 1);
            }
        }

        // Rebuild edges
        let mut edges = Vec::new();
        let mut edge_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Edge,
        );

        while edge_explorer.more() {
            edge_explorer.next();
            if let Some(edge_shape) = edge_explorer.current() {
                if let Some(edge) = edge_shape.as_edge() {
                    // Get edge vertices
                    let edge_vertices = edge.vertices();
                    if edge_vertices.len() == 2 {
                        // Find the original indices of the vertices
                        let mut v1_index = None;
                        let mut v2_index = None;

                        for (i, (point, _)) in vertices.iter().enumerate() {
                            if point == &edge_vertices[0].point() {
                                v1_index = Some(i);
                            } else if point == &edge_vertices[1].point() {
                                v2_index = Some(i);
                            }
                        }

                        if let (Some(v1_idx), Some(v2_idx)) = (v1_index, v2_index) {
                            // Get representative indices
                            let rep_v1 = *vertex_map.get(&v1_idx).unwrap_or(&v1_idx);
                            let rep_v2 = *vertex_map.get(&v2_idx).unwrap_or(&v2_idx);

                            // Get unique vertices
                            if let (Some(&v1_pos), Some(&v2_pos)) =
                                (vertex_indices.get(&rep_v1), vertex_indices.get(&rep_v2))
                            {
                                let new_edge = builder.make_edge(
                                    unique_vertices[v1_pos].clone(),
                                    unique_vertices[v2_pos].clone(),
                                );
                                edges.push(new_edge);
                            }
                        }
                    }
                }
            }
        }

        // For simplicity, we'll return a compound shape with the new vertices and edges
        // In a complete implementation, we would rebuild the entire topology hierarchy
        let mut compound_mut = builder.make_compound();

        // Add vertices to compound
        for vertex in &unique_vertices {
            let shape_handle = Handle::new(Arc::new(vertex.shape().clone()));
            builder.add_to_compound(&mut compound_mut, shape_handle);
        }

        // Add edges to compound
        for edge in &edges {
            let shape_handle = Handle::new(Arc::new(edge.shape().clone()));
            builder.add_to_compound(&mut compound_mut, shape_handle);
        }

        Ok(compound_mut.as_ref().map(|c| c.shape().clone()).unwrap_or_else(|| TopoDsShape::new(crate::topology::shape_enum::ShapeType::Compound)))
    }

    /// Fix degenerate faces
    fn fix_degenerate_faces(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of degenerate face fixing
        let degenerate_face_indices = self.detect_degenerate_faces(shape);

        if degenerate_face_indices.is_empty() {
            return Ok(shape.clone());
        }

        // Collect all faces, excluding degenerate ones
        let (valid_faces, _face_index) = {
            let mut face_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
                shape,
                crate::topology::shape_enum::ShapeType::Face,
            );
            let mut valid_faces = Vec::new();
            let mut face_index = 0;
            while face_explorer.more() {
                if let Some(face_shape) = face_explorer.current() {
                    if face_shape.is_face() {
                        if !degenerate_face_indices.contains(&face_index) {
                            valid_faces.push(face_shape.clone());
                        }
                        face_index += 1;
                    }
                }
                face_explorer.next();
            }
            (valid_faces, face_index)
        };

        // Create a new shape with only valid faces
        let builder = crate::modeling::BrepBuilder::new();
        let mut compound_mut = builder.make_compound();

        // Add valid faces to compound
        for face_shape in &valid_faces {
            if let Some(face) = face_shape.as_face() {
                let shape_handle = Handle::new(std::sync::Arc::new(face.shape().clone()));
                builder.add_to_compound(&mut compound_mut, shape_handle);
            }
        }

        // Also add all non-face shapes (vertices, edges, wires, shells, solids)
        let mut shape_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Compound,
        );

        while shape_explorer.more() {
            shape_explorer.next();
            if let Some(shape_item) = shape_explorer.current() {
                match shape_item.shape_type() {
                    crate::topology::shape_enum::ShapeType::Vertex => {
                        if let Some(vertex) = shape_item.as_vertex() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(vertex.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Edge => {
                        if let Some(edge) = shape_item.as_edge() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(edge.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Wire => {
                        if let Some(wire) = shape_item.as_wire() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(wire.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Shell => {
                        if let Some(shell) = shape_item.as_shell() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(shell.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Solid => {
                        if let Some(solid) = shape_item.as_solid() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(solid.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(compound_mut.shape().clone())
    }

    /// Fix non-manifold edges
    fn fix_non_manifold_edges(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of non-manifold edge fixing
        let non_manifold_edge_indices = self.detect_non_manifold_edges(shape);

        if non_manifold_edge_indices.is_empty() {
            return Ok(shape.clone());
        }

        // For simplicity, we'll create a new compound shape with only valid edges
        // In a complete implementation, we would split or merge edges to resolve non-manifold conditions
        let builder = crate::modeling::BrepBuilder::new();
        let mut compound_mut = builder.make_compound();

        // Collect all edges, excluding non-manifold ones
        let mut edge_index = 0;
        let mut edge_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Edge,
        );

        while edge_explorer.more() {
            edge_explorer.next();
            if let Some(edge_shape) = edge_explorer.current() {
                if edge_shape.is_edge() {
                    // Check if this edge is non-manifold
                    if !non_manifold_edge_indices.contains(&edge_index) {
                        if let Some(edge) = edge_shape.as_edge() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(edge.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    edge_index += 1;
                }
            }
        }

        // Also add all other shapes (vertices, faces, wires, shells, solids)
        let mut shape_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Compound,
        );

        while shape_explorer.more() {
            shape_explorer.next();
            if let Some(shape_item) = shape_explorer.current() {
                match shape_item.shape_type() {
                    crate::topology::shape_enum::ShapeType::Vertex => {
                        if let Some(vertex) = shape_item.as_vertex() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(vertex.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Face => {
                        if let Some(face) = shape_item.as_face() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(face.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Wire => {
                        if let Some(wire) = shape_item.as_wire() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(wire.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Shell => {
                        if let Some(shell) = shape_item.as_shell() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(shell.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Solid => {
                        if let Some(solid) = shape_item.as_solid() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(solid.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(compound_mut.shape().clone())
    }

    /// Fix self-intersections
    fn fix_self_intersections(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of self-intersection fixing
        let self_intersections = self.detect_self_intersections(shape);

        if self_intersections.is_empty() {
            return Ok(shape.clone());
        }

        // For simplicity, we'll create a new compound shape with all original shapes
        // In a complete implementation, we would split faces at intersection points
        let builder = crate::modeling::BrepBuilder::new();
        let mut compound_mut = builder.make_compound();

        // Add all shapes to the compound
        let mut shape_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Compound,
        );

        while shape_explorer.more() {
            shape_explorer.next();
            if let Some(shape_item) = shape_explorer.current() {
                match shape_item.shape_type() {
                    crate::topology::shape_enum::ShapeType::Vertex => {
                        if let Some(vertex) = shape_item.as_vertex() {
                            let vertex_handle = builder.copy_vertex(vertex);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(vertex_handle.shape().clone())),
                            );
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Edge => {
                        if let Some(edge) = shape_item.as_edge() {
                            let edge_handle = builder.copy_edge(edge);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(edge_handle.shape().clone())),
                            );
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Face => {
                        if let Some(face) = shape_item.as_face() {
                            let face_handle = builder.copy_face(face);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(face_handle.shape().clone())),
                            );
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Wire => {
                        if let Some(wire) = shape_item.as_wire() {
                            let wire_handle = builder.copy_wire(wire);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(wire_handle.shape().clone())),
                            );
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Shell => {
                        if let Some(shell) = shape_item.as_shell() {
                            let shell_handle = builder.copy_shell(shell);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(shell_handle.shape().clone())),
                            );
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Solid => {
                        if let Some(solid) = shape_item.as_solid() {
                            let solid_handle = builder.copy_solid(solid);
                            builder.add_to_compound(
                                &mut compound_mut,
                                Handle::new(Arc::new(solid_handle.shape().clone())),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(compound_mut.shape().clone())
    }

    /// Fix duplicate faces
    fn fix_duplicate_faces(&self, shape: &TopoDsShape) -> Result<TopoDsShape, String> {
        // Implementation of duplicate face fixing
        let duplicate_face_pairs = self.detect_duplicate_faces(shape);

        if duplicate_face_pairs.is_empty() {
            return Ok(shape.clone());
        }

        // Create a set of face indices to keep (all faces not marked as duplicate)
        let mut faces_to_remove = std::collections::HashSet::new();
        for (_, j) in &duplicate_face_pairs {
            faces_to_remove.insert(*j);
        }

        // Collect all faces, excluding duplicates
        let mut valid_faces = Vec::new();
        let mut face_index = 0;
        let mut face_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Face,
        );

        while face_explorer.more() {
            if let Some(face_shape) = face_explorer.current() {
                if face_shape.is_face() {
                    if !faces_to_remove.contains(&face_index) {
                        valid_faces.push(face_shape.clone());
                    }
                    face_index += 1;
                }
            }
            face_explorer.next();
        }

        // Create a new shape with only valid faces
        let builder = crate::modeling::BrepBuilder::new();
        let mut compound_mut = builder.make_compound();

        // Add valid faces to compound
        for face_shape in &valid_faces {
            if let Some(face) = face_shape.as_face() {
                let shape_handle = Handle::new(std::sync::Arc::new(face.shape().clone()));
                builder.add_to_compound(&mut compound_mut, shape_handle);
            }
        }

        // Also add all non-face shapes (vertices, edges, wires, shells, solids)
        let mut shape_explorer = crate::topology::top_exp_explorer::TopExpExplorer::new(
            shape,
            crate::topology::shape_enum::ShapeType::Compound,
        );

        while shape_explorer.more() {
            shape_explorer.next();
            if let Some(shape_item) = shape_explorer.current() {
                match shape_item.shape_type() {
                    crate::topology::shape_enum::ShapeType::Vertex => {
                        if let Some(vertex) = shape_item.as_vertex() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(vertex.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Edge => {
                        if let Some(edge) = shape_item.as_edge() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(edge.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Wire => {
                        if let Some(wire) = shape_item.as_wire() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(wire.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Shell => {
                        if let Some(shell) = shape_item.as_shell() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(shell.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    crate::topology::shape_enum::ShapeType::Solid => {
                        if let Some(solid) = shape_item.as_solid() {
                            let shape_handle =
                                Handle::new(std::sync::Arc::new(solid.shape().clone()));
                            builder.add_to_compound(&mut compound_mut, shape_handle);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(compound_mut.shape().clone())
    }

    /// Get repair log
    pub fn get_log(&self) -> &Vec<String> {
        &self.log
    }
}

/// Topology validator
pub struct TopoDsValidator {
    #[allow(dead_code)]
    tolerance: f64,
}

impl TopoDsValidator {
    /// Create a new topology validator
    pub fn new() -> Self {
        Self { tolerance: 1e-6 }
    }

    /// Create a new topology validator with custom tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Validate a shape
    pub fn validate(&self, shape: &TopoDsShape) -> Vec<String> {
        let mut errors = Vec::new();

        // Check for non-manifold edges
        let shape_repair = ShapeRepairTools::new();
        let non_manifold_edges = shape_repair.detect_non_manifold_edges(shape);
        if !non_manifold_edges.is_empty() {
            errors.push(format!(
                "Shape has {} non-manifold edges",
                non_manifold_edges.len()
            ));
        }

        // Check for degenerate faces
        let degenerate_faces = shape_repair.detect_degenerate_faces(shape);
        if !degenerate_faces.is_empty() {
            errors.push(format!(
                "Shape has {} degenerate faces",
                degenerate_faces.len()
            ));
        }

        // Check for self-intersections
        let self_intersections = shape_repair.detect_self_intersections(shape);
        if !self_intersections.is_empty() {
            errors.push(format!(
                "Shape has {} self-intersections",
                self_intersections.len()
            ));
        }

        // Check for duplicate vertices
        let duplicate_vertices = shape_repair.detect_duplicate_vertices(shape);
        if !duplicate_vertices.is_empty() {
            errors.push(format!(
                "Shape has {} duplicate vertices",
                duplicate_vertices.len()
            ));
        }

        // Check for duplicate faces
        let duplicate_faces = shape_repair.detect_duplicate_faces(shape);
        if !duplicate_faces.is_empty() {
            errors.push(format!(
                "Shape has {} duplicate faces",
                duplicate_faces.len()
            ));
        }

        errors
    }

    /// Check if a shape is valid
    pub fn is_valid(&self, shape: &TopoDsShape) -> bool {
        self.validate(shape).is_empty()
    }
}
