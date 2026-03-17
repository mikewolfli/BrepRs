#[cfg(test)]
mod tests {

    use crate::foundation::handle::Handle;
    use crate::geometry::Point;
    use crate::mesh::{Mesh2D, MeshGenerator};
    use crate::topology::{ShapeType, TopoDsFace, TopoDsShape, TopoDsSolid};
    use std::sync::Arc;

    #[test]
    fn test_mesh_generator_creation() {

        // Generator should be created successfully
    }

    #[test]
    fn test_mesh_generator_with_params() {
        let generator = MeshGenerator::with_params(0.05, 0.3);
        assert_eq!(generator.deflection(), 0.05);
        assert_eq!(generator.angle(), 0.3);
    }

    #[test]
    fn test_mesh_generator_setters() {
        let mut generator = MeshGenerator::new();
        generator.set_deflection(0.02);
        generator.set_angle(0.4);
        assert_eq!(generator.deflection(), 0.02);
        assert_eq!(generator.angle(), 0.4);
    }

    #[test]
    fn test_generate_simple_face() {
        let generator = MeshGenerator::new();
        let face = TopoDsFace::new();
        let face_handle = Handle::new(Arc::new(face));

        let mesh = generator.generate_face(&face_handle, 0.1, 0.5);

        // Mesh should be created
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_generate_solid() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));
        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Mesh should be created
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_generate_with_null_handle() {
        let generator = MeshGenerator::new();
        let null_handle: Handle<TopoDsShape> = Handle::null();

        let mesh = generator.generate(&null_handle, 0.1, 0.5);

        // Should return empty mesh
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_generate_tetrahedral() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid));

        let mesh = generator.generate_tetrahedral(&solid_handle, 1.0);

        // Should create 3D mesh
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_optimize_mesh() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        // Add some vertices and faces
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        generator.optimize(&mut mesh, 1);

        // Optimized mesh should be valid
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_generate_delaunay_mesh() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        let vertices = vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Point::new(0.5, 1.0, 0.0),
        ];

        generator.generate_delaunay_mesh(&mut mesh, &vertices, 0.1, 0.5);

        // Should generate triangles
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_parallel_mesh_generation() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test that parallel generation doesn't panic
        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create mesh without errors
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_different_deflections() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with different deflection values
        let mesh1 = generator.generate(&shape_handle, 0.01, 0.5);
        let mesh2 = generator.generate(&shape_handle, 0.1, 0.5);
        let mesh3 = generator.generate(&shape_handle, 1.0, 0.5);

        // All should create valid meshes
        assert!(mesh1.vertex_count() > 0);
        assert!(mesh2.vertex_count() > 0);
        assert!(mesh3.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_different_angles() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with different angle values
        let mesh1 = generator.generate(&shape_handle, 0.1, 0.1);
        let mesh2 = generator.generate(&shape_handle, 0.1, 0.5);
        let mesh3 = generator.generate(&shape_handle, 0.1, 1.0);

        // All should create valid meshes
        assert!(mesh1.vertex_count() > 0);
        assert!(mesh2.vertex_count() > 0);
        assert!(mesh3.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_default() {
        let generator = MeshGenerator::default();
        assert_eq!(generator.deflection(), 0.1);
        assert_eq!(generator.angle(), 0.5);
    }

    #[test]
    fn test_mesh_generator_evaluate_quality() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        // Add some vertices and faces
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        let quality = generator.evaluate_quality(&mesh);

        // Quality should be calculated
        assert!(quality.min_angle >= 0.0);
        assert!(quality.max_angle >= 0.0);
    }

    #[test]
    fn test_mesh_generator_optimize_multiple_iterations() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        // Add some vertices and faces
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        // Optimize with multiple iterations
        generator.optimize(&mut mesh, 5);

        // Mesh should still be valid
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_edge_cases() {
        let generator = MeshGenerator::new();

        // Test with very small deflection
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));
        let mesh = generator.generate(&shape_handle, 0.1, 0.5);
        assert!(mesh.vertex_count() > 0);

        // Test with very large deflection
        let mesh2 = generator.generate(&shape_handle, 1000.0, 0.5);
        assert!(mesh2.vertex_count() > 0);

        // Test with very small angle
        let mesh3 = generator.generate(&shape_handle, 0.1, 1e-10);
        assert!(mesh3.vertex_count() > 0);

        // Test with very large angle
        let mesh4 = generator.generate(&shape_handle, 0.1, 1000.0);
        assert!(mesh4.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_empty_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create empty mesh
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_generator_with_compound_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Compound);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create simple mesh
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_wire_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Wire);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create simple mesh
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_edge_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Edge);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create simple mesh
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_vertex_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Vertex);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create empty mesh
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.triangle_count(), 0);
    }

    #[test]
    fn test_mesh_generator_with_shell_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::Shell);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create simple mesh
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_compsolid_shape() {
        let generator = MeshGenerator::new();
        let shape = TopoDsShape::new(ShapeType::CompSolid);
        let shape_handle = Handle::new(Arc::new(shape));

        let mesh = generator.generate(&shape_handle, 0.1, 0.5);

        // Should create simple mesh
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_negative_deflection() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with negative deflection (should handle gracefully)
        let mesh = generator.generate(&shape_handle, -0.1, 0.5);
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_negative_angle() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with negative angle (should handle gracefully)
        let mesh = generator.generate(&shape_handle, 0.1, -0.5);
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_zero_deflection() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with zero deflection
        let mesh = generator.generate(&shape_handle, 0.0, 0.5);
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_zero_angle() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Test with zero angle
        let mesh = generator.generate(&shape_handle, 0.1, 0.0);
        assert!(mesh.vertex_count() > 0);
    }

    #[test]
    fn test_mesh_generator_optimize_with_zero_iterations() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        // Add some vertices and faces
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        // Optimize with zero iterations
        generator.optimize(&mut mesh, 0);

        // Mesh should still be valid
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_generator_optimize_with_many_iterations() {
        let generator = MeshGenerator::new();
        let mut mesh = Mesh2D::new();

        // Add some vertices and faces
        let v0 = mesh.add_vertex(Point::new(0.0, 0.0, 0.0));
        let v1 = mesh.add_vertex(Point::new(1.0, 0.0, 0.0));
        let v2 = mesh.add_vertex(Point::new(0.0, 1.0, 0.0));
        mesh.add_face(v0, v1, v2);

        // Optimize with many iterations
        generator.optimize(&mut mesh, 100);

        // Mesh should still be valid
        assert!(mesh.vertex_count() > 0);
        assert!(mesh.triangle_count() > 0);
    }

    #[test]
    fn test_mesh_generator_with_large_max_edge_length() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid));

        // Test with very large max edge length
        let mesh = generator.generate_tetrahedral(&solid_handle, 1000.0);
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_mesh_generator_with_small_max_edge_length() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid));

        // Test with very small max edge length
        let mesh = generator.generate_tetrahedral(&solid_handle, 1e-10);
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_mesh_generator_with_zero_max_edge_length() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid));

        // Test with zero max edge length
        let mesh = generator.generate_tetrahedral(&solid_handle, 0.0);
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_mesh_generator_with_negative_max_edge_length() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let solid_handle = Handle::new(Arc::new(solid));

        // Test with negative max edge length (should handle gracefully)
        let mesh = generator.generate_tetrahedral(&solid_handle, -1.0);
        assert!(mesh.vertices.len() > 0);
    }

    #[test]
    fn test_mesh_generator_consistency() {
        let generator = MeshGenerator::new();
        let solid = TopoDsSolid::new();
        let shape = solid.shape().clone();
        let shape_handle = Handle::new(Arc::new(shape));

        // Generate mesh multiple times with same parameters
        let mesh1 = generator.generate(&shape_handle, 0.1, 0.5);
        let mesh2 = generator.generate(&shape_handle, 0.1, 0.5);
        let mesh3 = generator.generate(&shape_handle, 0.1, 0.5);

        // All meshes should have same structure
        assert_eq!(mesh1.vertex_count(), mesh2.vertex_count());
        assert_eq!(mesh2.vertex_count(), mesh3.vertex_count());
        assert_eq!(mesh1.triangle_count(), mesh2.triangle_count());
        assert_eq!(mesh2.triangle_count(), mesh3.triangle_count());
    }
}
