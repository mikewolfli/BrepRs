use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: StandardReal,
    pub y: StandardReal,
    pub z: StandardReal,
    pub w: StandardReal,
}

impl Quaternion {
    pub fn new(x: StandardReal, y: StandardReal, z: StandardReal, w: StandardReal) -> Self {
        Self { x, y, z, w }
    }

    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    pub fn from_axis_angle(axis: &crate::geometry::Axis, angle: StandardReal) -> Self {
        let half_angle = angle / 2.0;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();

        Self {
            x: axis.direction.x * sin_half,
            y: axis.direction.y * sin_half,
            z: axis.direction.z * sin_half,
            w: cos_half,
        }
    }

    pub fn from_euler_angles(roll: StandardReal, pitch: StandardReal, yaw: StandardReal) -> Self {
        let cy = (yaw / 2.0).cos();
        let sy = (yaw / 2.0).sin();
        let cp = (pitch / 2.0).cos();
        let sp = (pitch / 2.0).sin();
        let cr = (roll / 2.0).cos();
        let sr = (roll / 2.0).sin();

        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }

    pub fn from_rotation_matrix(matrix: &crate::geometry::Matrix) -> Self {
        let trace = matrix.value(0, 0) + matrix.value(1, 1) + matrix.value(2, 2);

        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            let inv_s = 1.0 / s;
            Self {
                w: 0.25 * s,
                x: (matrix.value(2, 1) - matrix.value(1, 2)) * inv_s,
                y: (matrix.value(0, 2) - matrix.value(2, 0)) * inv_s,
                z: (matrix.value(1, 0) - matrix.value(0, 1)) * inv_s,
            }
        } else if (matrix.value(0, 0) > matrix.value(1, 1)) && (matrix.value(0, 0) > matrix.value(2, 2)) {
            let s = (1.0 + matrix.value(0, 0) - matrix.value(1, 1) - matrix.value(2, 2)).sqrt() * 2.0;
            let inv_s = 1.0 / s;
            Self {
                w: (matrix.value(2, 1) - matrix.value(1, 2)) * inv_s,
                x: 0.25 * s,
                y: (matrix.value(0, 1) + matrix.value(1, 0)) * inv_s,
                z: (matrix.value(0, 2) + matrix.value(2, 0)) * inv_s,
            }
        } else if matrix.value(1, 1) > matrix.value(2, 2) {
            let s = (1.0 + matrix.value(1, 1) - matrix.value(0, 0) - matrix.value(2, 2)).sqrt() * 2.0;
            let inv_s = 1.0 / s;
            Self {
                w: (matrix.value(0, 2) - matrix.value(2, 0)) * inv_s,
                x: (matrix.value(0, 1) + matrix.value(1, 0)) * inv_s,
                y: 0.25 * s,
                z: (matrix.value(1, 2) + matrix.value(2, 1)) * inv_s,
            }
        } else {
            let s = (1.0 + matrix.value(2, 2) - matrix.value(0, 0) - matrix.value(1, 1)).sqrt() * 2.0;
            let inv_s = 1.0 / s;
            Self {
                w: (matrix.value(1, 0) - matrix.value(0, 1)) * inv_s,
                x: (matrix.value(0, 2) + matrix.value(2, 0)) * inv_s,
                y: (matrix.value(1, 2) + matrix.value(2, 1)) * inv_s,
                z: 0.25 * s,
            }
        }
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

    pub fn w(&self) -> StandardReal {
        self.w
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

    pub fn set_w(&mut self, w: StandardReal) {
        self.w = w;
    }

    pub fn set_coord(&mut self, x: StandardReal, y: StandardReal, z: StandardReal, w: StandardReal) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
    }

    pub fn magnitude(&self) -> StandardReal {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn square_magnitude(&self) -> StandardReal {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > STANDARD_REAL_EPSILON {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
            self.w /= mag;
        }
    }

    pub fn normalized(&self) -> Quaternion {
        let mag = self.magnitude();
        if mag > STANDARD_REAL_EPSILON {
            Quaternion {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
                w: self.w / mag,
            }
        } else {
            *self
        }
    }

    pub fn conjugate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    pub fn conjugated(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn inverse(&mut self) -> bool {
        let mag_sq = self.square_magnitude();
        if mag_sq < STANDARD_REAL_EPSILON {
            return false;
        }
        let inv_mag_sq = 1.0 / mag_sq;
        self.x = -self.x * inv_mag_sq;
        self.y = -self.y * inv_mag_sq;
        self.z = -self.z * inv_mag_sq;
        self.w = self.w * inv_mag_sq;
        true
    }

    pub fn inverted(&self) -> Option<Quaternion> {
        let mag_sq = self.square_magnitude();
        if mag_sq < STANDARD_REAL_EPSILON {
            None
        } else {
            let inv_mag_sq = 1.0 / mag_sq;
            Some(Quaternion {
                x: -self.x * inv_mag_sq,
                y: -self.y * inv_mag_sq,
                z: -self.z * inv_mag_sq,
                w: self.w * inv_mag_sq,
            })
        }
    }

    pub fn dot(&self, other: &Quaternion) -> StandardReal {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn multiply(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }

    pub fn multiply_vec(&self, vec: &crate::geometry::Vector) -> crate::geometry::Vector {
        let q_vec = Quaternion::new(vec.x, vec.y, vec.z, 0.0);
        let q_conj = self.conjugated();
        let q_temp = self.multiply(&q_vec);
        let result = q_temp.multiply(&q_conj);
        crate::geometry::Vector::new(result.x, result.y, result.z)
    }

    pub fn slerp(&self, other: &Quaternion, t: StandardReal) -> Quaternion {
        let dot = self.dot(other);

        let mut other = *other;
        let dot = if dot < 0.0 {
            other = Quaternion {
                x: -other.x,
                y: -other.y,
                z: -other.z,
                w: -other.w,
            };
            -dot
        } else {
            dot
        };

        if dot > 0.9995 {
            let result = Quaternion {
                x: self.x + t * (other.x - self.x),
                y: self.y + t * (other.y - self.y),
                z: self.z + t * (other.z - self.z),
                w: self.w + t * (other.w - self.w),
            };
            result.normalized()
        } else {
            let theta_0 = dot.acos();
            let theta = theta_0 * t;
            let sin_theta = theta.sin();
            let sin_theta_0 = theta_0.sin();
            let s0 = (theta_0 - theta).cos() - dot * sin_theta / sin_theta_0;
            let s1 = sin_theta / sin_theta_0;

            Quaternion {
                x: s0 * self.x + s1 * other.x,
                y: s0 * self.y + s1 * other.y,
                z: s0 * self.z + s1 * other.z,
                w: s0 * self.w + s1 * other.w,
            }
        }
    }

    pub fn to_matrix(&self) -> crate::geometry::Matrix {
        let xx = self.x * self.x;
        let yy = self.y * self.y;
        let zz = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;

        crate::geometry::Matrix::from_array([
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy)],
            [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx)],
            [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy)],
        ])
    }

    pub fn to_axis_angle(&self) -> (crate::geometry::Axis, StandardReal) {
        let half_angle = self.w.acos();
        let angle = 2.0 * half_angle;
        let sin_half = half_angle.sin();

        if sin_half.abs() < STANDARD_REAL_EPSILON {
            (crate::geometry::Axis::x_axis(), angle)
        } else {
            let scale = 1.0 / sin_half;
            let direction = crate::geometry::Direction::new(
                self.x * scale,
                self.y * scale,
                self.z * scale,
            );
            (crate::geometry::Axis::origin(direction), angle)
        }
    }

    pub fn is_identity(&self, tolerance: StandardReal) -> bool {
        let identity = Quaternion::identity();
        (self.x - identity.x).abs() <= tolerance &&
        (self.y - identity.y).abs() <= tolerance &&
        (self.z - identity.z).abs() <= tolerance &&
        (self.w - identity.w).abs() <= tolerance
    }

    pub fn is_equal(&self, other: &Quaternion, tolerance: StandardReal) -> bool {
        (self.x - other.x).abs() <= tolerance &&
        (self.y - other.y).abs() <= tolerance &&
        (self.z - other.z).abs() <= tolerance &&
        (self.w - other.w).abs() <= tolerance
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
    }

    pub fn negated(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl std::ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Self::Output {
        self.multiply(&other)
    }
}

impl std::ops::Mul<crate::geometry::Vector> for Quaternion {
    type Output = crate::geometry::Vector;

    fn mul(self, vec: crate::geometry::Vector) -> Self::Output {
        self.multiply_vec(&vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quaternion_creation() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(q.x(), 1.0);
        assert_eq!(q.y(), 2.0);
        assert_eq!(q.z(), 3.0);
        assert_eq!(q.w(), 4.0);
    }

    #[test]
    fn test_quaternion_identity() {
        let q = Quaternion::identity();
        assert!(q.is_identity(0.001));
    }

    #[test]
    fn test_quaternion_zero() {
        let q = Quaternion::zero();
        assert_eq!(q.x(), 0.0);
        assert_eq!(q.y(), 0.0);
        assert_eq!(q.z(), 0.0);
        assert_eq!(q.w(), 0.0);
    }

    #[test]
    fn test_quaternion_from_axis_angle() {
        let axis = crate::geometry::Axis::z_axis();
        let q = Quaternion::from_axis_angle(&axis, std::f64::consts::PI / 2.0);
        let mag = q.magnitude();
        assert!((mag - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_from_euler_angles() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, 0.0);
        assert!(q.is_identity(0.001));
    }

    #[test]
    fn test_quaternion_normalize() {
        let mut q = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        q.normalize();
        assert!((q.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_conjugate() {
        let mut q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        q.conjugate();
        assert_eq!(q.x(), -1.0);
        assert_eq!(q.y(), -2.0);
        assert_eq!(q.z(), -3.0);
        assert_eq!(q.w(), 4.0);
    }

    #[test]
    fn test_quaternion_dot() {
        let q1 = Quaternion::identity();
        let q2 = Quaternion::identity();
        assert!((q1.dot(&q2) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_multiply() {
        let q1 = Quaternion::identity();
        let q2 = Quaternion::identity();
        let result = q1.multiply(&q2);
        assert!(result.is_identity(0.001));
    }

    #[test]
    fn test_quaternion_multiply_vec() {
        let axis = crate::geometry::Axis::z_axis();
        let q = Quaternion::from_axis_angle(&axis, std::f64::consts::PI / 2.0);
        let vec = crate::geometry::Vector::new(1.0, 0.0, 0.0);
        let result = q.multiply_vec(&vec);
        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - 1.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_to_matrix() {
        let q = Quaternion::identity();
        let mat = q.to_matrix();
        assert!(mat.is_identity(0.001));
    }

    #[test]
    fn test_quaternion_to_axis_angle() {
        let axis = crate::geometry::Axis::z_axis();
        let q = Quaternion::from_axis_angle(&axis, std::f64::consts::PI / 2.0);
        let (result_axis, angle) = q.to_axis_angle();
        assert!((angle - std::f64::consts::PI / 2.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_slerp() {
        let q1 = Quaternion::identity();
        let axis = crate::geometry::Axis::z_axis();
        let q2 = Quaternion::from_axis_angle(&axis, std::f64::consts::PI / 2.0);
        let result = q1.slerp(&q2, 0.5);
        assert!((result.magnitude() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_quaternion_negate() {
        let mut q = Quaternion::new(1.0, 2.0, 3.0, 4.0);
        q.negate();
        assert_eq!(q.x(), -1.0);
        assert_eq!(q.y(), -2.0);
        assert_eq!(q.z(), -3.0);
        assert_eq!(q.w(), -4.0);
    }
}
