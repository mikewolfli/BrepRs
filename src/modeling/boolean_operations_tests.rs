#[cfg(test)]
mod tests {
    use super::*;
    use crate::foundation::handle::Handle;
    use crate::geometry::{Point, Direction, Plane};
    use crate::topology::{ShapeType, TopoDsSolid, TopoDsFace, TopoDsShape};
    use std::sync::Arc;

    #[test]
    fn test_boolean_operations_creation() {
        let boolean_ops = BooleanOperations::new();
        assert!(!boolean_ops.is_none());
    }

    #[test]
    fn test_boolean_operations_default() {
        let boolean_ops = BooleanOperations::default();
        assert!(!boolean_ops.is_none());
    }

    #[test]
    fn test_fuse() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Fuse solids
        let result = boolean_ops.fuse(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_cut() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Cut solid2 from solid1
        let result = boolean_ops.cut(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_common() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Find common of solids
        let result = boolean_ops.common(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_section() {
        let boolean_ops = BooleanOperations::new();

        // Create a simple solid
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));

        // Create a plane
        let point = Point::new(0.0, 0.0, 0.0);
        let normal = Direction::new(0.0, 0.0, 1.0);
        let plane = Plane::new(point, normal);

        // Section solid with plane
        let result = boolean_ops.section(&solid_handle, &plane);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();
        let solid3 = TopoDsSolid::new();

        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::new(Arc::new(solid2.shape().clone())),
            Handle::new(Arc::new(solid3.shape().clone())),
        ];

        // Fuse all solids
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all_empty() {
        let boolean_ops = BooleanOperations::new();

        // Fuse empty list
        let shapes: Vec<Handle<TopoDsShape>> = vec![];
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be an empty compound
        assert!(result.shape().is_compound());
        assert_eq!(result.num_components(), 0);
    }

    #[test]
    fn test_fuse_all_single() {
        let boolean_ops = BooleanOperations::new();

        // Fuse single shape
        let solid = TopoDsSolid::new();
        let shapes = vec![Handle::new(Arc::new(solid.shape().clone()))];
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound with one component
        assert!(result.shape().is_compound());
        assert_eq!(result.num_components(), 1);
    }

    #[test]
    fn test_fuse_all_parallel() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple solids for parallel processing
        let mut shapes = Vec::new();
        for i in 0..10 {
            let solid = TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all solids (should use parallel processing if rayon is enabled)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Create shapes including null handles
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();
        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::null(),
            Handle::new(Arc::new(solid2.shape().clone())),
        ];

        // Fuse all solids (should handle null handles gracefully)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_all_consistency() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();
        let solid3 = TopoDsSolid::new();

        let shapes = vec![
            Handle::new(Arc::new(solid1.shape().clone())),
            Handle::new(Arc::new(solid2.shape().clone())),
            Handle::new(Arc::new(solid3.shape().clone())),
        ];

        // Fuse all solids multiple times
        let result1 = boolean_ops.fuse_all(&shapes);
        let result2 = boolean_ops.fuse_all(&shapes);
        let result3 = boolean_ops.fuse_all(&shapes);

        // Results should be consistent
        assert_eq!(result1.num_components(), result2.num_components());
        assert_eq!(result2.num_components(), result3.num_components());
    }

    #[test]
    fn test_fuse_all_with_many_shapes() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for stress testing
        let mut shapes = Vec::new();
        for i in 0..100 {
            let solid = TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all shapes (should handle large number of shapes)
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_section_with_different_planes() {
        let boolean_ops = BooleanOperations::new();

        // Create a simple solid
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));

        // Create different planes
        let plane1 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 1.0),
        );
        let plane2 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(1.0, 0.0, 0.0),
        );
        let plane3 = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 1.0, 0.0),
        );

        // Section solid with different planes
        let result1 = boolean_ops.section(&solid_handle, &plane1);
        let result2 = boolean_ops.section(&solid_handle, &plane2);
        let result3 = boolean_ops.section(&solid_handle, &plane3);

        // All results should be compounds
        assert!(result1.shape().is_compound());
        assert!(result2.shape().is_compound());
        assert!(result3.shape().is_compound());
    }

    #[test]
    fn test_section_with_null_handle() {
        let boolean_ops = BooleanOperations::new();

        // Create a plane
        let plane = Plane::new(
            Point::new(0.0, 0.0, 0.0),
            Direction::new(0.0, 0.0, 1.0),
        );

        // Section null handle with plane (should handle gracefully)
        let null_handle: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.section(&null_handle, &plane);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_fuse_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Fuse null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.fuse(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_cut_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Cut null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.cut(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_common_with_null_handles() {
        let boolean_ops = BooleanOperations::new();

        // Find common of null handles (should handle gracefully)
        let null_handle1: Handle<TopoDsShape> = Handle::null();
        let null_handle2: Handle<TopoDsShape> = Handle::null();
        let result = boolean_ops.common(&null_handle1, &null_handle2);

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_with_different_shape_types() {
        let boolean_ops = BooleanOperations::new();

        // Create different shape types
        let solid = TopoDsSolid::new();
        let face = TopoDsFace::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));
        let face_handle = Handle::new(Arc::new(face.shape().clone()));

        // Test operations with different shape types
        let fuse_result = boolean_ops.fuse(&solid_handle, &face_handle);
        let cut_result = boolean_ops.cut(&solid_handle, &face_handle);
        let common_result = boolean_ops.common(&solid_handle, &face_handle);

        // All results should be compounds
        assert!(fuse_result.shape().is_compound());
        assert!(cut_result.shape().is_compound());
        assert!(common_result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_with_same_handles() {
        let boolean_ops = BooleanOperations::new();

        // Create a single handle
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));

        // Test operations with same handle
        let fuse_result = boolean_ops.fuse(&solid_handle, &solid_handle);
        let cut_result = boolean_ops.cut(&solid_handle, &solid_handle);
        let common_result = boolean_ops.common(&solid_handle, &solid_handle);

        // All results should be compounds
        assert!(fuse_result.shape().is_compound());
        assert!(cut_result.shape().is_compound());
        assert!(common_result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_performance() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for performance testing
        let mut shapes = Vec::new();
        for i in 0..50 {
            let solid = TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Measure performance of fuse_all
        let start = std::time::Instant::now();
        let result = boolean_ops.fuse_all(&shapes);
        let duration = start.elapsed();

        // Result should be a compound
        assert!(result.shape().is_compound());
        
        // Performance should be reasonable (less than 1 second for 50 shapes)
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_boolean_operations_thread_safety() {
        use std::thread;

        let boolean_ops = BooleanOperations::new();

        // Create shapes for thread safety testing
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();
        let solid3 = TopoDsSolid::new();
        let solid4 = TopoDsSolid::new();

        let handle1 = Handle::new(Arc::new(solid1.shape().clone()));
        let handle2 = Handle::new(Arc::new(solid2.shape().clone()));
        let handle3 = Handle::new(Arc::new(solid3.shape().clone()));
        let handle4 = Handle::new(Arc::new(solid4.shape().clone()));

        // Spawn multiple threads performing operations
        let handle1_clone = handle1.clone();
        let handle2_clone = handle2.clone();
        let handle3_clone = handle3.clone();
        let handle4_clone = handle4.clone();

        let thread1 = thread::spawn(move || {
            let ops = BooleanOperations::new();
            ops.fuse(&handle1_clone, &handle2_clone)
        });

        let thread2 = thread::spawn(move || {
            let ops = BooleanOperations::new();
            ops.cut(&handle3_clone, &handle4_clone)
        });

        // Wait for threads to complete
        let result1 = thread1.join().unwrap();
        let result2 = thread2.join().unwrap();

        // Results should be compounds
        assert!(result1.shape().is_compound());
        assert!(result2.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_memory_usage() {
        let boolean_ops = BooleanOperations::new();

        // Create many shapes for memory testing
        let mut shapes = Vec::new();
        for i in 0..100 {
            let solid = TopoDsSolid::new();
            shapes.push(Handle::new(Arc::new(solid.shape().clone())));
        }

        // Fuse all shapes
        let result = boolean_ops.fuse_all(&shapes);

        // Result should be a compound
        assert!(result.shape().is_compound());
        
        // Memory usage should be reasonable (number of components should be reasonable)
        assert!(result.num_components() <= 100);
    }
}
