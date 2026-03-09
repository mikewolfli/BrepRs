//! Unit tests for BrepRs modeling module

use breprs::modeling::*;
use breprs::topology::*;

#[test]
fn test_primitive_creation() {
    // Test box creation
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    assert_eq!(box_shape.shape_type(), ShapeType::Solid);
    
    // Test sphere creation
    let sphere_shape = Primitives::create_sphere(1.0);
    assert_eq!(sphere_shape.shape_type(), ShapeType::Solid);
    
    // Test cylinder creation
    let cylinder_shape = Primitives::create_cylinder(1.0, 2.0);
    assert_eq!(cylinder_shape.shape_type(), ShapeType::Solid);
}

#[test]
fn test_boolean_operations() {
    // Test boolean operations
    let box1 = Primitives::create_box(1.0, 1.0, 1.0);
    let box2 = Primitives::create_box(1.0, 1.0, 1.0);
    
    let boolean = BooleanOperations::new();
    
    // Test fuse operation
    let fused = boolean.fuse(&box1, &box2);
    assert_eq!(fused.shape_type(), ShapeType::Solid);
    
    // Test cut operation
    let cut = boolean.cut(&box1, &box2);
    assert_eq!(cut.shape_type(), ShapeType::Solid);
    
    // Test common operation
    let common = boolean.common(&box1, &box2);
    assert_eq!(common.shape_type(), ShapeType::Solid);
}

#[test]
fn test_fillet_chamfer() {
    // Test fillet and chamfer operations
    let box_shape = Primitives::create_box(1.0, 1.0, 1.0);
    let fillet_chamfer = FilletChamfer::new();
    
    // Test edge filleting
    let filleted = fillet_chamfer.fillet(&box_shape, 0.1);
    assert_eq!(filleted.shape_type(), ShapeType::Solid);
    
    // Test face chamfering
    let chamfered = fillet_chamfer.chamfer(&box_shape, 0.1);
    assert_eq!(chamfered.shape_type(), ShapeType::Solid);
}