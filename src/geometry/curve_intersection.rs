use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector, Line2D, Circle2D, Ellipse2D};

/// Represents an intersection point between two curves
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveIntersection {
    pub point: Point,
    pub parameter1: Standard_Real,
    pub parameter2: Standard_Real,
}

impl CurveIntersection {
    pub fn new(point: Point, parameter1: Standard_Real, parameter2: Standard_Real) -> Self {
        Self {
            point,
            parameter1,
            parameter2,
        }
    }
}

/// Intersection calculator for 2D curves
pub struct CurveIntersection2D;

impl CurveIntersection2D {
    /// Find intersections between two lines
    pub fn line_line(line1: &Line2D, line2: &Line2D, tolerance: Standard_Real) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();
        
        let p1 = line1.location();
        let d1 = line1.direction();
        let p2 = line2.location();
        let d2 = line2.direction();
        
        // Check if lines are parallel
        let cross = d1.x * d2.y - d1.y * d2.x;
        
        if cross.abs() < tolerance {
            // Lines are parallel or coincident
            // Check if they are coincident
            let p1_to_p2 = Vector::new(p2.x - p1.x, p2.y - p1.y, 0.0);
            let cross2 = d1.x * p1_to_p2.y - d1.y * p1_to_p2.x;
            
            if cross2.abs() < tolerance {
                // Lines are coincident - infinite intersections
                // Return one point for simplicity
                intersections.push(CurveIntersection::new(*p1, 0.0, 0.0));
            }
            return intersections;
        }
        
        // Calculate intersection parameters
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        
        let t1 = (dx * d2.y - dy * d2.x) / cross;
        let t2 = (dx * d1.y - dy * d1.x) / cross;
        
        let intersection_point = Point::new(
            p1.x + t1 * d1.x,
            p1.y + t1 * d1.y,
            0.0,
        );
        
        intersections.push(CurveIntersection::new(intersection_point, t1, t2));
        intersections
    }
    
    /// Find intersections between a line and a circle
    pub fn line_circle(line: &Line2D, circle: &Circle2D, tolerance: Standard_Real) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();
        
        let line_loc = line.location();
        let line_dir = line.direction();
        let circle_loc = circle.location();
        let radius = circle.radius();
        
        // Transform line to circle's local coordinate system
        let dx = line_loc.x - circle_loc.x;
        let dy = line_loc.y - circle_loc.y;
        
        // Line parameterization: P = line_loc + t * line_dir
        // Circle equation: (x - circle_loc.x)^2 + (y - circle_loc.y)^2 = radius^2
        
        let a = line_dir.x * line_dir.x + line_dir.y * line_dir.y;
        let b = 2.0 * (dx * line_dir.x + dy * line_dir.y);
        let c = dx * dx + dy * dy - radius * radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < -tolerance {
            // No intersection
            return intersections;
        }
        
        if discriminant.abs() < tolerance {
            // Tangent - one intersection
            let t = -b / (2.0 * a);
            let point = Point::new(
                line_loc.x + t * line_dir.x,
                line_loc.y + t * line_dir.y,
                0.0,
            );
            intersections.push(CurveIntersection::new(point, t, 0.0));
        } else {
            // Two intersections
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b + sqrt_disc) / (2.0 * a);
            let t2 = (-b - sqrt_disc) / (2.0 * a);
            
            let point1 = Point::new(
                line_loc.x + t1 * line_dir.x,
                line_loc.y + t1 * line_dir.y,
                0.0,
            );
            let point2 = Point::new(
                line_loc.x + t2 * line_dir.x,
                line_loc.y + t2 * line_dir.y,
                0.0,
            );
            
            intersections.push(CurveIntersection::new(point1, t1, 0.0));
            intersections.push(CurveIntersection::new(point2, t2, 0.0));
        }
        
        intersections
    }
    
    /// Find intersections between two circles
    pub fn circle_circle(circle1: &Circle2D, circle2: &Circle2D, tolerance: Standard_Real) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();
        
        let c1 = circle1.location();
        let r1 = circle1.radius();
        let c2 = circle2.location();
        let r2 = circle2.radius();
        
        let dx = c2.x - c1.x;
        let dy = c2.y - c1.y;
        let d = (dx * dx + dy * dy).sqrt();
        
        // Check for no intersection
        if d > r1 + r2 + tolerance || d < (r1 - r2).abs() - tolerance {
            return intersections;
        }
        
        // Check for coincident circles
        if d < tolerance && (r1 - r2).abs() < tolerance {
            // Circles are coincident - infinite intersections
            // Return one point for simplicity
            intersections.push(CurveIntersection::new(
                Point::new(c1.x + r1, c1.y, 0.0),
                0.0,
                0.0,
            ));
            return intersections;
        }
        
        // Calculate intersection points
        let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
        let h_sq = r1 * r1 - a * a;
        
        if h_sq < -tolerance {
            return intersections;
        }
        
        let h = if h_sq < 0.0 { 0.0 } else { h_sq.sqrt() };
        
        let x2 = c1.x + a * dx / d;
        let y2 = c1.y + a * dy / d;
        
        if h < tolerance {
            // One intersection point (tangent)
            intersections.push(CurveIntersection::new(
                Point::new(x2, y2, 0.0),
                0.0,
                0.0,
            ));
        } else {
            // Two intersection points
            let rx = -dy * h / d;
            let ry = dx * h / d;
            
            let point1 = Point::new(x2 + rx, y2 + ry, 0.0);
            let point2 = Point::new(x2 - rx, y2 - ry, 0.0);
            
            // Calculate angles for parameterization
            let angle1 = (point1.y - c1.y).atan2(point1.x - c1.x);
            let angle2 = (point2.y - c1.y).atan2(point2.x - c1.x);
            
            intersections.push(CurveIntersection::new(point1, angle1, 0.0));
            intersections.push(CurveIntersection::new(point2, angle2, 0.0));
        }
        
        intersections
    }
    
    /// Find intersections between a line and an ellipse
    pub fn line_ellipse(line: &Line2D, ellipse: &Ellipse2D, tolerance: Standard_Real) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();
        
        let line_loc = line.location();
        let line_dir = line.direction();
        let ellipse_loc = ellipse.location();
        let major_radius = ellipse.major_radius();
        let minor_radius = ellipse.minor_radius();
        
        // Transform to ellipse's local coordinate system
        let dx = line_loc.x - ellipse_loc.x;
        let dy = line_loc.y - ellipse_loc.y;
        
        // Simplified intersection calculation
        // Line: P = line_loc + t * line_dir
        // Ellipse: (x/a)^2 + (y/b)^2 = 1
        
        let a = major_radius;
        let b = minor_radius;
        
        let A = (line_dir.x * line_dir.x) / (a * a) + (line_dir.y * line_dir.y) / (b * b);
        let B = 2.0 * ((dx * line_dir.x) / (a * a) + (dy * line_dir.y) / (b * b));
        let C = (dx * dx) / (a * a) + (dy * dy) / (b * b) - 1.0;
        
        let discriminant = B * B - 4.0 * A * C;
        
        if discriminant < -tolerance {
            return intersections;
        }
        
        if discriminant.abs() < tolerance {
            let t = -B / (2.0 * A);
            let point = Point::new(
                line_loc.x + t * line_dir.x,
                line_loc.y + t * line_dir.y,
                0.0,
            );
            intersections.push(CurveIntersection::new(point, t, 0.0));
        } else {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-B + sqrt_disc) / (2.0 * A);
            let t2 = (-B - sqrt_disc) / (2.0 * A);
            
            let point1 = Point::new(
                line_loc.x + t1 * line_dir.x,
                line_loc.y + t1 * line_dir.y,
                0.0,
            );
            let point2 = Point::new(
                line_loc.x + t2 * line_dir.x,
                line_loc.y + t2 * line_dir.y,
                0.0,
            );
            
            intersections.push(CurveIntersection::new(point1, t1, 0.0));
            intersections.push(CurveIntersection::new(point2, t2, 0.0));
        }
        
        intersections
    }
}

/// Curve operations for 2D curves
pub struct CurveOperations2D;

impl CurveOperations2D {
    /// Calculate the distance from a point to a line
    pub fn point_line_distance(point: &Point, line: &Line2D) -> Standard_Real {
        let line_loc = line.location();
        let line_dir = line.direction();
        
        let dx = point.x - line_loc.x;
        let dy = point.y - line_loc.y;
        
        // Cross product magnitude / line direction magnitude
        let cross = dx * line_dir.y - dy * line_dir.x;
        let dir_mag = (line_dir.x * line_dir.x + line_dir.y * line_dir.y).sqrt();
        
        if dir_mag < STANDARD_REAL_EPSILON {
            return (dx * dx + dy * dy).sqrt();
        }
        
        cross.abs() / dir_mag
    }
    
    /// Calculate the distance from a point to a circle
    pub fn point_circle_distance(point: &Point, circle: &Circle2D) -> Standard_Real {
        let center = circle.location();
        let radius = circle.radius();
        
        let dx = point.x - center.x;
        let dy = point.y - center.y;
        let dist_to_center = (dx * dx + dy * dy).sqrt();
        
        (dist_to_center - radius).abs()
    }
    
    /// Check if a point is on a line within tolerance
    pub fn is_point_on_line(point: &Point, line: &Line2D, tolerance: Standard_Real) -> bool {
        Self::point_line_distance(point, line) <= tolerance
    }
    
    /// Check if a point is on a circle within tolerance
    pub fn is_point_on_circle(point: &Point, circle: &Circle2D, tolerance: Standard_Real) -> bool {
        Self::point_circle_distance(point, circle) <= tolerance
    }
    
    /// Project a point onto a line
    pub fn project_point_to_line(point: &Point, line: &Line2D) -> Point {
        let line_loc = line.location();
        let line_dir = line.direction();
        
        let dx = point.x - line_loc.x;
        let dy = point.y - line_loc.y;
        
        let dot = dx * line_dir.x + dy * line_dir.y;
        let dir_mag_sq = line_dir.x * line_dir.x + line_dir.y * line_dir.y;
        
        if dir_mag_sq < STANDARD_REAL_EPSILON {
            return *line_loc;
        }
        
        let t = dot / dir_mag_sq;
        
        Point::new(
            line_loc.x + t * line_dir.x,
            line_loc.y + t * line_dir.y,
            0.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Point, Direction};

    #[test]
    fn test_line_line_intersection() {
        let line1 = Line2D::new(Point::new(0.0, 0.0, 0.0), Direction::new(1.0, 0.0, 0.0));
        let line2 = Line2D::new(Point::new(1.0, 0.0, 0.0), Direction::new(0.0, 1.0, 0.0));
        let intersections = CurveIntersection2D::line_line(&line1, &line2, 0.001);
        assert_eq!(intersections.len(), 1);
        assert!((intersections[0].point.x - 1.0).abs() < 0.001);
        assert!((intersections[0].point.y - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_line_circle_intersection() {
        let line = Line2D::new(Point::new(0.0, 0.0, 0.0), Direction::new(1.0, 0.0, 0.0));
        let circle = Circle2D::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0), 1.0);
        
        let intersections = CurveIntersection2D::line_circle(&line, &circle, 0.001);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_circle_circle_intersection() {
        let circle1 = Circle2D::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0), 1.0);
        let circle2 = Circle2D::new(Point::new(1.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0), 1.0);
        
        let intersections = CurveIntersection2D::circle_circle(&circle1, &circle2, 0.001);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_point_line_distance() {
        let point = Point::new(0.0, 1.0, 0.0);
        let line = Line2D::new(Point::new(0.0, 0.0, 0.0), Direction::new(1.0, 0.0, 0.0));
        
        let distance = CurveOperations2D::point_line_distance(&point, &line);
        assert!((distance - 1.0).abs() < 0.001);
    }
}
