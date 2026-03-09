use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: StandardReal,
    pub y: StandardReal,
    pub z: StandardReal,
}

impl Vector {
    pub fn new(x: StandardReal, y: StandardReal, z: StandardReal) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_point(p1: &crate::geometry::Point, p2: &crate::geometry::Point) -> Self {
        Self {
            x: p2.x - p1.x,
            y: p2.y - p1.y,
            z: p2.z - p1.z,
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

    pub fn x(&self) -> StandardReal {
        self.x
    }

    pub fn y(&self) -> StandardReal {
        self.y
    }

    pub fn z(&self) -> StandardReal {
        self.z
    }

    pub fn magnitude(&self) -> StandardReal {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn square_magnitude(&self) -> StandardReal {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_zero(&self, tolerance: StandardReal) -> bool {
        self.magnitude() <= tolerance
    }

    pub fn is_equal(&self, other: &Vector, tolerance: StandardReal) -> bool {
        let diff = *self - *other;
        diff.magnitude() <= tolerance
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > STANDARD_REAL_EPSILON {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    pub fn normalized(&self) -> Vector {
        let mag = self.magnitude();
        if mag > STANDARD_REAL_EPSILON {
            Vector {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }

    pub fn reverse(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn reversed(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn dot(&self, other: &Vector) -> StandardReal {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn cross_mag(&self, other: &Vector) -> StandardReal {
        self.cross(other).magnitude()
    }

    pub fn cross_square_magnitude(&self, other: &Vector) -> StandardReal {
        self.cross(other).square_magnitude()
    }

    pub fn angle(&self, other: &Vector) -> StandardReal {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();
        if mag1 < STANDARD_REAL_EPSILON || mag2 < STANDARD_REAL_EPSILON {
            return 0.0;
        }
        let dot = self.dot(other) / (mag1 * mag2);
        let dot = dot.max(-1.0).min(1.0);
        dot.acos()
    }

    pub fn angle_with_ref(&self, other: &Vector, v_ref: &Vector) -> StandardReal {
        let angle = self.angle(other);
        let cross = self.cross(other);
        if cross.dot(v_ref) >= 0.0 {
            angle
        } else {
            -angle
        }
    }

    pub fn is_parallel(&self, other: &Vector, angular_tolerance: StandardReal) -> bool {
        let angle = self.angle(other);
        angle <= angular_tolerance || (angle - std::f64::consts::PI).abs() <= angular_tolerance
    }

    pub fn is_normal(&self, other: &Vector, angular_tolerance: StandardReal) -> bool {
        let angle = self.angle(other);
        (angle - std::f64::consts::PI / 2.0).abs() <= angular_tolerance
    }

    pub fn is_opposite(&self, other: &Vector, angular_tolerance: StandardReal) -> bool {
        let angle = self.angle(other);
        (angle - std::f64::consts::PI).abs() <= angular_tolerance
    }

    pub fn is_co_linear(
        &self,
        other: &Vector,
        linear_tolerance: StandardReal,
        angular_tolerance: StandardReal,
    ) -> bool {
        self.is_parallel(other, angular_tolerance)
            && self.cross(other).magnitude() <= linear_tolerance
    }

    pub fn mirror(&mut self, _point: &crate::geometry::Point) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn mirrored(&self, _point: &crate::geometry::Point) -> Vector {
        self.reversed()
    }

    pub fn mirror_axis(&mut self, axis: &crate::geometry::Axis) {
        let result = self.mirrored_axis(axis);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn mirrored_axis(&self, axis: &crate::geometry::Axis) -> Vector {
        let direction = &axis.direction;
        let dot = self.dot(&Vector::new(direction.x, direction.y, direction.z));
        let proj = Vector::new(direction.x * dot, direction.y * dot, direction.z * dot);
        Vector {
            x: 2.0 * proj.x - self.x,
            y: 2.0 * proj.y - self.y,
            z: 2.0 * proj.z - self.z,
        }
    }

    pub fn rotate(&mut self, axis: &crate::geometry::Axis, angle: StandardReal) {
        let result = self.rotated(axis, angle);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn rotated(&self, axis: &crate::geometry::Axis, angle: StandardReal) -> Vector {
        let direction = &axis.direction;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let dot = direction.x * self.x + direction.y * self.y + direction.z * self.z;

        let cross_x = direction.y * self.z - direction.z * self.y;
        let cross_y = direction.z * self.x - direction.x * self.z;
        let cross_z = direction.x * self.y - direction.y * self.x;

        Vector {
            x: self.x * cos_a + cross_x * sin_a + direction.x * dot * (1.0 - cos_a),
            y: self.y * cos_a + cross_y * sin_a + direction.y * dot * (1.0 - cos_a),
            z: self.z * cos_a + cross_z * sin_a + direction.z * dot * (1.0 - cos_a),
        }
    }

    pub fn scale(&mut self, factor: StandardReal) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn scaled(&self, factor: StandardReal) -> Vector {
        Vector {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn multiply(&self, scalar: StandardReal) -> Vector {
        self.scaled(scalar)
    }

    pub fn divide(&self, scalar: StandardReal) -> Vector {
        if scalar.abs() > STANDARD_REAL_EPSILON {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        } else {
            *self
        }
    }

    pub fn lerp(&self, other: &Vector, alpha: StandardReal) -> Vector {
        Vector {
            x: self.x * (1.0 - alpha) + other.x * alpha,
            y: self.y * (1.0 - alpha) + other.y * alpha,
            z: self.z * (1.0 - alpha) + other.z * alpha,
        }
    }

    pub fn to_dir(&self) -> crate::geometry::Direction {
        crate::geometry::Direction::new(self.x, self.y, self.z)
    }
}

impl Default for Vector {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<StandardReal> for Vector {
    type Output = Vector;

    fn mul(self, scalar: StandardReal) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Div<StandardReal> for Vector {
    type Output = Vector;

    fn div(self, scalar: StandardReal) -> Self::Output {
        if scalar.abs() > STANDARD_REAL_EPSILON {
            Vector {
                x: self.x / scalar,
                y: self.y / scalar,
                z: self.z / scalar,
            }
        } else {
            self
        }
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        self.reversed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_creation() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vector_zero() {
        let v = Vector::zero();
        assert_eq!(v.x(), 0.0);
        assert_eq!(v.y(), 0.0);
        assert_eq!(v.z(), 0.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_vector_square_magnitude() {
        let v = Vector::new(3.0, 4.0, 0.0);
        assert!((v.square_magnitude() - 25.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_vector_normalize() {
        let mut v = Vector::new(3.0, 4.0, 0.0);
        v.normalize();
        assert!((v.magnitude() - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
    }

    #[test]
    fn test_vector_cross() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let result = v1.cross(&v2);
        assert!((result.x - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.y - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.z - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_vector_angle() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let angle = v1.angle(&v2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_vector_reverse() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v.reverse();
        assert_eq!(v.x(), -1.0);
        assert_eq!(v.y(), -2.0);
        assert_eq!(v.z(), -3.0);
    }

    #[test]
    fn test_vector_scale() {
        let mut v = Vector::new(1.0, 2.0, 3.0);
        v.scale(2.0);
        assert_eq!(v.x(), 2.0);
        assert_eq!(v.y(), 4.0);
        assert_eq!(v.z(), 6.0);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let result = v1.add(&v2);
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 7.0);
        assert_eq!(result.z(), 9.0);
    }

    #[test]
    fn test_vector_subtract() {
        let v1 = Vector::new(4.0, 5.0, 6.0);
        let v2 = Vector::new(1.0, 2.0, 3.0);
        let result = v1.subtract(&v2);
        assert_eq!(result.x(), 3.0);
        assert_eq!(result.y(), 3.0);
        assert_eq!(result.z(), 3.0);
    }

    #[test]
    fn test_vector_lerp() {
        let v1 = Vector::new(0.0, 0.0, 0.0);
        let v2 = Vector::new(10.0, 10.0, 10.0);
        let result = v1.lerp(&v2, 0.5);
        assert_eq!(result.x(), 5.0);
        assert_eq!(result.y(), 5.0);
        assert_eq!(result.z(), 5.0);
    }
}
