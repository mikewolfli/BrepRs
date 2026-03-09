use crate::foundation::types::StandardReal;
use crate::geometry::{Cylinder, Direction, Plane, Point, Sphere, Vector};

/// Represents an intersection curve or point between two surfaces
#[derive(Debug, Clone, PartialEq)]
pub enum SurfaceIntersection {
    /// Single intersection point
    Point(Point),
    /// Intersection curve (represented as a list of points)
    Curve(Vec<Point>),
    /// No intersection
    None,
}

/// Intersection calculator for 3D surfaces
pub struct SurfaceIntersection3D;

impl SurfaceIntersection3D {
    /// Find intersections between two planes
    pub fn plane_plane(
        plane1: &Plane,
        plane2: &Plane,
        tolerance: StandardReal,
    ) -> SurfaceIntersection {
        let n1 = plane1.normal();
        let n2 = plane2.normal();

        // Convert directions to vectors for cross product
        let n1_vec = Vector::new(n1.x, n1.y, n1.z);
        let n2_vec = Vector::new(n2.x, n2.y, n2.z);

        // Check if planes are parallel
        let cross_vec = n1_vec.cross(&n2_vec);
        let cross_mag = cross_vec.magnitude();

        if cross_mag < tolerance {
            // Planes are parallel or coincident
            // Check if they are coincident
            let d1 = -(n1.x * plane1.location().x
                + n1.y * plane1.location().y
                + n1.z * plane1.location().z);
            let d2 = -(n2.x * plane2.location().x
                + n2.y * plane2.location().y
                + n2.z * plane2.location().z);

            if (d1 - d2).abs() < tolerance {
                // Planes are coincident - infinite intersection
                // Return a single point for simplicity
                SurfaceIntersection::Point(*plane1.location())
            } else {
                SurfaceIntersection::None
            }
        } else {
            // Planes intersect in a line
            // Calculate a point on the intersection line
            let mut points = Vec::new();

            // Find the direction of the intersection line
            let line_dir = cross_vec.normalized();

            // Find a point on the intersection line
            // Solve the system of plane equations
            let d1 = -(n1.x * plane1.location().x
                + n1.y * plane1.location().y
                + n1.z * plane1.location().z);
            let d2 = -(n2.x * plane2.location().x
                + n2.y * plane2.location().y
                + n2.z * plane2.location().z);

            // Choose the component with largest cross product magnitude
            let abs_cross = [cross_vec.x.abs(), cross_vec.y.abs(), cross_vec.z.abs()];

            let max_idx = if abs_cross[0] > abs_cross[1] && abs_cross[0] > abs_cross[2] {
                0
            } else if abs_cross[1] > abs_cross[2] {
                1
            } else {
                2
            };

            let point = if max_idx == 0 {
                // Use y-z plane
                let y = (d2 * n1.z - d1 * n2.z) / cross_vec.x;
                let z = (d1 * n2.y - d2 * n1.y) / cross_vec.x;
                Point::new(0.0, y, z)
            } else if max_idx == 1 {
                // Use x-z plane
                let x = (d1 * n2.z - d2 * n1.z) / cross_vec.y;
                let z = (d2 * n1.x - d1 * n2.x) / cross_vec.y;
                Point::new(x, 0.0, z)
            } else {
                // Use x-y plane
                let x = (d2 * n1.y - d1 * n2.y) / cross_vec.z;
                let y = (d1 * n2.x - d2 * n1.x) / cross_vec.z;
                Point::new(x, y, 0.0)
            };

            // Generate points along the intersection line
            for i in -5..=5 {
                let t = i as StandardReal;
                points.push(Point::new(
                    point.x + t * line_dir.x,
                    point.y + t * line_dir.y,
                    point.z + t * line_dir.z,
                ));
            }

            SurfaceIntersection::Curve(points)
        }
    }

    /// Find intersections between a plane and a sphere
    pub fn plane_sphere(
        plane: &Plane,
        sphere: &Sphere,
        tolerance: StandardReal,
    ) -> SurfaceIntersection {
        let plane_loc = plane.location();
        let normal = plane.normal();
        let sphere_loc = sphere.location();
        let radius = sphere.radius();

        // Calculate distance from sphere center to plane
        let dx = sphere_loc.x - plane_loc.x;
        let dy = sphere_loc.y - plane_loc.y;
        let dz = sphere_loc.z - plane_loc.z;

        let dist = (dx * normal.x + dy * normal.y + dz * normal.z).abs();

        if dist > radius + tolerance {
            // No intersection
            SurfaceIntersection::None
        } else if (dist - radius).abs() < tolerance {
            // Tangent - one intersection point
            let sign = if dx * normal.x + dy * normal.y + dz * normal.z > 0.0 {
                1.0
            } else {
                -1.0
            };
            let point = Point::new(
                sphere_loc.x - sign * radius * normal.x,
                sphere_loc.y - sign * radius * normal.y,
                sphere_loc.z - sign * radius * normal.z,
            );
            SurfaceIntersection::Point(point)
        } else {
            // Circle intersection
            let mut points = Vec::new();

            // Calculate circle center and radius
            let sign = if dx * normal.x + dy * normal.y + dz * normal.z > 0.0 {
                1.0
            } else {
                -1.0
            };
            let circle_center = Point::new(
                sphere_loc.x - sign * dist * normal.x,
                sphere_loc.y - sign * dist * normal.y,
                sphere_loc.z - sign * dist * normal.z,
            );

            let circle_radius = (radius * radius - dist * dist).sqrt();

            // Generate two perpendicular directions in the plane
            let u_dir = if normal.x.abs() > 0.1 {
                Vector::new(-normal.y, normal.x, 0.0).normalized()
            } else {
                Vector::new(0.0, -normal.z, normal.y).normalized()
            };

            let v_dir = Vector::new(
                normal.y * u_dir.z - normal.z * u_dir.y,
                normal.z * u_dir.x - normal.x * u_dir.z,
                normal.x * u_dir.y - normal.y * u_dir.x,
            );

            // Generate points on the intersection circle
            for i in 0..32 {
                let angle = 2.0 * std::f64::consts::PI * (i as StandardReal) / 32.0;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                points.push(Point::new(
                    circle_center.x + circle_radius * (cos_a * u_dir.x + sin_a * v_dir.x),
                    circle_center.y + circle_radius * (cos_a * u_dir.y + sin_a * v_dir.y),
                    circle_center.z + circle_radius * (cos_a * u_dir.z + sin_a * v_dir.z),
                ));
            }

            SurfaceIntersection::Curve(points)
        }
    }

    /// Find intersections between a plane and a cylinder
    pub fn plane_cylinder(
        plane: &Plane,
        cylinder: &Cylinder,
        tolerance: StandardReal,
    ) -> SurfaceIntersection {
        let plane_loc = plane.location();
        let plane_normal = plane.normal();
        let cyl_loc = cylinder.location();
        let cyl_axis = cylinder.direction();
        let radius = cylinder.radius();

        // Calculate angle between plane normal and cylinder axis
        let dot =
            plane_normal.x * cyl_axis.x + plane_normal.y * cyl_axis.y + plane_normal.z * cyl_axis.z;
        let angle = dot.abs();

        if angle < tolerance {
            // Plane is parallel to cylinder axis
            // Distance from cylinder axis to plane
            let dx = cyl_loc.x - plane_loc.x;
            let dy = cyl_loc.y - plane_loc.y;
            let dz = cyl_loc.z - plane_loc.z;
            let dist = (dx * plane_normal.x + dy * plane_normal.y + dz * plane_normal.z).abs();

            if dist > radius + tolerance {
                SurfaceIntersection::None
            } else if (dist - radius).abs() < tolerance {
                // Tangent - one line
                let mut points = Vec::new();
                for i in -5..=5 {
                    let t = i as StandardReal;
                    points.push(Point::new(
                        cyl_loc.x + t * cyl_axis.x,
                        cyl_loc.y + t * cyl_axis.y,
                        cyl_loc.z + t * cyl_axis.z,
                    ));
                }
                SurfaceIntersection::Curve(points)
            } else {
                // Two parallel lines - simplified to return cylinder location line
                let mut points = Vec::new();
                for i in -5..=5 {
                    let t = i as StandardReal;
                    points.push(Point::new(
                        cyl_loc.x + t * cyl_axis.x,
                        cyl_loc.y + t * cyl_axis.y,
                        cyl_loc.z + t * cyl_axis.z,
                    ));
                }
                SurfaceIntersection::Curve(points)
            }
        } else if (angle - 1.0).abs() < tolerance {
            // Plane is perpendicular to cylinder axis
            // Circle intersection
            let mut points = Vec::new();

            // Project cylinder location onto plane
            let dx = cyl_loc.x - plane_loc.x;
            let dy = cyl_loc.y - plane_loc.y;
            let dz = cyl_loc.z - plane_loc.z;
            let dist = dx * plane_normal.x + dy * plane_normal.y + dz * plane_normal.z;

            let center = Point::new(
                cyl_loc.x - dist * plane_normal.x,
                cyl_loc.y - dist * plane_normal.y,
                cyl_loc.z - dist * plane_normal.z,
            );

            // Generate two perpendicular directions in the plane
            let u_dir = if plane_normal.x.abs() > 0.1 {
                Vector::new(-plane_normal.y, plane_normal.x, 0.0).normalized()
            } else {
                Vector::new(0.0, -plane_normal.z, plane_normal.y).normalized()
            };

            let v_dir = Vector::new(
                plane_normal.y * u_dir.z - plane_normal.z * u_dir.y,
                plane_normal.z * u_dir.x - plane_normal.x * u_dir.z,
                plane_normal.x * u_dir.y - plane_normal.y * u_dir.x,
            );

            // Generate points on the intersection circle
            for i in 0..32 {
                let angle = 2.0 * std::f64::consts::PI * (i as StandardReal) / 32.0;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                points.push(Point::new(
                    center.x + radius * (cos_a * u_dir.x + sin_a * v_dir.x),
                    center.y + radius * (cos_a * u_dir.y + sin_a * v_dir.y),
                    center.z + radius * (cos_a * u_dir.z + sin_a * v_dir.z),
                ));
            }

            SurfaceIntersection::Curve(points)
        } else {
            // Ellipse intersection - simplified to return a curve
            let mut points = Vec::new();

            // Generate sample points
            for i in 0..32 {
                let angle = 2.0 * std::f64::consts::PI * (i as StandardReal) / 32.0;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                points.push(Point::new(
                    cyl_loc.x + radius * cos_a,
                    cyl_loc.y + radius * sin_a,
                    cyl_loc.z,
                ));
            }

            SurfaceIntersection::Curve(points)
        }
    }

    /// Find intersections between two spheres
    pub fn sphere_sphere(
        sphere1: &Sphere,
        sphere2: &Sphere,
        tolerance: StandardReal,
    ) -> SurfaceIntersection {
        let c1 = sphere1.location();
        let r1 = sphere1.radius();
        let c2 = sphere2.location();
        let r2 = sphere2.radius();

        let dx = c2.x - c1.x;
        let dy = c2.y - c1.y;
        let dz = c2.z - c1.z;
        let d = (dx * dx + dy * dy + dz * dz).sqrt();

        // Check for no intersection
        if d > r1 + r2 + tolerance || d < (r1 - r2).abs() - tolerance {
            return SurfaceIntersection::None;
        }

        // Check for coincident spheres
        if d < tolerance && (r1 - r2).abs() < tolerance {
            return SurfaceIntersection::Point(*c1);
        }

        // Check for one sphere inside another without touching
        if d + r1.min(r2) < r1.max(r2) - tolerance {
            return SurfaceIntersection::None;
        }

        if (d - r1 - r2).abs() < tolerance || (d - (r1 - r2).abs()).abs() < tolerance {
            // Tangent - one intersection point
            let t = if d > tolerance { r1 / d } else { 0.0 };
            let point = Point::new(c1.x + t * dx, c1.y + t * dy, c1.z + t * dz);
            SurfaceIntersection::Point(point)
        } else {
            // Circle intersection
            let mut points = Vec::new();

            // Calculate circle center and radius
            let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
            let h_sq = r1 * r1 - a * a;

            if h_sq < -tolerance {
                return SurfaceIntersection::None;
            }

            let h = if h_sq < 0.0 { 0.0 } else { h_sq.sqrt() };

            let center = Point::new(c1.x + a * dx / d, c1.y + a * dy / d, c1.z + a * dz / d);

            // Generate two perpendicular directions
            let normal = Vector::new(dx / d, dy / d, dz / d);
            let u_dir = if normal.x.abs() > 0.1 {
                Vector::new(-normal.y, normal.x, 0.0).normalized()
            } else {
                Vector::new(0.0, -normal.z, normal.y).normalized()
            };

            let v_dir = Vector::new(
                normal.y * u_dir.z - normal.z * u_dir.y,
                normal.z * u_dir.x - normal.x * u_dir.z,
                normal.x * u_dir.y - normal.y * u_dir.x,
            );

            // Generate points on the intersection circle
            for i in 0..32 {
                let angle = 2.0 * std::f64::consts::PI * (i as StandardReal) / 32.0;
                let cos_a = angle.cos();
                let sin_a = angle.sin();

                points.push(Point::new(
                    center.x + h * (cos_a * u_dir.x + sin_a * v_dir.x),
                    center.y + h * (cos_a * u_dir.y + sin_a * v_dir.y),
                    center.z + h * (cos_a * u_dir.z + sin_a * v_dir.z),
                ));
            }

            SurfaceIntersection::Curve(points)
        }
    }
}

/// Surface operations for 3D surfaces
pub struct SurfaceOperations3D;

impl SurfaceOperations3D {
    /// Calculate the distance from a point to a plane
    pub fn point_plane_distance(point: &Point, plane: &Plane) -> StandardReal {
        let plane_loc = plane.location();
        let normal = plane.normal();

        let dx = point.x - plane_loc.x;
        let dy = point.y - plane_loc.y;
        let dz = point.z - plane_loc.z;

        (dx * normal.x + dy * normal.y + dz * normal.z).abs()
    }

    /// Calculate the signed distance from a point to a plane
    pub fn point_plane_signed_distance(point: &Point, plane: &Plane) -> StandardReal {
        let plane_loc = plane.location();
        let normal = plane.normal();

        let dx = point.x - plane_loc.x;
        let dy = point.y - plane_loc.y;
        let dz = point.z - plane_loc.z;

        dx * normal.x + dy * normal.y + dz * normal.z
    }

    /// Calculate the distance from a point to a sphere
    pub fn point_sphere_distance(point: &Point, sphere: &Sphere) -> StandardReal {
        let center = sphere.location();
        let radius = sphere.radius();

        let dx = point.x - center.x;
        let dy = point.y - center.y;
        let dz = point.z - center.z;
        let dist_to_center = (dx * dx + dy * dy + dz * dz).sqrt();

        (dist_to_center - radius).abs()
    }

    /// Project a point onto a plane
    pub fn project_point_to_plane(point: &Point, plane: &Plane) -> Point {
        let plane_loc = plane.location();
        let normal = plane.normal();

        let dx = point.x - plane_loc.x;
        let dy = point.y - plane_loc.y;
        let dz = point.z - plane_loc.z;

        let dist = dx * normal.x + dy * normal.y + dz * normal.z;

        Point::new(
            point.x - dist * normal.x,
            point.y - dist * normal.y,
            point.z - dist * normal.z,
        )
    }

    /// Check if a point is on a surface within tolerance
    pub fn is_point_on_plane(point: &Point, plane: &Plane, tolerance: StandardReal) -> bool {
        Self::point_plane_distance(point, plane) <= tolerance
    }

    /// Check if a point is on a sphere within tolerance
    pub fn is_point_on_sphere(point: &Point, sphere: &Sphere, tolerance: StandardReal) -> bool {
        Self::point_sphere_distance(point, sphere) <= tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Direction, Point};

    #[test]
    fn test_plane_plane_intersection() {
        let plane1 = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Direction::z_axis());
        let plane2 = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Direction::x_axis());

        let intersection = SurfaceIntersection3D::plane_plane(&plane1, &plane2, 0.001);
        match intersection {
            SurfaceIntersection::Curve(points) => {
                assert!(!points.is_empty());
            }
            _ => panic!("Expected curve intersection"),
        }
    }

    #[test]
    fn test_plane_sphere_intersection() {
        let plane = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Direction::z_axis());
        let sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let intersection = SurfaceIntersection3D::plane_sphere(&plane, &sphere, 0.001);
        match intersection {
            SurfaceIntersection::Curve(points) => {
                assert!(!points.is_empty());
            }
            _ => panic!("Expected curve intersection"),
        }
    }

    #[test]
    fn test_sphere_sphere_intersection() {
        let sphere1 = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let sphere2 = Sphere::new(Point::new(0.5, 0.0, 0.0), 1.0);

        let intersection = SurfaceIntersection3D::sphere_sphere(&sphere1, &sphere2, 0.001);
        match intersection {
            SurfaceIntersection::Curve(points) => {
                assert!(!points.is_empty());
            }
            _ => panic!("Expected curve intersection"),
        }
    }

    #[test]
    fn test_point_plane_distance() {
        let point = Point::new(0.0, 0.0, 1.0);
        let plane = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Direction::z_axis());

        let distance = SurfaceOperations3D::point_plane_distance(&point, &plane);
        assert!((distance - 1.0).abs() < 0.001);
    }
}
