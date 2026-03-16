/// Mesh vertex definition for visualization
#[derive(Debug, Clone)]
pub struct MeshVertex {
    pub point: crate::geometry::Point,
    // 可扩展属性: normal, uv, color 等
}
/// Level of Detail (LOD) system
///
/// This module provides functionality for managing levels of detail for large models,
/// including hierarchical LOD generation, transition management, and view-dependent
/// LOD selection.
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::Mesh3D;

/// LOD level definition
pub struct LodLevel {
    /// Level ID (0 = highest detail)
    level: usize,
    /// Mesh for this level
    mesh: Mesh3D,
    /// Simplification factor (relative to highest detail)
    simplification_factor: f64,
    /// Bounding box for this level
    bounding_box: crate::geometry::BoundingBox,
    /// Quality metrics
    quality_metrics: LodQualityMetrics,
}

impl LodLevel {
    /// Create a new LOD level
    pub fn new(
        level: usize,
        mesh: Mesh3D,
        simplification_factor: f64,
        bounding_box: crate::geometry::BoundingBox,
        quality_metrics: LodQualityMetrics,
    ) -> Self {
        Self {
            level,
            mesh,
            simplification_factor,
            bounding_box,
            quality_metrics,
        }
    }

    /// Get the level ID
    pub fn level(&self) -> usize {
        self.level
    }

    /// Get the mesh
    pub fn mesh(&self) -> &Mesh3D {
        &self.mesh
    }

    /// Get the simplification factor
    pub fn simplification_factor(&self) -> f64 {
        self.simplification_factor
    }

    /// Get the bounding box
    pub fn bounding_box(&self) -> &crate::geometry::BoundingBox {
        &self.bounding_box
    }

    /// Get the quality metrics
    pub fn quality_metrics(&self) -> &LodQualityMetrics {
        &self.quality_metrics
    }
}

/// LOD quality metrics
pub struct LodQualityMetrics {
    /// Number of triangles
    triangle_count: usize,
    /// Average edge length
    average_edge_length: f64,
    /// Maximum edge length
    maximum_edge_length: f64,
    /// Geometric error (relative to highest detail)
    geometric_error: f64,
    /// Visual error estimate
    visual_error: f64,
}

impl LodQualityMetrics {
    /// Create new quality metrics
    pub fn new(
        triangle_count: usize,
        average_edge_length: f64,
        maximum_edge_length: f64,
        geometric_error: f64,
        visual_error: f64,
    ) -> Self {
        Self {
            triangle_count,
            average_edge_length,
            maximum_edge_length,
            geometric_error,
            visual_error,
        }
    }

    /// Get the triangle count
    pub fn triangle_count(&self) -> usize {
        self.triangle_count
    }

    /// Get the average edge length
    pub fn average_edge_length(&self) -> f64 {
        self.average_edge_length
    }

    /// Get the maximum edge length
    pub fn maximum_edge_length(&self) -> f64 {
        self.maximum_edge_length
    }

    /// Get the geometric error
    pub fn geometric_error(&self) -> f64 {
        self.geometric_error
    }

    /// Get the visual error
    pub fn visual_error(&self) -> f64 {
        self.visual_error
    }
}

/// LOD system
pub struct LodSystem {
    /// Original mesh (highest detail)
    original_mesh: Mesh3D,
    /// LOD levels
    lod_levels: Vec<LodLevel>,
    /// LOD generation parameters
    params: LodParams,
    /// Current LOD level
    current_level: usize,
}

/// LOD generation parameters
pub struct LodParams {
    /// Number of LOD levels
    num_levels: usize,
    /// Base simplification factor (per level)
    base_simplification: f64,
    /// Maximum geometric error
    max_geometric_error: f64,
    /// Maximum visual error
    max_visual_error: f64,
    /// Use view-dependent LOD
    use_view_dependent: bool,
    /// Screen space error threshold
    screen_space_error: f64,
    /// Enable LOD transitions
    enable_transitions: bool,
    /// Transition duration (in seconds)
    transition_duration: f64,
}

impl LodParams {
    /// Create default LOD parameters
    pub fn default() -> Self {
        Self {
            num_levels: 5,
            base_simplification: 2.0,
            max_geometric_error: 0.1,
            max_visual_error: 1.0,
            use_view_dependent: true,
            screen_space_error: 2.0,
            enable_transitions: true,
            transition_duration: 0.5,
        }
    }

    /// Get the number of LOD levels
    pub fn num_levels(&self) -> usize {
        self.num_levels
    }

    /// Get the base simplification factor
    pub fn base_simplification(&self) -> f64 {
        self.base_simplification
    }

    /// Get the maximum geometric error
    pub fn max_geometric_error(&self) -> f64 {
        self.max_geometric_error
    }

    /// Get the maximum visual error
    pub fn max_visual_error(&self) -> f64 {
        self.max_visual_error
    }

    /// Get whether to use view-dependent LOD
    pub fn use_view_dependent(&self) -> bool {
        self.use_view_dependent
    }

    /// Get the screen space error threshold
    pub fn screen_space_error(&self) -> f64 {
        self.screen_space_error
    }

    /// Get whether transitions are enabled
    pub fn enable_transitions(&self) -> bool {
        self.enable_transitions
    }

    /// Get the transition duration
    pub fn transition_duration(&self) -> f64 {
        self.transition_duration
    }

    /// Set the number of LOD levels
    pub fn set_num_levels(&mut self, num_levels: usize) {
        self.num_levels = num_levels;
    }

    /// Set the base simplification factor
    pub fn set_base_simplification(&mut self, base_simplification: f64) {
        self.base_simplification = base_simplification;
    }

    /// Set the maximum geometric error
    pub fn set_max_geometric_error(&mut self, max_geometric_error: f64) {
        self.max_geometric_error = max_geometric_error;
    }

    /// Set the maximum visual error
    pub fn set_max_visual_error(&mut self, max_visual_error: f64) {
        self.max_visual_error = max_visual_error;
    }

    /// Set whether to use view-dependent LOD
    pub fn set_use_view_dependent(&mut self, use_view_dependent: bool) {
        self.use_view_dependent = use_view_dependent;
    }

    /// Set the screen space error threshold
    pub fn set_screen_space_error(&mut self, screen_space_error: f64) {
        self.screen_space_error = screen_space_error;
    }

    /// Set whether transitions are enabled
    pub fn set_enable_transitions(&mut self, enable_transitions: bool) {
        self.enable_transitions = enable_transitions;
    }

    /// Set the transition duration
    pub fn set_transition_duration(&mut self, transition_duration: f64) {
        self.transition_duration = transition_duration;
    }
}

impl LodSystem {
    /// Create a new LOD system from a mesh
    pub fn new(mesh: Mesh3D, params: LodParams) -> Self {
        let mut lod_system = Self {
            original_mesh: mesh,
            lod_levels: Vec::new(),
            params,
            current_level: 0,
        };

        // Generate LOD levels
        lod_system.generate_lod_levels();

        lod_system
    }

    /// Generate LOD levels
    fn generate_lod_levels(&mut self) {
        // Add original mesh as level 0 (highest detail)
        let (min_point, max_point) = self.original_mesh.calculate_bounding_box();
        let original_bbox = crate::geometry::BoundingBox::new(min_point, max_point);
        let original_metrics = LodQualityMetrics {
            triangle_count: self.original_mesh.faces.len(),
            average_edge_length: self.calculate_average_edge_length(&self.original_mesh),
            maximum_edge_length: self.calculate_maximum_edge_length(&self.original_mesh),
            geometric_error: 0.0,
            visual_error: 0.0,
        };

        self.lod_levels.push(LodLevel {
            level: 0,
            mesh: self.original_mesh.clone(),
            simplification_factor: 1.0,
            bounding_box: original_bbox,
            quality_metrics: original_metrics,
        });

        // Generate lower detail levels
        for level in 1..self.params.num_levels {
            let prev_level = &self.lod_levels[level - 1];
            let simplification_factor = self.params.base_simplification.powi(level as i32);

            // Simplify mesh
            let simplified_mesh = self.simplify_mesh(&prev_level.mesh, simplification_factor);
            let (min_point, max_point) = simplified_mesh.calculate_bounding_box();
            let bbox = crate::geometry::BoundingBox::new(min_point, max_point);
            let metrics = LodQualityMetrics {
                triangle_count: simplified_mesh.faces.len(),
                average_edge_length: self.calculate_average_edge_length(&simplified_mesh),
                maximum_edge_length: self.calculate_maximum_edge_length(&simplified_mesh),
                geometric_error: self
                    .calculate_geometric_error(&simplified_mesh, &self.original_mesh),
                visual_error: self.calculate_visual_error(&simplified_mesh, &self.original_mesh),
            };

            self.lod_levels.push(LodLevel {
                level,
                mesh: simplified_mesh,
                simplification_factor,
                bounding_box: bbox,
                quality_metrics: metrics,
            });
        }
    }

    /// Simplify mesh by a given factor
    fn simplify_mesh(&self, mesh: &Mesh3D, simplification_factor: f64) -> Mesh3D {
        // Edge collapse decimation for simplification
        let target_triangles = (mesh.faces.len() as f64 / simplification_factor).max(1.0) as usize;
        let mut decimated = mesh.clone();
        while decimated.faces.len() > target_triangles {
            let mut min_len = std::f64::MAX;
            let mut min_edge = None;
            for edge in &decimated.edges {
                let v0 = &decimated.vertices[edge.vertices[0]];
                let v1 = &decimated.vertices[edge.vertices[1]];
                let len = ((v0.point.x - v1.point.x).powi(2)
                    + (v0.point.y - v1.point.y).powi(2)
                    + (v0.point.z - v1.point.z).powi(2))
                .sqrt();
                if len < min_len {
                    min_len = len;
                    min_edge = Some(edge.id);
                }
            }
            if let Some(edge_id) = min_edge {
                decimated.edges.retain(|e| e.id != edge_id);
                decimated.faces.retain(|f| !f.edges.contains(&edge_id));
            } else {
                break;
            }
        }
        decimated
    }

    /// Calculate average edge length for a mesh
    fn calculate_average_edge_length(&self, mesh: &Mesh3D) -> f64 {
        // Average edge length
        let mut total = 0.0;
        let mut count = 0;
        for edge in &mesh.edges {
            let v0 = &mesh.vertices[edge.vertices[0]];
            let v1 = &mesh.vertices[edge.vertices[1]];
            total += ((v0.point.x - v1.point.x).powi(2)
                + (v0.point.y - v1.point.y).powi(2)
                + (v0.point.z - v1.point.z).powi(2))
            .sqrt();
            count += 1;
        }
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    /// Calculate maximum edge length for a mesh
    fn calculate_maximum_edge_length(&self, mesh: &Mesh3D) -> f64 {
        // Maximum edge length
        let mut max_len = 0.0;
        for edge in &mesh.edges {
            let v0 = &mesh.vertices[edge.vertices[0]];
            let v1 = &mesh.vertices[edge.vertices[1]];
            let len = ((v0.point.x - v1.point.x).powi(2)
                + (v0.point.y - v1.point.y).powi(2)
                + (v0.point.z - v1.point.z).powi(2))
            .sqrt();
            if len > max_len {
                max_len = len;
            }
        }
        max_len
    }

    /// Calculate geometric error between simplified mesh and original mesh
    fn calculate_geometric_error(&self, simplified: &Mesh3D, original: &Mesh3D) -> f64 {
        // Geometric error: average distance between corresponding vertices
        let mut total = 0.0;
        let mut count = 0;
        for (i, v) in simplified.vertices.iter().enumerate() {
            if i < original.vertices.len() {
                let o = &original.vertices[i];
                total += ((v.point.x - o.point.x).powi(2)
                    + (v.point.y - o.point.y).powi(2)
                    + (v.point.z - o.point.z).powi(2))
                .sqrt();
                count += 1;
            }
        }
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }

    /// Calculate visual error between simplified mesh and original mesh
    fn calculate_visual_error(&self, simplified: &Mesh3D, original: &Mesh3D) -> f64 {
        // Visual error: difference in triangle count
        (original.faces.len() as f64 - simplified.faces.len() as f64).abs()
            / original.faces.len() as f64
    }

    /// Select LOD level based on view parameters
    pub fn select_lod_level(
        &mut self,
        camera_position: Point,
        camera_direction: Vector,
        screen_width: f64,
        screen_height: f64,
    ) -> usize {
        if !self.params.use_view_dependent {
            // Use fixed LOD level
            return self.current_level;
        }

        // Calculate distance from camera to mesh
        let mesh_center = self.lod_levels[0].bounding_box.center();
        let distance = camera_position.distance(&mesh_center);

        // Calculate screen space error for each LOD level
        let mut best_level = 0;
        let mut min_error = f64::MAX;

        for (level, lod_level) in self.lod_levels.iter().enumerate() {
            // Calculate screen space error
            let screen_error = self.calculate_screen_space_error(
                &lod_level,
                camera_position,
                camera_direction,
                screen_width,
                screen_height,
                distance,
            );

            if screen_error < self.params.screen_space_error && screen_error < min_error {
                best_level = level;
                min_error = screen_error;
            }
        }

        self.current_level = best_level;
        best_level
    }

    /// Calculate screen space error for a LOD level
    fn calculate_screen_space_error(
        &self,
        lod_level: &LodLevel,
        _camera_position: Point,
        _camera_direction: Vector,
        screen_width: f64,
        screen_height: f64,
        distance: f64,
    ) -> f64 {
        // Screen space error: geometric error scaled by distance
        let error = lod_level.quality_metrics.geometric_error;
        error * distance / (screen_width * screen_height)
    }

    /// Get current LOD level
    pub fn get_current_level(&self) -> usize {
        self.current_level
    }

    /// Get mesh for current LOD level
    pub fn get_current_mesh(&self) -> &Mesh3D {
        &self.lod_levels[self.current_level].mesh
    }

    /// Get mesh for specific LOD level
    pub fn get_mesh_for_level(&self, level: usize) -> Option<&Mesh3D> {
        if level < self.lod_levels.len() {
            Some(&self.lod_levels[level].mesh)
        } else {
            None
        }
    }

    /// Get number of LOD levels
    pub fn get_num_levels(&self) -> usize {
        self.lod_levels.len()
    }

    /// Get LOD quality metrics for a level
    pub fn get_quality_metrics(&self, level: usize) -> Option<&LodQualityMetrics> {
        if level < self.lod_levels.len() {
            Some(&self.lod_levels[level].quality_metrics)
        } else {
            None
        }
    }

    /// Export LOD levels to files
    pub fn export_lod_levels(&self, base_path: &str) -> Result<(), std::io::Error> {
        // Export each LOD mesh to a file
        use std::fs::File;
        use std::io::Write;
        for level in &self.lod_levels {
            let path = format!("{}-lod{}.obj", base_path, level.level);
            let mut file = File::create(&path)?;
            // Write vertices
            for v in &level.mesh.vertices {
                writeln!(file, "v {} {} {}", v.point.x, v.point.y, v.point.z)?;
            }
            // Write faces
            for f in &level.mesh.faces {
                let indices: Vec<_> = f.vertices.iter().map(|i| i + 1).collect();
                writeln!(file, "f {} {} {}", indices[0], indices[1], indices[2])?;
            }
        }
        Ok(())
    }

    /// Import LOD levels from files
    pub fn import_lod_levels(base_path: &str) -> Result<Self, std::io::Error> {
        // Import LOD meshes from files
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let mut lod_levels = Vec::new();
        let mut level = 0;
        loop {
            let path = format!("{}-lod{}.obj", base_path, level);
            if let Ok(file) = File::open(&path) {
                let mut vertices = Vec::new();
                let mut faces = Vec::new();
                for line in BufReader::new(file).lines() {
                    let line = line?;
                    if line.starts_with("v ") {
                        let parts: Vec<_> = line.split_whitespace().collect();
                        let x = parts[1].parse().unwrap_or(0.0);
                        let y = parts[2].parse().unwrap_or(0.0);
                        let z = parts[3].parse().unwrap_or(0.0);
                        vertices.push(crate::mesh::mesh_data::MeshVertex::new(
                            vertices.len(),
                            Point::new(x, y, z),
                        ));
                    } else if line.starts_with("f ") {
                        let parts: Vec<_> = line.split_whitespace().collect();
                        let v0 = parts[1].parse::<usize>().unwrap_or(1) - 1;
                        let v1 = parts[2].parse::<usize>().unwrap_or(1) - 1;
                        let v2 = parts[3].parse::<usize>().unwrap_or(1) - 1;
                        faces.push(crate::mesh::mesh_data::MeshFace::new(
                            faces.len(),
                            vec![v0, v1, v2],
                        ));
                    }
                }
                let mesh = Mesh3D {
                    vertices,
                    faces,
                    ..Default::default()
                };
                let (min_point, max_point) = mesh.calculate_bounding_box();
                let triangle_count = mesh.faces.len();
                lod_levels.push(LodLevel {
                    level,
                    mesh,
                    simplification_factor: 1.0,
                    bounding_box: crate::geometry::BoundingBox::new(min_point, max_point),
                    quality_metrics: LodQualityMetrics {
                        triangle_count,
                        average_edge_length: 0.0,
                        maximum_edge_length: 0.0,
                        geometric_error: 0.0,
                        visual_error: 0.0,
                    },
                });
                level += 1;
            } else {
                break;
            }
        }
        Ok(Self {
            original_mesh: lod_levels
                .get(0)
                .map(|l| l.mesh.clone())
                .unwrap_or_default(),
            lod_levels,
            params: LodParams::default(),
            current_level: 0,
        })
    }
}

/// LOD transition manager
pub struct LodTransitionManager {
    /// Current transition state
    current_transition: Option<LodTransition>,
    /// Transition parameters
    params: LodTransitionParams,
}

/// LOD transition parameters
pub struct LodTransitionParams {
    /// Transition duration (in seconds)
    duration: f64,
    /// Use cross-fade transition
    use_cross_fade: bool,
    /// Use morph transition
    use_morph: bool,
}

impl LodTransitionParams {
    /// Create default transition parameters
    pub fn default() -> Self {
        Self {
            duration: 0.5,
            use_cross_fade: true,
            use_morph: false,
        }
    }

    /// Get the transition duration
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Set the transition duration
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    /// Get whether to use cross-fade transition
    pub fn use_cross_fade(&self) -> bool {
        self.use_cross_fade
    }

    /// Set whether to use cross-fade transition
    pub fn set_use_cross_fade(&mut self, use_cross_fade: bool) {
        self.use_cross_fade = use_cross_fade;
    }

    /// Get whether to use morph transition
    pub fn use_morph(&self) -> bool {
        self.use_morph
    }

    /// Set whether to use morph transition
    pub fn set_use_morph(&mut self, use_morph: bool) {
        self.use_morph = use_morph;
    }
}

/// LOD transition state
pub struct LodTransition {
    /// Start time (in seconds since epoch)
    start_time: f64,
    /// From LOD level
    from_level: usize,
    /// To LOD level
    to_level: usize,
    /// Progress (0.0 to 1.0)
    progress: f64,
}

impl LodTransition {
    /// Create a new LOD transition
    pub fn new(from_level: usize, to_level: usize) -> Self {
        Self {
            start_time: 0.0,
            from_level,
            to_level,
            progress: 0.0,
        }
    }

    /// Get the start time
    pub fn start_time(&self) -> f64 {
        self.start_time
    }

    /// Set the start time
    pub fn set_start_time(&mut self, start_time: f64) {
        self.start_time = start_time;
    }

    /// Get the from level
    pub fn from_level(&self) -> usize {
        self.from_level
    }

    /// Get the to level
    pub fn to_level(&self) -> usize {
        self.to_level
    }

    /// Get the progress
    pub fn progress(&self) -> f64 {
        self.progress
    }

    /// Set the progress
    pub fn set_progress(&mut self, progress: f64) {
        self.progress = progress;
    }
}

impl LodTransitionManager {
    /// Create a new LOD transition manager
    pub fn new(params: LodTransitionParams) -> Self {
        Self {
            current_transition: None,
            params,
        }
    }

    /// Start a new transition
    pub fn start_transition(&mut self, from_level: usize, to_level: usize) {
        self.current_transition = Some(LodTransition {
            start_time: self.get_current_time(),
            from_level,
            to_level,
            progress: 0.0,
        });
    }

    /// Update transition progress
    pub fn update_transition(&mut self) -> bool {
        // Get current time first to avoid borrowing conflicts
        let current_time = self.get_current_time();

        if let Some(transition) = &mut self.current_transition {
            let elapsed = current_time - transition.start_time;
            transition.progress = (elapsed / self.params.duration).clamp(0.0, 1.0);

            if transition.progress >= 1.0 {
                self.current_transition = None;
                return true;
            }
        }

        false
    }

    /// Get current transition state
    pub fn get_current_transition(&self) -> Option<&LodTransition> {
        self.current_transition.as_ref()
    }

    /// Get current time in seconds since epoch
    fn get_current_time(&self) -> f64 {
        // Get current time using std::time
        use std::time::{SystemTime, UNIX_EPOCH};

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs_f64(),
            Err(_) => 0.0,
        }
    }
}

/// LOD-aware collision detection
pub struct LodCollisionDetector {
    /// LOD system
    lod_system: LodSystem,
    /// Collision detection parameters
    params: CollisionParams,
}

/// Collision detection parameters
pub struct CollisionParams {
    /// Use LOD for collision detection
    use_lod: bool,
    /// Minimum LOD level for collision detection
    min_lod_level: usize,
    /// Maximum LOD level for collision detection
    max_lod_level: usize,
    /// Collision tolerance
    tolerance: f64,
}

impl LodCollisionDetector {
    /// Create a new LOD collision detector
    pub fn new(lod_system: LodSystem, params: CollisionParams) -> Self {
        Self { lod_system, params }
    }

    /// Get the LOD system
    pub fn lod_system(&self) -> &LodSystem {
        &self.lod_system
    }

    /// Get the collision parameters
    pub fn params(&self) -> &CollisionParams {
        &self.params
    }

    /// Check collision with a point
    pub fn check_point_collision(&self, point: Point) -> bool {
        // Get current LOD mesh
        let current_mesh = self.lod_system.get_current_mesh();

        // Check if point is inside any face (simplified implementation)
        for face in &current_mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &current_mesh.vertices[face.vertices[0]];
                let v1 = &current_mesh.vertices[face.vertices[1]];
                let v2 = &current_mesh.vertices[face.vertices[2]];

                // Calculate normal vector
                let vec1 = Vector::new(
                    v1.point.x - v0.point.x,
                    v1.point.y - v0.point.y,
                    v1.point.z - v0.point.z,
                );
                let vec2 = Vector::new(
                    v2.point.x - v0.point.x,
                    v2.point.y - v0.point.y,
                    v2.point.z - v0.point.z,
                );
                let normal = vec1.cross(&vec2);

                // Calculate distance from point to plane
                let plane_eq = normal.dot(&Vector::new(v0.point.x, v0.point.y, v0.point.z));
                let point_vec = Vector::new(point.x, point.y, point.z);
                let distance = normal.dot(&point_vec) - plane_eq;

                if distance.abs() < self.params.tolerance {
                    // Point is on the plane, check if it's inside the triangle
                    let barycentric = self.calculate_barycentric(&point, v0, v1, v2);
                    if barycentric.0 >= 0.0
                        && barycentric.1 >= 0.0
                        && (barycentric.0 + barycentric.1) <= 1.0
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check collision with a ray
    pub fn check_ray_collision(&self, origin: Point, direction: Vector) -> Option<Point> {
        // Get current LOD mesh
        let current_mesh = self.lod_system.get_current_mesh();

        // Check ray intersection with each face
        let mut closest_hit: Option<(Point, f64)> = None;

        for face in &current_mesh.faces {
            if face.vertices.len() >= 3 {
                let v0 = &current_mesh.vertices[face.vertices[0]];
                let v1 = &current_mesh.vertices[face.vertices[1]];
                let v2 = &current_mesh.vertices[face.vertices[2]];

                // Calculate normal vector
                let vec1 = Vector::new(
                    v1.point.x - v0.point.x,
                    v1.point.y - v0.point.y,
                    v1.point.z - v0.point.z,
                );
                let vec2 = Vector::new(
                    v2.point.x - v0.point.x,
                    v2.point.y - v0.point.y,
                    v2.point.z - v0.point.z,
                );
                let normal = vec1.cross(&vec2);

                // Calculate plane equation
                let plane_eq = normal.dot(&Vector::new(v0.point.x, v0.point.y, v0.point.z));

                // Calculate ray direction dot normal
                let dir_dot_normal = normal.dot(&direction);
                if dir_dot_normal.abs() < 1e-6 {
                    // Ray is parallel to plane
                    continue;
                }

                // Calculate t parameter
                let origin_vec = Vector::new(origin.x, origin.y, origin.z);
                let t = (plane_eq - normal.dot(&origin_vec)) / dir_dot_normal;

                if t < 0.0 {
                    // Intersection is behind the ray origin
                    continue;
                }

                // Calculate intersection point
                let intersection = Point::new(
                    origin.x + t * direction.x,
                    origin.y + t * direction.y,
                    origin.z + t * direction.z,
                );

                // Check if intersection is inside the triangle
                let barycentric = self.calculate_barycentric(&intersection, v0, v1, v2);
                if barycentric.0 >= 0.0
                    && barycentric.1 >= 0.0
                    && (barycentric.0 + barycentric.1) <= 1.0
                {
                    // Update closest hit
                    match closest_hit {
                        None => closest_hit = Some((intersection, t)),
                        Some((_, current_t)) if t < current_t => {
                            closest_hit = Some((intersection, t))
                        }
                        _ => {}
                    }
                }
            }
        }

        closest_hit.map(|(point, _)| point)
    }

    /// Check collision with another mesh
    pub fn check_mesh_collision(&self, other_mesh: &Mesh3D) -> bool {
        // Get current LOD mesh
        let current_mesh = self.lod_system.get_current_mesh();

        // Simple bounding box check first
        let (self_min, self_max) = current_mesh.calculate_bounding_box();
        let (other_min, other_max) = other_mesh.calculate_bounding_box();

        // Check if bounding boxes overlap
        if self_max.x < other_min.x || self_min.x > other_max.x {
            return false;
        }
        if self_max.y < other_min.y || self_min.y > other_max.y {
            return false;
        }
        if self_max.z < other_min.z || self_min.z > other_max.z {
            return false;
        }

        // For a more accurate check, we would check each triangle pair
        // For now, just return true if bounding boxes overlap
        true
    }

    /// Calculate barycentric coordinates for a point in a triangle
    fn calculate_barycentric(
        &self,
        point: &Point,
        v0: &MeshVertex,
        v1: &MeshVertex,
        v2: &MeshVertex,
    ) -> (f64, f64) {
        let vec0 = Vector::new(
            v2.point.x - v0.point.x,
            v2.point.y - v0.point.y,
            v2.point.z - v0.point.z,
        );
        let vec1 = Vector::new(
            v1.point.x - v0.point.x,
            v1.point.y - v0.point.y,
            v1.point.z - v0.point.z,
        );
        let vec2 = Vector::new(
            point.x - v0.point.x,
            point.y - v0.point.y,
            point.z - v0.point.z,
        );

        let dot00 = vec0.dot(&vec0);
        let dot01 = vec0.dot(&vec1);
        let dot02 = vec0.dot(&vec2);
        let dot11 = vec1.dot(&vec1);
        let dot12 = vec1.dot(&vec2);

        let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

        (u, v)
    }
}

impl CollisionParams {
    /// Create default collision parameters
    pub fn default() -> Self {
        Self {
            use_lod: true,
            min_lod_level: 0,
            max_lod_level: 3,
            tolerance: 0.001,
        }
    }

    /// Get whether to use LOD for collision detection
    pub fn use_lod(&self) -> bool {
        self.use_lod
    }

    /// Set whether to use LOD for collision detection
    pub fn set_use_lod(&mut self, use_lod: bool) {
        self.use_lod = use_lod;
    }

    /// Get the minimum LOD level
    pub fn min_lod_level(&self) -> usize {
        self.min_lod_level
    }

    /// Set the minimum LOD level
    pub fn set_min_lod_level(&mut self, min_lod_level: usize) {
        self.min_lod_level = min_lod_level;
    }

    /// Get the maximum LOD level
    pub fn max_lod_level(&self) -> usize {
        self.max_lod_level
    }

    /// Set the maximum LOD level
    pub fn set_max_lod_level(&mut self, max_lod_level: usize) {
        self.max_lod_level = max_lod_level;
    }

    /// Get the collision tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the collision tolerance
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }
}

/// LOD debugging and visualization tools
pub struct LodDebugger {
    /// LOD system
    lod_system: LodSystem,
    /// Debug visualization parameters
    params: DebugParams,
}

/// Debug visualization parameters
pub struct DebugParams {
    /// Show LOD levels
    show_lod_levels: bool,
    /// Show bounding boxes
    show_bounding_boxes: bool,
    /// Show error metrics
    show_error_metrics: bool,
    /// Show transition states
    show_transitions: bool,
}

impl LodDebugger {
    /// Create a new LOD debugger
    pub fn new(lod_system: LodSystem, params: DebugParams) -> Self {
        Self { lod_system, params }
    }

    /// Get the LOD system
    pub fn lod_system(&self) -> &LodSystem {
        &self.lod_system
    }

    /// Get the debug parameters
    pub fn params(&self) -> &DebugParams {
        &self.params
    }

    /// Visualize LOD levels
    pub fn visualize_lod_levels(&self) {
        // Log LOD levels information
        println!("=== LOD Levels Visualization ===");
        println!("Number of LOD levels: {}", self.lod_system.get_num_levels());

        for i in 0..self.lod_system.get_num_levels() {
            if let Some(metrics) = self.lod_system.get_quality_metrics(i) {
                println!("LOD Level {}:", i);
                println!("  Triangle count: {}", metrics.triangle_count());
                println!(
                    "  Average edge length: {:.3}",
                    metrics.average_edge_length()
                );
                println!(
                    "  Maximum edge length: {:.3}",
                    metrics.maximum_edge_length()
                );
                println!("  Geometric error: {:.6}", metrics.geometric_error());
                println!("  Visual error: {:.6}", metrics.visual_error());
            }
        }
        println!("=============================");
    }

    /// Visualize bounding boxes
    pub fn visualize_bounding_boxes(&self) {
        // Log bounding boxes information
        println!("=== Bounding Boxes Visualization ===");

        for i in 0..self.lod_system.get_num_levels() {
            if let Some(mesh) = self.lod_system.get_mesh_for_level(i) {
                let (min_point, max_point) = mesh.calculate_bounding_box();
                println!("LOD Level {} Bounding Box:", i);
                println!(
                    "  Min: ({:.3}, {:.3}, {:.3})",
                    min_point.x, min_point.y, min_point.z
                );
                println!(
                    "  Max: ({:.3}, {:.3}, {:.3})",
                    max_point.x, max_point.y, max_point.z
                );
            }
        }
        println!("================================");
    }

    /// Visualize error metrics
    pub fn visualize_error_metrics(&self) {
        // Log error metrics information
        println!("=== Error Metrics Visualization ===");

        for i in 0..self.lod_system.get_num_levels() {
            if let Some(metrics) = self.lod_system.get_quality_metrics(i) {
                println!("LOD Level {} Error Metrics:", i);
                println!("  Geometric error: {:.6}", metrics.geometric_error());
                println!("  Visual error: {:.6}", metrics.visual_error());
            }
        }
        println!("==============================");
    }

    /// Visualize transition states
    pub fn visualize_transitions(&self) {
        // Log transition states information
        println!("=== Transition States Visualization ===");
        println!("Current LOD level: {}", self.lod_system.get_current_level());
        println!("====================================");
    }
}

impl DebugParams {
    /// Create default debug parameters
    pub fn default() -> Self {
        Self {
            show_lod_levels: true,
            show_bounding_boxes: true,
            show_error_metrics: true,
            show_transitions: true,
        }
    }

    /// Get whether to show LOD levels
    pub fn show_lod_levels(&self) -> bool {
        self.show_lod_levels
    }

    /// Set whether to show LOD levels
    pub fn set_show_lod_levels(&mut self, show_lod_levels: bool) {
        self.show_lod_levels = show_lod_levels;
    }

    /// Get whether to show bounding boxes
    pub fn show_bounding_boxes(&self) -> bool {
        self.show_bounding_boxes
    }

    /// Set whether to show bounding boxes
    pub fn set_show_bounding_boxes(&mut self, show_bounding_boxes: bool) {
        self.show_bounding_boxes = show_bounding_boxes;
    }

    /// Get whether to show error metrics
    pub fn show_error_metrics(&self) -> bool {
        self.show_error_metrics
    }

    /// Set whether to show error metrics
    pub fn set_show_error_metrics(&mut self, show_error_metrics: bool) {
        self.show_error_metrics = show_error_metrics;
    }

    /// Get whether to show transitions
    pub fn show_transitions(&self) -> bool {
        self.show_transitions
    }

    /// Set whether to show transitions
    pub fn set_show_transitions(&mut self, show_transitions: bool) {
        self.show_transitions = show_transitions;
    }
}
