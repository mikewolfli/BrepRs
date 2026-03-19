//! Implicit surfaces module
//!
//! This module provides surfaces defined by mathematical equations,
//! including various implicit surface types and mesh generation.

use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::Mesh3D;

/// Implicit surface type
pub enum ImplicitSurfaceType {
    /// Sphere: (x-cx)^2 + (y-cy)^2 + (z-cz)^2 = r^2
    Sphere { center: Point, radius: f64 },
    /// Ellipsoid: (x-cx)^2/a^2 + (y-cy)^2/b^2 + (z-cz)^2/c^2 = 1
    Ellipsoid {
        center: Point,
        radii: (f64, f64, f64),
    },
    /// Cylinder: (x-cx)^2 + (y-cy)^2 = r^2, bounded in z
    Cylinder {
        center: Point,
        radius: f64,
        height: f64,
        axis: Vector,
    },
    /// Cone: sqrt((x-cx)^2 + (y-cy)^2) = r * (z-cz)/h
    Cone {
        apex: Point,
        radius: f64,
        height: f64,
        axis: Vector,
    },
    /// Torus: (sqrt((x-cx)^2 + (y-cy)^2) - R)^2 + (z-cz)^2 = r^2
    Torus {
        center: Point,
        major_radius: f64,
        minor_radius: f64,
        axis: Vector,
    },
    /// Metaballs: Sum of radial basis functions
    Metaballs {
        centers: Vec<Point>,
        radii: Vec<f64>,
        threshold: f64,
    },
    /// Custom implicit surface
    Custom(Box<dyn Fn(&Point) -> f64 + Send + Sync>),
}

impl Clone for ImplicitSurfaceType {
    fn clone(&self) -> Self {
        match self {
            ImplicitSurfaceType::Sphere { center, radius } => ImplicitSurfaceType::Sphere {
                center: center.clone(),
                radius: *radius,
            },
            ImplicitSurfaceType::Ellipsoid { center, radii } => ImplicitSurfaceType::Ellipsoid {
                center: center.clone(),
                radii: *radii,
            },
            ImplicitSurfaceType::Cylinder {
                center,
                radius,
                height,
                axis,
            } => ImplicitSurfaceType::Cylinder {
                center: center.clone(),
                radius: *radius,
                height: *height,
                axis: axis.clone(),
            },
            ImplicitSurfaceType::Cone {
                apex,
                radius,
                height,
                axis,
            } => ImplicitSurfaceType::Cone {
                apex: apex.clone(),
                radius: *radius,
                height: *height,
                axis: axis.clone(),
            },
            ImplicitSurfaceType::Torus {
                center,
                major_radius,
                minor_radius,
                axis,
            } => ImplicitSurfaceType::Torus {
                center: center.clone(),
                major_radius: *major_radius,
                minor_radius: *minor_radius,
                axis: axis.clone(),
            },
            ImplicitSurfaceType::Metaballs {
                centers,
                radii,
                threshold,
            } => ImplicitSurfaceType::Metaballs {
                centers: centers.clone(),
                radii: radii.clone(),
                threshold: *threshold,
            },
            ImplicitSurfaceType::Custom(_) => {
                panic!("Cannot clone custom implicit surface function")
            }
        }
    }
}

impl std::fmt::Debug for ImplicitSurfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImplicitSurfaceType::Sphere { center, radius } => {
                write!(f, "Sphere {{ center: {:?}, radius: {} }}", center, radius)
            }
            ImplicitSurfaceType::Ellipsoid { center, radii } => {
                write!(
                    f,
                    "Ellipsoid {{ center: {:?}, radii: {:?} }}",
                    center, radii
                )
            }
            ImplicitSurfaceType::Cylinder {
                center,
                radius,
                height,
                axis,
            } => {
                write!(
                    f,
                    "Cylinder {{ center: {:?}, radius: {}, height: {}, axis: {:?} }}",
                    center, radius, height, axis
                )
            }
            ImplicitSurfaceType::Cone {
                apex,
                radius,
                height,
                axis,
            } => {
                write!(
                    f,
                    "Cone {{ apex: {:?}, radius: {}, height: {}, axis: {:?} }}",
                    apex, radius, height, axis
                )
            }
            ImplicitSurfaceType::Torus {
                center,
                major_radius,
                minor_radius,
                axis,
            } => {
                write!(
                    f,
                    "Torus {{ center: {:?}, major_radius: {}, minor_radius: {}, axis: {:?} }}",
                    center, major_radius, minor_radius, axis
                )
            }
            ImplicitSurfaceType::Metaballs {
                centers,
                radii,
                threshold,
            } => {
                write!(
                    f,
                    "Metaballs {{ centers: {:?}, radii: {:?}, threshold: {} }}",
                    centers, radii, threshold
                )
            }
            ImplicitSurfaceType::Custom(_) => {
                write!(f, "Custom(...)")
            }
        }
    }
}

/// Implicit surface parameters
#[derive(Debug, Clone)]
pub struct ImplicitSurfaceParams {
    /// Surface type
    pub surface_type: ImplicitSurfaceType,
    /// Marching cubes resolution
    pub resolution: usize,
    /// Bounding box
    pub bounds: (Point, Point),
    /// Adaptive subdivision
    pub adaptive: bool,
    /// Minimum cell size for adaptive subdivision
    pub min_cell_size: f64,
}

impl Default for ImplicitSurfaceParams {
    fn default() -> Self {
        Self {
            surface_type: ImplicitSurfaceType::Sphere {
                center: Point::origin(),
                radius: 1.0,
            },
            resolution: 50,
            bounds: (Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0)),
            adaptive: false,
            min_cell_size: 0.01,
        }
    }
}

/// Implicit surface
pub struct ImplicitSurface {
    /// Surface parameters
    params: ImplicitSurfaceParams,
}

impl ImplicitSurface {
    /// Create a new implicit surface
    pub fn new(params: ImplicitSurfaceParams) -> Self {
        Self { params }
    }

    /// Evaluate the implicit function at a point
    pub fn evaluate(&self, point: &Point) -> f64 {
        match &self.params.surface_type {
            ImplicitSurfaceType::Sphere { center, radius } => {
                let dist = point.distance(center);
                dist - radius
            }
            ImplicitSurfaceType::Ellipsoid { center, radii } => {
                let dx = (point.x - center.x) / radii.0;
                let dy = (point.y - center.y) / radii.1;
                let dz = (point.z - center.z) / radii.2;
                dx * dx + dy * dy + dz * dz - 1.0
            }
            ImplicitSurfaceType::Cylinder {
                center,
                radius,
                height,
                axis,
            } => {
                let vec = Vector::from_point(&point, &center);
                let mut axis_normalized = axis.clone();
                axis_normalized.normalize();
                let parallel = vec.dot(&axis_normalized);
                let perp = vec - axis_normalized * parallel;

                if parallel.abs() > height / 2.0 {
                    parallel.abs() - height / 2.0
                } else {
                    perp.magnitude() - radius
                }
            }
            ImplicitSurfaceType::Cone {
                apex,
                radius,
                height,
                axis,
            } => {
                let vec = Vector::from_point(&point, &apex);
                let mut axis_normalized = axis.clone();
                axis_normalized.normalize();
                let parallel = vec.dot(&axis_normalized);
                let perp = vec - axis_normalized * parallel;

                if parallel < 0.0 || parallel > *height {
                    if parallel < 0.0 {
                        -parallel
                    } else {
                        parallel - *height
                    }
                } else {
                    let expected_radius = radius * parallel / height;
                    perp.magnitude() - expected_radius
                }
            }
            ImplicitSurfaceType::Torus {
                center,
                major_radius,
                minor_radius,
                axis,
            } => {
                let vec = Vector::from_point(&point, &center);
                let mut axis_normalized = axis.clone();
                axis_normalized.normalize();
                let parallel = vec.dot(&axis_normalized);
                let perp = vec - axis_normalized * parallel;

                let dist_from_major = perp.magnitude() - major_radius;
                (dist_from_major * dist_from_major + parallel * parallel).sqrt() - minor_radius
            }
            ImplicitSurfaceType::Metaballs {
                centers,
                radii,
                threshold,
            } => {
                let mut sum = 0.0;
                for (center, radius) in centers.iter().zip(radii.iter()) {
                    let dist = point.distance(center);
                    if dist < *radius {
                        sum += (1.0 - dist / radius).powi(2);
                    }
                }
                sum - threshold
            }
            ImplicitSurfaceType::Custom(func) => func(point),
        }
    }

    /// Calculate gradient at a point
    pub fn gradient(&self, point: &Point) -> Vector {
        let epsilon = 1e-6;
        let f = self.evaluate(point);

        let fx = self.evaluate(&Point::new(point.x + epsilon, point.y, point.z));
        let fy = self.evaluate(&Point::new(point.x, point.y + epsilon, point.z));
        let fz = self.evaluate(&Point::new(point.x, point.y, point.z + epsilon));

        Vector::new((fx - f) / epsilon, (fy - f) / epsilon, (fz - f) / epsilon)
    }

    /// Generate mesh from implicit surface using marching cubes
    pub fn generate_mesh(&self) -> Mesh3D {
        if self.params.adaptive {
            self.adaptive_marching_cubes()
        } else {
            self.uniform_marching_cubes()
        }
    }

    /// Uniform marching cubes algorithm
    fn uniform_marching_cubes(&self) -> Mesh3D {
        let mut mesh = Mesh3D::new();
        let (min, max) = self.params.bounds;
        let resolution = self.params.resolution;

        let dx = (max.x - min.x) / resolution as f64;
        let dy = (max.y - min.y) / resolution as f64;
        let dz = (max.z - min.z) / resolution as f64;

        // Marching cubes lookup table
        let _edge_table = [
            0x0, 0x109, 0x203, 0x30a, 0x406, 0x50f, 0x605, 0x70c, 0x80c, 0x905, 0xa0f, 0xb06,
            0xc0a, 0xd03, 0xe09, 0xf00, 0x190, 0x99, 0x393, 0x29a, 0x596, 0x49f, 0x795, 0x69c,
            0x99c, 0x895, 0xb9f, 0xa96, 0xd9a, 0xc93, 0xf99, 0xe90, 0x230, 0x339, 0x33, 0x13a,
            0x636, 0x73f, 0x435, 0x53c, 0xa3c, 0xb35, 0x83f, 0x936, 0xe3a, 0xf33, 0xc39, 0xd30,
            0x3a0, 0x2a9, 0x1a3, 0xaa, 0x7a6, 0x6af, 0x5a5, 0x4ac, 0xbac, 0xaa5, 0x9af, 0x8a6,
            0xfaa, 0xea3, 0xda9, 0xca0, 0x460, 0x569, 0x663, 0x76a, 0x66, 0x16f, 0x265, 0x36c,
            0xc6c, 0xd65, 0xe6f, 0xf66, 0x86a, 0x963, 0xa69, 0xb60, 0x5f0, 0x4f9, 0x7f3, 0x6fa,
            0x1f6, 0xff, 0x3f5, 0x2fc, 0xdfc, 0xcf5, 0xfff, 0xef6, 0x9fa, 0x8f3, 0xbf9, 0xaf0,
            0x650, 0x759, 0x453, 0x55a, 0x256, 0x35f, 0x55, 0x15c, 0xe5c, 0xf55, 0xc5f, 0xd56,
            0xa5a, 0xb53, 0x859, 0x950, 0x7c0, 0x6c9, 0x5c3, 0x4ca, 0x3c6, 0x2cf, 0x1c5, 0xcc,
            0xfcc, 0xec5, 0xdcf, 0xcc6, 0xbca, 0xac3, 0x9c9, 0x8c0, 0x8c0, 0x9c9, 0xac3, 0xbca,
            0xcc6, 0xdcf, 0xec5, 0xfcc, 0xcc, 0x1c5, 0x2cf, 0x3c6, 0x4ca, 0x5c3, 0x6c9, 0x7c0,
            0x950, 0x859, 0xb53, 0xa5a, 0xd56, 0xc5f, 0xf55, 0xe5c, 0x15c, 0x55, 0x35f, 0x256,
            0x55a, 0x453, 0x759, 0x650, 0xaf0, 0xbf9, 0x8f3, 0x9fa, 0xef6, 0xfff, 0xcf5, 0xdfc,
            0x2fc, 0x3f5, 0xff, 0x1f6, 0x6fa, 0x7f3, 0x4f9, 0x5f0, 0xb60, 0xa69, 0x963, 0x86a,
            0xf66, 0xe6f, 0xd65, 0xc6c, 0x36c, 0x265, 0x16f, 0x66, 0x76a, 0x663, 0x569, 0x460,
            0xca0, 0xda9, 0xea3, 0xfaa, 0x8a6, 0x9af, 0xaa5, 0xbac, 0x4ac, 0x5a5, 0x6af, 0x7a6,
            0xaa, 0x1a3, 0x2a9, 0x3a0, 0xd30, 0xc39, 0xf33, 0xe3a, 0x936, 0x83f, 0xb35, 0xa3c,
            0x53c, 0x435, 0x73f, 0x636, 0x13a, 0x33, 0x339, 0x230, 0xe90, 0xf99, 0xc93, 0xd9a,
            0xa96, 0xb9f, 0x895, 0x99c, 0x69c, 0x795, 0x49f, 0x596, 0x29a, 0x393, 0x99, 0x190,
            0xf00, 0xe09, 0xd03, 0xc0a, 0xb06, 0xa0f, 0x905, 0x80c, 0x70c, 0x605, 0x50f, 0x406,
            0x30a, 0x203, 0x109, 0x0,
        ];

        let tri_table = vec![
            vec![-1],
            vec![0, 8, 3, -1],
            vec![0, 1, 9, -1],
            vec![1, 8, 3, 9, 8, 1, -1],
            vec![1, 2, 10, -1],
            vec![0, 8, 3, 1, 2, 10, -1],
            vec![9, 2, 10, 0, 2, 9, -1],
            vec![2, 8, 3, 2, 10, 8, 10, 9, 8, -1],
            vec![3, 11, 2, -1],
            vec![0, 11, 2, 8, 11, 0, -1],
            vec![1, 9, 0, 2, 3, 11, -1],
            vec![1, 11, 2, 1, 9, 11, 9, 8, 11, -1],
            vec![3, 10, 1, 11, 10, 3, -1],
            vec![0, 10, 1, 0, 8, 10, 8, 11, 10, -1],
            vec![3, 9, 0, 3, 11, 9, 11, 10, 9, -1],
            vec![9, 8, 10, 10, 8, 11, -1],
            vec![4, 7, 8, -1],
            vec![4, 3, 0, 7, 3, 4, -1],
            vec![0, 1, 9, 8, 4, 7, -1],
            vec![4, 1, 9, 4, 7, 1, 7, 3, 1, -1],
            vec![1, 2, 10, 8, 4, 7, -1],
            vec![3, 4, 7, 3, 0, 4, 1, 2, 10, -1],
            vec![9, 2, 10, 9, 0, 2, 8, 4, 7, -1],
            vec![2, 10, 9, 2, 9, 7, 2, 7, 3, 7, 9, 4, -1],
            vec![8, 4, 7, 3, 11, 2, -1],
            vec![11, 4, 7, 11, 2, 4, 2, 0, 4, -1],
            vec![9, 0, 1, 8, 4, 7, 2, 3, 11, -1],
            vec![4, 7, 11, 9, 4, 11, 9, 11, 2, 9, 2, 1, -1],
            vec![3, 10, 1, 3, 11, 10, 7, 8, 4, -1],
            vec![1, 11, 10, 1, 4, 11, 1, 0, 4, 7, 11, 4, -1],
            vec![4, 7, 8, 9, 0, 11, 9, 11, 10, 11, 0, 3, -1],
            vec![4, 7, 11, 4, 11, 9, 9, 11, 10, -1],
            vec![9, 5, 4, -1],
            vec![9, 5, 4, 0, 8, 3, -1],
            vec![0, 5, 4, 1, 5, 0, -1],
            vec![8, 5, 4, 8, 3, 5, 3, 1, 5, -1],
            vec![1, 2, 10, 9, 5, 4, -1],
            vec![3, 0, 8, 1, 2, 10, 4, 9, 5, -1],
            vec![5, 2, 10, 5, 4, 2, 4, 0, 2, -1],
            vec![2, 10, 5, 3, 2, 5, 3, 5, 4, 3, 4, 8, -1],
            vec![9, 5, 4, 2, 3, 11, -1],
            vec![0, 11, 2, 0, 8, 11, 4, 9, 5, -1],
            vec![0, 5, 4, 0, 1, 5, 2, 3, 11, -1],
            vec![2, 1, 5, 2, 5, 8, 2, 8, 11, 4, 8, 5, -1],
            vec![10, 3, 11, 10, 1, 3, 9, 5, 4, -1],
            vec![4, 9, 5, 0, 8, 1, 8, 10, 1, 8, 11, 10, -1],
            vec![5, 4, 0, 5, 0, 11, 5, 11, 10, 11, 0, 3, -1],
            vec![5, 4, 8, 5, 8, 10, 10, 8, 11, -1],
            vec![9, 7, 8, 5, 7, 9, -1],
            vec![9, 3, 0, 9, 5, 3, 5, 7, 3, -1],
            vec![0, 7, 8, 0, 1, 7, 1, 5, 7, -1],
            vec![1, 5, 3, 3, 5, 7, -1],
            vec![9, 7, 8, 9, 5, 7, 10, 1, 2, -1],
            vec![10, 1, 2, 9, 5, 0, 5, 3, 0, 5, 7, 3, -1],
            vec![8, 0, 2, 8, 2, 5, 8, 5, 7, 10, 5, 2, -1],
            vec![2, 10, 5, 2, 5, 3, 3, 5, 7, -1],
            vec![7, 9, 5, 7, 8, 9, 3, 11, 2, -1],
            vec![9, 5, 7, 9, 7, 2, 9, 2, 0, 2, 7, 11, -1],
            vec![2, 3, 11, 0, 1, 8, 1, 7, 8, 1, 5, 7, -1],
            vec![11, 2, 1, 11, 1, 7, 7, 1, 5, -1],
            vec![9, 5, 8, 8, 5, 7, 10, 1, 3, 10, 3, 11, -1],
            vec![5, 7, 0, 5, 0, 9, 7, 11, 0, 1, 0, 10, 11, 10, 0, -1],
            vec![11, 10, 0, 11, 0, 3, 10, 5, 0, 8, 0, 7, 5, 7, 0, -1],
            vec![11, 10, 5, 7, 11, 5, -1],
            vec![10, 6, 5, -1],
            vec![0, 8, 3, 5, 10, 6, -1],
            vec![9, 0, 1, 5, 10, 6, -1],
            vec![1, 8, 3, 1, 9, 8, 5, 10, 6, -1],
            vec![1, 6, 5, 2, 6, 1, -1],
            vec![1, 6, 5, 1, 2, 6, 3, 0, 8, -1],
            vec![9, 6, 5, 9, 0, 6, 0, 2, 6, -1],
            vec![5, 9, 8, 5, 8, 2, 5, 2, 6, 3, 2, 8, -1],
            vec![2, 3, 11, 10, 6, 5, -1],
            vec![11, 0, 8, 11, 2, 0, 10, 6, 5, -1],
            vec![0, 1, 9, 2, 3, 11, 5, 10, 6, -1],
            vec![5, 10, 6, 1, 9, 2, 9, 11, 2, 9, 8, 11, -1],
            vec![6, 3, 11, 6, 5, 3, 5, 1, 3, -1],
            vec![0, 8, 11, 0, 11, 5, 0, 5, 1, 5, 11, 6, -1],
            vec![3, 11, 6, 0, 3, 6, 0, 6, 5, 0, 5, 9, -1],
            vec![6, 5, 9, 6, 9, 11, 11, 9, 8, -1],
            vec![5, 10, 6, 4, 7, 8, -1],
            vec![4, 3, 0, 4, 7, 3, 6, 5, 10, -1],
            vec![1, 9, 0, 5, 10, 6, 8, 4, 7, -1],
            vec![10, 6, 5, 1, 9, 7, 1, 7, 3, 7, 9, 4, -1],
            vec![6, 1, 2, 6, 5, 1, 4, 7, 8, -1],
            vec![1, 2, 5, 5, 2, 6, 3, 0, 4, 3, 4, 7, -1],
            vec![8, 4, 7, 9, 0, 5, 0, 6, 5, 0, 2, 6, -1],
            vec![7, 3, 9, 7, 9, 4, 3, 2, 9, 5, 9, 6, 2, 6, 9, -1],
            vec![3, 11, 2, 7, 8, 4, 10, 6, 5, -1],
            vec![5, 10, 6, 4, 7, 2, 4, 2, 0, 2, 7, 11, -1],
            vec![0, 1, 9, 4, 7, 8, 2, 3, 11, 5, 10, 6, -1],
            vec![9, 2, 1, 9, 11, 2, 9, 4, 11, 7, 11, 4, 5, 10, 6, -1],
            vec![8, 4, 7, 3, 11, 5, 3, 5, 1, 5, 11, 6, -1],
            vec![5, 1, 11, 5, 11, 6, 1, 0, 11, 7, 11, 4, 0, 4, 11, -1],
            vec![0, 5, 9, 0, 6, 5, 0, 3, 6, 11, 6, 3, 8, 4, 7, -1],
            vec![6, 5, 9, 6, 9, 11, 4, 7, 9, 7, 11, 9, -1],
            vec![10, 4, 9, 6, 4, 10, -1],
            vec![4, 10, 6, 4, 9, 10, 0, 8, 3, -1],
            vec![10, 0, 1, 10, 6, 0, 6, 4, 0, -1],
            vec![8, 3, 1, 8, 1, 6, 8, 6, 4, 6, 1, 10, -1],
            vec![1, 4, 9, 1, 2, 4, 2, 6, 4, -1],
            vec![3, 0, 8, 1, 2, 9, 2, 4, 9, 2, 6, 4, -1],
            vec![0, 2, 4, 4, 2, 6, -1],
            vec![8, 3, 2, 8, 2, 4, 4, 2, 6, -1],
            vec![10, 4, 9, 10, 6, 4, 11, 2, 3, -1],
            vec![0, 8, 2, 2, 8, 11, 4, 9, 10, 4, 10, 6, -1],
            vec![3, 11, 2, 0, 1, 6, 0, 6, 4, 6, 1, 10, -1],
            vec![6, 4, 1, 6, 1, 10, 4, 8, 1, 2, 1, 11, 8, 11, 1, -1],
            vec![9, 6, 4, 9, 3, 6, 9, 1, 3, 11, 6, 3, -1],
            vec![8, 11, 1, 8, 1, 0, 11, 6, 1, 9, 1, 4, 6, 4, 1, -1],
            vec![3, 11, 6, 3, 6, 0, 0, 6, 4, -1],
            vec![6, 4, 8, 11, 6, 8, -1],
            vec![7, 10, 6, 7, 8, 10, 8, 9, 10, -1],
            vec![0, 7, 3, 0, 10, 7, 0, 9, 10, 6, 7, 10, -1],
            vec![10, 6, 7, 1, 10, 7, 1, 7, 8, 1, 8, 0, -1],
            vec![10, 6, 7, 10, 7, 1, 1, 7, 3, -1],
            vec![1, 2, 6, 1, 6, 8, 1, 8, 9, 8, 6, 7, -1],
            vec![2, 6, 9, 2, 9, 1, 6, 7, 9, 0, 9, 3, 7, 3, 9, -1],
            vec![7, 8, 0, 7, 0, 6, 6, 0, 2, -1],
            vec![7, 3, 2, 6, 7, 2, -1],
            vec![2, 3, 11, 10, 6, 8, 10, 8, 9, 8, 6, 7, -1],
            vec![2, 0, 7, 2, 7, 11, 0, 9, 7, 6, 7, 10, 9, 10, 7, -1],
            vec![1, 8, 0, 1, 7, 8, 1, 10, 7, 6, 7, 10, 2, 3, 11, -1],
            vec![11, 2, 1, 11, 1, 7, 10, 6, 1, 6, 7, 1, -1],
            vec![8, 9, 6, 8, 6, 7, 9, 1, 6, 11, 6, 3, 1, 3, 6, -1],
            vec![0, 9, 1, 11, 6, 7, -1],
            vec![7, 8, 0, 7, 0, 6, 3, 11, 0, 11, 6, 0, -1],
            vec![7, 11, 6, -1],
            vec![7, 6, 11, -1],
            vec![3, 0, 8, 11, 7, 6, -1],
            vec![0, 1, 9, 11, 7, 6, -1],
            vec![8, 1, 9, 8, 3, 1, 11, 7, 6, -1],
            vec![10, 1, 2, 6, 11, 7, -1],
            vec![1, 2, 10, 3, 0, 8, 6, 11, 7, -1],
            vec![2, 9, 0, 2, 10, 9, 6, 11, 7, -1],
            vec![6, 11, 7, 2, 10, 3, 10, 8, 3, 10, 9, 8, -1],
            vec![7, 2, 3, 6, 2, 7, -1],
            vec![7, 0, 8, 7, 6, 0, 6, 2, 0, -1],
            vec![2, 7, 6, 2, 3, 7, 0, 1, 9, -1],
            vec![1, 6, 2, 1, 8, 6, 1, 9, 8, 8, 7, 6, -1],
            vec![10, 7, 6, 10, 1, 7, 1, 3, 7, -1],
            vec![10, 7, 6, 1, 7, 10, 1, 8, 7, 1, 0, 8, -1],
            vec![0, 3, 7, 0, 7, 10, 0, 10, 9, 6, 10, 7, -1],
            vec![7, 6, 10, 7, 10, 8, 8, 10, 9, -1],
            vec![6, 8, 4, 11, 8, 6, -1],
            vec![3, 6, 11, 3, 0, 6, 0, 4, 6, -1],
            vec![8, 6, 11, 8, 4, 6, 9, 0, 1, -1],
            vec![9, 4, 6, 9, 6, 3, 9, 3, 1, 11, 3, 6, -1],
            vec![6, 8, 4, 6, 11, 8, 2, 10, 1, -1],
            vec![1, 2, 10, 3, 0, 11, 0, 6, 11, 0, 4, 6, -1],
            vec![4, 11, 8, 4, 6, 11, 0, 2, 9, 2, 10, 9, -1],
            vec![10, 9, 3, 10, 3, 2, 9, 4, 3, 11, 3, 6, 4, 6, 3, -1],
            vec![8, 2, 3, 8, 4, 2, 4, 6, 2, -1],
            vec![0, 4, 2, 4, 6, 2, -1],
            vec![1, 9, 0, 2, 3, 4, 2, 4, 6, 4, 3, 8, -1],
            vec![1, 9, 4, 1, 4, 2, 2, 4, 6, -1],
            vec![8, 1, 3, 8, 6, 1, 8, 4, 6, 6, 10, 1, -1],
            vec![10, 1, 0, 10, 0, 6, 6, 0, 4, -1],
            vec![4, 6, 3, 4, 3, 8, 6, 10, 3, 0, 3, 9, 10, 9, 3, -1],
            vec![10, 9, 4, 6, 10, 4, -1],
            vec![4, 9, 5, 7, 6, 11, -1],
            vec![0, 8, 3, 4, 9, 5, 11, 7, 6, -1],
            vec![5, 0, 1, 5, 4, 0, 7, 6, 11, -1],
            vec![11, 7, 6, 8, 3, 4, 3, 5, 4, 3, 1, 5, -1],
            vec![9, 5, 4, 10, 1, 2, 7, 6, 11, -1],
            vec![6, 11, 7, 1, 2, 10, 0, 8, 3, 4, 9, 5, -1],
            vec![7, 6, 11, 5, 4, 10, 4, 2, 10, 4, 0, 2, -1],
            vec![3, 4, 8, 3, 5, 4, 3, 2, 5, 10, 5, 2, 11, 7, 6, -1],
            vec![7, 2, 3, 7, 6, 2, 5, 4, 9, -1],
            vec![9, 5, 4, 0, 8, 6, 0, 6, 2, 6, 8, 7, -1],
            vec![3, 6, 2, 3, 7, 6, 1, 5, 0, 5, 4, 0, -1],
            vec![6, 2, 8, 6, 8, 7, 2, 1, 8, 4, 8, 5, 1, 5, 8, -1],
            vec![9, 5, 4, 10, 1, 6, 1, 7, 6, 1, 3, 7, -1],
            vec![1, 6, 10, 1, 7, 6, 1, 0, 7, 8, 7, 0, 9, 5, 4, -1],
            vec![4, 0, 10, 4, 10, 5, 0, 3, 10, 6, 10, 7, 3, 7, 10, -1],
            vec![7, 6, 10, 7, 10, 8, 5, 4, 10, 4, 8, 10, -1],
            vec![6, 9, 5, 6, 11, 9, 11, 8, 9, -1],
            vec![3, 6, 11, 0, 6, 3, 0, 5, 6, 0, 9, 5, -1],
            vec![0, 11, 8, 0, 5, 11, 0, 1, 5, 5, 6, 11, -1],
            vec![6, 11, 3, 6, 3, 5, 5, 3, 1, -1],
            vec![1, 2, 10, 9, 5, 11, 9, 11, 8, 11, 5, 6, -1],
            vec![0, 11, 3, 0, 6, 11, 0, 9, 6, 5, 6, 9, 1, 2, 10, -1],
            vec![11, 8, 5, 11, 5, 6, 8, 0, 5, 10, 5, 2, 0, 2, 5, -1],
            vec![6, 11, 3, 6, 3, 5, 2, 10, 3, 10, 5, 3, -1],
            vec![5, 8, 9, 5, 2, 8, 5, 6, 2, 3, 8, 2, -1],
            vec![9, 5, 6, 9, 6, 0, 0, 6, 2, -1],
            vec![1, 5, 8, 1, 8, 0, 5, 6, 8, 3, 8, 2, 6, 2, 8, -1],
            vec![1, 5, 6, 2, 1, 6, -1],
            vec![1, 3, 6, 1, 6, 10, 3, 8, 6, 5, 6, 9, 8, 9, 6, -1],
            vec![10, 1, 0, 10, 0, 6, 9, 5, 0, 5, 6, 0, -1],
            vec![0, 3, 8, 5, 6, 10, -1],
            vec![10, 5, 6, -1],
            vec![11, 5, 10, 7, 5, 11, -1],
            vec![11, 5, 10, 11, 7, 5, 8, 3, 0, -1],
            vec![5, 11, 7, 5, 10, 11, 1, 9, 0, -1],
            vec![10, 7, 5, 10, 11, 7, 9, 8, 1, 8, 3, 1, -1],
            vec![11, 1, 2, 11, 7, 1, 7, 5, 1, -1],
            vec![0, 8, 3, 1, 2, 7, 1, 7, 5, 7, 2, 11, -1],
            vec![9, 7, 5, 9, 2, 7, 9, 0, 2, 2, 11, 7, -1],
            vec![7, 5, 2, 7, 2, 11, 5, 9, 2, 3, 2, 8, 9, 8, 2, -1],
            vec![2, 5, 10, 2, 3, 5, 3, 7, 5, -1],
            vec![8, 2, 0, 8, 5, 2, 8, 7, 5, 10, 2, 5, -1],
            vec![9, 0, 1, 5, 10, 3, 5, 3, 7, 3, 10, 2, -1],
            vec![9, 8, 2, 9, 2, 1, 8, 7, 2, 10, 2, 5, 7, 5, 2, -1],
            vec![1, 3, 5, 3, 7, 5, -1],
            vec![0, 8, 7, 0, 7, 1, 1, 7, 5, -1],
            vec![9, 0, 3, 9, 3, 5, 5, 3, 7, -1],
            vec![9, 8, 7, 5, 9, 7, -1],
            vec![5, 8, 4, 5, 10, 8, 10, 11, 8, -1],
            vec![5, 0, 4, 5, 11, 0, 5, 10, 11, 11, 3, 0, -1],
            vec![0, 1, 9, 8, 4, 10, 8, 10, 11, 10, 4, 5, -1],
            vec![10, 11, 4, 10, 4, 5, 11, 3, 4, 9, 4, 1, 3, 1, 4, -1],
            vec![2, 5, 1, 2, 8, 5, 2, 11, 8, 4, 5, 8, -1],
            vec![0, 4, 11, 0, 11, 3, 4, 5, 11, 2, 11, 1, 5, 1, 11, -1],
            vec![0, 2, 5, 0, 5, 9, 2, 11, 5, 4, 5, 8, 11, 8, 5, -1],
            vec![9, 4, 5, 2, 11, 3, -1],
            vec![2, 5, 10, 3, 5, 2, 3, 4, 5, 3, 8, 4, -1],
            vec![5, 10, 2, 5, 2, 4, 4, 2, 0, -1],
            vec![3, 10, 2, 3, 5, 10, 3, 8, 5, 4, 5, 8, 0, 1, 9, -1],
            vec![5, 10, 2, 5, 2, 4, 1, 9, 2, 9, 4, 2, -1],
            vec![8, 4, 5, 8, 5, 3, 3, 5, 1, -1],
            vec![0, 4, 5, 1, 0, 5, -1],
            vec![8, 4, 5, 8, 5, 3, 9, 0, 5, 0, 3, 5, -1],
            vec![9, 4, 5, -1],
            vec![4, 11, 7, 4, 9, 11, 9, 10, 11, -1],
            vec![0, 8, 3, 4, 9, 7, 9, 11, 7, 9, 10, 11, -1],
            vec![1, 10, 11, 1, 11, 4, 1, 4, 0, 7, 4, 11, -1],
            vec![3, 1, 4, 3, 4, 8, 1, 10, 4, 7, 4, 11, 10, 11, 4, -1],
            vec![4, 11, 7, 9, 11, 4, 9, 2, 11, 9, 1, 2, -1],
            vec![9, 7, 4, 9, 11, 7, 9, 1, 11, 2, 11, 1, 0, 8, 3, -1],
            vec![11, 7, 4, 11, 4, 2, 2, 4, 0, -1],
            vec![11, 7, 4, 11, 4, 2, 8, 3, 4, 3, 2, 4, -1],
            vec![2, 9, 10, 2, 7, 9, 2, 3, 7, 7, 4, 9, -1],
            vec![9, 10, 7, 9, 7, 4, 10, 2, 7, 8, 7, 0, 2, 0, 7, -1],
            vec![3, 7, 10, 3, 10, 2, 7, 4, 10, 1, 10, 0, 4, 0, 10, -1],
            vec![1, 10, 2, 8, 7, 4, -1],
            vec![4, 9, 1, 4, 1, 7, 7, 1, 3, -1],
            vec![4, 9, 1, 4, 1, 7, 0, 8, 1, 8, 7, 1, -1],
            vec![4, 0, 3, 7, 4, 3, -1],
            vec![4, 8, 7, -1],
            vec![9, 10, 8, 10, 11, 8, -1],
            vec![3, 0, 9, 3, 9, 11, 11, 9, 10, -1],
            vec![0, 1, 10, 0, 10, 8, 8, 10, 11, -1],
            vec![3, 1, 10, 11, 3, 10, -1],
            vec![1, 2, 11, 1, 11, 9, 9, 11, 8, -1],
            vec![3, 0, 9, 3, 9, 11, 1, 2, 9, 2, 11, 9, -1],
            vec![0, 2, 11, 8, 0, 11, -1],
            vec![3, 2, 11, -1],
            vec![2, 3, 8, 2, 8, 10, 10, 8, 9, -1],
            vec![9, 10, 2, 0, 9, 2, -1],
            vec![2, 3, 8, 2, 8, 10, 0, 1, 8, 1, 10, 8, -1],
            vec![1, 10, 2, -1],
            vec![1, 3, 8, 9, 1, 8, -1],
            vec![0, 9, 1, -1],
            vec![0, 3, 8, -1],
            vec![-1],
        ];

        // Process each cell
        for i in 0..resolution {
            for j in 0..resolution {
                for k in 0..resolution {
                    let x = min.x + i as f64 * dx;
                    let y = min.y + j as f64 * dy;
                    let z = min.z + k as f64 * dz;

                    // Evaluate at cell corners
                    let corners = [
                        Point::new(x, y, z),
                        Point::new(x + dx, y, z),
                        Point::new(x + dx, y, z + dz),
                        Point::new(x, y, z + dz),
                        Point::new(x, y + dy, z),
                        Point::new(x + dx, y + dy, z),
                        Point::new(x + dx, y + dy, z + dz),
                        Point::new(x, y + dy, z + dz),
                    ];

                    let values: Vec<f64> = corners.iter().map(|p| self.evaluate(p)).collect();

                    // Calculate cube index
                    let mut cube_index = 0;
                    for (idx, &val) in values.iter().enumerate() {
                        if val < 0.0 {
                            cube_index |= 1 << idx;
                        }
                    }

                    if cube_index == 0 || cube_index == 255 {
                        continue;
                    }

                    // Get triangulation for this cube
                    let tri = &tri_table[cube_index];

                    // Interpolate vertices on edges
                    let mut vertices = Vec::new();
                    let mut idx = 0;
                    while idx < tri.len() && tri[idx] != -1 {
                        let edge = tri[idx] as usize;
                        let v0 = edge / 2;
                        let v1 = if edge % 2 == 0 { v0 + 1 } else { v0 + 2 };

                        let p0 = &corners[v0];
                        let p1 = &corners[v1];
                        let val0 = values[v0];
                        let val1 = values[v1];

                        let t = val0 / (val0 - val1);
                        let point = Point::new(
                            p0.x + t * (p1.x - p0.x),
                            p0.y + t * (p1.y - p0.y),
                            p0.z + t * (p1.z - p0.z),
                        );

                        let normal = self.gradient(&point).normalize();
                        vertices.push((point, normal));

                        idx += 1;
                    }

                    // Create triangles
                    for tri_idx in (0..vertices.len()).step_by(3) {
                        if tri_idx + 2 < vertices.len() {
                            let v0 = mesh.add_vertex(vertices[tri_idx].0);
                            let v1 = mesh.add_vertex(vertices[tri_idx + 1].0);
                            let v2 = mesh.add_vertex(vertices[tri_idx + 2].0);

                            mesh.add_face(vec![v0, v1, v2]);
                        }
                    }
                }
            }
        }

        mesh
    }

    /// Adaptive marching cubes algorithm
    fn adaptive_marching_cubes(&self) -> Mesh3D {
        // Simplified adaptive implementation
        // In a real implementation, you would recursively subdivide cells
        // based on the gradient magnitude or other criteria
        self.uniform_marching_cubes()
    }
}

impl Default for ImplicitSurface {
    fn default() -> Self {
        Self::new(ImplicitSurfaceParams::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere() {
        let params = ImplicitSurfaceParams {
            surface_type: ImplicitSurfaceType::Sphere {
                center: Point::origin(),
                radius: 1.0,
            },
            resolution: 20,
            bounds: (Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0)),
            adaptive: false,
            min_cell_size: 0.01,
        };

        let surface = ImplicitSurface::new(params);

        let origin = Point::origin();
        assert_eq!(surface.evaluate(&origin), -1.0);

        let on_surface = Point::new(1.0, 0.0, 0.0);
        assert_eq!(surface.evaluate(&on_surface), 0.0);

        let outside = Point::new(2.0, 0.0, 0.0);
        assert_eq!(surface.evaluate(&outside), 1.0);
    }

    #[test]
    fn test_ellipsoid() {
        let params = ImplicitSurfaceParams {
            surface_type: ImplicitSurfaceType::Ellipsoid {
                center: Point::origin(),
                radii: (2.0, 1.0, 0.5),
            },
            resolution: 20,
            bounds: (Point::new(-2.0, -1.0, -0.5), Point::new(2.0, 1.0, 0.5)),
            adaptive: false,
            min_cell_size: 0.01,
        };

        let surface = ImplicitSurface::new(params);

        let origin = Point::origin();
        assert_eq!(surface.evaluate(&origin), -1.0);
    }

    #[test]
    fn test_metaballs() {
        let params = ImplicitSurfaceParams {
            surface_type: ImplicitSurfaceType::Metaballs {
                centers: vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)],
                radii: vec![1.0, 1.0],
                threshold: 0.5,
            },
            resolution: 20,
            bounds: (Point::new(-1.0, -1.0, -1.0), Point::new(2.0, 1.0, 1.0)),
            adaptive: false,
            min_cell_size: 0.01,
        };

        let surface = ImplicitSurface::new(params);

        let origin = Point::origin();
        let value = surface.evaluate(&origin);
        assert!(value < 0.0);
    }

    #[test]
    fn test_gradient() {
        let params = ImplicitSurfaceParams {
            surface_type: ImplicitSurfaceType::Sphere {
                center: Point::origin(),
                radius: 1.0,
            },
            resolution: 20,
            bounds: (Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0)),
            adaptive: false,
            min_cell_size: 0.01,
        };

        let surface = ImplicitSurface::new(params);

        let point = Point::new(1.0, 0.0, 0.0);
        let gradient = surface.gradient(&point);

        assert!((gradient.x - 1.0).abs() < 0.1);
        assert!(gradient.y.abs() < 0.1);
        assert!(gradient.z.abs() < 0.1);
    }

    #[test]
    fn test_generate_mesh() {
        let params = ImplicitSurfaceParams {
            surface_type: ImplicitSurfaceType::Sphere {
                center: Point::origin(),
                radius: 1.0,
            },
            resolution: 10,
            bounds: (Point::new(-1.0, -1.0, -1.0), Point::new(1.0, 1.0, 1.0)),
            adaptive: false,
            min_cell_size: 0.01,
        };

        let surface = ImplicitSurface::new(params);
        let mesh = surface.generate_mesh();

        assert!(mesh.vertices.len() > 0);
    }
}
