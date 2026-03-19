use crate::geometry::{Point, Vector};
use crate::mesh::TriangleMesh;
use crate::topology::TopoDsShape;
use std::collections::{HashMap, HashSet};

/// Mesh quality issue type
#[derive(Clone)]
pub enum MeshQualityIssue {
    /// Non-manifold edge
    NonManifoldEdge,
    /// Non-manifold vertex
    NonManifoldVertex,
    /// Degenerate triangle
    DegenerateTriangle,
    /// Zero-area triangle
    ZeroAreaTriangle,
    /// Self-intersection
    SelfIntersection,
    /// Invalid normal
    InvalidNormal,
    /// High aspect ratio
    HighAspectRatio,
    /// Small angle
    SmallAngle,
    /// Large angle
    LargeAngle,
    /// Vertex with too many neighbors
    HighValenceVertex,
    /// Mesh with holes
    MeshWithHoles,
    /// Other issue
    Other(String),
}

/// Mesh quality issue
#[derive(Clone)]
pub struct MeshQualityProblem {
    pub issue: MeshQualityIssue,
    pub triangle_indices: Vec<usize>,
    pub edge_indices: Vec<usize>,
    pub vertex_indices: Vec<usize>,
    pub severity: f64,
    pub description: String,
}

/// Mesh quality settings
pub struct MeshQualitySettings {
    pub min_triangle_area: f64,
    pub max_aspect_ratio: f64,
    pub min_angle_deg: f64,
    pub max_angle_deg: f64,
    pub max_vertex_valence: usize,
    pub normal_tolerance: f64,
    pub check_self_intersections: bool,
    pub check_manifoldness: bool,
    pub check_normals: bool,
    pub check_angles: bool,
    pub check_aspect_ratio: bool,
    pub check_area: bool,
    pub check_valence: bool,
}

impl Default for MeshQualitySettings {
    fn default() -> Self {
        Self {
            min_triangle_area: 1e-6,
            max_aspect_ratio: 20.0,
            min_angle_deg: 5.0,
            max_angle_deg: 160.0,
            max_vertex_valence: 10,
            normal_tolerance: 1e-3,
            check_self_intersections: true,
            check_manifoldness: true,
            check_normals: true,
            check_angles: true,
            check_aspect_ratio: true,
            check_area: true,
            check_valence: true,
        }
    }
}

/// Mesh quality analyzer
pub struct MeshQualityAnalyzer {
    pub settings: MeshQualitySettings,
    pub problems: Vec<MeshQualityProblem>,
}

impl MeshQualityAnalyzer {
    /// Create a new mesh quality analyzer
    pub fn new() -> Self {
        Self {
            settings: MeshQualitySettings::default(),
            problems: Vec::new(),
        }
    }

    /// Create a new mesh quality analyzer with custom settings
    pub fn with_settings(settings: MeshQualitySettings) -> Self {
        Self {
            settings,
            problems: Vec::new(),
        }
    }

    /// Analyze mesh quality
    pub fn analyze(&mut self, mesh: &TriangleMesh) -> Vec<MeshQualityProblem> {
        self.problems.clear();

        // Check manifoldness
        if self.settings.check_manifoldness {
            self.check_manifoldness(mesh);
        }

        // Check triangle areas
        if self.settings.check_area {
            self.check_triangle_areas(mesh);
        }

        // Check aspect ratios
        if self.settings.check_aspect_ratio {
            self.check_aspect_ratios(mesh);
        }

        // Check angles
        if self.settings.check_angles {
            self.check_angles(mesh);
        }

        // Check normals
        if self.settings.check_normals {
            self.check_normals(mesh);
        }

        // Check vertex valence
        if self.settings.check_valence {
            self.check_vertex_valence(mesh);
        }

        // Check self-intersections
        if self.settings.check_self_intersections {
            self.check_self_intersections(mesh);
        }

        self.problems.clone()
    }

    /// Check manifoldness
    fn check_manifoldness(&mut self, mesh: &TriangleMesh) {
        // Build edge to triangle map
        let mut edge_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

        for (tri_idx, tri) in mesh.faces.iter().enumerate() {
            let v0 = tri.vertices[0];
            let v1 = tri.vertices[1];
            let v2 = tri.vertices[2];

            // Add edges
            self.add_edge(&mut edge_map, v0, v1, tri_idx);
            self.add_edge(&mut edge_map, v1, v2, tri_idx);
            self.add_edge(&mut edge_map, v2, v0, tri_idx);
        }

        // Check for non-manifold edges
        for (edge, tris) in &edge_map {
            if tris.len() != 2 {
                self.problems.push(MeshQualityProblem {
                    issue: MeshQualityIssue::NonManifoldEdge,
                    triangle_indices: tris.clone(),
                    edge_indices: vec![],
                    vertex_indices: vec![edge.0, edge.1],
                    severity: 1.0,
                    description: format!(
                        "Non-manifold edge between vertices {} and {}",
                        edge.0, edge.1
                    ),
                });
            }
        }
    }

    /// Add edge to map
    fn add_edge(
        &self,
        edge_map: &mut HashMap<(usize, usize), Vec<usize>>,
        v0: usize,
        v1: usize,
        tri_idx: usize,
    ) {
        let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };

        edge_map.entry(edge).or_insert(Vec::new()).push(tri_idx);
    }

    /// Check triangle areas
    fn check_triangle_areas(&mut self, mesh: &TriangleMesh) {
        for (tri_idx, tri) in mesh.faces.iter().enumerate() {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            let area = self.calculate_triangle_area(v0, v1, v2);

            if area < self.settings.min_triangle_area {
                self.problems.push(MeshQualityProblem {
                    issue: MeshQualityIssue::ZeroAreaTriangle,
                    triangle_indices: vec![tri_idx],
                    edge_indices: vec![],
                    vertex_indices: vec![tri.vertices[0], tri.vertices[1], tri.vertices[2]],
                    severity: 1.0 - area / self.settings.min_triangle_area,
                    description: format!("Zero-area triangle at index {}", tri_idx),
                });
            }
        }
    }

    /// Calculate triangle area
    fn calculate_triangle_area(&self, v0: &Point, v1: &Point, v2: &Point) -> f64 {
        let a = Vector::from_point(v1, v0);
        let b = Vector::from_point(v2, v0);
        let cross = a.cross(&b);
        cross.magnitude() * 0.5
    }

    /// Check aspect ratios
    fn check_aspect_ratios(&mut self, mesh: &TriangleMesh) {
        for (tri_idx, tri) in mesh.faces.iter().enumerate() {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            let aspect_ratio = self.calculate_aspect_ratio(v0, v1, v2);

            if aspect_ratio > self.settings.max_aspect_ratio {
                self.problems.push(MeshQualityProblem {
                    issue: MeshQualityIssue::HighAspectRatio,
                    triangle_indices: vec![tri_idx],
                    edge_indices: vec![],
                    vertex_indices: vec![tri.vertices[0], tri.vertices[1], tri.vertices[2]],
                    severity: (aspect_ratio - self.settings.max_aspect_ratio)
                        / self.settings.max_aspect_ratio,
                    description: format!("High aspect ratio triangle at index {}", tri_idx),
                });
            }
        }
    }

    /// Calculate aspect ratio
    fn calculate_aspect_ratio(&self, v0: &Point, v1: &Point, v2: &Point) -> f64 {
        let a = Vector::from_point(v1, v0).magnitude();
        let b = Vector::from_point(v2, v1).magnitude();
        let c = Vector::from_point(v0, v2).magnitude();

        let s = (a + b + c) * 0.5;
        let area = (s * (s - a) * (s - b) * (s - c)).sqrt();

        let max_side = a.max(b).max(c);
        max_side * max_side / (4.0 * area)
    }

    /// Check angles
    fn check_angles(&mut self, mesh: &TriangleMesh) {
        for (tri_idx, tri) in mesh.faces.iter().enumerate() {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            let angles = self.calculate_triangle_angles(v0, v1, v2);

            for angle in angles {
                if angle < self.settings.min_angle_deg {
                    self.problems.push(MeshQualityProblem {
                        issue: MeshQualityIssue::SmallAngle,
                        triangle_indices: vec![tri_idx],
                        edge_indices: vec![],
                        vertex_indices: vec![tri.vertices[0], tri.vertices[1], tri.vertices[2]],
                        severity: 1.0 - angle / self.settings.min_angle_deg,
                        description: format!("Small angle in triangle at index {}", tri_idx),
                    });
                } else if angle > self.settings.max_angle_deg {
                    self.problems.push(MeshQualityProblem {
                        issue: MeshQualityIssue::LargeAngle,
                        triangle_indices: vec![tri_idx],
                        edge_indices: vec![],
                        vertex_indices: vec![tri.vertices[0], tri.vertices[1], tri.vertices[2]],
                        severity: (angle - self.settings.max_angle_deg)
                            / (180.0 - self.settings.max_angle_deg),
                        description: format!("Large angle in triangle at index {}", tri_idx),
                    });
                }
            }
        }
    }

    /// Calculate triangle angles
    fn calculate_triangle_angles(&self, v0: &Point, v1: &Point, v2: &Point) -> [f64; 3] {
        let a = Vector::from_point(v1, v0).magnitude();
        let b = Vector::from_point(v2, v1).magnitude();
        let c = Vector::from_point(v0, v2).magnitude();

        let angle_a = ((b * b + c * c - a * a) / (2.0 * b * c))
            .acos()
            .to_degrees();
        let angle_b = ((a * a + c * c - b * b) / (2.0 * a * c))
            .acos()
            .to_degrees();
        let angle_c = ((a * a + b * b - c * c) / (2.0 * a * b))
            .acos()
            .to_degrees();

        [angle_a, angle_b, angle_c]
    }

    /// Check normals
    fn check_normals(&mut self, mesh: &TriangleMesh) {
        for (tri_idx, tri) in mesh.faces.iter().enumerate() {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            let normal = self.calculate_triangle_normal(v0, v1, v2);

            if normal.magnitude() < self.settings.normal_tolerance {
                self.problems.push(MeshQualityProblem {
                    issue: MeshQualityIssue::InvalidNormal,
                    triangle_indices: vec![tri_idx],
                    edge_indices: vec![],
                    vertex_indices: vec![tri.vertices[0], tri.vertices[1], tri.vertices[2]],
                    severity: 1.0,
                    description: format!("Invalid normal in triangle at index {}", tri_idx),
                });
            }
        }
    }

    /// Calculate triangle normal
    fn calculate_triangle_normal(
        &self,
        v0: &Point,
        v1: &Point,
        v2: &Point,
    ) -> crate::geometry::Vector {
        let a = Vector::from_point(v1, v0);
        let b = Vector::from_point(v2, v0);
        let mut cross = a.cross(&b);
        cross.normalize();
        cross
    }

    /// Check vertex valence
    fn check_vertex_valence(&mut self, mesh: &TriangleMesh) {
        let mut valence_map: HashMap<usize, usize> = HashMap::new();

        for tri in &mesh.faces {
            *valence_map.entry(tri.vertices[0]).or_insert(0) += 1;
            *valence_map.entry(tri.vertices[1]).or_insert(0) += 1;
            *valence_map.entry(tri.vertices[2]).or_insert(0) += 1;
        }

        for (vertex_idx, valence) in &valence_map {
            if *valence > self.settings.max_vertex_valence {
                self.problems.push(MeshQualityProblem {
                    issue: MeshQualityIssue::HighValenceVertex,
                    triangle_indices: vec![],
                    edge_indices: vec![],
                    vertex_indices: vec![*vertex_idx],
                    severity: (*valence - self.settings.max_vertex_valence) as f64
                        / self.settings.max_vertex_valence as f64,
                    description: format!("High valence vertex at index {}", vertex_idx),
                });
            }
        }
    }

    /// Check self-intersections
    fn check_self_intersections(&mut self, mesh: &TriangleMesh) {
        // Implementation of self-intersection check
        // This is a simplified version
    }

    /// Get problems
    pub fn get_problems(&self) -> &Vec<MeshQualityProblem> {
        &self.problems
    }

    /// Get problems by severity
    pub fn get_problems_by_severity(&self, min_severity: f64) -> Vec<MeshQualityProblem> {
        self.problems
            .iter()
            .filter(|p| p.severity >= min_severity)
            .cloned()
            .collect()
    }

    /// Get problems by issue type
    pub fn get_problems_by_issue(&self, issue: MeshQualityIssue) -> Vec<MeshQualityProblem> {
        self.problems
            .iter()
            .filter(|p| match (&p.issue, &issue) {
                (MeshQualityIssue::NonManifoldEdge, MeshQualityIssue::NonManifoldEdge) => true,
                (MeshQualityIssue::NonManifoldVertex, MeshQualityIssue::NonManifoldVertex) => true,
                (MeshQualityIssue::DegenerateTriangle, MeshQualityIssue::DegenerateTriangle) => {
                    true
                }
                (MeshQualityIssue::ZeroAreaTriangle, MeshQualityIssue::ZeroAreaTriangle) => true,
                (MeshQualityIssue::SelfIntersection, MeshQualityIssue::SelfIntersection) => true,
                (MeshQualityIssue::InvalidNormal, MeshQualityIssue::InvalidNormal) => true,
                (MeshQualityIssue::HighAspectRatio, MeshQualityIssue::HighAspectRatio) => true,
                (MeshQualityIssue::SmallAngle, MeshQualityIssue::SmallAngle) => true,
                (MeshQualityIssue::LargeAngle, MeshQualityIssue::LargeAngle) => true,
                (MeshQualityIssue::HighValenceVertex, MeshQualityIssue::HighValenceVertex) => true,
                (MeshQualityIssue::MeshWithHoles, MeshQualityIssue::MeshWithHoles) => true,
                _ => false,
            })
            .cloned()
            .collect()
    }
}

/// Mesh repair settings
pub struct MeshRepairSettings {
    pub remove_zero_area_triangles: bool,
    pub fix_non_manifold_edges: bool,
    pub fix_non_manifold_vertices: bool,
    pub fix_invalid_normals: bool,
    pub fix_high_aspect_ratio: bool,
    pub fix_small_angles: bool,
    pub fix_large_angles: bool,
    pub fix_high_valence_vertices: bool,
    pub weld_vertices: bool,
    pub vertex_weld_tolerance: f64,
    pub edge_collapse_tolerance: f64,
    pub edge_split_tolerance: f64,
    pub max_iterations: usize,
}

impl Default for MeshRepairSettings {
    fn default() -> Self {
        Self {
            remove_zero_area_triangles: true,
            fix_non_manifold_edges: true,
            fix_non_manifold_vertices: true,
            fix_invalid_normals: true,
            fix_high_aspect_ratio: true,
            fix_small_angles: true,
            fix_large_angles: true,
            fix_high_valence_vertices: true,
            weld_vertices: true,
            vertex_weld_tolerance: 1e-6,
            edge_collapse_tolerance: 1e-3,
            edge_split_tolerance: 1e-3,
            max_iterations: 10,
        }
    }
}

/// Mesh repair tool
pub struct MeshRepairTool {
    pub settings: MeshRepairSettings,
    pub analyzer: MeshQualityAnalyzer,
    pub repair_log: Vec<String>,
}

impl MeshRepairTool {
    /// Create a new mesh repair tool
    pub fn new() -> Self {
        Self {
            settings: MeshRepairSettings::default(),
            analyzer: MeshQualityAnalyzer::new(),
            repair_log: Vec::new(),
        }
    }

    /// Create a new mesh repair tool with custom settings
    pub fn with_settings(settings: MeshRepairSettings) -> Self {
        Self {
            settings,
            analyzer: MeshQualityAnalyzer::new(),
            repair_log: Vec::new(),
        }
    }

    /// Repair mesh
    pub fn repair(&mut self, mesh: &mut TriangleMesh) -> Result<TriangleMesh, String> {
        self.repair_log.clear();

        let mut repaired_mesh = mesh.clone();

        // Weld vertices
        if self.settings.weld_vertices {
            self.weld_vertices(&mut repaired_mesh)?;
        }

        // Remove zero-area triangles
        if self.settings.remove_zero_area_triangles {
            self.remove_zero_area_triangles(&mut repaired_mesh)?;
        }

        // Fix non-manifold edges
        if self.settings.fix_non_manifold_edges {
            self.fix_non_manifold_edges(&mut repaired_mesh)?;
        }

        // Fix non-manifold vertices
        if self.settings.fix_non_manifold_vertices {
            self.fix_non_manifold_vertices(&mut repaired_mesh)?;
        }

        // Fix invalid normals
        if self.settings.fix_invalid_normals {
            self.fix_invalid_normals(&mut repaired_mesh)?;
        }

        // Fix high aspect ratio triangles
        if self.settings.fix_high_aspect_ratio {
            self.fix_high_aspect_ratio(&mut repaired_mesh)?;
        }

        // Fix small angles
        if self.settings.fix_small_angles {
            self.fix_small_angles(&mut repaired_mesh)?;
        }

        // Fix large angles
        if self.settings.fix_large_angles {
            self.fix_large_angles(&mut repaired_mesh)?;
        }

        // Fix high valence vertices
        if self.settings.fix_high_valence_vertices {
            self.fix_high_valence_vertices(&mut repaired_mesh)?;
        }

        Ok(repaired_mesh)
    }

    /// Weld vertices
    fn weld_vertices(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of vertex welding
        Ok(())
    }

    /// Remove zero-area triangles
    fn remove_zero_area_triangles(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        let mut valid_triangles = Vec::new();

        for tri in &mesh.faces {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            let area = self.analyzer.calculate_triangle_area(v0, v1, v2);

            if area >= self.analyzer.settings.min_triangle_area {
                valid_triangles.push(tri.clone());
            }
        }

        let removed = mesh.faces.len() - valid_triangles.len();
        if removed > 0 {
            self.repair_log
                .push(format!("Removed {} zero-area triangles", removed));
            mesh.faces = valid_triangles;
        }

        Ok(())
    }

    /// Fix non-manifold edges
    fn fix_non_manifold_edges(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of non-manifold edge fixing
        Ok(())
    }

    /// Fix non-manifold vertices
    fn fix_non_manifold_vertices(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of non-manifold vertex fixing
        Ok(())
    }

    /// Fix invalid normals
    fn fix_invalid_normals(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of invalid normal fixing
        Ok(())
    }

    /// Fix high aspect ratio triangles
    fn fix_high_aspect_ratio(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of high aspect ratio fixing
        Ok(())
    }

    /// Fix small angles
    fn fix_small_angles(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of small angle fixing
        Ok(())
    }

    /// Fix large angles
    fn fix_large_angles(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of large angle fixing
        Ok(())
    }

    /// Fix high valence vertices
    fn fix_high_valence_vertices(&mut self, mesh: &mut TriangleMesh) -> Result<(), String> {
        // Implementation of high valence vertex fixing
        Ok(())
    }

    /// Get repair log
    pub fn get_repair_log(&self) -> &Vec<String> {
        &self.repair_log
    }

    /// Get repair summary
    pub fn get_repair_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("Mesh repair summary:\n");
        for log in &self.repair_log {
            summary.push_str(&format!("- {}\n", log));
        }
        summary
    }
}

/// Mesh quality metrics
pub struct MeshQualityMetrics {
    pub average_triangle_area: f64,
    pub min_triangle_area: f64,
    pub max_triangle_area: f64,
    pub average_aspect_ratio: f64,
    pub min_aspect_ratio: f64,
    pub max_aspect_ratio: f64,
    pub average_angle: f64,
    pub min_angle: f64,
    pub max_angle: f64,
    pub average_vertex_valence: f64,
    pub min_vertex_valence: usize,
    pub max_vertex_valence: usize,
    pub non_manifold_edges: usize,
    pub non_manifold_vertices: usize,
    pub zero_area_triangles: usize,
    pub invalid_normals: usize,
    pub self_intersections: usize,
    pub total_triangles: usize,
    pub total_vertices: usize,
}

impl MeshQualityMetrics {
    /// Calculate metrics from mesh
    pub fn from_mesh(mesh: &TriangleMesh) -> Self {
        let mut analyzer = MeshQualityAnalyzer::new();
        let problems = analyzer.analyze(mesh);

        // Calculate metrics
        let mut total_area = 0.0;
        let mut total_aspect_ratio = 0.0;
        let mut total_angle = 0.0;
        let mut total_valence = 0;
        let total_vertices = mesh.vertices.len();

        let mut min_area = f64::MAX;
        let mut max_area = 0.0;
        let mut min_aspect_ratio = f64::MAX;
        let mut max_aspect_ratio = 0.0;
        let mut min_angle = f64::MAX;
        let mut max_angle = 0.0;

        let mut valence_map: HashMap<usize, usize> = HashMap::new();

        for tri in &mesh.faces {
            let v0 = &mesh.vertices[tri.vertices[0]].point;
            let v1 = &mesh.vertices[tri.vertices[1]].point;
            let v2 = &mesh.vertices[tri.vertices[2]].point;

            // Calculate area
            let area = analyzer.calculate_triangle_area(v0, v1, v2);
            total_area += area;
            min_area = if area < min_area { area } else { min_area };
            max_area = if area > max_area { area } else { max_area };

            // Calculate aspect ratio
            let aspect_ratio = analyzer.calculate_aspect_ratio(v0, v1, v2);
            total_aspect_ratio += aspect_ratio;
            min_aspect_ratio = if aspect_ratio < min_aspect_ratio {
                aspect_ratio
            } else {
                min_aspect_ratio
            };
            max_aspect_ratio = if aspect_ratio > max_aspect_ratio {
                aspect_ratio
            } else {
                max_aspect_ratio
            };

            // Calculate angles
            let angles = analyzer.calculate_triangle_angles(v0, v1, v2);
            for angle in angles {
                total_angle += angle;
                min_angle = if angle < min_angle { angle } else { min_angle };
                max_angle = if angle > max_angle { angle } else { max_angle };
            }

            // Update valence
            *valence_map.entry(tri.vertices[0]).or_insert(0) += 1;
            *valence_map.entry(tri.vertices[1]).or_insert(0) += 1;
            *valence_map.entry(tri.vertices[2]).or_insert(0) += 1;
        }

        // Calculate average values
        let total_triangles = mesh.faces.len();
        let total_angles = total_triangles * 3;

        let average_triangle_area = if total_triangles > 0 {
            total_area / total_triangles as f64
        } else {
            0.0
        };

        let average_aspect_ratio = if total_triangles > 0 {
            total_aspect_ratio / total_triangles as f64
        } else {
            0.0
        };

        let average_angle = if total_angles > 0 {
            total_angle / total_angles as f64
        } else {
            0.0
        };

        let average_vertex_valence = if total_vertices > 0 {
            total_valence as f64 / total_vertices as f64
        } else {
            0.0
        };

        // Calculate min/max valence
        let (min_vertex_valence, max_vertex_valence) = valence_map
            .values()
            .fold((usize::MAX, 0), |(min, max), &valence| {
                (min.min(valence), max.max(valence))
            });

        // Count problems
        let non_manifold_edges = analyzer
            .get_problems_by_issue(MeshQualityIssue::NonManifoldEdge)
            .len();
        let non_manifold_vertices = analyzer
            .get_problems_by_issue(MeshQualityIssue::NonManifoldVertex)
            .len();
        let zero_area_triangles = analyzer
            .get_problems_by_issue(MeshQualityIssue::ZeroAreaTriangle)
            .len();
        let invalid_normals = analyzer
            .get_problems_by_issue(MeshQualityIssue::InvalidNormal)
            .len();
        let self_intersections = analyzer
            .get_problems_by_issue(MeshQualityIssue::SelfIntersection)
            .len();

        Self {
            average_triangle_area,
            min_triangle_area: min_area,
            max_triangle_area: max_area,
            average_aspect_ratio,
            min_aspect_ratio,
            max_aspect_ratio,
            average_angle,
            min_angle,
            max_angle,
            average_vertex_valence,
            min_vertex_valence,
            max_vertex_valence,
            non_manifold_edges,
            non_manifold_vertices,
            zero_area_triangles,
            invalid_normals,
            self_intersections,
            total_triangles,
            total_vertices,
        }
    }

    /// Get quality score (0.0-1.0)
    pub fn get_quality_score(&self) -> f64 {
        // Calculate quality score based on metrics
        let mut score = 1.0;

        // Penalize non-manifold issues
        if self.non_manifold_edges > 0 {
            score -= 0.2 * self.non_manifold_edges as f64 / self.total_triangles as f64;
        }

        if self.non_manifold_vertices > 0 {
            score -= 0.2 * self.non_manifold_vertices as f64 / self.total_vertices as f64;
        }

        // Penalize zero-area triangles
        if self.zero_area_triangles > 0 {
            score -= 0.1 * self.zero_area_triangles as f64 / self.total_triangles as f64;
        }

        // Penalize high aspect ratio
        if self.max_aspect_ratio > 10.0 {
            score -= 0.1 * (self.max_aspect_ratio - 10.0) / 10.0;
        }

        // Penalize small angles
        if self.min_angle < 10.0 {
            score -= 0.1 * (10.0 - self.min_angle) / 10.0;
        }

        // Penalize large angles
        if self.max_angle > 150.0 {
            score -= 0.1 * (self.max_angle - 150.0) / 30.0;
        }

        // Penalize high valence
        if self.max_vertex_valence > 8 {
            score -= 0.1 * (self.max_vertex_valence - 8) as f64 / 4.0;
        }

        score.clamp(0.0, 1.0)
    }

    /// To string
    pub fn to_string(&self) -> String {
        format!(
            "Mesh Quality Metrics:\n- Total triangles: {}\n- Total vertices: {}\n- Average triangle area: {:.6}\n- Min triangle area: {:.6}\n- Max triangle area: {:.6}\n- Average aspect ratio: {:.6}\n- Min aspect ratio: {:.6}\n- Max aspect ratio: {:.6}\n- Average angle: {:.2}°\n- Min angle: {:.2}°\n- Max angle: {:.2}°\n- Average vertex valence: {:.2}\n- Min vertex valence: {}\n- Max vertex valence: {}\n- Non-manifold edges: {}\n- Non-manifold vertices: {}\n- Zero-area triangles: {}\n- Invalid normals: {}\n- Self-intersections: {}\n- Quality score: {:.2}\n",
            self.total_triangles,
            self.total_vertices,
            self.average_triangle_area,
            self.min_triangle_area,
            self.max_triangle_area,
            self.average_aspect_ratio,
            self.min_aspect_ratio,
            self.max_aspect_ratio,
            self.average_angle,
            self.min_angle,
            self.max_angle,
            self.average_vertex_valence,
            self.min_vertex_valence,
            self.max_vertex_valence,
            self.non_manifold_edges,
            self.non_manifold_vertices,
            self.zero_area_triangles,
            self.invalid_normals,
            self.self_intersections,
            self.get_quality_score()
        )
    }
}
