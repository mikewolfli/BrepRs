use crate::foundation::types::{StandardReal, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NurbsCurve3D {
    poles: Vec<Point>,
    weights: Vec<StandardReal>,
    knots: Vec<StandardReal>,
    multiplicities: Vec<i32>,
    degree: i32,
    is_rational: bool,
    is_periodic: bool,
}

impl NurbsCurve3D {
    pub fn new(
        degree: i32,
        poles: Vec<Point>,
        weights: Vec<StandardReal>,
        knots: Vec<StandardReal>,
        multiplicities: Vec<i32>,
    ) -> Self {
        assert!(!poles.is_empty(), "Poles cannot be empty");
        assert_eq!(poles.len(), weights.len(), "Poles and weights must have the same length");
        assert!(degree >= 0, "Degree must be non-negative");
        assert!(!knots.is_empty(), "Knots cannot be empty");
        assert_eq!(knots.len(), multiplicities.len(), "Knots and multiplicities must have the same length");

        let is_rational = weights.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON);

        Self {
            degree,
            poles,
            weights,
            knots,
            multiplicities,
            is_rational,
            is_periodic: false,
        }
    }

    pub fn degree(&self) -> i32 {
        self.degree
    }

    pub fn nb_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    pub fn nb_knots(&self) -> i32 {
        self.knots.len() as i32
    }

    pub fn poles(&self) -> &[Point] {
        &self.poles
    }

    pub fn weights(&self) -> &[StandardReal] {
        &self.weights
    }

    pub fn knots(&self) -> &[StandardReal] {
        &self.knots
    }

    pub fn multiplicities(&self) -> &[i32] {
        &self.multiplicities
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

    pub fn knot(&self, index: i32) -> Option<StandardReal> {
        if index >= 0 && (index as usize) < self.knots.len() {
            Some(self.knots[index as usize])
        } else {
            None
        }
    }

    pub fn multiplicity(&self, index: i32) -> Option<i32> {
        if index >= 0 && (index as usize) < self.multiplicities.len() {
            Some(self.multiplicities[index as usize])
        } else {
            None
        }
    }

    pub fn is_rational(&self) -> bool {
        self.is_rational
    }

    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }

    pub fn set_periodic(&mut self, periodic: bool) {
        self.is_periodic = periodic;
    }

    pub fn evaluate(&self, parameter: StandardReal) -> Point {
        let span = self.find_span(parameter);
        let basis = self.basis_functions(span, parameter);
        
        let mut weighted_sum = Point::origin();
        let mut weight_sum = 0.0;
        
        for i in 0..=self.degree {
            let weight = self.weights[span - self.degree + i as usize] * basis[i as usize];
            weighted_sum = weighted_sum + (self.poles[span - self.degree + i as usize] * weight);
            weight_sum += weight;
        }
        
        if weight_sum > STANDARD_REAL_EPSILON {
            weighted_sum = weighted_sum * (1.0 / weight_sum);
        }
        
        weighted_sum
    }

    pub fn evaluate_derivative(&self, parameter: StandardReal, order: i32) -> Vector {
        if order <= 0 || order > self.degree {
            return Vector::new(0.0, 0.0, 0.0);
        }
        
        let span = self.find_span(parameter);
        let mut basis = vec![vec![0.0; (self.degree + 1) as usize]; (order + 1) as usize];
        
        self.basis_functions_derivatives(span, parameter, order, &mut basis);
        
        let mut weighted_sum = Vector::new(0.0, 0.0, 0.0);
        let mut weight_sum = 0.0;
        
        for i in 0..=self.degree {
            let weight = self.weights[span - self.degree + i as usize] * basis[order as usize][i as usize];
            let point = self.poles[span - self.degree + i as usize];
            weighted_sum = weighted_sum + (Vector::new(point.x, point.y, point.z) * weight);
            weight_sum += weight;
        }
        
        if weight_sum > STANDARD_REAL_EPSILON {
            weighted_sum = weighted_sum * (1.0 / weight_sum);
        }
        
        weighted_sum
    }

    fn find_span(&self, parameter: StandardReal) -> usize {
        let n = self.poles.len() - 1;
        let m = self.knots.len() - 1;
        
        if (parameter - self.knots[m]).abs() < STANDARD_REAL_EPSILON {
            return n;
        }
        
        let mut low = self.degree as usize;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        
        while parameter < self.knots[mid] || parameter >= self.knots[mid + 1] {
            if parameter < self.knots[mid] {
                high = mid;
            } else {
                low = mid + 1;
            }
            mid = (low + high) / 2;
        }
        
        mid
    }

    fn basis_functions(&self, span: usize, parameter: StandardReal) -> Vec<StandardReal> {
        let mut basis = vec![0.0; (self.degree + 1) as usize];
        let mut left = vec![0.0; (self.degree + 1) as usize];
        let mut right = vec![0.0; (self.degree + 1) as usize];
        
        basis[0] = 1.0;
        
        for j in 1..=self.degree {
            left[j as usize] = parameter - self.knots[span - j as usize];
            right[j as usize] = self.knots[span + j as usize] - parameter;
            
            let mut saved = 0.0;
            for r in 0..j {
                let temp = basis[r as usize] / (right[r + 1 as usize] + left[j - r as usize]);
                basis[r as usize] = saved + right[r + 1 as usize] * temp;
                saved = left[j - r as usize] * temp;
            }
            basis[j as usize] = saved;
        }
        
        basis
    }

    fn basis_functions_derivatives(&self, span: usize, parameter: StandardReal, order: i32, basis: &mut Vec<Vec<StandardReal>>) {
        let mut ndu = vec![vec![0.0; (self.degree + 1) as usize]; (self.degree + 1) as usize];
        ndu[0][0] = 1.0;
        
        let mut left = vec![0.0; (self.degree + 1) as usize];
        let mut right = vec![0.0; (self.degree + 1) as usize];
        
        for j in 1..=self.degree {
            left[j as usize] = parameter - self.knots[span - j as usize];
            right[j as usize] = self.knots[span + j as usize] - parameter;
            
            let mut saved = 0.0;
            for r in 0..j {
                ndu[j as usize][r as usize] = right[r + 1 as usize] + left[j - r as usize];
                let temp = ndu[r as usize][j - 1 as usize] / ndu[j as usize][r as usize];
                
                ndu[r as usize][j as usize] = saved + right[r + 1 as usize] * temp;
                saved = left[j - r as usize] * temp;
            }
            ndu[j as usize][j as usize] = saved;
        }
        
        for j in 0..=self.degree {
            basis[0][j as usize] = ndu[j as usize][self.degree as usize];
        }
        
        for r in 0..=self.degree {
            let mut s1 = 0;
            let mut s2 = 1;
            
            let mut a = vec![vec![0.0; (self.degree + 1) as usize]; 2];
            a[0][0] = 1.0;
            
            for k in 1..=order {
                let d = 0.0;
                
                let rk = r - k;
                let pk = self.degree - k;
                
                if r >= k {
                    a[s2][0] = a[s1][0] / ndu[pk + 1 as usize][rk as usize];
                    d = a[s2][0] * ndu[rk as usize][pk as usize];
                }
                
                let j1 = if (rk + 1) > 0 { rk + 1 } else { 0 };
                let j2 = if (r - 1) <= pk { r - 1 } else { pk as i32 };
                
                for j in j1..=j2 {
                    a[s2][(j - rk) as usize] = (a[s1][(j - rk) as usize] - a[s1][(j - rk - 1) as usize]) / ndu[pk + 1 as usize][j as usize];
                    d += a[s2][(j - rk) as usize] * ndu[j as usize][pk as usize];
                }
                
                if r <= pk {
                    a[s2][(self.degree - r) as usize] = -a[s1][(self.degree - r - 1) as usize] / ndu[pk + 1 as usize][r as usize];
                    d += a[s2][(self.degree - r) as usize] * ndu[r as usize][pk as usize];
                }
                
                basis[k as usize][r as usize] = d;
                
                let temp = s1;
                s1 = s2;
                s2 = temp;
            }
        }
        
        let mut k = self.degree;
        for i in 1..=order {
            for j in 0..=self.degree {
                basis[i as usize][j as usize] *= k as StandardReal;
            }
            k *= self.degree - i;
        }
    }

    pub fn length(&self, tolerance: StandardReal) -> StandardReal {
        let mut length = 0.0;
        let mut t = self.knots[0];
        let step = 0.01;
        
        while t < self.knots[self.knots.len() - 1] {
            let next_t = (t + step).min(self.knots[self.knots.len() - 1]);
            let p1 = self.evaluate(t);
            let p2 = self.evaluate(next_t);
            length += p1.distance(&p2);
            t = next_t;
        }
        
        length
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        if self.poles.is_empty() {
            return (Point::origin(), Point::origin());
        }
        
        let mut min_x = self.poles[0].x;
        let mut min_y = self.poles[0].y;
        let mut min_z = self.poles[0].z;
        let mut max_x = self.poles[0].x;
        let mut max_y = self.poles[0].y;
        let mut max_z = self.poles[0].z;
        
        for pole in &self.poles {
            min_x = min_x.min(pole.x);
            min_y = min_y.min(pole.y);
            min_z = min_z.min(pole.z);
            max_x = max_x.max(pole.x);
            max_y = max_y.max(pole.y);
            max_z = max_z.max(pole.z);
        }
        
        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nurbs_curve3d_creation() {
        let degree = 2;
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 1.0),
            Point::new(2.0, 1.0, 2.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let multiplicities = vec![3, 1, 1, 3];
        
        let curve = NurbsCurve3D::new(degree, poles, weights, knots, multiplicities);
        assert_eq!(curve.degree(), 2);
        assert_eq!(curve.nb_poles(), 4);
        assert_eq!(curve.nb_knots(), 4);
        assert!(!curve.is_rational());
        assert!(!curve.is_periodic());
    }

    #[test]
    fn test_nurbs_curve3d_evaluate() {
        let degree = 2;
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(2.0, 1.0, 0.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        let weights = vec![1.0, 1.0, 1.0, 1.0];
        let knots = vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let multiplicities = vec![3, 1, 1, 3];
        
        let curve = NurbsCurve3D::new(degree, poles, weights, knots, multiplicities);
        let point = curve.evaluate(0.5);
        
        // For a quadratic NURBS with these control points, the midpoint should be at (1.5, 1.0, 0.0)
        assert!((point.x - 1.5).abs() < STANDARD_REAL_EPSILON);
        assert!((point.y - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((point.z - 0.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_nurbs_curve3d_length() {
        let degree = 1;
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        let weights = vec![1.0, 1.0];
        let knots = vec![0.0, 0.0, 1.0, 1.0];
        let multiplicities = vec![2, 2];
        
        let curve = NurbsCurve3D::new(degree, poles, weights, knots, multiplicities);
        let length = curve.length(0.001);
        
        // For a straight line, the length should be 3.0
        assert!((length - 3.0).abs() < 0.01);
    }
}