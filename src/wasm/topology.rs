//! WebAssembly bindings for topology types

use super::geometry::WasmPoint;
use crate::foundation::handle::Handle;
use crate::topology::{
    topods_compound::TopoDsCompound, topods_edge::TopoDsEdge, topods_face::TopoDsFace,
    topods_shell::TopoDsShell, topods_solid::TopoDsSolid, topods_vertex::TopoDsVertex,
    topods_wire::TopoDsWire,
};
use wasm_bindgen::prelude::*;

/// WebAssembly wrapper for Vertex
#[wasm_bindgen(js_name = Vertex)]
#[derive(Debug, Clone)]
pub struct WasmVertex {
    pub(crate) inner: Handle<TopoDsVertex>,
}

#[wasm_bindgen(js_class = Vertex)]
impl WasmVertex {
    /// Create a vertex from a point
    #[wasm_bindgen(constructor)]
    pub fn new(point: &WasmPoint) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsVertex::new(point.inner.clone()))),
        }
    }

    /// Get the point of this vertex
    #[wasm_bindgen(getter, js_name = point)]
    pub fn point(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.point().clone(),
        }
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Check if vertex is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Vertex({})", self.point().to_string())
    }
}

/// WebAssembly wrapper for Edge
#[wasm_bindgen(js_name = Edge)]
#[derive(Debug, Clone)]
pub struct WasmEdge {
    pub(crate) inner: Handle<TopoDsEdge>,
}

#[wasm_bindgen(js_class = Edge)]
impl WasmEdge {
    /// Create an edge from two vertices
    #[wasm_bindgen(constructor)]
    pub fn new(v1: &WasmVertex, v2: &WasmVertex) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsEdge::new(
                v1.inner.clone(),
                v2.inner.clone(),
            ))),
        }
    }

    /// Check if this edge is degenerate
    #[wasm_bindgen(js_name = isDegenerate)]
    pub fn is_degenerate(&self) -> bool {
        self.inner.is_degenerate()
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Check if edge is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Get first vertex
    #[wasm_bindgen(js_name = firstVertex)]
    pub fn first_vertex(&self) -> WasmVertex {
        WasmVertex {
            inner: self.inner.start_vertex().clone(),
        }
    }

    /// Get last vertex
    #[wasm_bindgen(js_name = lastVertex)]
    pub fn last_vertex(&self) -> WasmVertex {
        WasmVertex {
            inner: self.inner.end_vertex().clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Edge".to_string()
    }
}

/// WebAssembly wrapper for Wire
#[wasm_bindgen(js_name = Wire)]
#[derive(Debug, Clone)]
pub struct WasmWire {
    pub(crate) inner: Handle<TopoDsWire>,
}

#[wasm_bindgen(js_class = Wire)]
impl WasmWire {
    /// Create an empty wire
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsWire::new())),
        }
    }

    /// Add an edge to this wire
    #[wasm_bindgen(js_name = addEdge)]
    pub fn add_edge(&mut self, edge: &WasmEdge) {
        self.inner.add_edge(edge.inner.clone());
    }

    /// Remove an edge from this wire
    #[wasm_bindgen(js_name = removeEdge)]
    pub fn remove_edge(&mut self, edge: &WasmEdge) {
        self.inner.remove_edge(&edge.inner);
    }

    /// Get number of edges
    #[wasm_bindgen(js_name = edgeCount)]
    pub fn edge_count(&self) -> usize {
        self.inner.num_edges()
    }

    /// Get edges
    #[wasm_bindgen(js_name = edges)]
    pub fn edges(&self) -> Vec<WasmEdge> {
        self.inner
            .edges()
            .iter()
            .map(|e| WasmEdge { inner: e.clone() })
            .collect()
    }

    /// Check if this wire is closed
    #[wasm_bindgen(js_name = isClosed)]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Check if wire is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Wire(edges={}, closed={})", self.edge_count(), self.is_closed())
    }
}

impl Default for WasmWire {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Face
#[wasm_bindgen(js_name = Face)]
#[derive(Debug, Clone)]
pub struct WasmFace {
    pub(crate) inner: Handle<TopoDsFace>,
}

#[wasm_bindgen(js_class = Face)]
impl WasmFace {
    /// Create an empty face
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsFace::new())),
        }
    }

    /// Create a face from a wire
    #[wasm_bindgen(js_name = fromWire)]
    pub fn from_wire(wire: &WasmWire) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsFace::with_outer_wire((*wire.inner).clone()))),
        }
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Check if face is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Get area
    #[wasm_bindgen(js_name = area)]
    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Face(area={})", self.area())
    }
}

impl Default for WasmFace {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Shell
#[wasm_bindgen(js_name = Shell)]
#[derive(Debug, Clone)]
pub struct WasmShell {
    pub(crate) inner: Handle<TopoDsShell>,
}

#[wasm_bindgen(js_class = Shell)]
impl WasmShell {
    /// Create an empty shell
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsShell::new())),
        }
    }

    /// Add a face to this shell
    #[wasm_bindgen(js_name = addFace)]
    pub fn add_face(&mut self, face: &WasmFace) {
        self.inner.add_face(face.inner.clone());
    }

    /// Remove a face from this shell
    #[wasm_bindgen(js_name = removeFace)]
    pub fn remove_face(&mut self, face: &WasmFace) {
        self.inner.remove_face(&face.inner);
    }

    /// Get number of faces
    #[wasm_bindgen(js_name = faceCount)]
    pub fn face_count(&self) -> usize {
        self.inner.num_faces()
    }

    /// Get faces
    #[wasm_bindgen(js_name = faces)]
    pub fn faces(&self) -> Vec<WasmFace> {
        self.inner
            .faces()
            .iter()
            .map(|f| WasmFace { inner: f.clone() })
            .collect()
    }

    /// Check if this shell is closed
    #[wasm_bindgen(js_name = isClosed)]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Check if shell is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Get area
    #[wasm_bindgen(js_name = area)]
    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Shell(faces={}, closed={}, area={})", self.face_count(), self.is_closed(), self.area())
    }
}

impl Default for WasmShell {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Solid
#[wasm_bindgen(js_name = Solid)]
#[derive(Debug, Clone)]
pub struct WasmSolid {
    pub(crate) inner: Handle<TopoDsSolid>,
}

#[wasm_bindgen(js_class = Solid)]
impl WasmSolid {
    /// Create an empty solid
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsSolid::new())),
        }
    }

    /// Create a solid from a shell
    #[wasm_bindgen(js_name = fromShell)]
    pub fn from_shell(shell: &WasmShell) -> Self {
        let mut solid = TopoDsSolid::new();
        solid.set_outer_shell(shell.inner.clone());
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Get tolerance
    #[wasm_bindgen(js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Check if solid is null
    #[wasm_bindgen(js_name = isNull)]
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    /// Get area
    #[wasm_bindgen(js_name = area)]
    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    /// Get shells
    #[wasm_bindgen(js_name = shells)]
    pub fn shells(&self) -> Vec<WasmShell> {
        self.inner
            .shells()
            .iter()
            .map(|s| WasmShell { inner: s.clone() })
            .collect()
    }

    /// Get number of shells
    #[wasm_bindgen(js_name = shellCount)]
    pub fn shell_count(&self) -> usize {
        self.inner.num_shells()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Solid(shells={}, volume={}, area={})", self.shell_count(), self.volume(), self.area())
    }
}

impl Default for WasmSolid {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Compound
#[wasm_bindgen(js_name = Compound)]
#[derive(Debug, Clone)]
pub struct WasmCompound {
    #[allow(dead_code)]
    pub(crate) inner: Handle<TopoDsCompound>,
}

#[wasm_bindgen(js_class = Compound)]
impl WasmCompound {
    /// Create an empty compound
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsCompound::new())),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Compound".to_string()
    }
}

impl Default for WasmCompound {
    fn default() -> Self {
        Self::new()
    }
}
