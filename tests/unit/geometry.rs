//! Unit tests for BrepRs geometry module

use breprs::geometry::*;

#[test]
fn test_point() {
    // Test point creation
    let point = Point::new(1.0, 2.0, 3.0);
    assert_eq!(point.x(), 1.0);
    assert_eq!(point.y(), 2.0);
    assert_eq!(point.z(), 3.0);
    
    // Test point operations
    let point2 = Point::new(4.0, 5.0, 6.0);
    let distance = point.distance(&point2);
    assert!((distance - 5.196152).abs() < 1e-6);
}

#[test]
fn test_vector() {
    // Test vector creation
    let vector = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(vector.x(), 1.0);
    assert_eq!(vector.y(), 2.0);
    assert_eq!(vector.z(), 3.0);
    
    // Test vector operations
    let length = vector.length();
    assert!((length - 3.741657).abs() < 1e-6);
    
    let normalized = vector.normalized();
    assert!((normalized.length() - 1.0).abs() < 1e-6);
}

#[test]
fn test_transform() {
    // Test transform creation
    let transform = Transform::identity();
    
    // Test point transformation
    let point = Point::new(1.0, 2.0, 3.0);
    let transformed = transform.transform_point(&point);
    assert_eq!(transformed, point);
}

#[test]
fn test_plane() {
    // Test plane creation
    let origin = Point::new(0.0, 0.0, 0.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let plane = Plane::new(origin, normal);
    
    // Test plane properties
    assert_eq!(plane.origin(), origin);
    assert_eq!(plane.normal(), normal);
}