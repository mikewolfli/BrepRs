//! Distance calculation module
//! 
//! This module provides distance calculation functions for various geometric objects,
//! including point to surface and surface to surface distance calculations.

use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, SurfaceEnum, Vector};
use crate::topology::topods_face::TopoDsFace;
use std::collections::VecDeque;

/// Distance result
#[derive(Debug, Clone, PartialEq)]
pub struct DistanceResult {
    /// Minimum distance
    pub distance: StandardReal,
    /// Closest point on the first object
    pub closest_point1: Point,
    /// Closest point on the second object (if applicable)
    pub closest_point2: Option<Point>,
}

/// Distance calculator
pub struct DistanceCalculator {
    /// Tolerance for distance calculations
    tolerance: StandardReal,
    /// Maximum number of iterations for numerical methods
    max_iterations: usize,
}

impl DistanceCalculator {
    /// Create a new distance calculator with default parameters
    pub fn new() -> Self {
        Self {
            tolerance: STANDARD_REAL_EPSILON,
            max_iterations: 100,
        }
    }

    /// Create a new distance calculator with custom parameters
    pub fn with_parameters(tolerance: StandardReal, max_iterations: usize) -> Self {
        Self {
            tolerance,
            max_iterations,
        }
    }

    /// Calculate distance from a point to a surface
    pub fn point_to_surface(
        &self,
        point: &Point,
        surface: &SurfaceEnum,
    ) -> DistanceResult {
        // Get surface parameter range
        let ((u_min, u_max), (v_min, v_max)) = surface.parameter_range();
        
        // Use adaptive subdivision to find potential closest regions
        let mut queue = VecDeque::new();
        queue.push_back((u_min, u_max, v_min, v_max));
        
        let mut min_distance = StandardReal::MAX;
        let mut closest_point = Point::origin();
        
        while let Some((u1, u2, v1, v2)) = queue.pop_front() {
            // Calculate center of the parameter region
            let u_center = (u1 + u2) / 2.0;
            let v_center = (v1 + v2) / 2.0;
            let center_point = surface.value(u_center, v_center);
            let center_distance = point.distance(&center_point);
            
            // Update minimum distance if needed
            if center_distance < min_distance {
                min_distance = center_distance;
                closest_point = center_point;
            }
            
            // Check if the region is small enough
            let u_range = u2 - u1;
            let v_range = v2 - v1;
            
            if u_range < self.tolerance && v_range < self.tolerance {
                // Try to refine using Newton-Raphson
                if let Some((refined_point, refined_distance)) = self.newton_raphson_point_surface(
                    point, surface, u_center, v_center
                ) {
                    if refined_distance < min_distance {
                        min_distance = refined_distance;
                        closest_point = refined_point;
                    }
                }
            } else {
                // Subdivide the region
                let u_mid = (u1 + u2) / 2.0;
                let v_mid = (v1 + v2) / 2.0;
                
                queue.push_back((u1, u_mid, v1, v_mid));
                queue.push_back((u_mid, u2, v1, v_mid));
                queue.push_back((u1, u_mid, v_mid, v2));
                queue.push_back((u_mid, u2, v_mid, v2));
            }
        }
        
        DistanceResult {
            distance: min_distance,
            closest_point1: point.clone(),
            closest_point2: Some(closest_point),
        }
    }

    /// Calculate distance between two surfaces
    pub fn surface_to_surface(
        &self,
        surface1: &SurfaceEnum,
        surface2: &SurfaceEnum,
    ) -> DistanceResult {
        // Get parameter ranges
        let ((u1_min, u1_max), (v1_min, v1_max)) = surface1.parameter_range();
        let ((u2_min, u2_max), (v2_min, v2_max)) = surface2.parameter_range();
        
        // Use adaptive subdivision to find potential closest regions
        let mut queue = VecDeque::new();
        queue.push_back((u1_min, u1_max, v1_min, v1_max, u2_min, u2_max, v2_min, v2_max));
        
        let mut min_distance = StandardReal::MAX;
        let mut closest_point1 = Point::origin();
        let mut closest_point2 = Point::origin();
        
        while let Some((u1_1, u1_2, v1_1, v1_2, u2_1, u2_2, v2_1, v2_2)) = queue.pop_front() {
            // Calculate centers of the parameter regions
            let u1_center = (u1_1 + u1_2) / 2.0;
            let v1_center = (v1_1 + v1_2) / 2.0;
            let u2_center = (u2_1 + u2_2) / 2.0;
            let v2_center = (v2_1 + v2_2) / 2.0;
            
            let point1 = surface1.value(u1_center, v1_center);
            let point2 = surface2.value(u2_center, v2_center);
            let center_distance = point1.distance(&point2);
            
            // Update minimum distance if needed
            if center_distance < min_distance {
                min_distance = center_distance;
                closest_point1 = point1;
                closest_point2 = point2;
            }
            
            // Check if the regions are small enough
            let u1_range = u1_2 - u1_1;
            let v1_range = v1_2 - v1_1;
            let u2_range = u2_2 - u2_1;
            let v2_range = v2_2 - v2_1;
            
            if u1_range < self.tolerance && v1_range < self.tolerance && 
               u2_range < self.tolerance && v2_range < self.tolerance {
                // Try to refine using Newton-Raphson
                if let Some((p1, p2, refined_distance)) = self.newton_raphson_surface_surface(
                    surface1, surface2, 
                    u1_center, v1_center, u2_center, v2_center
                ) {
                    if refined_distance < min_distance {
                        min_distance = refined_distance;
                        closest_point1 = p1;
                        closest_point2 = p2;
                    }
                }
            } else {
                // Subdivide the regions
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
        
        DistanceResult {
            distance: min_distance,
            closest_point1,
            closest_point2: Some(closest_point2),
        }
    }

    /// Calculate distance from a point to a face
    pub fn point_to_face(&self, point: &Point, face: &TopoDsFace) -> DistanceResult {
        // Get the surface of the face
        if let Some(surface) = face.surface() {
            // Calculate distance to the surface
            let surface_distance = self.point_to_surface(point, surface.as_ref().unwrap());
            
            // Check if the closest point is inside the face
            if self.is_point_inside_face(&surface_distance.closest_point2.unwrap(), face) {
                return surface_distance;
            }
            
            // If not, find the closest point on the face boundary
            self.point_to_face_boundary(point, face)
        } else {
            // If no surface, return max distance
            DistanceResult {
                distance: StandardReal::MAX,
                closest_point1: point.clone(),
                closest_point2: None,
            }
        }
    }

    /// Calculate distance from a point to a face boundary
    fn point_to_face_boundary(&self, point: &Point, face: &TopoDsFace) -> DistanceResult {
        let wires = face.wires();
        let mut min_distance = StandardReal::MAX;
        let mut closest_point = Point::origin();
        
        for wire in wires {
            if let Some(wire_ref) = wire.as_ref() {
                let edges = wire_ref.edges();
                
                for edge in &edges {
                    if let Some(edge_ref) = edge.as_ref() {
                        let edge_distance = self.point_to_edge(point, edge_ref);
                        if edge_distance.distance < min_distance {
                            min_distance = edge_distance.distance;
                            closest_point = edge_distance.closest_point2.unwrap();
                        }
                    }
                }
            }
        }
        
        DistanceResult {
            distance: min_distance,
            closest_point1: point.clone(),
            closest_point2: Some(closest_point),
        }
    }

    /// Calculate distance from a point to an edge
    fn point_to_edge(
        &self,
        point: &Point,
        edge: &crate::topology::topods_edge::TopoDsEdge,
    ) -> DistanceResult {
        // Get the vertices of the edge
        let v1 = edge.start_vertex();
        let v2 = edge.end_vertex();
        
        if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
            let p1 = v1_ref.point();
            let p2 = v2_ref.point();
            
            // Calculate line segment distance
            let (closest, distance) = self.point_to_line_segment(point, &p1, &p2);
            
            DistanceResult {
                distance,
                closest_point1: point.clone(),
                closest_point2: Some(closest),
            }
        } else {
            // If no vertices, return max distance
            DistanceResult {
                distance: StandardReal::MAX,
                closest_point1: point.clone(),
                closest_point2: None,
            }
        }
    }

    /// Calculate distance from a point to a line segment
    fn point_to_line_segment(&self, point: &Point, p1: &Point, p2: &Point) -> (Point, StandardReal) {
        let v = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let w = Vector::new(point.x - p1.x, point.y - p1.y, point.z - p1.z);
        
        let c1 = w.dot(&v);
        let c2 = v.dot(&v);
        
        if c1 <= 0.0 {
            // Closest point is p1
            (p1.clone(), point.distance(p1))
        } else if c2 <= c1 {
            // Closest point is p2
            (p2.clone(), point.distance(p2))
        } else {
            // Closest point is along the segment
            let t = c1 / c2;
            let closest = Point::new(
                p1.x + t * v.x,
                p1.y + t * v.y,
                p1.z + t * v.z
            );
            (closest, point.distance(&closest))
        }
    }

    /// Check if a point is inside a face
    fn is_point_inside_face(&self, point: &Point, face: &TopoDsFace) -> bool {
        let wires = face.wires();
        
        for wire in wires {
            if let Some(wire_ref) = wire.as_ref() {
                if !self.is_point_inside_wire(point, wire_ref) {
                    return false;
                }
            }
        }
        
        true
    }

    /// Check if a point is inside a wire
    fn is_point_inside_wire(&self, point: &Point, wire: &crate::topology::topods_wire::TopoDsWire) -> bool {
        // Use ray casting algorithm
        let edges = wire.edges();
        let mut crossings = 0;
        
        // Create a ray from the point to infinity
        let ray_end = Point::new(point.x + 1000.0, point.y, point.z);
        
        for edge in &edges {
            if let Some(edge_ref) = edge.as_ref() {
                let v1 = edge_ref.start_vertex();
                let v2 = edge_ref.end_vertex();
                
                if let (Some(v1_ref), Some(v2_ref)) = (v1.get(), v2.get()) {
                    let p1 = v1_ref.point();
                    let p2 = v2_ref.point();
                    
                    if self.do_segments_intersect(point, &ray_end, &p1, &p2) {
                        crossings += 1;
                    }
                }
            }
        }
        
        // If crossings is odd, point is inside
        crossings % 2 == 1
    }

    /// Check if two line segments intersect
    fn do_segments_intersect(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
        // Calculate orientation
        let o1 = self.orientation(p1, p2, p3);
        let o2 = self.orientation(p1, p2, p4);
        let o3 = self.orientation(p3, p4, p1);
        let o4 = self.orientation(p3, p4, p2);
        
        // General case
        if (o1 != o2) && (o3 != o4) {
            return true;
        }
        
        // Special cases
        // p3 lies on p1-p2
        if o1 == 0 && self.on_segment(p1, p3, p2) {
            return true;
        }
        
        // p4 lies on p1-p2
        if o2 == 0 && self.on_segment(p1, p4, p2) {
            return true;
        }
        
        // p1 lies on p3-p4
        if o3 == 0 && self.on_segment(p3, p1, p4) {
            return true;
        }
        
        // p2 lies on p3-p4
        if o4 == 0 && self.on_segment(p3, p2, p4) {
            return true;
        }
        
        false
    }

    /// Calculate orientation of three points
    fn orientation(&self, p: &Point, q: &Point, r: &Point) -> i32 {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        
        if val > self.tolerance {
            1 // Clockwise
        } else if val < -self.tolerance {
            2 // Counterclockwise
        } else {
            0 // Collinear
        }
    }

    /// Check if point q lies on segment pr
    fn on_segment(&self, p: &Point, q: &Point, r: &Point) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
        q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y) &&
        q.z <= p.z.max(r.z) && q.z >= p.z.min(r.z)
    }

    /// Newton-Raphson method for point-surface distance
    fn newton_raphson_point_surface(
        &self,
        point: &Point,
        surface: &SurfaceEnum,
        initial_u: StandardReal,
        initial_v: StandardReal,
    ) -> Option<(Point, StandardReal)> {
        let mut u = initial_u;
        let mut v = initial_v;
        
        for _ in 0..self.max_iterations {
            // Evaluate surface at current parameters
            let surface_point = surface.value(u, v);
            let normal = surface.normal(u, v);
            
            // Calculate distance vector
            let distance_vec = surface_point - *point;
            let distance = distance_vec.magnitude();
            
            // Check if we've converged
            if distance < self.tolerance {
                return Some((surface_point, distance));
            }
            
            // Calculate derivatives
            let du = self.surface_derivative_u(surface, u, v);
            let dv = self.surface_derivative_v(surface, u, v);
            
            // Calculate Jacobian matrix
            let j11 = du.dot(&du);
            let j12 = du.dot(&dv);
            let j21 = j12;
            let j22 = dv.dot(&dv);
            
            // Calculate determinant
            let det = j11 * j22 - j12 * j21;
            
            if det.abs() < self.tolerance {
                break;
            }
            
            // Calculate gradient
            let grad_u = distance_vec.dot(&du);
            let grad_v = distance_vec.dot(&dv);
            
            // Solve for delta
            let delta_u = (j22 * grad_u - j12 * grad_v) / det;
            let delta_v = (j11 * grad_v - j21 * grad_u) / det;
            
            // Update parameters
            u -= delta_u;
            v -= delta_v;
            
            // Check if parameters are within bounds
            let ((u_min, u_max), (v_min, v_max)) = surface.parameter_range();
            
            if u < u_min || u > u_max || v < v_min || v > v_max {
                break;
            }
        }
        
        None
    }

    /// Newton-Raphson method for surface-surface distance
    fn newton_raphson_surface_surface(
        &self,
        surface1: &SurfaceEnum,
        surface2: &SurfaceEnum,
        initial_u1: StandardReal,
        initial_v1: StandardReal,
        initial_u2: StandardReal,
        initial_v2: StandardReal,
    ) -> Option<(Point, Point, StandardReal)> {
        let mut u1 = initial_u1;
        let mut v1 = initial_v1;
        let mut u2 = initial_u2;
        let mut v2 = initial_v2;
        
        for _ in 0..self.max_iterations {
            // Evaluate both surfaces at current parameters
            let point1 = surface1.value(u1, v1);
            let point2 = surface2.value(u2, v2);
            
            // Calculate distance vector
            let distance_vec = point2 - point1;
            let distance = distance_vec.magnitude();
            
            // Check if we've converged
            if distance < self.tolerance {
                return Some((point1, point2, distance));
            }
            
            // Calculate derivatives
            let du1 = self.surface_derivative_u(surface1, u1, v1);
            let dv1 = self.surface_derivative_v(surface1, u1, v1);
            let du2 = self.surface_derivative_u(surface2, u2, v2);
            let dv2 = self.surface_derivative_v(surface2, u2, v2);
            
            // For simplicity, use gradient descent
            let learning_rate = 0.1;
            let delta_u1 = learning_rate * distance_vec.dot(&du1);
            let delta_v1 = learning_rate * distance_vec.dot(&dv1);
            let delta_u2 = -learning_rate * distance_vec.dot(&du2);
            let delta_v2 = -learning_rate * distance_vec.dot(&dv2);
            
            // Update parameters
            u1 += delta_u1;
            v1 += delta_v1;
            u2 += delta_u2;
            v2 += delta_v2;
            
            // Check if parameters are within bounds
            let ((u1_min, u1_max), (v1_min, v1_max)) = surface1.parameter_range();
            let ((u2_min, u2_max), (v2_min, v2_max)) = surface2.parameter_range();
            
            if u1 < u1_min || u1 > u1_max || v1 < v1_min || v1 > v1_max ||
               u2 < u2_min || u2 > u2_max || v2 < v2_min || v2 > v2_max {
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
}

impl Default for DistanceCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{plane::Plane, Point};

    #[test]
    fn test_point_to_surface() {
        // Create a plane
        let plane = Plane::xy_plane();
        let surface = SurfaceEnum::Plane(plane);
        
        // Create a point above the plane
        let point = Point::new(1.0, 1.0, 1.0);
        
        let calculator = DistanceCalculator::new();
        let result = calculator.point_to_surface(&point, &surface);
        
        assert!((result.distance - 1.0).abs() < 1e-6);
        assert!((result.closest_point2.unwrap().x - 1.0).abs() < 1e-6);
        assert!((result.closest_point2.unwrap().y - 1.0).abs() < 1e-6);
        assert!((result.closest_point2.unwrap().z).abs() < 1e-6);
    }

    #[test]
    fn test_surface_to_surface() {
        // Create two parallel planes
        let plane1 = Plane::xy_plane();
        let surface1 = SurfaceEnum::Plane(plane1);
        
        let plane2 = Plane::new(Point::new(0.0, 0.0, 2.0), crate::geometry::Direction::z_axis(), crate::geometry::Direction::x_axis());
        let surface2 = SurfaceEnum::Plane(plane2);
        
        let calculator = DistanceCalculator::new();
        let result = calculator.surface_to_surface(&surface1, &surface2);
        
        assert!((result.distance - 2.0).abs() < 1e-6);
    }
}
