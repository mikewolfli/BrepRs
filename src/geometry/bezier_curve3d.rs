use crate::foundation::types::StandardReal;
use crate::geometry::{Point, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BezierCurve3D {
    poles: Vec<Point>,
    weights: Vec<StandardReal>,
}

impl BezierCurve3D {
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
        if index >= -1 && (index as usize) < self.poles.len() {
            let insert_pos = (index + 1) as usize;
            self.poles.insert(insert_pos, pole);
            self.weights.insert(insert_pos, weight);
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

    pub fn evaluate(&self, parameter: StandardReal) -> Point {
        let mut temp_poles = self.poles.clone();
        let mut n = temp_poles.len();
        
        while n > 1 {
            for i in 0..n-1 {
                temp_poles[i] = temp_poles[i].barycenter(&temp_poles[i+1], parameter);
            }
            n -= 1;
        }
        
        temp_poles[0]
    }

    pub fn evaluate_derivative(&self, _parameter: StandardReal, order: i32) -> Vector {
        if order <= 0 || order > self.degree() {
            return Vector::new(0.0, 0.0, 0.0);
        }
        
        let mut temp_poles = self.poles.clone();
        let degree = self.degree();
        
        for _ in 0..order {
            let n = temp_poles.len();
            for i in 0..n-1 {
                let vector = temp_poles[i+1] - temp_poles[i];
                temp_poles[i] = temp_poles[i] + vector * degree as StandardReal;
            }
            temp_poles.pop();
        }
        
        let point = temp_poles[0];
        Vector::new(point.x, point.y, point.z)
    }

    pub fn split(&self, parameter: StandardReal) -> (BezierCurve3D, BezierCurve3D) {
        let mut left_poles = Vec::new();
        let mut right_poles = Vec::new();
        let mut temp_poles = self.poles.clone();
        
        let n = temp_poles.len();
        left_poles.push(temp_poles[0]);
        
        for i in 1..n {
            for j in 0..n-i {
                temp_poles[j] = temp_poles[j].barycenter(&temp_poles[j+1], parameter);
            }
            left_poles.push(temp_poles[0]);
            right_poles.insert(0, temp_poles[n-i-1]);
        }
        
        right_poles.insert(0, temp_poles[0]);
        
        let left_weights = left_poles.iter().map(|_| 1.0).collect();
        let right_weights = right_poles.iter().map(|_| 1.0).collect();
        
        (
            BezierCurve3D::with_weights(left_poles, left_weights),
            BezierCurve3D::with_weights(right_poles, right_weights)
        )
    }

    pub fn length(&self, _tolerance: StandardReal) -> StandardReal {
        let mut length = 0.0;
        let mut t = 0.0;
        let step = 0.01;
        
        while t < 1.0 {
            let next_t = if t + step > 1.0 { 1.0 } else { t + step };
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
    use crate::foundation::types::STANDARD_REAL_EPSILON;

    #[test]
    fn test_bezier_curve3d_creation() {
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 1.0),
            Point::new(2.0, 1.0, 2.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        
        let curve = BezierCurve3D::new(poles);
        assert_eq!(curve.degree(), 3);
        assert_eq!(curve.nb_poles(), 4);
    }

    #[test]
    fn test_bezier_curve3d_evaluate() {
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(2.0, 1.0, 0.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        
        let curve = BezierCurve3D::new(poles);
        let point = curve.evaluate(0.5);
        
        // For a cubic bezier with these control points, the midpoint should be at (1.5, 1.0, 0.0)
        assert!((point.x - 1.5).abs() < STANDARD_REAL_EPSILON);
        assert!((point.y - 1.0).abs() < STANDARD_REAL_EPSILON);
        assert!((point.z - 0.0).abs() < STANDARD_REAL_EPSILON);
    }

    #[test]
    fn test_bezier_curve3d_split() {
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 1.0),
            Point::new(2.0, 1.0, 2.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        
        let curve = BezierCurve3D::new(poles);
        let (left, right) = curve.split(0.5);
        
        assert_eq!(left.degree(), 3);
        assert_eq!(right.degree(), 3);
        assert_eq!(left.nb_poles(), 4);
        assert_eq!(right.nb_poles(), 4);
    }

    #[test]
    fn test_bezier_curve3d_length() {
        let poles = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
            Point::new(3.0, 0.0, 0.0)
        ];
        
        let curve = BezierCurve3D::new(poles);
        let length = curve.length(0.001);
        
        // For a straight line, the length should be 3.0
        assert!((length - 3.0).abs() < 0.01);
    }
}