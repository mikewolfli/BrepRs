#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_imports,
    unused_variables
)]
//! OpenCASCADE Modeling Compatibility Module
//!
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for modeling operations.

use crate::foundation::handle::Handle;
use crate::geometry::Point as gp_Pnt;
use crate::topology::{
    topods_compound::TopoDsCompound as TopoDS_Compound,
    topods_edge::{Curve, TopoDsEdge as TopoDS_Edge},
    topods_face::{Surface, TopoDsFace as TopoDS_Face},
    topods_shape::TopoDsShape as TopoDS_Shape,
    topods_shell::TopoDsShell as TopoDS_Shell,
    topods_solid::TopoDsSolid as TopoDS_Solid,
    topods_vertex::TopoDsVertex as TopoDS_Vertex,
    topods_wire::TopoDsWire as TopoDS_Wire,
};

// Re-export modeling types with OpenCASCADE naming

// BRepOffsetAPI_MakeOffset wrapper with OpenCASCADE-compatible method names and parameters
pub struct BRepOffsetAPI_MakeOffset {
    inner: crate::modeling::offset_operations::OffsetOperations,
}

/// Join type for offset operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinType {
    /// Sharp join (intersection)
    Sharp,
    /// Round join (fillet)
    Round,
    /// Chamfer join
    Chamfer,
}

/// Intersection type for offset operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntersectionType {
    /// No intersection (separate shells)
    NoIntersection,
    /// Intersection (single shell)
    Intersection,
}

impl BRepOffsetAPI_MakeOffset {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::offset_operations::OffsetOperations::new(),
        }
    }

    pub fn new_with_offset(OffsetDistance: f64) -> Self {
        Self {
            inner: crate::modeling::offset_operations::OffsetOperations::with_offset_distance(
                OffsetDistance,
            ),
        }
    }

    pub fn SetOffsetDistance(&mut self, Distance: f64) {
        self.inner.set_offset_distance(Distance)
    }

    pub fn OffsetDistance(&self) -> f64 {
        self.inner.offset_distance()
    }

    pub fn SetTolerance(&mut self, Tolerance: f64) {
        self.inner.set_tolerance(Tolerance)
    }

    pub fn Tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    pub fn SetJoinType(&mut self, JoinType: JoinType) {
        let inner_join_type = match JoinType {
            JoinType::Sharp => crate::modeling::offset_operations::JoinType::Sharp,
            JoinType::Round => crate::modeling::offset_operations::JoinType::Round,
            JoinType::Chamfer => crate::modeling::offset_operations::JoinType::Chamfer,
        };
        self.inner.set_join_type(inner_join_type)
    }

    pub fn JoinType(&self) -> JoinType {
        match self.inner.join_type() {
            crate::modeling::offset_operations::JoinType::Sharp => JoinType::Sharp,
            crate::modeling::offset_operations::JoinType::Round => JoinType::Round,
            crate::modeling::offset_operations::JoinType::Chamfer => JoinType::Chamfer,
        }
    }

    pub fn SetIntersectionType(&mut self, IntersectionType: IntersectionType) {
        let inner_intersection_type = match IntersectionType {
            IntersectionType::NoIntersection => {
                crate::modeling::offset_operations::IntersectionType::NoIntersection
            }
            IntersectionType::Intersection => {
                crate::modeling::offset_operations::IntersectionType::Intersection
            }
        };
        self.inner.set_intersection_type(inner_intersection_type)
    }

    pub fn IntersectionType(&self) -> IntersectionType {
        match self.inner.intersection_type() {
            crate::modeling::offset_operations::IntersectionType::NoIntersection => {
                IntersectionType::NoIntersection
            }
            crate::modeling::offset_operations::IntersectionType::Intersection => {
                IntersectionType::Intersection
            }
        }
    }

    // Surface Offset Operations
    pub fn OffsetFace(
        &self,
        Face: &crate::topology::topods_face::TopoDsFace,
        Distance: f64,
    ) -> crate::topology::topods_face::TopoDsFace {
        self.inner.offset_face(Face, Distance)
    }

    pub fn OffsetShell(
        &self,
        Shell: &crate::topology::topods_shell::TopoDsShell,
        Distance: f64,
    ) -> crate::topology::topods_shell::TopoDsShell {
        self.inner.offset_shell(Shell, Distance)
    }

    // Thick Solid Creation
    pub fn MakeThickSolid(
        &self,
        Shell: &crate::topology::topods_shell::TopoDsShell,
        Thickness: f64,
        Offset: f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.make_thick_solid(Shell, Thickness, Offset)
    }

    pub fn MakeThickFromFace(
        &self,
        Face: &crate::topology::topods_face::TopoDsFace,
        Thickness: f64,
        Offset: f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.make_thick_from_face(Face, Thickness, Offset)
    }

    // Pipe Creation
    pub fn MakePipe(
        &self,
        Path: &crate::topology::topods_wire::TopoDsWire,
        Profile: &crate::topology::topods_wire::TopoDsWire,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.make_pipe(Path, Profile)
    }

    pub fn MakePipeVariable(
        &self,
        Path: &crate::topology::topods_wire::TopoDsWire,
        Profile: &crate::topology::topods_wire::TopoDsWire,
        RadiusFunc: impl Fn(f64) -> f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.make_pipe_variable(Path, Profile, RadiusFunc)
    }

    // Shell Operations
    pub fn MakeOffsetShell(
        &self,
        Shell: &crate::topology::topods_shell::TopoDsShell,
        Offset: f64,
    ) -> crate::topology::topods_shell::TopoDsShell {
        self.inner.make_offset_shell(Shell, Offset)
    }

    pub fn MakeShellFromSolid(
        &self,
        Solid: &crate::topology::topods_solid::TopoDsSolid,
    ) -> crate::topology::topods_shell::TopoDsShell {
        self.inner.make_shell_from_solid(Solid)
    }

    pub fn MakeShellFromFaces(
        &self,
        Faces: &[Handle<TopoDS_Face>],
    ) -> crate::topology::topods_shell::TopoDsShell {
        self.inner.make_shell_from_faces(Faces)
    }

    // Utility Methods
    pub fn CanOffsetFace(&self, Face: &crate::topology::topods_face::TopoDsFace) -> bool {
        self.inner.can_offset_face(Face)
    }

    pub fn CanOffsetShell(&self, Shell: &crate::topology::topods_shell::TopoDsShell) -> bool {
        self.inner.can_offset_shell(Shell)
    }

    pub fn CalculateOffsetDirection(
        &self,
        Face: &crate::topology::topods_face::TopoDsFace,
    ) -> Option<crate::geometry::Vector> {
        self.inner.calculate_offset_direction(Face)
    }

    pub fn Reset(&mut self) {
        self.inner.reset()
    }
}

impl Default for BRepOffsetAPI_MakeOffset {
    fn default() -> Self {
        Self::new()
    }
}

// BRepFilletAPI_MakeFillet wrapper with OpenCASCADE-compatible method names and parameters
pub struct BRepFilletAPI_MakeFillet {
    inner: crate::modeling::fillet_chamfer::FilletChamfer,
}

impl BRepFilletAPI_MakeFillet {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::fillet_chamfer::FilletChamfer::new(),
        }
    }

    pub fn new_with_radius(Radius: f64) -> Self {
        Self {
            inner: crate::modeling::fillet_chamfer::FilletChamfer::with_radius(Radius),
        }
    }

    pub fn new_with_chamfer(ChamferDistance: f64) -> Self {
        Self {
            inner: crate::modeling::fillet_chamfer::FilletChamfer::with_chamfer_distance(
                ChamferDistance,
            ),
        }
    }

    pub fn SetRadius(&mut self, Radius: f64) {
        self.inner.set_radius(Radius)
    }

    pub fn Radius(&self) -> f64 {
        self.inner.radius()
    }

    pub fn SetChamferDistance(&mut self, Distance: f64) {
        self.inner.set_chamfer_distance(Distance)
    }

    pub fn ChamferDistance(&self) -> f64 {
        self.inner.chamfer_distance()
    }

    // Edge Fillet Operations
    pub fn AddEdge(&mut self, Edge: Handle<TopoDS_Edge>) {
        self.inner.add_edge(Edge)
    }

    pub fn AddEdges(&mut self, Edges: &[Handle<TopoDS_Edge>]) {
        self.inner.add_edges(Edges)
    }

    pub fn Remove(&mut self, Edge: &Handle<TopoDS_Edge>) {
        self.inner.remove_edge(Edge)
    }

    pub fn ClearEdges(&mut self) {
        self.inner.clear_edges()
    }

    pub fn NumEdges(&self) -> usize {
        self.inner.num_edges()
    }

    pub fn Build(
        &self,
        Shape: &crate::topology::topods_solid::TopoDsSolid,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.apply_fillet(Shape)
    }

    pub fn BuildFillet(
        &self,
        Shape: &crate::topology::topods_solid::TopoDsSolid,
        Edges: &[Handle<TopoDS_Edge>],
        Radius: f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.fillet_edges(Shape, Edges, Radius)
    }

    // Face Chamfer Operations
    pub fn AddSingleFace(&mut self, Face: Handle<TopoDS_Face>) {
        self.inner.add_face(Face)
    }

    pub fn AddFaces(&mut self, Faces: &[Handle<TopoDS_Face>]) {
        self.inner.add_faces(Faces)
    }

    pub fn RemoveFace(&mut self, Face: &Handle<TopoDS_Face>) {
        self.inner.remove_face(Face)
    }

    pub fn ClearFaces(&mut self) {
        self.inner.clear_faces()
    }

    pub fn NumFaces(&self) -> usize {
        self.inner.num_faces()
    }

    pub fn BuildChamfer(
        &self,
        Shape: &crate::topology::topods_solid::TopoDsSolid,
        Faces: &[Handle<TopoDS_Face>],
        Distance: f64,
    ) -> crate::topology::topods_solid::TopoDsSolid {
        self.inner.chamfer_faces(Shape, Faces, Distance)
    }

    // Utility Methods
    pub fn CanFilletEdge(&self, Edge: &Handle<TopoDS_Edge>) -> bool {
        self.inner.can_fillet_edge(Edge)
    }

    pub fn CanChamferFace(&self, Face: &Handle<TopoDS_Face>) -> bool {
        self.inner.can_chamfer_face(Face)
    }

    pub fn CalculateFilletSurface(&self, Edge: &Handle<TopoDS_Edge>, Radius: f64) -> Vec<gp_Pnt> {
        self.inner.calculate_fillet_surface(Edge, Radius)
    }

    pub fn CalculateChamferSurface(
        &self,
        Edge: &Handle<TopoDS_Edge>,
        Distance: f64,
    ) -> Vec<gp_Pnt> {
        self.inner.calculate_chamfer_surface(Edge, Distance)
    }

    pub fn Reset(&mut self) {
        self.inner.reset()
    }
}

impl Default for BRepFilletAPI_MakeFillet {
    fn default() -> Self {
        Self::new()
    }
}

// BRepAlgoAPI_BooleanOperation wrapper with OpenCASCADE-compatible method names and parameters
pub struct BRepAlgoAPI_BooleanOperation {
    inner: crate::modeling::boolean_operations::BooleanOperations,
}

impl BRepAlgoAPI_BooleanOperation {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::boolean_operations::BooleanOperations::new(),
        }
    }

    pub fn IsNone(&self) -> bool {
        self.inner.is_none()
    }

    // Fuse Operation
    pub fn Fuse(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.fuse(Shape1, Shape2)
    }

    pub fn FuseAll(&self, Shapes: &[Handle<TopoDS_Shape>]) -> TopoDS_Compound {
        self.inner.fuse_all(Shapes)
    }

    // Cut Operation
    pub fn Cut(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.cut(Shape1, Shape2)
    }

    // Common Operation
    pub fn Common(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.common(Shape1, Shape2)
    }

    // Section Operation
    pub fn Section(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> TopoDS_Compound {
        self.inner.section(Shape1, Shape2)
    }

    pub fn SectionWithPlane(
        &self,
        Shape: &Handle<TopoDS_Shape>,
        Plane: &crate::geometry::Plane,
    ) -> TopoDS_Compound {
        self.inner.section_with_plane(Shape, Plane)
    }

    // Helper Methods
    pub fn CanPerformBoolean(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> bool {
        self.inner.can_perform_boolean(Shape1, Shape2)
    }

    pub fn MightIntersect(
        &self,
        Shape1: &Handle<TopoDS_Shape>,
        Shape2: &Handle<TopoDS_Shape>,
    ) -> bool {
        self.inner.might_intersect(Shape1, Shape2)
    }

    pub fn BoundingBoxesIntersect(
        &self,
        BB1: &(crate::geometry::Point, crate::geometry::Point),
        BB2: &(crate::geometry::Point, crate::geometry::Point),
    ) -> bool {
        let (min1, max1) = BB1;
        let (min2, max2) = BB2;

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
    /// Attempts to convert the curve handle to CurveEnum for compatibility.
    pub fn MakeEdgeWithCurve(
        &self,
        V1: Handle<TopoDS_Vertex>,
        V2: Handle<TopoDS_Vertex>,
        C: Handle<dyn Curve>,
    ) -> Handle<TopoDS_Edge> {
        // 尝试将 Handle<dyn Curve> 转换为 CurveEnum
        if let Some(curve_enum) = C.as_any().downcast_ref::<CurveEnum>() {
            self.inner
                .make_edge_with_curve(V1, V2, Handle::new(curve_enum.clone()))
        } else {
            self.inner.make_edge(V1, V2)
        }
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
    /// Attempts to convert the curve handle to CurveEnum for compatibility.
    pub fn SetEdgeCurve(&self, E: &mut TopoDS_Edge, C: Handle<dyn Curve>) {
        // 尝试将 Handle<dyn Curve> 转换为 CurveEnum
        if let Some(curve_enum) = C.as_any().downcast_ref::<CurveEnum>() {
            self.inner
                .set_edge_curve(E, Handle::new(curve_enum.clone()));
        }
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
    /// Attempts to convert the surface handle to SurfaceEnum for compatibility.
    pub fn MakeFaceFromSurface(&self, S: Handle<dyn Surface>) -> Handle<TopoDS_Face> {
        // 尝试将 Handle<dyn Surface> 转换为 SurfaceEnum
        if let Some(surface_enum) = S.as_any().downcast_ref::<SurfaceEnum>() {
            self.inner
                .make_face_with_surface(Handle::new(surface_enum.clone()))
        } else {
            self.inner.make_face()
        }
    }

    pub fn MakeFaceWithWire(&self, W: Handle<TopoDS_Wire>) -> Handle<TopoDS_Face> {
        self.inner.make_face_with_wire(W)
    }

    /// Create a face from a surface and a wire.
    /// Attempts to convert the surface handle to SurfaceEnum for compatibility.
    pub fn MakeFaceWithSurfaceAndWire(
        &self,
        S: Handle<dyn Surface>,
        W: Handle<TopoDS_Wire>,
    ) -> Handle<TopoDS_Face> {
        // 尝试将 Handle<dyn Surface> 转换为 SurfaceEnum
        if let Some(surface_enum) = S.as_any().downcast_ref::<SurfaceEnum>() {
            self.inner
                .make_face_with_surface_and_wire(Handle::new(surface_enum.clone()), W)
        } else {
            self.inner.make_face_with_wire(W)
        }
    }

    pub fn AddWire(&self, F: &mut TopoDS_Face, W: Handle<TopoDS_Wire>) {
        self.inner.add_wire_to_face(F, W)
    }

    pub fn SetFaceOuterWire(&self, F: &mut TopoDS_Face, W: Handle<TopoDS_Wire>) {
        self.inner.set_face_outer_wire(F, W)
    }

    /// Set the surface for a face.
    /// Attempts to convert the surface handle to SurfaceEnum for compatibility.
    pub fn SetFaceSurface(&self, F: &mut TopoDS_Face, S: Handle<dyn Surface>) {
        // 尝试将 Handle<dyn Surface> 转换为 SurfaceEnum
        if let Some(surface_enum) = S.as_any().downcast_ref::<SurfaceEnum>() {
            self.inner
                .set_face_surface(F, Handle::new(surface_enum.clone()));
        }
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

    pub fn MakeSolidFromShell(&self, S: Handle<TopoDS_Shell>) -> Handle<TopoDS_Solid> {
        self.inner.make_solid_from_shell(S)
    }

    pub fn AddShell(&self, So: &mut TopoDS_Solid, S: Handle<TopoDS_Shell>) {
        self.inner.add_shell_to_solid(So, S)
    }

    pub fn SetSolidOuterShell(&self, So: &mut TopoDS_Solid, S: Handle<TopoDS_Shell>) {
        self.inner.set_solid_outer_shell(So, S)
    }

    pub fn ClearSolid(&self, So: &mut TopoDS_Solid) {
        self.inner.clear_solid(So)
    }

    // Compound Operations
    pub fn MakeCompound(&self) -> Handle<TopoDS_Compound> {
        self.inner.make_compound()
    }

    pub fn Add(&self, C: &mut TopoDS_Compound, S: Handle<TopoDS_Shape>) {
        self.inner.add_to_compound(C, S)
    }

    pub fn Remove(&self, C: &mut TopoDS_Compound, S: &Handle<TopoDS_Shape>) {
        self.inner.remove_from_compound(C, S)
    }

    pub fn ClearCompound(&self, C: &mut TopoDS_Compound) {
        self.inner.clear_compound(C)
    }

    // Copy Operations
    pub fn CopyVertex(&self, V: &TopoDS_Vertex) -> Handle<TopoDS_Vertex> {
        self.inner.copy_vertex(V)
    }

    pub fn CopyEdge(&self, E: &TopoDS_Edge) -> Handle<TopoDS_Edge> {
        self.inner.copy_edge(E)
    }

    pub fn CopyWire(&self, W: &TopoDS_Wire) -> Handle<TopoDS_Wire> {
        self.inner.copy_wire(W)
    }

    pub fn CopyFace(&self, F: &TopoDS_Face) -> Handle<TopoDS_Face> {
        self.inner.copy_face(F)
    }

    pub fn CopyShell(&self, S: &TopoDS_Shell) -> Handle<TopoDS_Shell> {
        self.inner.copy_shell(S)
    }

    pub fn CopySolid(&self, So: &TopoDS_Solid) -> Handle<TopoDS_Solid> {
        self.inner.copy_solid(So)
    }

    pub fn CopyCompound(&self, C: &TopoDS_Compound) -> Handle<TopoDS_Compound> {
        self.inner.copy_compound(C)
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
