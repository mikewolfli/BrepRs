use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Direction, Axis, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoordinateSystem {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
}

impl CoordinateSystem {
    pub fn new(location: Point, direction: Direction, x_direction: Direction) -> Self {
        let y_direction = direction.cross(&x_direction).normalized();
        Self {
            location,
            direction,
            x_direction,
            y_direction,
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

    pub fn from_plane(plane: &crate::geometry::Plane) -> Self {
        Self {
            location: *plane.location(),
            direction: *plane.direction(),
            x_direction: *plane.x_direction(),
            y_direction: *plane.y_direction(),
        }
    }

    pub fn from_point_direction_xdir(location: Point, direction: Direction, x_direction: Direction) -> Self {
        let y_direction = direction.cross(&x_direction).normalized();
        Self {
            location,
            direction,
            x_direction,
            y_direction,
        }
    }

    pub fn from_axes(main_axis: &Axis, x_axis: &Axis) -> Self {
        let location = main_axis.location();
        let direction = main_axis.direction();
        let x_direction = x_axis.direction();
        let y_direction = direction.cross(x_direction).normalized();

        Self {
            location: *location,
            direction: *direction,
            x_direction: *x_direction,
            y_direction,
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

    pub fn main_axis(&self) -> Axis {
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
        self.x_direction.reverse();
        self.y_direction.reverse();
    }

    pub fn reversed(&self) -> CoordinateSystem {
        CoordinateSystem {
            location: self.location,
            direction: self.direction.reversed(),
            x_direction: self.x_direction.reversed(),
            y_direction: self.y_direction.reversed(),
        }
    }

    pub fn angle(&self, other: &CoordinateSystem) -> Standard_Real {
        self.direction.angle(&other.direction)
    }

    pub fn is_coaxial(&self, other: &CoordinateSystem, angular_tolerance: Standard_Real, linear_tolerance: Standard_Real) -> bool {
        self.direction.is_co_linear(&other.direction, angular_tolerance) &&
        self.location.distance(&other.location) <= linear_tolerance
    }

    pub fn is_opposite(&self, other: &CoordinateSystem, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_opposite(&other.direction, angular_tolerance)
    }

    pub fn is_parallel(&self, other: &CoordinateSystem, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_parallel(&other.direction, angular_tolerance)
    }

    pub fn is_normal(&self, other: &CoordinateSystem, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_normal(&other.direction, angular_tolerance)
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> CoordinateSystem {
        CoordinateSystem {
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

    pub fn mirrored_axis(&self, axis: &Axis) -> CoordinateSystem {
        CoordinateSystem {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
        }
    }

    pub fn mirror_cs(&mut self, cs: &CoordinateSystem) {
        self.location.mirror_axis(&cs.main_axis());
        self.direction.mirror_axis(&cs.main_axis());
        self.x_direction.mirror_axis(&cs.main_axis());
        self.y_direction.mirror_axis(&cs.main_axis());
    }

    pub fn mirrored_cs(&self, cs: &CoordinateSystem) -> CoordinateSystem {
        CoordinateSystem {
            location: self.location.mirrored_axis(&cs.main_axis()),
            direction: self.direction.mirrored_axis(&cs.main_axis()),
            x_direction: self.x_direction.mirrored_axis(&cs.main_axis()),
            y_direction: self.y_direction.mirrored_axis(&cs.main_axis()),
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: Standard_Real) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: Standard_Real) -> CoordinateSystem {
        CoordinateSystem {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
        }
    }

    pub fn scale(&mut self, point: &Point, factor: Standard_Real) {
        self.location.scale(point, factor);
    }

    pub fn scaled(&self, point: &Point, factor: Standard_Real) -> CoordinateSystem {
        CoordinateSystem {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> CoordinateSystem {
        CoordinateSystem {
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

    pub fn transformed(&self, trsf: &Transform) -> CoordinateSystem {
        CoordinateSystem {
            location: trsf.transforms(&self.location),
            direction: trsf.transforms_dir(&self.direction),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
        }
    }

    pub fn to_transform(&self) -> Transform {
        let mut rotation = crate::geometry::Matrix::identity();
        
        rotation.data[0][0] = self.x_direction.x;
        rotation.data[1][0] = self.x_direction.y;
        rotation.data[2][0] = self.x_direction.z;
        
        rotation.data[0][1] = self.y_direction.x;
        rotation.data[1][1] = self.y_direction.y;
        rotation.data[2][1] = self.y_direction.z;
        
        rotation.data[0][2] = self.direction.x;
        rotation.data[1][2] = self.direction.y;
        rotation.data[2][2] = self.direction.z;

        let translation = crate::geometry::Vector::new(self.location.x, self.location.y, self.location.z);

        Transform {
            scale: 1.0,
            translation,
            rotation,
            shape: crate::geometry::TrsfForm::Other,
        }
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

    pub fn to_plane(&self) -> crate::geometry::Plane {
        crate::geometry::Plane::from_coordinate_system(self)
    }
}

impl Default for CoordinateSystem {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            direction: Direction::z_axis(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_system_creation() {
        let location = Point::origin();
        let direction = Direction::z_axis();
        let x_direction = Direction::x_axis();
        let cs = CoordinateSystem::new(location, direction, x_direction);
        assert_eq!(cs.location(), &location);
        assert_eq!(cs.direction(), &direction);
        assert_eq!(cs.x_direction(), &x_direction);
    }

    #[test]
    fn test_coordinate_system_from_axis() {
        let axis = Axis::z_axis();
        let cs = CoordinateSystem::from_axis(&axis);
        assert_eq!(cs.location(), &Point::origin());
        assert_eq!(cs.direction(), &Direction::z_axis());
    }

    #[test]
    fn test_coordinate_system_default() {
        let cs = CoordinateSystem::default();
        assert_eq!(cs.location(), &Point::origin());
        assert_eq!(cs.direction(), &Direction::z_axis());
        assert_eq!(cs.x_direction(), &Direction::x_axis());
        assert_eq!(cs.y_direction(), &Direction::y_axis());
    }

    #[test]
    fn test_coordinate_system_is_direct() {
        let cs = CoordinateSystem::default();
        assert!(cs.is_direct());
        assert!(cs.is_right_handed());
        assert!(!cs.is_left_handed());
    }

    #[test]
    fn test_coordinate_system_reverse() {
        let mut cs = CoordinateSystem::default();
        cs.reverse();
        assert!(cs.direction.is_opposite(&Direction::z_axis(), 0.001));
    }

    #[test]
    fn test_coordinate_system_rotate() {
        let mut cs = CoordinateSystem::default();
        let axis = Axis::z_axis();
        cs.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!((cs.x_direction.x() - 0.0).abs() < 0.001);
        assert!((cs.x_direction.y() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_coordinate_system_translate() {
        let mut cs = CoordinateSystem::default();
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        cs.translate(&vec);
        assert_eq!(cs.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_coordinate_system_scale() {
        let mut cs = CoordinateSystem::default();
        let point = Point::origin();
        cs.scale(&point, 2.0);
        assert_eq!(cs.location(), &Point::origin());
    }

    #[test]
    fn test_coordinate_system_is_parallel() {
        let cs1 = CoordinateSystem::default();
        let cs2 = CoordinateSystem::default();
        assert!(cs1.is_parallel(&cs2, 0.001));
    }

    #[test]
    fn test_coordinate_system_is_normal() {
        let cs1 = CoordinateSystem::default();
        let mut cs2 = CoordinateSystem::default();
        cs2.set_direction(Direction::x_axis());
        assert!(cs1.is_normal(&cs2, 0.001));
    }

    #[test]
    fn test_coordinate_system_angle() {
        let cs1 = CoordinateSystem::default();
        let mut cs2 = CoordinateSystem::default();
        cs2.set_direction(Direction::x_axis());
        let angle = cs1.angle(&cs2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_coordinate_system_to_transform() {
        let cs = CoordinateSystem::default();
        let trsf = cs.to_transform();
        assert!((trsf.scale() - 1.0).abs() < 0.001);
    }
}
