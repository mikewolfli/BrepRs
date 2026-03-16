//! Curve enumeration type for dyn-compatible curve handling
//!
//! This module provides an enum-based approach to handle different curve types
//! without using trait objects, solving the dyn-compatibility issues with serde.

use crate::geometry::{Point, Vector};
use crate::topology::Curve;
use serde::{Deserialize, Serialize};

/// Enumeration of all supported curve types
///
/// This enum provides a dyn-compatible way to work with curves by wrapping
/// concrete curve implementations. It avoids the limitations of trait objects
/// while maintaining flexibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurveEnum {
    /// Line curve
    Line(super::line::Line),
    /// Circle curve
    Circle(super::circle::Circle),
    /// Ellipse curve
    Ellipse(super::ellipse::Ellipse),
    /// Bezier curve
    BezierCurve2D(super::bezier_curve2d::BezierCurve2D),
    /// NURBS curve
    NurbsCurve2D(super::nurbs_curve2d::NurbsCurve2D),
    /// Spiral curve
    Spiral(super::spiral::Spiral),
}

impl CurveEnum {
    /// Get the point on the curve at a parameter value
    pub fn value(&self, parameter: f64) -> Point {
        match self {
            CurveEnum::Line(c) => c.value(parameter),
            CurveEnum::Circle(c) => c.value(parameter),
            CurveEnum::Ellipse(c) => c.value(parameter),
            CurveEnum::BezierCurve2D(c) => c.value(parameter),
            CurveEnum::NurbsCurve2D(c) => c.value(parameter),
            CurveEnum::Spiral(c) => c.value(parameter),
        }
    }

    /// Get the derivative (tangent) at a parameter value
    pub fn derivative(&self, parameter: f64) -> Vector {
        match self {
            CurveEnum::Line(c) => c.derivative(parameter),
            CurveEnum::Circle(c) => c.derivative(parameter),
            CurveEnum::Ellipse(c) => c.derivative(parameter),
            CurveEnum::BezierCurve2D(c) => c.derivative(parameter),
            CurveEnum::NurbsCurve2D(c) => c.derivative(parameter),
            CurveEnum::Spiral(c) => c.derivative(parameter),
        }
    }

    /// Get the parameter range of the curve
    pub fn parameter_range(&self) -> (f64, f64) {
        match self {
            CurveEnum::Line(c) => c.parameter_range(),
            CurveEnum::Circle(c) => c.parameter_range(),
            CurveEnum::Ellipse(c) => c.parameter_range(),
            CurveEnum::BezierCurve2D(c) => c.parameter_range(),
            CurveEnum::NurbsCurve2D(c) => c.parameter_range(),
            CurveEnum::Spiral(c) => c.parameter_range(),
        }
    }

    /// Calculate the length of the curve
    pub fn length(&self) -> f64 {
        match self {
            CurveEnum::Line(c) => {
                // For line, calculate distance between two points
                let p1 = c.position(0.0);
                let p2 = c.position(1.0);
                let dx = p2.x - p1.x;
                let dy = p2.y - p1.y;
                let dz = p2.z - p1.z;
                (dx * dx + dy * dy + dz * dz).sqrt()
            },
            CurveEnum::Circle(c) => c.length(),
            CurveEnum::Ellipse(c) => c.length(),
            CurveEnum::BezierCurve2D(c) => {
                // Approximate length by sampling points along the curve
                let samples = 100;
                let (u_min, u_max) = c.parameter_range();
                let du = (u_max - u_min) / samples as f64;
                let mut length = 0.0;
                let mut prev_point = c.value(u_min);
                for i in 1..=samples {
                    let u = u_min + i as f64 * du;
                    let point = c.value(u);
                    let dx = point.x - prev_point.x;
                    let dy = point.y - prev_point.y;
                    let dz = point.z - prev_point.z;
                    length += (dx * dx + dy * dy + dz * dz).sqrt();
                    prev_point = point;
                }
                length
            },
            CurveEnum::NurbsCurve2D(c) => {
                // Approximate length by sampling points along the curve
                let samples = 100;
                let (u_min, u_max) = c.parameter_range();
                let du = (u_max - u_min) / samples as f64;
                let mut length = 0.0;
                let mut prev_point = c.value(u_min);
                for i in 1..=samples {
                    let u = u_min + i as f64 * du;
                    let point = c.value(u);
                    let dx = point.x - prev_point.x;
                    let dy = point.y - prev_point.y;
                    let dz = point.z - prev_point.z;
                    length += (dx * dx + dy * dy + dz * dz).sqrt();
                    prev_point = point;
                }
                length
            },
            CurveEnum::Spiral(c) => c.arc_length(1.0),
        }
    }

    /// Get the start point of the curve
    pub fn start_point(&self) -> Point {
        let (u_min, _) = self.parameter_range();
        self.value(u_min)
    }

    /// Get the end point of the curve
    pub fn end_point(&self) -> Point {
        let (_, u_max) = self.parameter_range();
        self.value(u_max)
    }

    /// Reverse the curve direction
    pub fn reversed(&self) -> Self {
        // For now, return a clone. Subclasses should implement proper reversal
        self.clone()
    }

    /// Check if the curve is closed
    pub fn is_closed(&self) -> bool {
        let start = self.start_point();
        let end = self.end_point();
        (start - end).magnitude() < 1e-10
    }

    /// Get the bounding box of the curve
    pub fn bounding_box(&self) -> (Point, Point) {
        // Sample points along the curve to approximate bounding box
        let samples = 100;
        let (u_min, u_max) = self.parameter_range();
        let du = (u_max - u_min) / samples as f64;

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for i in 0..=samples {
            let u = u_min + i as f64 * du;
            let p = self.value(u);
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            min_z = min_z.min(p.z);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
            max_z = max_z.max(p.z);
        }

        (
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }
}

// Implement Curve trait for CurveEnum to maintain backward compatibility
impl Curve for CurveEnum {
    fn value(&self, parameter: f64) -> Point {
        match self {
            CurveEnum::Line(c) => c.value(parameter),
            CurveEnum::Circle(c) => c.value(parameter),
            CurveEnum::Ellipse(c) => c.value(parameter),
            CurveEnum::BezierCurve2D(c) => c.value(parameter),
            CurveEnum::NurbsCurve2D(c) => c.value(parameter),
            CurveEnum::Spiral(c) => c.value(parameter),
        }
    }

    fn derivative(&self, parameter: f64) -> Vector {
        match self {
            CurveEnum::Line(c) => c.derivative(parameter),
            CurveEnum::Circle(c) => c.derivative(parameter),
            CurveEnum::Ellipse(c) => c.derivative(parameter),
            CurveEnum::BezierCurve2D(c) => c.derivative(parameter),
            CurveEnum::NurbsCurve2D(c) => c.derivative(parameter),
            CurveEnum::Spiral(c) => c.derivative(parameter),
        }
    }

    fn parameter_range(&self) -> (f64, f64) {
        match self {
            CurveEnum::Line(c) => c.parameter_range(),
            CurveEnum::Circle(c) => c.parameter_range(),
            CurveEnum::Ellipse(c) => c.parameter_range(),
            CurveEnum::BezierCurve2D(c) => c.parameter_range(),
            CurveEnum::NurbsCurve2D(c) => c.parameter_range(),
            CurveEnum::Spiral(c) => c.parameter_range(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_enum_line() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 1.0);
        let line = super::super::line::Line::from_points(&p1, &p2);
        let curve = CurveEnum::Line(line);

        let start = curve.start_point();
        assert!((start.x - 0.0).abs() < 1e-10);

        let end = curve.end_point();
        assert!((end.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_curve_enum_circle() {
        let circle = super::super::circle::Circle::new(
            Point::new(0.0, 0.0, 0.0),
            super::super::direction::Direction::from_vector(&super::super::vector::Vector::new(0.0, 0.0, 1.0)),
            1.0,
        );
        let curve = CurveEnum::Circle(circle);

        assert!(curve.is_closed());
    }
}