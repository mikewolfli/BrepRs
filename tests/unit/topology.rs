//! Unit tests for BrepRs topology module

use breprs::topology::*;

#[test]
fn test_shape_creation() {
    // Test vertex creation
    let vertex = TopoDsVertex::new();
    assert_eq!(vertex.shape_type(), ShapeType::Vertex);
    
    // Test edge creation
    let edge = TopoDsEdge::new();
    assert_eq!(edge.shape_type(), ShapeType::Edge);
    
    // Test wire creation
    let wire = TopoDsWire::new();
    assert_eq!(wire.shape_type(), ShapeType::Wire);
    
    // Test face creation
    let face = TopoDsFace::new();
    assert_eq!(face.shape_type(), ShapeType::Face);
    
    // Test shell creation
    let shell = TopoDsShell::new();
    assert_eq!(shell.shape_type(), ShapeType::Shell);
    
    // Test solid creation
    let solid = TopoDsSolid::new();
    assert_eq!(solid.shape_type(), ShapeType::Solid);
    
    // Test compound creation
    let compound = TopoDsCompound::new();
    assert_eq!(compound.shape_type(), ShapeType::Compound);
}

#[test]
fn test_shape_explorer() {
    // Test TopExpExplorer
    let compound = TopoDsCompound::new();
    let explorer = TopExpExplorer::new(&compound, ShapeType::Vertex);
    
    // Verify explorer can be created
    assert!(explorer.is_valid());
}

#[test]
fn test_shape_tools() {
    // Test TopExpTools
    let vertex = TopoDsVertex::new();
    let tools = TopExpTools::new();
    
    // Verify tools can be created
    assert!(tools.is_valid());
}