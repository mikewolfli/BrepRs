use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Direction, Axis, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sphere {
    location: Point,
    radius: StandardReal,
}

impl Sphere {
    pub fn new(location: Point, radius: StandardReal) -> Self {
        Self {
            location,
            radius,
        }
    }

    pub fn from_center_radius(center: Point, radius: StandardReal) -> Self {
        Self {
            location: center,
            radius,
        }
    }

    pub fn from_axis(axis: &Axis, radius: StandardReal) -> Self {
        Self {
            location: *axis.location(),
            radius,
        }
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn radius(&self) -> StandardReal {
        self.radius
    }

    pub fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    pub fn set_radius(&mut self, radius: StandardReal) {
        self.radius = radius;
    }

    pub fn axis(&self) -> Axis {
        Axis::new(self.location, Direction::z_axis())
    }

    pub fn position(&self, u: StandardReal, v: StandardReal) -> Point {
        let sin_u = u.sin();
        let cos_u = u.cos();
        let sin_v = v.sin();
        let cos_v = v.cos();

        let x = self.radius * sin_u * cos_v;
        let y = self.radius * sin_u * sin_v;
        let z = self.radius * cos_u;

        Point::new(
            self.location.x + x,
            self.location.y + y,
            self.location.z + z,
        )
    }

    pub fn contains(&self, point: &Point, tolerance: StandardReal) -> bool {
        let distance = self.distance(point);
        distance <= tolerance
    }

    pub fn distance(&self, point: &Point) -> StandardReal {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let distance_to_center = vec.magnitude();
        (distance_to_center - self.radius).abs()
    }

    pub fn square_distance(&self, point: &Point) -> StandardReal {
        let vec = crate::geometry::Vector::from_point(&self.location, point);
        let distance_to_center = vec.magnitude();
        let diff = distance_to_center - self.radius;
        diff * diff
    }

    pub fn area(&self) -> StandardReal {
        4.0 * std::f64::consts::PI * self.radius * self.radius
    }

    pub fn volume(&self) -> StandardReal {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius * self.radius * self.radius
    }

    pub fn mirror(&mut self, point: &Point) {
        self.location.mirror(point);
    }

    pub fn mirrored(&self, point: &Point) -> Sphere {
        Sphere {
            location: self.location.mirrored(point),
            radius: self.radius,
        }
    }

    pub fn mirror_axis(&mut self, axis: &Axis) {
        self.location.mirror_axis(axis);
    }

    pub fn mirrored_axis(&self, axis: &Axis) -> Sphere {
        Sphere {
            location: self.location.mirrored_axis(axis),
            radius: self.radius,
        }
    }

    pub fn rotate(&mut self, axis: &Axis, angle: StandardReal) {
        self.location.rotate(axis, angle);
    }

    pub fn rotated(&self, axis: &Axis, angle: StandardReal) -> Sphere {
        Sphere {
            location: self.location.rotated(axis, angle),
            radius: self.radius,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.location.scale(point, factor);
        self.radius *= factor.abs();
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Sphere {
        Sphere {
            location: self.location.scaled(point, factor),
            radius: self.radius * factor.abs(),
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.location.translate(vec);
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Sphere {
        Sphere {
            location: self.location.translated(vec),
            radius: self.radius,
        }
    }

    pub fn transform(&mut self, trsf: &Transform) {
        self.location = trsf.transforms(&self.location);
        self.radius *= trsf.scale.abs();
    }

    pub fn transformed(&self, trsf: &Transform) -> Sphere {
        Sphere {
            location: trsf.transforms(&self.location),
            radius: self.radius * trsf.scale.abs(),
        }
    }

    pub fn is_closed(&self, tolerance: StandardReal) -> bool {
        self.radius <= tolerance
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

    pub fn uper(&self) -> StandardReal {
        2.0 * std::f64::consts::PI
    }

    pub fn vper(&self) -> StandardReal {
        std::f64::consts::PI
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            location: Point::origin(),
            radius: 1.0,
        }
    }
}

impl crate::topology::topods_face::Surface for Sphere {
    fn value(&self, u: f64, v: f64) -> Point {
        self.position(u, v)
    }

    fn normal(&self, u: f64, v: f64) -> crate::geometry::Vector {
        let point = self.position(u, v);
        let center_to_point = crate::geometry::Vector::from_point(&self.location, &point);
        center_to_point.normalized()
    }

    fn parameter_range(&self) -> ((f64, f64), (f64, f64)) {
        ((0.0, 2.0 * std::f64::consts::PI), (0.0, std::f64::consts::PI))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_creation() {
        let center = Point::origin();
        let radius =5.0;
        let sphere = Sphere::new(center, radius);
        assert_eq!(sphere.location(), &center);
        assert_eq!(sphere.radius(), radius);
    }

    #[test]
    fn test_sphere_from_center_radius() {
        let center = Point::origin();
        let radius =5.0;
        let sphere = Sphere::from_center_radius(center, radius);
        assert_eq!(sphere.location(), &center);
        assert_eq!(sphere.radius(), radius);
    }

    #[test]
    fn test_sphere_position() {
        let sphere = Sphere::new(Point::origin(),5.0);
        let point = sphere.position(0.0, 0.0);
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
        assert_eq!(point.z, 5.0);
    }

    #[test]
    fn test_sphere_contains() {
        let sphere = Sphere::new(Point::origin(),5.0);
        let point = Point::new(0.0, 0.0, 5.0);
        assert!(sphere.contains(&point, 0.001));
    }

    #[test]
    fn test_sphere_area() {
        let sphere = Sphere::new(Point::origin(),5.0);
        let area = sphere.area();
        assert!((area - 100.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_sphere_volume() {
        let sphere = Sphere::new(Point::origin(),5.0);
        let volume = sphere.volume();
        assert!((volume - (500.0 / 3.0) * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_sphere_translate() {
        let mut sphere = Sphere::new(Point::origin(),5.0);
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        sphere.translate(&vec);
        assert_eq!(sphere.location(), &Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_sphere_scale() {
        let mut sphere = Sphere::new(Point::origin(),5.0);
        let point = Point::origin();
        sphere.scale(&point, 2.0);
        assert_eq!(sphere.radius(), 10.0);
    }

    #[test]
    fn test_sphere_rotate() {
        let mut sphere = Sphere::new(Point::origin(),5.0);
        let axis = Axis::z_axis();
        sphere.rotate(&axis, std::f64::consts::PI / 2.0);
        assert_eq!(sphere.location(), &Point::origin());
    }

    #[test]
    fn test_sphere_axis() {
        let sphere = Sphere::new(Point::origin(),5.0);
        let axis = sphere.axis();
        assert_eq!(axis.location(), &Point::origin());
        assert_eq!(axis.direction(), &Direction::z_axis());
    }

    #[test]
    fn test_sphere_uper() {
        let sphere = Sphere::new(Point::origin(),5.0);
        assert!((sphere.uper() - 2.0 * std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_sphere_vper() {
        let sphere = Sphere::new(Point::origin(),5.0);
        assert!((sphere.vper() - std::f64::consts::PI).abs() < 0.001);
    }
}
