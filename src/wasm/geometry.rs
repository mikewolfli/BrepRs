//! WebAssembly bindings for geometry types

use crate::geometry::{Axis, Direction, Plane, Point, Vector};
use wasm_bindgen::prelude::*;

/// WebAssembly wrapper for Point
#[wasm_bindgen(js_name = Point)]
#[derive(Debug, Clone)]
pub struct WasmPoint {
    pub(crate) inner: Point,
}

#[wasm_bindgen(js_class = Point)]
impl WasmPoint {
    /// Create a new point
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: Point::new(x, y, z),
        }
    }

    /// Get X coordinate
    #[wasm_bindgen(getter, js_name = x)]
    pub fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y coordinate
    #[wasm_bindgen(getter, js_name = y)]
    pub fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z coordinate
    #[wasm_bindgen(getter, js_name = z)]
    pub fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Set X coordinate
    #[wasm_bindgen(setter, js_name = x)]
    pub fn set_x(&mut self, x: f64) {
        self.inner.set_x(x);
    }

    /// Set Y coordinate
    #[wasm_bindgen(setter, js_name = y)]
    pub fn set_y(&mut self, y: f64) {
        self.inner.set_y(y);
    }

    /// Set Z coordinate
    #[wasm_bindgen(setter, js_name = z)]
    pub fn set_z(&mut self, z: f64) {
        self.inner.set_z(z);
    }

    /// Distance to another point
    #[wasm_bindgen(js_name = distanceTo)]
    pub fn distance_to(&self, other: &WasmPoint) -> f64 {
        self.inner.distance_to(&other.inner)
    }

    /// Add a vector to this point
    #[wasm_bindgen(js_name = add)]
    pub fn add(&self, vec: &WasmVector) -> WasmPoint {
        WasmPoint {
            inner: self.inner.add(&vec.inner),
        }
    }

    /// Subtract another point from this point
    #[wasm_bindgen(js_name = sub)]
    pub fn sub(&self, other: &WasmPoint) -> WasmVector {
        WasmVector {
            inner: self.inner.sub(&other.inner),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Point({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// WebAssembly wrapper for Vector
#[wasm_bindgen(js_name = Vector)]
#[derive(Debug, Clone)]
pub struct WasmVector {
    pub(crate) inner: Vector,
}

#[wasm_bindgen(js_class = Vector)]
impl WasmVector {
    /// Create a new vector
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: Vector::new(x, y, z),
        }
    }

    /// Get X component
    #[wasm_bindgen(getter, js_name = x)]
    pub fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y component
    #[wasm_bindgen(getter, js_name = y)]
    pub fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z component
    #[wasm_bindgen(getter, js_name = z)]
    pub fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Get vector magnitude
    #[wasm_bindgen(js_name = magnitude)]
    pub fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    /// Normalize the vector
    #[wasm_bindgen(js_name = normalized)]
    pub fn normalized(&self) -> WasmVector {
        WasmVector {
            inner: self.inner.normalized(),
        }
    }

    /// Dot product with another vector
    #[wasm_bindgen(js_name = dot)]
    pub fn dot(&self, other: &WasmVector) -> f64 {
        self.inner.dot(&other.inner)
    }

    /// Cross product with another vector
    #[wasm_bindgen(js_name = cross)]
    pub fn cross(&self, other: &WasmVector) -> WasmVector {
        WasmVector {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// Add another vector
    #[wasm_bindgen(js_name = add)]
    pub fn add(&self, other: &WasmVector) -> WasmVector {
        WasmVector {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Scale the vector
    #[wasm_bindgen(js_name = scale)]
    pub fn scale(&self, factor: f64) -> WasmVector {
        WasmVector {
            inner: self.inner.scaled(factor),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Vector({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// WebAssembly wrapper for Direction
#[wasm_bindgen(js_name = Direction)]
#[derive(Debug, Clone)]
pub struct WasmDirection {
    pub(crate) inner: Direction,
}

#[wasm_bindgen(js_class = Direction)]
impl WasmDirection {
    /// Create a new direction
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Result<WasmDirection, JsValue> {
        let dir = Direction::new(x, y, z);
        Ok(Self { inner: dir })
    }

    /// Get X component
    #[wasm_bindgen(getter, js_name = x)]
    pub fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y component
    #[wasm_bindgen(getter, js_name = y)]
    pub fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z component
    #[wasm_bindgen(getter, js_name = z)]
    pub fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Reversed direction
    #[wasm_bindgen(js_name = reversed)]
    pub fn reversed(&self) -> WasmDirection {
        WasmDirection {
            inner: self.inner.reversed(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Direction({}, {}, {})", self.x(), self.y(), self.z())
    }
}

/// WebAssembly wrapper for Axis
#[wasm_bindgen(js_name = Axis)]
#[derive(Debug, Clone)]
pub struct WasmAxis {
    pub(crate) inner: Axis,
}

#[wasm_bindgen(js_class = Axis)]
impl WasmAxis {
    /// Create a new axis
    #[wasm_bindgen(constructor)]
    pub fn new(origin: &WasmPoint, direction: &WasmDirection) -> Self {
        Self {
            inner: Axis::new(origin.inner.clone(), direction.inner.clone()),
        }
    }

    /// Get origin point
    #[wasm_bindgen(getter, js_name = origin)]
    pub fn origin(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.location().clone(),
        }
    }

    /// Get direction
    #[wasm_bindgen(getter, js_name = direction)]
    pub fn direction(&self) -> WasmDirection {
        WasmDirection {
            inner: self.inner.direction().clone(),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "Axis(origin={}, direction={})",
            self.origin().to_string(),
            self.direction().to_string()
        )
    }
}

/// WebAssembly wrapper for Plane
#[wasm_bindgen(js_name = Plane)]
#[derive(Debug, Clone)]
pub struct WasmPlane {
    pub(crate) inner: Plane,
}

#[wasm_bindgen(js_class = Plane)]
impl WasmPlane {
    /// Create a plane from point and normal
    #[wasm_bindgen(constructor)]
    pub fn from_point_normal(point: &WasmPoint, normal: &WasmDirection) -> Self {
        Self {
            inner: Plane::from_point_normal(point.inner.clone(), normal.inner.clone()),
        }
    }

    /// Get origin point
    #[wasm_bindgen(getter, js_name = origin)]
    pub fn origin(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.location().clone(),
        }
    }
    pub fn signed_distance_to(&self, point: &WasmPoint) -> f64 {
        self.inner.distance(&point.inner)
    }
    pub fn project_point(&self, point: &WasmPoint) -> WasmPoint {
        // Project point onto plane
        let d = self.inner.distance(&point.inner);
        let normal = self.inner.direction();
        let projected = crate::geometry::Point::new(
            point.inner.x - d * normal.x,
            point.inner.y - d * normal.y,
            point.inner.z - d * normal.z,
        );
        WasmPoint { inner: projected }
    }

    /// Get normal direction
    #[wasm_bindgen(getter, js_name = normal)]
    pub fn normal(&self) -> WasmDirection {
        WasmDirection {
            inner: self.inner.normal().clone(),
        }
    }

    /// Distance from point to plane
    #[wasm_bindgen(js_name = distanceTo)]
    pub fn distance_to(&self, point: &WasmPoint) -> f64 {
        self.inner.signed_distance_to(&point.inner)
    }

    /// Project point onto plane
    #[wasm_bindgen(js_name = project)]
    pub fn project(&self, point: &WasmPoint) -> WasmPoint {
        WasmPoint {
            inner: self.inner.project_point(&point.inner),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "Plane(origin={}, normal={})",
            self.origin().to_string(),
            self.normal().to_string()
        )
    }
}
