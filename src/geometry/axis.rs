use crate::geometry::traits::{GetCoord, SetCoord};
impl GetCoord for Axis {
    fn coord(&self) -> (f64, f64, f64) {
        self.location.coord()
    }
}

impl SetCoord for Axis {
    fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.location.set_coord(x, y, z);
    }
}
use crate::foundation::types::StandardReal;
use crate::geometry::{Direction, Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axis {
    pub location: Point,
    pub direction: Direction,
}

impl Axis {
    pub fn new(location: Point, direction: Direction) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn from_point_and_direction(location: &Point, direction: &Direction) -> Self {
        Self {
            location: *location,
            direction: *direction,
        }
    }

    pub fn origin(direction: Direction) -> Self {
        Self {
            location: Point::origin(),
            direction,
        }
    }

    pub fn x_axis() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::x_axis(),
        }
    }

    pub fn y_axis() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::y_axis(),
        }
    }

    pub fn z_axis() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::z_axis(),
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

    pub fn reversed(&self) -> Axis {
        Axis {
            location: self.location,
            direction: self.direction.reversed(),
        }
    }

    pub fn is_co_linear(
        &self,
        other: &Axis,
        angular_tolerance: StandardReal,
        linear_tolerance: StandardReal,
    ) -> bool {
        self.direction
            .is_co_linear(&other.direction, angular_tolerance)
            && self.location.distance(&other.location) <= linear_tolerance
    }

    pub fn is_normal(&self, other: &Axis, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_normal(&other.direction, angular_tolerance)
    }

    pub fn is_opposite(&self, other: &Axis, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_opposite(&other.direction, angular_tolerance)
    }

    pub fn is_parallel(&self, other: &Axis, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_parallel(&other.direction, angular_tolerance)
    }

    pub fn angle(&self, other: &Axis) -> StandardReal {
        self.direction.angle(&other.direction)
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Axis {
        Axis {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Axis {
        Axis {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Axis {
        Axis {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Axis {
        Axis {
            location: self.location.scaled(point, factor),
            direction: self.direction,
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Axis {
        Axis {
            location: self.location.translated(vec),
            direction: self.direction,
        }
    }
}

impl Default for Axis {
    fn default() -> Self {
        Self::x_axis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axis_creation() {
        let location = Point::new(1.0, 2.0, 3.0);
        let direction = Direction::x_axis();
        let axis = Axis::new(location, direction);
        assert_eq!(axis.location(), &location);
        assert_eq!(axis.direction(), &direction);
    }

    #[test]
    fn test_axis_origin() {
        let axis = Axis::origin(Direction::x_axis());
        assert_eq!(axis.location(), &Point::origin());
    }

    #[test]
    fn test_axis_axes() {
        let x_axis = Axis::x_axis();
        let y_axis = Axis::y_axis();
        let z_axis = Axis::z_axis();

        assert_eq!(x_axis.location(), &Point::origin());
        assert_eq!(y_axis.location(), &Point::origin());
        assert_eq!(z_axis.location(), &Point::origin());
    }

    #[test]
    fn test_axis_reverse() {
        let mut axis = Axis::x_axis();
        axis.reverse();
        assert!(axis.direction.is_opposite(&Direction::x_axis(), 0.001));
    }

    #[test]
    fn test_axis_is_parallel() {
        let axis1 = Axis::x_axis();
        let axis2 = Axis::origin(Direction::new(1.0, 0.0, 0.0));
        assert!(axis1.is_parallel(&axis2, 0.001));
    }

    #[test]
    fn test_axis_is_normal() {
        let axis1 = Axis::x_axis();
        let axis2 = Axis::y_axis();
        assert!(axis1.is_normal(&axis2, 0.001));
    }

    #[test]
    fn test_axis_angle() {
        let axis1 = Axis::x_axis();
        let axis2 = Axis::y_axis();
        let angle = axis1.angle(&axis2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_axis_translate() {
        let mut axis = Axis::x_axis();
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        axis.translate(&vec);
        assert_eq!(axis.location(), &Point::new(1.0, 2.0, 3.0));
    }
}
