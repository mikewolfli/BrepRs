use crate::foundation::types::StandardReal;
use crate::geometry::{Axis, Direction, Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    location: Point,
    x_direction: Direction,
    y_direction: Direction,
    radius: StandardReal,
}

impl Circle {
    pub fn new(location: Point, x_direction: Direction, radius: StandardReal) -> Self {
        let y_direction = x_direction
            .cross(&Direction::new(0.0, 0.0, 1.0))
            .normalized();
        Self {
            location,
            x_direction,
            y_direction,
            radius,
        }
    }

    pub fn from_center_radius(center: Point, radius: StandardReal) -> Self {
        Self {
            location: center,
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            radius,
        }
    }

    pub fn from_center_axis_radius(center: Point, axis: &Axis, radius: StandardReal) -> Self {
        let x_dir = axis.direction();
        let z_dir = Direction::new(0.0, 0.0, 1.0);
        let y_dir = x_dir.cross(&z_dir).normalized();
        Self {
            location: center,
            x_direction: *x_dir,
            y_direction: y_dir,
            radius,
        }
    }

    pub fn from_axis(axis: &Axis, radius: StandardReal) -> Self {
        Self {
            location: *axis.location(),
            x_direction: *axis.direction(),
            y_direction: axis
                .direction()
                .cross(&Direction::new(0.0, 0.0, 1.0))
                .normalized(),
            radius,
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

    pub fn radius(&self) -> StandardReal {
        self.radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_x_direction(&mut self, x_direction: Direction) {
        self.x_direction = x_direction;
        self.update_y_direction();
    }

    pub fn set_radius(&mut self, radius: StandardReal) {
        self.radius = radius;
    }

    fn update_y_direction(&mut self) {
        self.y_direction = self
            .x_direction
            .cross(&Direction::new(0.0, 0.0, 1.0))
            .normalized();
    }

    pub fn area(&self) -> StandardReal {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn length(&self) -> StandardReal {
        2.0 * std::f64::consts::PI * self.radius
    }

    pub fn position(&self, parameter: StandardReal) -> Point {
        let cos_a = parameter.cos();
        let sin_a = parameter.sin();

        let x_offset = self.radius * cos_a;
        let y_offset = self.radius * sin_a;

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Point::new(
            self.location.x + x_offset * x_vec.x + y_offset * y_vec.x,
            self.location.y + x_offset * x_vec.y + y_offset * y_vec.y,
            self.location.z + x_offset * x_vec.z + y_offset * y_vec.z,
        )
    }

    pub fn d1(&self, parameter: StandardReal) -> Vector {
        let sin_a = parameter.sin();
        let cos_a = parameter.cos();

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Vector::new(
            self.radius * (-sin_a * x_vec.x + cos_a * y_vec.x),
            self.radius * (-sin_a * x_vec.y + cos_a * y_vec.y),
            self.radius * (-sin_a * x_vec.z + cos_a * y_vec.z),
        )
    }

    pub fn d2(&self, parameter: StandardReal) -> Vector {
        let cos_a = parameter.cos();
        let sin_a = parameter.sin();

        let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
        let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

        Vector::new(
            self.radius * (-cos_a * x_vec.x - sin_a * y_vec.x),
            self.radius * (-cos_a * x_vec.y - sin_a * y_vec.y),
            self.radius * (-cos_a * x_vec.z - sin_a * y_vec.z),
        )
    }

    pub fn dn(&self, parameter: StandardReal, n: i32) -> Vector {
        match n {
            0 => Vector::from_point(&self.location, &self.position(parameter)),
            1 => self.d1(parameter),
            2 => self.d2(parameter),
            _ => {
                let angle_offset = std::f64::consts::PI / 2.0 * (n as f64);
                let cos_a = (parameter + angle_offset).cos();
                let sin_a = (parameter + angle_offset).sin();

                let x_vec = Vector::new(self.x_direction.x, self.x_direction.y, self.x_direction.z);
                let y_vec = Vector::new(self.y_direction.x, self.y_direction.y, self.y_direction.z);

                let magnitude = self.radius * (std::f64::consts::PI / 2.0).powi(n - 1);

                Vector::new(
                    magnitude * (cos_a * x_vec.x + sin_a * y_vec.x),
                    magnitude * (cos_a * x_vec.y + sin_a * y_vec.y),
                    magnitude * (cos_a * x_vec.z + sin_a * y_vec.z),
                )
            }
        }
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        let vec = Vector::from_point(&self.location, point);
        let distance_to_center = vec.magnitude();
        (distance_to_center - self.radius).abs() <= tolerance
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = Vector::from_point(&self.location, point);
        let distance_to_center = vec.magnitude();
        (distance_to_center - self.radius).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let dist = self.distance(point);
        dist * dist
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
        self.x_direction.mirror(point);
        self.y_direction.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Circle {
        Circle {
            location: self.location.mirrored(point),
            x_direction: self.x_direction.mirrored(point),
            y_direction: self.y_direction.mirrored(point),
            radius: self.radius,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
        self.x_direction.mirror_axis(axis);
        self.y_direction.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Circle {
        Circle {
            location: self.location.mirrored_axis(axis),
            x_direction: self.x_direction.mirrored_axis(axis),
            y_direction: self.y_direction.mirrored_axis(axis),
            radius: self.radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
        self.x_direction.rotate(axis, angle);
        self.y_direction.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Circle {
        Circle {
            location: self.location.rotated(axis, angle),
            x_direction: self.x_direction.rotated(axis, angle),
            y_direction: self.y_direction.rotated(axis, angle),
            radius: self.radius,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
        self.radius *= factor.abs();
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Circle {
        Circle {
            location: self.location.scaled(point, factor),
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            radius: self.radius * factor.abs(),
        }
    }

    pub fn translate(&mut self, vec: &Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &Vector) -> Circle {
        Circle {
            location: self.location.translated(vec),
            x_direction: self.x_direction,
            y_direction: self.y_direction,
            radius: self.radius,
        }
    }

    pub fn transform(&mut self, trsf: &crate::geometry::Transform) {
        self.location = trsf.transforms(&self.location);
        self.x_direction = trsf.transforms_dir(&self.x_direction);
        self.y_direction = trsf.transforms_dir(&self.y_direction);
        self.radius *= trsf.scale.abs();
    }

    pub fn transformed(&self, trsf: &crate::geometry::Transform) -> Circle {
        Circle {
            location: trsf.transforms(&self.location),
            x_direction: trsf.transforms_dir(&self.x_direction),
            y_direction: trsf.transforms_dir(&self.y_direction),
            radius: self.radius * trsf.scale.abs(),
        }
    }

    pub fn to_axis(&self) -> Axis {
        Axis::new(self.location, self.x_direction)
    }

    pub fn to_circle2d(&self) -> crate::geometry::Circle2D {
        crate::geometry::Circle2D::new(self.location, self.x_direction, self.radius)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            x_direction: Direction::x_axis(),
            y_direction: Direction::y_axis(),
            radius: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_creation() {
        let center = Point::origin();
        let x_direction = Direction::x_axis();
        let radius = 5.0;
        let circle = Circle::new(center, x_direction, radius);
        assert_eq!(circle.location(), &center);
        assert_eq!(circle.radius(), radius);
    }

    #[test]
    fn test_circle_from_center_radius() {
        let center = Point::origin();
        let radius = 5.0;
        let circle = Circle::from_center_radius(center, radius);
        assert_eq!(circle.location(), &center);
        assert_eq!(circle.radius(), radius);
    }

    #[test]
    fn test_circle_area() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let area = circle.area();
        assert!((area - std::f64::consts::PI * 25.0).abs() < 0.001);
    }

    #[test]
    fn test_circle_length() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let length = circle.length();
        assert!((length - 10.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_circle_position() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let point = circle.position(0.0);
        assert_eq!(point.x, 5.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 0.0);
    }

    #[test]
    fn test_circle_contains() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let point = Point::new(5.0, 0.0, 0.0);
        assert!(circle.contains(&point, 0.001));
    }

    #[test]
    fn test_circle_distance() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let point = Point::new(5.0, 0.0, 0.0);
        let distance = circle.distance(&point);
        assert!((distance - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_circle_translate() {
        let mut circle = Circle::from_center_radius(Point::origin(), 5.0);
        let vec = Vector::new(1.0, 2.0, 3.0);
        circle.translate(&vec);
        assert_eq!(circle.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_circle_scale() {
        let mut circle = Circle::from_center_radius(Point::origin(), 5.0);
        let point = Point::origin();
        circle.scale(&point, 2.0);
        assert_eq!(circle.radius(), 10.0);
    }

    #[test]
    fn test_circle_rotate() {
        let mut circle = Circle::from_center_radius(Point::origin(), 5.0);
        let axis = Axis::z_axis();
        circle.rotate(&axis, std::f64::consts::PI / 2.0);
        assert!(circle.x_direction.is_equal(&Direction::y_axis(), 0.001));
    }

    #[test]
    fn test_circle_to_axis() {
        let circle = Circle::from_center_radius(Point::origin(), 5.0);
        let axis = circle.to_axis();
        assert_eq!(axis.location(), &Point::origin());
        assert_eq!(axis.direction(), &Direction::x_axis());
    }
}
