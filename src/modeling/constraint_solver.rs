use std::collections::HashSet;

use crate::geometry::{Point, Vector};


/// Constraint type
enum ConstraintType {
    /// Distance constraint between two points
    Distance(f64),
    /// Angle constraint between three points
    Angle(f64),
    /// Coincident constraint between two points
    Coincident,
    /// Horizontal constraint for a point (y-coordinate fixed)
    Horizontal,
    /// Vertical constraint for a point (x-coordinate fixed)
    Vertical,
    /// Parallel constraint between two vectors
    Parallel,
    /// Perpendicular constraint between two vectors
    Perpendicular,
}

/// Variable type
enum Variable {
    /// Point variable (x, y, z)
    Point(usize), // index in points array
    /// Vector variable (dx, dy, dz)
    Vector(usize), // index in vectors array
    /// Angle variable
    Angle(usize), // index in angles array
}

/// Constraint definition
struct Constraint {
    /// Constraint type
    constraint_type: ConstraintType,
    /// Variables involved in the constraint
    variables: Vec<Variable>,
    /// Weight of the constraint
    weight: f64,
}

/// Constraint solver
pub struct ConstraintSolver {
    /// Points variables
    points: Vec<Point>,
    /// Vectors variables
    vectors: Vec<Vector>,
    /// Angles variables
    angles: Vec<f64>,
    /// Constraints
    constraints: Vec<Constraint>,
    /// Fixed variables (indices)
    fixed_points: HashSet<usize>,
    fixed_vectors: HashSet<usize>,
    fixed_angles: HashSet<usize>,
}

impl ConstraintSolver {
    /// Create a new constraint solver
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            vectors: Vec::new(),
            angles: Vec::new(),
            constraints: Vec::new(),
            fixed_points: HashSet::new(),
            fixed_vectors: HashSet::new(),
            fixed_angles: HashSet::new(),
        }
    }

    /// Add a point variable
    pub fn add_point(&mut self, point: Point) -> usize {
        let index = self.points.len();
        self.points.push(point);
        index
    }

    /// Add a vector variable
    pub fn add_vector(&mut self, vector: Vector) -> usize {
        let index = self.vectors.len();
        self.vectors.push(vector);
        index
    }

    /// Add an angle variable
    pub fn add_angle(&mut self, angle: f64) -> usize {
        let index = self.angles.len();
        self.angles.push(angle);
        index
    }

    /// Fix a point variable
    pub fn fix_point(&mut self, index: usize) {
        self.fixed_points.insert(index);
    }

    /// Fix a vector variable
    pub fn fix_vector(&mut self, index: usize) {
        self.fixed_vectors.insert(index);
    }

    /// Fix an angle variable
    pub fn fix_angle(&mut self, index: usize) {
        self.fixed_angles.insert(index);
    }

    /// Add a distance constraint between two points
    pub fn add_distance_constraint(&mut self, point1: usize, point2: usize, distance: f64, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Distance(distance),
            variables: vec![Variable::Point(point1), Variable::Point(point2)],
            weight,
        });
    }

    /// Add a coincident constraint between two points
    pub fn add_coincident_constraint(&mut self, point1: usize, point2: usize, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Coincident,
            variables: vec![Variable::Point(point1), Variable::Point(point2)],
            weight,
        });
    }

    /// Add a horizontal constraint for a point
    pub fn add_horizontal_constraint(&mut self, point: usize, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Horizontal,
            variables: vec![Variable::Point(point)],
            weight,
        });
    }

    /// Add a vertical constraint for a point
    pub fn add_vertical_constraint(&mut self, point: usize, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Vertical,
            variables: vec![Variable::Point(point)],
            weight,
        });
    }

    /// Add a parallel constraint between two vectors
    pub fn add_parallel_constraint(&mut self, vector1: usize, vector2: usize, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Parallel,
            variables: vec![Variable::Vector(vector1), Variable::Vector(vector2)],
            weight,
        });
    }

    /// Add a perpendicular constraint between two vectors
    pub fn add_perpendicular_constraint(&mut self, vector1: usize, vector2: usize, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Perpendicular,
            variables: vec![Variable::Vector(vector1), Variable::Vector(vector2)],
            weight,
        });
    }

    /// Add an angle constraint between three points
    pub fn add_angle_constraint(&mut self, point1: usize, point2: usize, point3: usize, angle: f64, weight: f64) {
        self.constraints.push(Constraint {
            constraint_type: ConstraintType::Angle(angle),
            variables: vec![Variable::Point(point1), Variable::Point(point2), Variable::Point(point3)],
            weight,
        });
    }

    /// Solve the constraints using Gauss-Newton method
    pub fn solve(&mut self, max_iterations: usize, tolerance: f64) -> bool {
        let mut iteration = 0;
        let mut error = std::f64::MAX;

        while iteration < max_iterations && error > tolerance {
            let (new_error, updated) = self.solve_iteration();
            error = new_error;
            iteration += 1;

            if !updated {
                break;
            }
        }

        error <= tolerance
    }

    /// Perform a single iteration of the Gauss-Newton method
    fn solve_iteration(&mut self) -> (f64, bool) {
        // Calculate the number of variables
        let num_point_vars = self.points.len() * 3;
        let num_vector_vars = self.vectors.len() * 3;
        let num_angle_vars = self.angles.len();
        let total_vars = num_point_vars + num_vector_vars + num_angle_vars;

        // Initialize Jacobian matrix and residual vector
        let mut jacobian = vec![vec![0.0; total_vars]; self.constraints.len() * 2]; // Each constraint contributes 2 equations
        let mut residual = vec![0.0; self.constraints.len() * 2];

        // Calculate Jacobian and residual
        let mut constraint_index = 0;
        for constraint in &self.constraints {
            match &constraint.constraint_type {
                ConstraintType::Distance(distance) => {
                    if let [Variable::Point(p1), Variable::Point(p2)] = &constraint.variables[..] {
                        let p1_idx = *p1;
                        let p2_idx = *p2;
                        let p1 = self.points[p1_idx];
                        let p2 = self.points[p2_idx];

                        let dx = p2.x - p1.x;
                        let dy = p2.y - p1.y;
                        let dz = p2.z - p1.z;
                        let current_distance = (dx * dx + dy * dy + dz * dz).sqrt();
                        let error = current_distance - distance;

                        residual[constraint_index * 2] = error * constraint.weight;

                        if !self.fixed_points.contains(&p1_idx) {
                            let j = constraint_index * 2;
                            jacobian[j][p1_idx * 3] = -dx / current_distance * constraint.weight;
                            jacobian[j][p1_idx * 3 + 1] = -dy / current_distance * constraint.weight;
                            jacobian[j][p1_idx * 3 + 2] = -dz / current_distance * constraint.weight;
                        }

                        if !self.fixed_points.contains(&p2_idx) {
                            let j = constraint_index * 2;
                            jacobian[j][p2_idx * 3] = dx / current_distance * constraint.weight;
                            jacobian[j][p2_idx * 3 + 1] = dy / current_distance * constraint.weight;
                            jacobian[j][p2_idx * 3 + 2] = dz / current_distance * constraint.weight;
                        }
                    }
                }
                ConstraintType::Coincident => {
                    if let [Variable::Point(p1), Variable::Point(p2)] = &constraint.variables[..] {
                        let p1_idx = *p1;
                        let p2_idx = *p2;
                        let p1 = self.points[p1_idx];
                        let p2 = self.points[p2_idx];

                        residual[constraint_index * 2] = (p1.x - p2.x) * constraint.weight;
                        residual[constraint_index * 2 + 1] = (p1.y - p2.y) * constraint.weight;

                        if !self.fixed_points.contains(&p1_idx) {
                            jacobian[constraint_index * 2][p1_idx * 3] = constraint.weight;
                            jacobian[constraint_index * 2 + 1][p1_idx * 3 + 1] = constraint.weight;
                        }

                        if !self.fixed_points.contains(&p2_idx) {
                            jacobian[constraint_index * 2][p2_idx * 3] = -constraint.weight;
                            jacobian[constraint_index * 2 + 1][p2_idx * 3 + 1] = -constraint.weight;
                        }
                    }
                }
                ConstraintType::Horizontal => {
                    if let [Variable::Point(p)] = &constraint.variables[..] {
                        let p_idx = *p;
                        let p = self.points[p_idx];

                        residual[constraint_index * 2] = p.y * constraint.weight;

                        if !self.fixed_points.contains(&p_idx) {
                            jacobian[constraint_index * 2][p_idx * 3 + 1] = constraint.weight;
                        }
                    }
                }
                ConstraintType::Vertical => {
                    if let [Variable::Point(p)] = &constraint.variables[..] {
                        let p_idx = *p;
                        let p = self.points[p_idx];

                        residual[constraint_index * 2] = p.x * constraint.weight;

                        if !self.fixed_points.contains(&p_idx) {
                            jacobian[constraint_index * 2][p_idx * 3] = constraint.weight;
                        }
                    }
                }
                _ => {
                    // Implement other constraint types
                }
            }
            constraint_index += 1;
        }

        // Calculate the normal equations: J^T * J * delta = J^T * residual
        let mut jt_j = vec![vec![0.0; total_vars]; total_vars];
        let mut jt_r = vec![0.0; total_vars];

        for i in 0..jacobian.len() {
            for j in 0..total_vars {
                if jacobian[i][j] != 0.0 {
                    for k in 0..total_vars {
                        jt_j[j][k] += jacobian[i][j] * jacobian[i][k];
                    }
                    jt_r[j] += jacobian[i][j] * residual[i];
                }
            }
        }

        // Solve the linear system using Gauss-Seidel method
        let delta = self.solve_linear_system(&jt_j, &jt_r, total_vars);

        // Update variables
        let mut updated = false;
        for i in 0..self.points.len() {
            if !self.fixed_points.contains(&i) {
                let dx = delta[i * 3];
                let dy = delta[i * 3 + 1];
                let dz = delta[i * 3 + 2];
                
                if dx.abs() > 1e-10 || dy.abs() > 1e-10 || dz.abs() > 1e-10 {
                    self.points[i] = Point::new(
                        self.points[i].x - dx,
                        self.points[i].y - dy,
                        self.points[i].z - dz,
                    );
                    updated = true;
                }
            }
        }

        // Calculate error
        let error = residual.iter().map(|&x| x * x).sum::<f64>().sqrt();
        (error, updated)
    }

    /// Solve a linear system using Gauss-Seidel method
    fn solve_linear_system(&self, a: &Vec<Vec<f64>>, b: &Vec<f64>, size: usize) -> Vec<f64> {
        let mut x = vec![0.0; size];
        let mut x_new = vec![0.0; size];
        let max_iterations = 100;
        let tolerance = 1e-10;

        for _ in 0..max_iterations {
            for i in 0..size {
                let mut sum = b[i];
                for j in 0..size {
                    if i != j {
                        sum -= a[i][j] * x[j];
                    }
                }
                x_new[i] = sum / a[i][i];
            }

            let mut error = 0.0;
            for i in 0..size {
                error += (x_new[i] - x[i]).abs();
            }

            x = x_new.clone();
            if error < tolerance {
                break;
            }
        }

        x
    }

    /// Get the points
    pub fn points(&self) -> &Vec<Point> {
        &self.points
    }

    /// Get the vectors
    pub fn vectors(&self) -> &Vec<Vector> {
        &self.vectors
    }

    /// Get the angles
    pub fn angles(&self) -> &Vec<f64> {
        &self.angles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_constraint() {
        let mut solver = ConstraintSolver::new();

        // Add points
        let p0 = solver.add_point(Point::new(0.0, 0.0, 0.0));
        let p1 = solver.add_point(Point::new(1.0, 0.0, 0.0));

        // Fix p0
        solver.fix_point(p0);

        // Add distance constraint
        solver.add_distance_constraint(p0, p1, 2.0, 1.0);

        // Solve
        let success = solver.solve(100, 1e-10);
        assert!(success);

        // Check the result
        let points = solver.points();
        let distance = ((points[p1].x - points[p0].x).powi(2) + 
                       (points[p1].y - points[p0].y).powi(2) + 
                       (points[p1].z - points[p0].z).powi(2)).sqrt();
        assert!((distance - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_coincident_constraint() {
        let mut solver = ConstraintSolver::new();

        // Add points
        let p0 = solver.add_point(Point::new(0.0, 0.0, 0.0));
        let p1 = solver.add_point(Point::new(1.0, 1.0, 0.0));

        // Fix p0
        solver.fix_point(p0);

        // Add coincident constraint
        solver.add_coincident_constraint(p0, p1, 1.0);

        // Solve
        let success = solver.solve(100, 1e-10);
        assert!(success);

        // Check the result
        let points = solver.points();
        assert!((points[p1].x - points[p0].x).abs() < 1e-10);
        assert!((points[p1].y - points[p0].y).abs() < 1e-10);
    }

    #[test]
    fn test_horizontal_constraint() {
        let mut solver = ConstraintSolver::new();

        // Add point
        let p0 = solver.add_point(Point::new(1.0, 1.0, 0.0));

        // Add horizontal constraint
        solver.add_horizontal_constraint(p0, 1.0);

        // Solve
        let success = solver.solve(100, 1e-10);
        assert!(success);

        // Check the result
        let points = solver.points();
        assert!(points[p0].y.abs() < 1e-10);
    }

    #[test]
    fn test_vertical_constraint() {
        let mut solver = ConstraintSolver::new();

        // Add point
        let p0 = solver.add_point(Point::new(1.0, 1.0, 0.0));

        // Add vertical constraint
        solver.add_vertical_constraint(p0, 1.0);

        // Solve
        let success = solver.solve(100, 1e-10);
        assert!(success);

        // Check the result
        let points = solver.points();
        assert!(points[p0].x.abs() < 1e-10);
    }
}
