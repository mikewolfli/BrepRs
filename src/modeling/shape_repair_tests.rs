use crate::modeling::shape_repair::{RepairResult, ShapeRepairTools, TopoDsValidator};
use crate::topology::TopoDsShape;

#[test]
fn test_shape_repair_basic() {
    // Create a simple shape for testing
    let shape = TopoDsShape::new();
    
    // Test shape repair tools
    let mut repair_tools = ShapeRepairTools::new();
    let result = repair_tools.repair(&shape);
    
    // Verify repair result
    assert!(matches!(result.status, crate::modeling::shape_repair::RepairStatus::NoRepairNeeded));
    assert!(result.issues_detected.is_empty());
    assert!(result.issues_fixed.is_empty());
    assert!(result.issues_remaining.is_empty());
}

#[test]
fn test_topology_validator() {
    // Create a simple shape for testing
    let shape = TopoDsShape::new();
    
    // Test topology validator
    let validator = TopoDsValidator::new();
    let errors = validator.validate(&shape);
    
    // Verify validation result
    assert!(errors.is_empty());
    assert!(validator.is_valid(&shape));
}

#[test]
fn test_shape_repair_with_settings() {
    // Create a simple shape for testing
    let shape = TopoDsShape::new();
    
    // Create repair tools with custom settings
    let settings = crate::modeling::shape_repair::RepairSettings {
        fix_non_manifold_edges: true,
        fix_degenerate_faces: true,
        fix_self_intersections: true,
        fix_duplicate_vertices: true,
        fix_duplicate_faces: true,
        merge_close_vertices: true,
        vertex_merge_tolerance: 1e-6,
        max_iterations: 10,
        enable_logging: true,
    };
    
    let mut repair_tools = ShapeRepairTools::with_settings(settings);
    let result = repair_tools.repair(&shape);
    
    // Verify repair result
    assert!(matches!(result.status, crate::modeling::shape_repair::RepairStatus::NoRepairNeeded));
    assert!(!repair_tools.get_log().is_empty());
}
