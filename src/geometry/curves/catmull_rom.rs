//! Catmull-Rom spline implementation
//! 
//! Catmull-Rom splines are interpolating splines that pass through all control points.

use super::{Curve, Curve3D};
use crate::geometry::{Point, Vector};

/// Catmull-Rom spline
#[derive(Debug, Clone, PartialEq)]
pub struct CatmullRomSpline {
    /// Control points
    control_points: Vec<Point>,
    /// Tension parameter (0.0 = uniform, 0.5 = centripetal, 1.0 = chordal)
    tension: f64,
    /// Alpha parameter for centripetal Catmull-Rom
    alpha: f64,
}

impl CatmullRomSpline {
    /// Create a new Catmull-Rom spline
    pub fn new(control_points: Vec<Point>, tension: f64) -> Self {
        Self {
            control_points,
            tension,
            alpha: 0.5, // Centripetal by default
        }
    }

    /// Create a uniform Catmull-Rom spline
    pub fn uniform(control_points: Vec<Point>) -> Self {
        Self::new(control_points, 0.0)
    }

    /// Create a centripetal Catmull-Rom spline
    pub fn centripetal(control_points: Vec<Point>) -> Self {
        let mut spline = Self::new(control_points, 0.5);
        spline.alpha = 0.5;
        spline
    }

    /// Create a chordal Catmull-Rom spline
    pub fn chordal(control_points: Vec<Point>) -> Self {
        let mut spline = Self::new(control_points, 1.0);
        spline.alpha = 1.0;
        spline
    }

    /// Get the control points
    pub fn control_points(&self) -> &[Point] {
        &self.control_points
    }

    /// Get the tension
    pub fn tension(&self) -> f64 {
        self.tension
    }

    /// Calculate parameter values using centripetal method
    fn calculate_parameters(&self) -> Vec<f64> {
        let mut params = vec![0.0];
        
        for i in 1..self.control_points.len() {
            let distance = self.control_points[i].distance(&self.control_points[i - 1]);
            let param = params[i - 1] + distance.powf(self.alpha);
            params.push(param);
        }
        
        // Normalize parameters to [0, 1]
        let max_param = params.last().unwrap();
        for i in 1..params.len() {
            params[i] /= max_param;
        }
        
        params
    }

    /// Find the segment index for a given parameter t
    fn find_segment(&self, t: f64) -> usize {
        let params = self.calculate_parameters();
        let clamped_t = t.max(0.0).min(1.0);
        
        for i in 1..params.len() {
            if clamped_t <= params[i] {
                return i - 1;
            }
        }
        
        params.len() - 2
    }

    /// Calculate the Catmull-Rom spline segment
    fn calculate_segment(&self, p0: &Point, p1: &Point, p2: &Point, p3: &Point, t: f64) -> Point {
        let t2 = t * t;
        let t3 = t2 * t;
        
        // Catmull-Rom basis matrix
        let m0 = (-self.tension * t3 + 2.0 * self.tension * t2 - self.tension * t);
        let m1 = ((2.0 - self.tension) * t3 + (self.tension - 3.0) * t2 + 1.0);
        let m2 = ((self.tension - 2.0) * t3 + (3.0 - 2.0 * self.tension) * t2 + self.tension * t);
        let m3 = (self.tension * t3 - self.tension * t2);
        
        p0 * m0 + p1 * m1 + p2 * m2 + p3 * m3
    }

    /// Calculate the derivative of the Catmull-Rom spline segment
    fn calculate_derivative(&self, p0: &Point, p1: &Point, p2: &Point, p3: &Point, t: f64) -> Vector {
        let t2 = t * t;
        
        // Derivative of Catmull-Rom basis matrix
        let m0 = (-3.0 * self.tension * t2 + 4.0 * self.tension * t - self.tension);
        let m1 = ((6.0 - 3.0 * self.tension) * t2 + (2.0 * self.tension - 6.0) * t);
        let m2 = ((3.0 * self.tension - 6.0) * t2 + (6.0 - 4.0 * self.tension) * t + self.tension);
        let m3 = (3.0 * self.tension * t2 - 2.0 * self.tension * t);
        
        (p0 * m0 + p1 * m1 + p2 * m2 + p3 * m3) - Point::origin()
    }
}

impl Curve for CatmullRomSpline {
    fn point(&self, t: f64) -> Point {
        if self.control_points.len() < 4 {
            if self.control_points.is_empty() {
                return Point::origin();
            } else if self.control_points.len() == 1 {
                return self.control_points[0];
            } else if self.control_points.len() == 2 {
                // Linear interpolation
                let clamped_t = t.max(0.0).min(1.0);
                return self.control_points[0] + (self.control_points[1] - self.control_points[0]) * clamped_t;
            } else if self.control_points.len() == 3 {
                // Use first three points, duplicate the first and last
                let p0 = self.control_points[0];
                let p1 = self.control_points[0];
                let p2 = self.control_points[1];
                let p3 = self.control_points[2];
                let clamped_t = t.max(0.0).min(1.0);
                return self.calculate_segment(&p0, &p1, &p2, &p3, clamped_t);
            }
        }
        
        let clamped_t = t.max(0.0).min(1.0);
        let segment_index = self.find_segment(clamped_t);
        
        // Get the four control points for this segment
        let p0 = if segment_index == 0 {
            &self.control_points[0]
        } else {
            &self.control_points[segment_index - 1]
        };
        
        let p1 = &self.control_points[segment_index];
        let p2 = &self.control_points[segment_index + 1];
        
        let p3 = if segment_index == self.control_points.len() - 2 {
            &self.control_points[self.control_points.len() - 1]
        } else {
            &self.control_points[segment_index + 2]
        };
        
        // Calculate the local parameter within the segment
        let params = self.calculate_parameters();
        let t_start = params[segment_index];
        let t_end = params[segment_index + 1];
        let local_t = (clamped_t - t_start) / (t_end - t_start);
        
        self.calculate_segment(p0, p1, p2, p3, local_t)
    }

    fn derivative(&self, t: f64) -> Vector {
        if self.control_points.len() < 4 {
            if self.control_points.len() <= 1 {
                return Vector::zero();
            } else if self.control_points.len() == 2 {
                // Constant derivative
                return (self.control_points[1] - self.control_points[0]).normalize();
            } else if self.control_points.len() == 3 {
                // Use first three points, duplicate the first and last
                let p0 = self.control_points[0];
                let p1 = self.control_points[0];
                let p2 = self.control_points[1];
                let p3 = self.control_points[2];
                let clamped_t = t.max(0.0).min(1.0);
                return self.calculate_derivative(&p0, &p1, &p2, &p3, clamped_t);
            }
        }
        
        let clamped_t = t.max(0.0).min(1.0);
        let segment_index = self.find_segment(clamped_t);
        
        // Get the four control points for this segment
        let p0 = if segment_index == 0 {
            &self.control_points[0]
        } else {
            &self.control_points[segment_index - 1]
        };
        
        let p1 = &self.control_points[segment_index];
        let p2 = &self.control_points[segment_index + 1];
        
        let p3 = if segment_index == self.control_points.len() - 2 {
            &self.control_points[self.control_points.len() - 1]
        } else {
            &self.control_points[segment_index + 2]
        };
        
        // Calculate the local parameter within the segment
        let params = self.calculate_parameters();
        let t_start = params[segment_index];
        let t_end = params[segment_index + 1];
        let local_t = (clamped_t - t_start) / (t_end - t_start);
        
        self.calculate_derivative(p0, p1, p2, p3, local_t)
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
        if self.control_points.is_empty() {
            return (Point::origin(), Point::origin());
        }
        
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

impl Curve3D for CatmullRomSpline {
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
    fn test_catmull_rom_creation() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let spline = CatmullRomSpline::centripetal(control_points);
        assert_eq!(spline.control_points().len(), 4);
        assert!((spline.tension() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_catmull_rom_point() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let spline = CatmullRomSpline::centripetal(control_points);
        
        // Test start point
        let p0 = spline.point(0.0);
        assert!((p0.x - 0.0).abs() < 1e-10);
        assert!((p0.y - 0.0).abs() < 1e-10);
        
        // Test end point
        let p1 = spline.point(1.0);
        assert!((p1.x - 4.0).abs() < 1e-10);
        assert!((p1.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_catmull_rom_derivative() {
        let control_points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 2.0, 0.0),
            Point::new(3.0, 3.0, 0.0),
            Point::new(4.0, 1.0, 0.0),
        ];
        
        let spline = CatmullRomSpline::centripetal(control_points);
        
        // Test derivative at mid point
        let deriv = spline.derivative(0.5);
        assert!((deriv.length() - 0.0).abs() > 1e-10); // Derivative should not be zero
    }
}
