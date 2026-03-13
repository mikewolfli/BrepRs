use crate::foundation::types::StandardReal;
use crate::geometry::{Axis, Direction, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D {
    location: Point,
    direction: Direction,
}

impl Line2D {
    pub fn new(location: Point, direction: Direction) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        let dir = Vector::from_point(p1, p2).to_dir();
        Self {
            location: *p1,
            direction: dir,
        }
    }

    pub fn from_axis(axis: &Axis) -> Self {
        Self {
            location: *axis.location(),
            direction: *axis.direction(),
        }
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
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

    pub fn reversed(&self) -> Line2D {
        Line2D {
            location: self.location,
            direction: self.direction.reversed(),
        }
    }

    pub fn angle(&self, other: &Line2D) -> StandardReal {
        self.direction.angle(&other.direction)
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

    pub fn normal(&self) -> Direction {
        let dir_vec = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let normal_vec = dir_vec.cross(&Vector::new(0.0, 0.0, 1.0));
        normal_vec.to_dir()
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Line2D {
        Line2D {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Line2D {
        Line2D {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Line2D {
        Line2D {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Line2D {
        Line2D {
            location: self.location.scaled(point, factor),
            direction: self.direction,
        }
    }

    pub fn translate(&mut self, vec: &Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &Vector) -> Line2D {
        Line2D {
            location: self.location.translated(vec),
            direction: self.direction,
        }
    }

    pub fn transform(&mut self, trsf: &crate::geometry::Transform) {
        self.location = trsf.transforms(&self.location);
        self.direction = trsf.transforms_dir(&self.direction);
    }

    pub fn transformed(&self, trsf: &crate::geometry::Transform) -> Line2D {
        Line2D {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
        }
    }

    pub fn is_parallel(&self, other: &Line2D, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_parallel(&other.direction, angular_tolerance)
    }

    pub fn is_normal(&self, other: &Line2D, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_normal(&other.direction, angular_tolerance)
    }

    pub fn is_opposite(&self, other: &Line2D, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_opposite(&other.direction, angular_tolerance)
    }

    pub fn intersect(&self, other: &Line2D, tolerance: StandardReal) -> Option<Point> {
        if self.is_parallel(other, tolerance) {
            return None;
        }

        let p1 = &self.location;
        let d1 = Vector::new(self.direction.x, self.direction.y, self.direction.z);
        let p2 = &other.location;
        let d2 = Vector::new(other.direction.x, other.direction.y, other.direction.z);

        let p1_p2 = *p2 - *p1;
        let cross_d1_d2 = d1.cross(&d2);
        let cross_p1p2_d2 = p1_p2.cross(&d2);

        let t = cross_p1p2_d2.magnitude() / cross_d1_d2.magnitude();

        let sign1 = d1.cross(&p1_p2).dot(&cross_d1_d2);
        let sign2 = cross_d1_d2.dot(&cross_d1_d2);

        let t = if sign1 * sign2 < 0.0 { -t } else { t };

        Some(Point::new(
            p1.x + t * d1.x,
            p1.y + t * d1.y,
            p1.z + t * d1.z,
        ))
    }

    pub fn to_axis(&self) -> Axis {
        Axis::new(self.location, self.direction)
    }

    pub fn to_line3d(&self) -> crate::geometry::Line {
        crate::geometry::Line::new(self.location, self.direction)
    }
}

impl Default for Line2D {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::x_axis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line2d_creation() {
        let location = Point::origin();
        let direction = Direction::x_axis();
        let line = Line2D::new(location, direction);
        assert_eq!(line.location(), &location);
        assert_eq!(line.direction(), &direction);
    }

    #[test]
    fn test_line2d_from_points() {
        let p1 = Point::origin();
        let p2 = Point::new(1.0, 0.0, 0.0);
        let line = Line2D::from_points(&p1, &p2);
        assert!(line.direction.is_equal(&Direction::x_axis(), 0.001));
    }

    #[test]
    fn test_line2d_reverse() {
        let mut line = Line2D::default();
        line.reverse();
        assert!(line.direction.is_opposite(&Direction::x_axis(), 0.001));
    }

    #[test]
    fn test_line2d_distance() {
        let line = Line2D::new(Point::origin(), Direction::x_axis());
        let point = Point::new(0.0, 1.0, 0.0);
        let distance = line.distance(&point);
        assert!((distance - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_line2d_contains() {
        let line = Line2D::new(Point::origin(), Direction::x_axis());
        let point = Point::new(1.0, 0.0, 0.0);
        assert!(line.contains(&point, 0.001));
    }

    #[test]
    fn test_line2d_is_parallel() {
        let line1 = Line2D::new(Point::origin(), Direction::x_axis());
        let line2 = Line2D::new(Point::new(0.0, 1.0, 0.0), Direction::x_axis());
        assert!(line1.is_parallel(&line2, 0.001));
    }

    #[test]
    fn test_line2d_is_normal() {
        let line1 = Line2D::new(Point::origin(), Direction::x_axis());
        let line2 = Line2D::new(Point::origin(), Direction::y_axis());
        assert!(line1.is_normal(&line2, 0.001));
    }

    #[test]
    fn test_line2d_angle() {
        let line1 = Line2D::new(Point::origin(), Direction::x_axis());
        let line2 = Line2D::new(Point::origin(), Direction::y_axis());
        let angle = line1.angle(&line2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_line2d_translate() {
        let mut line = Line2D::default();
        let vec = Vector::new(1.0, 2.0, 3.0);
        line.translate(&vec);
        assert_eq!(line.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_line2d_scale() {
        let mut line = Line2D::new(Point::new(1.0, 0.0, 0.0), Direction::x_axis());
        let point = Point::origin();
        line.scale(&point, 2.0);
        assert_eq!(line.location(), &Point::new(2.0, 0.0, 0.0));
    }

    #[test]
    fn test_line2d_rotate() {
        let mut line = Line2D::new(Point::origin(), Direction::x_axis());
        let axis = Axis::z_axis();
        line.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!(line.direction.is_equal(&Direction::y_axis(), 0.001));
    }

    #[test]
    fn test_line2d_to_axis() {
        let line = Line2D::default();
        let axis = line.to_axis();
        assert_eq!(axis.location(), &Point::origin());
        assert_eq!(axis.direction(), &Direction::x_axis());
    }
}
