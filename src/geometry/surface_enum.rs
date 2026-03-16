//! Surface enumeration type for dyn-compatible surface handling
//!
//! This module provides an enum-based approach to handle different surface types
//! without using trait objects, solving the dyn-compatibility issues with serde.

use crate::geometry::{Point, Vector};
use crate::topology::topods_face::Surface;
use serde::{Deserialize, Serialize};

/// Enumeration of all supported surface types
///
/// This enum provides a dyn-compatible way to work with surfaces by wrapping
/// concrete surface implementations. It avoids the limitations of trait objects
/// while maintaining flexibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceEnum {
    /// Plane surface
    Plane(super::plane::Plane),
    /// Cylindrical surface
    Cylinder(super::cylinder::Cylinder),
    /// Spherical surface
    Sphere(super::sphere::Sphere),
    /// Conical surface
    Cone(super::cone::Cone),
    /// Toroidal surface
    Torus(super::torus::Torus),
    /// Bezier surface
    BezierSurface(super::bezier_surface::BezierSurface),
    /// NURBS surface
    NurbsSurface(super::nurbs_surface::NurbsSurface),
}

impl SurfaceEnum {
    /// Get the point on the surface at (u, v) parameters
    pub fn value(&self, u: f64, v: f64) -> Point {
        match self {
            SurfaceEnum::Plane(s) => s.value(u, v),
            SurfaceEnum::Cylinder(s) => s.value(u, v),
            SurfaceEnum::Sphere(s) => s.value(u, v),
            SurfaceEnum::Cone(s) => s.value(u, v),
            SurfaceEnum::Torus(s) => s.value(u, v),
            SurfaceEnum::BezierSurface(s) => s.value(u, v),
            SurfaceEnum::NurbsSurface(s) => s.value(u, v),
        }
    }

    /// Get the normal at (u, v) parameters
    pub fn normal(&self, u: f64, v: f64) -> Vector {
        match self {
            SurfaceEnum::Plane(s) => s.normal().to_vec(),
            SurfaceEnum::Cylinder(s) => s.normal(u, v),
            SurfaceEnum::Sphere(s) => s.normal(u, v),
            SurfaceEnum::Cone(s) => s.normal(u, v),
            SurfaceEnum::Torus(s) => s.normal(u, v),
            SurfaceEnum::BezierSurface(s) => s.normal(u, v),
            SurfaceEnum::NurbsSurface(s) => s.normal(u, v),
        }
    }

    /// Get the parameter range of the surface
    pub fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            SurfaceEnum::Plane(s) => s.parameter_range(),
            SurfaceEnum::Cylinder(s) => s.parameter_range(),
            SurfaceEnum::Sphere(s) => s.parameter_range(),
            SurfaceEnum::Cone(s) => s.parameter_range(),
            SurfaceEnum::Torus(s) => s.parameter_range(),
            SurfaceEnum::BezierSurface(s) => s.parameter_range(),
            SurfaceEnum::NurbsSurface(s) => s.parameter_range(),
        }
    }

    /// Check if the surface is closed in the u direction
    pub fn is_u_closed(&self) -> bool {
        match self {
            SurfaceEnum::Plane(_) => false,
            SurfaceEnum::Cylinder(_) => true,
            SurfaceEnum::Sphere(_) => true,
            SurfaceEnum::Cone(_) => false,
            SurfaceEnum::Torus(_) => true,
            SurfaceEnum::BezierSurface(_) => false,
            SurfaceEnum::NurbsSurface(_) => false,
        }
    }

    /// Check if the surface is closed in the v direction
    pub fn is_v_closed(&self) -> bool {
        match self {
            SurfaceEnum::Plane(_) => false,
            SurfaceEnum::Cylinder(_) => false,
            SurfaceEnum::Sphere(_) => false,
            SurfaceEnum::Cone(_) => false,
            SurfaceEnum::Torus(_) => true,
            SurfaceEnum::BezierSurface(_) => false,
            SurfaceEnum::NurbsSurface(_) => false,
        }
    }

    /// Get the bounding box of the surface
    pub fn bounding_box(&self) -> (Point, Point) {
        // Sample points on the surface to approximate bounding box
        let samples_u = 50;
        let samples_v = 50;
        let ((u_min, u_max), (v_min, v_max)) = self.parameter_range();
        let du = (u_max - u_min) / samples_u as f64;
        let dv = (v_max - v_min) / samples_v as f64;

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for i in 0..=samples_u {
            for j in 0..=samples_v {
                let u = u_min + i as f64 * du;
                let v = v_min + j as f64 * dv;
                let p = self.value(u, v);
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                min_z = min_z.min(p.z);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
                max_z = max_z.max(p.z);
            }
        }

        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    /// Get the surface area (approximation)
    pub fn area(&self) -> f64 {
        // Use numerical integration to approximate surface area
        let samples = 100;
        let ((u_min, u_max), (v_min, v_max)) = self.parameter_range();
        let du = (u_max - u_min) / samples as f64;
        let dv = (v_max - v_min) / samples as f64;

        let mut area = 0.0;

        for i in 0..samples {
            for j in 0..samples {
                let u = u_min + (i as f64 + 0.5) * du;
                let v = v_min + (j as f64 + 0.5) * dv;

                // Calculate partial derivatives
                let h = 1e-6;
                let p = self.value(u, v);
                let pu = self.value(u + h, v);
                let pv = self.value(u, v + h);

                let du_vec = Vector::new(
                    (pu.x - p.x) / h,
                    (pu.y - p.y) / h,
                    (pu.z - p.z) / h,
                );
                let dv_vec = Vector::new(
                    (pv.x - p.x) / h,
                    (pv.y - p.y) / h,
                    (pv.z - p.z) / h,
                );

                // Cross product gives the area element
                let cross = du_vec.cross(&dv_vec);
                area += cross.magnitude() * du * dv;
            }
        }

        area
    }

    /// Check if a point is on the surface
    pub fn contains(&self, point: &Point, tolerance: f64) -> bool {
        // Find the closest point on the surface and check distance
        let ((u_min, u_max), (v_min, v_max)) = self.parameter_range();
        let samples = 100;
        let du = (u_max - u_min) / samples as f64;
        let dv = (v_max - v_min) / samples as f64;

        for i in 0..=samples {
            for j in 0..=samples {
                let u = u_min + i as f64 * du;
                let v = v_min + j as f64 * dv;
                let p = self.value(u, v);
                if (p - *point).magnitude() < tolerance {
                    return true;
                }
            }
        }

        false
    }

    /// Get the surface type name
    pub fn type_name(&self) -> &'static str {
        match self {
            SurfaceEnum::Plane(_) => "Plane",
            SurfaceEnum::Cylinder(_) => "Cylinder",
            SurfaceEnum::Sphere(_) => "Sphere",
            SurfaceEnum::Cone(_) => "Cone",
            SurfaceEnum::Torus(_) => "Torus",
            SurfaceEnum::BezierSurface(_) => "BezierSurface",
            SurfaceEnum::NurbsSurface(_) => "NurbsSurface",
        }
    }
}

// Implement Surface trait for SurfaceEnum to maintain backward compatibility
impl Surface for SurfaceEnum {
    fn value(&self, u: f64, v: f64) -> Point {
        match self {
            SurfaceEnum::Plane(s) => s.value(u, v),
            SurfaceEnum::Cylinder(s) => s.value(u, v),
            SurfaceEnum::Sphere(s) => s.value(u, v),
            SurfaceEnum::Cone(s) => s.value(u, v),
            SurfaceEnum::Torus(s) => s.value(u, v),
            SurfaceEnum::BezierSurface(s) => s.value(u, v),
            SurfaceEnum::NurbsSurface(s) => s.value(u, v),
        }
    }

    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector {
        match self {
            SurfaceEnum::Plane(s) => s.normal().to_vec(),
            SurfaceEnum::Cylinder(s) => s.normal(u, v),
            SurfaceEnum::Sphere(s) => s.normal(u, v),
            SurfaceEnum::Cone(s) => s.normal(u, v),
            SurfaceEnum::Torus(s) => s.normal(u, v),
            SurfaceEnum::BezierSurface(s) => s.normal(u, v),
            SurfaceEnum::NurbsSurface(s) => s.normal(u, v),
        }
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        match self {
            SurfaceEnum::Plane(s) => s.parameter_range(),
            SurfaceEnum::Cylinder(s) => s.parameter_range(),
            SurfaceEnum::Sphere(s) => s.parameter_range(),
            SurfaceEnum::Cone(s) => s.parameter_range(),
            SurfaceEnum::Torus(s) => s.parameter_range(),
            SurfaceEnum::BezierSurface(s) => s.parameter_range(),
            SurfaceEnum::NurbsSurface(s) => s.parameter_range(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surface_enum_plane() {
        let plane = super::super::plane::Plane::xy_plane();
        let surface = SurfaceEnum::Plane(plane);

        let origin = surface.value(0.0, 0.0);
        assert!((origin.x - 0.0).abs() < 1e-10);
        assert!((origin.y - 0.0).abs() < 1e-10);
        assert!((origin.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_surface_enum_sphere() {
        let sphere = super::super::sphere::Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            1.0,
        );
        let surface = SurfaceEnum::Sphere(sphere);

        assert!(surface.is_u_closed());
        assert!(!surface.is_v_closed());
    }
}