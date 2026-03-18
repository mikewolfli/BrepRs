use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Direction, Point, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineSegment {
    start: Point,
    end: Point,
}

impl LineSegment {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &Point {
        &self.start
    }

    pub fn end(&self) -> &Point {
        &self.end
    }

    pub fn set_start(&mut self, start: Point) {
        self.start = start;
    }

    pub fn set_end(&mut self, end: Point) {
        self.end = end;
    }

    pub fn length(&self) -> StandardReal {
        self.start.distance(&self.end)
    }

    pub fn direction(&self) -> Direction {
        Vector::from_point(&self.start, &self.end).to_dir()
    }

    pub fn midpoint(&self) -> Point {
        self.start.barycenter(&self.end, 0.5)
    }

    pub fn reverse(&mut self) {
        std::mem::swap(&mut self.start, &mut self.end);
    }

    pub fn reversed(&self) -> LineSegment {
        LineSegment::new(self.end, self.start)
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        // Check if point is on the line
        let line_vec = Vector::from_point(&self.start, &self.end);
        let point_vec = Vector::from_point(&self.start, point);
        let cross_vec = line_vec.cross(&point_vec);

        if cross_vec.magnitude() > tolerance {
            return false;
        }

        // Check if point is within the segment
        let dot_product = line_vec.dot(&point_vec);
        if dot_product < -tolerance {
            return false;
        }

        if dot_product > line_vec.magnitude() * line_vec.magnitude() + tolerance {
            return false;
        }

        true
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let line_vec = Vector::from_point(&self.start, &self.end);
        let point_vec = Vector::from_point(&self.start, point);

        let line_length = line_vec.magnitude();
        let line_length_squared = line_length * line_length;
        if line_length_squared < STANDARD_REAL_EPSILON {
            return self.start.distance(point);
        }

        let t = line_vec.dot(&point_vec) / line_length_squared;
        let t_clamped = t.max(0.0).min(1.0);

        let closest_point = self.start + line_vec * t_clamped;
        closest_point.distance(point)
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn closest_point(&self, point: &Point) -> Point {
        let line_vec = Vector::from_point(&self.start, &self.end);
        let point_vec = Vector::from_point(&self.start, point);

        let line_length = line_vec.magnitude();
        let line_length_squared = line_length * line_length;
        if line_length_squared < STANDARD_REAL_EPSILON {
            return self.start;
        }

        let t = line_vec.dot(&point_vec) / line_length_squared;
        let t_clamped = t.max(0.0).min(1.0);

        self.start + line_vec * t_clamped
    }

    pub fn intersects(&self, other: &LineSegment, tolerance: StandardReal) -> bool {
        self.intersection(other, tolerance).is_some()
    }

    pub fn intersection(&self, other: &LineSegment, tolerance: StandardReal) -> Option<Point> {
        let p1 = &self.start;
        let p2 = &self.end;
        let p3 = &other.start;
        let p4 = &other.end;

        let d1 = Vector::from_point(p1, p2);
        let d2 = Vector::from_point(p3, p4);

        let denom = d1.x * d2.y - d1.y * d2.x;
        if denom.abs() < STANDARD_REAL_EPSILON {
            return None; // Lines are parallel
        }

        let dx = p3.x - p1.x;
        let dy = p3.y - p1.y;

        let t = (dx * d2.y - dy * d2.x) / denom;
        let u = (dx * d1.y - dy * d1.x) / denom;

        if t >= -tolerance && t <= 1.0 + tolerance && u >= -tolerance && u <= 1.0 + tolerance {
            let intersection = Point::new(
                p1.x + d1.x * t,
                p1.y + d1.y * t,
                p1.z + d1.z * t
            );
            Some(intersection)
        } else {
            None
        }
    }

    pub fn offset(&self, distance: StandardReal) -> (LineSegment, LineSegment) {
        let line_vec = Vector::from_point(&self.start, &self.end);
        let normal = line_vec.cross(&Vector::new(0.0, 0.0, 1.0)).normalized();

        let offset_vec = Vector::new(
            normal.x * distance,
            normal.y * distance,
            normal.z * distance,
        );

        let offset_start1 = Point::new(
            self.start.x + offset_vec.x,
            self.start.y + offset_vec.y,
            self.start.z + offset_vec.z,
        );
        let offset_end1 = Point::new(
            self.end.x + offset_vec.x,
            self.end.y + offset_vec.y,
            self.end.z + offset_vec.z,
        );

        let offset_start2 = Point::new(
            self.start.x - offset_vec.x,
            self.start.y - offset_vec.y,
            self.start.z - offset_vec.z,
        );
        let offset_end2 = Point::new(
            self.end.x - offset_vec.x,
            self.end.y - offset_vec.y,
            self.end.z - offset_vec.z,
        );

        (
            LineSegment::new(offset_start1, offset_end1),
            LineSegment::new(offset_start2, offset_end2),
        )
    }

    pub fn project(&self, point: &Point) -> Point {
        self.closest_point(point)
    }

    pub fn angle(&self, other: &LineSegment) -> StandardReal {
        let dir1 = self.direction();
        let dir2 = other.direction();
        dir1.angle(&dir2)
    }

    pub fn angle_cos(&self, other: &LineSegment) -> StandardReal {
        let dir1 = self.direction();
        let dir2 = other.direction();
        dir1.dot(&dir2)
    }

    pub fn to_line(&self) -> crate::geometry::Line {
        crate::geometry::Line::from_points(&self.start, &self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_segment_creation() {
        let start = Point::new(0.0, 0.0, 0.0);
        let end = Point::new(1.0, 1.0, 0.0);
        let segment = LineSegment::new(start, end);

        assert_eq!(*segment.start(), start);
        assert_eq!(*segment.end(), end);
        assert!((segment.length() - 2.0_f64.sqrt()).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_line_segment_contains() {
        let start = Point::new(0.0, 0.0, 0.0);
        let end = Point::new(2.0, 2.0, 0.0);
        let segment = LineSegment::new(start, end);

        let point1 = Point::new(1.0, 1.0, 0.0); // Midpoint
        let point2 = Point::new(0.0, 0.0, 0.0); // Start
        let point3 = Point::new(2.0, 2.0, 0.0); // End
        let point4 = Point::new(3.0, 3.0, 0.0); // Outside

        assert!(segment.contains(&point1, 0.001));
        assert!(segment.contains(&point2, 0.001));
        assert!(segment.contains(&point3, 0.001));
        assert!(!segment.contains(&point4, 0.001));
    }

    #[test]
    fn test_line_segment_distance() {
        let start = Point::new(0.0, 0.0, 0.0);
        let end = Point::new(2.0, 0.0, 0.0);
        let segment = LineSegment::new(start, end);

        let point1 = Point::new(1.0, 1.0, 0.0); // Perpendicular distance
        let point2 = Point::new(3.0, 0.0, 0.0); // Beyond end
        let point3 = Point::new(-1.0, 0.0, 0.0); // Before start

        assert!((segment.distance(&point1) - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((segment.distance(&point2) - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((segment.distance(&point3) - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_line_segment_intersection() {
        let segment1 = LineSegment::new(Point::new(0.0, 0.0, 0.0), Point::new(2.0, 2.0, 0.0));
        let segment2 = LineSegment::new(Point::new(0.0, 2.0, 0.0), Point::new(2.0, 0.0, 0.0));

        let intersection = segment1.intersection(&segment2, 0.001);
        assert!(intersection.is_some());
        let point = intersection.unwrap();
        assert!((point.x - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((point.y - 1.0).abs() < STANDARD_REAL_EPSILON);
    }
}
