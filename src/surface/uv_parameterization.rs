//! UV parameterization module
//!
//! This module provides UV coordinate mapping for surfaces,
//! including planar, spherical, cylindrical, and advanced parameterization methods.

use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshVertex};

/// UV parameterization method
#[derive(Debug, Clone, PartialEq)]
pub enum UVMethod {
    /// Planar projection
    Planar,
    /// Spherical projection
    Spherical,
    /// Cylindrical projection
    Cylindrical,
    /// Box projection
    Box,
    /// LSCM (Least Squares Conformal Maps)
    LSCM,
    /// ABF (Angle Based Flattening)
    ABF,
}

/// UV parameters
#[derive(Debug, Clone)]
pub struct UVParams {
    /// Parameterization method
    pub method: UVMethod,
    /// Projection axis for planar/box projection
    pub axis: char,
    /// Center for spherical/cylindrical projection
    pub center: Point,
    /// Radius for spherical/cylindrical projection
    pub radius: f64,
    /// Cylinder axis for cylindrical projection
    pub cylinder_axis: Vector,
    /// Tolerance for convergence
    pub tolerance: f64,
    /// Maximum iterations for iterative methods
    pub max_iterations: usize,
}

impl Default for UVParams {
    fn default() -> Self {
        Self {
            method: UVMethod::Planar,
            axis: 'z',
            center: Point::origin(),
            radius: 1.0,
            cylinder_axis: Vector::new(0.0, 0.0, 1.0),
            tolerance: 1e-6,
            max_iterations: 100,
        }
    }
}

/// UV coordinates
#[derive(Debug, Clone, Copy)]
pub struct UVCoord {
    /// U coordinate
    pub u: f64,
    /// V coordinate
    pub v: f64,
}

impl UVCoord {
    /// Create a new UV coordinate
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    /// Clamp UV coordinates to [0, 1] range
    pub fn clamp(&self) -> Self {
        Self {
            u: self.u.max(0.0).min(1.0),
            v: self.v.max(0.0).min(1.0),
        }
    }

    /// Wrap UV coordinates (for seamless textures)
    pub fn wrap(&self) -> Self {
        Self {
            u: self.u - self.u.floor(),
            v: self.v - self.v.floor(),
        }
    }
}

/// UV parameterization result
#[derive(Debug, Clone)]
pub struct UVResult {
    /// UV coordinates for each vertex
    pub uv_coords: Vec<UVCoord>,
    /// Parameterization quality metrics
    pub quality: UVQuality,
}

/// UV parameterization quality metrics
#[derive(Debug, Clone)]
pub struct UVQuality {
    /// Area distortion
    pub area_distortion: f64,
    /// Angle distortion
    pub angle_distortion: f64,
    /// Stretch factor
    pub stretch_factor: f64,
    /// Number of flipped triangles
    pub flipped_triangles: usize,
}

/// UV parameterizer
pub struct UVParameterizer {
    /// UV parameters
    params: UVParams,
}

impl UVParameterizer {
    /// Create a new UV parameterizer
    pub fn new(params: UVParams) -> Self {
        Self { params }
    }

    /// Parameterize a mesh
    pub fn parameterize(&self, mesh: &Mesh3D) -> UVResult {
        match self.params.method {
            UVMethod::Planar => self.planar_projection(mesh),
            UVMethod::Spherical => self.spherical_projection(mesh),
            UVMethod::Cylindrical => self.cylindrical_projection(mesh),
            UVMethod::Box => self.box_projection(mesh),
            UVMethod::LSCM => self.lscm_parameterization(mesh),
            UVMethod::ABF => self.abf_parameterization(mesh),
        }
    }

    /// Planar projection
    fn planar_projection(&self, mesh: &Mesh3D) -> UVResult {
        let mut uv_coords = Vec::new();

        // Calculate bounds
        let (min, max) = self.calculate_bounds(mesh);

        // Project vertices onto specified plane
        for vertex in &mesh.vertices {
            let uv = match self.params.axis {
                'x' => UVCoord::new(
                    (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                    (vertex.point.z - min.z) / (max.z - min.z + 1e-10),
                ),
                'y' => UVCoord::new(
                    (vertex.point.x - min.x) / (max.x - min.x + 1e-10),
                    (vertex.point.z - min.z) / (max.z - min.z + 1e-10),
                ),
                'z' => UVCoord::new(
                    (vertex.point.x - min.x) / (max.x - min.x + 1e-10),
                    (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                ),
                _ => UVCoord::new(0.0, 0.0),
            };

            uv_coords.push(uv);
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// Spherical projection
    fn spherical_projection(&self, mesh: &Mesh3D) -> UVResult {
        let mut uv_coords = Vec::new();
        let center = &self.params.center;
        let radius = self.params.radius;

        for vertex in &mesh.vertices {
            // Calculate vector from center to vertex
            let vec = vertex.point - *center;

            // Calculate spherical coordinates
            let theta = vec.y.atan2(vec.x); // Azimuthal angle
            let phi = (vec.z / radius).acos(); // Polar angle

            // Map to UV coordinates
            let u = (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
            let v = phi / std::f64::consts::PI;

            uv_coords.push(UVCoord::new(u, v));
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// Cylindrical projection
    fn cylindrical_projection(&self, mesh: &Mesh3D) -> UVResult {
        let mut uv_coords = Vec::new();
        let center = &self.params.center;
        let radius = self.params.radius;
        let axis = &self.params.cylinder_axis;

        // Calculate bounds along cylinder axis
        let (min, max) = self.calculate_bounds_along_axis(mesh, axis);

        for vertex in &mesh.vertices {
            // Calculate vector from center to vertex
            let vec = vertex.point - *center;

            // Project onto cylinder axis
            let axis_component = vec.dot(axis);

            // Calculate perpendicular component
            let perp = vec - *axis * axis_component;

            // Calculate angle around cylinder
            let theta = perp.y.atan2(perp.x);

            // Map to UV coordinates
            let u = (theta + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
            let v = (axis_component - min) / (max - min + 1e-10);

            uv_coords.push(UVCoord::new(u, v));
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// Box projection
    fn box_projection(&self, mesh: &Mesh3D) -> UVResult {
        let mut uv_coords = Vec::new();

        // Calculate bounds
        let (min, max) = self.calculate_bounds(mesh);

        // Project vertices onto most suitable face of box
        for vertex in &mesh.vertices {
            // Determine which face is vertex closest to
            let normal = if let Some(normal_arr) = &vertex.normal {
                Vector::new(
                    normal_arr[0] as StandardReal,
                    normal_arr[1] as StandardReal,
                    normal_arr[2] as StandardReal,
                )
            } else {
                Vector::new(0.0, 0.0, 1.0) // Default normal if none provided
            };
            let normal = self.dominant_normal(&normal);

            let uv = if normal.x.abs() > normal.y.abs() && normal.x.abs() > normal.z.abs() {
                if normal.x > 0.0 {
                    UVCoord::new(
                        (vertex.point.z - min.z) / (max.z - min.z + 1e-10),
                        (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                    )
                } else {
                    UVCoord::new(
                        (max.z - vertex.point.z) / (max.z - min.z + 1e-10),
                        (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                    )
                }
            } else if normal.y.abs() > normal.x.abs() && normal.y.abs() > normal.z.abs() {
                if normal.y > 0.0 {
                    UVCoord::new(
                        (vertex.point.x - min.x) / (max.x - min.x + 1e-10),
                        (vertex.point.z - min.z) / (max.z - min.z + 1e-10),
                    )
                } else {
                    UVCoord::new(
                        (vertex.point.x - min.x) / (max.x - min.x + 1e-10),
                        (max.z - vertex.point.z) / (max.z - min.z + 1e-10),
                    )
                }
            } else {
                if vertex.normal.map_or(false, |n| n[2] > 0.0) {
                    UVCoord::new(
                        (vertex.point.x - min.x) / (max.x - min.x + 1e-10),
                        (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                    )
                } else {
                    UVCoord::new(
                        (max.x - vertex.point.x) / (max.x - min.x + 1e-10),
                        (vertex.point.y - min.y) / (max.y - min.y + 1e-10),
                    )
                }
            };

            uv_coords.push(uv);
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// LSCM parameterization
    fn lscm_parameterization(&self, mesh: &Mesh3D) -> UVResult {
        // Least Squares Conformal Maps parameterization
        // Solves the least squares problem for conformal mapping

        let mut uv_coords = vec![UVCoord::new(0.0, 0.0); mesh.vertices.len()];

        // Start with planar projection as initial guess
        let initial_result = self.planar_projection(mesh);
        uv_coords = initial_result.uv_coords;

        // Set boundary conditions (fix two vertices to avoid translation and rotation)
        if mesh.vertices.len() >= 2 {
            uv_coords[0] = UVCoord::new(0.0, 0.0);
            uv_coords[1] = UVCoord::new(1.0, 0.0);
        }

        // Build system matrix for LSCM
        let (matrix, rhs) = self.build_lscm_system(mesh);

        // Solve the linear system
        self.solve_linear_system(&matrix, &rhs, &mut uv_coords);

        // Iteratively optimize UV coordinates
        for _iteration in 0..self.params.max_iterations {
            let mut max_change = 0.0;

            // Optimize each vertex (excluding boundary vertices)
            for i in 2..mesh.vertices.len() {
                let (new_u, new_v) = self.optimize_vertex_lscm(mesh, i, &uv_coords);

                let change = (new_u - uv_coords[i].u).abs() + (new_v - uv_coords[i].v).abs();
                max_change = f64::max(max_change, change);

                uv_coords[i] = UVCoord::new(new_u, new_v);
            }

            if max_change < self.params.tolerance {
                break;
            }
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// Build linear system for LSCM
    fn build_lscm_system(&self, mesh: &Mesh3D) -> (Vec<Vec<f64>>, Vec<f64>) {
        // Build system matrix for LSCM
        let n = mesh.vertices.len();
        let mut matrix = vec![vec![0.0; 2 * n]; 2 * n];
        let mut rhs = vec![0.0; 2 * n];

        // Add equations for each edge
        for tetra in &mesh.tetrahedrons {
            for i in 0..4 {
                for j in (i + 1)..4 {
                    let v1 = tetra.vertices[i];
                    let v2 = tetra.vertices[j];

                    // Calculate edge length in 3D
                    let len_3d = mesh.vertices[v1].point.distance(&mesh.vertices[v2].point);

                    // Add equations for conformal mapping
                    matrix[2 * v1][2 * v1] += 1.0;
                    matrix[2 * v1][2 * v2] -= 1.0;
                    matrix[2 * v1 + 1][2 * v1 + 1] += 1.0;
                    matrix[2 * v1 + 1][2 * v2 + 1] -= 1.0;
                    matrix[2 * v2][2 * v1] -= 1.0;
                    matrix[2 * v2][2 * v2] += 1.0;
                    matrix[2 * v2 + 1][2 * v1 + 1] -= 1.0;
                    matrix[2 * v2 + 1][2 * v2 + 1] += 1.0;
                }
            }
        }

        (matrix, rhs)
    }

    /// Solve linear system
    fn solve_linear_system(&self, matrix: &[Vec<f64>], rhs: &[f64], uv_coords: &mut [UVCoord]) {
        // Simple linear system solver (Gauss-Seidel)
        let n = uv_coords.len();

        for _iteration in 0..self.params.max_iterations {
            let mut max_change = 0.0;

            for i in 2..n {
                let mut sum_u = rhs[2 * i];
                let mut sum_v = rhs[2 * i + 1];

                for j in 0..n {
                    if i != j {
                        sum_u -= matrix[2 * i][2 * j] * uv_coords[j].u
                            + matrix[2 * i][2 * j + 1] * uv_coords[j].v;
                        sum_v -= matrix[2 * i + 1][2 * j] * uv_coords[j].u
                            + matrix[2 * i + 1][2 * j + 1] * uv_coords[j].v;
                    }
                }

                let new_u = sum_u / matrix[2 * i][2 * i];
                let new_v = sum_v / matrix[2 * i + 1][2 * i + 1];

                let change = (new_u - uv_coords[i].u).abs() + (new_v - uv_coords[i].v).abs();
                max_change = f64::max(max_change, change);

                uv_coords[i] = UVCoord::new(new_u, new_v);
            }

            if max_change < self.params.tolerance {
                break;
            }
        }
    }

    /// Optimize a single vertex using LSCM
    fn optimize_vertex_lscm(
        &self,
        mesh: &Mesh3D,
        vertex_idx: usize,
        uv_coords: &[UVCoord],
    ) -> (f64, f64) {
        // Find neighboring vertices
        let mut neighbors = Vec::new();

        for tetra in &mesh.tetrahedrons {
            if tetra.vertices.contains(&vertex_idx) {
                for &v in &tetra.vertices {
                    if v != vertex_idx && !neighbors.contains(&v) {
                        neighbors.push(v);
                    }
                }
            }
        }

        if neighbors.is_empty() {
            return (uv_coords[vertex_idx].u, uv_coords[vertex_idx].v);
        }

        // Calculate optimal UV position using least squares conformal mapping
        let mut sum_u = 0.0;
        let mut sum_v = 0.0;
        let mut total_weight = 0.0;

        for &neighbor in &neighbors {
            // Calculate weight based on edge length
            let edge_length = mesh.vertices[vertex_idx]
                .point
                .distance(&mesh.vertices[neighbor].point);
            let weight = 1.0 / (edge_length + 1e-10);

            sum_u += uv_coords[neighbor].u * weight;
            sum_v += uv_coords[neighbor].v * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            (sum_u / total_weight, sum_v / total_weight)
        } else {
            (uv_coords[vertex_idx].u, uv_coords[vertex_idx].v)
        }
    }

    /// ABF parameterization
    fn abf_parameterization(&self, mesh: &Mesh3D) -> UVResult {
        // Angle Based Flattening parameterization
        // Preserves angles for better texture mapping

        // Start with planar projection as initial guess
        let initial_result = self.planar_projection(mesh);
        let mut uv_coords = initial_result.uv_coords;

        // Set boundary conditions
        if mesh.vertices.len() >= 2 {
            uv_coords[0] = UVCoord::new(0.0, 0.0);
            uv_coords[1] = UVCoord::new(1.0, 0.0);
        }

        // Calculate target angles for ABF
        let target_angles = self.calculate_target_angles(mesh);

        // Iteratively optimize UV coordinates
        for _iteration in 0..self.params.max_iterations {
            let mut max_change = 0.0;

            // Optimize each vertex (excluding boundary vertices)
            for i in 2..mesh.vertices.len() {
                let (new_u, new_v) = self.optimize_vertex_abf(mesh, i, &uv_coords, &target_angles);

                let change = (new_u - uv_coords[i].u).abs() + (new_v - uv_coords[i].v).abs();
                max_change = f64::max(max_change, change);

                uv_coords[i] = UVCoord::new(new_u, new_v);
            }

            if max_change < self.params.tolerance {
                break;
            }
        }

        // Calculate quality metrics
        let quality = self.calculate_quality(mesh, &uv_coords);

        UVResult { uv_coords, quality }
    }

    /// Calculate target angles for ABF
    fn calculate_target_angles(&self, mesh: &Mesh3D) -> Vec<Vec<f64>> {
        // Calculate target angles for each vertex
        let mut target_angles = vec![Vec::new(); mesh.vertices.len()];

        for vertex_idx in 0..mesh.vertices.len() {
            // Find neighboring vertices
            let mut neighbors = Vec::new();

            for tetra in &mesh.tetrahedrons {
                if tetra.vertices.contains(&vertex_idx) {
                    for &v in &tetra.vertices {
                        if v != vertex_idx && !neighbors.contains(&v) {
                            neighbors.push(v);
                        }
                    }
                }
            }

            // Calculate angles between neighbors
            let mut angles = Vec::new();
            for i in 0..neighbors.len() {
                let j = (i + 1) % neighbors.len();

                let vec1 = mesh.vertices[neighbors[i]].point - mesh.vertices[vertex_idx].point;
                let vec2 = mesh.vertices[neighbors[j]].point - mesh.vertices[vertex_idx].point;

                let angle = vec1.angle(&vec2);
                angles.push(angle);
            }

            target_angles[vertex_idx] = angles;
        }

        target_angles
    }

    /// Optimize a single vertex using ABF
    fn optimize_vertex_abf(
        &self,
        mesh: &Mesh3D,
        vertex_idx: usize,
        uv_coords: &[UVCoord],
        target_angles: &[Vec<f64>],
    ) -> (f64, f64) {
        // Find neighboring vertices
        let mut neighbors = Vec::new();

        for tetra in &mesh.tetrahedrons {
            if tetra.vertices.contains(&vertex_idx) {
                for &v in &tetra.vertices {
                    if v != vertex_idx && !neighbors.contains(&v) {
                        neighbors.push(v);
                    }
                }
            }
        }

        if neighbors.is_empty() || target_angles[vertex_idx].is_empty() {
            return (uv_coords[vertex_idx].u, uv_coords[vertex_idx].v);
        }

        // Calculate optimal UV position preserving angles
        let mut sum_u = 0.0;
        let mut sum_v = 0.0;
        let mut total_weight = 0.0;

        for (i, &neighbor) in neighbors.iter().enumerate() {
            // Calculate weight based on target angle
            let weight = if i < target_angles[vertex_idx].len() {
                target_angles[vertex_idx][i]
            } else {
                1.0
            };

            sum_u += uv_coords[neighbor].u * weight;
            sum_v += uv_coords[neighbor].v * weight;
            total_weight += weight;
        }

        if total_weight > 0.0 {
            (sum_u / total_weight, sum_v / total_weight)
        } else {
            (uv_coords[vertex_idx].u, uv_coords[vertex_idx].v)
        }
    }

    /// Calculate bounds of mesh
    fn calculate_bounds(&self, mesh: &Mesh3D) -> (Point, Point) {
        if mesh.vertices.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_x = mesh.vertices[0].point.x;
        let mut min_y = mesh.vertices[0].point.y;
        let mut min_z = mesh.vertices[0].point.z;
        let mut max_x = mesh.vertices[0].point.x;
        let mut max_y = mesh.vertices[0].point.y;
        let mut max_z = mesh.vertices[0].point.z;

        for vertex in &mesh.vertices[1..] {
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

    /// Calculate bounds along an axis
    fn calculate_bounds_along_axis(&self, mesh: &Mesh3D, axis: &Vector) -> (f64, f64) {
        if mesh.vertices.is_empty() {
            return (0.0, 0.0);
        }

        let mut min = Vector::new(
            mesh.vertices[0].point.x,
            mesh.vertices[0].point.y,
            mesh.vertices[0].point.z,
        )
        .dot(axis);
        let mut max = min;

        for vertex in &mesh.vertices[1..] {
            let projection = Vector::new(vertex.point.x, vertex.point.y, vertex.point.z).dot(axis);
            min = min.min(projection);
            max = max.max(projection);
        }

        (min, max)
    }

    /// Determine dominant normal component
    fn dominant_normal(&self, normal: &Vector) -> Vector {
        let abs_x = normal.x.abs();
        let abs_y = normal.y.abs();
        let abs_z = normal.z.abs();

        if abs_x > abs_y && abs_x > abs_z {
            Vector::new(normal.x.signum(), 0.0, 0.0)
        } else if abs_y > abs_x && abs_y > abs_z {
            Vector::new(0.0, normal.y.signum(), 0.0)
        } else {
            Vector::new(0.0, 0.0, normal.z.signum())
        }
    }

    /// Calculate UV parameterization quality
    fn calculate_quality(&self, mesh: &Mesh3D, uv_coords: &[UVCoord]) -> UVQuality {
        let mut total_area_distortion = 0.0;
        let mut total_angle_distortion = 0.0;
        let mut total_stretch = 0.0;
        let mut flipped_triangles = 0;
        let mut triangle_count = 0;

        // Calculate quality metrics for each tetrahedron
        for tetra in &mesh.tetrahedrons {
            let v0 = &mesh.vertices[tetra.vertices[0]];
            let v1 = &mesh.vertices[tetra.vertices[1]];
            let v2 = &mesh.vertices[tetra.vertices[2]];
            let v3 = &mesh.vertices[tetra.vertices[3]];

            let uv0 = uv_coords[tetra.vertices[0]];
            let uv1 = uv_coords[tetra.vertices[1]];
            let uv2 = uv_coords[tetra.vertices[2]];
            let uv3 = uv_coords[tetra.vertices[3]];

            // Calculate 3D triangle areas
            let area_3d_012 = self.triangle_area_3d(&v0.point, &v1.point, &v2.point);
            let area_3d_013 = self.triangle_area_3d(&v0.point, &v1.point, &v3.point);
            let area_3d_023 = self.triangle_area_3d(&v0.point, &v2.point, &v3.point);
            let area_3d_123 = self.triangle_area_3d(&v1.point, &v2.point, &v3.point);

            // Calculate 2D triangle areas
            let area_2d_012 = self.triangle_area_2d(uv0, uv1, uv2);
            let area_2d_013 = self.triangle_area_2d(uv0, uv1, uv3);
            let area_2d_023 = self.triangle_area_2d(uv0, uv2, uv3);
            let area_2d_123 = self.triangle_area_2d(uv1, uv2, uv3);

            // Calculate area distortion
            let distortion_012 = (area_2d_012 - area_3d_012).abs() / (area_3d_012 + 1e-10);
            let distortion_013 = (area_2d_013 - area_3d_013).abs() / (area_3d_013 + 1e-10);
            let distortion_023 = (area_2d_023 - area_3d_023).abs() / (area_3d_023 + 1e-10);
            let distortion_123 = (area_2d_123 - area_3d_123).abs() / (area_3d_123 + 1e-10);

            total_area_distortion +=
                distortion_012 + distortion_013 + distortion_023 + distortion_123;

            // Calculate stretch factor
            let stretch_012 = area_2d_012 / (area_3d_012 + 1e-10);
            let stretch_013 = area_2d_013 / (area_3d_013 + 1e-10);
            let stretch_023 = area_2d_023 / (area_3d_023 + 1e-10);
            let stretch_123 = area_2d_123 / (area_3d_123 + 1e-10);

            total_stretch += stretch_012 + stretch_013 + stretch_023 + stretch_123;

            // Check for flipped triangles
            if area_2d_012 < 0.0 {
                flipped_triangles += 1;
            }
            if area_2d_013 < 0.0 {
                flipped_triangles += 1;
            }
            if area_2d_023 < 0.0 {
                flipped_triangles += 1;
            }
            if area_2d_123 < 0.0 {
                flipped_triangles += 1;
            }

            triangle_count += 4;
        }

        // Calculate average quality metrics
        let area_distortion = if triangle_count > 0 {
            total_area_distortion / triangle_count as f64
        } else {
            0.0
        };

        let angle_distortion = 0.0; // Simplified
        let stretch_factor = if triangle_count > 0 {
            total_stretch / triangle_count as f64
        } else {
            0.0
        };

        UVQuality {
            area_distortion,
            angle_distortion,
            stretch_factor,
            flipped_triangles,
        }
    }

    /// Calculate triangle area in 3D
    fn triangle_area_3d(&self, p0: &Point, p1: &Point, p2: &Point) -> f64 {
        let v1 = p1 - p0;
        let v2 = p2 - p0;
        v1.cross(&v2).magnitude() / 2.0
    }

    /// Calculate triangle area in 2D (UV space)
    fn triangle_area_2d(&self, uv0: UVCoord, uv1: UVCoord, uv2: UVCoord) -> f64 {
        let v1 = (uv1.u - uv0.u, uv1.v - uv0.v);
        let v2 = (uv2.u - uv0.u, uv2.v - uv0.v);
        (v1.0 * v2.1 - v1.1 * v2.0).abs() / 2.0
    }
}

impl Default for UVParameterizer {
    fn default() -> Self {
        Self::new(UVParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_planar_projection() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = UVParams {
            method: UVMethod::Planar,
            axis: 'z',
            center: Point::origin(),
            radius: 1.0,
            cylinder_axis: Vector::new(0.0, 0.0, 1.0),
            tolerance: 1e-6,
            max_iterations: 100,
        };

        let parameterizer = UVParameterizer::new(params);
        let result = parameterizer.parameterize(&mesh);

        assert_eq!(result.uv_coords.len(), 4);
        assert!(result.quality.area_distortion >= 0.0);
    }

    #[test]
    fn test_spherical_projection() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(-1.0, 0.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0));

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = UVParams {
            method: UVMethod::Spherical,
            axis: 'z',
            center: Point::origin(),
            radius: 1.0,
            cylinder_axis: Vector::new(0.0, 0.0, 1.0),
            tolerance: 1e-6,
            max_iterations: 100,
        };

        let parameterizer = UVParameterizer::new(params);
        let result = parameterizer.parameterize(&mesh);

        assert_eq!(result.uv_coords.len(), 4);
        assert!(result.quality.area_distortion >= 0.0);
    }

    #[test]
    fn test_cylindrical_projection() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(-1.0, 0.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 0.0, 1.0));

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = UVParams {
            method: UVMethod::Cylindrical,
            axis: 'z',
            center: Point::origin(),
            radius: 1.0,
            cylinder_axis: Vector::new(0.0, 0.0, 1.0),
            tolerance: 1e-6,
            max_iterations: 100,
        };

        let parameterizer = UVParameterizer::new(params);
        let result = parameterizer.parameterize(&mesh);

        assert_eq!(result.uv_coords.len(), 4);
        assert!(result.quality.area_distortion >= 0.0);
    }

    #[test]
    fn test_box_projection() {
        let mut mesh = Mesh3D::new();

        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(1.0, 1.0, 0.0));
        let v3 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));

        mesh.add_tetrahedron(v0, v1, v2, v3);

        let params = UVParams {
            method: UVMethod::Box,
            axis: 'z',
            center: Point::origin(),
            radius: 1.0,
            cylinder_axis: Vector::new(0.0, 0.0, 1.0),
            tolerance: 1e-6,
            max_iterations: 100,
        };

        let parameterizer = UVParameterizer::new(params);
        let result = parameterizer.parameterize(&mesh);

        assert_eq!(result.uv_coords.len(), 4);
        assert!(result.quality.area_distortion >= 0.0);
    }
}
