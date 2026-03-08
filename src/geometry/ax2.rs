use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Direction, Axis, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ax2 {
    location: Point,
    direction: Direction,
    x_direction: Direction,
    y_direction: Direction,
}

impl Ax2 {
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

    pub fn from_axes(main_axis: &Axis, x_axis: &Axis) -> Self {
        let location = *main_axis.location();
        let direction = *main_axis.direction();
        let x_direction = *x_axis.direction();
        let y_direction = direction.cross(&x_direction).normalized();

        Self {
            location,
            direction,
            x_direction,
            y_direction,
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

    pub fn reversed(&self) -> Ax2 {
        Ax2 {
            location: self.location,
            direction: self.direction.reversed(),
            x_direction: self.x_direction.reversed(),
            y_direction: self.y_direction.reversed(),
        }
    }

    pub fn angle(&self, other: &Ax2) -> Standard_Real {
        self.direction.angle(&other.direction)
    }

    pub fn is_coaxial(&self, other: &Ax2, angular_tolerance: Standard_Real, linear_tolerance: Standard_Real) -> bool {
        self.direction.is_co_linear(&other.direction, angular_tolerance) &&
        self.location.distance(&other.location) <= linear_tolerance
    }

    pub fn is_opposite(&self, other: &Ax2, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_opposite(&other.direction, angular_tolerance)
    }

    pub fn is_parallel(&self, other: &Ax2, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_parallel(&other.direction, angular_tolerance)
    }

    pub fn is_normal(&self, other: &Ax2, angular_tolerance: Standard_Real) -> bool {
        self.direction.is_normal(&other.direction, angular_tolerance)
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.direction.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Ax2 {
        Ax2 {
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

    pub fn mirrored_axis(&self, axis: &Axis) -> Ax2 {
        Ax2 {
            location: self.location.mirrored_axis(axis),
            direction: self.direction.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
        }
    }

    pub fn mirror_ax2(&mut self, ax2: &Ax2) {
        self.location.mirror_axis(&ax2.main_axis());
        self.direction.mirror_axis(&ax2.main_axis());
        self.x_direction.mirror_axis(&ax2.main_axis());
        self.y_direction.mirror_axis(&ax2.main_axis());
    }

    pub fn mirrored_ax2(&self, ax2: &Ax2) -> Ax2 {
        Ax2 {
            location: self.location.mirrored_axis(&ax2.main_axis()),
            direction: self.direction.mirrored_axis(&ax2.main_axis()),
            x_direction: self.x_direction.mirrored_axis(&ax2.main_axis()),
            y_direction: self.y_direction.mirrored_axis(&ax2.main_axis()),
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: Standard_Real) {
        self.location.rotate(axis, angle);
        self.direction.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: Standard_Real) -> Ax2 {
        Ax2 {
            location: self.location.rotated(axis, angle),
            direction: self.direction.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
        }
    }

    pub fn scale(&mut self, point: &Point, factor: Standard_Real) {
        self.location.scale(point, factor);
    }

    pub fn scaled(&self, point: &Point, factor: Standard_Real) -> Ax2 {
        Ax2 {
            location: self.location.scaled(point, factor),
            direction: self.direction,
            x_direction: self.x_direction,
            y_direction: self.y_direction,
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Ax2 {
        Ax2 {
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

    pub fn transformed(&self, trsf: &Transform) -> Ax2 {
        Ax2 {
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

    pub fn to_coordinate_system(&self) -> crate::geometry::CoordinateSystem {
        crate::geometry::CoordinateSystem::new(self.location, self.direction, self.x_direction)
    }

    pub fn to_plane(&self) -> crate::geometry::Plane {
        crate::geometry::Plane::from_ax2(self)
    }
}

impl Default for Ax2 {
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
    fn test_ax2_creation() {
        let location = Point::origin();
        let direction = Direction::z_axis();
        let x_direction = Direction::x_axis();
        let ax2 = Ax2::new(location, direction, x_direction);
        assert_eq!(ax2.location(), &location);
        assert_eq!(ax2.direction(), &direction);
        assert_eq!(ax2.x_direction(), &x_direction);
    }

    #[test]
    fn test_ax2_from_axis() {
        let axis = Axis::z_axis();
        let ax2 = Ax2::from_axis(&axis);
        assert_eq!(ax2.location(), &Point::origin());
        assert_eq!(ax2.direction(), &Direction::z_axis());
    }

    #[test]
    fn test_ax2_default() {
        let ax2 = Ax2::default();
        assert_eq!(ax2.location(), &Point::origin());
        assert_eq!(ax2.direction(), &Direction::z_axis());
        assert_eq!(ax2.x_direction(), &Direction::x_axis());
        assert_eq!(ax2.y_direction(), &Direction::y_axis());
    }

    #[test]
    fn test_ax2_is_direct() {
        let ax2 = Ax2::default();
        assert!(ax2.is_direct());
        assert!(ax2.is_right_handed());
        assert!(!ax2.is_left_handed());
    }

    #[test]
    fn test_ax2_reverse() {
        let mut ax2 = Ax2::default();
        ax2.reverse();
        assert!(ax2.direction.is_opposite(&Direction::z_axis(), 0.001));
    }

    #[test]
    fn test_ax2_rotate() {
        let mut ax2 = Ax2::default();
        let axis = Axis::z_axis();
        ax2.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!((ax2.x_direction.x() - 0.0).abs() < 0.001);
        assert!((ax2.x_direction.y() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ax2_translate() {
        let mut ax2 = Ax2::default();
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        ax2.translate(&vec);
        assert_eq!(ax2.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_ax2_scale() {
        let mut ax2 = Ax2::default();
        let point = Point::origin();
        ax2.scale(&point, 2.0);
        assert_eq!(ax2.location(), &Point::origin());
    }

    #[test]
    fn test_ax2_is_parallel() {
        let ax1 = Ax2::default();
        let ax2 = Ax2::default();
        assert!(ax1.is_parallel(&ax2, 0.001));
    }

    #[test]
    fn test_ax2_is_normal() {
        let ax1 = Ax2::default();
        let mut ax2 = Ax2::default();
        ax2.set_direction(Direction::x_axis());
        assert!(ax1.is_normal(&ax2, 0.001));
    }

    #[test]
    fn test_ax2_angle() {
        let ax1 = Ax2::default();
        let mut ax2 = Ax2::default();
        ax2.set_direction(Direction::x_axis());
        let angle = ax1.angle(&ax2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_ax2_to_transform() {
        let ax2 = Ax2::default();
        let trsf = ax2.to_transform();
        assert!((trsf.scale() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ax2_to_coordinate_system() {
        let ax2 = Ax2::default();
        let cs = ax2.to_coordinate_system();
        assert_eq!(cs.location(), &Point::origin());
        assert_eq!(cs.direction(), &Direction::z_axis());
    }

    #[test]
    fn test_ax2_to_plane() {
        let ax2 = Ax2::default();
        let plane = ax2.to_plane();
        assert_eq!(plane.location(), &Point::origin());
        assert_eq!(plane.direction(), &Direction::z_axis());
    }
}
