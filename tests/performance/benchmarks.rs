//! Performance benchmarks for BrepRs

use breprs::topology::*;
use breprs::modeling::*;
use breprs::geometry::*;
use std::time::Instant;

#[test]
fn benchmark_shape_creation() {
    // Benchmark shape creation performance
    let start = Instant::now();
    
    for _ in 0..1000 {
        let _ = TopoDsVertex::new();
        let _ = TopoDsEdge::new();
        let _ = TopoDsFace::new();
    }
    
    let duration = start.elapsed();
    println!("Shape creation: {:?}", duration);
    
    // Ensure the test passes
    assert!(duration.as_millis() < 1000);
}

#[test]
fn benchmark_boolean_operations() {
    // Benchmark boolean operations performance
    let box1 = Primitives::create_box(1.0, 1.0, 1.0);
    let box2 = Primitives::create_box(1.0, 1.0, 1.0);
    let boolean = BooleanOperations::new();
    
    let start = Instant::now();
    
    for _ in 0..100 {
        let _ = boolean.fuse(&box1, &box2);
    }
    
    let duration = start.elapsed();
    println!("Boolean operations: {:?}", duration);
    
    // Ensure the test passes
    assert!(duration.as_millis() < 5000);
}

#[test]
fn benchmark_point_operations() {
    // Benchmark point operations performance
    let point1 = Point::new(1.0, 2.0, 3.0);
    let point2 = Point::new(4.0, 5.0, 6.0);
    
    let start = Instant::now();
    
    for _ in 0..1000000 {
        let _ = point1.distance(&point2);
    }
    
    let duration = start.elapsed();
    println!("Point operations: {:?}", duration);
    
    // Ensure the test passes
    assert!(duration.as_millis() < 1000);
}