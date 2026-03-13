use crate::geometry::traits::{GetCoord, SetCoord};
impl GetCoord for Point {
    fn coord(&self) -> (f64, f64, f64) {
        (self.x as f64, self.y as f64, self.z as f64)
    }
}

impl SetCoord for Point {
    fn set_coord(&mut self, x: f64, y: f64, z: f64) {
        self.x = x as StandardReal;
        self.y = y as StandardReal;
        self.z = z as StandardReal;
    }
}
use crate::foundation::types::StandardReal;
#[cfg(test)]
use crate::foundation::types::STANDARD_REAL_EPSILON;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub x: StandardReal,
    pub y: StandardReal,
    pub z: StandardReal,
}

impl Point {
    pub fn new(x: StandardReal, y: StandardReal, z: StandardReal) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn set_x(&mut self, x: StandardReal) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: StandardReal) {
        self.y = y;
    }

    pub fn set_z(&mut self, z: StandardReal) {
        self.z = z;
    }

    pub fn set_coord(&mut self, x: StandardReal, y: StandardReal, z: StandardReal) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn coord(&self) -> (StandardReal, StandardReal, StandardReal) {
        (self.x, self.y, self.z)
    }

    #[inline]
    pub fn x(&self) -> StandardReal {
        self.x
    }

    #[inline]
    pub fn y(&self) -> StandardReal {
        self.y
    }

    #[inline]
    pub fn z(&self) -> StandardReal {
        self.z
    }

    #[inline]
    pub fn distance(&self, other: &Point) -> StandardReal {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    #[inline]
    pub fn square_distance(&self, other: &Point) -> StandardReal {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    #[inline]
    pub fn is_equal(&self, other: &Point, tolerance: StandardReal) -> bool {
        self.distance(other) <= tolerance
    }

    /// 使用全局容差判断相等
    pub fn is_equal_tol(&self, other: &Point) -> bool {
        self.distance(other) <= crate::geometry::traits::TOLERANCE as StandardReal
    }

    /// Calculate dot product with a vector
    pub fn dot(&self, other: &crate::geometry::Vector) -> StandardReal {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mirror(&mut self, point: &Point) {
        self.x = 2.0 * point.x - self.x;
        self.y = 2.0 * point.y - self.y;
        self.z = 2.0 * point.z - self.z;
    }

    pub fn mirrored(&self, point: &Point) -> Point {
        Point {
            x: 2.0 * point.x - self.x,
            y: 2.0 * point.y - self.y,
            z: 2.0 * point.z - self.z,
        }
    }

    pub fn mirror_axis(&mut self, axis: &crate::geometry::Axis) {
        let result = self.mirrored_axis(axis);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn mirrored_axis(&self, axis: &crate::geometry::Axis) -> Point {
        let origin = &axis.location;
        let direction = &axis.direction;

        let p = *self - *origin;
        let dot = direction.x * p.x + direction.y * p.y + direction.z * p.z;

        let proj =
            crate::geometry::Vector::new(direction.x * dot, direction.y * dot, direction.z * dot);

        let result = crate::geometry::Vector::new(
            2.0 * proj.x - p.x,
            2.0 * proj.y - p.y,
            2.0 * proj.z - p.z,
        );

        Point {
            x: result.x + origin.x,
            y: result.y + origin.y,
            z: result.z + origin.z,
        }
    }

    pub fn rotate(&mut self, axis: &crate::geometry::Axis, angle: StandardReal) {
        let result = self.rotated(axis, angle);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn rotated(&self, axis: &crate::geometry::Axis, angle: StandardReal) -> Point {
        let origin = &axis.location;
        let direction = &axis.direction;

        let p = *self - *origin;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let dot = direction.x * p.x + direction.y * p.y + direction.z * p.z;

        let cross_x = direction.y * p.z - direction.z * p.y;
        let cross_y = direction.z * p.x - direction.x * p.z;
        let cross_z = direction.x * p.y - direction.y * p.x;

        let x = p.x * cos_a + cross_x * sin_a + direction.x * dot * (1.0 - cos_a);
        let y = p.y * cos_a + cross_y * sin_a + direction.y * dot * (1.0 - cos_a);
        let z = p.z * cos_a + cross_z * sin_a + direction.z * dot * (1.0 - cos_a);

        Point {
            x: x + origin.x,
            y: y + origin.y,
            z: z + origin.z,
        }
    }

    pub fn scale(&mut self, point: &Point, factor: StandardReal) {
        self.x = point.x + factor * (self.x - point.x);
        self.y = point.y + factor * (self.y - point.y);
        self.z = point.z + factor * (self.z - point.z);
    }

    pub fn scaled(&self, point: &Point, factor: StandardReal) -> Point {
        Point {
            x: point.x + factor * (self.x - point.x),
            y: point.y + factor * (self.y - point.y),
            z: point.z + factor * (self.z - point.z),
        }
    }

    pub fn translate(&mut self, vec: &crate::geometry::Vector) {
        self.x += vec.x;
        self.y += vec.y;
        self.z += vec.z;
    }

    pub fn translated(&self, vec: &crate::geometry::Vector) -> Point {
        Point {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }

    pub fn translate_by_coords(&mut self, dx: StandardReal, dy: StandardReal, dz: StandardReal) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    pub fn translated_by_coords(
        &self,
        dx: StandardReal,
        dy: StandardReal,
        dz: StandardReal,
    ) -> Point {
        Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    pub fn transform(&mut self, transform: &crate::geometry::Transform) {
        let result = self.transformed(transform);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn transformed(&self, transform: &crate::geometry::Transform) -> Point {
        let matrix = &transform.rotation.data;
        let translation = &transform.translation;
        let scale = transform.scale;

        let x = scale * (matrix[0][0] * self.x + matrix[0][1] * self.y + matrix[0][2] * self.z)
            + translation.x;
        let y = scale * (matrix[1][0] * self.x + matrix[1][1] * self.y + matrix[1][2] * self.z)
            + translation.y;
        let z = scale * (matrix[2][0] * self.x + matrix[2][1] * self.y + matrix[2][2] * self.z)
            + translation.z;

        Point { x, y, z }
    }

    pub fn add(&self, vec: &crate::geometry::Vector) -> Point {
        self.translated(vec)
    }

    pub fn subtract(&self, other: &Point) -> crate::geometry::Vector {
        crate::geometry::Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Compute distance to another point
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Compute vector from this point to another point
    pub fn sub(&self, other: &Point) -> crate::geometry::Vector {
        self.subtract(other)
    }

    pub fn barycenter(&self, other: &Point, alpha: StandardReal) -> Point {
        Point {
            x: self.x * (1.0 - alpha) + other.x * alpha,
            y: self.y * (1.0 - alpha) + other.y * alpha,
            z: self.z * (1.0 - alpha) + other.z * alpha,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self::origin()
    }
}

impl std::ops::Add<crate::geometry::Vector> for Point {
    type Output = Point;

    fn add(self, vec: crate::geometry::Vector) -> Self::Output {
        Point {
            x: self.x + vec.x,
            y: self.y + vec.y,
            z: self.z + vec.z,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = crate::geometry::Vector;

    fn sub(self, other: Point) -> Self::Output {
        crate::geometry::Vector::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
    }

    #[test]
    fn test_point_origin() {
        let p = Point::origin();
        assert_eq!(p.x(), 0.0);
        assert_eq!(p.y(), 0.0);
        assert_eq!(p.z(), 0.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        assert!((p1.distance(&p2) - 5.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_point_square_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        assert!((p1.square_distance(&p2) - 25.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_point_is_equal() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0001, 2.0001, 3.0001);
        assert!(p1.is_equal(&p2, 0.001));
        assert!(!p1.is_equal(&p2, 0.00001));
    }

    #[test]
    fn test_point_mirror() {
        let mut p = Point::new(1.0, 2.0, 3.0);
        let origin = Point::origin();
        p.mirror(&origin);
        assert_eq!(p.x(), -1.0);
        assert_eq!(p.y(), -2.0);
        assert_eq!(p.z(), -3.0);
    }

    #[test]
    fn test_point_scale() {
        let mut p = Point::new(2.0, 4.0, 6.0);
        let origin = Point::origin();
        p.scale(&origin, 2.0);
        assert_eq!(p.x(), 4.0);
        assert_eq!(p.y(), 8.0);
        assert_eq!(p.z(), 12.0);
    }

    #[test]
    fn test_point_barycenter() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(10.0, 10.0, 10.0);
        let p = p1.barycenter(&p2, 0.5);
        assert_eq!(p.x(), 5.0);
        assert_eq!(p.y(), 5.0);
        assert_eq!(p.z(), 5.0);
    }
}
