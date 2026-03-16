use crate::geometry::traits::{GetCoord, SetCoord};
impl GetCoord for Plane {
    fn coord(&self) -> (f64, f64, f64) {
        self.location.coord()
    }
}

impl SetCoord for Plane {
    fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.location.set_coord(x, y, z);
    }
}
impl Plane {
    /// Create a plane from three points
    pub fn from_points(p1: Point, p2: Point, p3: Point) -> Option<Self> {
        let v1 = crate::geometry::Vector::from_point(&p1, &p2);
        let v2 = crate::geometry::Vector::from_point(&p1, &p3);
        let normal = v1.cross(&v2);
        if normal.magnitude() < 1e-8 {
            return None; // Collinear points
        }
        let direction = crate::geometry::Direction::from_vector(&normal);
        let x_direction = crate::geometry::Direction::from_vector(&v1);
        Some(Plane::new(p1, direction, x_direction))
    }

    /// Get the origin point of the plane
    pub fn origin(&self) -> &Point {
        &self.location
    }
}
use crate::foundation::types::StandardReal;
use crate::geometry::{Axis, CoordinateSystem, Direction, Point, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plane {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
}

impl Plane {
    pub fn new(location: Point, direction: Direction, x_direction: Direction) -> Self {
        let y_direction = direction.cross(&x_direction).normalized();
        Self {
            location,
            direction,
            x_direction,
            y_direction,
        }
    }

    pub fn from_point_normal(location: Point, normal: Direction) -> Self {
        let x_dir = if normal.is_parallel(&Direction::z_axis(), 0.001) {
            Direction::x_axis()
        } else {
            normal.cross(&Direction::z_axis()).normalized()
        };
        let y_dir = normal.cross(&x_dir).normalized();

        Self {
            location,
            direction: normal,
            x_direction: x_dir,
            y_direction: y_dir,
        }
    }

    pub fn from_coordinate_system(cs: &CoordinateSystem) -> Self {
        Self {
            location: *cs.location(),
            direction: *cs.direction(),
            x_direction: *cs.x_direction(),
            y_direction: *cs.y_direction(),
        }
    }

    pub fn from_axis(axis: &Axis) -> Self {
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
        }
    }

    pub fn from_ax2(ax2: &crate::geometry::Ax2) -> Self {
        Self {
            location: *ax2.location(),
            direction: *ax2.direction(),
            x_direction: *ax2.x_direction(),
            y_direction: *ax2.y_direction(),
        }
    }

    pub fn xy_plane() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::z_axis(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
        }
    }

    pub fn yz_plane() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::x_axis(),
            x_direction: Direction::y_axis(),
            y_direction: Direction::z_axis(),
        }
    }

    pub fn zx_plane() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::y_axis(),
            x_direction: Direction::z_axis(),
            y_direction: Direction::x_axis(),
        }
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

    pub fn position(&self) -> &Point {
        &self.location
    }

    pub fn axis(&self) -> Axis {
        Axis::new(self.location, self.direction)
    }

    pub fn x_axis(&self) -> Axis {
        Axis::new(self.location, self.x_direction)
    }

    pub fn y_axis(&self) -> Axis {
        Axis::new(self.location, self.y_direction)
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
        self.update_y_direction();
    }

    pub fn set_x_direction(&mut self, x_direction: Direction) {
        self.x_direction = x_direction;
        self.update_y_direction();
    }

    pub fn set_position(&mut self, position: Point) {
        self.location = position;
    }

    pub fn set_axis(&mut self, axis: &Axis) {
        self.location = *axis.location();
        self.direction = *axis.direction();
        self.update_x_direction();
        self.update_y_direction();
    }

    pub fn set_axes(&mut self, main_axis: &Axis, x_axis: &Axis) {
        self.location = *main_axis.location();
        self.direction = *main_axis.direction();
        self.x_direction = *x_axis.direction();
        self.update_y_direction();
    }

    fn update_y_direction(&mut self) {
        self.y_direction = self.direction.cross(&self.x_direction).normalized();
    }

    fn update_x_direction(&mut self) {
        if self.direction.is_parallel(&Direction::z_axis(), 0.001) {
            self.x_direction = Direction::x_axis();
        } else {
            self.x_direction = self.direction.cross(&Direction::z_axis()).normalized();
        }
    }

    pub fn reverse(&mut self) {
        self.direction.reverse();
    }

    pub fn reversed(&self) -> Plane {
        Plane {
            location: self.location,
            direction: self.direction.reversed(),
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    pub fn angle(&self, other: &Plane) -> StandardReal {
        self.direction.angle(&other.direction)
    }

    pub fn normal(&self) -> Direction {
        self.direction
    }

    pub fn is_coaxial(
        &self,
        other: &Plane,
        angular_tolerance: StandardReal,
        linear_tolerance: StandardReal,
    ) -> bool {
        self.direction
            .is_co_linear(&other.direction, angular_tolerance)
            && self.location.distance(&other.location) <= linear_tolerance
    }

    pub fn is_opposite(&self, other: &Plane, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_opposite(&other.direction, angular_tolerance)
    }

    pub fn is_parallel(&self, other: &Plane, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_parallel(&other.direction, angular_tolerance)
    }

    pub fn is_normal(&self, other: &Plane, angular_tolerance: StandardReal) -> bool {
        self.direction
            .is_normal(&other.direction, angular_tolerance)
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let normal =
            crate::geometry::Vector::new(self.direction.x, self.direction.y, self.direction.z);
        normal.dot(&vec).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        self.distance(point) <= tolerance
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Plane {
        Plane {
            location: self.location.mirrored(point),
            direction: self.direction.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.direction.mirror_axis(axis);
        self.x_direction.mirror_axis(axis);
        self.y_direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Plane {
        Plane {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
        }
    }

    pub fn mirror_plane(&mut self, plane: &Plane) {
        self.location.mirror_axis(&plane.axis());
        self.direction.mirror_axis(&plane.axis());
        self.x_direction.mirror_axis(&plane.axis());
        self.y_direction.mirror_axis(&plane.axis());
    }

    pub fn mirrored_plane(&self, plane: &Plane) -> Plane {
        Plane {
            location: self.location.mirrored_axis(&plane.axis()),
            direction: self.direction.mirrored_axis(&plane.axis()),
            x_direction: self.x_direction.mirrored_axis(&plane.axis()),
            y_direction: self.y_direction.mirrored_axis(&plane.axis()),
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Plane {
        Plane {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Plane {
        Plane {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Plane {
        Plane {
            location: self.location.translated(vec),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    pub fn transform(&mut self, trsf: &Transform) {
        self.location = trsf.transforms(&self.location);
        self.direction = trsf.transforms_dir(&self.direction);
        self.x_direction = trsf.transforms_dir(&self.x_direction);
        self.y_direction = trsf.transforms_dir(&self.y_direction);
    }

    pub fn transformed(&self, trsf: &Transform) -> Plane {
        Plane {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
        }
    }

    pub fn to_coordinate_system(&self) -> CoordinateSystem {
        CoordinateSystem::new(self.location, self.direction, self.x_direction)
    }

    pub fn to_ax2(&self) -> crate::geometry::Ax2 {
        crate::geometry::Ax2::new(self.location, self.direction, self.x_direction)
    }

    pub fn is_direct(&self) -> bool {
        let cross = self.x_direction.cross(&self.y_direction);
        cross.dot(&self.direction) > 0.0
    }

    pub fn is_right_handed(&self) -> bool {
        self.is_direct()
    }

    pub fn is_left_handed(&self) -> bool {
        !self.is_direct()
    }

    /// Compute signed distance from point to plane
    pub fn signed_distance_to(&self, point: &Point) -> f64 {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let normal =
            crate::geometry::Vector::new(self.direction.x, self.direction.y, self.direction.z);
        normal.dot(&vec)
    }

    /// Project a point onto the plane
    pub fn project_point(&self, point: &Point) -> Point {
        let distance = self.signed_distance_to(point);
        let normal_vec = crate::geometry::Vector::new(
            self.direction.x * distance,
            self.direction.y * distance,
            self.direction.z * distance,
        );
        point.translated(&normal_vec.negated())
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::xy_plane()
    }
}

impl crate::topology::topods_face::Surface for Plane {
    fn value(&self, u: f64, v: f64) -> Point {
        let x_vec = self.x_direction.to_vec() * u;
        let y_vec = self.y_direction.to_vec() * v;
        self.location + x_vec + y_vec
    }

    fn normal(&self, _u: f64, _v: f64) -> crate::geometry::Vector {
        self.direction.to_vec()
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        (
            (-f64::INFINITY, f64::INFINITY),
            (-f64::INFINITY, f64::INFINITY),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_creation() {
        let location = Point::origin();
        let direction = Direction::z_axis();
        let x_direction = Direction::x_axis();
        let plane = Plane::new(location, direction, x_direction);
        assert_eq!(plane.location(), &location);
        assert_eq!(plane.direction(), &direction);
        assert_eq!(plane.x_direction(), &x_direction);
    }

    #[test]
    fn test_plane_from_point_normal() {
        let location = Point::origin();
        let normal = Direction::z_axis();
        let plane = Plane::from_point_normal(location, normal);
        assert_eq!(plane.location(), &location);
        assert_eq!(plane.direction(), &normal);
    }

    #[test]
    fn test_plane_xy_plane() {
        let plane = Plane::xy_plane();
        assert_eq!(plane.location(), &Point::origin());
        assert_eq!(plane.direction(), &Direction::z_axis());
        assert_eq!(plane.x_direction(), &Direction::x_axis());
        assert_eq!(plane.y_direction(), &Direction::y_axis());
    }

    #[test]
    fn test_plane_yz_plane() {
        let plane = Plane::yz_plane();
        assert_eq!(plane.location(), &Point::origin());
        assert_eq!(plane.direction(), &Direction::x_axis());
    }

    #[test]
    fn test_plane_zx_plane() {
        let plane = Plane::zx_plane();
        assert_eq!(plane.location(), &Point::origin());
        assert_eq!(plane.direction(), &Direction::y_axis());
    }

    #[test]
    fn test_plane_distance() {
        let plane = Plane::xy_plane();
        let point = Point::new(0.0, 0.0, 5.0);
        let distance = plane.distance(&point);
        assert!((distance - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_plane_contains() {
        let plane = Plane::xy_plane();
        let point = Point::new(1.0, 2.0, 0.0);
        assert!(plane.contains(&point, 0.001));
    }

    #[test]
    fn test_plane_reverse() {
        let mut plane = Plane::xy_plane();
        plane.reverse();
        assert!(plane.direction.is_opposite(&Direction::z_axis(), 0.001));
    }

    #[test]
    fn test_plane_is_parallel() {
        let plane1 = Plane::xy_plane();
        let plane2 = Plane::xy_plane();
        assert!(plane1.is_parallel(&plane2, 0.001));
    }

    #[test]
    fn test_plane_is_normal() {
        let plane1 = Plane::xy_plane();
        let plane2 = Plane::yz_plane();
        assert!(plane1.is_normal(&plane2, 0.001));
    }

    #[test]
    fn test_plane_angle() {
        let plane1 = Plane::xy_plane();
        let plane2 = Plane::yz_plane();
        let angle = plane1.angle(&plane2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_plane_translate() {
        let mut plane = Plane::xy_plane();
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        plane.translate(&vec);
        assert_eq!(plane.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_plane_rotate() {
        let mut plane = Plane::xy_plane();
        let axis = Axis::z_axis();
        plane.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!((plane.x_direction.x() - 0.0).abs() < 0.001);
        assert!((plane.x_direction.y() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_plane_is_direct() {
        let plane = Plane::xy_plane();
        assert!(plane.is_direct());
        assert!(plane.is_right_handed());
        assert!(!plane.is_left_handed());
    }
}
