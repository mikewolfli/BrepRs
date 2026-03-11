//! Hexahedral mesh generation
//!
//! This module provides functionality for generating structured hexahedral meshes
//! for regular geometries like boxes, cylinders, and other parametric shapes.

use super::mesh_data::{Mesh3D, MeshHexahedron};
use crate::geometry::Point;

/// Hexahedral mesher error types
#[derive(Debug)]
pub enum HexMesherError {
    /// Invalid input parameters
    InvalidParameters,
    /// Meshing failed
    MeshingFailed,
    /// Empty input
    EmptyInput,
}

/// Hexahedral mesher parameters
#[derive(Debug, Clone)]
pub struct HexMesherParams {
    /// Number of divisions in x direction
    pub nx: usize,
    /// Number of divisions in y direction
    pub ny: usize,
    /// Number of divisions in z direction
    pub nz: usize,
    /// Use quality mesh
    pub quality_mesh: bool,
    /// Maximum hex aspect ratio
    pub max_aspect_ratio: f64,
    /// Minimum hex angle (in degrees)
    pub min_hex_angle: f64,
}

impl Default for HexMesherParams {
    fn default() -> Self {
        Self {
            nx: 10,
            ny: 10,
            nz: 10,
            quality_mesh: true,
            max_aspect_ratio: 5.0,
            min_hex_angle: 45.0,
        }
    }
}

/// Hexahedral mesher
pub struct HexMesher {
    /// Mesher parameters
    params: HexMesherParams,
    /// Input geometry
    input_geometry: HexGeometry,
}

/// Hexahedral geometry types
#[derive(Debug, Clone)]
pub enum HexGeometry {
    /// Box geometry
    Box {
        /// Minimum corner
        min: Point,
        /// Maximum corner
        max: Point,
    },
    /// Cylinder geometry
    Cylinder {
        /// Bottom center
        bottom_center: Point,
        /// Top center
        top_center: Point,
        /// Radius
        radius: f64,
    },
    /// Sphere geometry
    Sphere {
        /// Center
        center: Point,
        /// Radius
        radius: f64,
    },
    /// Custom geometry
    Custom {
        /// Vertices
        vertices: Vec<Point>,
        /// Hexahedron definitions
        hexahedrons: Vec<[usize; 8]>,
    },
}

impl HexMesher {
    /// Create a new hexahedral mesher
    pub fn new(params: HexMesherParams, geometry: HexGeometry) -> Self {
        Self {
            params,
            input_geometry: geometry,
        }
    }

    /// Generate hexahedral mesh
    pub fn generate(&mut self) -> Result<Mesh3D, HexMesherError> {
        match &self.input_geometry {
            HexGeometry::Box { min, max } => self.generate_box_mesh(min, max),
            HexGeometry::Cylinder {
                bottom_center,
                top_center,
                radius,
            } => self.generate_cylinder_mesh(bottom_center, top_center, *radius),
            HexGeometry::Sphere { center, radius } => self.generate_sphere_mesh(center, *radius),
            HexGeometry::Custom {
                vertices,
                hexahedrons,
            } => self.generate_custom_mesh(vertices, hexahedrons),
        }
    }

    /// Generate box mesh
    fn generate_box_mesh(&self, min: &Point, max: &Point) -> Result<Mesh3D, HexMesherError> {
        let mut mesh = Mesh3D::new();

        // Calculate cell sizes
        let dx = (max.x - min.x) / self.params.nx as f64;
        let dy = (max.y - min.y) / self.params.ny as f64;
        let dz = (max.z - min.z) / self.params.nz as f64;

        // Generate vertices
        let mut vertices = Vec::new();
        for k in 0..=self.params.nz {
            for j in 0..=self.params.ny {
                for i in 0..=self.params.nx {
                    let x = min.x + i as f64 * dx;
                    let y = min.y + j as f64 * dy;
                    let z = min.z + k as f64 * dz;
                    let vertex_id = mesh.add_vertex(Point::new(x, y, z));
                    vertices.push(vertex_id);
                }
            }
        }

        // Generate hexahedrons
        for k in 0..self.params.nz {
            for j in 0..self.params.ny {
                for i in 0..self.params.nx {
                    // Calculate vertex indices for this hexahedron
                    let v0 = k * (self.params.nx + 1) * (self.params.ny + 1)
                        + j * (self.params.nx + 1)
                        + i;
                    let v1 = v0 + 1;
                    let v2 = v0 + (self.params.nx + 1);
                    let v3 = v2 + 1;
                    let v4 = v0 + (self.params.nx + 1) * (self.params.ny + 1);
                    let v5 = v4 + 1;
                    let v6 = v4 + (self.params.nx + 1);
                    let v7 = v6 + 1;

                    // Add hexahedron
                    mesh.add_hexahedron(
                        vertices[v0],
                        vertices[v1],
                        vertices[v2],
                        vertices[v3],
                        vertices[v4],
                        vertices[v5],
                        vertices[v6],
                        vertices[v7],
                    );
                }
            }
        }

        // Optimize mesh if needed
        if self.params.quality_mesh {
            self.optimize_mesh(&mut mesh);
        }

        Ok(mesh)
    }

    /// Generate cylinder mesh
    fn generate_cylinder_mesh(
        &self,
        bottom_center: &Point,
        top_center: &Point,
        radius: f64,
    ) -> Result<Mesh3D, HexMesherError> {
        let mut mesh = Mesh3D::new();

        // Calculate cylinder properties
        let height = ((top_center.x - bottom_center.x).powi(2)
            + (top_center.y - bottom_center.y).powi(2)
            + (top_center.z - bottom_center.z).powi(2))
        .sqrt();

        let _axis = Point::new(
            top_center.x - bottom_center.x,
            top_center.y - bottom_center.y,
            top_center.z - bottom_center.z,
        );

        // Generate vertices
        let mut vertices = Vec::new();

        // Generate bottom and top circles
        for k in 0..=self.params.nz {
            let t = k as f64 / self.params.nz as f64;
            let z = bottom_center.z + t * height;

            for i in 0..=self.params.nx {
                let angle = 2.0 * std::f64::consts::PI * i as f64 / self.params.nx as f64;
                let x = bottom_center.x + radius * angle.cos();
                let y = bottom_center.y + radius * angle.sin();
                let vertex_id = mesh.add_vertex(Point::new(x, y, z));
                vertices.push(vertex_id);
            }
        }

        // Generate hexahedrons
        for k in 0..self.params.nz {
            for i in 0..self.params.nx {
                let v0 = k * (self.params.nx + 1) + i;
                let v1 = k * (self.params.nx + 1) + (i + 1) % (self.params.nx + 1);
                let v2 = (k + 1) * (self.params.nx + 1) + i;
                let v3 = (k + 1) * (self.params.nx + 1) + (i + 1) % (self.params.nx + 1);
                let v4 = v0 + (self.params.nx + 1);
                let v5 = v1 + (self.params.nx + 1);
                let v6 = v2 + (self.params.nx + 1);
                let v7 = v3 + (self.params.nx + 1);

                // Add hexahedron
                mesh.add_hexahedron(
                    vertices[v0],
                    vertices[v1],
                    vertices[v2],
                    vertices[v3],
                    vertices[v4],
                    vertices[v5],
                    vertices[v6],
                    vertices[v7],
                );
            }
        }

        // Optimize mesh if needed
        if self.params.quality_mesh {
            self.optimize_mesh(&mut mesh);
        }

        Ok(mesh)
    }

    /// Generate sphere mesh
    fn generate_sphere_mesh(&self, center: &Point, radius: f64) -> Result<Mesh3D, HexMesherError> {
        let mut mesh = Mesh3D::new();

        // Generate vertices using spherical coordinates
        let mut vertices = Vec::new();

        for k in 0..=self.params.nz {
            let theta = std::f64::consts::PI * k as f64 / self.params.nz as f64;
            let z = center.z + radius * theta.cos();
            let r = radius * theta.sin();

            for i in 0..=self.params.nx {
                let phi = 2.0 * std::f64::consts::PI * i as f64 / self.params.nx as f64;
                let x = center.x + r * phi.cos();
                let y = center.y + r * phi.sin();
                let vertex_id = mesh.add_vertex(Point::new(x, y, z));
                vertices.push(vertex_id);
            }
        }

        // Generate hexahedrons
        for k in 0..self.params.nz {
            for i in 0..self.params.nx {
                let v0 = k * (self.params.nx + 1) + i;
                let v1 = k * (self.params.nx + 1) + (i + 1) % (self.params.nx + 1);
                let v2 = (k + 1) * (self.params.nx + 1) + i;
                let v3 = (k + 1) * (self.params.nx + 1) + (i + 1) % (self.params.nx + 1);
                let v4 = v0 + (self.params.nx + 1);
                let v5 = v1 + (self.params.nx + 1);
                let v6 = v2 + (self.params.nx + 1);
                let v7 = v3 + (self.params.nx + 1);

                // Add hexahedron
                mesh.add_hexahedron(
                    vertices[v0],
                    vertices[v1],
                    vertices[v2],
                    vertices[v3],
                    vertices[v4],
                    vertices[v5],
                    vertices[v6],
                    vertices[v7],
                );
            }
        }

        // Optimize mesh if needed
        if self.params.quality_mesh {
            self.optimize_mesh(&mut mesh);
        }

        Ok(mesh)
    }

    /// Generate custom mesh
    fn generate_custom_mesh(
        &self,
        vertices: &[Point],
        hexahedrons: &[[usize; 8]],
    ) -> Result<Mesh3D, HexMesherError> {
        let mut mesh = Mesh3D::new();

        // Add vertices
        let mut vertex_map = Vec::new();
        for vertex in vertices {
            let vertex_id = mesh.add_vertex(vertex.clone());
            vertex_map.push(vertex_id);
        }

        // Add hexahedrons
        for hex in hexahedrons {
            mesh.add_hexahedron(
                vertex_map[hex[0]],
                vertex_map[hex[1]],
                vertex_map[hex[2]],
                vertex_map[hex[3]],
                vertex_map[hex[4]],
                vertex_map[hex[5]],
                vertex_map[hex[6]],
                vertex_map[hex[7]],
            );
        }

        // Optimize mesh if needed
        if self.params.quality_mesh {
            self.optimize_mesh(&mut mesh);
        }

        Ok(mesh)
    }

    /// Optimize hexahedral mesh
    fn optimize_mesh(&self, mesh: &mut Mesh3D) {
        // Simple optimization: adjust vertex positions to improve hex quality
        for i in 0..mesh.vertices.len() {
            self.optimize_vertex_position(mesh, i);
        }
    }

    /// Optimize vertex position
    fn optimize_vertex_position(&self, mesh: &mut Mesh3D, vertex_id: usize) {
        // Get adjacent hexahedrons
        let adjacent_hexes = self.get_adjacent_hexahedrons(mesh, vertex_id);

        if adjacent_hexes.is_empty() {
            return;
        }

        // Calculate average position from adjacent hex centers
        let mut avg_x = 0.0;
        let mut avg_y = 0.0;
        let mut avg_z = 0.0;

        for hex_id in &adjacent_hexes {
            let hex = &mesh.hexahedrons[*hex_id];
            let center = self.calculate_hex_center(mesh, hex);
            avg_x += center.x;
            avg_y += center.y;
            avg_z += center.z;
        }

        let count = adjacent_hexes.len() as f64;
        avg_x /= count;
        avg_y /= count;
        avg_z /= count;

        // Move vertex towards average position
        let current_pos = &mesh.vertices[vertex_id].point;
        let new_x = current_pos.x * 0.8 + avg_x * 0.2;
        let new_y = current_pos.y * 0.8 + avg_y * 0.2;
        let new_z = current_pos.z * 0.8 + avg_z * 0.2;

        mesh.vertices[vertex_id].point = Point::new(new_x, new_y, new_z);
    }

    /// Get adjacent hexahedrons for a vertex
    fn get_adjacent_hexahedrons(&self, mesh: &Mesh3D, vertex_id: usize) -> Vec<usize> {
        let mut adjacent_hexes = Vec::new();

        for (hex_id, hex) in mesh.hexahedrons.iter().enumerate() {
            if hex.vertices.contains(&vertex_id) {
                adjacent_hexes.push(hex_id);
            }
        }

        adjacent_hexes
    }

    /// Calculate hexahedron center
    fn calculate_hex_center(&self, mesh: &Mesh3D, hex: &MeshHexahedron) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;

        for &vertex_id in &hex.vertices {
            let vertex = &mesh.vertices[vertex_id];
            sum_x += vertex.point.x;
            sum_y += vertex.point.y;
            sum_z += vertex.point.z;
        }

        Point::new(sum_x / 8.0, sum_y / 8.0, sum_z / 8.0)
    }

    /// Calculate hexahedron quality
    fn calculate_hex_quality(&self, mesh: &Mesh3D, hex: &MeshHexahedron) -> f64 {
        // Calculate edge lengths
        let edges = vec![
            (hex.vertices[0], hex.vertices[1]),
            (hex.vertices[1], hex.vertices[2]),
            (hex.vertices[2], hex.vertices[3]),
            (hex.vertices[3], hex.vertices[0]),
            (hex.vertices[4], hex.vertices[5]),
            (hex.vertices[5], hex.vertices[6]),
            (hex.vertices[6], hex.vertices[7]),
            (hex.vertices[7], hex.vertices[4]),
            (hex.vertices[0], hex.vertices[4]),
            (hex.vertices[1], hex.vertices[5]),
            (hex.vertices[2], hex.vertices[6]),
            (hex.vertices[3], hex.vertices[7]),
        ];

        let mut edge_lengths = Vec::new();
        for (v0, v1) in edges {
            let p0 = &mesh.vertices[v0].point;
            let p1 = &mesh.vertices[v1].point;
            let length =
                ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2) + (p1.z - p0.z).powi(2)).sqrt();
            edge_lengths.push(length);
        }

        // Calculate aspect ratio
        let max_edge = edge_lengths.iter().fold(0.0_f64, |max, &e| max.max(e));
        let min_edge = edge_lengths.iter().fold(f64::MAX, |min, &e| min.min(e));
        let aspect_ratio = if min_edge > 0.0 {
            max_edge / min_edge
        } else {
            10.0
        };

        // Calculate quality score
        let aspect_score = if aspect_ratio < self.params.max_aspect_ratio {
            1.0
        } else {
            1.0 / (aspect_ratio / self.params.max_aspect_ratio)
        };

        aspect_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_hex_mesher_creation() {
        let params = HexMesherParams::default();
        let geometry = HexGeometry::Box {
            min: Point::new(0.0, 0.0, 0.0),
            max: Point::new(1.0, 1.0, 1.0),
        };
        let mesher = HexMesher::new(params, geometry);
        // Test passed if no panic
    }

    #[test]
    fn test_generate_box_mesh() {
        let mut mesher = HexMesher::new(
            HexMesherParams {
                nx: 2,
                ny: 2,
                nz: 2,
                ..Default::default()
            },
            HexGeometry::Box {
                min: Point::new(0.0, 0.0, 0.0),
                max: Point::new(1.0, 1.0, 1.0),
            },
        );

        let result = mesher.generate();
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.hexahedrons.is_empty());
    }

    #[test]
    fn test_generate_cylinder_mesh() {
        let mut mesher = HexMesher::new(
            HexMesherParams {
                nx: 8,
                ny: 1,
                nz: 2,
                ..Default::default()
            },
            HexGeometry::Cylinder {
                bottom_center: Point::new(0.0, 0.0, 0.0),
                top_center: Point::new(0.0, 0.0, 1.0),
                radius: 0.5,
            },
        );

        let result = mesher.generate();
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.hexahedrons.is_empty());
    }

    #[test]
    fn test_generate_sphere_mesh() {
        let mut mesher = HexMesher::new(
            HexMesherParams {
                nx: 8,
                ny: 1,
                nz: 4,
                ..Default::default()
            },
            HexGeometry::Sphere {
                center: Point::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
        );

        let result = mesher.generate();
        assert!(result.is_ok());

        let mesh = result.unwrap();
        assert!(!mesh.vertices.is_empty());
        assert!(!mesh.hexahedrons.is_empty());
    }
}
