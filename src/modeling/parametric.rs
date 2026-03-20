//! Parametric modeling module
//!
//! This module provides a comprehensive parametric modeling system that allows
//! users to define parameters, create constraints, and update models dynamically.

use crate::foundation::handle::Handle;
use crate::geometry::{Direction, Plane, Point, Vector};
use crate::modeling::BrepBuilder;
use crate::topology::{TopoDsEdge, TopoDsFace, TopoDsShape, TopoDsSolid, TopoDsVertex, TopoDsWire};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

// ============================================================================
// Parameter Definitions
// ============================================================================

/// Parameter type enum
#[derive(Debug, Clone, PartialEq)]
pub enum ParamType {
    /// Numeric parameter (e.g., length, angle, radius)
    Numeric(f64),
    /// Boolean parameter (e.g., enable/disable feature)
    Boolean(bool),
    /// String parameter (e.g., feature name, material)
    String(String),
    /// Point parameter (e.g., position, origin)
    Point(Point),
    /// Vector parameter (e.g., direction, normal)
    Vector(Vector),
    /// Plane parameter (e.g., reference plane, cutting plane)
    Plane(Plane),
    /// Direction parameter (e.g., axis direction)
    Direction(Direction),
    /// Integer parameter (e.g., count, index)
    Integer(i32),
    /// Enum parameter (e.g., material type, operation type)
    Enum(String),
}

/// Parameter trait
pub trait Parameter {
    /// Get parameter name
    fn name(&self) -> &str;
    /// Get parameter type
    fn param_type(&self) -> ParamType;
    /// Set parameter value
    fn set_value(&mut self, value: ParamType);
    /// Get parameter value
    fn value(&self) -> ParamType;
    /// Get parameter description
    fn description(&self) -> &str;
}

/// Numeric parameter implementation
#[derive(Debug, Clone)]
pub struct NumericParam {
    name: String,
    value: f64,
    description: String,
    min_value: Option<f64>,
    max_value: Option<f64>,
    step: Option<f64>,
}

impl NumericParam {
    /// Create a new numeric parameter
    pub fn new(name: &str, value: f64, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
            min_value: None,
            max_value: None,
            step: None,
        }
    }

    /// Set minimum value constraint
    pub fn with_min(self, min: f64) -> Self {
        Self {
            min_value: Some(min),
            ..self
        }
    }

    /// Set maximum value constraint
    pub fn with_max(self, max: f64) -> Self {
        Self {
            max_value: Some(max),
            ..self
        }
    }

    /// Set step value
    pub fn with_step(self, step: f64) -> Self {
        Self {
            step: Some(step),
            ..self
        }
    }

    /// Get numeric value
    pub fn get_value(&self) -> f64 {
        self.value
    }

    /// Set numeric value with validation
    pub fn set_value(&mut self, value: f64) -> bool {
        // Validate value against constraints
        if let Some(min) = self.min_value {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max_value {
            if value > max {
                return false;
            }
        }
        self.value = value;
        true
    }
}

impl Parameter for NumericParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Numeric(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Numeric(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Numeric(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Boolean parameter implementation
#[derive(Debug, Clone)]
pub struct BooleanParam {
    name: String,
    value: bool,
    description: String,
}

impl BooleanParam {
    /// Create a new boolean parameter
    pub fn new(name: &str, value: bool, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }

    /// Get boolean value
    pub fn get_value(&self) -> bool {
        self.value
    }

    /// Set boolean value
    pub fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}

impl Parameter for BooleanParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Boolean(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Boolean(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Boolean(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// String parameter implementation
#[derive(Debug, Clone)]
pub struct StringParam {
    name: String,
    value: String,
    description: String,
}

impl StringParam {
    /// Create a new string parameter
    pub fn new(name: &str, value: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            description: description.to_string(),
        }
    }

    /// Get string value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set string value
    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_string();
    }
}

impl Parameter for StringParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::String(self.value.clone())
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::String(v) = value {
            self.set_value(&v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::String(self.value.clone())
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Point parameter implementation
#[derive(Debug, Clone)]
pub struct PointParam {
    name: String,
    value: Point,
    description: String,
}

impl PointParam {
    /// Create a new point parameter
    pub fn new(name: &str, value: Point, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }

    /// Get point value
    pub fn get_value(&self) -> &Point {
        &self.value
    }

    /// Set point value
    pub fn set_value(&mut self, value: Point) {
        self.value = value;
    }
}

impl Parameter for PointParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Point(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Point(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Point(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Vector parameter implementation
#[derive(Debug, Clone)]
pub struct VectorParam {
    name: String,
    value: Vector,
    description: String,
}

impl VectorParam {
    /// Create a new vector parameter
    pub fn new(name: &str, value: Vector, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }

    /// Get vector value
    pub fn get_value(&self) -> &Vector {
        &self.value
    }

    /// Set vector value
    pub fn set_value(&mut self, value: Vector) {
        self.value = value;
    }
}

impl Parameter for VectorParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Vector(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Vector(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Vector(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Plane parameter implementation
#[derive(Debug, Clone)]
pub struct PlaneParam {
    name: String,
    value: Plane,
    description: String,
}

impl PlaneParam {
    /// Create a new plane parameter
    pub fn new(name: &str, value: Plane, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }

    /// Get plane value
    pub fn get_value(&self) -> &Plane {
        &self.value
    }

    /// Set plane value
    pub fn set_value(&mut self, value: Plane) {
        self.value = value;
    }
}

impl Parameter for PlaneParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Plane(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Plane(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Plane(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Direction parameter implementation
#[derive(Debug, Clone)]
pub struct DirectionParam {
    name: String,
    value: Direction,
    description: String,
}

impl DirectionParam {
    /// Create a new direction parameter
    pub fn new(name: &str, value: Direction, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
        }
    }

    /// Get direction value
    pub fn get_value(&self) -> &Direction {
        &self.value
    }

    /// Set direction value
    pub fn set_value(&mut self, value: Direction) {
        self.value = value;
    }
}

impl Parameter for DirectionParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Direction(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Direction(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Direction(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Integer parameter implementation
#[derive(Debug, Clone)]
pub struct IntegerParam {
    name: String,
    value: i32,
    description: String,
    min_value: Option<i32>,
    max_value: Option<i32>,
}

impl IntegerParam {
    /// Create a new integer parameter
    pub fn new(name: &str, value: i32, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value,
            description: description.to_string(),
            min_value: None,
            max_value: None,
        }
    }

    /// Set minimum value constraint
    pub fn with_min(self, min: i32) -> Self {
        Self {
            min_value: Some(min),
            ..self
        }
    }

    /// Set maximum value constraint
    pub fn with_max(self, max: i32) -> Self {
        Self {
            max_value: Some(max),
            ..self
        }
    }

    /// Get integer value
    pub fn get_value(&self) -> i32 {
        self.value
    }

    /// Set integer value with validation
    pub fn set_value(&mut self, value: i32) -> bool {
        // Validate value against constraints
        if let Some(min) = self.min_value {
            if value < min {
                return false;
            }
        }
        if let Some(max) = self.max_value {
            if value > max {
                return false;
            }
        }
        self.value = value;
        true
    }
}

impl Parameter for IntegerParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Integer(self.value)
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Integer(v) = value {
            self.set_value(v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Integer(self.value)
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Enum parameter implementation
#[derive(Debug, Clone)]
pub struct EnumParam {
    name: String,
    value: String,
    description: String,
    options: Vec<String>,
}

impl EnumParam {
    /// Create a new enum parameter
    pub fn new(name: &str, value: &str, description: &str, options: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            description: description.to_string(),
            options,
        }
    }

    /// Get enum value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set enum value with validation
    pub fn set_value(&mut self, value: &str) -> bool {
        if self.options.contains(&value.to_string()) {
            self.value = value.to_string();
            true
        } else {
            false
        }
    }

    /// Get available options
    pub fn get_options(&self) -> &Vec<String> {
        &self.options
    }
}

impl Parameter for EnumParam {
    fn name(&self) -> &str {
        &self.name
    }

    fn param_type(&self) -> ParamType {
        ParamType::Enum(self.value.clone())
    }

    fn set_value(&mut self, value: ParamType) {
        if let ParamType::Enum(v) = value {
            self.set_value(&v);
        }
    }

    fn value(&self) -> ParamType {
        ParamType::Enum(self.value.clone())
    }

    fn description(&self) -> &str {
        &self.description
    }
}

// ============================================================================
// Constraints
// ============================================================================

/// Constraint type enum
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    /// Distance constraint between two points
    Distance,
    /// Angle constraint between two vectors
    Angle,
    /// Coincident constraint between two points
    Coincident,
    /// Parallel constraint between two lines
    Parallel,
    /// Perpendicular constraint between two lines
    Perpendicular,
    /// Tangent constraint between two curves
    Tangent,
    /// Equal constraint between two geometric elements
    Equal,
    /// Symmetric constraint between two geometric elements
    Symmetric,
}

/// Constraint trait
pub trait Constraint {
    /// Get constraint type
    fn constraint_type(&self) -> ConstraintType;
    /// Get constraint name
    fn name(&self) -> &str;
    /// Check if constraint is satisfied
    fn is_satisfied(&self, model: &ParametricModel) -> bool;
    /// Solve constraint
    fn solve(&mut self, model: &mut ParametricModel) -> bool;
    /// Get constraint parameters
    fn parameters(&self) -> Vec<&str>;
    /// Get constraint target value (for distance, angle, etc.)
    fn value(&self) -> Option<f64> {
        None
    }
}

/// Distance constraint between two points
#[derive(Debug, Clone)]
pub struct DistanceConstraint {
    name: String,
    point1: String, // Parameter name of first point
    point2: String, // Parameter name of second point
    distance: f64,  // Target distance
    tolerance: f64, // Tolerance for satisfaction
}

impl DistanceConstraint {
    /// Create a new distance constraint
    pub fn new(name: &str, point1: &str, point2: &str, distance: f64, tolerance: f64) -> Self {
        Self {
            name: name.to_string(),
            point1: point1.to_string(),
            point2: point2.to_string(),
            distance,
            tolerance,
        }
    }
}

impl Constraint for DistanceConstraint {
    fn constraint_type(&self) -> ConstraintType {
        ConstraintType::Distance
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_satisfied(&self, model: &ParametricModel) -> bool {
        // Get the two points from the model
        let point1 = model.get_parameter(&self.point1);
        let point2 = model.get_parameter(&self.point2);

        if let (Some(ParamType::Point(p1)), Some(ParamType::Point(p2))) = (point1, point2) {
            let current_distance = p1.distance(&p2);
            (current_distance - self.distance).abs() < self.tolerance
        } else {
            false
        }
    }

    fn solve(&mut self, _model: &mut ParametricModel) -> bool {
        // This is a simplified implementation
        // In a real system, you would use a constraint solver
        true
    }

    fn parameters(&self) -> Vec<&str> {
        vec![&self.point1, &self.point2]
    }

    fn value(&self) -> Option<f64> {
        Some(self.distance)
    }
}

/// Angle constraint between two vectors
#[derive(Debug, Clone)]
pub struct AngleConstraint {
    name: String,
    vector1: String, // Parameter name of first vector
    vector2: String, // Parameter name of second vector
    angle: f64,      // Target angle in radians
    tolerance: f64,  // Tolerance for satisfaction
}

impl AngleConstraint {
    /// Create a new angle constraint
    pub fn new(name: &str, vector1: &str, vector2: &str, angle: f64, tolerance: f64) -> Self {
        Self {
            name: name.to_string(),
            vector1: vector1.to_string(),
            vector2: vector2.to_string(),
            angle,
            tolerance,
        }
    }
}

impl Constraint for AngleConstraint {
    fn constraint_type(&self) -> ConstraintType {
        ConstraintType::Angle
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_satisfied(&self, model: &ParametricModel) -> bool {
        // Get the two vectors from the model
        let vector1 = model.get_parameter(&self.vector1);
        let vector2 = model.get_parameter(&self.vector2);

        if let (Some(ParamType::Vector(v1)), Some(ParamType::Vector(v2))) = (vector1, vector2) {
            let current_angle = v1.angle(&v2);
            (current_angle - self.angle).abs() < self.tolerance
        } else {
            false
        }
    }

    fn solve(&mut self, _model: &mut ParametricModel) -> bool {
        // This is a simplified implementation
        // In a real system, you would use a constraint solver
        true
    }

    fn parameters(&self) -> Vec<&str> {
        vec![&self.vector1, &self.vector2]
    }

    fn value(&self) -> Option<f64> {
        Some(self.angle)
    }
}

/// Coincident constraint between two points
#[derive(Debug, Clone)]
pub struct CoincidentConstraint {
    name: String,
    point1: String, // Parameter name of first point
    point2: String, // Parameter name of second point
    tolerance: f64, // Tolerance for satisfaction
}

impl CoincidentConstraint {
    /// Create a new coincident constraint
    pub fn new(name: &str, point1: &str, point2: &str, tolerance: f64) -> Self {
        Self {
            name: name.to_string(),
            point1: point1.to_string(),
            point2: point2.to_string(),
            tolerance,
        }
    }
}

impl Constraint for CoincidentConstraint {
    fn constraint_type(&self) -> ConstraintType {
        ConstraintType::Coincident
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_satisfied(&self, model: &ParametricModel) -> bool {
        // Get the two points from the model
        let point1 = model.get_parameter(&self.point1);
        let point2 = model.get_parameter(&self.point2);

        if let (Some(ParamType::Point(p1)), Some(ParamType::Point(p2))) = (point1, point2) {
            p1.distance(&p2) < self.tolerance
        } else {
            false
        }
    }

    fn solve(&mut self, _model: &mut ParametricModel) -> bool {
        // This is a simplified implementation
        // In a real system, you would use a constraint solver
        true
    }

    fn parameters(&self) -> Vec<&str> {
        vec![&self.point1, &self.point2]
    }
}

// ============================================================================
// Parametric Model
// ============================================================================

/// Parametric model struct
///
/// This struct represents a parametric model with parameters, constraints,
/// and the ability to update the model when parameters change.
pub struct ParametricModel {
    name: String,
    parameters: HashMap<String, Box<dyn Parameter + Send + Sync>>,
    constraints: Vec<Box<dyn Constraint + Send + Sync>>,
    shapes: HashMap<String, Handle<TopoDsShape>>,
    update_callback: Option<Box<dyn Fn(&mut ParametricModel) + Send + Sync>>,
    dirty: bool,
    dependencies: HashMap<String, Vec<String>>, // Parameter dependencies
}

impl std::fmt::Debug for ParametricModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParametricModel")
            .field("name", &self.name)
            .field("parameters", &self.parameters.keys())
            .field("constraints", &self.constraints.len())
            .field("shapes", &self.shapes.keys())
            .field("dirty", &self.dirty)
            .field("dependencies", &self.dependencies)
            .finish()
    }
}

impl Clone for ParametricModel {
    fn clone(&self) -> Self {
        // Note: We can't clone the update_callback since it's a boxed trait object
        // For now, we'll create a new model without the callback
        Self {
            name: self.name.clone(),
            parameters: HashMap::new(), // We can't clone trait objects
            constraints: Vec::new(),    // We can't clone trait objects
            shapes: self.shapes.clone(),
            update_callback: None, // Can't clone closures
            dirty: self.dirty,
            dependencies: self.dependencies.clone(),
        }
    }
}

impl ParametricModel {
    /// Create a new parametric model
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parameters: HashMap::new(),
            constraints: Vec::new(),
            shapes: HashMap::new(),
            update_callback: None,
            dirty: false,
            dependencies: HashMap::new(),
        }
    }

    /// Set update callback
    pub fn set_update_callback<F>(&mut self, callback: F)
    where
        F: Fn(&mut ParametricModel) + Send + Sync + 'static,
    {
        self.update_callback = Some(Box::new(callback));
    }

    /// Add a parameter to the model
    pub fn add_parameter<P: Parameter + Send + Sync + 'static>(&mut self, parameter: P) {
        self.parameters
            .insert(parameter.name().to_string(), Box::new(parameter));
        self.dirty = true;
    }

    /// Add a constraint to the model
    pub fn add_constraint<C: Constraint + Send + Sync + 'static>(&mut self, constraint: C) {
        self.constraints.push(Box::new(constraint));
        self.dirty = true;
    }

    /// Add a shape to the model
    pub fn add_shape(&mut self, name: &str, shape: Handle<TopoDsShape>) {
        self.shapes.insert(name.to_string(), shape);
    }

    /// Get a parameter by name
    pub fn get_parameter(&self, name: &str) -> Option<ParamType> {
        self.parameters.get(name).map(|p| p.value())
    }

    /// Get a mutable parameter by name
    pub fn get_parameter_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut Box<dyn Parameter + Send + Sync>> {
        self.parameters.get_mut(name)
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, name: &str, value: ParamType) -> bool {
        if let Some(parameter) = self.parameters.get_mut(name) {
            parameter.set_value(value);
            self.dirty = true;
            true
        } else {
            false
        }
    }

    /// Get a shape by name
    pub fn get_shape(&self, name: &str) -> Option<&Handle<TopoDsShape>> {
        self.shapes.get(name)
    }

    /// Get all parameters
    pub fn get_parameters(&self) -> &HashMap<String, Box<dyn Parameter + Send + Sync>> {
        &self.parameters
    }

    /// Get all constraints
    pub fn get_constraints(&self) -> &Vec<Box<dyn Constraint + Send + Sync>> {
        &self.constraints
    }

    /// Get all shapes
    pub fn get_shapes(&self) -> &HashMap<String, Handle<TopoDsShape>> {
        &self.shapes
    }

    /// Check if all constraints are satisfied
    pub fn check_constraints(&self) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied(self))
    }

    /// Solve all constraints
    pub fn solve_constraints(&mut self) -> bool {
        let all_solved = true;
        // Note: We can't iterate directly over constraints due to mutable borrow issues
        // In a real system, you would need a more sophisticated approach
        all_solved
    }

    /// Update the model
    pub fn update(&mut self) {
        if self.dirty {
            // Solve constraints
            self.solve_constraints();

            // Call update callback if set
            if let Some(callback) = self.update_callback.take() {
                callback(self);
                self.update_callback = Some(callback);
            }

            self.dirty = false;
        }
    }

    /// Mark the model as dirty
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Get model name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if model is dirty
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Add a dependency between parameters
    ///
    /// # Parameters
    /// - `dependent`: The parameter that depends on another
    /// - `dependency`: The parameter that is depended upon
    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) {
        self.dependencies
            .entry(dependent.to_string())
            .or_insert(Vec::new())
            .push(dependency.to_string());
    }

    /// Get dependencies for a parameter
    ///
    /// # Parameters
    /// - `param_name`: The parameter name
    ///
    /// # Returns
    /// A vector of parameter names that the given parameter depends on
    pub fn get_dependencies(&self, param_name: &str) -> Option<&Vec<String>> {
        self.dependencies.get(param_name)
    }

    /// Get all dependencies
    pub fn get_all_dependencies(&self) -> &HashMap<String, Vec<String>> {
        &self.dependencies
    }

    /// Check if a parameter has dependencies
    pub fn has_dependencies(&self, param_name: &str) -> bool {
        self.dependencies.contains_key(param_name)
    }

    /// Update parameter and all dependent parameters
    pub fn update_parameter(&mut self, name: &str, value: ParamType) -> bool {
        if self.set_parameter(name, value) {
            // Update dependent parameters
            self.update_dependents(name);
            true
        } else {
            false
        }
    }

    /// Update all parameters that depend on the given parameter
    fn update_dependents(&mut self, param_name: &str) {
        // Find all parameters that depend on this parameter
        let mut dependents = Vec::new();
        for (dependent, dependencies) in &self.dependencies {
            if dependencies.contains(&param_name.to_string()) {
                dependents.push(dependent.clone());
            }
        }

        // Update each dependent
        for _dependent in dependents {
            // In a real system, you would have a way to update the dependent parameter
            // based on the changed parameter
            self.mark_dirty();
        }
    }
}

// ============================================================================
// Parametric Shape Builders
// ============================================================================

/// Parametric box builder
pub struct ParametricBoxBuilder {
    model: ParametricModel,
}

impl ParametricBoxBuilder {
    /// Create a new parametric box builder
    pub fn new(name: &str) -> Self {
        let mut model = ParametricModel::new(name);

        // Add parameters
        model.add_parameter(NumericParam::new("width", 10.0, "Box width"));
        model.add_parameter(NumericParam::new("height", 10.0, "Box height"));
        model.add_parameter(NumericParam::new("depth", 10.0, "Box depth"));
        model.add_parameter(PointParam::new("origin", Point::origin(), "Box origin"));

        // Set update callback
        model.set_update_callback(|model| {
            // Get parameters
            let width = model
                .get_parameter("width")
                .unwrap_or(ParamType::Numeric(10.0));
            let height = model
                .get_parameter("height")
                .unwrap_or(ParamType::Numeric(10.0));
            let depth = model
                .get_parameter("depth")
                .unwrap_or(ParamType::Numeric(10.0));
            let origin = model
                .get_parameter("origin")
                .unwrap_or(ParamType::Point(Point::origin()));

            // Extract values
            let width_val = if let ParamType::Numeric(w) = width {
                w
            } else {
                10.0
            };
            let height_val = if let ParamType::Numeric(h) = height {
                h
            } else {
                10.0
            };
            let depth_val = if let ParamType::Numeric(d) = depth {
                d
            } else {
                10.0
            };
            let origin_val = if let ParamType::Point(o) = origin {
                o
            } else {
                Point::origin()
            };

            // Create box shape
            let builder = BrepBuilder::new();
            let box_solid = builder.make_box(width_val, height_val, depth_val, origin_val);
            let box_handle = Handle::new(Arc::new(box_solid.shape().clone()));

            // Update shape in model
            model.add_shape("box", box_handle);
        });

        Self { model }
    }

    /// Set box width
    pub fn width(mut self, width: f64) -> Self {
        self.model.set_parameter("width", ParamType::Numeric(width));
        self
    }

    /// Set box height
    pub fn height(mut self, height: f64) -> Self {
        self.model
            .set_parameter("height", ParamType::Numeric(height));
        self
    }

    /// Set box depth
    pub fn depth(mut self, depth: f64) -> Self {
        self.model.set_parameter("depth", ParamType::Numeric(depth));
        self
    }

    /// Set box origin
    pub fn origin(mut self, origin: Point) -> Self {
        self.model.set_parameter("origin", ParamType::Point(origin));
        self
    }

    /// Build the parametric box model
    pub fn build(self) -> ParametricModel {
        let mut model = self.model;
        model.update();
        model
    }
}

/// Parametric cylinder builder
pub struct ParametricCylinderBuilder {
    model: ParametricModel,
}

impl ParametricCylinderBuilder {
    /// Create a new parametric cylinder builder
    pub fn new(name: &str) -> Self {
        let mut model = ParametricModel::new(name);

        // Add parameters
        model.add_parameter(NumericParam::new("radius", 5.0, "Cylinder radius"));
        model.add_parameter(NumericParam::new("height", 20.0, "Cylinder height"));
        model.add_parameter(PointParam::new(
            "origin",
            Point::origin(),
            "Cylinder origin",
        ));
        model.add_parameter(VectorParam::new(
            "axis",
            Vector::new(0.0, 0.0, 1.0),
            "Cylinder axis",
        ));

        // Set update callback
        model.set_update_callback(|model| {
            // Get parameters
            let radius = model
                .get_parameter("radius")
                .unwrap_or(ParamType::Numeric(5.0));
            let height = model
                .get_parameter("height")
                .unwrap_or(ParamType::Numeric(20.0));
            let origin = model
                .get_parameter("origin")
                .unwrap_or(ParamType::Point(Point::origin()));
            let axis = model
                .get_parameter("axis")
                .unwrap_or(ParamType::Vector(Vector::new(0.0, 0.0, 1.0)));

            // Extract values
            let radius_val = if let ParamType::Numeric(r) = radius {
                r
            } else {
                5.0
            };
            let height_val = if let ParamType::Numeric(h) = height {
                h
            } else {
                20.0
            };
            let origin_val = if let ParamType::Point(o) = origin {
                o
            } else {
                Point::origin()
            };
            let axis_val = if let ParamType::Vector(a) = axis {
                a
            } else {
                Vector::new(0.0, 0.0, 1.0)
            };

            // Create cylinder shape
            let builder = BrepBuilder::new();
            let cylinder_solid = builder.make_cylinder(
                radius_val,
                height_val,
                origin_val,
                crate::geometry::Direction::new(axis_val.x, axis_val.y, axis_val.z),
            );
            let cylinder_handle = Handle::new(Arc::new(cylinder_solid.shape().clone()));

            // Update shape in model
            model.add_shape("cylinder", cylinder_handle);
        });

        Self { model }
    }

    /// Set cylinder radius
    pub fn radius(mut self, radius: f64) -> Self {
        self.model
            .set_parameter("radius", ParamType::Numeric(radius));
        self
    }

    /// Set cylinder height
    pub fn height(mut self, height: f64) -> Self {
        self.model
            .set_parameter("height", ParamType::Numeric(height));
        self
    }

    /// Set cylinder origin
    pub fn origin(mut self, origin: Point) -> Self {
        self.model.set_parameter("origin", ParamType::Point(origin));
        self
    }

    /// Set cylinder axis
    pub fn axis(mut self, axis: Vector) -> Self {
        self.model.set_parameter("axis", ParamType::Vector(axis));
        self
    }

    /// Build the parametric cylinder model
    pub fn build(self) -> ParametricModel {
        let mut model = self.model;
        model.update();
        model
    }
}

/// Parametric sphere builder
pub struct ParametricSphereBuilder {
    model: ParametricModel,
}

impl ParametricSphereBuilder {
    /// Create a new parametric sphere builder
    pub fn new(name: &str) -> Self {
        let mut model = ParametricModel::new(name);

        // Add parameters
        model.add_parameter(NumericParam::new("radius", 5.0, "Sphere radius"));
        model.add_parameter(PointParam::new("center", Point::origin(), "Sphere center"));

        // Set update callback
        model.set_update_callback(|model| {
            // Get parameters
            let radius = model
                .get_parameter("radius")
                .unwrap_or(ParamType::Numeric(5.0));
            let center = model
                .get_parameter("center")
                .unwrap_or(ParamType::Point(Point::origin()));

            // Extract values
            let radius_val = if let ParamType::Numeric(r) = radius {
                r
            } else {
                5.0
            };
            let center_val = if let ParamType::Point(c) = center {
                c
            } else {
                Point::origin()
            };

            // Create sphere shape
            let builder = BrepBuilder::new();
            let sphere_solid = builder.make_sphere(radius_val, center_val);
            let sphere_handle = Handle::new(Arc::new(sphere_solid.shape().clone()));

            // Update shape in model
            model.add_shape("sphere", sphere_handle);
        });

        Self { model }
    }

    /// Set sphere radius
    pub fn radius(mut self, radius: f64) -> Self {
        self.model
            .set_parameter("radius", ParamType::Numeric(radius));
        self
    }

    /// Set sphere center
    pub fn center(mut self, center: Point) -> Self {
        self.model.set_parameter("center", ParamType::Point(center));
        self
    }

    /// Build the parametric sphere model
    pub fn build(self) -> ParametricModel {
        let mut model = self.model;
        model.update();
        model
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_parametric_box() {
        // Create a parametric box
        let mut model = ParametricBoxBuilder::new("test_box")
            .width(20.0)
            .height(10.0)
            .depth(15.0)
            .origin(Point::new(0.0, 0.0, 0.0))
            .build();

        // Check parameters
        assert_eq!(model.get_parameter("width"), Some(ParamType::Numeric(20.0)));
        assert_eq!(
            model.get_parameter("height"),
            Some(ParamType::Numeric(10.0))
        );
        assert_eq!(model.get_parameter("depth"), Some(ParamType::Numeric(15.0)));

        // Update parameter and check
        model.set_parameter("width", ParamType::Numeric(30.0));
        model.update();
        assert_eq!(model.get_parameter("width"), Some(ParamType::Numeric(30.0)));
    }

    #[test]
    fn test_parametric_cylinder() {
        // Create a parametric cylinder
        let mut model = ParametricCylinderBuilder::new("test_cylinder")
            .radius(8.0)
            .height(25.0)
            .origin(Point::new(0.0, 0.0, 0.0))
            .axis(Vector::new(0.0, 1.0, 0.0))
            .build();

        // Check parameters
        assert_eq!(model.get_parameter("radius"), Some(ParamType::Numeric(8.0)));
        assert_eq!(
            model.get_parameter("height"),
            Some(ParamType::Numeric(25.0))
        );

        // Update parameter and check
        model.set_parameter("radius", ParamType::Numeric(10.0));
        model.update();
        assert_eq!(
            model.get_parameter("radius"),
            Some(ParamType::Numeric(10.0))
        );
    }

    #[test]
    fn test_parametric_sphere() {
        // Create a parametric sphere
        let mut model = ParametricSphereBuilder::new("test_sphere")
            .radius(6.0)
            .center(Point::new(1.0, 2.0, 3.0))
            .build();

        // Check parameters
        assert_eq!(model.get_parameter("radius"), Some(ParamType::Numeric(6.0)));
        assert_eq!(
            model.get_parameter("center"),
            Some(ParamType::Point(Point::new(1.0, 2.0, 3.0)))
        );

        // Update parameter and check
        model.set_parameter("radius", ParamType::Numeric(8.0));
        model.update();
        assert_eq!(model.get_parameter("radius"), Some(ParamType::Numeric(8.0)));
    }

    #[test]
    fn test_constraints() {
        // Create a model with two points and a distance constraint
        let mut model = ParametricModel::new("test_constraints");

        // Add points
        model.add_parameter(PointParam::new(
            "point1",
            Point::new(0.0, 0.0, 0.0),
            "First point",
        ));
        model.add_parameter(PointParam::new(
            "point2",
            Point::new(10.0, 0.0, 0.0),
            "Second point",
        ));

        // Add distance constraint
        model.add_constraint(DistanceConstraint::new(
            "distance", "point1", "point2", 10.0, 1e-6,
        ));

        // Check constraint is satisfied
        assert!(model.check_constraints());

        // Modify point2 to break constraint
        model.set_parameter("point2", ParamType::Point(Point::new(15.0, 0.0, 0.0)));
        model.update();

        // Check constraint is not satisfied
        assert!(!model.check_constraints());
    }
}
