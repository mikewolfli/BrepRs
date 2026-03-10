//! WebAssembly bindings for topology types

use wasm_bindgen::prelude::*;
use crate::foundation::handle::Handle;
use crate::topology::{
    topods_vertex::TopoDsVertex,
    topods_edge::TopoDsEdge,
    topods_wire::TopoDsWire,
    topods_face::TopoDsFace,
    topods_shell::TopoDsShell,
    topods_solid::TopoDsSolid,
    topods_compound::TopoDsCompound,
};
use super::geometry::WasmPoint;

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

    /// Check if this wire is closed
    #[wasm_bindgen(js_name = isClosed)]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Wire".to_string()
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

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Face".to_string()
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

    /// Check if this shell is closed
    #[wasm_bindgen(js_name = isClosed)]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Shell".to_string()
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

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Solid".to_string()
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
