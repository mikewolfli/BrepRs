//! Intersection detection module
//! 
//! This module provides advanced intersection detection algorithms for
//! curve-surface and surface-surface intersections.

use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{curve_enum::CurveEnum, Point, SurfaceEnum, Vector};
use std::collections::VecDeque;

/// Intersection result for curve-surface intersection
#[derive(Debug, Clone, PartialEq)]
pub struct CurveSurfaceIntersection {
    /// Parameter on the curve where intersection occurs
    pub curve_param: StandardReal,
    /// Parameters on the surface where intersection occurs
    pub surface_params: (StandardReal, StandardReal),
    /// Intersection point
    pub point: Point,
}

/// Intersection result for surface-surface intersection
#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceSurfaceIntersection {
    /// Parameters on the first surface where intersection occurs
    pub surface1_params: (StandardReal, StandardReal),
    /// Parameters on the second surface where intersection occurs
    pub surface2_params: (StandardReal, StandardReal),
    /// Intersection point
    pub point: Point,
}

/// Intersection solver
pub struct IntersectionSolver {
    /// Tolerance for intersection calculations
    tolerance: StandardReal,
    /// Maximum number of iterations for numerical methods
    max_iterations: usize,
}

impl IntersectionSolver {
    /// Create a new intersection solver with default parameters
    pub fn new() -> Self {
        Self {
            tolerance: STANDARD_REAL_EPSILON,
            max_iterations: 100,
        }
    }

    /// Create a new intersection solver with custom parameters
    pub fn with_parameters(tolerance: StandardReal, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }

    /// Find intersections between a curve and a surface
    pub fn curve_surface_intersection(
        &self,
        curve: &CurveEnum,
        surface: &SurfaceEnum,
    ) -> Vec<CurveSurfaceIntersection> {
        let mut intersections = Vec::new();
        
        // Get parameter ranges
        let (curve_min, curve_max) = curve.parameter_range();
        let ((u_min, u_max), (v_min, v_max)) = surface.parameter_range();
        
        // Use adaptive subdivision to find potential intersection regions
        let mut queue = VecDeque::new();
        queue.push_back((curve_min, curve_max, u_min, u_max, v_min, v_max));
        
        while let Some((t1, t2, u1, u2, v1, v2)) = queue.pop_front() {
            // Check if the bounding boxes intersect
            if !self.curve_surface_bbox_intersect(curve, surface, t1, t2, u1, u2, v1, v2) {
                continue;
            }
            
            // Check if the region is small enough for root finding
            let curve_range = t2 - t1;
            let u_range = u2 - u1;
            let v_range = v2 - v1;
            
            if curve_range < self.tolerance && u_range < self.tolerance && v_range < self.tolerance {
                // Try to find a precise intersection using Newton-Raphson
                if let Some(intersection) = self.newton_raphson_curve_surface(
                    curve, surface, (t1 + t2) / 2.0, (u1 + u2) / 2.0, (v1 + v2) / 2.0
                ) {
                    // Check if this intersection is already found
                    if !self.intersection_exists(&intersections, &intersection) {
                        intersections.push(intersection);
                    }
                }
            } else {
                // Subdivide the region
                let t_mid = (t1 + t2) / 2.0;
                let u_mid = (u1 + u2) / 2.0;
                let v_mid = (v1 + v2) / 2.0;
                
                queue.push_back((t1, t_mid, u1, u_mid, v1, v_mid));
                queue.push_back((t_mid, t2, u1, u_mid, v1, v_mid));
                queue.push_back((t1, t_mid, u_mid, u2, v1, v_mid));
                queue.push_back((t_mid, t2, u_mid, u2, v1, v_mid));
                queue.push_back((t1, t_mid, u1, u_mid, v_mid, v2));
                queue.push_back((t_mid, t2, u1, u_mid, v_mid, v2));
                queue.push_back((t1, t_mid, u_mid, u2, v_mid, v2));
                queue.push_back((t_mid, t2, u_mid, u2, v_mid, v2));
            }
        }
        
        intersections
    }

    /// Find intersections between two surfaces
    pub fn surface_surface_intersection(
        &self,
        surface1: &SurfaceEnum,
        surface2: &SurfaceEnum,
    ) -> Vec<SurfaceSurfaceIntersection> {
        let mut intersections = Vec::new();
        
        // Get parameter ranges
        let ((u1_min, u1_max), (v1_min, v1_max)) = surface1.parameter_range();
        let ((u2_min, u2_max), (v2_min, v2_max)) = surface2.parameter_range();
        
        // Use adaptive subdivision to find potential intersection regions
        let mut queue = VecDeque::new();
        queue.push_back((u1_min, u1_max, v1_min, v1_max, u2_min, u2_max, v2_min, v2_max));
        
        while let Some((u1_1, u1_2, v1_1, v1_2, u2_1, u2_2, v2_1, v2_2)) = queue.pop_front() {
            // Check if the bounding boxes intersect
            if !self.surface_surface_bbox_intersect(
                surface1, surface2, u1_1, u1_2, v1_1, v1_2, u2_1, u2_2, v2_1, v2_2
            ) {
                continue;
            }
            
            // Check if the region is small enough for root finding
            let u1_range = u1_2 - u1_1;
            let v1_range = v1_2 - v1_1;
            let u2_range = u2_2 - u2_1;
            let v2_range = v2_2 - v2_1;
            
            if u1_range < self.tolerance && v1_range < self.tolerance && 
               u2_range < self.tolerance && v2_range < self.tolerance {
                // Try to find a precise intersection using Newton-Raphson
                if let Some(intersection) = self.newton_raphson_surface_surface(
                    surface1, surface2, 
                    (u1_1 + u1_2) / 2.0, (v1_1 + v1_2) / 2.0,
                    (u2_1 + u2_2) / 2.0, (v2_1 + v2_2) / 2.0
                ) {
                    // Check if this intersection is already found
                    if !self.surface_intersection_exists(&intersections, &intersection) {
                        intersections.push(intersection);
                    }
                }
            } else {
                // Subdivide the region
                let u1_mid = (u1_1 + u1_2) / 2.0;
                let v1_mid = (v1_1 + v1_2) / 2.0;
                let u2_mid = (u2_1 + u2_2) / 2.0;
                let v2_mid = (v2_1 + v2_2) / 2.0;
                
                // Subdivide both surfaces
                queue.push_back((u1_1, u1_mid, v1_1, v1_mid, u2_1, u2_mid, v2_1, v2_mid));
                queue.push_back((u1_mid, u1_2, v1_1, v1_mid, u2_1, u2_mid, v2_1, v2_mid));
                queue.push_back((u1_1, u1_mid, v1_mid, v1_2, u2_1, u2_mid, v2_1, v2_mid));
                queue.push_back((u1_mid, u1_2, v1_mid, v1_2, u2_1, u2_mid, v2_1, v2_mid));
                queue.push_back((u1_1, u1_mid, v1_1, v1_mid, u2_mid, u2_2, v2_1, v2_mid));
                queue.push_back((u1_mid, u1_2, v1_1, v1_mid, u2_mid, u2_2, v2_1, v2_mid));
                queue.push_back((u1_1, u1_mid, v1_mid, v1_2, u2_mid, u2_2, v2_1, v2_mid));
                queue.push_back((u1_mid, u1_2, v1_mid, v1_2, u2_mid, u2_2, v2_1, v2_mid));
                queue.push_back((u1_1, u1_mid, v1_1, v1_mid, u2_1, u2_mid, v2_mid, v2_2));
                queue.push_back((u1_mid, u1_2, v1_1, v1_mid, u2_1, u2_mid, v2_mid, v2_2));
                queue.push_back((u1_1, u1_mid, v1_mid, v1_2, u2_1, u2_mid, v2_mid, v2_2));
                queue.push_back((u1_mid, u1_2, v1_mid, v1_2, u2_1, u2_mid, v2_mid, v2_2));
                queue.push_back((u1_1, u1_mid, v1_1, v1_mid, u2_mid, u2_2, v2_mid, v2_2));
                queue.push_back((u1_mid, u1_2, v1_1, v1_mid, u2_mid, u2_2, v2_mid, v2_2));
                queue.push_back((u1_1, u1_mid, v1_mid, v1_2, u2_mid, u2_2, v2_mid, v2_2));
                queue.push_back((u1_mid, u1_2, v1_mid, v1_2, u2_mid, u2_2, v2_mid, v2_2));
            }
        }
        
        intersections
    }

    /// Check if curve and surface bounding boxes intersect
    fn curve_surface_bbox_intersect(
        &self,
        curve: &CurveEnum,
        surface: &SurfaceEnum,
        t1: StandardReal,
        t2: StandardReal,
        u1: StandardReal,
        u2: StandardReal,
        v1: StandardReal,
        v2: StandardReal,
    ) -> bool {
        // Get curve bounding box
        let (curve_min, curve_max) = self.curve_bbox(curve, t1, t2);
        
        // Get surface bounding box
        let (surface_min, surface_max) = self.surface_bbox(surface, u1, u2, v1, v2);
        
        // Check bounding box intersection
        self.bboxes_intersect(&curve_min, &curve_max, &surface_min, &surface_max)
    }

    /// Check if two surface bounding boxes intersect
    fn surface_surface_bbox_intersect(
        &self,
        surface1: &SurfaceEnum,
        surface2: &SurfaceEnum,
        u1_1: StandardReal,
        u1_2: StandardReal,
        v1_1: StandardReal,
        v1_2: StandardReal,
        u2_1: StandardReal,
        u2_2: StandardReal,
        v2_1: StandardReal,
        v2_2: StandardReal,
    ) -> bool {
        // Get surface1 bounding box
        let (surface1_min, surface1_max) = self.surface_bbox(surface1, u1_1, u1_2, v1_1, v1_2);
        
        // Get surface2 bounding box
        let (surface2_min, surface2_max) = self.surface_bbox(surface2, u2_1, u2_2, v2_1, v2_2);
        
        // Check bounding box intersection
        self.bboxes_intersect(&surface1_min, &surface1_max, &surface2_min, &surface2_max)
    }

    /// Calculate bounding box for a curve segment
    fn curve_bbox(&self, curve: &CurveEnum, t1: StandardReal, t2: StandardReal) -> (Point, Point) {
        // Sample the curve to approximate the bounding box
        let samples = 10;
        let dt = (t2 - t1) / samples as StandardReal;
        
        let mut min_x = StandardReal::MAX;
        let mut min_y = StandardReal::MAX;
        let mut min_z = StandardReal::MAX;
        let mut max_x = StandardReal::MIN;
        let mut max_y = StandardReal::MIN;
        let mut max_z = StandardReal::MIN;
        
        for i in 0..=samples {
            let t = t1 + i as StandardReal * dt;
            let point = curve.value(t);
            
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }
        
        (Point::new(min_x, min_y, min_z), Point::new(max_x, max_y, max_z))
    }

    /// Calculate bounding box for a surface patch
    fn surface_bbox(
        &self,
        surface: &SurfaceEnum,
        u1: StandardReal,
        u2: StandardReal,
        v1: StandardReal,
        v2: StandardReal,
    ) -> (Point, Point) {
        // Sample the surface to approximate the bounding box
        let samples = 10;
        let du = (u2 - u1) / samples as StandardReal;
        let dv = (v2 - v1) / samples as StandardReal;
        
        let mut min_x = StandardReal::MAX;
        let mut min_y = StandardReal::MAX;
        let mut min_z = StandardReal::MAX;
        let mut max_x = StandardReal::MIN;
        let mut max_y = StandardReal::MIN;
        let mut max_z = StandardReal::MIN;
        
        for i in 0..=samples {
            for j in 0..=samples {
                let u = u1 + i as StandardReal * du;
                let v = v1 + j as StandardReal * dv;
                let point = surface.value(u, v);
                
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                min_z = min_z.min(point.z);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
                max_z = max_z.max(point.z);
            }
        }
        
        (Point::new(min_x, min_y, min_z), Point::new(max_x, max_y, max_z))
    }

    /// Check if two bounding boxes intersect
    fn bboxes_intersect(
        &self,
        min1: &Point,
        max1: &Point,
        min2: &Point,
        max2: &Point,
    ) -> bool {
        !(max1.x < min2.x || min1.x > max2.x ||
          max1.y < min2.y || min1.y > max2.y ||
          max1.z < min2.z || min1.z > max2.z)
    }

    /// Newton-Raphson method for curve-surface intersection
    fn newton_raphson_curve_surface(
        &self,
        curve: &CurveEnum,
        surface: &SurfaceEnum,
        initial_t: StandardReal,
        initial_u: StandardReal,
        initial_v: StandardReal,
    ) -> Option<CurveSurfaceIntersection> {
        let mut t = initial_t;
        let mut u = initial_u;
        let mut v = initial_v;
        
        for _ in 0..self.max_iterations {
            // Evaluate curve and surface at current parameters
            let curve_point = curve.value(t);
            let surface_point = surface.value(u, v);
            
            // Calculate distance vector
            let distance = surface_point - curve_point;
            let distance_magnitude = distance.magnitude();
            
            // Check if we've converged
            if distance_magnitude < self.tolerance {
                return Some(CurveSurfaceIntersection {
                    curve_param: t,
                    surface_params: (u, v),
                    point: curve_point,
                });
            }
            
            // Calculate derivatives
            let curve_deriv = curve.derivative(t, 1);
            let surface_du = self.surface_derivative_u(surface, u, v);
            let surface_dv = self.surface_derivative_v(surface, u, v);
            
            // Create Jacobian matrix
            let j11 = -curve_deriv.x;
            let j12 = surface_du.x;
            let j13 = surface_dv.x;
            let j21 = -curve_deriv.y;
            let j22 = surface_du.y;
            let j23 = surface_dv.y;
            let j31 = -curve_deriv.z;
            let j32 = surface_du.z;
            let j33 = surface_dv.z;
            
            // Calculate determinant
            let det = j11 * (j22 * j33 - j23 * j32) -
                      j12 * (j21 * j33 - j23 * j31) +
                      j13 * (j21 * j32 - j22 * j31);
            
            if det.abs() < self.tolerance {
                // Singular matrix, try a different approach
                break;
            }
            
            // Calculate inverse and solve for delta
            let delta_t = ((distance.x * (j22 * j33 - j23 * j32) -
                           distance.y * (j12 * j33 - j13 * j32) +
                           distance.z * (j12 * j23 - j13 * j22)) / det);
            
            let delta_u = ((-distance.x * (j21 * j33 - j23 * j31) +
                           distance.y * (j11 * j33 - j13 * j31) -
                           distance.z * (j11 * j23 - j13 * j21)) / det);
            
            let delta_v = ((distance.x * (j21 * j32 - j22 * j31) -
                           distance.y * (j11 * j32 - j12 * j31) +
                           distance.z * (j11 * j22 - j12 * j21)) / det);
            
            // Update parameters
            t += delta_t;
            u += delta_u;
            v += delta_v;
            
            // Check if parameters are within bounds
            let (curve_min, curve_max) = curve.parameter_range();
            let ((u_min, u_max), (v_min, v_max)) = surface.parameter_range();
            
            if t < curve_min || t > curve_max ||
               u < u_min || u > u_max ||
               v < v_min || v > v_max {
                break;
            }
        }
        
        None
    }

    /// Newton-Raphson method for surface-surface intersection
    fn newton_raphson_surface_surface(
        &self,
        surface1: &SurfaceEnum,
        surface2: &SurfaceEnum,
        initial_u1: StandardReal,
        initial_v1: StandardReal,
        initial_u2: StandardReal,
        initial_v2: StandardReal,
    ) -> Option<SurfaceSurfaceIntersection> {
        let mut u1 = initial_u1;
        let mut v1 = initial_v1;
        let mut u2 = initial_u2;
        let mut v2 = initial_v2;
        
        for _ in 0..self.max_iterations {
            // Evaluate both surfaces at current parameters
            let point1 = surface1.value(u1, v1);
            let point2 = surface2.value(u2, v2);
            
            // Calculate distance vector
            let distance = point2 - point1;
            let distance_magnitude = distance.magnitude();
            
            // Check if we've converged
            if distance_magnitude < self.tolerance {
                return Some(SurfaceSurfaceIntersection {
                    surface1_params: (u1, v1),
                    surface2_params: (u2, v2),
                    point: point1,
                });
            }
            
            // Calculate derivatives
            let du1 = self.surface_derivative_u(surface1, u1, v1);
            let dv1 = self.surface_derivative_v(surface1, u1, v1);
            let du2 = self.surface_derivative_u(surface2, u2, v2);
            let dv2 = self.surface_derivative_v(surface2, u2, v2);
            
            // Create Jacobian matrix
            let j11 = -du1.x;
            let j12 = -dv1.x;
            let j13 = du2.x;
            let j14 = dv2.x;
            let j21 = -du1.y;
            let j22 = -dv1.y;
            let j23 = du2.y;
            let j24 = dv2.y;
            let j31 = -du1.z;
            let j32 = -dv1.z;
            let j33 = du2.z;
            let j34 = dv2.z;
            
            // For simplicity, use a least-squares approach
            // This is a simplified version and may not work for all cases
            let mut delta_u1 = 0.0;
            let mut delta_v1 = 0.0;
            let mut delta_u2 = 0.0;
            let mut delta_v2 = 0.0;
            
            // Use gradient descent for simplicity
            let learning_rate = 0.1;
            delta_u1 = -learning_rate * (du1.x * distance.x + du1.y * distance.y + du1.z * distance.z);
            delta_v1 = -learning_rate * (dv1.x * distance.x + dv1.y * distance.y + dv1.z * distance.z);
            delta_u2 = learning_rate * (du2.x * distance.x + du2.y * distance.y + du2.z * distance.z);
            delta_v2 = learning_rate * (dv2.x * distance.x + dv2.y * distance.y + dv2.z * distance.z);
            
            // Update parameters
            u1 += delta_u1;
            v1 += delta_v1;
            u2 += delta_u2;
            v2 += delta_v2;
            
            // Check if parameters are within bounds
            let ((u1_min, u1_max), (v1_min, v1_max)) = surface1.parameter_range();
            let ((u2_min, u2_max), (v2_min, v2_max)) = surface2.parameter_range();
            
            if u1 < u1_min || u1 > u1_max ||
               v1 < v1_min || v1 > v1_max ||
               u2 < u2_min || u2 > u2_max ||
               v2 < v2_min || v2 > v2_max {
                break;
            }
        }
        
        None
    }

    /// Calculate surface derivative with respect to u
    fn surface_derivative_u(&self, surface: &SurfaceEnum, u: StandardReal, v: StandardReal) -> Vector {
        let h = 1e-6;
        let p1 = surface.value(u, v);
        let p2 = surface.value(u + h, v);
        Vector::new((p2.x - p1.x) / h, (p2.y - p1.y) / h, (p2.z - p1.z) / h)
    }

    /// Calculate surface derivative with respect to v
    fn surface_derivative_v(&self, surface: &SurfaceEnum, u: StandardReal, v: StandardReal) -> Vector {
        let h = 1e-6;
        let p1 = surface.value(u, v);
        let p2 = surface.value(u, v + h);
        Vector::new((p2.x - p1.x) / h, (p2.y - p1.y) / h, (p2.z - p1.z) / h)
    }

    /// Check if an intersection already exists in the list
    fn intersection_exists(&self, intersections: &[CurveSurfaceIntersection], new_intersection: &CurveSurfaceIntersection) -> bool {
        for intersection in intersections {
            if (intersection.point - new_intersection.point).magnitude() < self.tolerance {
                return true;
            }
        }
        false
    }

    /// Check if a surface intersection already exists in the list
    fn surface_intersection_exists(&self, intersections: &[SurfaceSurfaceIntersection], new_intersection: &SurfaceSurfaceIntersection) -> bool {
        for intersection in intersections {
            if (intersection.point - new_intersection.point).magnitude() < self.tolerance {
                return true;
            }
        }
        false
    }
}

impl Default for IntersectionSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{curve_enum::CurveEnum, line::Line, plane::Plane, Point};

    #[test]
    fn test_curve_surface_intersection() {
        // Create a line intersecting a plane
        let line = Line::new(Point::new(-1.0, -1.0, 0.0), Point::new(1.0, 1.0, 0.0));
        let curve = CurveEnum::Line(line);
        
        let plane = Plane::xy_plane();
        let surface = SurfaceEnum::Plane(plane);
        
        let solver = IntersectionSolver::new();
        let intersections = solver.curve_surface_intersection(&curve, &surface);
        
        assert!(!intersections.is_empty());
        let intersection = &intersections[0];
        assert!((intersection.point.x).abs() < 1e-6);
        assert!((intersection.point.y).abs() < 1e-6);
        assert!((intersection.point.z).abs() < 1e-6);
    }

    #[test]
    fn test_surface_surface_intersection() {
        // Create two planes that intersect
        let plane1 = Plane::xy_plane();
        let surface1 = SurfaceEnum::Plane(plane1);
        
        let plane2 = Plane::new(Point::origin(), crate::geometry::Direction::x_axis(), crate::geometry::Direction::z_axis());
        let surface2 = SurfaceEnum::Plane(plane2);
        
        let solver = IntersectionSolver::new();
        let intersections = solver.surface_surface_intersection(&surface1, &surface2);
        
        assert!(!intersections.is_empty());
    }
}
