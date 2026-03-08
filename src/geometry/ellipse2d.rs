use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector, Direction, Axis};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse2D {
    location: Point,
    x_direction: Direction,
    y_direction: Direction,
    major_radius: Standard_Real,
    minor_radius: Standard_Real,
}

impl Ellipse2D {
    pub fn new(location: Point, x_direction: Direction, major_radius: Standard_Real, minor_radius: Standard_Real) -> Self {
        let y_direction = Direction::new(0.0, 0.0, 1.0).cross(&x_direction).normalized();
        Self {
            location,
            x_direction,
            y_direction,
            major_radius,
            minor_radius,
        }
    }

    pub fn from_center_radii(center: Point, major_radius: Standard_Real, minor_radius: Standard_Real) -> Self {
        Self {
            location: center,
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            major_radius,
            minor_radius,
        }
    }

    pub fn from_center_axis_radii(center: Point, axis: &Axis, major_radius: Standard_Real, minor_radius: Standard_Real) -> Self {
        let x_dir = axis.direction();
        let y_dir = Direction::new(0.0, 0.0, 1.0).cross(x_dir).normalized();
        Self {
            location: center,
            x_direction: *x_dir,
            y_direction: y_dir,
            major_radius,
            minor_radius,
        }
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn x_direction(&self) -> &Direction {
        &self.x_direction
    }

    pub fn y_direction(&self) -> &Direction {
        &self.y_direction
    }

    pub fn major_radius(&self) -> Standard_Real {
        self.major_radius
    }

    pub fn minor_radius(&self) -> Standard_Real {
        self.minor_radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_x_direction(&mut self, x_direction: Direction) {
        self.x_direction = x_direction;
        self.update_y_direction();
    }

    pub fn set_major_radius(&mut self, major_radius: Standard_Real) {
        self.major_radius = major_radius;
    }

    pub fn set_minor_radius(&mut self, minor_radius: Standard_Real) {
        self.minor_radius = minor_radius;
    }

    fn update_y_direction(&mut self) {
        self.y_direction = Direction::new(0.0, 0.0, 1.0).cross(&self.x_direction).normalized();
    }

    pub fn area(&self) -> Standard_Real {
        std::f64::consts::PI * self.major_radius * self.minor_radius
    }

    pub fn length(&self) -> Standard_Real {
        let a = self.major_radius;
        let b = self.minor_radius;
        let h = ((a - b) / (a + b)).powi(2);
        let approximation = std::f64::consts::PI * (a + b) * (1.0 + 3.0 * h / (10.0 + (4.0 - 3.0 * h).sqrt()));
        approximation
    }

    pub fn eccentricity(&self) -> Standard_Real {
        let a = self.major_radius.max(self.minor_radius);
        let b = self.major_radius.min(self.minor_radius);
        if a <= STANDARD_REAL_EPSILON {
            0.0
        } else {
            (1.0 - (b * b) / (a * a)).sqrt()
        }
    }

    pub fn focal_distance(&self) -> Standard_Real {
        let a = self.major_radius.max(self.minor_radius);
        let b = self.major_radius.min(self.minor_radius);
        (a * a - b * b).sqrt()
    }

    pub fn position(&self, parameter: Standard_Real) -> Point {
        let cos_a = parameter.cos();
        let sin_a = parameter.sin();
        
        let x_offset = self.major_radius * cos_a;
        let y_offset = self.minor_radius * sin_a;

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Point::new(
            self.location.x + x_offset * x_vec.x + y_offset * y_vec.x,
            self.location.y + x_offset * x_vec.y + y_offset * y_vec.y,
            self.location.z + x_offset * x_vec.z + y_offset * y_vec.z,
        )
    }

    pub fn d1(&self, parameter: Standard_Real) -> Vector {
        let sin_a = parameter.sin();
        let cos_a = parameter.cos();

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Vector::new(
            -self.major_radius * sin_a * x_vec.x + self.minor_radius * cos_a * y_vec.x,
            -self.major_radius * sin_a * x_vec.y + self.minor_radius * cos_a * y_vec.y,
            -self.major_radius * sin_a * x_vec.z + self.minor_radius * cos_a * y_vec.z,
        )
    }

    pub fn d2(&self, parameter: Standard_Real) -> Vector {
        let cos_a = parameter.cos();
        let sin_a = parameter.sin();

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Vector::new(
            -self.major_radius * cos_a * x_vec.x - self.minor_radius * sin_a * y_vec.x,
            -self.major_radius * cos_a * x_vec.y - self.minor_radius * sin_a * y_vec.y,
            -self.major_radius * cos_a * x_vec.z - self.minor_radius * sin_a * y_vec.z,
        )
    }

    pub fn contains(&self, point: &Point, tolerance: Standard_Real) -> bool {
        let distance = self.distance(point);
        distance <= tolerance
    }

    pub fn distance(&self, point: &Point) -> Standard_Real {
        let vec = Vector::from_point(&self.location, point);
        
        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        let x_coord = vec.dot(&x_vec);
        let y_coord = vec.dot(&y_vec);

        let a = self.major_radius;
        let b = self.minor_radius;

        let normalized_x = x_coord / a;
        let normalized_y = y_coord / b;

        let distance_from_center = (normalized_x * normalized_x + normalized_y * normalized_y).sqrt();
        let distance_to_ellipse = (distance_from_center - 1.0).abs() * a.min(b);

        distance_to_ellipse
    }

    pub fn square_distance(&self, point: &Point) -> Standard_Real {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Ellipse2D {
        Ellipse2D {
            location: self.location.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.x_direction.mirror_axis(axis);
        self.y_direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Ellipse2D {
        Ellipse2D {
            location: self.location.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: Standard_Real) {
        self.location.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: Standard_Real) -> Ellipse2D {
        Ellipse2D {
            location: self.location.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: Standard_Real) {
        self.location.scale(point, factor);
        self.major_radius *= factor.abs();
        self.minor_radius *= factor.abs();
    }

    pub fn scaled(&self, point: &Point, factor: Standard_Real) -> Ellipse2D {
        Ellipse2D {
            location: self.location.scaled(point, factor),
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            major_radius: self.major_radius * factor.abs(),
            minor_radius: self.minor_radius * factor.abs(),
        }
    }

    pub fn translate(&mut self, vec: &Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &Vector) -> Ellipse2D {
        Ellipse2D {
            location: self.location.translated(vec),
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            major_radius: self.major_radius,
            minor_radius: self.minor_radius,
        }
    }

    pub fn transform(&mut self, trsf: &crate::geometry::Transform) {
        self.location = trsf.transforms(&self.location);
        self.x_direction = trsf.transforms_dir(&self.x_direction);
        self.y_direction = trsf.transforms_dir(&self.y_direction);
        self.major_radius *= trsf.scale.abs();
        self.minor_radius *= trsf.scale.abs();
    }

    pub fn transformed(&self, trsf: &crate::geometry::Transform) -> Ellipse2D {
        Ellipse2D {
            location: trsf.transforms(&self.location),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
            major_radius: self.major_radius * trsf.scale.abs(),
            minor_radius: self.minor_radius * trsf.scale.abs(),
        }
    }

    pub fn to_ellipse3d(&self) -> crate::geometry::Ellipse {
        crate::geometry::Ellipse::new(
            self.location,
            self.x_direction,
            self.major_radius,
            self.minor_radius,
        )
    }

    pub fn is_circle(&self, tolerance: Standard_Real) -> bool {
        (self.major_radius - self.minor_radius).abs() <= tolerance
    }
}

impl Default for Ellipse2D {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            major_radius: 1.0,
            minor_radius: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipse2d_creation() {
        let center = Point::origin();
        let x_direction = Direction::x_axis();
        let major_radius = 5.0;
        let minor_radius = 3.0;
        let ellipse = Ellipse2D::new(center, x_direction, major_radius, minor_radius);
        assert_eq!(ellipse.location(), &center);
        assert_eq!(ellipse.major_radius(), major_radius);
        assert_eq!(ellipse.minor_radius(), minor_radius);
    }

    #[test]
    fn test_ellipse2d_from_center_radii() {
        let center = Point::origin();
        let major_radius = 5.0;
        let minor_radius = 3.0;
        let ellipse = Ellipse2D::from_center_radii(center, major_radius, minor_radius);
        assert_eq!(ellipse.location(), &center);
        assert_eq!(ellipse.major_radius(), major_radius);
        assert_eq!(ellipse.minor_radius(), minor_radius);
    }

    #[test]
    fn test_ellipse2d_area() {
        let ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let area = ellipse.area();
        assert!((area - std::f64::consts::PI * 15.0).abs() < 0.001);
    }

    #[test]
    fn test_ellipse2d_eccentricity() {
        let ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let eccentricity = ellipse.eccentricity();
        let expected = (1.0f64 - 9.0 / 25.0).sqrt();
        assert!((eccentricity - expected).abs() < 0.001);
    }

    #[test]
    fn test_ellipse2d_position() {
        let ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let point = ellipse.position(0.0);
        assert_eq!(point.x, 5.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }

    #[test]
    fn test_ellipse2d_is_circle() {
        let ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 5.0);
        assert!(ellipse.is_circle(0.001));
    }

    #[test]
    fn test_ellipse2d_translate() {
        let mut ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let vec = Vector::new(1.0, 2.0, 3.0);
        ellipse.translate(&vec);
        assert_eq!(ellipse.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_ellipse2d_scale() {
        let mut ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let point = Point::origin();
        ellipse.scale(&point, 2.0);
        assert_eq!(ellipse.major_radius(), 10.0);
        assert_eq!(ellipse.minor_radius(), 6.0);
    }

    #[test]
    fn test_ellipse2d_rotate() {
        let mut ellipse = Ellipse2D::from_center_radii(Point::origin(), 5.0, 3.0);
        let axis = Axis::z_axis();
        ellipse.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!(ellipse.x_direction.is_equal(&Direction::y_axis(), 0.001));
    }
}
