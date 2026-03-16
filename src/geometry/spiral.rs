//! Spiral curves
//!
//! This module provides functionality for creating and manipulating spiral curves,
//! including Archimedean, logarithmic, helical, and conical spirals.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::Curve;
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};
use std::f64::consts::PI;

/// Spiral types
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

impl Clone for SpiralType {
    fn clone(&self) -> Self {
        match self {
            SpiralType::Archimedean => SpiralType::Archimedean,
            SpiralType::Logarithmic => SpiralType::Logarithmic,
            SpiralType::Helical => SpiralType::Helical,
            SpiralType::Conical => SpiralType::Conical,
            SpiralType::Custom(_) => SpiralType::Custom(Box::new(|_| 0.0)),
        }
    }
}

impl std::fmt::Debug for SpiralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpiralType::Archimedean => write!(f, "Archimedean"),
            SpiralType::Logarithmic => write!(f, "Logarithmic"),
            SpiralType::Helical => write!(f, "Helical"),
            SpiralType::Conical => write!(f, "Conical"),
            SpiralType::Custom(_) => write!(f, "Custom(<function>"),
        }
    }
}

// Custom serialization for SpiralType
#[cfg(feature = "serde")]
impl Serialize for SpiralType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SpiralType::Archimedean => serializer.serialize_unit_variant("SpiralType", 0, "Archimedean"),
            SpiralType::Logarithmic => serializer.serialize_unit_variant("SpiralType", 1, "Logarithmic"),
            SpiralType::Helical => serializer.serialize_unit_variant("SpiralType", 2, "Helical"),
            SpiralType::Conical => serializer.serialize_unit_variant("SpiralType", 3, "Conical"),
            SpiralType::Custom(_) => serializer.serialize_unit_variant("SpiralType", 4, "Custom"),
        }
    }
}

// Custom deserialization for SpiralType
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SpiralType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SpiralTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for SpiralTypeVisitor {
            type Value = SpiralType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("variant of SpiralType")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "Archimedean" => Ok(SpiralType::Archimedean),
                    "Logarithmic" => Ok(SpiralType::Logarithmic),
                    "Helical" => Ok(SpiralType::Helical),
                    "Conical" => Ok(SpiralType::Conical),
                    "Custom" => Ok(SpiralType::Custom(Box::new(|_| 0.0))),
                    _ => Err(E::unknown_variant(value, &["Archimedean", "Logarithmic", "Helical", "Conical", "Custom"])),
                }
            }
        }

        deserializer.deserialize_str(SpiralTypeVisitor)
    }
}

/// Spiral curve
#[derive(Debug, Serialize, Deserialize)]
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

impl Clone for Spiral {
    fn clone(&self) -> Self {
        let spiral_type = match &self.spiral_type {
            SpiralType::Archimedean => SpiralType::Archimedean,
            SpiralType::Logarithmic => SpiralType::Logarithmic,
            SpiralType::Helical => SpiralType::Helical,
            SpiralType::Conical => SpiralType::Conical,
            SpiralType::Custom(_) => {
                // For custom functions, we can't clone the closure directly
                // Instead, we create a new closure that returns 0.0
                SpiralType::Custom(Box::new(|_| 0.0))
            }
        };

        Self {
            spiral_type,
            base_point: self.base_point,
            axis: self.axis,
            initial_direction: self.initial_direction,
            pitch: self.pitch,
            growth_rate: self.growth_rate,
            cone_angle: self.cone_angle,
        }
    }
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
        pitch_function: Box<dyn Fn(f64) -> f64 + Send + Sync>,
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
        let axis_dir = crate::geometry::Direction::from_vector(&self.axis);
        let axis_obj = crate::geometry::Axis::new(self.base_point, axis_dir);
        let radial_direction = self.initial_direction.rotated(&axis_obj, angle);
        let planar_position = radial_direction.scaled(radius);

        // Calculate position along the axis
        let axial_position = self.axis.scaled(self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate logarithmic spiral
    fn evaluate_logarithmic(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.pitch * (self.growth_rate * angle).exp();

        // Calculate position in the plane perpendicular to the axis
        let axis_dir = crate::geometry::Direction::from_vector(&self.axis);
        let axis_obj = crate::geometry::Axis::new(self.base_point, axis_dir);
        let radial_direction = self.initial_direction.rotated(&axis_obj, angle);
        let planar_position = radial_direction.scaled(radius);

        // Calculate position along the axis
        let axial_position = self.axis.scaled(self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate helical spiral
    fn evaluate_helical(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.growth_rate; // For helical, growth_rate is the radius

        // Calculate position in the plane perpendicular to the axis
        let axis_dir = crate::geometry::Direction::from_vector(&self.axis);
        let axis_obj = crate::geometry::Axis::new(self.base_point, axis_dir);
        let radial_direction = self.initial_direction.rotated(&axis_obj, angle);
        let planar_position = radial_direction.scaled(radius);

        // Calculate position along the axis
        let axial_position = self.axis.scaled(self.pitch * t);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate conical spiral
    fn evaluate_conical(&self, t: f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = self.pitch * angle * (self.cone_angle).tan();

        // Calculate position in the plane perpendicular to the axis
        let axis_dir = crate::geometry::Direction::from_vector(&self.axis);
        let axis_obj = crate::geometry::Axis::new(self.base_point, axis_dir);
        let radial_direction = self.initial_direction.rotated(&axis_obj, angle);
        let planar_position = radial_direction.scaled(radius);

        // Calculate position along the axis
        let axial_position = self.axis.scaled(self.pitch * angle);

        self.base_point + planar_position + axial_position
    }

    /// Evaluate custom spiral
    fn evaluate_custom(&self, t: f64, pitch_function: &dyn Fn(f64) -> f64) -> Point {
        let angle = 2.0 * PI * t;
        let radius = pitch_function(t);

        // Calculate position in the plane perpendicular to the axis
        let axis_dir = crate::geometry::Direction::from_vector(&self.axis);
        let axis_obj = crate::geometry::Axis::new(self.base_point, axis_dir);
        let radial_direction = self.initial_direction.rotated(&axis_obj, angle);
        let planar_position = radial_direction.scaled(radius);

        // Calculate position along the axis
        let axial_position = self.axis.scaled(self.pitch * t);

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
        let dt = (t2 - t1).scaled(1.0 / (2.0 * h));
        dt.magnitude()
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
            sum += weight * velocity.magnitude();
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
            let q = other.value(t);
            if (p - q).magnitude() < tolerance {
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
        // Create edge from first and last point (simplified)
        if points.len() >= 2 {
            Ok(TopoDsEdge::new(
                Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(points[0]),
                )),
                Handle::new(std::sync::Arc::new(
                    crate::topology::topods_vertex::TopoDsVertex::new(points[points.len() - 1]),
                )),
            ))
        } else {
            Err("Not enough points to create edge".to_string())
        }
    }

    /// Validate spiral parameters
    pub fn validate(&self) -> Result<(), String> {
        // Check if axis and initial direction are perpendicular
        let dot_product = self.axis.dot(&self.initial_direction);
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
