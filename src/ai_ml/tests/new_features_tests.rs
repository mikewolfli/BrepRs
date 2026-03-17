//! New Features Tests
//!
//! Tests for the newly implemented AI/ML features including:
//! - Sketch-to-3D and Image-to-3D generation
//! - Function Call Tools and Skills/Plugins
//! - AI Design Assistant

use crate::ai_ml::design_assistant::{AiDesignAssistant, DesignAssistantSettings};
use crate::ai_ml::function_tools::{
    create_builtin_plugins, create_builtin_tools, FunctionToolManager,
};
use crate::ai_ml::style_transfer::{StyleReference, StyleTransferSettings};
use crate::ai_ml::text_to_3d::{TextTo3DGenerator, TextTo3DSettings};
use crate::mesh::mesh_data::Mesh3D;

#[test]
fn test_sketch_to_3d_generation() {
    let generator = TextTo3DGenerator::new();
    let result = generator.generate_from_sketch("test_sketch.png");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.mesh.vertices.is_empty());
    assert!(!result.mesh.faces.is_empty());
    assert!(result.generation_time > 0.0);
    assert!(result.quality_score >= 0.0 && result.quality_score <= 1.0);
}

#[test]
fn test_image_to_3d_generation() {
    let generator = TextTo3DGenerator::new();
    let result = generator.generate_from_image("test_image.png");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.mesh.vertices.is_empty());
    assert!(!result.mesh.faces.is_empty());
    assert!(result.generation_time > 0.0);
    assert!(result.quality_score >= 0.0 && result.quality_score <= 1.0);
}

#[test]
fn test_function_tools() {
    let tools = create_builtin_tools();
    assert!(!tools.is_empty());
    assert!(tools.iter().any(|tool| tool.name() == "create_cube"));
    assert!(tools.iter().any(|tool| tool.name() == "create_sphere"));
}

#[test]
fn test_skill_plugins() {
    let plugins = create_builtin_plugins();
    assert!(!plugins.is_empty());
    assert!(plugins.iter().any(|plugin| plugin.name() == "geometry"));
}

#[test]
fn test_function_tool_manager() {
    let mut manager = FunctionToolManager::new();
    let plugins = create_builtin_plugins();
    for plugin in plugins {
        manager.register_plugin(plugin);
    }

    let tools = manager.list_tools();
    assert!(!tools.is_empty());
    assert!(tools.contains(&"create_cube".to_string()));
    assert!(tools.contains(&"create_sphere".to_string()));

    let plugins = manager.list_plugins();
    assert!(!plugins.is_empty());
    assert!(plugins.contains(&"geometry".to_string()));
}

#[test]
fn test_ai_design_assistant_multimodal() {
    let assistant = AiDesignAssistant::new();
    let result = assistant.generate_from_multimodal("a red cube", None, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.final_mesh.vertices.is_empty());
    assert!(!result.final_mesh.faces.is_empty());
    assert!(!result.steps.is_empty());
    assert!(result.total_time > 0.0);
    assert!(result.quality_score >= 0.0 && result.quality_score <= 1.0);
}

#[test]
fn test_ai_design_assistant_style_transfer() {
    let assistant = AiDesignAssistant::new();
    let mut mesh = Mesh3D::new();
    mesh.add_vertex(
        crate::geometry::Point::new(-0.5, -0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(0.5, -0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(0.5, 0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(-0.5, 0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_face(vec![0, 1, 2, 3], Some([0.0, 0.0, 1.0]));

    let style_reference = StyleReference::Description("low poly".to_string());
    let result = assistant.apply_style(&mesh, &style_reference);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.final_mesh.vertices.is_empty());
    assert!(!result.final_mesh.faces.is_empty());
    assert!(!result.steps.is_empty());
}

#[test]
fn test_ai_design_assistant_material_generation() {
    let assistant = AiDesignAssistant::new();
    let mut mesh = Mesh3D::new();
    mesh.add_vertex(
        crate::geometry::Point::new(-0.5, -0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(0.5, -0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(0.5, 0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_vertex(
        crate::geometry::Point::new(-0.5, 0.5, 0.5),
        Some([0.0, 0.0, 1.0]),
    );
    mesh.add_face(vec![0, 1, 2, 3], Some([0.0, 0.0, 1.0]));

    let result = assistant.generate_material(&mesh, "red metal");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.final_mesh.vertices.is_empty());
    assert!(!result.final_mesh.faces.is_empty());
    assert!(!result.steps.is_empty());
}

#[test]
fn test_ai_design_assistant_workflow() {
    let assistant = AiDesignAssistant::new();
    let result = assistant.execute_workflow("create a cube and optimize it");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(!result.final_mesh.vertices.is_empty());
    assert!(!result.final_mesh.faces.is_empty());
    assert!(!result.steps.is_empty());
}
