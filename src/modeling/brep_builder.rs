//! BRep Builder for constructing topological shapes
//!
//! This module provides the BrepBuilder class which is the primary tool
//! for constructing and modifying BRep (Boundary Representation) shapes.

use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_compound::TopoDsCompound,
    topods_edge::{Curve, TopoDsEdge},
    topods_face::{Surface, TopoDsFace},
    topods_shape::TopoDsShape,
    topods_shell::TopoDsShell,
    topods_solid::TopoDsSolid,
    topods_vertex::TopoDsVertex,
    topods_wire::TopoDsWire,
};

/// Builder class for constructing BRep shapes
///
/// BrepBuilder provides methods to create and modify topological shapes.
/// It follows the OpenCASCADE BrepBuilder pattern.
#[derive(Debug, Clone)]
pub struct BrepBuilder {}

impl BrepBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {}
    }

    // =========================================================================
    // Vertex Operations
    // =========================================================================

    /// Create a new vertex at the specified point
    #[inline]
    pub fn make_vertex(&self, point: Point) -> Handle<TopoDsVertex> {
        Handle::new(std::sync::Arc::new(TopoDsVertex::new(point)))
    }

    /// Create a new vertex with specified tolerance
    #[inline]
    pub fn make_vertex_with_tolerance(&self, point: Point, tolerance: f64) -> Handle<TopoDsVertex> {
        Handle::new(std::sync::Arc::new(TopoDsVertex::with_tolerance(
            point, tolerance,
        )))
    }

    /// Update vertex geometry
    #[inline]
    pub fn update_vertex(&self, vertex: &mut TopoDsVertex, point: Point) {
        vertex.set_point(point);
    }

    /// Set vertex tolerance
    #[inline]
    pub fn set_vertex_tolerance(&self, vertex: &mut TopoDsVertex, tolerance: f64) {
        vertex.set_tolerance(tolerance);
    }

    // =========================================================================
    // Edge Operations
    // =========================================================================

    /// Create a new edge connecting two vertices
    #[inline]
    pub fn make_edge(
        &self,
        v1: Handle<TopoDsVertex>,
        v2: Handle<TopoDsVertex>,
    ) -> Handle<TopoDsEdge> {
        Handle::new(std::sync::Arc::new(TopoDsEdge::new(v1, v2)))
    }

    /// Create a new edge with a 3D curve
    #[inline]
    pub fn make_edge_with_curve(
        &self,
        v1: Handle<TopoDsVertex>,
        v2: Handle<TopoDsVertex>,
        curve: Handle<dyn Curve>,
    ) -> Handle<TopoDsEdge> {
        Handle::new(std::sync::Arc::new(TopoDsEdge::with_curve(v1, v2, curve)))
    }

    /// Create a degenerate edge (both vertices are the same)
    #[inline]
    pub fn make_degenerate_edge(&self, vertex: Handle<TopoDsVertex>) -> Handle<TopoDsEdge> {
        Handle::new(std::sync::Arc::new(TopoDsEdge::new(vertex.clone(), vertex)))
    }

    /// Update edge vertices
    #[inline]
    pub fn update_edge_vertices(
        &self,
        edge: &mut TopoDsEdge,
        v1: Handle<TopoDsVertex>,
        v2: Handle<TopoDsVertex>,
    ) {
        edge.set_vertices([v1, v2]);
    }

    /// Set edge curve
    #[inline]
    pub fn set_edge_curve(&self, edge: &mut TopoDsEdge, curve: Handle<dyn Curve>) {
        edge.set_curve(Some(curve));
    }

    /// Set edge tolerance
    #[inline]
    pub fn set_edge_tolerance(&self, edge: &mut TopoDsEdge, tolerance: f64) {
        edge.set_tolerance(tolerance);
    }

    // =========================================================================
    // Wire Operations
    // =========================================================================

    /// Create a new empty wire
    #[inline]
    pub fn make_wire(&self) -> Handle<TopoDsWire> {
        Handle::new(std::sync::Arc::new(TopoDsWire::new()))
    }

    /// Create a wire from a single edge
    #[inline]
    pub fn make_wire_from_edge(&self, edge: Handle<TopoDsEdge>) -> Handle<TopoDsWire> {
        let mut wire = TopoDsWire::new();
        wire.add_edge(edge);
        Handle::new(std::sync::Arc::new(wire))
    }

    /// Add an edge to a wire
    #[inline]
    pub fn add_edge_to_wire(&self, wire: &mut TopoDsWire, edge: Handle<TopoDsEdge>) {
        wire.add_edge(edge);
    }

    /// Remove an edge from a wire
    #[inline]
    pub fn remove_edge_from_wire(&self, wire: &mut TopoDsWire, edge: &Handle<TopoDsEdge>) {
        wire.remove_edge(edge);
    }

    /// Clear all edges from a wire
    #[inline]
    pub fn clear_wire(&self, wire: &mut TopoDsWire) {
        wire.clear();
    }

    // =========================================================================
    // Face Operations
    // =========================================================================

    /// Create a new empty face
    #[inline]
    pub fn make_face(&self) -> Handle<TopoDsFace> {
        Handle::new(std::sync::Arc::new(TopoDsFace::new()))
    }

    /// Create a face from a surface
    #[inline]
    pub fn make_face_from_surface(&self, surface: Handle<dyn Surface>) -> Handle<TopoDsFace> {
        let mut face = TopoDsFace::new();
        face.set_surface(surface);
        Handle::new(std::sync::Arc::new(face))
    }

    /// Create a face with outer wire
    #[inline]
    pub fn make_face_with_wire(&self, wire: Handle<TopoDsWire>) -> Handle<TopoDsFace> {
        Handle::new(std::sync::Arc::new(TopoDsFace::with_outer_wire(
            (*wire).clone(),
        )))
    }

    /// Create a face from surface and outer wire
    #[inline]
    pub fn make_face_with_surface_and_wire(
        &self,
        surface: Handle<dyn Surface>,
        wire: Handle<TopoDsWire>,
    ) -> Handle<TopoDsFace> {
        let mut face = TopoDsFace::with_outer_wire((*wire).clone());
        face.set_surface(surface);
        Handle::new(std::sync::Arc::new(face))
    }

    /// Add a wire to a face (as a hole)
    #[inline]
    pub fn add_wire_to_face(&self, face: &mut TopoDsFace, wire: Handle<TopoDsWire>) {
        face.add_wire(wire);
    }

    /// Set the outer wire of a face
    #[inline]
    pub fn set_face_outer_wire(&self, face: &mut TopoDsFace, wire: Handle<TopoDsWire>) {
        face.set_outer_wire(wire);
    }

    /// Set face surface
    #[inline]
    pub fn set_face_surface(&self, face: &mut TopoDsFace, surface: Handle<dyn Surface>) {
        face.set_surface(surface);
    }

    /// Set face tolerance
    #[inline]
    pub fn set_face_tolerance(&self, face: &mut TopoDsFace, tolerance: f64) {
        face.set_tolerance(tolerance);
    }

    // =========================================================================
    // Shell Operations
    // =========================================================================

    /// Create a new empty shell
    #[inline]
    pub fn make_shell(&self) -> Handle<TopoDsShell> {
        Handle::new(std::sync::Arc::new(TopoDsShell::new()))
    }

    /// Create a shell from a single face
    #[inline]
    pub fn make_shell_from_face(&self, face: Handle<TopoDsFace>) -> Handle<TopoDsShell> {
        let mut shell = TopoDsShell::new();
        shell.add_face(face);
        Handle::new(std::sync::Arc::new(shell))
    }

    /// Add a face to a shell
    #[inline]
    pub fn add_face_to_shell(&self, shell: &mut TopoDsShell, face: Handle<TopoDsFace>) {
        shell.add_face(face);
    }

    /// Remove a face from a shell
    #[inline]
    pub fn remove_face_from_shell(&self, shell: &mut TopoDsShell, face: &Handle<TopoDsFace>) {
        shell.remove_face(face);
    }

    /// Clear all faces from a shell
    #[inline]
    pub fn clear_shell(&self, shell: &mut TopoDsShell) {
        shell.clear();
    }

    // =========================================================================
    // Solid Operations
    // =========================================================================

    /// Create a new empty solid
    #[inline]
    pub fn make_solid(&self) -> Handle<TopoDsSolid> {
        Handle::new(std::sync::Arc::new(TopoDsSolid::new()))
    }

    /// Create a solid from a single shell
    #[inline]
    pub fn make_solid_from_shell(&self, shell: Handle<TopoDsShell>) -> Handle<TopoDsSolid> {
        let mut solid = TopoDsSolid::new();
        solid.set_outer_shell(shell);
        Handle::new(std::sync::Arc::new(solid))
    }

    /// Add a shell to a solid (as a cavity)
    #[inline]
    pub fn add_shell_to_solid(&self, solid: &mut TopoDsSolid, shell: Handle<TopoDsShell>) {
        solid.add_cavity_shell(shell);
    }

    /// Set the outer shell of a solid
    #[inline]
    pub fn set_solid_outer_shell(&self, solid: &mut TopoDsSolid, shell: Handle<TopoDsShell>) {
        solid.set_outer_shell(shell);
    }

    /// Clear all shells from a solid
    #[inline]
    pub fn clear_solid(&self, solid: &mut TopoDsSolid) {
        solid.clear();
    }

    // =========================================================================
    // Compound Operations
    // =========================================================================

    /// Create a new empty compound
    #[inline]
    pub fn make_compound(&self) -> Handle<TopoDsCompound> {
        Handle::new(std::sync::Arc::new(TopoDsCompound::new()))
    }

    /// Add a shape to a compound
    #[inline]
    pub fn add_to_compound(&self, compound: &mut TopoDsCompound, shape: Handle<TopoDsShape>) {
        compound.add_component(shape);
    }

    /// Remove a shape from a compound
    #[inline]
    pub fn remove_from_compound(&self, compound: &mut TopoDsCompound, shape: &Handle<TopoDsShape>) {
        compound.remove_component(shape);
    }

    /// Clear all shapes from a compound
    #[inline]
    pub fn clear_compound(&self, compound: &mut TopoDsCompound) {
        compound.clear();
    }

    // =========================================================================
    // Copy Operations
    // =========================================================================

    /// Make a copy of a vertex
    #[inline]
    pub fn copy_vertex(&self, vertex: &TopoDsVertex) -> Handle<TopoDsVertex> {
        Handle::new(std::sync::Arc::new(vertex.clone()))
    }

    /// Make a copy of an edge
    #[inline]
    pub fn copy_edge(&self, edge: &TopoDsEdge) -> Handle<TopoDsEdge> {
        Handle::new(std::sync::Arc::new(edge.clone()))
    }

    /// Make a copy of a wire
    #[inline]
    pub fn copy_wire(&self, wire: &TopoDsWire) -> Handle<TopoDsWire> {
        Handle::new(std::sync::Arc::new(wire.clone()))
    }

    /// Make a copy of a face
    #[inline]
    pub fn copy_face(&self, face: &TopoDsFace) -> Handle<TopoDsFace> {
        Handle::new(std::sync::Arc::new(face.clone()))
    }

    /// Make a copy of a shell
    #[inline]
    pub fn copy_shell(&self, shell: &TopoDsShell) -> Handle<TopoDsShell> {
        Handle::new(std::sync::Arc::new(shell.clone()))
    }

    /// Make a copy of a solid
    #[inline]
    pub fn copy_solid(&self, solid: &TopoDsSolid) -> Handle<TopoDsSolid> {
        Handle::new(std::sync::Arc::new(solid.clone()))
    }

    /// Make a copy of a compound
    #[inline]
    pub fn copy_compound(&self, compound: &TopoDsCompound) -> Handle<TopoDsCompound> {
        Handle::new(std::sync::Arc::new(compound.clone()))
    }
}

impl Default for BrepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_vertex_creation() {
        let builder = BrepBuilder::new();
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = builder.make_vertex(point);

        assert_eq!(vertex.point().x(), 1.0);
        assert_eq!(vertex.point().y(), 2.0);
        assert_eq!(vertex.point().z(), 3.0);
    }

    #[test]
    fn test_builder_vertex_with_tolerance() {
        let builder = BrepBuilder::new();
        let point = Point::new(0.0, 0.0, 0.0);
        let vertex = builder.make_vertex_with_tolerance(point, 0.01);

        assert_eq!(vertex.tolerance(), 0.01);
    }

    #[test]
    fn test_builder_edge_creation() {
        let builder = BrepBuilder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);

        assert_eq!(edge.vertices().len(), 2);
    }

    #[test]
    fn test_builder_wire_creation() {
        let builder = BrepBuilder::new();
        let wire = builder.make_wire();

        assert_eq!(wire.edges().len(), 0);
    }

    #[test]
    fn test_builder_wire_from_edge() {
        let builder = BrepBuilder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);
        let wire = builder.make_wire_from_edge(edge);

        assert_eq!(wire.edges().len(), 1);
    }

    #[test]
    fn test_builder_add_edge_to_wire() {
        let builder = BrepBuilder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let edge = builder.make_edge(v1, v2);

        let mut wire = TopoDsWire::new();
        builder.add_edge_to_wire(&mut wire, edge);

        assert_eq!(wire.edges().len(), 1);
    }

    #[test]
    fn test_builder_face_creation() {
        let builder = BrepBuilder::new();
        let face = builder.make_face();

        assert!(face.surface().is_none());
    }

    #[test]
    fn test_builder_face_with_wire() {
        let builder = BrepBuilder::new();
        let v1 = builder.make_vertex(Point::new(0.0, 0.0, 0.0));
        let v2 = builder.make_vertex(Point::new(1.0, 0.0, 0.0));
        let v3 = builder.make_vertex(Point::new(1.0, 1.0, 0.0));

        let e1 = builder.make_edge(v1.clone(), v2.clone());
        let e2 = builder.make_edge(v2, v3.clone());
        let e3 = builder.make_edge(v3, v1);

        let mut wire = TopoDsWire::new();
        wire.add_edge(e1);
        wire.add_edge(e2);
        wire.add_edge(e3);

        let wire_handle = Handle::new(std::sync::Arc::new(wire));
        let face = builder.make_face_with_wire(wire_handle);

        assert!(face.outer_wire().is_some());
    }

    #[test]
    fn test_builder_shell_creation() {
        let builder = BrepBuilder::new();
        let shell = builder.make_shell();

        assert_eq!(shell.faces().len(), 0);
    }

    #[test]
    fn test_builder_solid_creation() {
        let builder = BrepBuilder::new();
        let solid = builder.make_solid();

        assert!(solid.outer_shell().is_none());
        assert_eq!(solid.cavity_shells().len(), 0);
    }

    #[test]
    fn test_builder_compound_creation() {
        let builder = BrepBuilder::new();
        let compound = builder.make_compound();

        assert_eq!(compound.components().len(), 0);
    }

    #[test]
    fn test_builder_copy_vertex() {
        let builder = BrepBuilder::new();
        let point = Point::new(1.0, 2.0, 3.0);
        let vertex = builder.make_vertex(point);
        let copy = builder.copy_vertex(&vertex);

        assert_eq!(copy.point().x(), vertex.point().x());
        assert_eq!(copy.point().y(), vertex.point().y());
        assert_eq!(copy.point().z(), vertex.point().z());
    }
}
