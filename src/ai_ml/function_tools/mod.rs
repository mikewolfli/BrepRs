//! Function Call Tools and Skills Module
//!
//! This module provides functionality for encapsulating graphics library API as tools
//! for large language models to call, and for developing specialized skill plugins.

use std::collections::HashMap;

use crate::ai_ml::protocol::{AiDataType, AiProtocolError, AiResult};
use crate::mesh::mesh_data::Mesh3D;

/// Function Call Tool
#[derive(Clone)]
pub struct FunctionCallTool {
    name: String,
    description: String,
    parameters: HashMap<String, String>,
    implementation: fn(&HashMap<String, String>) -> AiResult<AiDataType>,
}

impl FunctionCallTool {
    pub fn new(
        name: &str,
        description: &str,
        parameters: HashMap<String, String>,
        implementation: fn(&HashMap<String, String>) -> AiResult<AiDataType>,
    ) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
            implementation,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    pub fn execute(&self, args: &HashMap<String, String>) -> AiResult<AiDataType> {
        (self.implementation)(args)
    }
}

/// Skill Plugin
pub struct SkillPlugin {
    name: String,
    description: String,
    version: String,
    tools: Vec<FunctionCallTool>,
}

impl SkillPlugin {
    pub fn new(name: &str, description: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: version.to_string(),
            tools: Vec::new(),
        }
    }

    pub fn add_tool(&mut self, tool: FunctionCallTool) {
        self.tools.push(tool);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn tools(&self) -> &Vec<FunctionCallTool> {
        &self.tools
    }

    pub fn find_tool(&self, name: &str) -> Option<&FunctionCallTool> {
        self.tools.iter().find(|tool| tool.name() == name)
    }
}

/// Function Call Tool Manager
pub struct FunctionToolManager {
    tools: HashMap<String, FunctionCallTool>,
    plugins: HashMap<String, SkillPlugin>,
}

impl FunctionToolManager {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            plugins: HashMap::new(),
        }
    }

    pub fn register_tool(&mut self, tool: FunctionCallTool) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn add_plugin(&mut self, plugin: SkillPlugin) {
        for tool in plugin.tools() {
            self.tools.insert(tool.name().to_string(), tool.clone());
        }
        self.plugins.insert(plugin.name().to_string(), plugin);
    }

    pub fn get_tool(&self, name: &str) -> Option<&FunctionCallTool> {
        self.tools.get(name)
    }

    pub fn get_plugin(&self, name: &str) -> Option<&SkillPlugin> {
        self.plugins.get(name)
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    pub fn list_plugins(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    pub fn execute_tool(&self, name: &str, args: &HashMap<String, String>) -> AiResult<AiDataType> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(args),
            None => Err(AiProtocolError::InvalidData(format!(
                "Tool '{}' not found",
                name
            ))),
        }
    }
}

/// Built-in tools
pub fn create_builtin_tools() -> Vec<FunctionCallTool> {
    let mut tools = Vec::new();

    // Create cube tool
    let cube_params = HashMap::from([
        ("size".to_string(), "f64: Size of the cube".to_string()),
        (
            "position".to_string(),
            "[f64; 3]: Position of the cube".to_string(),
        ),
    ]);

    let cube_tool = FunctionCallTool::new(
        "create_cube",
        "Creates a cube mesh with the specified size and position",
        cube_params,
        |args| {
            let size = args.get("size").and_then(|s| s.parse().ok()).unwrap_or(1.0);
            let position = args
                .get("position")
                .and_then(|p| {
                    let parts: Vec<&str> = p
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .collect();
                    if parts.len() == 3 {
                        Some([
                            parts[0].trim().parse().unwrap_or(0.0),
                            parts[1].trim().parse().unwrap_or(0.0),
                            parts[2].trim().parse().unwrap_or(0.0),
                        ])
                    } else {
                        None
                    }
                })
                .unwrap_or([0.0, 0.0, 0.0]);

            // Create cube mesh
            let half_size = size / 2.0;
            let mut mesh = Mesh3D::new();

            // Add vertices
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] - half_size,
                position[1] - half_size,
                position[2] + half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] + half_size,
                position[1] - half_size,
                position[2] + half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] + half_size,
                position[1] + half_size,
                position[2] + half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] - half_size,
                position[1] + half_size,
                position[2] + half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] - half_size,
                position[1] - half_size,
                position[2] - half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] + half_size,
                position[1] - half_size,
                position[2] - half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] + half_size,
                position[1] + half_size,
                position[2] - half_size,
            ));
            mesh.add_vertex(crate::geometry::Point::new(
                position[0] - half_size,
                position[1] + half_size,
                position[2] - half_size,
            ));

            // Add faces
            mesh.add_face(vec![0, 1, 2, 3]);
            mesh.add_face(vec![4, 5, 6, 7]);
            mesh.add_face(vec![1, 5, 6, 2]);
            mesh.add_face(vec![4, 0, 3, 7]);
            mesh.add_face(vec![3, 2, 6, 7]);
            mesh.add_face(vec![4, 5, 1, 0]);

            Ok(AiDataType::Mesh(mesh))
        },
    );

    tools.push(cube_tool);

    // Create sphere tool
    let sphere_params = HashMap::from([
        (
            "radius".to_string(),
            "f64: Radius of the sphere".to_string(),
        ),
        (
            "position".to_string(),
            "[f64; 3]: Position of the sphere".to_string(),
        ),
        (
            "segments".to_string(),
            "usize: Number of segments".to_string(),
        ),
    ]);

    let sphere_tool = FunctionCallTool::new(
        "create_sphere",
        "Creates a sphere mesh with the specified radius, position, and segments",
        sphere_params,
        |args| {
            let radius = args
                .get("radius")
                .and_then(|r| r.parse().ok())
                .unwrap_or(1.0);
            let position = args
                .get("position")
                .and_then(|p| {
                    let parts: Vec<&str> = p
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .collect();
                    if parts.len() == 3 {
                        Some([
                            parts[0].trim().parse().unwrap_or(0.0),
                            parts[1].trim().parse().unwrap_or(0.0),
                            parts[2].trim().parse().unwrap_or(0.0),
                        ])
                    } else {
                        None
                    }
                })
                .unwrap_or([0.0, 0.0, 0.0]);
            let segments = args
                .get("segments")
                .and_then(|s| s.parse().ok())
                .unwrap_or(20);

            // Create sphere mesh
            let mut mesh = Mesh3D::new();

            // Generate vertices
            for i in 0..=segments {
                let v = i as f64 / segments as f64;
                let theta = v * std::f64::consts::PI;

                for j in 0..segments {
                    let u = j as f64 / segments as f64;
                    let phi = u * 2.0 * std::f64::consts::PI;

                    let x = radius * theta.sin() * phi.cos() + position[0];
                    let y = radius * theta.cos() + position[1];
                    let z = radius * theta.sin() * phi.sin() + position[2];

                    mesh.add_vertex(crate::geometry::Point::new(x, y, z));
                }
            }

            // Generate faces
            for i in 0..segments {
                for j in 0..segments {
                    let v0 = i * segments + j;
                    let v1 = i * segments + (j + 1) % segments;
                    let v2 = (i + 1) * segments + (j + 1) % segments;
                    let v3 = (i + 1) * segments + j;

                    mesh.add_face(vec![v0, v1, v2, v3]);
                }
            }

            Ok(AiDataType::Mesh(mesh))
        },
    );

    tools.push(sphere_tool);

    tools
}

/// Built-in plugins
pub fn create_builtin_plugins() -> Vec<SkillPlugin> {
    let mut plugins = Vec::new();

    // Create geometry plugin
    let mut geometry_plugin = SkillPlugin::new(
        "geometry",
        "Geometry creation and manipulation tools",
        "1.0.0",
    );

    let builtin_tools = create_builtin_tools();
    for tool in builtin_tools {
        geometry_plugin.add_tool(tool);
    }

    plugins.push(geometry_plugin);

    plugins
}
