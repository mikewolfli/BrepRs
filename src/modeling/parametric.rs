use std::collections::HashMap;

use crate::geometry::Point;
use crate::topology::topods_shape::TopoDsShape;

/// Parameter type
#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    /// Boolean parameter
    Boolean(bool),
    /// Integer parameter
    Integer(i64),
    /// Float parameter
    Float(f64),
    /// String parameter
    String(String),
    /// Vector parameter
    Vector([f64; 3]),
    /// Point parameter
    Point([f64; 3]),
}

/// Parameter definition
#[derive(Debug, Clone)]
pub struct ParamDefinition {
    /// Parameter name
    name: String,
    /// Parameter description
    description: String,
    /// Default value
    default_value: Parameter,
    /// Minimum value (if applicable)
    min_value: Option<Parameter>,
    /// Maximum value (if applicable)
    max_value: Option<Parameter>,
    /// Whether the parameter is read-only
    read_only: bool,
}

impl ParamDefinition {
    /// Create a new parameter definition
    pub fn new(
        name: String,
        description: String,
        default_value: Parameter,
        min_value: Option<Parameter>,
        max_value: Option<Parameter>,
        read_only: bool,
    ) -> Self {
        Self {
            name,
            description,
            default_value,
            min_value,
            max_value,
            read_only,
        }
    }

    /// Get the parameter name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the parameter description
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the default value
    pub fn default_value(&self) -> &Parameter {
        &self.default_value
    }

    /// Get the minimum value
    pub fn min_value(&self) -> Option<&Parameter> {
        self.min_value.as_ref()
    }

    /// Get the maximum value
    pub fn max_value(&self) -> Option<&Parameter> {
        self.max_value.as_ref()
    }

    /// Check if the parameter is read-only
    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    /// Validate a parameter value
    pub fn validate(&self, value: &Parameter) -> bool {
        // Check if types match
        if std::mem::discriminant(value) != std::mem::discriminant(&self.default_value) {
            return false;
        }

        // Check min/max values
        match (self.min_value.as_ref(), self.max_value.as_ref(), value) {
            (Some(min), Some(max), Parameter::Float(v)) => {
                if let (Parameter::Float(min_v), Parameter::Float(max_v)) = (min, max) {
                    *v >= *min_v && *v <= *max_v
                } else {
                    false
                }
            }
            (Some(min), Some(max), Parameter::Integer(v)) => {
                if let (Parameter::Integer(min_v), Parameter::Integer(max_v)) = (min, max) {
                    *v >= *min_v && *v <= *max_v
                } else {
                    false
                }
            }
            _ => true,
        }
    }
}

/// Parameter manager
pub struct ParameterManager {
    /// Parameter definitions
    definitions: HashMap<String, ParamDefinition>,
    /// Parameter values
    values: HashMap<String, Parameter>,
}

impl ParameterManager {
    /// Create a new parameter manager
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            values: HashMap::new(),
        }
    }

    /// Add a parameter definition
    pub fn add_parameter(&mut self, definition: ParamDefinition) {
        self.definitions
            .insert(definition.name().to_string(), definition);
    }

    /// Set a parameter value
    pub fn set_parameter(&mut self, name: &str, value: Parameter) -> bool {
        if let Some(def) = self.definitions.get(name) {
            if def.is_read_only() {
                return false;
            }

            if def.validate(&value) {
                self.values.insert(name.to_string(), value);
                return true;
            }
        }
        false
    }

    /// Get a parameter value
    pub fn get_parameter(&self, name: &str) -> Option<&Parameter> {
        self.values
            .get(name)
            .or_else(|| self.definitions.get(name).map(|def| def.default_value()))
    }

    /// Get all parameter definitions
    pub fn definitions(&self) -> &HashMap<String, ParamDefinition> {
        &self.definitions
    }

    /// Get all parameter values
    pub fn values(&self) -> &HashMap<String, Parameter> {
        &self.values
    }

    /// Reset all parameters to their default values
    pub fn reset_to_defaults(&mut self) {
        self.values.clear();
    }
}

/// Trait for parametric shapes
pub trait ParametricShape {
    /// Get the parameter manager
    fn parameters(&self) -> &ParameterManager;

    /// Get mutable access to the parameter manager
    fn parameters_mut(&mut self) -> &mut ParameterManager;

    /// Update the shape based on parameters
    fn update(&mut self) -> bool;

    /// Get the underlying shape
    fn shape(&self) -> &TopoDsShape;
}

/// Parametric cube
pub struct ParametricCube {
    /// Parameter manager
    params: ParameterManager,
    /// Underlying shape
    shape: TopoDsShape,
}

impl ParametricCube {
    /// Create a new parametric cube
    pub fn new() -> Self {
        let mut params = ParameterManager::new();

        // Add parameters
        params.add_parameter(ParamDefinition::new(
            "width".to_string(),
            "Cube width".to_string(),
            Parameter::Float(1.0),
            Some(Parameter::Float(0.001)),
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "height".to_string(),
            "Cube height".to_string(),
            Parameter::Float(1.0),
            Some(Parameter::Float(0.001)),
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "depth".to_string(),
            "Cube depth".to_string(),
            Parameter::Float(1.0),
            Some(Parameter::Float(0.001)),
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_x".to_string(),
            "Center X coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_y".to_string(),
            "Center Y coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_z".to_string(),
            "Center Z coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        let mut cube = Self {
            params,
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Vertex),
        };

        // Initial update
        cube.update();
        cube
    }
}

impl ParametricShape for ParametricCube {
    fn parameters(&self) -> &ParameterManager {
        &self.params
    }

    fn parameters_mut(&mut self) -> &mut ParameterManager {
        &mut self.params
    }

    fn update(&mut self) -> bool {
        // Get parameters
        let width = if let Some(Parameter::Float(w)) = self.params.get_parameter("width") {
            *w
        } else {
            1.0
        };

        let height = if let Some(Parameter::Float(h)) = self.params.get_parameter("height") {
            *h
        } else {
            1.0
        };

        let depth = if let Some(Parameter::Float(d)) = self.params.get_parameter("depth") {
            *d
        } else {
            1.0
        };

        let center_x = if let Some(Parameter::Float(x)) = self.params.get_parameter("center_x") {
            *x
        } else {
            0.0
        };

        let center_y = if let Some(Parameter::Float(y)) = self.params.get_parameter("center_y") {
            *y
        } else {
            0.0
        };

        let center_z = if let Some(Parameter::Float(z)) = self.params.get_parameter("center_z") {
            *z
        } else {
            0.0
        };

        // Calculate cube vertices
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        let half_depth = depth / 2.0;

        let _vertices = vec![
            Point::new(
                center_x - half_width,
                center_y - half_height,
                center_z - half_depth,
            ),
            Point::new(
                center_x + half_width,
                center_y - half_height,
                center_z - half_depth,
            ),
            Point::new(
                center_x + half_width,
                center_y + half_height,
                center_z - half_depth,
            ),
            Point::new(
                center_x - half_width,
                center_y + half_height,
                center_z - half_depth,
            ),
            Point::new(
                center_x - half_width,
                center_y - half_height,
                center_z + half_depth,
            ),
            Point::new(
                center_x + half_width,
                center_y - half_height,
                center_z + half_depth,
            ),
            Point::new(
                center_x + half_width,
                center_y + half_height,
                center_z + half_depth,
            ),
            Point::new(
                center_x - half_width,
                center_y + half_height,
                center_z + half_depth,
            ),
        ];

        // Create cube shape
        // TODO: Implement actual cube creation using BRepBuilder
        // For now, create a placeholder shape with the correct type
        self.shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Solid);
        true
    }

    fn shape(&self) -> &TopoDsShape {
        &self.shape
    }
}

/// Parametric cylinder
pub struct ParametricCylinder {
    /// Parameter manager
    params: ParameterManager,
    /// Underlying shape
    shape: TopoDsShape,
}

impl ParametricCylinder {
    /// Create a new parametric cylinder
    pub fn new() -> Self {
        let mut params = ParameterManager::new();

        // Add parameters
        params.add_parameter(ParamDefinition::new(
            "radius".to_string(),
            "Cylinder radius".to_string(),
            Parameter::Float(1.0),
            Some(Parameter::Float(0.001)),
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "height".to_string(),
            "Cylinder height".to_string(),
            Parameter::Float(2.0),
            Some(Parameter::Float(0.001)),
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_x".to_string(),
            "Center X coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_y".to_string(),
            "Center Y coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        params.add_parameter(ParamDefinition::new(
            "center_z".to_string(),
            "Center Z coordinate".to_string(),
            Parameter::Float(0.0),
            None,
            None,
            false,
        ));

        let mut cylinder = Self {
            params,
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Vertex),
        };

        // Initial update
        cylinder.update();
        cylinder
    }
}

impl ParametricShape for ParametricCylinder {
    fn parameters(&self) -> &ParameterManager {
        &self.params
    }

    fn parameters_mut(&mut self) -> &mut ParameterManager {
        &mut self.params
    }

    fn update(&mut self) -> bool {
        // Get parameters
        let _radius = if let Some(Parameter::Float(r)) = self.params.get_parameter("radius") {
            *r
        } else {
            1.0
        };

        let height = if let Some(Parameter::Float(h)) = self.params.get_parameter("height") {
            *h
        } else {
            2.0
        };

        let center_x = if let Some(Parameter::Float(x)) = self.params.get_parameter("center_x") {
            *x
        } else {
            0.0
        };

        let center_y = if let Some(Parameter::Float(y)) = self.params.get_parameter("center_y") {
            *y
        } else {
            0.0
        };

        let center_z = if let Some(Parameter::Float(z)) = self.params.get_parameter("center_z") {
            *z
        } else {
            0.0
        };

        // Calculate cylinder parameters
        let _bottom_center = Point::new(center_x, center_y, center_z - height / 2.0);
        let _top_center = Point::new(center_x, center_y, center_z + height / 2.0);

        // Create cylinder shape
        // TODO: Implement actual cylinder creation using BRepBuilder
        // For now, create a placeholder shape with the correct type
        self.shape = TopoDsShape::new(crate::topology::shape_enum::ShapeType::Solid);
        true
    }

    fn shape(&self) -> &TopoDsShape {
        &self.shape
    }
}
