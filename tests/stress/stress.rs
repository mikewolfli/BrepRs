//! Stress tests for BrepRs

use breprs::topology::*;
use breprs::modeling::*;
use breprs::geometry::*;
use std::collections::VecDeque;

#[test]
fn stress_test_shape_creation() {
    // Stress test: create many shapes
    let mut shapes = VecDeque::new();
    
    for i in 0..10000 {
        let shape = match i % 4 {
            0 => TopoDsVertex::new(),
            1 => TopoDsEdge::new(),
            2 => TopoDsFace::new(),
            _ => TopoDsSolid::new(),
        };
        shapes.push_back(shape);
    }
    
    // Verify all shapes were created
    assert_eq!(shapes.len(), 10000);
}

#[test]
fn stress_test_boolean_operations() {
    // Stress test: perform many boolean operations
    let boolean = BooleanOperations::new();
    let mut results = Vec::new();
    
    for i in 0..500 {
        let box1 = Primitives::create_box(1.0, 1.0, 1.0);
        let box2 = Primitives::create_box(1.0, 1.0, 1.0);
        
        let result = boolean.fuse(&box1, &box2);
        results.push(result);
    }
    
    // Verify all operations completed
    assert_eq!(results.len(), 500);
}

#[test]
fn stress_test_memory_usage() {
    // Stress test: memory usage
    let mut points = Vec::new();
    
    for i in 0..1000000 {
        let x = (i as f64) / 1000.0;
        let y = (i as f64) / 1000.0;
        let z = (i as f64) / 1000.0;
        points.push(Point::new(x, y, z));
    }
    
    // Verify all points were created
    assert_eq!(points.len(), 1000000);
}