//! Integration tests for BrepRs

use breprs::topology::*;
use breprs::modeling::*;
use breprs::geometry::*;
use breprs::visualization::*;

#[test]
fn test_shape_creation_and_transformation() {
    // Create a box
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    
    // Test shape type
    assert_eq!(box_shape.shape_type(), ShapeType::Solid);
    
    // Test shape transformation
    let transform = Transform::translation(Vector::new(1.0, 1.0, 1.0));
    let transformed = box_shape.transform(&transform);
    assert_eq!(transformed.shape_type(), ShapeType::Solid);
}

#[test]
fn test_boolean_operations_integration() {
    // Create two boxes
    let box1 = Primitives::create_box(1.0, 1.0, 1.0);
    let box2 = Primitives::create_box(1.0, 1.0, 1.0);
    
    // Test boolean operations
    let boolean = BooleanOperations::new();
    let fused = boolean.fuse(&box1, &box2);
    
    // Test result is valid
    assert_eq!(fused.shape_type(), ShapeType::Solid);
}

#[test]
fn test_renderer_integration() {
    // Test renderer creation
    let renderer = SoftwareRenderer::new();
    assert!(renderer.is_initialized());
    
    // Test rendering a simple shape
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    let result = renderer.render_shape(&box_shape);
    assert!(result);
}

#[test]
fn test_gpu_integration() {
    // Test GPU memory manager
    let gpu_memory = GpuMemoryManager::new();
    assert!(gpu_memory.is_valid());
    
    // Test GPU buffer creation
    let gpu_buffer = GpuBufferManager::new(Arc::new(gpu_memory));
    assert!(gpu_buffer.is_valid());
    
    // Test texture streaming
    let texture_stream = TextureStreamingSystem::new();
    assert!(texture_stream.is_valid());
}