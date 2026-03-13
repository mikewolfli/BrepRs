use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BezierCurve2D {
    poles: Vec<Point>,
    weights: Vec<StandardReal>,
}

impl BezierCurve2D {
    /// Batch insert control points
    pub fn insert_poles(
        &mut self,
        indices: &[i32],
        poles: &[Point],
        weights: &[StandardReal],
    ) -> bool {
        if indices.len() != poles.len() || poles.len() != weights.len() {
            return false;
        }
        for ((&idx, pole), weight) in indices.iter().zip(poles).zip(weights) {
            self.insert_pole_after(idx, *pole, *weight);
        }
        true
    }

    /// Batch remove control points
    pub fn remove_poles(&mut self, indices: &[i32]) -> bool {
        let mut success = true;
        for &idx in indices {
            success &= self.remove_pole(idx);
        }
        success
    }
    pub fn new(poles: Vec<Point>) -> Self {
        let weights = vec![1.0; poles.len()];
        Self { poles, weights }
    }

    pub fn with_weights(poles: Vec<Point>, weights: Vec<StandardReal>) -> Self {
        assert_eq!(
            poles.len(),
            weights.len(),
            "Poles and weights must have the same length"
        );
        Self { poles, weights }
    }

    pub fn degree(&self) -> i32 {
        if self.poles.is_empty() {
            0
        } else {
            (self.poles.len() - 1) as i32
        }
    }

    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    pub fn poles(&self) -> &[Point] {
        &self.poles
    }

    pub fn weights(&self) -> &[StandardReal] {
        &self.weights
    }

    pub fn pole(&self, index: i32) -> Option<&Point> {
        if index >= 0 && (index as usize) < self.poles.len() {
            Some(&self.poles[index as usize])
        } else {
            None
        }
    }

    pub fn weight(&self, index: i32) -> Option<StandardReal> {
        if index >= 0 && (index as usize) < self.weights.len() {
            Some(self.weights[index as usize])
        } else {
            None
        }
    }

    pub fn set_pole(&mut self, index: i32, pole: Point) -> bool {
        if index >= 0 && (index as usize) < self.poles.len() {
            self.poles[index as usize] = pole;
            true
        } else {
            false
        }
    }

    pub fn set_weight(&mut self, index: i32, weight: StandardReal) -> bool {
        if index >= 0 && (index as usize) < self.weights.len() {
            self.weights[index as usize] = weight;
            true
        } else {
            false
        }
    }

    pub fn insert_pole_after(&mut self, index: i32, pole: Point, weight: StandardReal) -> bool {
        if index >= 0 && (index as usize) < self.poles.len() {
            let insert_index = (index + 1) as usize;
            self.poles.insert(insert_index, pole);
            self.weights.insert(insert_index, weight);
            true
        } else {
            false
        }
    }

    pub fn remove_pole(&mut self, index: i32) -> bool {
        if index >= 0 && (index as usize) < self.poles.len() {
            self.poles.remove(index as usize);
            self.weights.remove(index as usize);
            true
        } else {
            false
        }
    }

    pub fn increase_degree(&mut self, new_degree: i32) -> bool {
        let current_degree = self.degree();
        if new_degree <= current_degree {
            return false;
        }

        let degree_diff = new_degree - current_degree;
        for _ in 0..degree_diff {
            self.increase_degree_by_one();
        }

        true
    }

    fn increase_degree_by_one(&mut self) {
        let n = self.poles.len();
        if n == 0 {
            return;
        }
        let mut new_poles = Vec::with_capacity(n + 1);
        let mut new_weights = Vec::with_capacity(n + 1);
        new_poles.push(self.poles[0].clone());
        new_weights.push(self.weights[0]);
        for i in 1..n {
            let alpha = i as StandardReal / n as StandardReal;
            let pole = Point::new(
                (1.0 - alpha) * self.poles[i - 1].x + alpha * self.poles[i].x,
                (1.0 - alpha) * self.poles[i - 1].y + alpha * self.poles[i].y,
                (1.0 - alpha) * self.poles[i - 1].z + alpha * self.poles[i].z,
            );
            let weight = (1.0 - alpha) * self.weights[i - 1] + alpha * self.weights[i];
            new_poles.push(pole);
            new_weights.push(weight);
        }
        new_poles.push(self.poles[n - 1].clone());
        new_weights.push(self.weights[n - 1]);
        self.poles = new_poles;
        self.weights = new_weights;
    }

    /// Evaluate curve point at parameter (supports rational Bezier)
    ///
    /// # Parameters
    /// * `parameter` - Curve parameter, range [0, 1]
    ///
    /// # Returns
    /// Point on curve
    pub fn position(&self, parameter: StandardReal) -> Point {
        if self.poles.is_empty() {
            return Point::origin();
        }

        let n = self.poles.len() - 1;
        let mut result = Point::origin();
        let mut weight_sum = 0.0;

        for i in 0..=n {
            let binomial = Self::binomial_coefficient(n, i);
            let t_pow_i = parameter.powi(i as i32);
            let one_minus_t_pow = (1.0 - parameter).powi((n - i) as i32);
            let basis = binomial * t_pow_i * one_minus_t_pow;
            let weighted_basis = basis * self.weights[i];

            result.x += weighted_basis * self.poles[i].x;
            result.y += weighted_basis * self.poles[i].y;
            result.z += weighted_basis * self.poles[i].z;
            weight_sum += weighted_basis;
        }

        if weight_sum.abs() > STANDARD_REAL_EPSILON {
            result.x /= weight_sum;
            result.y /= weight_sum;
            result.z /= weight_sum;
        }

        result
    }

    /// Compute first derivative (tangent vector) at parameter
    ///
    /// # Parameters
    /// * `parameter` - Curve parameter, range [0, 1]
    ///
    /// # Returns
    /// Curve tangent vector
    pub fn d1(&self, parameter: StandardReal) -> Vector {
        if self.poles.len() < 2 {
            return Vector::zero();
        }

        let n = self.poles.len() - 1;
        let mut result = Vector::zero();
        let mut weight_sum = 0.0;
        let mut weight_derivative_sum = 0.0;

        for i in 0..=n {
            let binomial = Self::binomial_coefficient(n, i);
            let t_pow_i = parameter.powi(i as i32);
            let one_minus_t_pow = (1.0 - parameter).powi((n - i) as i32);
            let basis = binomial * t_pow_i * one_minus_t_pow;
            let weighted_basis = basis * self.weights[i];

            let basis_derivative = binomial
                * (if i > 0 {
                    i as StandardReal * parameter.powi(i as i32 - 1) * one_minus_t_pow
                } else {
                    0.0
                } + if i < n {
                    -((n - i) as StandardReal)
                        * t_pow_i
                        * (1.0 - parameter).powi((n - i - 1) as i32)
                } else {
                    0.0
                });

            let weighted_basis_derivative = basis_derivative * self.weights[i];

            result.x += weighted_basis_derivative * self.poles[i].x;
            result.y += weighted_basis_derivative * self.poles[i].y;
            result.z += weighted_basis_derivative * self.poles[i].z;

            weight_sum += weighted_basis;
            weight_derivative_sum += weighted_basis_derivative;
        }

        if weight_sum.abs() > STANDARD_REAL_EPSILON {
            let pos = self.position(parameter);
            let derivative_weight = Vector::new(
                result.x / weight_sum - pos.x * weight_derivative_sum / (weight_sum * weight_sum),
                result.y / weight_sum - pos.y * weight_derivative_sum / (weight_sum * weight_sum),
                result.z / weight_sum - pos.z * weight_derivative_sum / (weight_sum * weight_sum),
            );
            derivative_weight
        } else {
            Vector::zero()
        }
    }

    /// Compute second derivative (curvature vector) at parameter
    ///
    /// # Parameters
    /// * `parameter` - Curve parameter, range [0, 1]
    ///
    /// # Returns
    /// Curve second derivative vector
    pub fn d2(&self, parameter: StandardReal) -> Vector {
        let epsilon = 0.0001;
        let d1_plus = self.d1(parameter + epsilon);
        let d1_minus = self.d1(parameter - epsilon);
        Vector::new(
            (d1_plus.x - d1_minus.x) / (2.0 * epsilon),
            (d1_plus.y - d1_minus.y) / (2.0 * epsilon),
            (d1_plus.z - d1_minus.z) / (2.0 * epsilon),
        )
    }

    /// Compute binomial coefficient
    ///
    /// # Parameters
    /// * `n` - Total number
    /// * `k` - Selection number
    ///
    /// # Returns
    /// Binomial coefficient
    fn binomial_coefficient(n: usize, k: usize) -> StandardReal {
        if k > n {
            return 0.0;
        }
        if k == 0 || k == n {
            return 1.0;
        }

        let mut result = 1.0;
        for i in 0..k.min(n - k) {
            result = result * (n - i) as StandardReal / (i + 1) as StandardReal;
        }
        result
    }

    pub fn is_rational(&self) -> bool {
        self.weights
            .iter()
            .any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON)
    }

    pub fn is_periodic(&self) -> bool {
        false
    }

    pub fn is_closed(&self, tolerance: StandardReal) -> bool {
        if self.poles.len() < 2 {
            return true;
        }
        self.poles
            .first()
            .unwrap()
            .distance(self.poles.last().unwrap())
            <= tolerance
    }

    /// Get curve degree
    pub fn continuity(&self) -> i32 {
        if self.poles.len() < 2 {
            return 0;
        }

        let start_tangent = self.d1(0.0);
        let end_tangent = self.d1(1.0);

        if start_tangent.magnitude() < STANDARD_REAL_EPSILON
            || end_tangent.magnitude() < STANDARD_REAL_EPSILON
        {
            return 0;
        }

        let normalized_start = start_tangent.normalized();
        let normalized_end = end_tangent.normalized();

        if normalized_start.is_equal(&normalized_end, STANDARD_REAL_EPSILON) {
            1
        } else {
            0
        }
    }
}

impl Default for BezierCurve2D {
    fn default() -> Self {
        Self {
            poles: vec![Point::origin()],
            weights: vec![1.0],
        }
    }
}

impl crate::topology::Curve for BezierCurve2D {
    fn value(&self, parameter: f64) -> Point {
        self.position(parameter as StandardReal)
    }

    fn derivative(&self, parameter: f64) -> Vector {
        self.d1(parameter as StandardReal)
    }

    fn parameter_range(&self) -> (f64, f64) {
        (0.0, 1.0)
    }
}

impl crate::geometry::advanced_traits::Curve for BezierCurve2D {
    type Point = Point;
    type Vector = Vector;

    fn sample(&self, t: f64) -> Self::Point {
        self.position(t as StandardReal)
    }

    fn derivative(&self, t: f64) -> Self::Vector {
        self.d1(t as StandardReal)
    }

    fn second_derivative(&self, t: f64) -> Self::Vector {
        self.d2(t as StandardReal)
    }

    fn project(&self, point: &Self::Point) -> f64 {
        // Use Newton-Raphson method to find closest point parameter
        let mut t = 0.5; // Initial guess
        let max_iter = 20;
        let tol = 1e-8;

        for _ in 0..max_iter {
            let p = self.position(t as StandardReal);
            let dp = self.d1(t as StandardReal);

            let vec = (point.x - p.x, point.y - p.y, point.z - p.z);
            let deriv_vec = (dp.x, dp.y, dp.z);

            let f = vec.0 * deriv_vec.0 + vec.1 * deriv_vec.1 + vec.2 * deriv_vec.2;
            let df =
                deriv_vec.0 * deriv_vec.0 + deriv_vec.1 * deriv_vec.1 + deriv_vec.2 * deriv_vec.2;

            if df.abs() < 1e-12 {
                break;
            }

            let t_next = t - f / df;
            let t_next = t_next.clamp(0.0, 1.0);

            if (t_next - t).abs() < tol {
                t = t_next;
                break;
            }

            t = t_next;
        }

        t
    }

    fn closest_point(&self, point: &Self::Point) -> Self::Point {
        // Use Newton-Raphson method to find closest point
        let t = self.project(point);
        self.position(t as StandardReal)
    }

    fn length(&self, t0: f64, t1: f64) -> f64 {
        // Use numerical integration to compute curve length
        let steps = 100;
        let mut length = 0.0;
        let mut prev_point = self.position(t0 as StandardReal);

        for i in 1..=steps {
            let t = t0 + (t1 - t0) * i as f64 / steps as f64;
            let current_point = self.position(t as StandardReal);
            length += prev_point.distance(&current_point);
            prev_point = current_point;
        }

        length
    }
}

impl BezierCurve2D {
    /// Split curve
    pub fn subcurve(&self, t0: f64, t1: f64) -> Self {
        // Use de Casteljau algorithm to split curve
        let t0_clamped = t0.clamp(0.0, 1.0);
        let t1_clamped = t1.clamp(0.0, 1.0);

        if (t0_clamped - t1_clamped).abs() < 1e-12 {
            return self.clone();
        }

        // First split at t0
        let (_left, right) = self.de_casteljau_split(t0_clamped);
        // Then split right curve at (t1 - t0)/(1 - t0)
        let t = (t1_clamped - t0_clamped) / (1.0 - t0_clamped);
        let (_, result) = right.de_casteljau_split(t);

        result
    }

    /// Use de Casteljau algorithm to split curve
    fn de_casteljau_split(&self, t: f64) -> (Self, Self) {
        let n = self.poles.len() - 1;
        let mut left_poles = Vec::with_capacity(n + 1);
        let mut right_poles = Vec::with_capacity(n + 1);
        let mut left_weights = Vec::with_capacity(n + 1);
        let mut right_weights = Vec::with_capacity(n + 1);

        // Initialize control points and weights
        let mut temp_poles = self.poles.clone();
        let mut temp_weights = self.weights.clone();

        for i in 0..=n {
            left_poles.push(temp_poles[0].clone());
            left_weights.push(temp_weights[0]);
            right_poles.push(temp_poles[n - i].clone());
            right_weights.push(temp_weights[n - i]);

            // Compute next layer control points
            for j in 0..n - i {
                temp_poles[j] = Point::new(
                    (1.0 - t) * temp_poles[j].x + t * temp_poles[j + 1].x,
                    (1.0 - t) * temp_poles[j].y + t * temp_poles[j + 1].y,
                    (1.0 - t) * temp_poles[j].z + t * temp_poles[j + 1].z,
                );
                temp_weights[j] = (1.0 - t) * temp_weights[j] + t * temp_weights[j + 1];
            }
        }

        (
            Self::with_weights(left_poles, left_weights),
            Self::with_weights(right_poles, right_weights),
        )
    }

    /// Join curves
    pub fn join(&self, other: &Self) -> Self {
        // Join two curves by concatenating their control points
        let mut poles = self.poles.clone();
        poles.extend_from_slice(&other.poles);
        let mut weights = self.weights.clone();
        weights.extend_from_slice(&other.weights);
        Self { poles, weights }
    }

    /// Fit curve to points using least squares
    pub fn fit(points: &[Point], degree: i32) -> Self {
        if points.is_empty() {
            return Self::default();
        }

        // Clamp degree to valid range
        let clamped_degree = degree.clamp(1, (points.len() - 1) as i32);
        let n = clamped_degree as usize;

        // If points count is less than or equal to degree+1, use points as control points directly
        if points.len() <= n + 1 {
            let poles = points.to_vec();
            let weights = vec![1.0; poles.len()];
            return Self::with_weights(poles, weights);
        }

        // Use least squares fitting
        let m = points.len();

        // Build matrix a and vector b
        let mut a = vec![vec![0.0; n + 1]; n + 1];
        let mut bx = vec![0.0; n + 1];
        let mut by = vec![0.0; n + 1];
        let mut bz = vec![0.0; n + 1];

        for i in 0..m {
            let t = i as f64 / (m - 1) as f64;
            let mut basis = vec![0.0; n + 1];

            // Compute Bernstein basis functions
            for j in 0..=n {
                basis[j] = Self::binomial_coefficient(n, j)
                    * t.powi(j as i32)
                    * (1.0 - t).powi((n - j) as i32);
            }

            // Fill matrix a and vector b
            for j in 0..=n {
                for k in 0..=n {
                    a[j][k] += basis[j] * basis[k];
                }
                bx[j] += basis[j] * points[i].x;
                by[j] += basis[j] * points[i].y;
                bz[j] += basis[j] * points[i].z;
            }
        }

        // Solve linear system
        let cx = Self::solve_linear_system(&a, &bx);
        let cy = Self::solve_linear_system(&a, &by);
        let cz = Self::solve_linear_system(&a, &bz);

        // Build control points
        let mut poles = Vec::with_capacity(n + 1);
        for i in 0..=n {
            poles.push(Point::new(cx[i], cy[i], cz[i]));
        }

        let weights = vec![1.0; poles.len()];
        Self::with_weights(poles, weights)
    }

    /// Solve linear system Ax = b using Gaussian elimination
    fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        let n = a.len();
        let mut aug = vec![vec![0.0; n + 1]; n];

        // Build augmented matrix
        for i in 0..n {
            for j in 0..n {
                aug[i][j] = a[i][j];
            }
            aug[i][n] = b[i];
        }

        // Gaussian elimination
        for i in 0..n {
            // Find pivot element
            let mut max_row = i;
            for j in i..n {
                if aug[j][i].abs() > aug[max_row][i].abs() {
                    max_row = j;
                }
            }

            // Swap rows
            if max_row != i {
                aug.swap(i, max_row);
            }

            // Normalize pivot row
            let pivot = aug[i][i];
            if pivot.abs() < 1e-12 {
                return vec![0.0; n]; // Singular matrix
            }

            for j in i..n + 1 {
                aug[i][j] /= pivot;
            }

            // Eliminate other rows
            for j in 0..n {
                if j != i {
                    let factor = aug[j][i];
                    for k in i..n + 1 {
                        aug[j][k] -= factor * aug[i][k];
                    }
                }
            }
        }

        // Extract solution
        let mut x = vec![0.0; n];
        for i in 0..n {
            x[i] = aug[i][n];
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_curve2d_creation() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let curve = BezierCurve2D::new(poles.clone());
        assert_eq!(curve.degree(), 2);
        assert_eq!(curve.nb_poles(), 3);
    }

    #[test]
    fn test_bezier_curve2d_position() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let curve = BezierCurve2D::new(poles);

        let start = curve.position(0.0);
        assert_eq!(start, Point::origin());

        let end = curve.position(1.0);
        assert_eq!(end, Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_bezier_curve2d_degree() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        let curve = BezierCurve2D::new(poles);
        assert_eq!(curve.degree(), 3);
    }

    #[test]
    fn test_bezier_curve2d_set_pole() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let mut curve = BezierCurve2D::new(poles);
        assert!(curve.set_pole(1, Point::new(2.0, 0.0, 0.0)));
        assert_eq!(curve.pole(1), Some(&Point::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_bezier_curve2d_insert_pole() {
        let poles = vec![Point::origin(), Point::new(1.0, 0.0, 0.0)];
        let mut curve = BezierCurve2D::new(poles);
        assert!(curve.insert_pole_after(0, Point::new(0.5, 0.5, 0.0), 1.0));
        assert_eq!(curve.nb_poles(), 3);
    }

    #[test]
    fn test_bezier_curve2d_remove_pole() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
        ];
        let mut curve = BezierCurve2D::new(poles);
        assert!(curve.remove_pole(1));
        assert_eq!(curve.nb_poles(), 2);
    }

    #[test]
    fn test_bezier_curve2d_increase_degree() {
        let poles = vec![Point::origin(), Point::new(1.0, 0.0, 0.0)];
        let mut curve = BezierCurve2D::new(poles);
        assert!(curve.increase_degree(3));
        assert_eq!(curve.degree(), 3);
    }

    #[test]
    fn test_bezier_curve2d_is_rational() {
        let poles = vec![Point::origin(), Point::new(1.0, 0.0, 0.0)];
        let curve = BezierCurve2D::new(poles);
        assert!(!curve.is_rational());
    }

    #[test]
    fn test_bezier_curve2d_is_closed() {
        let poles = vec![Point::origin(), Point::new(1.0, 0.0, 0.0), Point::origin()];
        let curve = BezierCurve2D::new(poles);
        assert!(curve.is_closed(0.001));
    }
}
