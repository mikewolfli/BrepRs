use crate::foundation::StandardReal;
use crate::geometry::{ffd::FFD, Point, Vector};
use crate::topology::{TopoDsFace, TopoDsShape};
use std::collections::{HashMap, HashSet};

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
}

/// Vertex constraint type
#[derive(Debug, Clone, PartialEq)]
pub enum VertexConstraint {
    /// Fixed vertex (cannot move)
    Fixed,
    /// Free vertex (can move freely)
    Free,
    /// Constrained to a plane
    Plane(Point, Vector),
    /// Constrained to a line
    Line(Point, Vector),
    /// Constrained to a surface
    Surface(TopoDsFace),
}

/// Dynamic surface update parameters
#[derive(Debug, Clone)]
pub struct UpdateParameters {
    /// Maximum displacement per vertex
    pub max_displacement: StandardReal,
    /// Smoothing factor
    pub smoothing_factor: StandardReal,
    /// Use FFD for deformation
    pub use_ffd: bool,
    /// FFD grid resolution
    pub ffd_resolution: (usize, usize, usize),
}

impl DynamicSurface {
    /// Create a new dynamic surface from a shape
    pub fn new(surface: TopoDsShape) -> Self {
        // TODO: Extract vertices, edges, and faces from the surface
        let vertices = Vec::new();
        let vertex_map = HashMap::new();
        let edges = Vec::new();
        let faces = Vec::new();
        let constraints = Vec::new();

        Self {
            original_surface: surface,
            vertices,
            vertex_map,
            edges,
            faces,
            constraints,
            ffd: None,
        }
    }

    /// Initialize FFD for the surface
    pub fn initialize_ffd(&mut self, resolution: (usize, usize, usize)) {
        // Calculate bounding box of vertices
        if self.vertices.is_empty() {
            return;
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

        // Create FFD grid
        self.ffd = Some(FFD::create_regular_grid(
            min_point,
            max_point,
            resolution.0,
            resolution.1,
            resolution.2,
        ));
    }

    /// Set vertex constraint
    pub fn set_vertex_constraint(&mut self, vertex_idx: usize, constraint: VertexConstraint) {
        if vertex_idx < self.constraints.len() {
            self.constraints[vertex_idx] = constraint;
        } else {
            // Extend constraints vector if needed
            while self.constraints.len() <= vertex_idx {
                self.constraints.push(VertexConstraint::Free);
            }
            self.constraints[vertex_idx] = constraint;
        }
    }

    /// Update vertex positions
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

        // Apply smoothing
        if params.smoothing_factor > 0.0 {
            self.smooth_surface(params.smoothing_factor);
        }

        // Apply FFD if enabled
        if params.use_ffd && self.ffd.is_some() {
            self.apply_ffd();
        }
    }

    /// Apply vertex constraint
    fn apply_constraint(&self, vertex_idx: usize, position: &Point) -> Point {
        if vertex_idx >= self.constraints.len() {
            return *position;
        }

        match &self.constraints[vertex_idx] {
            VertexConstraint::Fixed => {
                // Keep original position
                self.vertices[vertex_idx]
            }
            VertexConstraint::Free => {
                // Allow free movement
                *position
            }
            VertexConstraint::Plane(plane_point, plane_normal) => {
                // Project to plane
                let vector = *position - *plane_point;
                let distance = vector.dot(plane_normal);
                *position + (*plane_normal) * (-distance)
            }
            VertexConstraint::Line(line_point, line_direction) => {
                // Project to line
                let vector = *position - *line_point;
                let projection = *line_point + (*line_direction) * vector.dot(line_direction);
                projection
            }
            VertexConstraint::Surface(_surface) => {
                // TODO: Project to surface
                *position
            }
        }
    }

    /// Smooth the surface
    fn smooth_surface(&mut self, factor: StandardReal) {
        let original_vertices = self.vertices.clone();

        // Pre-compute neighbors for all vertices to avoid borrow conflicts
        let all_neighbors: Vec<Vec<usize>> = (0..self.vertices.len())
            .map(|i| self.find_neighbors(i))
            .collect();

        for (i, vertex) in self.vertices.iter_mut().enumerate() {
            // Skip fixed vertices
            if self.constraints.len() > i && self.constraints[i] == VertexConstraint::Fixed {
                continue;
            }

            // Get pre-computed neighbors
            let neighbors = &all_neighbors[i];
            if neighbors.is_empty() {
                continue;
            }

            // Calculate average position of neighbors
            let mut avg_position = Vector::new(0.0, 0.0, 0.0);
            for neighbor in neighbors {
                avg_position = avg_position + (original_vertices[*neighbor] - Point::origin());
            }
            avg_position = avg_position / neighbors.len() as StandardReal;
            let avg_point = Point::origin() + avg_position;

            // Move vertex towards average position
            *vertex = *vertex + (avg_point - *vertex) * factor;
        }
    }

    /// Find neighboring vertices
    fn find_neighbors(&self, vertex_idx: usize) -> Vec<usize> {
        let mut neighbors = HashSet::new();

        // Find all edges connected to this vertex
        for (v1, v2) in &self.edges {
            if *v1 == vertex_idx {
                neighbors.insert(*v2);
            } else if *v2 == vertex_idx {
                neighbors.insert(*v1);
            }
        }

        neighbors.into_iter().collect()
    }

    /// Apply FFD deformation
    fn apply_ffd(&mut self) {
        if let Some(ffd) = &self.ffd {
            for vertex in &mut self.vertices {
                *vertex = ffd.deform_point(vertex);
            }
        }
    }

    /// Update FFD control points
    pub fn update_ffd_control_point(&mut self, u: usize, v: usize, w: usize, point: Point) {
        if let Some(ffd) = &mut self.ffd {
            ffd.set_control_point(u, v, w, point);
        }
    }

    /// Generate the updated surface
    pub fn to_surface(&self) -> TopoDsShape {
        // TODO: Reconstruct surface from updated vertices
        // For now, return original surface
        self.original_surface.clone()
    }

    /// Check if topology is stable
    pub fn is_topology_stable(&self) -> bool {
        // TODO: Implement topology stability check
        true
    }
}

impl UpdateParameters {
    /// Create default update parameters
    pub fn default() -> Self {
        Self {
            max_displacement: 0.1,
            smoothing_factor: 0.1,
            use_ffd: false,
            ffd_resolution: (3, 3, 3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::topology::{TopoDsShape, shape_enum::ShapeType};

    #[test]
    fn test_dynamic_surface_creation() {
        let surface = TopoDsShape::new(ShapeType::Compound);
        let dynamic_surface = DynamicSurface::new(surface);

        assert!(dynamic_surface.vertices.is_empty());
        assert!(dynamic_surface.edges.is_empty());
        assert!(dynamic_surface.faces.is_empty());
    }

    #[test]
    fn test_vertex_constraint() {
        let surface = TopoDsShape::new(ShapeType::Compound);
        let mut dynamic_surface = DynamicSurface::new(surface);

        // Add a vertex
        dynamic_surface.vertices.push(Point::new(0.0, 0.0, 0.0));

        // Set fixed constraint
        dynamic_surface.set_vertex_constraint(0, VertexConstraint::Fixed);

        // Try to update the vertex
        let new_position = Point::new(1.0, 1.0, 1.0);
        dynamic_surface.update_vertices(&[(0, new_position)], &UpdateParameters::default());

        // Vertex should remain fixed
        assert_eq!(dynamic_surface.vertices[0], Point::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_surface_smoothing() {
        let surface = TopoDsShape::new(ShapeType::Compound);
        let mut dynamic_surface = DynamicSurface::new(surface);

        // Add vertices
        dynamic_surface.vertices.push(Point::new(0.0, 0.0, 0.0));
        dynamic_surface.vertices.push(Point::new(1.0, 0.0, 0.0));
        dynamic_surface.vertices.push(Point::new(0.0, 1.0, 0.0));

        // Add edges
        dynamic_surface.edges.push((0, 1));
        dynamic_surface.edges.push((1, 2));
        dynamic_surface.edges.push((2, 0));

        // Add constraints
        dynamic_surface.set_vertex_constraint(0, VertexConstraint::Fixed);
        dynamic_surface.set_vertex_constraint(1, VertexConstraint::Fixed);
        dynamic_surface.set_vertex_constraint(2, VertexConstraint::Free);

        // Move vertex 2
        let new_position = Point::new(0.0, 2.0, 0.0);
        dynamic_surface.update_vertices(
            &[(2, new_position)],
            &UpdateParameters {
                smoothing_factor: 0.5,
                ..UpdateParameters::default()
            },
        );

        // Vertex 2 should be smoothed towards the average of its neighbors
        assert!((dynamic_surface.vertices[2].y - 1.5).abs() < 1e-6);
    }
}
