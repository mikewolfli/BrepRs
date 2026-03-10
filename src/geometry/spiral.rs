//! Spiral curves
//!
//! This module provides functionality for creating and manipulating spiral curves,
//! including Archimedean, logarithmic, helical, and conical spirals.

use crate::geometry::{Point, Vector};
use crate::topology::Curve;
use std::f64::consts::PI;

/// Spiral types
#[derive(Debug)]
pub enum SpiralType {
    /// Archimedean spiral (constant pitch)
    Archimedean,
    /// Logarithmic spiral (variable pitch)
    Logarithmic,
    /// Helical spiral (3D)
    Helical,
    /// Conical spiral
    Conical,
    /// Custom spiral with user-defined pitch function
    Custom(Box<dyn Fn(f64) -> f64 + Send + Sync>),
}

/// Spiral curve
#[derive(Debug)]
pub struct Spiral {
    /// Spiral type
    spiral_type: SpiralType,
    /// Base point (origin)
    base_point: Point,
    /// Axis direction
    axis: Vector,
    /// Initial direction
    initial_direction: Vector,
    /// Pitch parameter
    pitch: f64,
    /// Growth rate (for logarithmic spiral)
    growth_rate: f64,
    /// Cone angle (for conical spiral)
    cone_angle: f64,
}

impl Spiral {
    /// Create a new Archimedean spiral
    pub fn new_archimedean(
        base_point: Point,
        axis: Vector,
        initial_direction: Vector,
        pitch: f64,
    ) -> Self {
        Self {
            spiral_type: SpiralType::Archimedean,
            base_point,
            axis: axis.normalized(),
            initial_direction: initial_direction.normalized(),
            pitch,
            growth_rate: 0.0,
            cone_angle: 0.0,
        }
    }

    /// Create a new logarithmic spiral
    pub fn new_logarithmic(
        base_point: Point,
        axis: Vector,
        initial_direction: Vector,
        pitch: f64,
        growth_rate: f64,
    ) -> Self {
        Self {
            spiral_type: SpiralType::Logarithmic,
            base_point,
            axis: axis.normalized(),
            initial_direction: initial_direction.normalized(),
            pitch,
            growth_rate,
            cone_angle: 0.0,
        }
    }

    /// Create a new helical spiral
    pub fn new_helical(
        base_point: Point,
        axis: Vector,
        initial_direction: Vector,
        pitch: f64,
        radius: f64,
    ) -> Self {
        // For helical spiral, pitch is the distance between turns along the axis
        // radius is the distance from the axis
        Self {
            spiral_type: SpiralType::Helical,
            base_point,
            axis: axis.normalized(),
            initial_direction: initial_direction.normalized(),
            pitch,
            growth_rate: radius,
            cone_angle: 0.0,
        }
    }

    /// Create a new conical spiral
    pub fn new_conical(
        base_point: Point,
        axis: Vector,
        initial_direction: Vector,
        pitch: f64,
        cone_angle: f64,
    ) -> Self {
        Self {
            spiral_type: SpiralType::Conical,
            base_point,
            axis: axis.normalized(),
            initial_direction: initial_direction.normalized(),
            pitch,
            growth_rate: 0.0,
            cone_angle,
        }
    }

    /// Create a new custom spiral with user-defined pitch function
    pub fn new_custom(
        base_point: Point,
        axis: Vector,
        initial_direction: Vector,
        pitch_function: Box<dyn Fn(f64) -> f64>,
    ) -> Self {
        Self {
            spiral_type: SpiralType::Custom(pitch_function),
            base_point,
            axis: axis.normalized(),
            initial_direction: initial_direction.normalized(),
            pitch: 0.0,
            growth_rate: 0.0,
            cone_angle: 0.0,
        }
    }

    /// Evaluate the spiral at parameter t
    pub fn evaluate(&self, t: f64) -> Point {
        match &self.spiral_type {
            SpiralType::Archimedean => self.evaluate_archimedean(t),
            SpiralType::Logarithmic => self.evaluate_logarithmic(t),
            SpiralType::Helical => self.evaluate_helical(t),
            SpiralType::Conical => self.evaluate_conical(t),
            SpiralType::Custom(f) => self.evaluate_custom(t, f),
        }
    }

    /// Evaluate Archimedean spiral
    fn evaluate_archimedean(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.pitch * angle;

        // Calculate position in the plane perpendicular to the axis
        let radial_direction = self.initial_direction.rotate_around_axis(self.axis, angle);
        let planar_position = radial_direction * radius;

        // Calculate position along the axis
        let axial_position = self.axis * (self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate logarithmic spiral
    fn evaluate_logarithmic(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.pitch * (self.growth_rate * angle).exp();

        // Calculate position in the plane perpendicular to the axis
        let radial_direction = self.initial_direction.rotate_around_axis(self.axis, angle);
        let planar_position = radial_direction * radius;

        // Calculate position along the axis
        let axial_position = self.axis * (self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate helical spiral
    fn evaluate_helical(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.growth_rate; // For helical, growth_rate is the radius

        // Calculate position in the plane perpendicular to the axis
        let radial_direction = self.initial_direction.rotate_around_axis(self.axis, angle);
        let planar_position = radial_direction * radius;

        // Calculate position along the axis
        let axial_position = self.axis * (self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate conical spiral
    fn evaluate_conical(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.pitch * angle * (self.cone_angle).tan();

        // Calculate position in the plane perpendicular to the axis
        let radial_direction = self.initial_direction.rotate_around_axis(self.axis, angle);
        let planar_position = radial_direction * radius;

        // Calculate position along the axis
        let axial_position = self.axis * (self.pitch * angle);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate custom spiral
    fn evaluate_custom(&self, t: f64, pitch_function: &dyn Fn(f64) -> f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = pitch_function(t);

        // Calculate position in the plane perpendicular to the axis
        let radial_direction = self.initial_direction.rotate_around_axis(self.axis, angle);
        let planar_position = radial_direction * radius;

        // Calculate position along the axis
        let axial_position = self.axis * (self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Calculate the tangent vector at parameter t
    pub fn tangent(&self, t: f64) -> Vector {
        // Finite difference approximation
        let h = 1e-6;
        let p1 = self.evaluate(t);
        let p2 = self.evaluate(t + h);
        (p2 - p1).normalized()
    }

    /// Calculate the curvature at parameter t
    pub fn curvature(&self, t: f64) -> f64 {
        // Finite difference approximation
        let h = 1e-6;
        let t1 = self.tangent(t - h);
        let t2 = self.tangent(t + h);
        let dt = (t2 - t1) / (2.0 * h);
        dt.length()
    }

    /// Calculate the arc length from t=0 to t=t
    pub fn arc_length(&self, t: f64) -> f64 {
        // Numerical integration using Simpson's rule
        let n = 1000;
        let h = t / n as f64;
        let mut sum = 0.0;

        for i in 0..=n {
            let ti = i as f64 * h;
            let weight = if i == 0 || i == n {
                1.0
            } else if i % 2 == 0 {
                2.0
            } else {
                4.0
            };

            let velocity = self.tangent(ti);
            sum += weight * velocity.length();
        }

        (h / 3.0) * sum
    }

    /// Find intersection with another curve
    pub fn intersect(&self, other: &dyn Curve, tolerance: f64) -> Vec<Point> {
        // Find intersection points by sampling
        let mut intersections = Vec::new();
        let n = 1000;
        for i in 0..n {
            let t = i as f64 / n as f64;
            let p = self.evaluate(t);
            let q = other.evaluate(t);
            if (p - q).length() < tolerance {
                intersections.push(p);
            }
        }
        intersections
    }

    /// Convert spiral to BRep curve
    pub fn to_brep(&self) -> Result<TopoDsEdge, String> {
        // Convert spiral to BRep edge by sampling points
        let n = 100;
        let mut points = Vec::new();
        for i in 0..=n {
            let t = i as f64 / n as f64;
            points.push(self.evaluate(t));
        }
        Ok(TopoDsEdge::from_points(points))
    }

    /// Validate spiral parameters
    pub fn validate(&self) -> Result<(), String> {
        // Check if axis and initial direction are perpendicular
        let dot_product = self.axis.dot(self.initial_direction);
        if dot_product.abs() > 1e-6 {
            return Err("Axis and initial direction must be perpendicular".to_string());
        }

        // Check if pitch is positive
        match &self.spiral_type {
            SpiralType::Archimedean
            | SpiralType::Logarithmic
            | SpiralType::Helical
            | SpiralType::Conical => {
                if self.pitch <= 0.0 {
                    return Err("Pitch must be positive".to_string());
                }
            }
            _ => {}
        }

        // Check if growth rate is positive for logarithmic spiral
        if let SpiralType::Logarithmic = &self.spiral_type {
            if self.growth_rate <= 0.0 {
                return Err("Growth rate must be positive for logarithmic spiral".to_string());
            }
        }

        // Check if radius is positive for helical spiral
        if let SpiralType::Helical = &self.spiral_type {
            if self.growth_rate <= 0.0 {
                return Err("Radius must be positive for helical spiral".to_string());
            }
        }

        // Check if cone angle is between 0 and 90 degrees
        if let SpiralType::Conical = &self.spiral_type {
            if self.cone_angle <= 0.0 || self.cone_angle >= 90.0 {
                return Err("Cone angle must be between 0 and 90 degrees".to_string());
            }
        }

        Ok(())
    }
}

impl Curve for Spiral {
    fn value(&self, parameter: f64) -> Point {
        self.evaluate(parameter)
    }

    fn derivative(&self, parameter: f64) -> Vector {
        self.tangent(parameter)
    }

    fn parameter_range(&self) -> (f64, f64) {
        (0.0, 1.0)
    }
}

// Import TopoDsEdge for BRep conversion
use crate::topology::topods_edge::TopoDsEdge;
