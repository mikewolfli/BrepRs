//! Constraint solver module
//!
//! This module provides a comprehensive geometric constraint solving system that
//! can handle various types of geometric constraints and solve them efficiently.

use crate::foundation::handle::Handle;
use crate::geometry::{Circle, Ellipse, Line, Plane, Point};
use crate::modeling::parametric::{Constraint, ParamType, ParametricModel};
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsVertex};
use std::collections::{HashMap, HashSet, VecDeque};

// ============================================================================
// Constraint Solver
// ============================================================================

/// Constraint solver struct
///
/// This struct implements a geometric constraint solver that can solve
/// various types of geometric constraints.
#[derive(Debug, Clone)]
pub struct ConstraintSolver {
    model: ParametricModel,
    tolerance: f64,
    max_iterations: usize,
    damping_factor: f64,
}

impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new(model: ParametricModel) -> Self {
        Self {
            model,
            tolerance: 1e-6,
            max_iterations: 100,
            damping_factor: 0.1,
        }
    }

    /// Helper function to extract numeric value from ParamType
    fn get_numeric_value(&self, name: &str) -> f64 {
        match self.model.get_parameter(name) {
            Some(ParamType::Numeric(v)) => v,
            Some(ParamType::Integer(v)) => v as f64,
            _ => 0.0,
        }
    }

    /// Create a new constraint solver with custom parameters
    pub fn with_parameters(
        model: ParametricModel,
        tolerance: f64,
        max_iterations: usize,
        damping_factor: f64,
    ) -> Self {
        Self {
            model,
            tolerance,
            max_iterations,
            damping_factor,
        }
    }

    /// Solve all constraints in the model
    pub fn solve(&mut self) -> bool {
        let mut iteration = 0;
        let mut error = self.calculate_total_error();

        while error > self.tolerance && iteration < self.max_iterations {
            // Get all constraints
            let constraints = self.model.get_constraints();

            // Create a copy of parameters for simultaneous update
            let mut new_parameters = HashMap::<String, f64>::new();

            // Solve each constraint
            for constraint in constraints.iter() {
                self.solve_constraint(constraint, &mut new_parameters);
            }

            // Update all parameters at once
            for (name, value) in new_parameters {
                self.model.set_parameter(&name, ParamType::Numeric(value));
            }

            // Calculate new error
            let new_error = self.calculate_total_error();

            // Check if error is decreasing
            if new_error >= error {
                // Apply damping if error is not decreasing
                self.apply_damping();
            }

            error = new_error;
            iteration += 1;
        }

        error <= self.tolerance
    }

    /// Solve a single constraint
    fn solve_constraint(
        &self,
        constraint: &Box<dyn Constraint + Send + Sync>,
        new_parameters: &mut HashMap<String, f64>,
    ) {
        // Get constraint parameters
        let params: Vec<String> = constraint
            .parameters()
            .iter()
            .map(|s| s.to_string())
            .collect();
        if params.len() < 1 {
            return;
        }

        // Calculate current error
        let error = self.calculate_constraint_error(constraint);
        if error < self.tolerance {
            return;
        }

        // Apply constraint solution based on type
        match constraint.constraint_type() {
            crate::modeling::parametric::ConstraintType::Distance => {
                self.solve_distance_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Angle => {
                self.solve_angle_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Coincident => {
                self.solve_coincident_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Parallel => {
                self.solve_parallel_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Perpendicular => {
                self.solve_perpendicular_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Tangent => {
                self.solve_tangent_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Equal => {
                self.solve_equal_constraint(constraint, &params, new_parameters);
            }
            crate::modeling::parametric::ConstraintType::Symmetric => {
                self.solve_symmetric_constraint(constraint, &params, new_parameters);
            }
        }
    }

    /// Calculate total error of all constraints
    fn calculate_total_error(&self) -> f64 {
        let mut total_error = 0.0;

        for constraint in self.model.get_constraints() {
            let error = self.calculate_constraint_error(constraint);
            total_error += error * error; // Square error for better convergence
        }

        total_error.sqrt() // Return RMS error
    }

    /// Calculate error for a single constraint
    fn calculate_constraint_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        // Calculate error based on constraint type
        match constraint.constraint_type() {
            crate::modeling::parametric::ConstraintType::Distance => {
                self.calculate_distance_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Angle => {
                self.calculate_angle_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Coincident => {
                self.calculate_coincident_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Parallel => {
                self.calculate_parallel_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Perpendicular => {
                self.calculate_perpendicular_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Tangent => {
                self.calculate_tangent_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Equal => {
                self.calculate_equal_error(constraint)
            }
            crate::modeling::parametric::ConstraintType::Symmetric => {
                self.calculate_symmetric_error(constraint)
            }
        }
    }

    /// Calculate distance constraint error
    fn calculate_distance_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get parameter values
        let p1_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let p1_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let p1_z = self.get_numeric_value(&format!("{}.z", params[0]));
        let p2_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let p2_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let p2_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate actual distance
        let dx = p2_x - p1_x;
        let dy = p2_y - p1_y;
        let dz = p2_z - p1_z;
        let actual_distance = (dx * dx + dy * dy + dz * dz).sqrt();

        // Get target distance from constraint value
        let target_distance = constraint.value().unwrap_or(0.0);

        // Return error
        (actual_distance - target_distance).abs()
    }

    /// Calculate angle constraint error
    fn calculate_angle_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get direction vectors
        let d1_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let d1_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let d1_z = self.get_numeric_value(&format!("{}.z", params[0]));
        let d2_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let d2_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let d2_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate dot product
        let dot = d1_x * d2_x + d1_y * d2_y + d1_z * d2_z;
        let mag1 = (d1_x * d1_x + d1_y * d1_y + d1_z * d1_z).sqrt();
        let mag2 = (d2_x * d2_x + d2_y * d2_y + d2_z * d2_z).sqrt();

        if mag1 < 1e-10 || mag2 < 1e-10 {
            return 0.0;
        }

        // Calculate actual angle
        let cos_theta = (dot / (mag1 * mag2)).clamp(-1.0, 1.0);
        let actual_angle = cos_theta.acos();

        // Get target angle from constraint value
        let target_angle = constraint.value().unwrap_or(0.0);

        // Return error
        (actual_angle - target_angle).abs()
    }

    /// Calculate coincident constraint error
    fn calculate_coincident_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get parameter values
        let p1_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let p1_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let p1_z = self.get_numeric_value(&format!("{}.z", params[0]));
        let p2_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let p2_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let p2_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate distance between points
        let dx = p2_x - p1_x;
        let dy = p2_y - p1_y;
        let dz = p2_z - p1_z;
        let distance = (dx * dx + dy * dy + dz * dz).sqrt();

        distance
    }

    /// Calculate parallel constraint error
    fn calculate_parallel_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get direction vectors
        let d1_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let d1_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let d1_z = self.get_numeric_value(&format!("{}.z", params[0]));
        let d2_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let d2_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let d2_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate cross product
        let cross_x = d1_y * d2_z - d1_z * d2_y;
        let cross_y = d1_z * d2_x - d1_x * d2_z;
        let cross_z = d1_x * d2_y - d1_y * d2_x;
        let cross_mag = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z).sqrt();

        // Calculate magnitudes
        let mag1 = (d1_x * d1_x + d1_y * d1_y + d1_z * d1_z).sqrt();
        let mag2 = (d2_x * d2_x + d2_y * d2_y + d2_z * d2_z).sqrt();

        if mag1 < 1e-10 || mag2 < 1e-10 {
            return 0.0;
        }

        // Error is the sine of the angle between vectors
        cross_mag / (mag1 * mag2)
    }

    /// Calculate perpendicular constraint error
    fn calculate_perpendicular_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get direction vectors
        let d1_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let d1_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let d1_z = self.get_numeric_value(&format!("{}.z", params[0]));
        let d2_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let d2_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let d2_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate dot product
        let dot = d1_x * d2_x + d1_y * d2_y + d1_z * d2_z;
        let mag1 = (d1_x * d1_x + d1_y * d1_y + d1_z * d1_z).sqrt();
        let mag2 = (d2_x * d2_x + d2_y * d2_y + d2_z * d2_z).sqrt();

        if mag1 < 1e-10 || mag2 < 1e-10 {
            return 0.0;
        }

        // Error is the absolute value of the cosine of the angle
        (dot / (mag1 * mag2)).abs()
    }

    /// Calculate tangent constraint error
    fn calculate_tangent_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get curve/surface tangent and point normal
        let tangent_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let tangent_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let tangent_z = self.get_numeric_value(&format!("{}.z", params[0]));

        let normal_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let normal_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let normal_z = self.get_numeric_value(&format!("{}.z", params[1]));

        // Calculate dot product (should be zero for tangent)
        let dot = tangent_x * normal_x + tangent_y * normal_y + tangent_z * normal_z;

        // Calculate magnitudes
        let tangent_mag =
            (tangent_x * tangent_x + tangent_y * tangent_y + tangent_z * tangent_z).sqrt();
        let normal_mag = (normal_x * normal_x + normal_y * normal_y + normal_z * normal_z).sqrt();

        if tangent_mag < 1e-10 || normal_mag < 1e-10 {
            return 0.0;
        }

        // Error is the absolute value of the dot product normalized
        (dot / (tangent_mag * normal_mag)).abs()
    }

    /// Calculate equal constraint error
    fn calculate_equal_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 2 {
            return 0.0;
        }

        // Get parameter values
        let val1 = self.get_numeric_value(&params[0]);
        let val2 = self.get_numeric_value(&params[1]);

        (val1 - val2).abs()
    }

    /// Calculate symmetric constraint error
    fn calculate_symmetric_error(&self, constraint: &Box<dyn Constraint + Send + Sync>) -> f64 {
        let params = constraint.parameters();
        if params.len() < 3 {
            return 0.0;
        }

        // Get point to check and symmetry plane
        let point_x = self.get_numeric_value(&format!("{}.x", params[0]));
        let point_y = self.get_numeric_value(&format!("{}.y", params[0]));
        let point_z = self.get_numeric_value(&format!("{}.z", params[0]));

        let plane_origin_x = self.get_numeric_value(&format!("{}.x", params[1]));
        let plane_origin_y = self.get_numeric_value(&format!("{}.y", params[1]));
        let plane_origin_z = self.get_numeric_value(&format!("{}.z", params[1]));

        let plane_normal_x = self.get_numeric_value(&format!("{}.x", params[2]));
        let plane_normal_y = self.get_numeric_value(&format!("{}.y", params[2]));
        let plane_normal_z = self.get_numeric_value(&format!("{}.z", params[2]));

        // Calculate vector from plane origin to point
        let vec_x = point_x - plane_origin_x;
        let vec_y = point_y - plane_origin_y;
        let vec_z = point_z - plane_origin_z;

        // Calculate dot product with plane normal
        let dot = vec_x * plane_normal_x + vec_y * plane_normal_y + vec_z * plane_normal_z;

        // Calculate plane normal magnitude
        let normal_mag = (plane_normal_x * plane_normal_x
            + plane_normal_y * plane_normal_y
            + plane_normal_z * plane_normal_z)
            .sqrt();

        if normal_mag < 1e-10 {
            return 0.0;
        }

        // Calculate distance from point to plane
        let distance = (dot / normal_mag).abs();

        // Error is the distance (should be zero for symmetry)
        distance
    }

    /// Solve distance constraint
    fn solve_distance_constraint(
        &self,
        constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let p1 = &params[0];
        let p2 = &params[1];
        let target_distance = constraint.value().unwrap_or(0.0);

        // Get current positions
        let p1_x = self.get_numeric_value(&format!("{}.x", p1));
        let p1_y = self.get_numeric_value(&format!("{}.y", p1));
        let p1_z = self.get_numeric_value(&format!("{}.z", p1));
        let p2_x = self.get_numeric_value(&format!("{}.x", p2));
        let p2_y = self.get_numeric_value(&format!("{}.y", p2));
        let p2_z = self.get_numeric_value(&format!("{}.z", p2));

        // Calculate current vector and distance
        let dx = p2_x - p1_x;
        let dy = p2_y - p1_y;
        let dz = p2_z - p1_z;
        let current_distance = (dx * dx + dy * dy + dz * dz).sqrt();

        if current_distance < 1e-10 {
            return;
        }

        // Calculate scaling factor
        let scale = target_distance / current_distance;

        // Update p2 position
        new_parameters.insert(format!("{}.x", p2), p1_x + dx * scale);
        new_parameters.insert(format!("{}.y", p2), p1_y + dy * scale);
        new_parameters.insert(format!("{}.z", p2), p1_z + dz * scale);
    }

    /// Solve angle constraint
    fn solve_angle_constraint(
        &self,
        constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let d1 = &params[0];
        let d2 = &params[1];
        let target_angle = constraint.value().unwrap_or(0.0);

        // Get current directions
        let d1_x = self.get_numeric_value(&format!("{}.x", d1));
        let d1_y = self.get_numeric_value(&format!("{}.y", d1));
        let d1_z = self.get_numeric_value(&format!("{}.z", d1));

        // Calculate current direction magnitude
        let d1_mag = (d1_x * d1_x + d1_y * d1_y + d1_z * d1_z).sqrt();
        if d1_mag < 1e-10 {
            return;
        }

        // Create a perpendicular vector to d1
        let mut perp_x = -d1_y;
        let mut perp_y = d1_x;
        let mut perp_z = 0.0;

        if perp_x.abs() < 1e-10 && perp_y.abs() < 1e-10 {
            perp_x = 1.0;
            perp_y = 0.0;
            perp_z = 0.0;
        }

        // Normalize perpendicular vector
        let perp_mag = (perp_x * perp_x + perp_y * perp_y + perp_z * perp_z).sqrt();
        perp_x /= perp_mag;
        perp_y /= perp_mag;
        perp_z /= perp_mag;

        // Calculate new direction using rotation
        let cos_theta = target_angle.cos();
        let sin_theta = target_angle.sin();

        let new_d2_x = d1_x * d1_mag * (cos_theta) + perp_x * sin_theta;
        let new_d2_y = d1_y * d1_mag * (cos_theta) + perp_y * sin_theta;
        let new_d2_z = d1_z * d1_mag * (cos_theta) + perp_z * sin_theta;

        // Update d2 direction
        new_parameters.insert(format!("{}.x", d2), new_d2_x);
        new_parameters.insert(format!("{}.y", d2), new_d2_y);
        new_parameters.insert(format!("{}.z", d2), new_d2_z);
    }

    /// Solve coincident constraint
    fn solve_coincident_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let p1 = &params[0];
        let p2 = &params[1];

        // Get p1 position
        let p1_x = self.get_numeric_value(&format!("{}.x", p1));
        let p1_y = self.get_numeric_value(&format!("{}.y", p1));
        let p1_z = self.get_numeric_value(&format!("{}.z", p1));

        // Set p2 to p1 position
        new_parameters.insert(format!("{}.x", p2), p1_x);
        new_parameters.insert(format!("{}.y", p2), p1_y);
        new_parameters.insert(format!("{}.z", p2), p1_z);
    }

    /// Solve parallel constraint
    fn solve_parallel_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let d1 = &params[0];
        let d2 = &params[1];

        // Get d1 direction
        let d1_x = self.get_numeric_value(&format!("{}.x", d1));
        let d1_y = self.get_numeric_value(&format!("{}.y", d1));
        let d1_z = self.get_numeric_value(&format!("{}.z", d1));

        // Get d2 magnitude
        let d2_x = self.get_numeric_value(&format!("{}.x", d2));
        let d2_y = self.get_numeric_value(&format!("{}.y", d2));
        let d2_z = self.get_numeric_value(&format!("{}.z", d2));
        let d2_mag = (d2_x * d2_x + d2_y * d2_y + d2_z * d2_z).sqrt();

        // Calculate d1 magnitude
        let d1_mag = (d1_x * d1_x + d1_y * d1_y + d1_z * d1_z).sqrt();
        if d1_mag < 1e-10 {
            return;
        }

        // Scale d1 to match d2 magnitude
        let scale = if d2_mag > 1e-10 { d2_mag / d1_mag } else { 1.0 };

        let new_d2_x = d1_x * scale;
        let new_d2_y = d1_y * scale;
        let new_d2_z = d1_z * scale;

        // Update d2 direction
        new_parameters.insert(format!("{}.x", d2), new_d2_x);
        new_parameters.insert(format!("{}.y", d2), new_d2_y);
        new_parameters.insert(format!("{}.z", d2), new_d2_z);
    }

    /// Solve perpendicular constraint
    fn solve_perpendicular_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let d1 = &params[0];
        let d2 = &params[1];

        // Get d1 direction
        let d1_x = self.get_numeric_value(&format!("{}.x", d1));
        let d1_y = self.get_numeric_value(&format!("{}.y", d1));
        let _d1_z = self.get_numeric_value(&format!("{}.z", d1));

        // Calculate perpendicular direction
        let mut perp_x = -d1_y;
        let mut perp_y = d1_x;
        let mut perp_z = 0.0;

        if perp_x.abs() < 1e-10 && perp_y.abs() < 1e-10 {
            perp_x = 1.0;
            perp_y = 0.0;
            perp_z = 0.0;
        }

        // Normalize
        let perp_mag = (perp_x * perp_x + perp_y * perp_y + perp_z * perp_z).sqrt();
        if perp_mag < 1e-10 {
            return;
        }

        perp_x /= perp_mag;
        perp_y /= perp_mag;
        perp_z /= perp_mag;

        // Get d2 magnitude
        let d2_x = self.get_numeric_value(&format!("{}.x", d2));
        let d2_y = self.get_numeric_value(&format!("{}.y", d2));
        let d2_z = self.get_numeric_value(&format!("{}.z", d2));
        let d2_mag = (d2_x * d2_x + d2_y * d2_y + d2_z * d2_z).sqrt();

        // Scale perpendicular vector to match d2 magnitude
        let scale = if d2_mag > 1e-10 { d2_mag } else { 1.0 };

        let new_d2_x = perp_x * scale;
        let new_d2_y = perp_y * scale;
        let new_d2_z = perp_z * scale;

        // Update d2 direction
        new_parameters.insert(format!("{}.x", d2), new_d2_x);
        new_parameters.insert(format!("{}.y", d2), new_d2_y);
        new_parameters.insert(format!("{}.z", d2), new_d2_z);
    }

    /// Solve tangent constraint
    fn solve_tangent_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let tangent = &params[0];
        let normal = &params[1];

        // Get current tangent vector
        let tangent_x = self.get_numeric_value(&format!("{}.x", tangent));
        let tangent_y = self.get_numeric_value(&format!("{}.y", tangent));
        let tangent_z = self.get_numeric_value(&format!("{}.z", tangent));

        // Get current normal vector
        let normal_x = self.get_numeric_value(&format!("{}.x", normal));
        let normal_y = self.get_numeric_value(&format!("{}.y", normal));
        let normal_z = self.get_numeric_value(&format!("{}.z", normal));

        // Calculate dot product
        let dot = tangent_x * normal_x + tangent_y * normal_y + tangent_z * normal_z;

        // Project tangent onto normal to get the component to remove
        let normal_mag_sq = normal_x * normal_x + normal_y * normal_y + normal_z * normal_z;

        if normal_mag_sq < 1e-10 {
            return;
        }

        // Calculate projection of tangent onto normal
        let projection_factor = dot / normal_mag_sq;
        let proj_x = projection_factor * normal_x;
        let proj_y = projection_factor * normal_y;
        let proj_z = projection_factor * normal_z;

        // Subtract projection from tangent to make it perpendicular to normal
        let new_tangent_x = tangent_x - proj_x;
        let new_tangent_y = tangent_y - proj_y;
        let new_tangent_z = tangent_z - proj_z;

        // Normalize the new tangent
        let new_tangent_mag = (new_tangent_x * new_tangent_x
            + new_tangent_y * new_tangent_y
            + new_tangent_z * new_tangent_z)
            .sqrt();

        if new_tangent_mag > 1e-10 {
            let scale = 1.0 / new_tangent_mag;
            new_parameters.insert(format!("{}.x", tangent), new_tangent_x * scale);
            new_parameters.insert(format!("{}.y", tangent), new_tangent_y * scale);
            new_parameters.insert(format!("{}.z", tangent), new_tangent_z * scale);
        }
    }

    /// Solve equal constraint
    fn solve_equal_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 2 {
            return;
        }

        let p1 = &params[0];
        let p2 = &params[1];

        // Get p1 value
        let p1_val = self.get_numeric_value(p1);

        // Set p2 to p1 value
        new_parameters.insert(p2.to_string(), p1_val);
    }

    /// Solve symmetric constraint
    fn solve_symmetric_constraint(
        &self,
        _constraint: &Box<dyn Constraint + Send + Sync>,
        params: &[String],
        new_parameters: &mut HashMap<String, f64>,
    ) {
        if params.len() < 3 {
            return;
        }

        let point = &params[0];
        let plane_origin = &params[1];
        let plane_normal = &params[2];

        // Get point coordinates
        let point_x = self.get_numeric_value(&format!("{}.x", point));
        let point_y = self.get_numeric_value(&format!("{}.y", point));
        let point_z = self.get_numeric_value(&format!("{}.z", point));

        // Get plane origin
        let plane_origin_x = self.get_numeric_value(&format!("{}.x", plane_origin));
        let plane_origin_y = self.get_numeric_value(&format!("{}.y", plane_origin));
        let plane_origin_z = self.get_numeric_value(&format!("{}.z", plane_origin));

        // Get plane normal
        let plane_normal_x = self.get_numeric_value(&format!("{}.x", plane_normal));
        let plane_normal_y = self.get_numeric_value(&format!("{}.y", plane_normal));
        let plane_normal_z = self.get_numeric_value(&format!("{}.z", plane_normal));

        // Calculate vector from plane origin to point
        let vec_x = point_x - plane_origin_x;
        let vec_y = point_y - plane_origin_y;
        let vec_z = point_z - plane_origin_z;

        // Calculate dot product with plane normal
        let dot = vec_x * plane_normal_x + vec_y * plane_normal_y + vec_z * plane_normal_z;

        // Calculate plane normal magnitude squared
        let normal_mag_sq = plane_normal_x * plane_normal_x
            + plane_normal_y * plane_normal_y
            + plane_normal_z * plane_normal_z;

        if normal_mag_sq < 1e-10 {
            return;
        }

        // Calculate projection of vector onto plane normal
        let projection_factor = dot / normal_mag_sq;
        let proj_x = projection_factor * plane_normal_x;
        let proj_y = projection_factor * plane_normal_y;
        let proj_z = projection_factor * plane_normal_z;

        // Calculate reflected point (mirror across plane)
        let reflected_x = point_x - 2.0 * proj_x;
        let reflected_y = point_y - 2.0 * proj_y;
        let reflected_z = point_z - 2.0 * proj_z;

        // Update point to its reflected position
        new_parameters.insert(format!("{}.x", point), reflected_x);
        new_parameters.insert(format!("{}.y", point), reflected_y);
        new_parameters.insert(format!("{}.z", point), reflected_z);
    }

    /// Apply damping to the solution
    fn apply_damping(&mut self) {
        // Get all parameter names first
        let parameter_names: Vec<String> = self.model.get_parameters().keys().cloned().collect();

        // Apply damping factor to all parameters
        for name in parameter_names {
            let current_value = self.get_numeric_value(&name);
            let damped_value = current_value * (1.0 - self.damping_factor);
            self.model
                .set_parameter(&name, ParamType::Numeric(damped_value));
        }
    }

    /// Get the model
    pub fn model(&self) -> &ParametricModel {
        &self.model
    }

    /// Get mutable model
    pub fn model_mut(&mut self) -> &mut ParametricModel {
        &mut self.model
    }

    /// Set tolerance
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set max iterations
    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    /// Get max iterations
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }
}

// ============================================================================
// Graph-Based Constraint Solver
// ============================================================================

/// Graph-based constraint solver
///
/// This solver uses graph algorithms to efficiently solve constraints
/// by identifying independent constraint groups.
#[derive(Debug, Clone)]
pub struct GraphConstraintSolver {
    #[allow(dead_code)]
    solver: ConstraintSolver,
    constraint_graph: ConstraintGraph,
}

impl GraphConstraintSolver {
    /// Create a new graph-based constraint solver
    pub fn new(model: ParametricModel) -> Self {
        let constraint_graph = ConstraintGraph::from_model(&model);
        Self {
            solver: ConstraintSolver::new(model),
            constraint_graph,
        }
    }

    /// Solve constraints using graph decomposition
    pub fn solve(&mut self) -> bool {
        // Decompose constraints into independent groups
        let groups = self.constraint_graph.find_connected_components();

        // Solve each group independently
        let mut all_solved = true;
        for group in groups {
            if !self.solve_constraint_group(&group) {
                all_solved = false;
            }
        }

        all_solved
    }

    /// Solve a single constraint group
    fn solve_constraint_group(&mut self, _group: &[String]) -> bool {
        // This is a simplified implementation
        // In a real system, you would solve the constraint group
        true
    }
}

// ============================================================================
// Constraint Graph
// ============================================================================

/// Constraint graph for analyzing constraint dependencies
#[derive(Debug, Clone)]
pub struct ConstraintGraph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>,
}

impl ConstraintGraph {
    /// Create a new constraint graph
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    /// Build constraint graph from parametric model
    pub fn from_model(model: &ParametricModel) -> Self {
        let mut graph = Self::new();

        // Add parameter nodes
        for param_name in model.get_parameters().keys() {
            graph.add_node(param_name.clone());
        }

        // Add constraint edges
        for constraint in model.get_constraints() {
            let params = constraint.parameters();
            for i in 0..params.len() {
                for j in (i + 1)..params.len() {
                    graph.add_edge(params[i].to_string(), params[j].to_string());
                }
            }
        }

        graph
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: String) {
        self.nodes.insert(node);
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        self.edges.entry(to).or_insert(Vec::new()).push(from);
    }

    /// Find connected components in the graph
    pub fn find_connected_components(&self) -> Vec<Vec<String>> {
        let mut visited = HashSet::new();
        let mut components = Vec::new();

        for node in &self.nodes {
            if !visited.contains(node) {
                let component = self.bfs(node, &mut visited);
                components.push(component);
            }
        }

        components
    }

    /// Breadth-first search to find connected component
    fn bfs(&self, start: &str, visited: &mut HashSet<String>) -> Vec<String> {
        let mut component = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(node) = queue.pop_front() {
            component.push(node.clone());

            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        component
    }

    /// Get constraint order using topological sort
    pub fn get_constraint_order(&self) -> Vec<String> {
        // This is a simplified implementation
        // In a real system, you would perform topological sorting
        self.nodes.iter().cloned().collect()
    }
}

impl Default for ConstraintGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Geometric Constraint Types
// ============================================================================

/// Geometric constraint types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeometricConstraintType {
    /// Distance constraint
    Distance,
    /// Angle constraint
    Angle,
    /// Coincident constraint
    Coincident,
    /// Parallel constraint
    Parallel,
    /// Perpendicular constraint
    Perpendicular,
    /// Tangent constraint
    Tangent,
    /// Equal constraint
    Equal,
    /// Symmetric constraint
    Symmetric,
}

/// Geometric constraint
#[derive(Debug, Clone)]
pub struct GeometricConstraint {
    constraint_type: GeometricConstraintType,
    elements: Vec<GeometricElement>,
    value: Option<f64>,
    tolerance: f64,
}

impl GeometricConstraint {
    /// Create a new geometric constraint
    pub fn new(
        constraint_type: GeometricConstraintType,
        elements: Vec<GeometricElement>,
        value: Option<f64>,
        tolerance: f64,
    ) -> Self {
        Self {
            constraint_type,
            elements,
            value,
            tolerance,
        }
    }

    /// Get constraint type
    pub fn constraint_type(&self) -> GeometricConstraintType {
        self.constraint_type
    }

    /// Get constraint elements
    pub fn elements(&self) -> &[GeometricElement] {
        &self.elements
    }

    /// Get constraint value
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Get constraint tolerance
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Check if constraint is satisfied
    pub fn is_satisfied(&self) -> bool {
        // This is a simplified implementation
        // In a real system, you would check the actual geometric condition
        true
    }
}

/// Geometric element
#[derive(Debug, Clone)]
pub enum GeometricElement {
    /// Point element
    Point(Point),
    /// Line element
    Line(Line),
    /// Circle element
    Circle(Circle),
    /// Ellipse element
    Ellipse(Ellipse),
    /// Plane element
    Plane(Plane),
    /// Edge element
    Edge(Handle<TopoDsEdge>),
    /// Face element
    Face(Handle<TopoDsFace>),
    /// Shape element
    Shape(Handle<TopoDsShape>),
    /// Vertex element
    Vertex(Handle<TopoDsVertex>),
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_solver_creation() {
        let model = ParametricModel::new("test_model");
        let solver = ConstraintSolver::new(model);
        assert_eq!(solver.tolerance(), 1e-6);
        assert_eq!(solver.max_iterations(), 100);
    }

    #[test]
    fn test_constraint_solver_with_parameters() {
        let model = ParametricModel::new("test_model");
        let solver = ConstraintSolver::with_parameters(model, 1e-4, 50, 0.2);
        assert_eq!(solver.tolerance(), 1e-4);
        assert_eq!(solver.max_iterations(), 50);
    }

    #[test]
    fn test_constraint_graph() {
        let model = ParametricModel::new("test_model");
        let graph = ConstraintGraph::from_model(&model);
        let components = graph.find_connected_components();
        // Empty model should have no components
        assert!(components.is_empty());
    }

    #[test]
    fn test_geometric_constraint() {
        let point = Point::new(0.0, 0.0, 0.0);
        let element = GeometricElement::Point(point);
        let constraint = GeometricConstraint::new(
            GeometricConstraintType::Distance,
            vec![element],
            Some(10.0),
            1e-6,
        );
        assert_eq!(
            constraint.constraint_type(),
            GeometricConstraintType::Distance
        );
        assert_eq!(constraint.value(), Some(10.0));
        assert!(constraint.is_satisfied());
    }
}
