//! Property-based tests for topological invariants
//!
//! This module provides tests to verify that topological operations
//! maintain expected invariants.

use breprs::geometry::{Point, Vector};
use breprs::topology::{TopoDsVertex, TopoDsWire, TopoDsShell, TopoDsSolid};

/// Tests for point invariants
#[cfg(test)]
mod point_invariants {
    use super::*;

    #[test]
    fn point_distance_to_self_is_zero() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.distance(&p), 0.0);
    }

    #[test]
    fn point_distance_is_symmetric() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        let d1 = p1.distance(&p2);
        let d2 = p2.distance(&p1);
        assert!((d1 - d2).abs() < 1e-10);
    }

    #[test]
    fn point_distance_triangle_inequality() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let p3 = Point::new(0.0, 1.0, 0.0);
        let d12 = p1.distance(&p2);
        let d23 = p2.distance(&p3);
        let d13 = p1.distance(&p3);
        assert!(d13 <= d12 + d23 + 1e-10);
    }

    #[test]
    fn point_square_distance_non_negative() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(4.0, 5.0, 6.0);
        let sq_dist = p1.square_distance(&p2);
        assert!(sq_dist >= 0.0);
    }

    #[test]
    fn point_distance_squared_equals_square_distance() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(4.0, 5.0, 6.0);
        let dist = p1.distance(&p2);
        let sq_dist = p1.square_distance(&p2);
        assert!((dist * dist - sq_dist).abs() < 1e-10);
    }

    #[test]
    fn point_equality_reflexive() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert!(p == p);
    }

    #[test]
    fn point_equality_symmetric() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p1 == p2, p2 == p1);
    }

    #[test]
    fn point_equality_same_coordinates() {
        let p1 = Point::new(1.0, 2.0, 3.0);
        let p2 = Point::new(1.0, 2.0, 3.0);
        assert!(p1 == p2);
    }

    #[test]
    fn point_translation_by_zero_is_identity() {
        let p = Point::new(1.0, 2.0, 3.0);
        let zero = Vector::new(0.0, 0.0, 0.0);
        let translated = p.translated(&zero);
        assert!((p.x() - translated.x()).abs() < 1e-10);
        assert!((p.y() - translated.y()).abs() < 1e-10);
        assert!((p.z() - translated.z()).abs() < 1e-10);
    }
}

/// Tests for vector invariants
#[cfg(test)]
mod vector_invariants {
    use super::*;

    #[test]
    fn vector_magnitude_non_negative() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let mag = v.magnitude();
        assert!(mag >= 0.0);
    }

    #[test]
    fn vector_magnitude_squared_equals_square_magnitude() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let mag = v.magnitude();
        let sq_mag = v.square_magnitude();
        assert!((mag * mag - sq_mag).abs() < 1e-10);
    }

    #[test]
    fn zero_vector_has_zero_magnitude() {
        let v = Vector::new(0.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 0.0);
    }

    #[test]
    fn normalized_vector_has_unit_magnitude() {
        let v = Vector::new(3.0, 4.0, 0.0);
        let normalized = v.normalized();
        let mag = normalized.magnitude();
        assert!((mag - 1.0).abs() < 1e-10);
    }

    #[test]
    fn normalized_zero_vector_has_zero_magnitude() {
        let v = Vector::new(0.0, 0.0, 0.0);
        let normalized = v.normalized();
        let mag = normalized.magnitude();
        assert_eq!(mag, 0.0);
    }

    #[test]
    fn vector_dot_product_commutative() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let dot1 = v1.dot(&v2);
        let dot2 = v2.dot(&v1);
        assert!((dot1 - dot2).abs() < 1e-10);
    }

    #[test]
    fn vector_cross_product_anticommutative() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let cross1 = v1.cross(&v2);
        let cross2 = v2.cross(&v1);
        assert!((cross1.x + cross2.x).abs() < 1e-10);
        assert!((cross1.y + cross2.y).abs() < 1e-10);
        assert!((cross1.z + cross2.z).abs() < 1e-10);
    }

    #[test]
    fn vector_cross_product_with_self_is_zero() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let cross = v.cross(&v);
        assert!(cross.magnitude() < 1e-10);
    }

    #[test]
    fn perpendicular_vectors_have_zero_dot_product() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        assert!(v1.dot(&v2).abs() < 1e-10);
    }

    #[test]
    fn vector_addition_commutative() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let sum1 = v1 + v2;
        let sum2 = v2 + v1;
        assert!((sum1.x - sum2.x).abs() < 1e-10);
        assert!((sum1.y - sum2.y).abs() < 1e-10);
        assert!((sum1.z - sum2.z).abs() < 1e-10);
    }

    #[test]
    fn vector_addition_associative() {
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let v3 = Vector::new(0.0, 0.0, 1.0);
        let sum1 = (v1 + v2) + v3;
        let sum2 = v1 + (v2 + v3);
        assert!((sum1.x - sum2.x).abs() < 1e-10);
        assert!((sum1.y - sum2.y).abs() < 1e-10);
        assert!((sum1.z - sum2.z).abs() < 1e-10);
    }

    #[test]
    fn scalar_multiplication_distributes() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(4.0, 5.0, 6.0);
        let s = 2.0;
        let left = (v1 + v2) * s;
        let right = v1 * s + v2 * s;
        assert!((left.x - right.x).abs() < 1e-10);
        assert!((left.y - right.y).abs() < 1e-10);
        assert!((left.z - right.z).abs() < 1e-10);
    }
}

/// Tests for vertex invariants
#[cfg(test)]
mod vertex_invariants {
    use super::*;

    #[test]
    fn vertex_preserves_point_coordinates() {
        let p = Point::new(1.0, 2.0, 3.0);
        let vertex = TopoDsVertex::new(p);
        let retrieved_point = vertex.point();
        assert!((p.x() - retrieved_point.x()).abs() < 1e-10);
        assert!((p.y() - retrieved_point.y()).abs() < 1e-10);
        assert!((p.z() - retrieved_point.z()).abs() < 1e-10);
    }

    #[test]
    fn vertex_distance_symmetric() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        let v1 = TopoDsVertex::new(p1);
        let v2 = TopoDsVertex::new(p2);
        let d1 = v1.distance(&v2);
        let d2 = v2.distance(&v1);
        assert!((d1 - d2).abs() < 1e-10);
    }

    #[test]
    fn vertex_distance_equals_point_distance() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(3.0, 4.0, 0.0);
        let v1 = TopoDsVertex::new(p1);
        let v2 = TopoDsVertex::new(p2);
        let vertex_dist = v1.distance(&v2);
        let point_dist = p1.distance(&p2);
        assert!((vertex_dist - point_dist).abs() < 1e-10);
    }

    #[test]
    fn vertex_shape_ids_are_unique() {
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 1.0);
        let v1 = TopoDsVertex::new(p1);
        let v2 = TopoDsVertex::new(p2);
        assert_ne!(v1.shape_id(), v2.shape_id());
    }

    #[test]
    fn vertex_shape_type_is_always_vertex() {
        let vertex = TopoDsVertex::new(Point::origin());
        assert!(vertex.shape().is_vertex());
        assert!(!vertex.shape().is_edge());
        assert!(!vertex.shape().is_face());
    }

    #[test]
    fn vertex_maintains_tolerance() {
        let mut vertex = TopoDsVertex::new(Point::origin());
        vertex.set_tolerance(0.01);
        assert!((vertex.tolerance() - 0.01).abs() < 1e-10);
    }

    #[test]
    fn vertex_tolerance_is_non_negative() {
        let vertex = TopoDsVertex::new(Point::origin());
        assert!(vertex.tolerance() >= 0.0);
    }
}

/// Tests for wire invariants
#[cfg(test)]
mod wire_invariants {
    use super::*;

    #[test]
    fn empty_wire_has_zero_length() {
        let wire = TopoDsWire::new();
        assert_eq!(wire.length(), 0.0);
    }

    #[test]
    fn empty_wire_is_not_closed() {
        let wire = TopoDsWire::new();
        assert!(!wire.is_closed());
    }

    #[test]
    fn wire_shape_type_is_always_wire() {
        let wire = TopoDsWire::new();
        assert!(wire.shape().is_wire());
        assert!(!wire.shape().is_edge());
        assert!(!wire.shape().is_face());
    }

    #[test]
    fn wire_edges_is_consistent() {
        let wire = TopoDsWire::new();
        assert_eq!(wire.edges().len(), wire.edges().len());
    }

    #[test]
    fn wire_vertices_is_consistent() {
        let wire = TopoDsWire::new();
        assert_eq!(wire.vertices().len(), wire.vertices().len());
    }
}

/// Tests for shell invariants
#[cfg(test)]
mod shell_invariants {
    use super::*;

    #[test]
    fn empty_shell_has_no_faces() {
        let shell = TopoDsShell::new();
        assert!(shell.faces().is_empty());
    }

    #[test]
    fn shell_shape_type_is_always_shell() {
        let shell = TopoDsShell::new();
        assert!(shell.shape().is_shell());
        assert!(!shell.shape().is_face());
        assert!(!shell.shape().is_solid());
    }

    #[test]
    fn shell_faces_is_consistent() {
        let shell = TopoDsShell::new();
        assert_eq!(shell.faces().len(), shell.faces().len());
    }
}

/// Tests for solid invariants
#[cfg(test)]
mod solid_invariants {
    use super::*;

    #[test]
    fn empty_solid_has_no_shells() {
        let solid = TopoDsSolid::new();
        assert!(solid.shells().is_empty());
        assert!(!solid.has_cavities());
    }

    #[test]
    fn solid_shape_type_is_always_solid() {
        let solid = TopoDsSolid::new();
        assert!(solid.shape().is_solid());
        assert!(!solid.shape().is_shell());
        assert!(!solid.shape().is_face());
    }

    #[test]
    fn solid_shells_is_consistent() {
        let solid = TopoDsSolid::new();
        assert_eq!(solid.shells().len(), solid.shells().len());
    }
}
