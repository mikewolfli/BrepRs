use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector, Direction, Axis, Matrix, Quaternion};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub scale: Standard_Real,
    pub translation: Vector,
    pub rotation: Matrix,
    pub shape: TrsfForm,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrsfForm {
    Identity,
    Translation,
    Rotation,
    Scale,
    PntMirror,
    Ax1Mirror,
    Ax2Mirror,
    Compound,
    Other,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            translation: Vector::zero(),
            rotation: Matrix::identity(),
            shape: TrsfForm::Identity,
        }
    }

    pub fn identity() -> Self {
        Self::new()
    }

    pub fn from_scale(scale: Standard_Real) -> Self {
        Self {
            scale,
            translation: Vector::zero(),
            rotation: Matrix::identity(),
            shape: TrsfForm::Scale,
        }
    }

    pub fn from_translation(vec: &Vector) -> Self {
        Self {
            scale: 1.0,
            translation: *vec,
            rotation: Matrix::identity(),
            shape: TrsfForm::Translation,
        }
    }

    pub fn from_rotation(axis: &Axis, angle: Standard_Real) -> Self {
        let quaternion = Quaternion::from_axis_angle(axis, angle);
        Self {
            scale: 1.0,
            translation: Vector::zero(),
            rotation: quaternion.to_matrix(),
            shape: TrsfForm::Rotation,
        }
    }

    pub fn from_point_mirror(point: &Point) -> Self {
        let mut trsf = Self::from_scale(-1.0);
        trsf.translation = Vector::new(2.0 * point.x, 2.0 * point.y, 2.0 * point.z);
        trsf.shape = TrsfForm::PntMirror;
        trsf
    }

    pub fn from_axis_mirror(axis: &Axis) -> Self {
        let dir = axis.direction();
        let loc = axis.location();
        let d = Vector::new(dir.x, dir.y, dir.z);
        let p = Vector::new(loc.x, loc.y, loc.z);

        let mut trsf = Self::new();
        let dd = d.dot(&d);
        let xx = d.x * d.x / dd;
        let yy = d.y * d.y / dd;
        let zz = d.z * d.z / dd;
        let xy = d.x * d.y / dd;
        let xz = d.x * d.z / dd;
        let yz = d.y * d.z / dd;

        trsf.rotation = Matrix::from_array([
            [2.0 * xx - 1.0, 2.0 * xy, 2.0 * xz],
            [2.0 * xy, 2.0 * yy - 1.0, 2.0 * yz],
            [2.0 * xz, 2.0 * yz, 2.0 * zz - 1.0],
        ]);

        let dp = d.x * p.x + d.y * p.y + d.z * p.z;
        trsf.translation = Vector::new(
            2.0 * (dp * d.x / dd - p.x),
            2.0 * (dp * d.y / dd - p.y),
            2.0 * (dp * d.z / dd - p.z),
        );

        trsf.shape = TrsfForm::Ax1Mirror;
        trsf
    }

    pub fn from_matrix(matrix: &Matrix) -> Self {
        Self {
            scale: 1.0,
            translation: Vector::zero(),
            rotation: *matrix,
            shape: TrsfForm::Rotation,
        }
    }

    pub fn from_quaternion(quaternion: &Quaternion) -> Self {
        Self {
            scale: 1.0,
            translation: Vector::zero(),
            rotation: quaternion.to_matrix(),
            shape: TrsfForm::Rotation,
        }
    }

    pub fn from_axis(axis: &Axis) -> Self {
        let dir = axis.direction();
        let loc = axis.location();
        let mut trsf = Self::from_rotation(axis, 0.0);
        trsf.translation = Vector::new(loc.x, loc.y, loc.z);
        trsf
    }

    pub fn from_direction(direction: &Direction) -> Self {
        Self::new()
    }

    pub fn scale(&self) -> Standard_Real {
        self.scale
    }

    pub fn translation(&self) -> &Vector {
        &self.translation
    }

    pub fn rotation(&self) -> &Matrix {
        &self.rotation
    }

    pub fn form(&self) -> TrsfForm {
        self.shape
    }

    pub fn set_scale(&mut self, scale: Standard_Real) {
        self.scale = scale;
        self.shape = TrsfForm::Scale;
    }

    pub fn set_translation(&mut self, vec: &Vector) {
        self.translation = *vec;
        self.shape = TrsfForm::Translation;
    }

    pub fn set_rotation(&mut self, matrix: &Matrix) {
        self.rotation = *matrix;
        self.shape = TrsfForm::Rotation;
    }

    pub fn set_form(&mut self, form: TrsfForm) {
        self.shape = form;
    }

    pub fn invert(&mut self) {
        if self.is_singular() {
            return;
        }

        let inv_scale = 1.0 / self.scale;
        let mut inv_rotation = self.rotation;
        inv_rotation.transpose();

        let inv_translation = inv_rotation.multiply_vec(&self.translation).scaled(-inv_scale);

        self.scale = inv_scale;
        self.rotation = inv_rotation;
        self.translation = inv_translation;
    }

    pub fn inverted(&self) -> Transform {
        let mut result = *self;
        result.invert();
        result
    }

    pub fn multiply(&self, other: &Transform) -> Transform {
        Transform {
            scale: self.scale * other.scale,
            translation: self.rotation.multiply_vec(&other.translation).scaled(self.scale) + self.translation,
            rotation: self.rotation.multiply(&other.rotation),
            shape: TrsfForm::Compound,
        }
    }

    pub fn pre_multiply(&mut self, other: &Transform) {
        *self = other.multiply(self);
    }

    pub fn power(&self, n: i32) -> Transform {
        if n == 0 {
            Transform::identity()
        } else if n == 1 {
            *self
        } else if n > 0 {
            let mut result = Transform::identity();
            for _ in 0..n {
                result = result.multiply(self);
            }
            result
        } else {
            self.inverted().power(-n)
        }
    }

    pub fn transforms(&self, point: &Point) -> Point {
        let rotated = self.rotation.multiply_vec(&Vector::new(point.x, point.y, point.z));
        let scaled = rotated.scaled(self.scale);
        Point::new(
            scaled.x + self.translation.x,
            scaled.y + self.translation.y,
            scaled.z + self.translation.z,
        )
    }

    pub fn transforms_vec(&self, vec: &Vector) -> Vector {
        let rotated = self.rotation.multiply_vec(vec);
        rotated.scaled(self.scale)
    }

    pub fn transforms_dir(&self, dir: &Direction) -> Direction {
        let vec = self.rotation.multiply_vec(&Vector::new(dir.x, dir.y, dir.z));
        Direction::new(vec.x, vec.y, vec.z)
    }

    pub fn is_identity(&self) -> bool {
        self.shape == TrsfForm::Identity
    }

    pub fn is_singular(&self) -> bool {
        self.scale.abs() < STANDARD_REAL_EPSILON
    }

    pub fn is_rotation(&self) -> bool {
        self.shape == TrsfForm::Rotation
    }

    pub fn is_translation(&self) -> bool {
        self.shape == TrsfForm::Translation
    }

    pub fn is_scale(&self) -> bool {
        self.shape == TrsfForm::Scale
    }

    pub fn translation_part(&self) -> Vector {
        self.translation
    }

    pub fn scale_factor(&self) -> Standard_Real {
        self.scale
    }

    pub fn rotation_part(&self) -> Matrix {
        self.rotation
    }

    pub fn h_vectorial_part(&self) -> Matrix {
        let mut result = self.rotation;
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] *= self.scale;
            }
        }
        result
    }

    pub fn set_values(&mut self, scale: Standard_Real, translation: &Vector, rotation: &Matrix) {
        self.scale = scale;
        self.translation = *translation;
        self.rotation = *rotation;
        self.shape = TrsfForm::Other;
    }

    pub fn set_values_3d(&mut self, a11: Standard_Real, a12: Standard_Real, a13: Standard_Real,
                          a21: Standard_Real, a22: Standard_Real, a23: Standard_Real,
                          a31: Standard_Real, a32: Standard_Real, a33: Standard_Real,
                          tx: Standard_Real, ty: Standard_Real, tz: Standard_Real) {
        self.rotation = Matrix::from_array([
            [a11, a12, a13],
            [a21, a22, a23],
            [a31, a32, a33],
        ]);
        self.translation = Vector::new(tx, ty, tz);
        self.scale = 1.0;
        self.shape = TrsfForm::Other;
    }

    pub fn set_rotation_part(&mut self, matrix: &Matrix) {
        self.rotation = *matrix;
    }

    pub fn set_translation_part(&mut self, vec: &Vector) {
        self.translation = *vec;
    }

    pub fn set_scale_factor(&mut self, scale: Standard_Real) {
        self.scale = scale;
    }

    pub fn to_matrix(&self) -> [[Standard_Real; 4]; 4] {
        let mut result = [[0.0; 4]; 4];
        for i in 0..3 {
            for j in 0..3 {
                result[i][j] = self.rotation.data[i][j] * self.scale;
            }
        }
        result[0][3] = self.translation.x;
        result[1][3] = self.translation.y;
        result[2][3] = self.translation.z;
        result[3][3] = 1.0;
        result
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Self::Output {
        self.multiply(&other)
    }
}

impl std::ops::Mul<Point> for Transform {
    type Output = Point;

    fn mul(self, point: Point) -> Self::Output {
        self.transforms(&point)
    }
}

impl std::ops::Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, vec: Vector) -> Self::Output {
        self.transforms_vec(&vec)
    }
}

impl std::ops::Mul<Direction> for Transform {
    type Output = Direction;

    fn mul(self, dir: Direction) -> Self::Output {
        self.transforms_dir(&dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_identity() {
        let trsf = Transform::identity();
        assert!(trsf.is_identity());
        let p = Point::new(1.0, 2.0, 3.0);
        let result = trsf.transforms(&p);
        assert_eq!(result.x, p.x);
        assert_eq!(result.y, p.y);
        assert_eq!(result.z, p.z);
    }

    #[test]
    fn test_transform_translation() {
        let vec = Vector::new(1.0, 2.0, 3.0);
        let trsf = Transform::from_translation(&vec);
        assert!(trsf.is_translation());
        let p = Point::origin();
        let result = trsf.transforms(&p);
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, 3.0);
    }

    #[test]
    fn test_transform_scale() {
        let trsf = Transform::from_scale(2.0);
        assert!(trsf.is_scale());
        let p = Point::new(1.0, 2.0, 3.0);
        let result = trsf.transforms(&p);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_transform_rotation() {
        let axis = Axis::z_axis();
        let trsf = Transform::from_rotation(&axis, std::f64::consts::PI / 2.0);
        assert!(trsf.is_rotation());
        let p = Point::new(1.0, 0.0, 0.0);
        let result = trsf.transforms(&p);
        assert!((result.x - 0.0).abs() < 0.001);
        assert!((result.y - 1.0).abs() < 0.001);
        assert!((result.z - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_transform_point_mirror() {
        let point = Point::origin();
        let trsf = Transform::from_point_mirror(&point);
        let p = Point::new(1.0, 2.0, 3.0);
        let result = trsf.transforms(&p);
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_transform_multiply() {
        let trsf1 = Transform::from_scale(2.0);
        let trsf2 = Transform::from_translation(&Vector::new(1.0, 2.0, 3.0));
        let result = trsf1.multiply(&trsf2);
        assert_eq!(result.scale(), 2.0);
    }

    #[test]
    fn test_transform_invert() {
        let vec = Vector::new(1.0, 2.0, 3.0);
        let mut trsf = Transform::from_translation(&vec);
        trsf.invert();
        let p = Point::new(1.0, 2.0, 3.0);
        let result = trsf.transforms(&p);
        assert_eq!(result.x, 0.0);
        assert_eq!(result.y, 0.0);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn test_transform_power() {
        let trsf = Transform::from_scale(2.0);
        let result = trsf.power(3);
        assert_eq!(result.scale(), 8.0);
    }

    #[test]
    fn test_transform_transforms_vec() {
        let trsf = Transform::from_scale(2.0);
        let vec = Vector::new(1.0, 2.0, 3.0);
        let result = trsf.transforms_vec(&vec);
        assert_eq!(result.x, 2.0);
        assert_eq!(result.y, 4.0);
        assert_eq!(result.z, 6.0);
    }

    #[test]
    fn test_transform_transforms_dir() {
        let axis = Axis::z_axis();
        let trsf = Transform::from_rotation(&axis, std::f64::consts::PI / 2.0);
        let dir = Direction::x_axis();
        let result = trsf.transforms_dir(&dir);
        assert!((result.x() - 0.0).abs() < 0.001);
        assert!((result.y() - 1.0).abs() < 0.001);
        assert!((result.z() - 0.0).abs() < 0.001);
    }
}
