use crate::foundation::handle::Handle;
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

        let vertices = vec![
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

        // Create cube shape using BRepBuilder
        let builder = crate::modeling::brep_builder::BrepBuilder::new();

        // Create vertices
        let v0 = builder.make_vertex(vertices[0]);
        let v1 = builder.make_vertex(vertices[1]);
        let v2 = builder.make_vertex(vertices[2]);
        let v3 = builder.make_vertex(vertices[3]);
        let v4 = builder.make_vertex(vertices[4]);
        let v5 = builder.make_vertex(vertices[5]);
        let v6 = builder.make_vertex(vertices[6]);
        let v7 = builder.make_vertex(vertices[7]);

        // Create edges
        let e0 = builder.make_edge(v0.clone(), v1.clone());
        let e1 = builder.make_edge(v1.clone(), v2.clone());
        let e2 = builder.make_edge(v2.clone(), v3.clone());
        let e3 = builder.make_edge(v3.clone(), v0.clone());
        let e4 = builder.make_edge(v4.clone(), v5.clone());
        let e5 = builder.make_edge(v5.clone(), v6.clone());
        let e6 = builder.make_edge(v6.clone(), v7.clone());
        let e7 = builder.make_edge(v7.clone(), v4.clone());
        let e8 = builder.make_edge(v0.clone(), v4.clone());
        let e9 = builder.make_edge(v1.clone(), v5.clone());
        let e10 = builder.make_edge(v2.clone(), v6.clone());
        let e11 = builder.make_edge(v3.clone(), v7.clone());

        // Create wires for each face
        // Note: Edges are shared between faces, so we need to clone them
        let mut wire_bottom = crate::topology::topods_wire::TopoDsWire::new();
        wire_bottom.add_edge(e0.clone());
        wire_bottom.add_edge(e1.clone());
        wire_bottom.add_edge(e2.clone());
        wire_bottom.add_edge(e3.clone());

        let mut wire_top = crate::topology::topods_wire::TopoDsWire::new();
        wire_top.add_edge(e4.clone());
        wire_top.add_edge(e5.clone());
        wire_top.add_edge(e6.clone());
        wire_top.add_edge(e7.clone());

        let mut wire_front = crate::topology::topods_wire::TopoDsWire::new();
        wire_front.add_edge(e0.clone());
        wire_front.add_edge(e9.clone());
        wire_front.add_edge(e4.clone());
        wire_front.add_edge(e8.clone());

        let mut wire_back = crate::topology::topods_wire::TopoDsWire::new();
        wire_back.add_edge(e2.clone());
        wire_back.add_edge(e10.clone());
        wire_back.add_edge(e6.clone());
        wire_back.add_edge(e11.clone());

        let mut wire_left = crate::topology::topods_wire::TopoDsWire::new();
        wire_left.add_edge(e3.clone());
        wire_left.add_edge(e11.clone());
        wire_left.add_edge(e7.clone());
        wire_left.add_edge(e8.clone());

        let mut wire_right = crate::topology::topods_wire::TopoDsWire::new();
        wire_right.add_edge(e1.clone());
        wire_right.add_edge(e10.clone());
        wire_right.add_edge(e5.clone());
        wire_right.add_edge(e9.clone());

        // Create faces
        let face_bottom =
            builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_bottom)));
        let face_top = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_top)));
        let face_front = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_front)));
        let face_back = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_back)));
        let face_left = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_left)));
        let face_right = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(wire_right)));

        // Create shell
        let mut shell = crate::topology::topods_shell::TopoDsShell::new();
        shell.add_face(face_bottom);
        shell.add_face(face_top);
        shell.add_face(face_front);
        shell.add_face(face_back);
        shell.add_face(face_left);
        shell.add_face(face_right);

        // Create solid
        let mut solid = crate::topology::topods_solid::TopoDsSolid::new();
        solid.set_outer_shell(Handle::new(std::sync::Arc::new(shell)));

        // Set the shape
        self.shape = solid.shape().clone();
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
        let radius = if let Some(Parameter::Float(r)) = self.params.get_parameter("radius") {
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
        let bottom_center = Point::new(center_x, center_y, center_z - height / 2.0);
        let top_center = Point::new(center_x, center_y, center_z + height / 2.0);

        // Create cylinder shape using BRepBuilder
        let builder = crate::modeling::brep_builder::BrepBuilder::new();

        // Create bottom and top centers
        let _bottom_center_vertex = builder.make_vertex(bottom_center);
        let _top_center_vertex = builder.make_vertex(top_center);

        // Create circular edges for bottom and top faces
        // For simplicity, we'll create a regular polygon approximation of a circle
        let num_segments = 32;
        let mut bottom_vertices = Vec::new();
        let mut top_vertices = Vec::new();

        for i in 0..num_segments {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / (num_segments as f64);
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            bottom_vertices.push(builder.make_vertex(Point::new(x, y, bottom_center.z)));
            top_vertices.push(builder.make_vertex(Point::new(x, y, top_center.z)));
        }

        // Create bottom face wire
        let mut bottom_wire = crate::topology::topods_wire::TopoDsWire::new();
        for i in 0..num_segments {
            let next_i = (i + 1) % num_segments;
            let edge =
                builder.make_edge(bottom_vertices[i].clone(), bottom_vertices[next_i].clone());
            bottom_wire.add_edge(edge);
        }

        // Create top face wire
        let mut top_wire = crate::topology::topods_wire::TopoDsWire::new();
        for i in 0..num_segments {
            let next_i = (i + 1) % num_segments;
            let edge = builder.make_edge(top_vertices[i].clone(), top_vertices[next_i].clone());
            top_wire.add_edge(edge);
        }

        // Create side face wires
        let mut side_faces = Vec::new();
        for i in 0..num_segments {
            let next_i = (i + 1) % num_segments;

            let mut side_wire = crate::topology::topods_wire::TopoDsWire::new();
            side_wire
                .add_edge(builder.make_edge(bottom_vertices[i].clone(), top_vertices[i].clone()));
            side_wire
                .add_edge(builder.make_edge(top_vertices[i].clone(), top_vertices[next_i].clone()));
            side_wire.add_edge(builder.make_edge(
                top_vertices[next_i].clone(),
                bottom_vertices[next_i].clone(),
            ));
            side_wire.add_edge(
                builder.make_edge(bottom_vertices[next_i].clone(), bottom_vertices[i].clone()),
            );

            let side_face =
                builder.make_face_with_wire(Handle::new(std::sync::Arc::new(side_wire)));
            side_faces.push(side_face);
        }

        // Create bottom and top faces
        let bottom_face =
            builder.make_face_with_wire(Handle::new(std::sync::Arc::new(bottom_wire)));
        let top_face = builder.make_face_with_wire(Handle::new(std::sync::Arc::new(top_wire)));

        // Create shell
        let mut shell = crate::topology::topods_shell::TopoDsShell::new();
        shell.add_face(bottom_face);
        shell.add_face(top_face);
        for face in side_faces {
            shell.add_face(face);
        }

        // Create solid
        let mut solid = crate::topology::topods_solid::TopoDsSolid::new();
        solid.set_outer_shell(Handle::new(std::sync::Arc::new(shell)));

        // Set the shape
        self.shape = solid.shape().clone();
        true
    }

    fn shape(&self) -> &TopoDsShape {
        &self.shape
    }
}
