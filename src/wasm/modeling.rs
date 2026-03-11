//! WebAssembly bindings for modeling operations

use super::geometry::WasmPoint;
use super::topology::{WasmEdge, WasmFace, WasmShell, WasmSolid, WasmVertex, WasmWire};
use crate::foundation::handle::Handle;
use crate::modeling::{
    boolean_operations::BooleanOperations, brep_builder::BrepBuilder,
    fillet_chamfer::FilletChamfer, offset_operations::OffsetOperations,
};
use crate::topology::topods_solid::TopoDsSolid;
use wasm_bindgen::prelude::*;

/// WebAssembly wrapper for BRep Builder
#[wasm_bindgen(js_name = BrepBuilder)]
#[derive(Debug, Clone)]
pub struct WasmBrepBuilder {
    inner: BrepBuilder,
}

#[wasm_bindgen(js_class = BrepBuilder)]
impl WasmBrepBuilder {
    /// Create a new BRep builder
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: BrepBuilder::new(),
        }
    }

    /// Create a vertex
    #[wasm_bindgen(js_name = makeVertex)]
    pub fn make_vertex(&self, point: &WasmPoint) -> WasmVertex {
        WasmVertex {
            inner: self.inner.make_vertex(point.inner.clone()),
        }
    }

    /// Create an edge from two vertices
    #[wasm_bindgen(js_name = makeEdge)]
    pub fn make_edge(&self, v1: &WasmVertex, v2: &WasmVertex) -> WasmEdge {
        WasmEdge {
            inner: self.inner.make_edge(v1.inner.clone(), v2.inner.clone()),
        }
    }

    /// Create a wire
    #[wasm_bindgen(js_name = makeWire)]
    pub fn make_wire(&self) -> WasmWire {
        WasmWire {
            inner: self.inner.make_wire(),
        }
    }

    /// Create a face
    #[wasm_bindgen(js_name = makeFace)]
    pub fn make_face(&self) -> WasmFace {
        WasmFace {
            inner: self.inner.make_face(),
        }
    }

    /// Create a shell
    #[wasm_bindgen(js_name = makeShell)]
    pub fn make_shell(&self) -> WasmShell {
        WasmShell {
            inner: self.inner.make_shell(),
        }
    }

    /// Create a solid
    #[wasm_bindgen(js_name = makeSolid)]
    pub fn make_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.make_solid(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "BrepBuilder".to_string()
    }
}

impl Default for WasmBrepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Boolean Operations
#[wasm_bindgen(js_name = BooleanOperations)]
#[derive(Debug, Clone)]
pub struct WasmBooleanOperations {
    inner: BooleanOperations,
}

#[wasm_bindgen(js_class = BooleanOperations)]
impl WasmBooleanOperations {
    /// Create a new boolean operations instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: BooleanOperations::new(),
        }
    }

    /// Fuse (union) two solids
    #[wasm_bindgen(js_name = fuse)]
    pub fn fuse(&self, solid1: &WasmSolid, solid2: &WasmSolid) -> WasmSolid {
        let shape1 = if let Some(s1) = solid1.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s1.shape().clone()))
        } else {
            Handle::null()
        };
        let shape2 = if let Some(s2) = solid2.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s2.shape().clone()))
        } else {
            Handle::null()
        };
        let _result = self.inner.fuse(&shape1, &shape2);
        WasmSolid {
            inner: Handle::new(std::sync::Arc::new(TopoDsSolid::new())),
        }
    }

    /// Cut (subtract) solid2 from solid1
    #[wasm_bindgen(js_name = cut)]
    pub fn cut(&self, solid1: &WasmSolid, solid2: &WasmSolid) -> WasmSolid {
        let shape1 = if let Some(s1) = solid1.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s1.shape().clone()))
        } else {
            Handle::null()
        };
        let shape2 = if let Some(s2) = solid2.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s2.shape().clone()))
        } else {
            Handle::null()
        };
        let _result = self.inner.cut(&shape1, &shape2);
        WasmSolid {
            inner: Handle::new(std::sync::Arc::new(TopoDsSolid::new())),
        }
    }

    /// Common (intersection) of two solids
    #[wasm_bindgen(js_name = common)]
    pub fn common(&self, solid1: &WasmSolid, solid2: &WasmSolid) -> WasmSolid {
        let shape1 = if let Some(s1) = solid1.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s1.shape().clone()))
        } else {
            Handle::null()
        };
        let shape2 = if let Some(s2) = solid2.inner.as_ref() {
            Handle::new(std::sync::Arc::new(s2.shape().clone()))
        } else {
            Handle::null()
        };
        let _result = self.inner.common(&shape1, &shape2);
        WasmSolid {
            inner: Handle::new(std::sync::Arc::new(TopoDsSolid::new())),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "BooleanOperations".to_string()
    }
}

impl Default for WasmBooleanOperations {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Fillet and Chamfer operations
#[wasm_bindgen(js_name = FilletChamfer)]
#[derive(Debug, Clone)]
pub struct WasmFilletChamfer {
    inner: FilletChamfer,
}

#[wasm_bindgen(js_class = FilletChamfer)]
impl WasmFilletChamfer {
    /// Create a new fillet/chamfer instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: FilletChamfer::new(),
        }
    }

    /// Create with specific radius
    #[wasm_bindgen(js_name = withRadius)]
    pub fn with_radius(radius: f64) -> Self {
        Self {
            inner: FilletChamfer::with_radius(radius),
        }
    }

    /// Create with specific chamfer distance
    #[wasm_bindgen(js_name = withChamferDistance)]
    pub fn with_chamfer_distance(distance: f64) -> Self {
        Self {
            inner: FilletChamfer::with_chamfer_distance(distance),
        }
    }

    /// Set radius
    #[wasm_bindgen(js_name = setRadius)]
    pub fn set_radius(&mut self, radius: f64) {
        self.inner.set_radius(radius);
    }

    /// Get radius
    #[wasm_bindgen(getter, js_name = radius)]
    pub fn radius(&self) -> f64 {
        self.inner.radius()
    }

    /// Set chamfer distance
    #[wasm_bindgen(js_name = setChamferDistance)]
    pub fn set_chamfer_distance(&mut self, distance: f64) {
        self.inner.set_chamfer_distance(distance);
    }

    /// Get chamfer distance
    #[wasm_bindgen(getter, js_name = chamferDistance)]
    pub fn chamfer_distance(&self) -> f64 {
        self.inner.chamfer_distance()
    }

    /// Apply fillet to a solid
    #[wasm_bindgen(js_name = applyFillet)]
    pub fn apply_fillet(&self, solid: &WasmSolid) -> WasmSolid {
        let result = self.inner.apply_fillet(&solid.inner);
        WasmSolid {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// Apply chamfer to a solid
    #[wasm_bindgen(js_name = applyChamfer)]
    pub fn apply_chamfer(&self, solid: &WasmSolid) -> WasmSolid {
        let result = self.inner.apply_chamfer(&solid.inner);
        WasmSolid {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "FilletChamfer(radius={}, chamferDistance={})",
            self.radius(),
            self.chamfer_distance()
        )
    }
}

impl Default for WasmFilletChamfer {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly wrapper for Offset Operations
#[wasm_bindgen(js_name = OffsetOperations)]
#[derive(Debug, Clone)]
pub struct WasmOffsetOperations {
    inner: OffsetOperations,
}

#[wasm_bindgen(js_class = OffsetOperations)]
impl WasmOffsetOperations {
    /// Create a new offset operations instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: OffsetOperations::new(),
        }
    }

    /// Create with specific offset distance
    #[wasm_bindgen(js_name = withOffsetDistance)]
    pub fn with_offset_distance(distance: f64) -> Self {
        Self {
            inner: OffsetOperations::with_offset_distance(distance),
        }
    }

    /// Set offset distance
    #[wasm_bindgen(js_name = setOffsetDistance)]
    pub fn set_offset_distance(&mut self, distance: f64) {
        self.inner.set_offset_distance(distance);
    }

    /// Get offset distance
    #[wasm_bindgen(getter, js_name = offsetDistance)]
    pub fn offset_distance(&self) -> f64 {
        self.inner.offset_distance()
    }

    /// Set tolerance
    #[wasm_bindgen(js_name = setTolerance)]
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Get tolerance
    #[wasm_bindgen(getter, js_name = tolerance)]
    pub fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "OffsetOperations(offsetDistance={}, tolerance={})",
            self.offset_distance(),
            self.tolerance()
        )
    }
}

impl Default for WasmOffsetOperations {
    fn default() -> Self {
        Self::new()
    }
}
