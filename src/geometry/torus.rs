use crate::foundation::types::StandardReal;
use crate::geometry::{Axis, Direction, Point, Transform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Torus {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
    major_radius: StandardReal,
    minor_radius: StandardReal,
}

impl Torus {
    pub fn new(
        location: Point,
        direction: Direction,
        major_radius: StandardReal,
        minor_radius: StandardReal,
    ) -> Self {
        let x_dir = if direction.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            direction.cross(&Direction::z_axis()).normalized()
        };
        let y_dir = direction.cross(&x_dir).normalized();

        Self {
            location,
            direction,
            x_direction: x_dir,
            y_direction: y_dir,
            major_radius,
            minor_radius,
        }
    }

    pub fn from_axis(major_radius: StandardReal, minor_radius: StandardReal) -> Self {
        Self::new(
            Point::origin(),
            Direction::z_axis(),
            major_radius,
            minor_radius,
        )
    }

    pub fn from_point_axis_radii(
        location: Point,
        direction: Direction,
        major_radius: StandardReal,
        minor_radius: StandardReal,
    ) -> Self {
        Self::new(location, direction, major_radius, minor_radius)
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn x_direction(&self) -> &Direction {
        &self.x_direction
    }

    pub fn y_direction(&self) -> &Direction {
        &self.y_direction
    }

    pub fn major_radius(&self) -> StandardReal {
        self.major_radius
    }

    pub fn minor_radius(&self) -> StandardReal {
        self.minor_radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.update_x_y_directions();
    }

    pub fn set_major_radius(&mut self, major_radius: StandardReal) {
        self.major_radius = major_radius;
    }

    pub fn set_minor_radius(&mut self, minor_radius: StandardReal) {
        self.minor_radius = minor_radius;
    }

    fn update_x_y_directions(&mut self) {
        self.x_direction = if self.direction.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            self.direction.cross(&Direction::z_axis()).normalized()
        };
        self.y_direction = self.direction.cross(&self.x_direction).normalized();
    }

    pub fn axis(&self) -> Axis {
        Axis::new(self.location, self.direction)
    }

    pub fn position(&self, u: StandardReal, v: StandardReal) -> Point {
        let cos_u = u.cos();
        let sin_u = u.sin();
        let cos_v = v.cos();
        let sin_v = v.sin();

        let r = self.major_radius + self.minor_radius * cos_v;

        let x_offset = r * cos_u;
        let y_offset = r * sin_u;
        let z_offset = self.minor_radius * sin_v;

        let x_vec = crate::geometry::Vector::new(
            self.x_direction.x,
            self.x_direction.y,
            self.x_direction.z,
        );
        let y_vec = crate::geometry::Vector::new(
            self.y_direction.x,
            self.y_direction.y,
            self.y_direction.z,
        );
        let dir_vec =
            crate::geometry::Vector::new(self.direction.x, self.direction.y, self.direction.z);

        Point::new(
            self.location.x + x_offset * x_vec.x + y_offset * y_vec.x + z_offset * dir_vec.x,
            self.location.y + x_offset * x_vec.y + y_offset * y_vec.y + z_offset * dir_vec.y,
            self.location.z + x_offset * x_vec.z + y_offset * y_vec.z + z_offset * dir_vec.z,
        )
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        let distance = self.distance(point);
        distance <= tolerance
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let dir_vec =
            crate::geometry::Vector::new(self.direction.x, self.direction.y, self.direction.z);

        let projection = vec.dot(&dir_vec);
        let projected_point = Point::new(
            self.location.x + projection * dir_vec.x,
            self.location.y + projection * dir_vec.y,
            self.location.z + projection * dir_vec.z,
        );

        let center_to_point = crate::geometry::Vector::from_point(&self.location, &projected_point);
        let distance_from_axis = center_to_point.magnitude();

        let expected_distance =
            (self.major_radius - distance_from_axis).powi(2) + projection.powi(2);
        let actual_distance = (point.x - self.location.x).powi(2)
            + (point.y - self.location.y).powi(2)
            + (point.z - self.location.z).powi(2);

        (actual_distance.sqrt() - (expected_distance.sqrt() + self.minor_radius)).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn area(&self) -> StandardReal {
        4.0 * std::f64::consts::PI * std::f64::consts::PI * self.major_radius * self.minor_radius
    }

    pub fn volume(&self) -> StandardReal {
        2.0 * std::f64::consts::PI
            * std::f64::consts::PI
            * self.major_radius
            * self.minor_radius
            * self.minor_radius
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Torus {
        Torus {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
        self.x_direction.mirror_axis(axis);
        self.y_direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Torus {
        Torus {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Torus {
        Torus {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
        self.major_radius *= factor.abs();
        self.minor_radius *= factor.abs();
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Torus {
        Torus {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            major_radius: self.major_radius * factor.abs(),
            minor_radius: self.minor_radius * factor.abs(),
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Torus {
        Torus {
            location: self.location.translated(vec),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn transform(&mut self, trsf: &Transform) {
        self.location = trsf.transforms(&self.location);
        self.direction = trsf.transforms_dir(&self.direction);
        self.x_direction = trsf.transforms_dir(&self.x_direction);
        self.y_direction = trsf.transforms_dir(&self.y_direction);
    }

    pub fn transformed(&self, trsf: &Transform) -> Torus {
        Torus {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn is_closed(&self, _tolerance: StandardReal) -> bool {
        false
    }

    pub fn is_periodic(&self) -> bool {
        true
    }

    pub fn u_periodic(&self) -> bool {
        true
    }

    pub fn v_periodic(&self) -> bool {
        true
    }

    pub fn u_period(&self) -> StandardReal {
        2.0 * std::f64::consts::PI
    }

    pub fn v_period(&self) -> StandardReal {
        2.0 * std::f64::consts::PI
    }
}

impl Default for Torus {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::z_axis(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            major_radius: 10.0,
            minor_radius: 3.0,
        }
    }
}

impl crate::topology::topods_face::Surface for Torus {
    fn value(&self, u: f64, v: f64) -> Point {
        self.position(u, v)
    }

    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector {
        let cos_u = u.cos();
        let sin_u = u.sin();
        let cos_v = v.cos();
        let sin_v = v.sin();

        let x_vec = crate::geometry::Vector::new(
            self.x_direction.x,
            self.x_direction.y,
            self.x_direction.z,
        );
        let y_vec = crate::geometry::Vector::new(
            self.y_direction.x,
            self.y_direction.y,
            self.y_direction.z,
        );
        let dir_vec =
            crate::geometry::Vector::new(self.direction.x, self.direction.y, self.direction.z);

        let r = self.major_radius + self.minor_radius * cos_v;

        let tangent_u = x_vec * (-r * sin_u) + y_vec * (r * cos_u);
        let tangent_v = x_vec * (-self.minor_radius * sin_v * cos_u)
            + y_vec * (-self.minor_radius * sin_v * sin_u)
            + dir_vec * (self.minor_radius * cos_v);

        let normal = tangent_v.cross(&tangent_u).normalized();
        normal
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        (
            (0.0, 2.0 * std::f64::consts::PI),
            (0.0, 2.0 * std::f64::consts::PI),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_creation() {
        let location = Point::origin();
        let direction = Direction::z_axis();
        let major_radius = 10.0;
        let minor_radius = 3.0;
        let torus = Torus::new(location, direction, major_radius, minor_radius);
        assert_eq!(torus.location(), &location);
        assert_eq!(torus.major_radius(), major_radius);
        assert_eq!(torus.minor_radius(), minor_radius);
    }

    #[test]
    fn test_torus_from_axis() {
        let major_radius = 10.0;
        let minor_radius = 3.0;
        let torus = Torus::from_axis(major_radius, minor_radius);
        assert_eq!(torus.location(), &Point::origin());
        assert_eq!(torus.major_radius(), major_radius);
    }

    #[test]
    fn test_torus_position() {
        let torus = Torus::new(Point::origin(), Direction::z_axis(), 10.0, 3.0);
        let point = torus.position(0.0, 0.0);
        assert!((point.x - 13.0).abs() < 0.001);
        assert!((point.y - 0.0).abs() < 0.001);
        assert!((point.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_torus_area() {
        let torus = Torus::new(Point::origin(), Direction::z_axis(), 10.0, 3.0);
        let area = torus.area();
        assert!(area > 0.0);
    }

    #[test]
    fn test_torus_volume() {
        let torus = Torus::new(Point::origin(), Direction::z_axis(), 10.0, 3.0);
        let volume = torus.volume();
        assert!(volume > 0.0);
    }
}
