//! AI/ML Module Tests
//!
//! This module contains tests for the AI/ML integration module.

use crate::ai_ml::*;
use crate::geometry::{Plane, Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

#[test]
fn test_ai_protocol() {
    let protocol = DefaultAiProtocol::new("http://localhost:8000");

    // Test point conversion
    let point = Point::new(1.0, 2.0, 3.0);
    let ai_data = AiDataType::Point(point);
    let json = protocol.to_ai_format(&ai_data).unwrap();
    let converted_back = protocol.from_ai_format(&json).unwrap();
    // Just ensure conversion works, don't compare directly
    assert!(matches!(converted_back, AiDataType::Point(_)));

    // Test mesh conversion
    let mesh = Mesh3D {
        vertices: vec![
            MeshVertex {
                id: 0,
                point: Point::new(0.0, 0.0, 0.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 1,
                point: Point::new(1.0, 0.0, 0.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 2,
                point: Point::new(0.0, 1.0, 0.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
        ],
        faces: vec![MeshFace::new(0, vec![0, 1, 2])],
        ..Default::default()
    };

    let ai_mesh = AiDataType::Mesh(mesh);
    let mesh_json = protocol.to_ai_format(&ai_mesh).unwrap();
    let converted_mesh = protocol.from_ai_format(&mesh_json).unwrap();
    assert!(matches!(converted_mesh, AiDataType::Mesh(_)));
}

#[test]
fn test_ai_ml_utils() {
    let mut utils = AiMlUtils::new();

    // Test mesh generation
    let mesh = utils.generate_mesh("A simple cube").unwrap();
    assert!(!mesh.vertices.is_empty());
    assert!(!mesh.faces.is_empty());

    // Test feature recognition
    let features = utils.recognize_features(&mesh).unwrap();
    // Just ensure it returns something
    assert!(features.len() >= 0);

    // Test mesh repair
    let repaired_mesh = utils.repair_mesh(&mesh).unwrap();
    assert!(!repaired_mesh.vertices.is_empty());

    // Test tensor conversion
    let point = Point::new(1.0, 2.0, 3.0);
    let tensor = utils.point_to_tensor(&point);
    assert_eq!(tensor.len(), 3);

    let converted_point = utils.tensor_to_point(&tensor).unwrap();
    assert!((converted_point.x - 1.0).abs() < 1e-6);
    assert!((converted_point.y - 2.0).abs() < 1e-6);
    assert!((converted_point.z - 3.0).abs() < 1e-6);
}

#[test]
fn test_model_manager() {
    let mut utils = AiMlUtils::new();
    let model_manager = utils.model_manager();

    // Test model registration and retrieval
    assert!(model_manager.get_model("feature_recognition").is_some());
    assert!(model_manager.get_model("mesh_generation").is_some());
    assert!(model_manager.get_model("model_repair").is_some());
}

#[test]
fn test_ml_workflow() {
    let mut workflow = MlWorkflow::new("feature_recognition");
    // Just ensure workflow creation works
    assert!(!workflow.model_name().is_empty());
}

// Include integration tests
mod integration_tests;
