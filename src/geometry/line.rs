use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Axis, Direction, Point, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Line {
    location: Point,
    direction: Direction,
    /// Optional length for finite line segments
    /// If None, the line is infinite
    length: Option<StandardReal>,
}

impl Line {
    pub fn new(location: Point, direction: Direction) -> Self {
        Self {
            location,
            direction,
            length: None,
        }
    }

    /// Create a finite line segment from two points
    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        let dir = Vector::from_point(p1, p2).to_dir();
        let length = p1.distance(p2);
        Self {
            location: *p1,
            direction: dir,
            length: Some(length),
        }
    }

    pub fn from_axis(axis: &Axis) -> Self {
        Self {
            location: *axis.location(),
            direction: *axis.direction(),
            length: None,
        }
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn length(&self) -> Option<StandardReal> {
        self.length
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn reverse(&mut self) {
        self.direction.reverse();
    }

    pub fn reversed(&self) -> Line {
        Line {
            location: self.location,
            direction: self.direction.reversed(),
            length: self.length,
        }
    }

    pub fn angle(&self, other: &Line) -> StandardReal {
        self.direction.angle(&other.direction)
    }

    pub fn angle_cos(&self, other: &Line) -> StandardReal {
        self.direction.dot(&other.direction)
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        let vec = Vector::from_point(&self.location, point);
        let dir_vec = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let cross_vec = dir_vec.cross(&vec);
        cross_vec.magnitude() <= tolerance
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = Vector::from_point(&self.location, point);
        let dir_vec = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let cross_vec = dir_vec.cross(&vec);
        cross_vec.magnitude()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn distance_line(&self, other: &Line) -> StandardReal {
        if self.is_parallel(other, STANDARD_REAL_EPSILON) {
            return self.distance(&other.location);
        }

        let _p1 = &self.location;
        let d1 = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let _p2 = &other.location;
        let d2 = Vector::new(other.direction.x, other.direction.y, other.direction.z);

        let cross_d1_d2 = d1.cross(&d2);

        cross_d1_d2.magnitude()
    }

    pub fn square_distance_line(&self, other: &Line) -> StandardReal {
        let dist = self.distance_line(other);
        dist * dist
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Line {
        Line {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
            length: self.length,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Line {
        Line {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            length: self.length,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Line {
        Line {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            length: self.length,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
        if let Some(ref mut len) = self.length {
            *len *= factor;
        }
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Line {
        let mut new_line = Line {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            length: self.length,
        };
        if let Some(len) = new_line.length {
            new_line.length = Some(len * factor);
        }
        new_line
    }

    pub fn translate(&mut self, vec: &Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &Vector) -> Line {
        Line {
            location: self.location.translated(vec),
            direction: self.direction,
            length: self.length,
        }
    }

    pub fn transform(&mut self, trsf: &crate::geometry::Transform) {
        self.location = trsf.transforms(&self.location);
        self.direction = trsf.transforms_dir(&self.direction);
    }

    pub fn transformed(&self, trsf: &crate::geometry::Transform) -> Line {
        Line {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
            length: self.length,
        }
    }

    pub fn is_parallel(&self, other: &Line, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_parallel(&other.direction, angular_tolerance)
    }

    pub fn is_normal(&self, other: &Line, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_normal(&other.direction, angular_tolerance)
    }

    pub fn is_opposite(&self, other: &Line, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_opposite(&other.direction, angular_tolerance)
    }

    pub fn intersect(&self, other: &Line, tolerance: StandardReal) -> Option<Point> {
        if self.is_parallel(other, tolerance) {
            return None;
        }

        let p1 = &self.location;
        let d1 = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let p2 = &other.location;
        let d2 = Vector::new(other.direction.x, other.direction.y, other.direction.z);

        // Solve: p1 + t1 * d1 = p2 + t2 * d2
        // This gives us: t1 * d1 - t2 * d2 = p2 - p1

        let r = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);

        // Cross product d1 x d2
        let cross_d1_d2 = d1.cross(&d2);
        let cross_mag_sq = cross_d1_d2.dot(&cross_d1_d2);

        if cross_mag_sq < tolerance * tolerance {
            return None; // Lines are parallel
        }

        // Calculate t1 using triple product
        let t1 = r.cross(&d2).dot(&cross_d1_d2) / cross_mag_sq;

        Some(Point::new(
            p1.x + t1 * d1.x,
            p1.y + t1 * d1.y,
            p1.z + t1 * d1.z,
        ))
    }

    pub fn to_axis(&self) -> Axis {
        Axis::new(self.location, self.direction)
    }

    pub fn position(&self, parameter: StandardReal) -> Point {
        let dir_vec = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        Point::new(
            self.location.x + parameter * dir_vec.x,
            self.location.y + parameter * dir_vec.y,
            self.location.z + parameter * dir_vec.z,
        )
    }

    pub fn location_at(&self, parameter: StandardReal) -> Point {
        self.position(parameter)
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::x_axis(),
            length: None,
        }
    }
}

impl crate::topology::Curve for Line {
    fn value(&self, parameter: f64) -> Point {
        self.position(parameter)
    }

    fn derivative(&self, _parameter: f64) -> Vector {
        Vector::new(self.direction.x, self.direction.y, self.direction.z)
    }

    fn parameter_range(&self) -> (f64, f64) {
        match self.length {
            Some(len) => (0.0, len),
            None => (f64::NEG_INFINITY, f64::INFINITY),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_creation() {
        let location = Point::origin();
        let direction = Direction::x_axis();
        let line = Line::new(location, direction);
        assert_eq!(line.location(), &location);
        assert_eq!(line.direction(), &direction);
    }

    #[test]
    fn test_line_from_points() {
        let p1 = Point::origin();
        let p2 = Point::new(1.0, 0.0, 0.0);
        let line = Line::from_points(&p1, &p2);
        assert!(line.direction.is_equal(&Direction::x_axis(), 0.001));
        assert_eq!(line.length, Some(1.0));
    }

    #[test]
    fn test_line_reverse() {
        let mut line = Line::default();
        line.reverse();
        assert!(line.direction.is_opposite(&Direction::x_axis(), 0.001));
    }

    #[test]
    fn test_line_distance() {
        let line = Line::new(Point::origin(), Direction::x_axis());
        let point = Point::new(0.0, 1.0, 0.0);
        let distance = line.distance(&point);
        assert!((distance - 1.0).abs() < 0.001);
    }
}
