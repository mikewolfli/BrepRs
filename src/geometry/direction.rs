use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction {
    pub x: StandardReal,
    pub y: StandardReal,
    pub z: StandardReal,
}

impl Direction {
    pub fn new(x: StandardReal, y: StandardReal, z: StandardReal) -> Self {
        let mut dir = Self { x, y, z };
        dir.normalize();
        dir
    }

    pub fn from_vector(vec: &crate::geometry::Vector) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
        .normalized()
    }

    pub fn x_axis() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn y_axis() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }

    pub fn z_axis() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
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

    pub fn coord(&self) -> (StandardReal, StandardReal, StandardReal) {
        (self.x, self.y, self.z)
    }

    pub fn set_x(&mut self, x: StandardReal) {
        self.x = x;
        self.normalize();
    }

    pub fn set_y(&mut self, y: StandardReal) {
        self.y = y;
        self.normalize();
    }

    pub fn set_z(&mut self, z: StandardReal) {
        self.z = z;
        self.normalize();
    }

    pub fn set_coord(&mut self, x: StandardReal, y: StandardReal, z: StandardReal) {
        self.x = x;
        self.y = y;
        self.z = z;
        self.normalize();
    }

    pub fn normalize(&mut self) {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if mag > STANDARD_REAL_EPSILON {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    pub fn normalized(&self) -> Direction {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if mag > STANDARD_REAL_EPSILON {
            Direction {
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

    pub fn reversed(&self) -> Direction {
        Direction {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn dot(&self, other: &Direction) -> StandardReal {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Direction) -> Direction {
        Direction {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
        .normalized()
    }

    pub fn cross_mag(&self, other: &Direction) -> StandardReal {
        let cross = self.cross(other);
        (cross.x * cross.x + cross.y * cross.y + cross.z * cross.z).sqrt()
    }

    pub fn cross_square_magnitude(&self, other: &Direction) -> StandardReal {
        let cross = self.cross(other);
        cross.x * cross.x + cross.y * cross.y + cross.z * cross.z
    }

    pub fn angle(&self, other: &Direction) -> StandardReal {
        let dot = self.dot(other);
        let dot = dot.max(-1.0).min(1.0);
        dot.acos()
    }

    pub fn angle_with_ref(&self, other: &Direction, v_ref: &Direction) -> StandardReal {
        let angle = self.angle(other);
        let cross = self.cross(other);
        if cross.dot(v_ref) >= 0.0 {
            angle
        } else {
            -angle
        }
    }

    pub fn is_equal(&self, other: &Direction, angular_tolerance: StandardReal) -> bool {
        self.angle(other) <= angular_tolerance
    }

    pub fn is_opposite(&self, other: &Direction, angular_tolerance: StandardReal) -> bool {
        (self.angle(other) - std::f64::consts::PI).abs() <= angular_tolerance
    }

    pub fn is_parallel(&self, other: &Direction, angular_tolerance: StandardReal) -> bool {
        self.is_equal(other, angular_tolerance) || self.is_opposite(other, angular_tolerance)
    }

    pub fn is_normal(&self, other: &Direction, angular_tolerance: StandardReal) -> bool {
        let angle = self.angle(other);
        (angle - std::f64::consts::PI / 2.0).abs() <= angular_tolerance
    }

    pub fn is_co_linear(&self, other: &Direction, angular_tolerance: StandardReal) -> bool {
        self.is_parallel(other, angular_tolerance)
    }

    pub fn mirror(&mut self, _point: &crate::geometry::Point) {
        self.reverse();
    }

    pub fn mirrored(&self, _point: &crate::geometry::Point) -> Direction {
        self.reversed()
    }

    pub fn mirror_axis(&mut self, axis: &crate::geometry::Axis) {
        let result = self.mirrored_axis(axis);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn mirrored_axis(&self, axis: &crate::geometry::Axis) -> Direction {
        let direction = &axis.direction;
        let dot = self.x * direction.x + self.y * direction.y + self.z * direction.z;
        let proj = Direction {
            x: direction.x * dot,
            y: direction.y * dot,
            z: direction.z * dot,
        };
        Direction {
            x: 2.0 * proj.x - self.x,
            y: 2.0 * proj.y - self.y,
            z: 2.0 * proj.z - self.z,
        }
        .normalized()
    }

    pub fn rotate(&mut self, axis: &crate::geometry::Axis, angle: StandardReal) {
        let result = self.rotated(axis, angle);
        self.x = result.x;
        self.y = result.y;
        self.z = result.z;
    }

    pub fn rotated(&self, axis: &crate::geometry::Axis, angle: StandardReal) -> Direction {
        let direction = &axis.direction;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let dot = direction.x * self.x + direction.y * self.y + direction.z * self.z;

        let cross_x = direction.y * self.z - direction.z * self.y;
        let cross_y = direction.z * self.x - direction.x * self.z;
        let cross_z = direction.x * self.y - direction.y * self.x;

        Direction {
            x: self.x * cos_a + cross_x * sin_a + direction.x * dot * (1.0 - cos_a),
            y: self.y * cos_a + cross_y * sin_a + direction.y * dot * (1.0 - cos_a),
            z: self.z * cos_a + cross_z * sin_a + direction.z * dot * (1.0 - cos_a),
        }
        .normalized()
    }

    pub fn to_vec(&self) -> crate::geometry::Vector {
        crate::geometry::Vector::new(self.x, self.y, self.z)
    }

    pub fn coord_ref(&self) -> &Direction {
        self
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::x_axis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_creation() {
        let dir = Direction::new(1.0, 0.0, 0.0);
        assert!((dir.x() - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((dir.y() - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((dir.z() - 0.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_normalize() {
        let dir = Direction::new(3.0, 4.0, 0.0);
        let mag = (dir.x * dir.x + dir.y * dir.y + dir.z * dir.z).sqrt();
        assert!((mag - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_axes() {
        let x_axis = Direction::x_axis();
        let y_axis = Direction::y_axis();
        let z_axis = Direction::z_axis();

        assert!((x_axis.x() - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((y_axis.y() - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((z_axis.z() - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_dot() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::y_axis();
        assert!((dir1.dot(&dir2) - 0.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_cross() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::y_axis();
        let result = dir1.cross(&dir2);
        assert!((result.x() - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.y() - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.z() - 1.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_angle() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::y_axis();
        let angle = dir1.angle(&dir2);
        assert!((angle - std::f64::consts::PI / 2.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_reverse() {
        let mut dir = Direction::x_axis();
        dir.reverse();
        assert!((dir.x() - (-1.0)).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_direction_is_equal() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::x_axis();
        assert!(dir1.is_equal(&dir2, 0.001));
    }

    #[test]
    fn test_direction_is_opposite() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::new(-1.0, 0.0, 0.0);
        assert!(dir1.is_opposite(&dir2, 0.001));
    }

    #[test]
    fn test_direction_is_normal() {
        let dir1 = Direction::x_axis();
        let dir2 = Direction::y_axis();
        assert!(dir1.is_normal(&dir2, 0.001));
    }

    #[test]
    fn test_direction_to_vec() {
        let dir = Direction::x_axis();
        let vec = dir.to_vec();
        assert!((vec.x() - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((vec.y() - 0.0).abs() < STANDARD_REAL_EPSILON);
        assert!((vec.z() - 0.0).abs() < STANDARD_REAL_EPSILON);
    }
}
