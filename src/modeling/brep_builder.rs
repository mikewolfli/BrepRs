//! BRep Builder for constructing topological shapes
//!
//! This module provides the BRep_Builder class which is the primary tool
//! for constructing and modifying BRep (Boundary Representation) shapes.

use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_vertex::TopoDS_Vertex,
    topods_edge::{TopoDS_Edge, Curve},
    topods_wire::TopoDS_Wire,
    topods_face::{TopoDS_Face, Surface},
    topods_shell::TopoDS_Shell,
    topods_solid::TopoDS_Solid,
    topods_compound::TopoDS_Compound,
    topods_shape::TopoDS_Shape,
};

/// Builder class for constructing BRep shapes
///
/// BRep_Builder provides methods to create and modify topological shapes.
/// It follows the OpenCASCADE BRep_Builder pattern.
pub struct BRep_Builder;

impl BRep_Builder {
    /// Create a new BRep_Builder instance
    pub fn new() -> Self {
        Self
    }

    // =========================================================================
    // Vertex Operations
    // =========================================================================

    /// Create a new vertex at the specified point
    pub fn make_vertex(&self, point: Point) -> Handle<TopoDS_Vertex> {
        Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(point)))
    }

    /// Create a new vertex with specified tolerance
    pub fn make_vertex_with_tolerance(&self, point: Point, tolerance: f64) -> Handle<TopoDS_Vertex> {
        Handle::new(std::sync::Arc::new(TopoDS_Vertex::with_tolerance(point, tolerance)))
    }

    /// Update vertex geometry
    pub fn update_vertex(&self, vertex: &mut TopoDS_Vertex, point: Point) {
        vertex.set_point(point);
    }

    /// Set vertex tolerance
    pub fn set_vertex_tolerance(&self, vertex: &mut TopoDS_Vertex, tolerance: f64) {
        vertex.set_tolerance(tolerance);
    }

    // =========================================================================
    // Edge Operations
    // =========================================================================

    /// Create a new edge connecting two vertices
    pub fn make_edge(&self, v1: Handle<TopoDS_Vertex>, v2: Handle<TopoDS_Vertex>) -> Handle<TopoDS_Edge> {
        Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1, v2)))
    }

    /// Create a new edge with a 3D curve
    pub fn make_edge_with_curve(
        &self,
        v1: Handle<TopoDS_Vertex>,
        v2: Handle<TopoDS_Vertex>,
        curve: Handle<dyn Curve>,
    ) -> Handle<TopoDS_Edge> {
        Handle::new(std::sync::Arc::new(TopoDS_Edge::with_curve(v1, v2, curve)))
    }

    /// Create a degenerate edge (both vertices are the same)
    pub fn make_degenerate_edge(&self, vertex: Handle<TopoDS_Vertex>) -> Handle<TopoDS_Edge> {
        Handle::new(std::sync::Arc::new(TopoDS_Edge::new(
            vertex.clone(),
            vertex,
        )))
    }

    /// Update edge vertices
    pub fn update_edge_vertices(
        &self,
        edge: &mut TopoDS_Edge,
        v1: Handle<TopoDS_Vertex>,
        v2: Handle<TopoDS_Vertex>,
    ) {
        edge.set_vertices([v1, v2]);
    }

    /// Set edge curve
    pub fn set_edge_curve(&self, edge: &mut TopoDS_Edge, curve: Handle<dyn Curve>) {
        edge.set_curve(Some(curve));
    }

    /// Set edge tolerance
    pub fn set_edge_tolerance(&self, edge: &mut TopoDS_Edge, tolerance: f64) {
        edge.set_tolerance(tolerance);
    }

    // =========================================================================
    // Wire Operations
    // =========================================================================

    /// Create a new empty wire
    pub fn make_wire(&self) -> Handle<TopoDS_Wire> {
        Handle::new(std::sync::Arc::new(TopoDS_Wire::new()))
    }

    /// Create a wire from a single edge
    pub fn make_wire_from_edge(&self, edge: Handle<TopoDS_Edge>) -> Handle<TopoDS_Wire> {
        let mut wire = TopoDS_Wire::new();
        wire.add_edge(edge);
        Handle::new(std::sync::Arc::new(wire))
    }

    /// Add an edge to a wire
    pub fn add_edge_to_wire(&self, wire: &mut TopoDS_Wire, edge: Handle<TopoDS_Edge>) {
        wire.add_edge(edge);
    }

    /// Remove an edge from a wire
    pub fn remove_edge_from_wire(&self, wire: &mut TopoDS_Wire, edge: &Handle<TopoDS_Edge>) {
        wire.remove_edge(edge);
    }

    /// Clear all edges from a wire
    pub fn clear_wire(&self, wire: &mut TopoDS_Wire) {
        wire.clear();
    }

    // =========================================================================
    // Face Operations
    // =========================================================================

    /// Create a new empty face
    pub fn make_face(&self) -> Handle<TopoDS_Face> {
        Handle::new(std::sync::Arc::new(TopoDS_Face::new()))
    }

    /// Create a face from a surface
    pub fn make_face_from_surface(&self, surface: Handle<dyn Surface>) -> Handle<TopoDS_Face> {
        let mut face = TopoDS_Face::new();
        face.set_surface(surface);
        Handle::new(std::sync::Arc::new(face))
    }

    /// Create a face with outer wire
    pub fn make_face_with_wire(&self, wire: Handle<TopoDS_Wire>) -> Handle<TopoDS_Face> {
        Handle::new(std::sync::Arc::new(TopoDS_Face::with_outer_wire(
            (*wire).clone(),
        )))
    }

    /// Create a face from surface and outer wire
    pub fn make_face_with_surface_and_wire(
        &self,
        surface: Handle<dyn Surface>,
        wire: Handle<TopoDS_Wire>,
    ) -> Handle<TopoDS_Face> {
        let mut face = TopoDS_Face::with_outer_wire((*wire).clone());
        face.set_surface(surface);
        Handle::new(std::sync::Arc::new(face))
    }

    /// Add a wire to a face (as a hole)
    pub fn add_wire_to_face(&self, face: &mut TopoDS_Face, wire: Handle<TopoDS_Wire>) {
        face.add_wire(wire);
    }

    /// Set the outer wire of a face
    pub fn set_face_outer_wire(&self, face: &mut TopoDS_Face, wire: Handle<TopoDS_Wire>) {
        face.set_outer_wire(wire);
    }

    /// Set face surface
    pub fn set_face_surface(&self, face: &mut TopoDS_Face, surface: Handle<dyn Surface>) {
        face.set_surface(surface);
    }

    /// Set face tolerance
    pub fn set_face_tolerance(&self, face: &mut TopoDS_Face, tolerance: f64) {
        face.set_tolerance(tolerance);
    }

    // =========================================================================
    // Shell Operations
    // =========================================================================

    /// Create a new empty shell
    pub fn make_shell(&self) -> Handle<TopoDS_Shell> {
        Handle::new(std::sync::Arc::new(TopoDS_Shell::new()))
    }

    /// Create a shell from a single face
    pub fn make_shell_from_face(&self, face: Handle<TopoDS_Face>) -> Handle<TopoDS_Shell> {
        let mut shell = TopoDS_Shell::new();
        shell.add_face(face);
        Handle::new(std::sync::Arc::new(shell))
    }

    /// Add a face to a shell
    pub fn add_face_to_shell(&self, shell: &mut TopoDS_Shell, face: Handle<TopoDS_Face>) {
        shell.add_face(face);
    }

    /// Remove a face from a shell
    pub fn remove_face_from_shell(&self, shell: &mut TopoDS_Shell, face: &Handle<TopoDS_Face>) {
        shell.remove_face(face);
    }

    /// Clear all faces from a shell
    pub fn clear_shell(&self, shell: &mut TopoDS_Shell) {
        shell.clear();
    }

    // =========================================================================
    // Solid Operations
    // =========================================================================

    /// Create a new empty solid
    pub fn make_solid(&self) -> Handle<TopoDS_Solid> {
        Handle::new(std::sync::Arc::new(TopoDS_Solid::new()))
    }

    /// Create a solid from a single shell
    pub fn make_solid_from_shell(&self, shell: Handle<TopoDS_Shell>) -> Handle<TopoDS_Solid> {
        let mut solid = TopoDS_Solid::new();
        solid.set_outer_shell(shell);
        Handle::new(std::sync::Arc::new(solid))
    }

    /// Add a shell to a solid (as a cavity)
    pub fn add_shell_to_solid(&self, solid: &mut TopoDS_Solid, shell: Handle<TopoDS_Shell>) {
        solid.add_cavity_shell(shell);
    }

    /// Set the outer shell of a solid
    pub fn set_solid_outer_shell(&self, solid: &mut TopoDS_Solid, shell: Handle<TopoDS_Shell>) {
        solid.set_outer_shell(shell);
    }

    /// Clear all shells from a solid
    pub fn clear_solid(&self, solid: &mut TopoDS_Solid) {
        solid.clear();
    }

    // =========================================================================
    // Compound Operations
    // =========================================================================

    /// Create a new empty compound
    pub fn make_compound(&self) -> Handle<TopoDS_Compound> {
        Handle::new(std::sync::Arc::new(TopoDS_Compound::new()))
    }

    /// Add a shape to a compound
    pub fn add_to_compound(&self, compound: &mut TopoDS_Compound, shape: Handle<TopoDS_Shape>) {
        compound.add_component(shape);
    }

    /// Remove a shape from a compound
    pub fn remove_from_compound(&self, compound: &mut TopoDS_Compound, shape: &Handle<TopoDS_Shape>) {
        compound.remove_component(shape);
    }

    /// Clear all shapes from a compound
    pub fn clear_compound(&self, compound: &mut TopoDS_Compound) {
        compound.clear();
    }

    // =========================================================================
    // Copy Operations
    // =========================================================================

    /// Make a copy of a vertex
    pub fn copy_vertex(&self, vertex: &TopoDS_Vertex) -> Handle<TopoDS_Vertex> {
        Handle::new(std::sync::Arc::new(vertex.clone()))
    }

    /// Make a copy of an edge
    pub fn copy_edge(&self, edge: &TopoDS_Edge) -> Handle<TopoDS_Edge> {
        Handle::new(std::sync::Arc::new(edge.clone()))
    }

    /// Make a copy of a wire
    pub fn copy_wire(&self, wire: &TopoDS_Wire) -> Handle<TopoDS_Wire> {
        Handle::new(std::sync::Arc::new(wire.clone()))
    }

    /// Make a copy of a face
    pub fn copy_face(&self, face: &TopoDS_Face) -> Handle<TopoDS_Face> {
        Handle::new(std::sync::Arc::new(face.clone()))
    }

    /// Make a copy of a shell
    pub fn copy_shell(&self, shell: &TopoDS_Shell) -> Handle<TopoDS_Shell> {
        Handle::new(std::sync::Arc::new(shell.clone()))
    }

    /// Make a copy of a solid
    pub fn copy_solid(&self, solid: &TopoDS_Solid) -> Handle<TopoDS_Solid> {
        Handle::new(std::sync::Arc::new(solid.clone()))
    }

    /// Make a copy of a compound
    pub fn copy_compound(&self, compound: &TopoDS_Compound) -> Handle<TopoDS_Compound> {
        Handle::new(std::sync::Arc::new(compound.clone()))
    }
}

impl Default for BRep_Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_vertex_creation() {
        let builder = BRep_Builder::new();
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = builder.make_vertex(point);
        
        assert_eq!(vertex.point().x(), 1.0);
        assert_eq!(vertex.point().y(), 2.0);
        assert_eq!(vertex.point().z(), 3.0);
    }

    #[test]
    fn test_builder_vertex_with_tolerance() {
        let builder = BRep_Builder::new();
        let point = Point::new(0.0, 0.0, 0.0);
        let vertex = builder.make_vertex_with_tolerance(point, 0.01);
        
        assert_eq!(vertex.tolerance(), 0.01);
    }

    #[test]
    fn test_builder_edge_creation() {
        let builder = BRep_Builder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);
        
        assert_eq!(edge.vertices().len(), 2);
    }

    #[test]
    fn test_builder_wire_creation() {
        let builder = BRep_Builder::new();
        let wire = builder.make_wire();
        
        assert_eq!(wire.edges().len(), 0);
    }

    #[test]
    fn test_builder_wire_from_edge() {
        let builder = BRep_Builder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);
        let wire = builder.make_wire_from_edge(edge);
        
        assert_eq!(wire.edges().len(), 1);
    }

    #[test]
    fn test_builder_add_edge_to_wire() {
        let builder = BRep_Builder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);
        
        let mut wire = TopoDS_Wire::new();
        builder.add_edge_to_wire(&mut wire, edge);
        
        assert_eq!(wire.edges().len(), 1);
    }

    #[test]
    fn test_builder_face_creation() {
        let builder = BRep_Builder::new();
        let face = builder.make_face();
        
        assert!(face.surface().is_none());
    }

    #[test]
    fn test_builder_face_with_wire() {
        let builder = BRep_Builder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let v3 = builder.make_vertex(Point::new(1.0, 1.0, 0.0));
        
        let e1 = builder.make_edge(v1.clone(), v2.clone());
        let e2 = builder.make_edge(v2, v3.clone());
        let e3 = builder.make_edge(v3, v1);
        
        let mut wire = TopoDS_Wire::new();
        wire.add_edge(e1);
        wire.add_edge(e2);
        wire.add_edge(e3);
        
        let wire_handle = Handle::new(std::sync::Arc::new(wire));
        let face = builder.make_face_with_wire(wire_handle);
        
        assert!(face.outer_wire().is_some());
    }

    #[test]
    fn test_builder_shell_creation() {
        let builder = BRep_Builder::new();
        let shell = builder.make_shell();
        
        assert_eq!(shell.faces().len(), 0);
    }

    #[test]
    fn test_builder_solid_creation() {
        let builder = BRep_Builder::new();
        let solid = builder.make_solid();
        
        assert!(solid.outer_shell().is_none());
        assert_eq!(solid.cavity_shells().len(), 0);
    }

    #[test]
    fn test_builder_compound_creation() {
        let builder = BRep_Builder::new();
        let compound = builder.make_compound();
        
        assert_eq!(compound.components().len(), 0);
    }

    #[test]
    fn test_builder_copy_vertex() {
        let builder = BRep_Builder::new();
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = builder.make_vertex(point);
        let copy = builder.copy_vertex(&vertex);
        
        assert_eq!(copy.point().x(), vertex.point().x());
        assert_eq!(copy.point().y(), vertex.point().y());
        assert_eq!(copy.point().z(), vertex.point().z());
    }
}
