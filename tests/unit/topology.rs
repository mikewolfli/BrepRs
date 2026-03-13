//! Unit tests for BrepRs topology module

use breprs::foundation::handle::Handle;
use breprs::geometry::{Point, Vector};
use breprs::topology::*;
use std::sync::Arc;

#[test]
fn test_shape_creation() {
    // Test vertex creation
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    assert_eq!(vertex.shape_type(), ShapeType::Vertex);
    
    // Test edge creation
    let v1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let v2 = TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
    let edge = TopoDsEdge::new(
        Handle::new(Arc::new(v1)),
        Handle::new(Arc::new(v2))
    );
    assert_eq!(edge.shape_type(), ShapeType::Edge);
    
    // Test wire creation
    let wire = TopoDsWire::new();
    wire.add_edge(Handle::new(Arc::new(edge)));
    assert_eq!(wire.shape_type(), ShapeType::Wire);
    
    // Test face creation
    let face = TopoDsFace::new();
    face.add_wire(Handle::new(Arc::new(wire)));
    assert_eq!(face.shape_type(), ShapeType::Face);
    
    // Test shell creation
    let shell = TopoDsShell::new();
    shell.add_face(Handle::new(Arc::new(face)));
    assert_eq!(shell.shape_type(), ShapeType::Shell);
    
    // Test solid creation
    let solid = TopoDsSolid::new();
    solid.add_shell(Handle::new(Arc::new(shell)));
    assert_eq!(solid.shape_type(), ShapeType::Solid);
    
    // Test compound creation
    let compound = TopoDsCompound::new();
    compound.add_component(Handle::new(Arc::new(solid)));
    assert_eq!(compound.shape_type(), ShapeType::Compound);
}

#[test]
fn test_shape_hierarchy() {
    // Test shape hierarchy
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let shape: &TopoDsShape = &vertex.shape();
    
    assert!(shape.is_vertex());
    assert!(!shape.is_edge());
    assert!(!shape.is_face());
    assert!(!shape.is_solid());
    
    // Test downcasting
    let downcasted_vertex = shape.downcast::<TopoDsVertex>().unwrap();
    assert_eq!(downcasted_vertex.point().x, 0.0);
}

#[test]
fn test_location_and_transformation() {
    // Test location and transformation
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let mut location = TopoDsLocation::new();
    
    // Test translation
    location.translate(&Vector::new(1.0, 1.0, 1.0));
    assert_eq!(location.translation().x, 1.0);
    
    // Test rotation
    location.rotate(&Vector::new(0.0, 0.0, 1.0), 90.0);
    
    // Test scaling
    location.scale(2.0);
    assert_eq!(location.scale_factor(), 2.0);
}

#[test]
fn test_orientation_and_mutability() {
    // Test orientation
    let mut edge = TopoDsEdge::new(
        Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0)))),
        Handle::new(Arc::new(TopoDsVertex::new(Point::new(1.0, 0.0, 0.0))))
    );
    
    assert_eq!(edge.orientation(), 1);
    edge.set_orientation(-1);
    assert_eq!(edge.orientation(), -1);
    
    // Test mutability
    let mut face = TopoDsFace::new();
    assert!(!face.is_mutable());
    face.set_mutable(true);
    assert!(face.is_mutable());
}

#[test]
fn test_shape_identification() {
    // Test shape identification
    let vertex1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let vertex2 = TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
    
    assert_ne!(vertex1.shape_id(), vertex2.shape_id());
    assert_eq!(vertex1.shape().shape_id(), vertex1.shape_id());
}

#[test]
fn test_handle_smart_pointer() {
    // Test Handle smart pointer
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let handle1 = Handle::new(Arc::new(vertex));
    
    // Test cloning
    let handle2 = handle1.clone();
    assert_eq!(handle1.shape_id(), handle2.shape_id());
    
    // Test null handle
    let null_handle: Handle<TopoDsVertex> = Handle::null();
    assert!(null_handle.is_null());
    
    // Test non-null handle
    assert!(!handle1.is_null());
}

#[test]
fn test_shape_traversal_tools() {
    // Test TopExpExplorer
    let compound = TopoDsCompound::new();
    let explorer = TopExpExplorer::new(&compound, ShapeType::Vertex);
    assert!(explorer.is_valid());
    
    // Test TopExpTools
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let tools = TopExpTools::new();
    assert!(tools.is_valid());
    
    // Test TopToolsAnalyzer
    let analyzer = TopToolsAnalyzer::new();
    assert!(analyzer.is_valid());
}

#[test]
fn test_validation_system() {
    // Test validation system
    let validator = TopologyValidator::new();
    
    // Test valid vertex
    let valid_vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let result = validator.validate(&valid_vertex.shape());
    assert!(result.is_valid);
    
    // Test validation errors
    let edge = TopoDsEdge::new(
        Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0)))),
        Handle::new(Arc::new(TopoDsVertex::new(Point::new(0.0, 0.0, 0.0))))
    );
    let result = validator.validate(&edge.shape());
    assert!(!result.is_valid);
    assert!(!result.errors.is_empty());
    
    // Test repair functionality
    let mut invalid_edge = edge;
    let repair_result = validator.repair(&mut invalid_edge.shape());
    assert!(repair_result);
    let repaired_result = validator.validate(&invalid_edge.shape());
    assert!(repaired_result.is_valid);
}

#[test]
fn test_topological_operations() {
    // Test boolean operations
    let boolean_ops = breprs::modeling::BooleanOperations::new();
    
    // Test fillet/chamfer operations
    let fillet_chamfer = breprs::modeling::FilletChamfer::new();
    
    // Test offset operations
    let offset_ops = breprs::modeling::OffsetOperations::new();
    
    // Verify all operation objects can be created
    assert!(true); // Just verifying creation doesn't panic
}

#[test]
fn test_compound_shapes() {
    // Test compound shape operations
    let compound = TopoDsCompound::new();
    
    // Add components
    let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    compound.add_component(Handle::new(Arc::new(vertex)));
    
    // Get components
    let components = compound.components();
    assert_eq!(components.len(), 1);
    
    // Test CompSolid
    let comp_solid = TopoDsCompSolid::new();
    let solid = TopoDsSolid::new();
    comp_solid.add_solid(Handle::new(Arc::new(solid)));
    
    let solids = comp_solid.solids();
    assert_eq!(solids.len(), 1);
}

#[test]
fn test_edge_wire_face_relations() {
    // Test edge-wire-face relationships
    
    // Create vertices
    let v1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let v2 = TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
    let v3 = TopoDsVertex::new(Point::new(1.0, 1.0, 0.0));
    let v4 = TopoDsVertex::new(Point::new(0.0, 1.0, 0.0));
    
    // Create edges
    let e1 = TopoDsEdge::new(
        Handle::new(Arc::new(v1.clone())),
        Handle::new(Arc::new(v2.clone()))
    );
    let e2 = TopoDsEdge::new(
        Handle::new(Arc::new(v2.clone())),
        Handle::new(Arc::new(v3.clone()))
    );
    let e3 = TopoDsEdge::new(
        Handle::new(Arc::new(v3.clone())),
        Handle::new(Arc::new(v4.clone()))
    );
    let e4 = TopoDsEdge::new(
        Handle::new(Arc::new(v4.clone())),
        Handle::new(Arc::new(v1.clone()))
    );
    
    // Create wire
    let wire = TopoDsWire::new();
    wire.add_edge(Handle::new(Arc::new(e1)));
    wire.add_edge(Handle::new(Arc::new(e2)));
    wire.add_edge(Handle::new(Arc::new(e3)));
    wire.add_edge(Handle::new(Arc::new(e4)));
    
    // Create face
    let face = TopoDsFace::new();
    face.add_wire(Handle::new(Arc::new(wire)));
    
    // Verify relationships
    assert_eq!(face.num_wires(), 1);
    
    // Test edge adjacency
    let edges = face.edges();
    assert_eq!(edges.len(), 4);
}

#[test]
fn test_shell_solid_relations() {
    // Test shell-solid relationships
    
    // Create a simple face
    let v1 = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
    let v2 = TopoDsVertex::new(Point::new(1.0, 0.0, 0.0));
    let v3 = TopoDsVertex::new(Point::new(1.0, 1.0, 0.0));
    let v4 = TopoDsVertex::new(Point::new(0.0, 1.0, 0.0));
    
    let e1 = TopoDsEdge::new(
        Handle::new(Arc::new(v1.clone())),
        Handle::new(Arc::new(v2.clone()))
    );
    let e2 = TopoDsEdge::new(
        Handle::new(Arc::new(v2.clone())),
        Handle::new(Arc::new(v3.clone()))
    );
    let e3 = TopoDsEdge::new(
        Handle::new(Arc::new(v3.clone())),
        Handle::new(Arc::new(v4.clone()))
    );
    let e4 = TopoDsEdge::new(
        Handle::new(Arc::new(v4.clone())),
        Handle::new(Arc::new(v1.clone()))
    );
    
    let wire = TopoDsWire::new();
    wire.add_edge(Handle::new(Arc::new(e1)));
    wire.add_edge(Handle::new(Arc::new(e2)));
    wire.add_edge(Handle::new(Arc::new(e3)));
    wire.add_edge(Handle::new(Arc::new(e4)));
    
    let face = TopoDsFace::new();
    face.add_wire(Handle::new(Arc::new(wire)));
    
    // Create shell
    let shell = TopoDsShell::new();
    shell.add_face(Handle::new(Arc::new(face)));
    
    // Create solid
    let solid = TopoDsSolid::new();
    solid.add_shell(Handle::new(Arc::new(shell)));
    
    // Verify relationships
    assert_eq!(solid.num_shells(), 1);
    
    let shells = solid.shells();
    if let Some(shell_ref) = shells[0].get() {
        assert_eq!(shell_ref.num_faces(), 1);
    }
}

#[test]
fn test_lod_support() {
    // Test LOD support in TopExpExplorer
    let compound = TopoDsCompound::new();
    let explorer = TopExpExplorer::new(&compound, ShapeType::Vertex);
    
    // Test LOD levels
    explorer.set_lod_level(1.0);
    assert_eq!(explorer.lod_level(), 1.0);
    
    // Test LOD-based traversal
    let _ = explorer.next(); // Should not panic
}

#[test]
fn test_serialization_support() {
    // Test serialization support
    #[cfg(feature = "serde")] {
        let vertex = TopoDsVertex::new(Point::new(0.0, 0.0, 0.0));
        let json = vertex.to_json().unwrap();
        assert!(!json.is_empty());
    }
}
