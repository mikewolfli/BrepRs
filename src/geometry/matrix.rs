use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub data: [[Standard_Real; 3]; 3],
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_array(data: [[Standard_Real; 3]; 3]) -> Self {
        Self { data }
    }

    pub fn from_row_major(row1: [Standard_Real; 3], row2: [Standard_Real; 3], row3: [Standard_Real; 3]) -> Self {
        Self {
            data: [row1, row2, row3],
        }
    }

    pub fn from_column_major(col1: [Standard_Real; 3], col2: [Standard_Real; 3], col3: [Standard_Real; 3]) -> Self {
        Self {
            data: [
                [col1[0], col2[0], col3[0]],
                [col1[1], col2[1], col3[1]],
                [col1[2], col2[2], col3[2]],
            ],
        }
    }

    pub fn identity() -> Self {
        Self::new()
    }

    pub fn zero() -> Self {
        Self {
            data: [[0.0; 3]; 3],
        }
    }

    pub fn row(&self, row: usize) -> [Standard_Real; 3] {
        self.data[row]
    }

    pub fn col(&self, col: usize) -> [Standard_Real; 3] {
        [
            self.data[0][col],
            self.data[1][col],
            self.data[2][col],
        ]
    }

    pub fn value(&self, row: usize, col: usize) -> Standard_Real {
        self.data[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: Standard_Real) {
        self.data[row][col] = value;
    }

    pub fn set_row(&mut self, row: usize, values: [Standard_Real; 3]) {
        self.data[row] = values;
    }

    pub fn set_col(&mut self, col: usize, values: [Standard_Real; 3]) {
        self.data[0][col] = values[0];
        self.data[1][col] = values[1];
        self.data[2][col] = values[2];
    }

    pub fn determinant(&self) -> Standard_Real {
        let a = self.data[0][0];
        let b = self.data[0][1];
        let c = self.data[0][2];
        let d = self.data[1][0];
        let e = self.data[1][1];
        let f = self.data[1][2];
        let g = self.data[2][0];
        let h = self.data[2][1];
        let i = self.data[2][2];

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    pub fn transpose(&mut self) {
        let temp = self.data;
        self.data[0][0] = temp[0][0];
        self.data[0][1] = temp[1][0];
        self.data[0][2] = temp[2][0];
        self.data[1][0] = temp[0][1];
        self.data[1][1] = temp[1][1];
        self.data[1][2] = temp[2][1];
        self.data[2][0] = temp[0][2];
        self.data[2][1] = temp[1][2];
        self.data[2][2] = temp[2][2];
    }

    pub fn transposed(&self) -> Matrix {
        Matrix {
            data: [
                [self.data[0][0], self.data[1][0], self.data[2][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2]],
            ],
        }
    }

    pub fn invert(&mut self) -> bool {
        let det = self.determinant();
        if det.abs() < STANDARD_REAL_EPSILON {
            return false;
        }

        let inv_det = 1.0 / det;
        let temp = self.data;

        self.data[0][0] = (temp[1][1] * temp[2][2] - temp[1][2] * temp[2][1]) * inv_det;
        self.data[0][1] = (temp[0][2] * temp[2][1] - temp[0][1] * temp[2][2]) * inv_det;
        self.data[0][2] = (temp[0][1] * temp[1][2] - temp[0][2] * temp[1][1]) * inv_det;
        self.data[1][0] = (temp[1][2] * temp[2][0] - temp[1][0] * temp[2][2]) * inv_det;
        self.data[1][1] = (temp[0][0] * temp[2][2] - temp[0][2] * temp[2][0]) * inv_det;
        self.data[1][2] = (temp[0][2] * temp[1][0] - temp[0][0] * temp[1][2]) * inv_det;
        self.data[2][0] = (temp[1][0] * temp[2][1] - temp[1][1] * temp[2][0]) * inv_det;
        self.data[2][1] = (temp[0][1] * temp[2][0] - temp[0][0] * temp[2][1]) * inv_det;
        self.data[2][2] = (temp[0][0] * temp[1][1] - temp[0][1] * temp[1][0]) * inv_det;

        true
    }

    pub fn inverted(&self) -> Option<Matrix> {
        let mut result = *self;
        if result.invert() {
            Some(result)
        } else {
            None
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::zero();
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn multiply_vec(&self, vec: &crate::geometry::Vector) -> crate::geometry::Vector {
        crate::geometry::Vector::new(
            self.data[0][0] * vec.x + self.data[0][1] * vec.y + self.data[0][2] * vec.z,
            self.data[1][0] * vec.x + self.data[1][1] * vec.y + self.data[1][2] * vec.z,
            self.data[2][0] * vec.x + self.data[2][1] * vec.y + self.data[2][2] * vec.z,
        )
    }

    pub fn multiply_xyz(&self, x: Standard_Real, y: Standard_Real, z: Standard_Real) -> (Standard_Real, Standard_Real, Standard_Real) {
        let new_x = self.data[0][0] * x + self.data[0][1] * y + self.data[0][2] * z;
        let new_y = self.data[1][0] * x + self.data[1][1] * y + self.data[1][2] * z;
        let new_z = self.data[2][0] * x + self.data[2][1] * y + self.data[2][2] * z;
        (new_x, new_y, new_z)
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::zero();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::zero();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        result
    }

    pub fn scale(&mut self, factor: Standard_Real) {
        for i in 0..3 {
            for j in 0..3 {
                self.data[i][j] *= factor;
            }
        }
    }

    pub fn scaled(&self, factor: Standard_Real) -> Matrix {
        let mut result = *self;
        result.scale(factor);
        result
    }

    pub fn power(&self, n: i32) -> Matrix {
        if n < 0 {
            let inv = self.inverted();
            match inv {
                Some(mat) => mat.power(-n),
                None => Matrix::zero(),
            }
        } else if n == 0 {
            Matrix::identity()
        } else if n == 1 {
            *self
        } else {
            let mut result = *self;
            for _ in 1..n {
                result = result.multiply(self);
            }
            result
        }
    }

    pub fn is_singular(&self) -> bool {
        self.determinant().abs() < STANDARD_REAL_EPSILON
    }

    pub fn is_identity(&self, tolerance: Standard_Real) -> bool {
        let identity = Matrix::identity();
        for i in 0..3 {
            for j in 0..3 {
                if (self.data[i][j] - identity.data[i][j]).abs() > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_negative(&self, tolerance: Standard_Real) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self.data[i][j] > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn set_diagonal(&mut self, value: Standard_Real) {
        for i in 0..3 {
            for j in 0..3 {
                if i == j {
                    self.data[i][j] = value;
                } else {
                    self.data[i][j] = 0.0;
                }
            }
        }
    }

    pub fn set_scale(&mut self, scale: Standard_Real) {
        self.set_diagonal(scale);
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        self.multiply(&other)
    }
}

impl std::ops::Mul<crate::geometry::Vector> for Matrix {
    type Output = crate::geometry::Vector;

    fn mul(self, vec: crate::geometry::Vector) -> Self::Output {
        self.multiply_vec(&vec)
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, other: Matrix) -> Self::Output {
        Matrix {
            data: [
                [
                    self.data[0][0] + other.data[0][0],
                    self.data[0][1] + other.data[0][1],
                    self.data[0][2] + other.data[0][2],
                ],
                [
                    self.data[1][0] + other.data[1][0],
                    self.data[1][1] + other.data[1][1],
                    self.data[1][2] + other.data[1][2],
                ],
                [
                    self.data[2][0] + other.data[2][0],
                    self.data[2][1] + other.data[2][1],
                    self.data[2][2] + other.data[2][2],
                ],
            ],
        }
    }
}

impl std::ops::Add<&Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Self::Output {
        Matrix {
            data: [
                [
                    self.data[0][0] + other.data[0][0],
                    self.data[0][1] + other.data[0][1],
                    self.data[0][2] + other.data[0][2],
                ],
                [
                    self.data[1][0] + other.data[1][0],
                    self.data[1][1] + other.data[1][1],
                    self.data[1][2] + other.data[1][2],
                ],
                [
                    self.data[2][0] + other.data[2][0],
                    self.data[2][1] + other.data[2][1],
                    self.data[2][2] + other.data[2][2],
                ],
            ],
        }
    }
}

impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;

    fn sub(self, other: Matrix) -> Self::Output {
        self.subtract(&other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let mat = Matrix::new();
        assert_eq!(mat.determinant(), 1.0);
    }

    #[test]
    fn test_matrix_identity() {
        let mat = Matrix::identity();
        assert!(mat.is_identity(0.001));
    }

    #[test]
    fn test_matrix_zero() {
        let mat = Matrix::zero();
        assert_eq!(mat.determinant(), 0.0);
    }

    #[test]
    fn test_matrix_determinant() {
        let mat = Matrix::from_array([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        assert!((mat.determinant() - 0.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut mat = Matrix::from_array([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        mat.transpose();
        assert_eq!(mat.data[0][1], 4.0);
        assert_eq!(mat.data[1][0], 2.0);
    }

    #[test]
    fn test_matrix_multiply() {
        let mat1 = Matrix::from_array([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        let mat2 = Matrix::identity();
        let result = mat1.multiply(&mat2);
        for i in 0..3 {
            for j in 0..3 {
                assert!((result.data[i][j] - mat1.data[i][j]).abs() < STANDARD_REAL_EPSILON);
            }
        }
    }

    #[test]
    fn test_matrix_multiply_vec() {
        let mat = Matrix::identity();
        let vec = crate::geometry::Vector::new(1.0, 2.0, 3.0);
        let result = mat.multiply_vec(&vec);
        assert!((result.x - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.y - 2.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.z - 3.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_matrix_scale() {
        let mut mat = Matrix::identity();
        mat.scale(2.0);
        assert_eq!(mat.data[0][0], 2.0);
        assert_eq!(mat.data[1][1], 2.0);
        assert_eq!(mat.data[2][2], 2.0);
    }

    #[test]
    fn test_matrix_set_diagonal() {
        let mut mat = Matrix::zero();
        mat.set_diagonal(5.0);
        assert_eq!(mat.data[0][0], 5.0);
        assert_eq!(mat.data[1][1], 5.0);
        assert_eq!(mat.data[2][2], 5.0);
        assert_eq!(mat.data[0][1], 0.0);
    }

    #[test]
    fn test_matrix_invert() {
        let mut mat = Matrix::from_array([
            [2.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 2.0],
        ]);
        assert!(mat.invert());
        assert!((mat.data[0][0] - 0.5).abs() < STANDARD_REAL_EPSILON);
        assert!((mat.data[1][1] - 0.5).abs() < STANDARD_REAL_EPSILON);
        assert!((mat.data[2][2] - 0.5).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_matrix_is_singular() {
        let mat = Matrix::zero();
        assert!(mat.is_singular());
    }

    #[test]
    fn test_matrix_power() {
        let mat = Matrix::from_array([
            [2.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [0.0, 0.0, 2.0],
        ]);
        let result = mat.power(2);
        assert!((result.data[0][0] - 4.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.data[1][1] - 4.0).abs() < STANDARD_REAL_EPSILON);
        assert!((result.data[2][2] - 4.0).abs() < STANDARD_REAL_EPSILON);
    }
}
