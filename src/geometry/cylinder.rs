use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Axis, Direction, Point, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
    radius: StandardReal,
}

impl Cylinder {
    pub fn new(location: Point, direction: Direction, radius: StandardReal) -> Self {
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
            radius,
        }
    }

    pub fn from_axis(axis: &Axis, radius: StandardReal) -> Self {
        let main_dir = axis.direction();
        let x_dir = if main_dir.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            main_dir.cross(&Direction::z_axis()).normalized()
        };
        let y_dir = main_dir.cross(&x_dir).normalized();

        Self {
            location: *axis.location(),
            direction: *main_dir,
            x_direction: x_dir,
            y_direction: y_dir,
            radius,
        }
    }

    pub fn from_point_axis_radius(
        location: Point,
        direction: Direction,
        radius: StandardReal,
    ) -> Self {
        Self::new(location, direction, radius)
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

    pub fn radius(&self) -> StandardReal {
        self.radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.update_x_y_directions();
    }

    pub fn set_radius(&mut self, radius: StandardReal) {
        self.radius = radius;
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

        let x_offset = self.radius * cos_u;
        let y_offset = self.radius * sin_u;

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
            self.location.x + x_offset * x_vec.x + y_offset * y_vec.x + v * dir_vec.x,
            self.location.y + x_offset * x_vec.y + y_offset * y_vec.y + v * dir_vec.y,
            self.location.z + x_offset * x_vec.z + y_offset * y_vec.z + v * dir_vec.z,
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

        let distance_to_axis =
            crate::geometry::Vector::from_point(&projected_point, point).magnitude();
        (distance_to_axis - self.radius).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn area(&self, height: StandardReal) -> StandardReal {
        2.0 * std::f64::consts::PI * self.radius * height
    }

    pub fn volume(&self, height: StandardReal) -> StandardReal {
        std::f64::consts::PI * self.radius * self.radius * height
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Cylinder {
        Cylinder {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
            radius: self.radius,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
        self.x_direction.mirror_axis(axis);
        self.y_direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Cylinder {
        Cylinder {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
            radius: self.radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Cylinder {
        Cylinder {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
            radius: self.radius,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
        self.radius *= factor.abs();
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Cylinder {
        Cylinder {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            radius: self.radius * factor.abs(),
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Cylinder {
        Cylinder {
            location: self.location.translated(vec),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            radius: self.radius,
        }
    }

    pub fn transform(&mut self, trsf: &Transform) {
        self.location = trsf.transforms(&self.location);
        self.direction = trsf.transforms_dir(&self.direction);
        self.x_direction = trsf.transforms_dir(&self.x_direction);
        self.y_direction = trsf.transforms_dir(&self.y_direction);
        self.radius *= trsf.scale.abs();
    }

    pub fn transformed(&self, trsf: &Transform) -> Cylinder {
        Cylinder {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
            radius: self.radius * trsf.scale.abs(),
        }
    }

    pub fn is_closed(&self, tolerance: StandardReal) -> bool {
        self.radius <= tolerance
    }

    pub fn is_periodic(&self) -> bool {
        true
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::z_axis(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            radius: 1.0,
        }
    }
}

impl crate::topology::topods_face::Surface for Cylinder {
    fn value(&self, u: f64, v: f64) -> Point {
        self.position(u, v)
    }

    fn normal(&self, u: f64, _v: f64) -> crate::geometry::Vector {
        let cos_u = u.cos();
        let sin_u = u.sin();

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

        let normal = x_vec * cos_u + y_vec * sin_u;
        normal.normalized()
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        (
            (0.0, 2.0 * std::f64::consts::PI),
            (-f64::INFINITY, f64::INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_creation() {
        let location = Point::origin();
        let direction = Direction::z_axis();
        let radius = 5.0;
        let cylinder = Cylinder::new(location, direction, radius);
        assert_eq!(cylinder.location(), &location);
        assert_eq!(cylinder.radius(), radius);
    }

    #[test]
    fn test_cylinder_from_axis() {
        let axis = Axis::z_axis();
        let radius = 5.0;
        let cylinder = Cylinder::from_axis(&axis, radius);
        assert_eq!(cylinder.location(), &Point::origin());
        assert_eq!(cylinder.radius(), radius);
    }

    #[test]
    fn test_cylinder_position() {
        let cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let point = cylinder.position(0.0, 0.0);
        assert_eq!(point.x, 5.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }

    #[test]
    fn test_cylinder_distance() {
        let cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let point = Point::new(5.0, 0.0, 0.0);
        let distance = cylinder.distance(&point);
        assert!((distance - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_cylinder_area() {
        let cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let area = cylinder.area(10.0);
        assert!((area - 100.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_cylinder_volume() {
        let cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let volume = cylinder.volume(10.0);
        assert!((volume - 250.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_cylinder_translate() {
        let mut cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        cylinder.translate(&vec);
        assert_eq!(cylinder.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_cylinder_scale() {
        let mut cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let point = Point::origin();
        cylinder.scale(&point, 2.0);
        assert_eq!(cylinder.radius(), 10.0);
    }

    #[test]
    fn test_cylinder_rotate() {
        let mut cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let axis = Axis::z_axis();
        cylinder.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!(cylinder.direction.is_equal(&Direction::z_axis(), 0.001));
    }

    #[test]
    fn test_cylinder_axis() {
        let cylinder = Cylinder::new(Point::origin(), Direction::z_axis(), 5.0);
        let axis = cylinder.axis();
        assert_eq!(axis.location(), &Point::origin());
        assert_eq!(axis.direction(), &Direction::z_axis());
    }
}
