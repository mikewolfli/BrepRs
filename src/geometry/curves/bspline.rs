//! B-spline curve implementation
//! 
//! B-spline curves are a type of spline curve that uses a set of control points
//! and a knot vector to define the curve.

use super::{Curve, Curve3D};
use crate::geometry::{Point, Vector};

/// B-spline curve
#[derive(Debug, Clone, PartialEq)]
pub struct BSplineCurve {
    /// Control points
    control_points: Vec<Point>,
    /// Knot vector
    knots: Vec<f64>,
    /// Degree of the curve
    degree: usize,
}

impl BSplineCurve {
    /// Create a new B-spline curve
    pub fn new(control_points: Vec<Point>, knots: Vec<f64>, degree: usize) -> Result<Self, String> {
        // Validate input
        if control_points.len() < degree + 1 {
            return Err("Not enough control points for the specified degree".to_string());
        }
        
        let n = control_points.len() - 1;
        let m = knots.len() - 1;
        
        if m != n + degree + 1 {
            return Err("Invalid knot vector length".to_string());
        }
        
        // Check if knots are non-decreasing
        for i in 1..knots.len() {
            if knots[i] < knots[i - 1] {
                return Err("Knots must be non-decreasing".to_string());
            }
        }
        
        Ok(Self {
            control_points,
            knots,
            degree,
        })
    }

    /// Create a uniform B-spline curve
    pub fn uniform(control_points: Vec<Point>, degree: usize) -> Result<Self, String> {
        let n = control_points.len() - 1;
        let m = n + degree + 1;
        
        let mut knots = Vec::with_capacity(m + 1);
        
        // Create uniform knot vector
        for i in 0..=m {
            knots.push(i as f64);
        }
        
        Self::new(control_points, knots, degree)
    }

    /// Create a clamped B-spline curve
    pub fn clamped(control_points: Vec<Point>, degree: usize) -> Result<Self, String> {
        let n = control_points.len() - 1;
        let m = n + degree + 1;
        
        let mut knots = Vec::with_capacity(m + 1);
        
        // Clamped knot vector has degree+1 copies of the first and last knot
        for i in 0..=degree {
            knots.push(0.0);
        }
        
        for i in 1..=n - degree {
            knots.push(i as f64);
        }
        
        for i in 0..=degree {
            knots.push((n - degree + 1) as f64);
        }
        
        Self::new(control_points, knots, degree)
    }

    /// Get the control points
    pub fn control_points(&self) -> &[Point] {
        &self.control_points
    }

    /// Get the knot vector
    pub fn knots(&self) -> &[f64] {
        &self.knots
    }

    /// Get the degree
    pub fn degree(&self) -> usize {
        self.degree
    }

    /// Find the span of the parameter t
    fn find_span(&self, t: f64) -> usize {
        let n = self.control_points.len() - 1;
        let p = self.degree;
        
        if t >= self.knots[n + 1] {
            return n;
        }
        
        if t <= self.knots[p] {
            return p;
        }
        
        // Binary search
        let mut low = p;
        let mut high = n + 1;
        let mut mid = (low + high) / 2;
        
        while t < self.knots[mid] || t >= self.knots[mid + 1] {
            if t < self.knots[mid] {
                high = mid;
            } else {
                low = mid;
            }
            mid = (low + high) / 2;
        }
        
        mid
    }

    /// Calculate basis functions
    fn basis_functions(&self, i: usize, t: f64) -> Vec<f64> {
        let p = self.degree;
        let knots = &self.knots;
        
        let mut N = vec![0.0; p + 1];
        N[0] = 1.0;
        
        for j in 1..=p {
            let mut left = vec![0.0; j];
            let mut right = vec![0.0; j];
            
            for l in 0..j {
                left[l] = t - knots[i + 1 - j + l];
                right[l] = knots[i + 1 + l] - t;
            }
            
            for l in 0..j {
                let saved = 0.0;
                for k in 0..=l {
                    let temp = N[k] / (right[k] + left[l - k]);
                    N[k] = saved + right[k] * temp;
                }
                N[j] = saved + left[j] * N[j - 1];
            }
        }
        
        N
    }

    /// Calculate basis function derivatives
    fn basis_function_derivatives(&self, i: usize, t: f64, n: usize) -> Vec<Vec<f64>> {
        let p = self.degree;
        let knots = &self.knots;
        
        let mut ndu = vec![vec![0.0; p + 1]; p + 1];
        ndu[0][0] = 1.0;
        
        for j in 1..=p {
            let mut left = t - knots[i + 1 - j];
            let mut right = knots[i + j] - t;
            
            let mut saved = 0.0;
            for r in 0..j {
                ndu[j][r] = right * ndu[j - 1][r];
                ndu[j][r + 1] = left * ndu[j - 1][r];
                saved += ndu[j][r];
            }
            ndu[j][j] = saved;
        }
        
        let mut derivatives = vec![vec![0.0; p + 1]; n + 1];
        for r in 0..=p {
            derivatives[0][r] = ndu[p][r];
        }
        
        for r in 0..=p {
            let mut s1 = 0;
            let mut s2 = 1;
            let mut a = vec![vec![0.0; p + 1]; 2];
            a[0][0] = 1.0;
            
            for k in 1..=n {
                let d = 0.0;
                let rk = r - k;
                let pk = p - k;
                
                if r >= k {
                    a[s2][0] = a[s1][0] / (knots[i + pk + 1] - knots[i + rk]);
                    d = a[s2][0] * ndu[pk][rk];
                }
                
                let j1 = if rk >= -1 { rk + 1 } else { 0 };
                let j2 = if r - 1 <= pk { r - 1 } else { pk };
                
                for j in j1..=j2 {
                    a[s2][j] = (a[s1][j] - a[s1][j - 1]) / (knots[i + pk + 1 + j] - knots[i + rk + j]);
                    d += a[s2][j] * ndu[pk][rk + j];
                }
                
                if r <= pk {
                    a[s2][k] = -a[s1][k - 1] / (knots[i + pk + 1 + k] - knots[i + r]);
                    d += a[s2][k] * ndu[pk][r];
                }
                
                derivatives[k][r] = d;
                std::mem::swap(&mut s1, &mut s2);
            }
        }
        
        let mut factor = p as f64;
        for k in 1..=n {
            for r in 0..=p {
                derivatives[k][r] *= factor;
            }
            factor *= (p - k) as f64;
        }
        
        derivatives
    }
}

impl Curve for BSplineCurve {
    fn point(&self, t: f64) -> Point {
        let n = self.control_points.len() - 1;
        let p = self.degree;
        
        if t <= self.knots[p] {
            return self.control_points[0];
        }
        
        if t >= self.knots[n + 1] {
            return self.control_points[n];
        }
        
        let i = self.find_span(t);
        let N = self.basis_functions(i, t);
        
        let mut point = Point::origin();
        for j in 0..=p {
            point += self.control_points[i - p + j] * N[j];
        }
        
        point
    }

    fn derivative(&self, t: f64) -> Vector {
        let n = self.control_points.len() - 1;
        let p = self.degree;
        
        if t <= self.knots[p] || t >= self.knots[n + 1] {
            return Vector::zero();
        }
        
        let i = self.find_span(t);
        let derivatives = self.basis_function_derivatives(i, t, 1);
        
        let mut derivative = Vector::zero();
        for j in 0..=p {
            derivative += (self.control_points[i - p + j] - Point::origin()) * derivatives[1][j];
        }
        
        derivative
    }

    fn length(&self) -> f64 {
        // Approximate length using adaptive sampling
        let mut length = 0.0;
        let mut t_prev = 0.0;
        let mut p_prev = self.point(t_prev);
        
        let num_samples = 100;
        for i in 1..=num_samples {
            let t = i as f64 / num_samples as f64;
            let p = self.point(t);
            length += p_prev.distance(&p);
            t_prev = t;
            p_prev = p;
        }
        
        length
    }

    fn bounding_box(&self) -> (Point, Point) {
        // Use control points to determine bounding box
        let mut min_x = self.control_points[0].x;
        let mut min_y = self.control_points[0].y;
        let mut min_z = self.control_points[0].z;
        let mut max_x = self.control_points[0].x;
        let mut max_y = self.control_points[0].y;
        let mut max_z = self.control_points[0].z;
        
        for point in &self.control_points[1..] {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }
        
        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z)
        )
    }
}

impl Curve3D for BSplineCurve {
    fn point_3d(&self, t: f64) -> Point {
        self.point(t)
    }

    fn derivative_3d(&self, t: f64) -> Vector {
        self.derivative(t)
    }

    fn length_3d(&self) -> f64 {
        self.length()
    }

    fn bounding_box_3d(&self) -> (Point, Point) {
        self.bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_bspline_creation() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let result = BSplineCurve::clamped(control_points, 3);
        assert!(result.is_ok());
        
        let curve = result.unwrap();
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.control_points().len(), 4);
        assert_eq!(curve.knots().len(), 8); // 4 control points, degree 3: knots length = 4 + 3 + 1 = 8
    }

    #[test]
    fn test_bspline_point() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let curve = BSplineCurve::clamped(control_points, 3).unwrap();
        
        // Test start point
        let p0 = curve.point(0.0);
        assert!((p0.x - 0.0).abs() < 1e-10);
        assert!((p0.y - 0.0).abs() < 1e-10);
        
        // Test end point
        let p1 = curve.point(1.0);
        assert!((p1.x - 4.0).abs() < 1e-10);
        assert!((p1.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_bspline_derivative() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let curve = BSplineCurve::clamped(control_points, 3).unwrap();
        
        // Test derivative at mid point
        let deriv = curve.derivative(0.5);
        assert!((deriv.length() - 0.0).abs() > 1e-10); // Derivative should not be zero
    }
}
