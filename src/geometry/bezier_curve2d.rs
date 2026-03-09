use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct BezierCurve2D {
    poles: Vec<Point>,
    weights: Vec<StandardReal>,
}

impl BezierCurve2D {
    pub fn new(poles: Vec<Point>) -> Self {
        let weights = vec![1.0; poles.len()];
        Self { poles, weights }
    }

    pub fn with_weights(poles: Vec<Point>, weights: Vec<StandardReal>) -> Self {
        assert_eq!(poles.len(), weights.len(), "Poles and weights must have the same length");
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

    pub fn position(&self, parameter: StandardReal) -> Point {
        if self.poles.is_empty() {
            return Point::origin();
        }

        let n = self.poles.len() - 1;
        let mut result = Point::origin();
        let mut weight_sum = 0.0;

        for i in 0..=n {
            let binomial = self.binomial_coefficient(n, i);
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

    pub fn d1(&self, parameter: StandardReal) -> Vector {
        if self.poles.len() < 2 {
            return Vector::zero();
        }

        let n = self.poles.len() - 1;
        let mut result = Vector::zero();
        let mut weight_sum = 0.0;
        let mut weight_derivative_sum = 0.0;

        for i in 0..=n {
            let binomial = self.binomial_coefficient(n, i);
            let t_pow_i = parameter.powi(i as i32);
            let one_minus_t_pow = (1.0 - parameter).powi((n - i) as i32);
            let basis = binomial * t_pow_i * one_minus_t_pow;
            let weighted_basis = basis * self.weights[i];

            let basis_derivative = binomial * (
                if i > 0 { i as StandardReal * parameter.powi(i as i32 - 1) * one_minus_t_pow } else { 0.0 } +
                if i < n { -((n - i) as StandardReal) * t_pow_i * (1.0 - parameter).powi((n - i - 1) as i32) } else { 0.0 }
            );

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

    fn binomial_coefficient(&self, n: usize, k: usize) -> StandardReal {
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
        self.weights.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON)
    }

    pub fn is_periodic(&self) -> bool {
        false
    }

    pub fn is_closed(&self, tolerance: StandardReal) -> bool {
        if self.poles.len() < 2 {
            return true;
        }
        self.poles.first().unwrap().distance(self.poles.last().unwrap()) <= tolerance
    }

    pub fn continuity(&self) -> i32 {
        if self.poles.len() < 2 {
            return 0;
        }
        
        let start_tangent = self.d1(0.0);
        let end_tangent = self.d1(1.0);
        
        if start_tangent.magnitude() < STANDARD_REAL_EPSILON || end_tangent.magnitude() < STANDARD_REAL_EPSILON {
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
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
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
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let mut curve = BezierCurve2D::new(poles);
        assert!(curve.increase_degree(3));
        assert_eq!(curve.degree(), 3);
    }

    #[test]
    fn test_bezier_curve2d_is_rational() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
        ];
        let curve = BezierCurve2D::new(poles);
        assert!(!curve.is_rational());
    }

    #[test]
    fn test_bezier_curve2d_is_closed() {
        let poles = vec![
            Point::origin(),
            Point::new(1.0, 0.0, 0.0),
            Point::origin(),
        ];
        let curve = BezierCurve2D::new(poles);
        assert!(curve.is_closed(0.001));
    }
}
