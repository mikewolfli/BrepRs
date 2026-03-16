use crate::foundation::StandardReal;
use crate::geometry::{ffd::FFD, subdivision_surface::SubdivisionSurface, Point};
use crate::topology::{TopoDsShell, TopoDsSolid};

/// Soft tissue parameters
#[derive(Debug, Clone)]
pub struct SoftTissueParameters {
    /// Subdivision level
    pub subdivision_level: usize,
    /// Smoothness factor
    pub smoothness: StandardReal,
    /// Stiffness (0.0-1.0)
    pub stiffness: StandardReal,
    /// Damping factor
    pub damping: StandardReal,
    /// Max iterations for relaxation
    pub max_iterations: usize,
}

/// Soft tissue geometry
#[derive(Debug, Clone)]
pub struct SoftTissue {
    /// Base surface
    pub base_surface: TopoDsSolid,
    /// Subdivision surface
    pub subdivision_surface: SubdivisionSurface,
    /// FFD for deformation
    pub ffd: Option<FFD>,
    /// Parameters
    pub parameters: SoftTissueParameters,
}

impl SoftTissue {
    /// Create a new soft tissue from a base surface
    pub fn new(base_surface: TopoDsSolid, parameters: SoftTissueParameters) -> Self {
        // 基础提取：假设 base_surface 有 get_vertices/get_faces 方法
        let vertices = base_surface.get_vertices();
        let faces = base_surface.get_faces();
        let edges = base_surface.get_edges();

        let subdivision_surface =
            SubdivisionSurface::new(vertices, faces, edges, Default::default());

        Self {
            base_surface,
            subdivision_surface,
            ffd: None,
            parameters,
        }
    }

    /// Generate smooth geometry
    pub fn generate_smooth_geometry(&mut self) -> TopoDsSolid {
        // Apply subdivision
        let mut subdivided = self.subdivision_surface.clone();
        for _ in 0..self.parameters.subdivision_level {
            subdivided = subdivided.subdivide();
        }

        // Apply relaxation to smooth the surface
        let relaxed = self.relax_surface(subdivided);

        // Convert to solid
        self.subdivision_to_solid(relaxed)
    }

    /// Relax the surface to make it smoother
    fn relax_surface(&self, surface: SubdivisionSurface) -> SubdivisionSurface {
        let mut relaxed = surface;

        for _ in 0..self.parameters.max_iterations {
            let mut new_vertices = relaxed.vertices.clone();

            for (i, vertex) in relaxed.vertices.iter().enumerate() {
                // Find neighboring vertices
                let neighbors = self.find_neighbors(&relaxed, i);
                if neighbors.is_empty() {
                    continue;
                }

                // Calculate average position
                let mut avg_position = Point::origin();
                for neighbor in &neighbors {
                    avg_position = avg_position + (relaxed.vertices[*neighbor] - Point::origin());
                }
                avg_position = avg_position / neighbors.len() as StandardReal;

                // Move vertex towards average position
                new_vertices[i] = *vertex + (avg_position - *vertex) * self.parameters.smoothness;
            }

            relaxed.vertices = new_vertices;
        }

        relaxed
    }

    /// Find neighboring vertices for a given vertex
    fn find_neighbors(&self, surface: &SubdivisionSurface, vertex_idx: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();

        // Check all edges
        for &(v1, v2) in &surface.edges {
            if v1 == vertex_idx {
                neighbors.push(v2);
            } else if v2 == vertex_idx {
                neighbors.push(v1);
            }
        }

        neighbors
    }

    /// Convert subdivision surface to solid
    fn subdivision_to_solid(&self, surface: SubdivisionSurface) -> TopoDsSolid {
        let mut solid = TopoDsSolid::new();
        let mut shell = TopoDsShell::new();

        // TODO: Convert subdivision surface to faces
        // For now, return an empty solid
        solid.add_shell(shell);
        solid
    }

    /// Apply deformation to the soft tissue
    pub fn deform(&mut self, deformation: &dyn Fn(Point) -> Point) {
        // Apply deformation to each vertex
        let mut new_vertices = self.subdivision_surface.vertices.clone();
        for vertex in &mut new_vertices {
            *vertex = deformation(*vertex);
        }

        self.subdivision_surface.vertices = new_vertices;

        // Re-generate smooth geometry
        self.generate_smooth_geometry();
    }

    /// Apply FFD deformation
    pub fn deform_with_ffd(&mut self, ffd: &FFD) {
        // Apply FFD to each vertex
        let mut new_vertices = self.subdivision_surface.vertices.clone();
        for vertex in &mut new_vertices {
            *vertex = ffd.deform_point(vertex);
        }

        self.subdivision_surface.vertices = new_vertices;
        self.ffd = Some(ffd.clone());

        // Re-generate smooth geometry
        self.generate_smooth_geometry();
    }

    /// Calculate the surface area
    pub fn surface_area(&self) -> StandardReal {
        // TODO: Implement surface area calculation
        0.0
    }

    /// Calculate the volume
    pub fn volume(&self) -> StandardReal {
        // TODO: Implement volume calculation
        0.0
    }
}

impl SoftTissueParameters {
    /// Create default soft tissue parameters
    pub fn default() -> Self {
        Self {
            subdivision_level: 2,
            smoothness: 0.1,
            stiffness: 0.5,
            damping: 0.1,
            max_iterations: 10,
        }
    }

    /// Create parameters for very soft tissue
    pub fn soft() -> Self {
        Self {
            subdivision_level: 3,
            smoothness: 0.2,
            stiffness: 0.1,
            damping: 0.2,
            max_iterations: 20,
        }
    }

    /// Create parameters for stiff tissue
    pub fn stiff() -> Self {
        Self {
            subdivision_level: 1,
            smoothness: 0.05,
            stiffness: 0.9,
            damping: 0.05,
            max_iterations: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::topology::TopoDsShape;

    #[test]
    fn test_soft_tissue_creation() {
        let surface = TopoDsShape::new();
        let params = SoftTissueParameters::default();
        let soft_tissue = SoftTissue::new(surface, params);

        assert_eq!(soft_tissue.parameters.subdivision_level, 2);
        assert_eq!(soft_tissue.parameters.smoothness, 0.1);
    }

    #[test]
    fn test_soft_tissue_parameters() {
        let soft_params = SoftTissueParameters::soft();
        assert_eq!(soft_params.subdivision_level, 3);
        assert_eq!(soft_params.stiffness, 0.1);

        let stiff_params = SoftTissueParameters::stiff();
        assert_eq!(stiff_params.subdivision_level, 1);
        assert_eq!(stiff_params.stiffness, 0.9);
    }

    #[test]
    fn test_smooth_geometry_generation() {
        let surface = TopoDsShape::new();
        let params = SoftTissueParameters::default();
        let mut soft_tissue = SoftTissue::new(surface, params);

        let solid = soft_tissue.generate_smooth_geometry();
        assert!(!solid.is_empty());
    }
}
