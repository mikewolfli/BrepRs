#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE Modeling Compatibility Module
//! 
//! Provides OpenCASCADE-compatible naming conventions
//! for modeling operations.

use std::any::Any;

// Re-export topology types
use crate::topology::{topods_edge::TopoDsEdge as TopoDS_Edge, topods_face::TopoDsFace as TopoDS_Face, topods_shape::TopoDsShape as TopoDS_Shape, topods_solid::TopoDsSolid as TopoDS_Solid, topods_vertex::TopoDsVertex as TopoDS_Vertex, topods_wire::TopoDsWire as TopoDS_Wire, topods_shell::TopoDsShell as TopoDS_Shell, topods_compound::TopoDsCompound as TopoDS_Compound};

// Re-export geometry types
use crate::geometry::{point::Point as gp_Pnt, curve_enum::CurveEnum, surface_enum::SurfaceEnum};

// Re-export Handle type
use crate::foundation::handle::Handle;



// BRepFilletAPI_MakeFillet wrapper with OpenCASCADE-compatible method names and parameters
#[allow(non_camel_case_types)]
pub struct BRepFilletAPI_MakeFillet {
    inner: crate::modeling::fillet_chamfer::FilletChamfer,
}

impl BRepFilletAPI_MakeFillet {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::fillet_chamfer::FilletChamfer::new(),
        }
    }

    // Edge Fillet Operations
    pub fn add_edge(&mut self, edge: Handle<TopoDS_Edge>, radius: f64) {
        self.inner.set_radius(radius);
        self.inner.add_edge(edge)
    }

    pub fn add_edges(&mut self, edges: &[Handle<TopoDS_Edge>], radius: f64) {
        self.inner.set_radius(radius);
        self.inner.add_edges(edges);
    }

    pub fn remove_edge(&mut self, edge: &Handle<TopoDS_Edge>) {
        self.inner.remove_edge(edge)
    }

    pub fn clear_edges(&mut self) {
        self.inner.clear_edges()
    }

    pub fn num_edges(&self) -> usize {
        self.inner.num_edges()
    }

    pub fn build_fillet(
        &self,
        shape: &crate::topology::topods_solid::TopoDsSolid,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.apply_fillet(shape)
    }

    // Face Chamfer Operations
    pub fn add_single_face(&mut self, face: Handle<TopoDS_Face>) {
        self.inner.add_face(face)
    }

    pub fn add_faces(&mut self, faces: &[Handle<TopoDS_Face>]) {
        self.inner.add_faces(faces)
    }

    pub fn remove_face(&mut self, face: &Handle<TopoDS_Face>) {
        self.inner.remove_face(face)
    }

    pub fn clear_faces(&mut self) {
        self.inner.clear_faces()
    }

    pub fn num_faces(&self) -> usize {
        self.inner.num_faces()
    }

    pub fn build_chamfer(
        &self,
        shape: &crate::topology::topods_solid::TopoDsSolid,
        faces: &[Handle<TopoDS_Face>],
        distance: f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.chamfer_faces(shape, faces, distance)
    }

    // Utility Methods
    pub fn can_fillet_edge(&self, edge: &Handle<TopoDS_Edge>) -> bool {
        self.inner.can_fillet_edge(edge)
    }

    pub fn can_chamfer_face(&self, face: &Handle<TopoDS_Face>) -> bool {
        self.inner.can_chamfer_face(face)
    }

    pub fn calculate_fillet_surface(&self, edge: &Handle<TopoDS_Edge>, radius: f64) -> Vec<gp_Pnt> {
        self.inner.calculate_fillet_surface(edge, radius)
    }

    pub fn calculate_chamfer_surface(
        &self,
        edge: &Handle<TopoDS_Edge>,
        distance: f64,
    ) -> Vec<gp_Pnt> {
        self.inner.calculate_chamfer_surface(edge, distance)
    }

    pub fn reset(&mut self) {
        self.inner.reset()
    }
}

impl Default for BRepFilletAPI_MakeFillet {
    fn default() -> Self {
        Self::new()
    }
}

// BRepAlgoAPI_BooleanOperation wrapper with OpenCASCADE-compatible method names and parameters
#[allow(non_camel_case_types)]
pub struct BRepAlgoAPI_BooleanOperation {
    inner: crate::modeling::boolean_operations::BooleanOperations,
}

impl BRepAlgoAPI_BooleanOperation {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::boolean_operations::BooleanOperations::new(),
        }
    }

    pub fn is_none(&self) -> bool {
        self.inner.is_none()
    }

    // Fuse Operation
    pub fn fuse(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.fuse(shape1, shape2)
    }

    pub fn fuse_all(&self, shapes: &[Handle<TopoDS_Shape>]) -> TopoDS_Compound {
        self.inner.fuse_all(shapes)
    }

    // Cut Operation
    pub fn cut(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.cut(shape1, shape2)
    }

    // Common Operation
    pub fn common(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.common(shape1, shape2)
    }

    // Section Operation
    pub fn section(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.section(shape1, shape2)
    }

    pub fn section_with_plane(
        &self,
        shape: &Handle<TopoDS_Shape>,
        plane: &crate::geometry::Plane,
    ) -> TopoDS_Compound {
        self.inner.section_with_plane(shape, plane)
    }

    // Helper Methods
    pub fn can_perform_boolean(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> bool {
        self.inner.can_perform_boolean(shape1, shape2)
    }

    pub fn might_intersect(
        &self,
        shape1: &Handle<TopoDS_Shape>,
        shape2: &Handle<TopoDS_Shape>,
    ) -> bool {
        self.inner.might_intersect(shape1, shape2)
    }

    pub fn bounding_boxes_intersect(
        &self,
        bb1: &(crate::geometry::Point, crate::geometry::Point),
        bb2: &(crate::geometry::Point, crate::geometry::Point),
    ) -> bool {
        let (min1, max1) = bb1;
        let (min2, max2) = bb2;

        self.inner
            .bounding_boxes_intersect(&(*min1, *max1), &(*min2, *max2))
    }
}

impl Default for BRepAlgoAPI_BooleanOperation {
    fn default() -> Self {
        Self::new()
    }
}

// BRep_Builder wrapper with OpenCASCADE-compatible method names and parameters
#[allow(non_camel_case_types)]
pub struct BRep_Builder {
    inner: crate::modeling::brep_builder::BrepBuilder,
}

impl BRep_Builder {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::brep_builder::BrepBuilder::new(),
        }
    }

    // Vertex Operations
    pub fn MakeVertex(&self, P: gp_Pnt) -> Handle<TopoDS_Vertex> {
        self.inner.make_vertex(P)
    }

    pub fn MakeVertexWithTolerance(&self, P: gp_Pnt, Tol: f64) -> Handle<TopoDS_Vertex> {
        self.inner.make_vertex_with_tolerance(P, Tol)
    }

    pub fn UpdateVertex(&self, V: &mut TopoDS_Vertex, P: gp_Pnt) {
        self.inner.update_vertex(V, P)
    }

    pub fn SetVertexTolerance(&self, V: &mut TopoDS_Vertex, Tol: f64) {
        self.inner.set_vertex_tolerance(V, Tol)
    }

    // Edge Operations
    pub fn MakeEdge(
        &self,
        V1: Handle<TopoDS_Vertex>,
        V2: Handle<TopoDS_Vertex>,
    ) -> Handle<TopoDS_Edge> {
        self.inner.make_edge(V1, V2)
    }

    /// Create an edge between two vertices with a curve.
    pub fn MakeEdgeWithCurve(
        &self,
        V1: Handle<TopoDS_Vertex>,
        V2: Handle<TopoDS_Vertex>,
        C: Handle<CurveEnum>,
    ) -> Handle<TopoDS_Edge> {
        self.inner.make_edge_with_curve(V1, V2, C)
    }

    pub fn MakeDegenerateEdge(&self, V: Handle<TopoDS_Vertex>) -> Handle<TopoDS_Edge> {
        self.inner.make_degenerate_edge(V)
    }

    pub fn UpdateEdgeVertices(
        &self,
        E: &mut TopoDS_Edge,
        V1: Handle<TopoDS_Vertex>,
        V2: Handle<TopoDS_Vertex>,
    ) {
        self.inner.update_edge_vertices(E, V1, V2)
    }

    /// Set the curve for an edge.
    pub fn SetEdgeCurve(&self, E: &mut TopoDS_Edge, C: Handle<CurveEnum>) {
        self.inner.set_edge_curve(E, C);
    }

    pub fn SetEdgeTolerance(&self, E: &mut TopoDS_Edge, Tol: f64) {
        self.inner.set_edge_tolerance(E, Tol)
    }

    // Wire Operations
    pub fn MakeWire(&self) -> Handle<TopoDS_Wire> {
        self.inner.make_wire()
    }

    pub fn MakeWireFromEdge(&self, E: Handle<TopoDS_Edge>) -> Handle<TopoDS_Wire> {
        self.inner.make_wire_from_edge(E)
    }

    pub fn AddEdge(&self, W: &mut TopoDS_Wire, E: Handle<TopoDS_Edge>) {
        self.inner.add_edge_to_wire(W, E)
    }

    pub fn RemoveEdge(&self, W: &mut TopoDS_Wire, E: &Handle<TopoDS_Edge>) {
        self.inner.remove_edge_from_wire(W, E)
    }

    pub fn ClearWire(&self, W: &mut TopoDS_Wire) {
        self.inner.clear_wire(W)
    }

    // Face Operations
    pub fn MakeFace(&self) -> Handle<TopoDS_Face> {
        self.inner.make_face()
    }

    /// Create a face from a surface.
    pub fn MakeFaceFromSurface(&self, S: Handle<SurfaceEnum>) -> Handle<TopoDS_Face> {
        self.inner.make_face_from_surface(S)
    }

    pub fn MakeFaceWithWire(&self, W: Handle<TopoDS_Wire>) -> Handle<TopoDS_Face> {
        self.inner.make_face_with_wire(W)
    }

    /// Create a face from a surface and a wire.
    pub fn MakeFaceWithSurfaceAndWire(
        &self,
        S: Handle<SurfaceEnum>,
        W: Handle<TopoDS_Wire>,
    ) -> Handle<TopoDS_Face> {
        self.inner.make_face_with_surface_and_wire(S, W)
    }

    pub fn AddWire(&self, F: &mut TopoDS_Face, W: Handle<TopoDS_Wire>) {
        self.inner.add_wire_to_face(F, W)
    }

    pub fn SetFaceOuterWire(&self, F: &mut TopoDS_Face, W: Handle<TopoDS_Wire>) {
        self.inner.set_face_outer_wire(F, W)
    }

    /// Set the surface for a face.
    pub fn SetFaceSurface(&self, F: &mut TopoDS_Face, S: Handle<SurfaceEnum>) {
        self.inner.set_face_surface(F, S);
    }

    pub fn SetFaceTolerance(&self, F: &mut TopoDS_Face, Tol: f64) {
        self.inner.set_face_tolerance(F, Tol)
    }

    // Shell Operations
    pub fn MakeShell(&self) -> Handle<TopoDS_Shell> {
        self.inner.make_shell()
    }

    pub fn MakeShellFromFace(&self, F: Handle<TopoDS_Face>) -> Handle<TopoDS_Shell> {
        self.inner.make_shell_from_face(F)
    }

    pub fn AddFace(&self, S: &mut TopoDS_Shell, F: Handle<TopoDS_Face>) {
        self.inner.add_face_to_shell(S, F)
    }

    pub fn RemoveFace(&self, S: &mut TopoDS_Shell, F: &Handle<TopoDS_Face>) {
        self.inner.remove_face_from_shell(S, F)
    }

    pub fn ClearShell(&self, S: &mut TopoDS_Shell) {
        self.inner.clear_shell(S)
    }

    // Solid Operations
    pub fn MakeSolid(&self) -> Handle<TopoDS_Solid> {
        self.inner.make_solid()
    }

    pub fn make_solid_from_shell(&self, s: Handle<TopoDS_Shell>) -> Handle<TopoDS_Solid> {
        self.inner.make_solid_from_shell(s)
    }

    pub fn add_shell(&self, so: &mut TopoDS_Solid, s: Handle<TopoDS_Shell>) {
        self.inner.add_shell_to_solid(so, s)
    }

    pub fn set_solid_outer_shell(&self, so: &mut TopoDS_Solid, s: Handle<TopoDS_Shell>) {
        self.inner.set_solid_outer_shell(so, s)
    }

    pub fn clear_solid(&self, so: &mut TopoDS_Solid) {
        self.inner.clear_solid(so)
    }

    pub fn make_compound(&self) -> Handle<TopoDS_Compound> {
        self.inner.make_compound()
    }

    pub fn add(&self, c: &mut TopoDS_Compound, s: Handle<TopoDS_Shape>) {
        self.inner.add_to_compound(c, s)
    }

    pub fn remove(&self, c: &mut TopoDS_Compound, s: &Handle<TopoDS_Shape>) {
        self.inner.remove_from_compound(c, s)
    }

    pub fn clear_compound(&self, c: &mut TopoDS_Compound) {
        self.inner.clear_compound(c)
    }

    pub fn copy_vertex(&self, v: &TopoDS_Vertex) -> Handle<TopoDS_Vertex> {
        self.inner.copy_vertex(v)
    }

    pub fn copy_edge(&self, e: &TopoDS_Edge) -> Handle<TopoDS_Edge> {
        self.inner.copy_edge(e)
    }

    pub fn copy_wire(&self, w: &TopoDS_Wire) -> Handle<TopoDS_Wire> {
        self.inner.copy_wire(w)
    }

    pub fn copy_face(&self, f: &TopoDS_Face) -> Handle<TopoDS_Face> {
        self.inner.copy_face(f)
    }

    pub fn copy_shell(&self, s: &TopoDS_Shell) -> Handle<TopoDS_Shell> {
        self.inner.copy_shell(s)
    }

    pub fn copy_solid(&self, so: &TopoDS_Solid) -> Handle<TopoDS_Solid> {
        self.inner.copy_solid(so)
    }

    pub fn copy_compound(&self, c: &TopoDS_Compound) -> Handle<TopoDS_Compound> {
        self.inner.copy_compound(c)
    }
}

impl Default for BRep_Builder {
    fn default() -> Self {
        Self::new()
    }
}

// Primitives module contains functions
pub mod primitives {
    pub use crate::modeling::primitives::*;
}
