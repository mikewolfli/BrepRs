//! Integration Tests for AI/ML Module
//!
//! This file contains integration tests for the AI/ML module, testing all core functionality
//! including model quality assessment, optimization, text-to-3D generation, material generation,
//! and style transfer.

use crate::ai_ml::material_texture::{
    MaterialGenerationSettings, MaterialTextureExt, MaterialTextureGenerator,
};
use crate::ai_ml::model_optimization::{
    LodSettings, MeshOptimizationExt, ModelOptimizer, OptimizationSettings,
};
use crate::ai_ml::model_quality::{MeshQualityExt, ModelQualityEvaluator, ModelRepairTool};
use crate::ai_ml::style_transfer::{
    StyleReference, StyleTransferExt, StyleTransferSettings, StyleTransferTool,
};
use crate::ai_ml::text_to_3d::{TextTo3DExt, TextTo3DGenerator, TextTo3DSettings};
use crate::geometry::{Point, Vector};
use crate::mesh::mesh_data::{Mesh3D, MeshFace, MeshVertex};

/// Create a simple cube mesh for testing
fn create_test_cube() -> Mesh3D {
    let half_size = 0.5;
    let mut mesh = Mesh3D::new();

    // Add vertices
    let v0 = mesh.add_vertex(Point::new(-half_size, -half_size, half_size));
    let v1 = mesh.add_vertex(Point::new(half_size, -half_size, half_size));
    let v2 = mesh.add_vertex(Point::new(half_size, half_size, half_size));
    let v3 = mesh.add_vertex(Point::new(-half_size, half_size, half_size));
    let v4 = mesh.add_vertex(Point::new(-half_size, -half_size, -half_size));
    let v5 = mesh.add_vertex(Point::new(half_size, -half_size, -half_size));
    let v6 = mesh.add_vertex(Point::new(half_size, half_size, -half_size));
    let v7 = mesh.add_vertex(Point::new(-half_size, half_size, -half_size));

    // Set vertex normals
    mesh.vertices[v0].normal = Some([0.0, 0.0, 1.0]);
    mesh.vertices[v1].normal = Some([0.0, 0.0, 1.0]);
    mesh.vertices[v2].normal = Some([0.0, 0.0, 1.0]);
    mesh.vertices[v3].normal = Some([0.0, 0.0, 1.0]);
    mesh.vertices[v4].normal = Some([0.0, 0.0, -1.0]);
    mesh.vertices[v5].normal = Some([0.0, 0.0, -1.0]);
    mesh.vertices[v6].normal = Some([0.0, 0.0, -1.0]);
    mesh.vertices[v7].normal = Some([0.0, 0.0, -1.0]);

    // Add faces
    mesh.add_face(vec![v0, v1, v2, v3]);
    mesh.add_face(vec![v4, v5, v6, v7]);
    mesh.add_face(vec![v1, v5, v6, v2]);
    mesh.add_face(vec![v4, v0, v3, v7]);
    mesh.add_face(vec![v3, v2, v6, v7]);
    mesh.add_face(vec![v4, v5, v1, v0]);

    // Set face normals
    mesh.faces[0].normal = Some([0.0, 0.0, 1.0]);
    mesh.faces[1].normal = Some([0.0, 0.0, -1.0]);
    mesh.faces[2].normal = Some([1.0, 0.0, 0.0]);
    mesh.faces[3].normal = Some([-1.0, 0.0, 0.0]);
    mesh.faces[4].normal = Some([0.0, 1.0, 0.0]);
    mesh.faces[5].normal = Some([0.0, -1.0, 0.0]);

    mesh
}

#[test]
fn test_model_quality_assessment() {
    let mesh = create_test_cube();

    // Test quality evaluation
    let report = mesh.evaluate_quality();
    assert!(report.is_valid, "Cube should be valid");
    assert_eq!(report.error_count, 0, "Cube should have no errors");
    assert!(
        report.quality_score > 0.9,
        "Cube should have high quality score"
    );

    // Test model repair
    let repaired_mesh = mesh.repair().unwrap();
    assert_eq!(
        repaired_mesh.vertices.len(),
        mesh.vertices.len(),
        "Repaired mesh should have same vertex count"
    );
    assert_eq!(
        repaired_mesh.faces.len(),
        mesh.faces.len(),
        "Repaired mesh should have same face count"
    );
}

#[test]
fn test_model_optimization() {
    let mesh = create_test_cube();

    // Test mesh simplification
    let settings = OptimizationSettings {
        target_polygon_count: Some(4),
        target_reduction_ratio: None,
        quality_threshold: 0.8,
        preserve_boundaries: true,
        preserve_features: true,
    };

    let result = mesh.simplify(&settings).unwrap();
    assert!(
        result.simplified_polygon_count <= 4,
        "Simplified mesh should have at most 4 polygons"
    );
    assert!(
        result.quality_score > 0.5,
        "Simplified mesh should have reasonable quality"
    );

    // Test LOD generation
    let lod_settings = LodSettings {
        levels: 2,
        reduction_ratios: vec![0.7, 0.4],
    };

    let lod_result = mesh.generate_lods(&lod_settings).unwrap();
    assert_eq!(lod_result.lods.len(), 2, "Should generate 2 LODs");
    assert!(
        lod_result.lods[0].faces.len() <= (mesh.faces.len() as f64 * 0.7) as usize,
        "First LOD should be 70% of original"
    );
    assert!(
        lod_result.lods[1].faces.len() <= (mesh.faces.len() as f64 * 0.4) as usize,
        "Second LOD should be 40% of original"
    );

    // Test topology optimization
    let optimized_mesh = mesh.optimize_topology().unwrap();
    assert!(
        !optimized_mesh.vertices.is_empty(),
        "Optimized mesh should not be empty"
    );
    assert!(
        !optimized_mesh.faces.is_empty(),
        "Optimized mesh should not be empty"
    );
}

#[test]
fn test_text_to_3d_generation() {
    let description = "a red cube";
    let settings = TextTo3DSettings::default();

    // Test text to 3D generation
    let result = Mesh3D::from_text(description, &settings).unwrap();
    assert!(
        !result.mesh.vertices.is_empty(),
        "Generated mesh should not be empty"
    );
    assert!(
        !result.mesh.faces.is_empty(),
        "Generated mesh should not be empty"
    );
    assert!(
        result.quality_score > 0.0,
        "Generated mesh should have positive quality score"
    );

    // Test with different description
    let description2 = "a blue sphere";
    let result2 = Mesh3D::from_text(description2, &settings).unwrap();
    assert!(
        !result2.mesh.vertices.is_empty(),
        "Generated mesh should not be empty"
    );
    assert!(
        !result2.mesh.faces.is_empty(),
        "Generated mesh should not be empty"
    );
}

#[test]
fn test_material_generation() {
    let mesh = create_test_cube();
    let description = "shiny red metal";
    let settings = MaterialGenerationSettings::default();

    // Test material generation and application
    let (material_mesh, material) = mesh
        .apply_material_from_text(description, &settings)
        .unwrap();
    assert!(
        !material_mesh.vertices.is_empty(),
        "Mesh with material should not be empty"
    );
    assert!(
        !material.textures.is_empty(),
        "Material should have textures"
    );
    assert_eq!(
        material.properties.name, "metal_red_shiny",
        "Material name should match description"
    );

    // Test with different material description
    let description2 = "matte green plastic";
    let (material_mesh2, material2) = mesh
        .apply_material_from_text(description2, &settings)
        .unwrap();
    assert!(
        !material_mesh2.vertices.is_empty(),
        "Mesh with material should not be empty"
    );
    assert!(
        !material2.textures.is_empty(),
        "Material should have textures"
    );
    assert_eq!(
        material2.properties.name, "plastic_green_matte",
        "Material name should match description"
    );
}

#[test]
fn test_style_transfer() {
    let source_mesh = create_test_cube();
    let style_description = "low poly cartoon style";
    let style_reference = StyleReference::Description(style_description.to_string());
    let settings = StyleTransferSettings::default();

    // Test style transfer
    let result = source_mesh
        .transfer_style(&style_reference, &settings)
        .unwrap();
    assert!(
        !result.stylized_mesh.vertices.is_empty(),
        "Stylized mesh should not be empty"
    );
    assert!(
        !result.stylized_mesh.faces.is_empty(),
        "Stylized mesh should not be empty"
    );
    assert!(
        result.quality_score > 0.0,
        "Stylized mesh should have positive quality score"
    );

    // Test style transfer with mesh reference
    let style_mesh = create_test_cube();
    let style_reference2 = StyleReference::Mesh(style_mesh);
    let result2 = source_mesh
        .transfer_style(&style_reference2, &settings)
        .unwrap();
    assert!(
        !result2.stylized_mesh.vertices.is_empty(),
        "Stylized mesh should not be empty"
    );
    assert!(
        !result2.stylized_mesh.faces.is_empty(),
        "Stylized mesh should not be empty"
    );
}

#[test]
fn test_integration_workflow() {
    // Test complete workflow: generate -> optimize -> add material -> transfer style

    // 1. Generate mesh from text
    let description = "a simple chair";
    let text_settings = TextTo3DSettings::default();
    let generation_result = Mesh3D::from_text(description, &text_settings).unwrap();
    let mut mesh = generation_result.mesh;

    // 2. Optimize mesh
    let opt_settings = OptimizationSettings {
        target_reduction_ratio: Some(0.5),
        ..Default::default()
    };
    let opt_result = mesh.simplify(&opt_settings).unwrap();
    mesh = opt_result.simplified_mesh;

    // 3. Add material
    let mat_description = "wooden chair";
    let mat_settings = MaterialGenerationSettings::default();
    let (mat_mesh, _) = mesh
        .apply_material_from_text(mat_description, &mat_settings)
        .unwrap();
    mesh = mat_mesh;

    // 4. Transfer style
    let style_description = "vintage style";
    let style_reference = StyleReference::Description(style_description.to_string());
    let style_settings = StyleTransferSettings::default();
    let style_result = mesh
        .transfer_style(&style_reference, &style_settings)
        .unwrap();

    // Verify final result
    assert!(
        !style_result.stylized_mesh.vertices.is_empty(),
        "Final mesh should not be empty"
    );
    assert!(
        !style_result.stylized_mesh.faces.is_empty(),
        "Final mesh should not be empty"
    );
    assert!(
        style_result.quality_score > 0.0,
        "Final mesh should have positive quality score"
    );
}
