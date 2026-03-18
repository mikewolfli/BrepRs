use crate::geometry::Point;
use crate::modeling::multi_resolution::{LodLevel, MultiResolutionManager, MultiResolutionShape};
use crate::topology::TopoDsShape;

#[test]
fn test_multi_resolution_shape_basic() {
    // Create a simple shape for testing
    let shape = TopoDsShape::new();
    
    // Create multi-resolution shape
    let mut multi_res_shape = MultiResolutionShape::new(shape);
    
    // Build LOD levels
    multi_res_shape.build_lod_levels();
    
    // Test LOD level switching
    multi_res_shape.set_lod_level(LodLevel::High);
    assert!(multi_res_shape.get_current_shape().is_some());
    
    multi_res_shape.set_lod_level(LodLevel::Medium);
    assert!(multi_res_shape.get_current_shape().is_some());
    
    multi_res_shape.set_lod_level(LodLevel::Low);
    assert!(multi_res_shape.get_current_shape().is_some());
    
    // Test distance-based LOD
    let camera_position = Point::new(0.0, 0.0, 0.0);
    multi_res_shape.update_lod_by_distance(5.0); // Should be high detail
    multi_res_shape.update_lod_by_distance(30.0); // Should be medium detail
    multi_res_shape.update_lod_by_distance(150.0); // Should be low detail
}

#[test]
fn test_multi_resolution_manager() {
    // Create a multi-resolution manager
    let mut manager = MultiResolutionManager::new();
    
    // Create test shapes
    let shape1 = TopoDsShape::new();
    let shape2 = TopoDsShape::new();
    
    // Add shapes to manager
    manager.add_shape("shape1", shape1);
    manager.add_shape("shape2", shape2);
    
    // Build LOD levels for all shapes
    manager.build_all_lod_levels();
    
    // Test shape retrieval
    assert!(manager.get_shape("shape1").is_some());
    assert!(manager.get_shape("shape2").is_some());
    assert!(manager.get_shape("non_existent").is_none());
    
    // Test shape removal
    assert!(manager.remove_shape("shape1").is_some());
    assert!(manager.get_shape("shape1").is_none());
    
    // Test distance-based LOD update
    let camera_position = Point::new(0.0, 0.0, 0.0);
    manager.update_all_lod_by_distance(&camera_position);
}

#[test]
fn test_lod_settings() {
    // Create a simple shape for testing
    let shape = TopoDsShape::new();
    
    // Create multi-resolution shape with custom settings
    let settings = crate::modeling::multi_resolution::LodSettings {
        high_detail_threshold: 5.0,
        medium_detail_threshold: 25.0,
        low_detail_threshold: 75.0,
        high_detail_triangle_count: 50000,
        medium_detail_triangle_count: 10000,
        low_detail_triangle_count: 2500,
        decimation_error_threshold: 0.05,
        enable_progressive_mesh: true,
    };
    
    let mut multi_res_shape = MultiResolutionShape::with_settings(shape, settings);
    multi_res_shape.build_lod_levels();
    
    // Test LOD level by distance
    assert!(matches!(multi_res_shape.get_lod_level_by_distance(2.0), LodLevel::High));
    assert!(matches!(multi_res_shape.get_lod_level_by_distance(15.0), LodLevel::Medium));
    assert!(matches!(multi_res_shape.get_lod_level_by_distance(50.0), LodLevel::Low));
    assert!(matches!(multi_res_shape.get_lod_level_by_distance(100.0), LodLevel::Low));
}
