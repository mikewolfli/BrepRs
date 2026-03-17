//! Polyline implementation
//! 
//! A polyline is a continuous curve composed of multiple line segments.

use super::{Curve, Curve3D};
use crate::geometry::{Point, Vector};

/// Polyline curve
#[derive(Debug, Clone, PartialEq)]
pub struct Polyline {
    /// Control points defining the polyline
    points: Vec<Point>,
    /// Length of the polyline
    length: f64,
}

impl Polyline {
    /// Create a new polyline from a list of points
    pub fn new(points: Vec<Point>) -> Self {
        let length = Self::calculate_length(&points);
        Self {
            points,
            length,
        }
    }

    /// Calculate the total length of the polyline
    fn calculate_length(points: &[Point]) -> f64 {
        if points.len() < 2 {
            return 0.0;
        }

        let mut total_length = 0.0;
        for i in 1..points.len() {
            total_length += points[i - 1].distance(&points[i]);
        }
        total_length
    }

    /// Get the control points
    pub fn points(&self) -> &[Point] {
        &self.points
    }

    /// Get the number of segments
    pub fn num_segments(&self) -> usize {
        if self.points.len() < 2 {
            0
        } else {
            self.points.len() - 1
        }
    }

    /// Get a specific segment
    pub fn segment(&self, index: usize) -> Option<(Point, Point)> {
        if index < self.num_segments() {
            Some((self.points[index], self.points[index + 1]))
        } else {
            None
        }
    }

    /// Reverse the polyline
    pub fn reverse(&mut self) {
        self.points.reverse();
    }

    /// Append a point to the polyline
    pub fn append(&mut self, point: Point) {
        if !self.points.is_empty() {
            let last_point = self.points.last().unwrap();
            self.length += last_point.distance(&point);
        }
        self.points.push(point);
    }

    /// Insert a point at the specified position
    pub fn insert(&mut self, index: usize, point: Point) {
        if index > self.points.len() {
            return;
        }

        if self.points.is_empty() {
            self.points.push(point);
        } else if index == 0 {
            let first_point = self.points[0];
            self.length += point.distance(&first_point);
            self.points.insert(0, point);
        } else if index == self.points.len() {
            self.append(point);
        } else {
            let prev_point = self.points[index - 1];
            let next_point = self.points[index];
            
            // Subtract the old segment length
            self.length -= prev_point.distance(&next_point);
            
            // Add the new segment lengths
            self.length += prev_point.distance(&point);
            self.length += point.distance(&next_point);
            
            self.points.insert(index, point);
        }
    }

    /// Remove a point at the specified position
    pub fn remove(&mut self, index: usize) {
        if index >= self.points.len() || self.points.len() <= 1 {
            return;
        }

        if index == 0 {
            let first_point = self.points[0];
            let second_point = self.points[1];
            self.length -= first_point.distance(&second_point);
        } else if index == self.points.len() - 1 {
            let second_last_point = self.points[self.points.len() - 2];
            let last_point = self.points[self.points.len() - 1];
            self.length -= second_last_point.distance(&last_point);
        } else {
            let prev_point = self.points[index - 1];
            let current_point = self.points[index];
            let next_point = self.points[index + 1];
            
            // Subtract the old segment lengths
            self.length -= prev_point.distance(&current_point);
            self.length -= current_point.distance(&next_point);
            
            // Add the new segment length
            self.length += prev_point.distance(&next_point);
        }

        self.points.remove(index);
    }

    /// Simplify the polyline using the Douglas-Peucker algorithm
    pub fn simplify(&mut self, epsilon: f64) {
        if self.points.len() <= 2 {
            return;
        }

        let simplified = Self::douglas_peucker(&self.points, epsilon);
        self.points = simplified;
        self.length = Self::calculate_length(&self.points);
    }

    /// Douglas-Peucker algorithm for polyline simplification
    fn douglas_peucker(points: &[Point], epsilon: f64) -> Vec<Point> {
        if points.len() <= 2 {
            return points.to_vec();
        }

        // Find the point with the maximum distance from the line segment
        let mut max_dist = 0.0;
        let mut max_idx = 1;
        
        let start = points[0];
        let end = points[points.len() - 1];
        
        for (i, point) in points.iter().enumerate().skip(1).take(points.len() - 2) {
            let dist = Self::point_to_line_distance(point, &start, &end);
            if dist > max_dist {
                max_dist = dist;
                max_idx = i;
            }
        }

        if max_dist > epsilon {
            // Recursively simplify the two parts
            let left = Self::douglas_peucker(&points[0..max_idx + 1], epsilon);
            let right = Self::douglas_peucker(&points[max_idx..], epsilon);
            
            // Combine the results, removing the duplicate point
            let mut result = left;
            result.pop();
            result.extend(right);
            result
        } else {
            // Simplify to just the start and end points
            vec![start, end]
        }
    }

    /// Calculate the distance from a point to a line segment
    fn point_to_line_distance(point: &Point, line_start: &Point, line_end: &Point) -> f64 {
        let line_vec = line_end - line_start;
        let point_vec = point - line_start;
        
        let line_length_squared = line_vec.length_squared();
        if line_length_squared < 1e-10 {
            return point_vec.length();
        }
        
        let t = point_vec.dot(&line_vec) / line_length_squared;
        let t_clamped = t.max(0.0).min(1.0);
        
        let closest_point = line_start + line_vec * t_clamped;
        (point - closest_point).length()
    }
}

impl Curve for Polyline {
    fn point(&self, t: f64) -> Point {
        if self.points.is_empty() {
            return Point::origin();
        }
        
        if self.points.len() == 1 {
            return self.points[0];
        }
        
        let clamped_t = t.max(0.0).min(1.0);
        let target_length = clamped_t * self.length;
        
        let mut current_length = 0.0;
        
        for i in 1..self.points.len() {
            let segment_start = self.points[i - 1];
            let segment_end = self.points[i];
            let segment_length = segment_start.distance(&segment_end);
            
            if current_length + segment_length >= target_length {
                let segment_t = (target_length - current_length) / segment_length;
                return segment_start + (segment_end - segment_start) * segment_t;
            }
            
            current_length += segment_length;
        }
        
        // Fallback to last point
        *self.points.last().unwrap()
    }

    fn derivative(&self, t: f64) -> Vector {
        if self.points.len() < 2 {
            return Vector::zero();
        }
        
        let clamped_t = t.max(0.0).min(1.0);
        let target_length = clamped_t * self.length;
        
        let mut current_length = 0.0;
        
        for i in 1..self.points.len() {
            let segment_start = self.points[i - 1];
            let segment_end = self.points[i];
            let segment_length = segment_start.distance(&segment_end);
            
            if current_length + segment_length >= target_length {
                let direction = segment_end - segment_start;
                return direction.normalize();
            }
            
            current_length += segment_length;
        }
        
        // Fallback to last segment direction
        let last_segment = self.points.last().unwrap() - self.points[self.points.len() - 2];
        last_segment.normalize()
    }

    fn length(&self) -> f64 {
        self.length
    }

    fn bounding_box(&self) -> (Point, Point) {
        if self.points.is_empty() {
            return (Point::origin(), Point::origin());
        }
        
        let mut min_x = self.points[0].x;
        let mut min_y = self.points[0].y;
        let mut min_z = self.points[0].z;
        let mut max_x = self.points[0].x;
        let mut max_y = self.points[0].y;
        let mut max_z = self.points[0].z;
        
        for point in &self.points[1..] {
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

impl Curve3D for Polyline {
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
    fn test_polyline_creation() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.0, 1.0, 0.0),
            Point::new(0.0, 1.0, 0.0),
        ];
        
        let polyline = Polyline::new(points);
        assert_eq!(polyline.num_segments(), 3);
        assert!((polyline.length() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_polyline_point() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(2.0, 0.0, 0.0),
            Point::new(2.0, 2.0, 0.0),
        ];
        
        let polyline = Polyline::new(points);
        
        // Test start point
        let p0 = polyline.point(0.0);
        assert!((p0.x - 0.0).abs() < 1e-10);
        assert!((p0.y - 0.0).abs() < 1e-10);
        
        // Test middle of first segment
        let p1 = polyline.point(0.25);
        assert!((p1.x - 1.0).abs() < 1e-10);
        assert!((p1.y - 0.0).abs() < 1e-10);
        
        // Test middle of second segment
        let p2 = polyline.point(0.75);
        assert!((p2.x - 2.0).abs() < 1e-10);
        assert!((p2.y - 1.0).abs() < 1e-10);
        
        // Test end point
        let p3 = polyline.point(1.0);
        assert!((p3.x - 2.0).abs() < 1e-10);
        assert!((p3.y - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_polyline_simplify() {
        let points = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.5, 0.1, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(1.5, -0.1, 0.0),
            Point::new(2.0, 0.0, 0.0),
        ];
        
        let mut polyline = Polyline::new(points);
        let original_length = polyline.points.len();
        
        polyline.simplify(0.5);
        
        // After simplification, we should have fewer points
        assert!(polyline.points.len() < original_length);
    }

    #[test]
    fn test_polyline_append() {
        let mut polyline = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        ]);
        
        let original_length = polyline.length();
        polyline.append(Point::new(1.0, 1.0, 0.0));
        
        assert_eq!(polyline.num_segments(), 2);
        assert!((polyline.length() - (original_length + 1.0)).abs() < 1e-10);
    }
}
