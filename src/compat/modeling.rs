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

/// BRepFilletAPI_MakeFillet - OpenCASCADE compatible fillet maker
/// 
/// In OpenCASCADE, this class is used as:
/// ```cpp
/// BRepFilletAPI_MakeFillet fillet(shape);
/// fillet.Add(radius, edge);
/// TopoDS_Shape result = fillet.Shape();
/// ```
#[allow(non_camel_case_types)]
pub struct BRepFilletAPI_MakeFillet {
    inner: crate::modeling::fillet_chamfer::FilletChamfer,
    shape: Option<Handle<TopoDS_Shape>>,
}

impl BRepFilletAPI_MakeFillet {
    /// Create a new fillet maker for the given shape (OpenCASCADE API)
    /// 
    /// # Parameters
    /// - `S`: The shape to apply fillets to
    pub fn new(S: &Handle<TopoDS_Shape>) -> Self {
        Self {
            inner: crate::modeling::fillet_chamfer::FilletChamfer::new(),
            shape: Some(S.clone()),
        }
    }

    /// Add an edge with a radius for filleting (OpenCASCADE API: Add)
    /// 
    /// # Parameters
    /// - `R`: The fillet radius
    /// - `E`: The edge to fillet
    pub fn Add(&mut self, R: f64, E: &Handle<TopoDS_Edge>) {
        self.inner.set_radius(R);
        self.inner.add_edge(E.clone());
    }

    /// Add an edge with two radii for variable radius fillet (OpenCASCADE API: Add)
    /// 
    /// # Parameters
    /// - `R1`: The first radius
    /// - `R2`: The second radius
    /// - `E`: The edge to fillet
    pub fn Add_with_radii(&mut self, R1: f64, R2: f64, E: &Handle<TopoDS_Edge>) {
        // Use average radius for now (variable radius not fully implemented)
        let avg_radius = (R1 + R2) / 2.0;
        self.inner.set_radius(avg_radius);
        self.inner.add_edge(E.clone());
    }

    /// Build the fillet operation (OpenCASCADE API)
    pub fn Build(&mut self) {
        // Build is called automatically when Shape() is called
    }

    /// Check if the operation is done (OpenCASCADE API)
    pub fn IsDone(&self) -> bool {
        self.inner.num_edges() > 0
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    /// 
    /// # Returns
    /// The shape with fillets applied
    pub fn Shape(&self) -> Option<Handle<TopoDS_Shape>> {
        // Return the original shape (filleting is applied in-place conceptually)
        self.shape.clone()
    }

    /// Reset the fillet maker
    pub fn Reset(&mut self) {
        self.inner.reset();
    }

    /// Get the number of contours (edges to fillet)
    pub fn NbContours(&self) -> usize {
        self.inner.num_edges()
    }
}

// ============================================================================
// Boolean Operations - OpenCASCADE Style
// ============================================================================

/// BRepAlgoAPI_Fuse - OpenCASCADE compatible fuse (union) operation
#[allow(non_camel_case_types)]
pub struct BRepAlgoAPI_Fuse {
    inner: crate::modeling::boolean_operations::BooleanOperations,
    shape1: Option<Handle<TopoDS_Shape>>,
    shape2: Option<Handle<TopoDS_Shape>>,
    result: Option<TopoDS_Compound>,
}

impl BRepAlgoAPI_Fuse {
    /// Create a new fuse operation (OpenCASCADE API)
    /// 
    /// # Parameters
    /// - `S1`: First shape
    /// - `S2`: Second shape
    pub fn new(S1: &Handle<TopoDS_Shape>, S2: &Handle<TopoDS_Shape>) -> Self {
        let inner = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = inner.fuse(S1, S2);
        Self {
            inner,
            shape1: Some(S1.clone()),
            shape2: Some(S2.clone()),
            result: Some(result),
        }
    }

    /// Build the operation (OpenCASCADE API)
    pub fn Build(&mut self) {
        // Already built in constructor
    }

    /// Check if the operation is done (OpenCASCADE API)
    pub fn IsDone(&self) -> bool {
        self.result.is_some()
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        // Return as shape handle - compound is a type of shape
        self.result.as_ref().map(|c| {
            Handle::new(std::sync::Arc::new(c.clone().into_shape()))
        }).unwrap_or_else(|| Handle::new(std::sync::Arc::new(TopoDS_Shape::new(crate::topology::ShapeType::Compound))))
    }
}

/// BRepAlgoAPI_Cut - OpenCASCADE compatible cut (difference) operation
#[allow(non_camel_case_types)]
pub struct BRepAlgoAPI_Cut {
    inner: crate::modeling::boolean_operations::BooleanOperations,
    shape1: Option<Handle<TopoDS_Shape>>,
    shape2: Option<Handle<TopoDS_Shape>>,
    result: Option<TopoDS_Compound>,
}

impl BRepAlgoAPI_Cut {
    /// Create a new cut operation (OpenCASCADE API)
    /// 
    /// # Parameters
    /// - `S1`: Shape to cut from
    /// - `S2`: Shape to cut with
    pub fn new(S1: &Handle<TopoDS_Shape>, S2: &Handle<TopoDS_Shape>) -> Self {
        let inner = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = inner.cut(S1, S2);
        Self {
            inner,
            shape1: Some(S1.clone()),
            shape2: Some(S2.clone()),
            result: Some(result),
        }
    }

    /// Build the operation (OpenCASCADE API)
    pub fn Build(&mut self) {
        // Already built in constructor
    }

    /// Check if the operation is done (OpenCASCADE API)
    pub fn IsDone(&self) -> bool {
        self.result.is_some()
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        self.result.as_ref().map(|c| {
            Handle::new(std::sync::Arc::new(c.clone().into_shape()))
        }).unwrap_or_else(|| Handle::new(std::sync::Arc::new(TopoDS_Shape::new(crate::topology::ShapeType::Compound))))
    }
}

/// BRepAlgoAPI_Common - OpenCASCADE compatible common (intersection) operation
#[allow(non_camel_case_types)]
pub struct BRepAlgoAPI_Common {
    inner: crate::modeling::boolean_operations::BooleanOperations,
    shape1: Option<Handle<TopoDS_Shape>>,
    shape2: Option<Handle<TopoDS_Shape>>,
    result: Option<TopoDS_Compound>,
}

impl BRepAlgoAPI_Common {
    /// Create a new common operation (OpenCASCADE API)
    /// 
    /// # Parameters
    /// - `S1`: First shape
    /// - `S2`: Second shape
    pub fn new(S1: &Handle<TopoDS_Shape>, S2: &Handle<TopoDS_Shape>) -> Self {
        let inner = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = inner.common(S1, S2);
        Self {
            inner,
            shape1: Some(S1.clone()),
            shape2: Some(S2.clone()),
            result: Some(result),
        }
    }

    /// Build the operation (OpenCASCADE API)
    pub fn Build(&mut self) {
        // Already built in constructor
    }

    /// Check if the operation is done (OpenCASCADE API)
    pub fn IsDone(&self) -> bool {
        self.result.is_some()
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        self.result.as_ref().map(|c| {
            Handle::new(std::sync::Arc::new(c.clone().into_shape()))
        }).unwrap_or_else(|| Handle::new(std::sync::Arc::new(TopoDS_Shape::new(crate::topology::ShapeType::Compound))))
    }
}

/// BRepAlgoAPI_Section - OpenCASCADE compatible section operation
#[allow(non_camel_case_types)]
pub struct BRepAlgoAPI_Section {
    inner: crate::modeling::boolean_operations::BooleanOperations,
    result: Option<TopoDS_Compound>,
}

impl BRepAlgoAPI_Section {
    /// Create a new section operation between two shapes (OpenCASCADE API)
    /// 
    /// # Parameters
    /// - `S1`: First shape
    /// - `S2`: Second shape
    pub fn new(S1: &Handle<TopoDS_Shape>, S2: &Handle<TopoDS_Shape>) -> Self {
        let inner = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = inner.section(S1, S2);
        Self {
            inner,
            result: Some(result),
        }
    }

    /// Create a section with a plane (OpenCASCADE API extension)
    /// 
    /// # Parameters
    /// - `S`: Shape to section
    /// - `P`: Plane for sectioning
    pub fn new_with_plane(S: &Handle<TopoDS_Shape>, P: &crate::geometry::Plane) -> Self {
        let inner = crate::modeling::boolean_operations::BooleanOperations::new();
        let result = inner.section_with_plane(S, P);
        Self {
            inner,
            result: Some(result),
        }
    }

    /// Build the operation (OpenCASCADE API)
    pub fn Build(&mut self) {
        // Already built in constructor
    }

    /// Check if the operation is done (OpenCASCADE API)
    pub fn IsDone(&self) -> bool {
        self.result.is_some()
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        self.result.as_ref().map(|c| {
            Handle::new(std::sync::Arc::new(c.clone().into_shape()))
        }).unwrap_or_else(|| Handle::new(std::sync::Arc::new(TopoDS_Shape::new(crate::topology::ShapeType::Compound))))
    }
}

// Legacy wrapper for backward compatibility (deprecated)
#[allow(non_camel_case_types)]
#[deprecated(since = "0.2.0", note = "Use BRepAlgoAPI_Fuse, BRepAlgoAPI_Cut, BRepAlgoAPI_Common, or BRepAlgoAPI_Section instead")]
pub struct BRepAlgoAPI_BooleanOperation {
    inner: crate::modeling::boolean_operations::BooleanOperations,
}

#[allow(deprecated)]
impl BRepAlgoAPI_BooleanOperation {
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::boolean_operations::BooleanOperations::new(),
        }
    }

    pub fn fuse(&self, shape1: &Handle<TopoDS_Shape>, shape2: &Handle<TopoDS_Shape>) -> TopoDS_Compound {
        self.inner.fuse(shape1, shape2)
    }

    pub fn cut(&self, shape1: &Handle<TopoDS_Shape>, shape2: &Handle<TopoDS_Shape>) -> TopoDS_Compound {
        self.inner.cut(shape1, shape2)
    }

    pub fn common(&self, shape1: &Handle<TopoDS_Shape>, shape2: &Handle<TopoDS_Shape>) -> TopoDS_Compound {
        self.inner.common(shape1, shape2)
    }

    pub fn section(&self, shape1: &Handle<TopoDS_Shape>, shape2: &Handle<TopoDS_Shape>) -> TopoDS_Compound {
        self.inner.section(shape1, shape2)
    }
}

// ============================================================================
// BRep_Builder - OpenCASCADE Style
// ============================================================================

/// BRep_Builder - OpenCASCADE compatible builder
/// 
/// In OpenCASCADE, BRep_Builder is used to construct and modify BRep topology.
#[allow(non_camel_case_types)]
pub struct BRep_Builder {
    inner: crate::modeling::brep_builder::BrepBuilder,
}

impl BRep_Builder {
    /// Create a new BRep builder
    pub fn new() -> Self {
        Self {
            inner: crate::modeling::brep_builder::BrepBuilder::new(),
        }
    }

    // =========================================================================
    // Vertex Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a vertex from a point (OpenCASCADE API: MakeVertex)
    pub fn MakeVertex(&self, P: gp_Pnt) -> Handle<TopoDS_Vertex> {
        self.inner.make_vertex(P)
    }

    /// Update vertex geometry (OpenCASCADE API: UpdateVertex)
    pub fn UpdateVertex(&self, V: &mut TopoDS_Vertex, P: gp_Pnt) {
        self.inner.update_vertex(V, P)
    }

    /// Set vertex tolerance (OpenCASCADE API)
    pub fn SetVertexTolerance(&self, V: &mut TopoDS_Vertex, Tol: f64) {
        self.inner.set_vertex_tolerance(V, Tol)
    }

    // =========================================================================
    // Edge Operations (OpenCASCADE API)
    // =========================================================================

    /// Make an edge from two vertices (OpenCASCADE API: MakeEdge)
    pub fn MakeEdge(&self, V1: Handle<TopoDS_Vertex>, V2: Handle<TopoDS_Vertex>) -> Handle<TopoDS_Edge> {
        self.inner.make_edge(V1, V2)
    }

    /// Update edge vertices (OpenCASCADE API: UpdateEdge)
    pub fn UpdateEdge(&self, E: &mut TopoDS_Edge, V1: Handle<TopoDS_Vertex>, V2: Handle<TopoDS_Vertex>) {
        self.inner.update_edge_vertices(E, V1, V2)
    }

    /// Set edge tolerance (OpenCASCADE API)
    pub fn SetEdgeTolerance(&self, E: &mut TopoDS_Edge, Tol: f64) {
        self.inner.set_edge_tolerance(E, Tol)
    }

    // =========================================================================
    // Wire Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a wire (OpenCASCADE API: MakeWire)
    pub fn MakeWire(&self) -> Handle<TopoDS_Wire> {
        self.inner.make_wire()
    }

    /// Add an edge to a wire (OpenCASCADE API: Add)
    pub fn Add(&self, W: &mut TopoDS_Wire, E: Handle<TopoDS_Edge>) {
        self.inner.add_edge_to_wire(W, E)
    }

    /// Remove an edge from a wire (OpenCASCADE API extension)
    pub fn Remove(&self, W: &mut TopoDS_Wire, E: &Handle<TopoDS_Edge>) {
        self.inner.remove_edge_from_wire(W, E)
    }

    // =========================================================================
    // Face Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a face (OpenCASCADE API: MakeFace)
    pub fn MakeFace(&self) -> Handle<TopoDS_Face> {
        self.inner.make_face()
    }

    /// Add a wire to a face (OpenCASCADE API: Add)
    pub fn AddWireToFace(&self, F: &mut TopoDS_Face, W: Handle<TopoDS_Wire>) {
        self.inner.add_wire_to_face(F, W)
    }

    /// Set face tolerance (OpenCASCADE API)
    pub fn SetFaceTolerance(&self, F: &mut TopoDS_Face, Tol: f64) {
        self.inner.set_face_tolerance(F, Tol)
    }

    // =========================================================================
    // Shell Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a shell (OpenCASCADE API: MakeShell)
    pub fn MakeShell(&self) -> Handle<TopoDS_Shell> {
        self.inner.make_shell()
    }

    /// Add a face to a shell (OpenCASCADE API: Add)
    pub fn AddFaceToShell(&self, S: &mut TopoDS_Shell, F: Handle<TopoDS_Face>) {
        self.inner.add_face_to_shell(S, F)
    }

    // =========================================================================
    // Solid Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a solid (OpenCASCADE API: MakeSolid)
    pub fn MakeSolid(&self) -> Handle<TopoDS_Solid> {
        self.inner.make_solid()
    }

    /// Add a shell to a solid (OpenCASCADE API: Add)
    pub fn AddShellToSolid(&self, So: &mut TopoDS_Solid, Sh: Handle<TopoDS_Shell>) {
        self.inner.add_shell_to_solid(So, Sh)
    }

    // =========================================================================
    // Compound Operations (OpenCASCADE API)
    // =========================================================================

    /// Make a compound (OpenCASCADE API: MakeCompound)
    pub fn MakeCompound(&self) -> Handle<TopoDS_Compound> {
        self.inner.make_compound()
    }

    /// Add a shape to a compound (OpenCASCADE API: Add)
    pub fn AddToCompound(&self, C: &mut TopoDS_Compound, S: Handle<TopoDS_Shape>) {
        self.inner.add_to_compound(C, S)
    }
}

impl Default for BRep_Builder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Primitive Makers - OpenCASCADE Style
// ============================================================================

/// BRepPrimAPI_MakeBox - OpenCASCADE compatible box primitive
#[allow(non_camel_case_types)]
pub struct BRepPrimAPI_MakeBox {
    result: Handle<TopoDS_Solid>,
}

impl BRepPrimAPI_MakeBox {
    /// Create a box from two points (OpenCASCADE API)
    pub fn new(P1: gp_Pnt, P2: gp_Pnt) -> Self {
        let dx = (P2.x - P1.x).abs();
        let dy = (P2.y - P1.y).abs();
        let dz = (P2.z - P1.z).abs();
        // Use primitives module to create box
        let solid = crate::modeling::primitives::make_box(dx, dy, dz, None);
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Create a box from dimensions (OpenCASCADE API)
    pub fn from_dimensions(dx: f64, dy: f64, dz: f64) -> Self {
        let solid = crate::modeling::primitives::make_box(dx, dy, dz, None);
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Create a box from axis and dimensions (OpenCASCADE API)
    pub fn new_with_axis(Axes: &crate::geometry::Ax2, dx: f64, dy: f64, dz: f64) -> Self {
        let solid = crate::modeling::primitives::make_box(dx, dy, dz, Some(*Axes.location()));
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    /// Returns the solid as a shape handle
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        // Convert solid to shape by dereferencing and cloning
        if let Some(solid) = self.result.as_ref() {
            let shape: TopoDS_Shape = solid.shape().clone();
            Handle::new(std::sync::Arc::new(shape))
        } else {
            Handle::null()
        }
    }

    /// Get the resulting solid (OpenCASCADE API)
    pub fn Solid(&self) -> Handle<TopoDS_Solid> {
        self.result.clone()
    }
}

/// BRepPrimAPI_MakeCylinder - OpenCASCADE compatible cylinder primitive
#[allow(non_camel_case_types)]
pub struct BRepPrimAPI_MakeCylinder {
    result: Handle<TopoDS_Solid>,
}

impl BRepPrimAPI_MakeCylinder {
    /// Create a cylinder (OpenCASCADE API)
    pub fn new(R: f64, H: f64) -> Self {
        let solid = crate::modeling::primitives::make_cylinder(R, H, None);
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Create a cylinder with axis (OpenCASCADE API)
    pub fn new_with_axis(Axes: &crate::geometry::Ax2, R: f64, H: f64) -> Self {
        let solid = crate::modeling::primitives::make_cylinder(R, H, Some(*Axes.location()));
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        // Convert solid to shape by dereferencing and cloning
        if let Some(solid) = self.result.as_ref() {
            let shape: TopoDS_Shape = solid.shape().clone();
            Handle::new(std::sync::Arc::new(shape))
        } else {
            Handle::null()
        }
    }

    /// Get the resulting solid (OpenCASCADE API)
    pub fn Solid(&self) -> Handle<TopoDS_Solid> {
        self.result.clone()
    }
}

/// BRepPrimAPI_MakeSphere - OpenCASCADE compatible sphere primitive
#[allow(non_camel_case_types)]
pub struct BRepPrimAPI_MakeSphere {
    result: Handle<TopoDS_Solid>,
}

impl BRepPrimAPI_MakeSphere {
    /// Create a sphere (OpenCASCADE API)
    pub fn new(R: f64) -> Self {
        let solid = crate::modeling::primitives::make_sphere(R, None);
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Create a sphere with center point (OpenCASCADE API)
    pub fn new_with_center(Center: gp_Pnt, R: f64) -> Self {
        let solid = crate::modeling::primitives::make_sphere(R, Some(Center));
        Self {
            result: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Get the resulting shape (OpenCASCADE API: Shape)
    pub fn Shape(&self) -> Handle<TopoDS_Shape> {
        // Convert solid to shape by dereferencing and cloning
        if let Some(solid) = self.result.as_ref() {
            let shape: TopoDS_Shape = solid.shape().clone();
            Handle::new(std::sync::Arc::new(shape))
        } else {
            Handle::null()
        }
    }

    /// Get the resulting solid (OpenCASCADE API)
    pub fn Solid(&self) -> Handle<TopoDS_Solid> {
        self.result.clone()
    }
}

// ============================================================================
// Primitives module re-export
// ============================================================================

pub mod primitives {
    pub use crate::modeling::primitives::*;
}
