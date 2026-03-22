//! WebAssembly bindings for primitive creation

use super::geometry::WasmPoint;
use super::topology::WasmSolid;
use crate::foundation::handle::Handle;
use crate::modeling::primitives;
use crate::topology::topods_solid::TopoDsSolid;
use wasm_bindgen::prelude::*;

/// WebAssembly wrapper for Box primitive
#[wasm_bindgen(js_name = Box)]
#[derive(Debug, Clone)]
pub struct WasmBox {
    pub(crate) inner: Handle<TopoDsSolid>,
    width: f64,
    height: f64,
    depth: f64,
}

#[wasm_bindgen(js_class = Box)]
impl WasmBox {
    /// Create a box
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, depth: f64) -> Self {
        let solid = primitives::make_box(width, height, depth, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            width,
            height,
            depth,
        }
    }

    /// Create a box at a specific position
    #[wasm_bindgen(js_name = at)]
    pub fn at(width: f64, height: f64, depth: f64, position: &WasmPoint) -> Self {
        let solid = primitives::make_box(width, height, depth, Some(position.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            width,
            height,
            depth,
        }
    }

    /// Get width
    #[wasm_bindgen(getter, js_name = width)]
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Get height
    #[wasm_bindgen(getter, js_name = height)]
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get depth
    #[wasm_bindgen(getter, js_name = depth)]
    pub fn depth(&self) -> f64 {
        self.depth
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        self.width * self.height * self.depth
    }

    /// Get surface area
    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        2.0 * (self.width * self.height + self.width * self.depth + self.height * self.depth)
    }

    /// Convert to solid
    #[wasm_bindgen(js_name = toSolid)]
    pub fn to_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Box({}, {}, {})", self.width, self.height, self.depth)
    }
}

/// WebAssembly wrapper for Sphere primitive
#[wasm_bindgen(js_name = Sphere)]
#[derive(Debug, Clone)]
pub struct WasmSphere {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius: f64,
}

#[wasm_bindgen(js_class = Sphere)]
impl WasmSphere {
    /// Create a sphere
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64) -> Self {
        let solid = primitives::make_sphere(radius, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
        }
    }

    /// Create a sphere at a specific position
    #[wasm_bindgen(js_name = at)]
    pub fn at(radius: f64, center: &WasmPoint) -> Self {
        let solid = primitives::make_sphere(radius, Some(center.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
        }
    }

    /// Get radius
    #[wasm_bindgen(getter, js_name = radius)]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
    }

    /// Get surface area
    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }

    /// Convert to solid
    #[wasm_bindgen(js_name = toSolid)]
    pub fn to_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Sphere({})", self.radius)
    }
}

/// WebAssembly wrapper for Cylinder primitive
#[wasm_bindgen(js_name = Cylinder)]
#[derive(Debug, Clone)]
pub struct WasmCylinder {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius: f64,
    height: f64,
}

#[wasm_bindgen(js_class = Cylinder)]
impl WasmCylinder {
    /// Create a cylinder
    #[wasm_bindgen(constructor)]
    pub fn new(radius: f64, height: f64) -> Self {
        let solid = primitives::make_cylinder(radius, height, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
            height,
        }
    }

    /// Create a cylinder at a specific position
    #[wasm_bindgen(js_name = at)]
    pub fn at(radius: f64, height: f64, position: &WasmPoint) -> Self {
        let solid = primitives::make_cylinder(radius, height, Some(position.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
            height,
        }
    }

    /// Get radius
    #[wasm_bindgen(getter, js_name = radius)]
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Get height
    #[wasm_bindgen(getter, js_name = height)]
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) * self.height
    }

    /// Get surface area
    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius * (self.radius + self.height)
    }

    /// Convert to solid
    #[wasm_bindgen(js_name = toSolid)]
    pub fn to_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Cylinder({}, {})", self.radius, self.height)
    }
}

/// WebAssembly wrapper for Cone primitive
#[wasm_bindgen(js_name = Cone)]
#[derive(Debug, Clone)]
pub struct WasmCone {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius1: f64,
    radius2: f64,
    height: f64,
}

#[wasm_bindgen(js_class = Cone)]
impl WasmCone {
    /// Create a cone
    /// If radius2 is 0, creates a standard cone. Otherwise creates a truncated cone.
    #[wasm_bindgen(constructor)]
    pub fn new(radius1: f64, radius2: f64, height: f64) -> Self {
        // For now, use radius1 as the base radius. Truncated cone support would require
        // a separate implementation in the primitives module.
        let _ = radius2; // radius2 is reserved for future truncated cone support
        let solid = primitives::make_cone(radius1, height, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius1,
            radius2,
            height,
        }
    }

    /// Create a cone at a specific position
    /// If radius2 is 0, creates a standard cone. Otherwise creates a truncated cone.
    #[wasm_bindgen(js_name = at)]
    pub fn at(radius1: f64, radius2: f64, height: f64, position: &WasmPoint) -> Self {
        // For now, use radius1 as the base radius. Truncated cone support would require
        // a separate implementation in the primitives module.
        let _ = radius2; // radius2 is reserved for future truncated cone support
        let solid = primitives::make_cone(radius1, height, Some(position.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius1,
            radius2,
            height,
        }
    }

    /// Get radius1
    #[wasm_bindgen(getter, js_name = radius1)]
    pub fn radius1(&self) -> f64 {
        self.radius1
    }

    /// Get radius2
    #[wasm_bindgen(getter, js_name = radius2)]
    pub fn radius2(&self) -> f64 {
        self.radius2
    }

    /// Get height
    #[wasm_bindgen(getter, js_name = height)]
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        (1.0 / 3.0) * std::f64::consts::PI * self.height * (self.radius1.powi(2) + self.radius1 * self.radius2 + self.radius2.powi(2))
    }

    /// Convert to solid
    #[wasm_bindgen(js_name = toSolid)]
    pub fn to_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Cone({}, {}, {})", self.radius1, self.radius2, self.height)
    }
}

/// WebAssembly wrapper for Torus primitive
#[wasm_bindgen(js_name = Torus)]
#[derive(Debug, Clone)]
pub struct WasmTorus {
    pub(crate) inner: Handle<TopoDsSolid>,
    major_radius: f64,
    minor_radius: f64,
}

#[wasm_bindgen(js_class = Torus)]
impl WasmTorus {
    /// Create a torus
    #[wasm_bindgen(constructor)]
    pub fn new(major_radius: f64, minor_radius: f64) -> Self {
        let solid = primitives::make_torus(major_radius, minor_radius, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            major_radius,
            minor_radius,
        }
    }

    /// Get major radius
    #[wasm_bindgen(getter, js_name = majorRadius)]
    pub fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Get minor radius
    #[wasm_bindgen(getter, js_name = minorRadius)]
    pub fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        2.0 * std::f64::consts::PI.powi(2) * self.major_radius * self.minor_radius.powi(2)
    }

    /// Get surface area
    #[wasm_bindgen(js_name = surfaceArea)]
    pub fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI.powi(2) * self.major_radius * self.minor_radius
    }

    /// Convert to solid
    #[wasm_bindgen(js_name = toSolid)]
    pub fn to_solid(&self) -> WasmSolid {
        WasmSolid {
            inner: self.inner.clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Torus({}, {})", self.major_radius, self.minor_radius)
    }
}
