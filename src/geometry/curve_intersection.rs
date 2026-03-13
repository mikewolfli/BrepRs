use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::advanced_traits::Curve;
use crate::geometry::{Circle2D, Ellipse2D, Line2D, Point, Vector};

/// Represents an intersection point between two curves
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveIntersection {
    pub point: Point,
    pub parameter1: StandardReal,
    pub parameter2: StandardReal,
}

impl CurveIntersection {
    pub fn new(point: Point, parameter1: StandardReal, parameter2: StandardReal) -> Self {
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
    /// Find intersections between two NURBS curves using subdivision method
    /// This implementation uses adaptive subdivision to find accurate intersections
    pub fn nurbs_nurbs(
        curve1: &crate::geometry::nurbs_curve2d::NurbsCurve2D,
        curve2: &crate::geometry::nurbs_curve2d::NurbsCurve2D,
        tolerance: StandardReal,
        _samples: usize,
    ) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();

        // Bounding box pre-check
        let bbox1 = Self::calculate_nurbs_bounding_box(curve1);
        let bbox2 = Self::calculate_nurbs_bounding_box(curve2);

        // If bounding boxes do not overlap, skip
        if !Self::bounding_boxes_overlap(&bbox1, &bbox2) {
            return intersections;
        }

        // Use adaptive subdivision to find intersections
        Self::subdivide_nurbs_nurbs(
            curve1,
            curve2,
            0.0,
            1.0,
            0.0,
            1.0,
            tolerance,
            &mut intersections,
        );

        // Remove duplicate intersections
        Self::remove_duplicates(&mut intersections, tolerance);

        intersections
    }

    /// Calculate bounding box for a NURBS curve
    fn calculate_nurbs_bounding_box(
        curve: &crate::geometry::nurbs_curve2d::NurbsCurve2D,
    ) -> (Point, Point) {
        let poles = curve.poles();

        if poles.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_x = poles[0].x;
        let mut min_y = poles[0].y;
        let mut max_x = poles[0].x;
        let mut max_y = poles[0].y;

        for pole in &poles[1..] {
            min_x = min_x.min(pole.x);
            min_y = min_y.min(pole.y);
            max_x = max_x.max(pole.x);
            max_y = max_y.max(pole.y);
        }

        (Point::new(min_x, min_y, 0.0), Point::new(max_x, max_y, 0.0))
    }

    /// Check if two bounding boxes overlap
    fn bounding_boxes_overlap(bbox1: &(Point, Point), bbox2: &(Point, Point)) -> bool {
        let (min1, max1) = bbox1;
        let (min2, max2) = bbox2;

        !(max1.x < min2.x || min1.x > max2.x || max1.y < min2.y || min1.y > max2.y)
    }

    /// Adaptive subdivision for NURBS-NURBS intersection
    fn subdivide_nurbs_nurbs(
        curve1: &crate::geometry::nurbs_curve2d::NurbsCurve2D,
        curve2: &crate::geometry::nurbs_curve2d::NurbsCurve2D,
        t1_start: StandardReal,
        t1_end: StandardReal,
        t2_start: StandardReal,
        t2_end: StandardReal,
        tolerance: StandardReal,
        intersections: &mut Vec<CurveIntersection>,
    ) {
        // Get the endpoints of the curve segments
        let p1_start = curve1.position(t1_start);
        let p1_end = curve1.position(t1_end);
        let p2_start = curve2.position(t2_start);
        let p2_end = curve2.position(t2_end);

        // Check if the line segments intersect
        if let Some((t1, t2)) =
            Self::line_segment_intersection(&p1_start, &p1_end, &p2_start, &p2_end, tolerance)
        {
            // Calculate the actual parameters on the original curves
            let actual_t1 = t1_start + t1 * (t1_end - t1_start);
            let actual_t2 = t2_start + t2 * (t2_end - t2_start);

            // Check if the intersection point is within the tolerance
            let p1 = curve1.position(actual_t1);
            let p2 = curve2.position(actual_t2);
            let dist = p1.distance(&p2);

            if dist < tolerance {
                intersections.push(CurveIntersection::new(p1, actual_t1, actual_t2));
                return;
            }
        }

        // Check if the segments are small enough
        let len1 = p1_start.distance(&p1_end);
        let len2 = p2_start.distance(&p2_end);

        if len1 < tolerance && len2 < tolerance {
            return;
        }

        // Subdivide the curves and recurse
        let t1_mid = (t1_start + t1_end) / 2.0;
        let t2_mid = (t2_start + t2_end) / 2.0;

        Self::subdivide_nurbs_nurbs(
            curve1,
            curve2,
            t1_start,
            t1_mid,
            t2_start,
            t2_mid,
            tolerance,
            intersections,
        );
        Self::subdivide_nurbs_nurbs(
            curve1,
            curve2,
            t1_mid,
            t1_end,
            t2_start,
            t2_mid,
            tolerance,
            intersections,
        );
        Self::subdivide_nurbs_nurbs(
            curve1,
            curve2,
            t1_start,
            t1_mid,
            t2_mid,
            t2_end,
            tolerance,
            intersections,
        );
        Self::subdivide_nurbs_nurbs(
            curve1,
            curve2,
            t1_mid,
            t1_end,
            t2_mid,
            t2_end,
            tolerance,
            intersections,
        );
    }

    /// Calculate intersection of two line segments
    fn line_segment_intersection(
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
        tolerance: StandardReal,
    ) -> Option<(StandardReal, StandardReal)> {
        let d1 = p2.x - p1.x;
        let d2 = p2.y - p1.y;
        let d3 = p4.x - p3.x;
        let d4 = p4.y - p3.y;

        let denominator = d1 * d4 - d2 * d3;

        if denominator.abs() < tolerance {
            // Lines are parallel or coincident
            return None;
        }

        let dx = p3.x - p1.x;
        let dy = p3.y - p1.y;

        let t1 = (dx * d4 - dy * d3) / denominator;
        let t2 = (dx * d2 - dy * d1) / denominator;

        if t1 >= 0.0 - tolerance
            && t1 <= 1.0 + tolerance
            && t2 >= 0.0 - tolerance
            && t2 <= 1.0 + tolerance
        {
            Some((t1.max(0.0).min(1.0), t2.max(0.0).min(1.0)))
        } else {
            None
        }
    }

    /// Remove duplicate intersections
    fn remove_duplicates(intersections: &mut Vec<CurveIntersection>, tolerance: StandardReal) {
        let mut unique: Vec<CurveIntersection> = Vec::new();

        for intersection in &mut *intersections {
            let mut is_duplicate = false;

            for unique_intersection in &unique {
                if intersection.point.distance(&unique_intersection.point) < tolerance {
                    is_duplicate = true;
                    break;
                }
            }

            if !is_duplicate {
                unique.push(*intersection);
            }
        }

        *intersections = unique;
    }
    /// Find intersections between two Bezier curves using subdivision method
    /// This implementation uses adaptive subdivision to find accurate intersections
    pub fn bezier_bezier(
        curve1: &crate::geometry::bezier_curve2d::BezierCurve2D,
        curve2: &crate::geometry::bezier_curve2d::BezierCurve2D,
        tolerance: StandardReal,
        _samples: usize,
    ) -> Vec<CurveIntersection> {
        let mut intersections = Vec::new();

        // Bounding box pre-check
        let bbox1 = Self::calculate_bezier_bounding_box(curve1);
        let bbox2 = Self::calculate_bezier_bounding_box(curve2);

        // If bounding boxes do not overlap, skip
        if !Self::bounding_boxes_overlap(&bbox1, &bbox2) {
            return intersections;
        }

        // Use adaptive subdivision to find intersections
        Self::subdivide_bezier_bezier(
            curve1,
            curve2,
            0.0,
            1.0,
            0.0,
            1.0,
            tolerance,
            &mut intersections,
        );

        // Remove duplicate intersections
        Self::remove_duplicates(&mut intersections, tolerance);

        intersections
    }

    /// Calculate bounding box for a Bezier curve
    fn calculate_bezier_bounding_box(
        curve: &crate::geometry::bezier_curve2d::BezierCurve2D,
    ) -> (Point, Point) {
        let poles = curve.poles();

        if poles.is_empty() {
            return (Point::origin(), Point::origin());
        }

        let mut min_x = poles[0].x;
        let mut min_y = poles[0].y;
        let mut max_x = poles[0].x;
        let mut max_y = poles[0].y;

        for pole in &poles[1..] {
            min_x = min_x.min(pole.x);
            min_y = min_y.min(pole.y);
            max_x = max_x.max(pole.x);
            max_y = max_y.max(pole.y);
        }

        (Point::new(min_x, min_y, 0.0), Point::new(max_x, max_y, 0.0))
    }

    /// Adaptive subdivision for Bezier-Bezier intersection
    fn subdivide_bezier_bezier(
        curve1: &crate::geometry::bezier_curve2d::BezierCurve2D,
        curve2: &crate::geometry::bezier_curve2d::BezierCurve2D,
        t1_start: StandardReal,
        t1_end: StandardReal,
        t2_start: StandardReal,
        t2_end: StandardReal,
        tolerance: StandardReal,
        intersections: &mut Vec<CurveIntersection>,
    ) {
        // Get the endpoints of the curve segments
        let p1_start = curve1.sample(t1_start);
        let p1_end = curve1.sample(t1_end);
        let p2_start = curve2.sample(t2_start);
        let p2_end = curve2.sample(t2_end);

        // Check if the line segments intersect
        if let Some((t1, t2)) =
            Self::line_segment_intersection(&p1_start, &p1_end, &p2_start, &p2_end, tolerance)
        {
            // Calculate the actual parameters on the original curves
            let actual_t1 = t1_start + t1 * (t1_end - t1_start);
            let actual_t2 = t2_start + t2 * (t2_end - t2_start);

            // Check if the intersection point is within the tolerance
            let p1 = curve1.sample(actual_t1);
            let p2 = curve2.sample(actual_t2);
            let dist = p1.distance(&p2);

            if dist < tolerance {
                intersections.push(CurveIntersection::new(p1, actual_t1, actual_t2));
                return;
            }
        }

        // Check if the segments are small enough
        let len1 = p1_start.distance(&p1_end);
        let len2 = p2_start.distance(&p2_end);

        if len1 < tolerance && len2 < tolerance {
            return;
        }

        // Subdivide the curves and recurse
        let t1_mid = (t1_start + t1_end) / 2.0;
        let t2_mid = (t2_start + t2_end) / 2.0;

        Self::subdivide_bezier_bezier(
            curve1,
            curve2,
            t1_start,
            t1_mid,
            t2_start,
            t2_mid,
            tolerance,
            intersections,
        );
        Self::subdivide_bezier_bezier(
            curve1,
            curve2,
            t1_mid,
            t1_end,
            t2_start,
            t2_mid,
            tolerance,
            intersections,
        );
        Self::subdivide_bezier_bezier(
            curve1,
            curve2,
            t1_start,
            t1_mid,
            t2_mid,
            t2_end,
            tolerance,
            intersections,
        );
        Self::subdivide_bezier_bezier(
            curve1,
            curve2,
            t1_mid,
            t1_end,
            t2_mid,
            t2_end,
            tolerance,
            intersections,
        );
    }
    /// Find intersections between two lines
    pub fn line_line(
        line1: &Line2D,
        line2: &Line2D,
        tolerance: StandardReal,
    ) -> Vec<CurveIntersection> {
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

        let intersection_point = Point::new(p1.x + t1 * d1.x, p1.y + t1 * d1.y, 0.0);

        intersections.push(CurveIntersection::new(intersection_point, t1, t2));
        intersections
    }

    /// Find intersections between a line and a circle
    pub fn line_circle(
        line: &Line2D,
        circle: &Circle2D,
        tolerance: StandardReal,
    ) -> Vec<CurveIntersection> {
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
    pub fn circle_circle(
        circle1: &Circle2D,
        circle2: &Circle2D,
        tolerance: StandardReal,
    ) -> Vec<CurveIntersection> {
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
            intersections.push(CurveIntersection::new(Point::new(x2, y2, 0.0), 0.0, 0.0));
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
    pub fn line_ellipse(
        line: &Line2D,
        ellipse: &Ellipse2D,
        tolerance: StandardReal,
    ) -> Vec<CurveIntersection> {
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

        let a_coeff = (line_dir.x * line_dir.x) / (a * a) + (line_dir.y * line_dir.y) / (b * b);
        let b_coeff = 2.0 * ((dx * line_dir.x) / (a * a) + (dy * line_dir.y) / (b * b));
        let c_coeff = (dx * dx) / (a * a) + (dy * dy) / (b * b) - 1.0;

        let discriminant = b_coeff * b_coeff - 4.0 * a_coeff * c_coeff;

        if discriminant < -tolerance {
            return intersections;
        }

        if discriminant.abs() < tolerance {
            let t = -b_coeff / (2.0 * a_coeff);
            let point = Point::new(
                line_loc.x + t * line_dir.x,
                line_loc.y + t * line_dir.y,
                0.0,
            );
            intersections.push(CurveIntersection::new(point, t, 0.0));
        } else {
            let sqrt_disc = discriminant.sqrt();
            let t1 = (-b_coeff + sqrt_disc) / (2.0 * a_coeff);
            let t2 = (-b_coeff - sqrt_disc) / (2.0 * a_coeff);

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
    pub fn point_line_distance(point: &Point, line: &Line2D) -> StandardReal {
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
    pub fn point_circle_distance(point: &Point, circle: &Circle2D) -> StandardReal {
        let center = circle.location();
        let radius = circle.radius();

        let dx = point.x - center.x;
        let dy = point.y - center.y;
        let dist_to_center = (dx * dx + dy * dy).sqrt();

        (dist_to_center - radius).abs()
    }

    /// Check if a point is on a line within tolerance
    pub fn is_point_on_line(point: &Point, line: &Line2D, tolerance: StandardReal) -> bool {
        Self::point_line_distance(point, line) <= tolerance
    }

    /// Check if a point is on a circle within tolerance
    pub fn is_point_on_circle(point: &Point, circle: &Circle2D, tolerance: StandardReal) -> bool {
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
    use crate::geometry::{Direction, Point};

    #[test]
    fn test_nurbs_nurbs_intersection() {
        use crate::geometry::nurbs_curve2d::NurbsCurve2D;
        let curve1 = NurbsCurve2D::new(
            1,
            vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0)],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
            vec![2, 2],
        );
        let curve2 = NurbsCurve2D::new(
            1,
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 0.0, 0.0)],
            vec![1.0, 1.0],
            vec![0.0, 1.0],
            vec![2, 2],
        );
        let intersections = CurveIntersection2D::nurbs_nurbs(&curve1, &curve2, 0.01, 32);
        assert!(!intersections.is_empty());
    }

    #[test]
    fn test_bezier_bezier_intersection() {
        use crate::geometry::bezier_curve2d::BezierCurve2D;
        let curve1 = BezierCurve2D::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 0.0)]);
        let curve2 = BezierCurve2D::new(vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
        let intersections = CurveIntersection2D::bezier_bezier(&curve1, &curve2, 0.01, 32);
        assert!(!intersections.is_empty());
    }

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
        let circle = Circle2D::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 1.0),
            1.0,
        );

        let intersections = CurveIntersection2D::line_circle(&line, &circle, 0.001);
        assert_eq!(intersections.len(), 2);
    }

    #[test]
    fn test_circle_circle_intersection() {
        let circle1 = Circle2D::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 1.0),
            1.0,
        );
        let circle2 = Circle2D::new(
            Point::new(1.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 1.0),
            1.0,
        );

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
