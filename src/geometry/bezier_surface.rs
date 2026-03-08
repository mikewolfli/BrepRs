use crate::foundation::types::{Standard_Real, STANDARD_REAL_EPSILON};
use crate::geometry::{Point, Vector};

#[derive(Debug, Clone, PartialEq)]
pub struct BezierSurface {
    poles: Vec<Vec<Point>>,
    weights: Vec<Vec<Standard_Real>>,
}

impl BezierSurface {
    pub fn new(poles: Vec<Vec<Point>>) -> Self {
        let u_degree = poles.len() - 1;
        let v_degree = if poles.is_empty() { 0 } else { poles[0].len() - 1 };

        let weights = vec![vec![1.0; v_degree + 1]; u_degree + 1];
        Self { poles, weights }
    }

    pub fn with_weights(poles: Vec<Vec<Point>>, weights: Vec<Vec<Standard_Real>>) -> Self {
        assert_eq!(poles.len(), weights.len(), "Poles and weights must have same number of U rows");
        if !poles.is_empty() {
            assert_eq!(poles[0].len(), weights[0].len(), "Poles and weights must have same number of V columns");
        }
        Self { poles, weights }
    }

    pub fn u_degree(&self) -> i32 {
        if self.poles.is_empty() {
            0
        } else {
            (self.poles.len() - 1) as i32
        }
    }

    pub fn v_degree(&self) -> i32 {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            0
        } else {
            (self.poles[0].len() - 1) as i32
        }
    }

    pub fn nb_u_poles(&self) -> i32 {
        self.poles.len() as i32
    }

    pub fn nb_v_poles(&self) -> i32 {
        if self.poles.is_empty() {
            0
        } else {
            self.poles[0].len() as i32
        }
    }

    pub fn poles(&self) -> &[Vec<Point>] {
        &self.poles
    }

    pub fn weights(&self) -> &[Vec<Standard_Real>] {
        &self.weights
    }

    pub fn pole(&self, u_index: i32, v_index: i32) -> Option<&Point> {
        if u_index >= 0 && (u_index as usize) < self.poles.len() {
            let u_row = &self.poles[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                Some(&u_row[v_index as usize])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn weight(&self, u_index: i32, v_index: i32) -> Option<Standard_Real> {
        if u_index >= 0 && (u_index as usize) < self.weights.len() {
            let u_row = &self.weights[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                Some(u_row[v_index as usize])
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn set_pole(&mut self, u_index: i32, v_index: i32, pole: Point) -> bool {
        if u_index >= 0 && (u_index as usize) < self.poles.len() {
            let u_row = &mut self.poles[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                u_row[v_index as usize] = pole;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn set_weight(&mut self, u_index: i32, v_index: i32, weight: Standard_Real) -> bool {
        if u_index >= 0 && (u_index as usize) < self.weights.len() {
            let u_row = &mut self.weights[u_index as usize];
            if v_index >= 0 && (v_index as usize) < u_row.len() {
                u_row[v_index as usize] = weight;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn position(&self, u_parameter: Standard_Real, v_parameter: Standard_Real) -> Point {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return Point::origin();
        }

        let u_degree = self.u_degree();
        let v_degree = self.v_degree();

        let mut result = Point::origin();
        let mut weight_sum = 0.0;

        for i in 0..=u_degree {
            for j in 0..=v_degree {
                let u_basis = self.basis_function(i, u_degree, u_parameter);
                let v_basis = self.basis_function(j, v_degree, v_parameter);
                let basis = u_basis * v_basis;
                let weighted_basis = basis * self.weights[i as usize][j as usize];

                result.x += weighted_basis * self.poles[i as usize][j as usize].x;
                result.y += weighted_basis * self.poles[i as usize][j as usize].y;
                result.z += weighted_basis * self.poles[i as usize][j as usize].z;
                weight_sum += weighted_basis;
            }
        }

        if weight_sum.abs() > STANDARD_REAL_EPSILON {
            result.x /= weight_sum;
            result.y /= weight_sum;
            result.z /= weight_sum;
        }

        result
    }

    pub fn d1(&self, u_parameter: Standard_Real, v_parameter: Standard_Real, u_direction: bool) -> Vector {
        let epsilon = 0.0001;
        if u_direction {
            let pos_plus = self.position(u_parameter + epsilon, v_parameter);
            let pos_minus = self.position(u_parameter - epsilon, v_parameter);
            Vector::new(
                (pos_plus.x - pos_minus.x) / (2.0 * epsilon),
                (pos_plus.y - pos_minus.y) / (2.0 * epsilon),
                (pos_plus.z - pos_minus.z) / (2.0 * epsilon),
            )
        } else {
            let pos_plus = self.position(u_parameter, v_parameter + epsilon);
            let pos_minus = self.position(u_parameter, v_parameter - epsilon);
            Vector::new(
                (pos_plus.x - pos_minus.x) / (2.0 * epsilon),
                (pos_plus.y - pos_minus.y) / (2.0 * epsilon),
                (pos_plus.z - pos_minus.z) / (2.0 * epsilon),
            )
        }
    }

    pub fn d2(&self, u_parameter: Standard_Real, v_parameter: Standard_Real) -> Vector {
        let epsilon = 0.0001;
        let d1_plus = self.d1(u_parameter + epsilon, v_parameter, true);
        let d1_minus = self.d1(u_parameter - epsilon, v_parameter, true);
        Vector::new(
            (d1_plus.x - d1_minus.x) / (2.0 * epsilon),
            (d1_plus.y - d1_minus.y) / (2.0 * epsilon),
            (d1_plus.z - d1_minus.z) / (2.0 * epsilon),
        )
    }

    fn basis_function(&self, i: i32, n: i32, t: Standard_Real) -> Standard_Real {
        let binomial = self.binomial_coefficient(n, i);
        let t_pow_i = t.powi(i);
        let one_minus_t_pow = (1.0 - t).powi(n - i);
        binomial * t_pow_i * one_minus_t_pow
    }

    fn binomial_coefficient(&self, n: i32, k: i32) -> Standard_Real {
        if k < 0 || k > n {
            return 0.0;
        }
        if k == 0 || k == n {
            return 1.0;
        }
        
        let mut result = 1.0;
        for i in 0..k.min(n - k) {
            result = result * (n - i) as Standard_Real / (i + 1) as Standard_Real;
        }
        result
    }

    pub fn is_rational(&self) -> bool {
        self.weights.iter().any(|row| row.iter().any(|&w| (w - 1.0).abs() > STANDARD_REAL_EPSILON))
    }

    pub fn is_periodic(&self) -> bool {
        false
    }

    pub fn is_u_closed(&self, tolerance: Standard_Real) -> bool {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return true;
        }
        
        for j in 0..self.nb_v_poles() {
            if let (Some(first), Some(last)) = (self.pole(0, j), self.pole(self.u_degree(), j)) {
                if first.distance(last) > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_v_closed(&self, tolerance: Standard_Real) -> bool {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return true;
        }
        
        for i in 0..self.nb_u_poles() {
            if let (Some(first), Some(last)) = (self.pole(i, 0), self.pole(i, self.v_degree())) {
                if first.distance(last) > tolerance {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_u_periodic(&self) -> bool {
        false
    }

    pub fn is_v_periodic(&self) -> bool {
        false
    }

    pub fn u_continuity(&self) -> i32 {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return 0;
        }
        
        let start_tangent = self.d1(0.0, 0.0, true);
        let end_tangent = self.d1(1.0, 0.0, true);
        
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

    pub fn v_continuity(&self) -> i32 {
        if self.poles.is_empty() || self.poles[0].is_empty() {
            return 0;
        }
        
        let start_tangent = self.d1(0.0, 0.0, false);
        let end_tangent = self.d1(0.0, 1.0, false);
        
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

impl Default for BezierSurface {
    fn default() -> Self {
        Self {
            poles: vec![vec![Point::origin()]],
            weights: vec![vec![1.0]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bezier_surface_creation() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let surface = BezierSurface::new(poles.clone());
        assert_eq!(surface.u_degree(), 1);
        assert_eq!(surface.v_degree(), 1);
    }

    #[test]
    fn test_bezier_surface_position() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let surface = BezierSurface::new(poles);
        
        let corner = surface.position(0.0, 0.0);
        assert_eq!(corner, Point::origin());
        
        let opposite_corner = surface.position(1.0, 1.0);
        assert_eq!(opposite_corner, Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_bezier_surface_degree() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0), Point::new(2.0, 1.0, 0.0)],
            vec![Point::new(0.0, 2.0, 0.0), Point::new(1.0, 2.0, 0.0), Point::new(2.0, 2.0, 0.0)],
        ];
        let surface = BezierSurface::new(poles);
        assert_eq!(surface.u_degree(), 2);
        assert_eq!(surface.v_degree(), 2);
    }

    #[test]
    fn test_bezier_surface_set_pole() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let mut surface = BezierSurface::new(poles);
        assert!(surface.set_pole(0, 0, Point::new(2.0, 0.0, 0.0)));
        assert_eq!(surface.pole(0, 0), Some(&Point::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_bezier_surface_is_rational() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::new(0.0, 1.0, 0.0), Point::new(1.0, 1.0, 0.0)],
        ];
        let surface = BezierSurface::new(poles);
        assert!(!surface.is_rational());
    }

    #[test]
    fn test_bezier_surface_is_u_closed() {
        let poles = vec![
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
            vec![Point::origin(), Point::new(1.0, 0.0, 0.0)],
        ];
        let surface = BezierSurface::new(poles);
        assert!(surface.is_u_closed(0.001));
    }
}
