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

/// WebAssembly wrapper for CurveEnum
#[wasm_bindgen(js_name = Curve)]
#[derive(Debug, Clone)]
pub struct WasmCurve {
    pub(crate) inner: crate::geometry::CurveEnum,
}

#[wasm_bindgen(js_class = Curve)]
impl WasmCurve {
    /// Create a line curve
    #[wasm_bindgen(js_name = line)]
    pub fn line(start: &WasmPoint, end: &WasmPoint) -> Self {
        let dir = crate::geometry::Vector::from_point(&start.inner, &end.inner).to_dir();
        let curve = crate::geometry::CurveEnum::Line(
            crate::geometry::line::Line::new(start.inner.clone(), dir)
        );
        Self { inner: curve }
    }

    /// Create a circle curve
    #[wasm_bindgen(js_name = circle)]
    pub fn circle(center: &WasmPoint, radius: f64, normal: &WasmDirection) -> Self {
        let curve = crate::geometry::CurveEnum::Circle(
            crate::geometry::circle::Circle::new(center.inner.clone(), normal.inner.clone(), radius)
        );
        Self { inner: curve }
    }

    /// Create an ellipse curve
    #[wasm_bindgen(js_name = ellipse)]
    pub fn ellipse(center: &WasmPoint, major_radius: f64, minor_radius: f64, normal: &WasmDirection) -> Self {
        let curve = crate::geometry::CurveEnum::Ellipse(
            crate::geometry::ellipse::Ellipse::new(center.inner.clone(), normal.inner.clone(), major_radius, minor_radius)
        );
        Self { inner: curve }
    }

    /// Evaluate curve at parameter t
    #[wasm_bindgen(js_name = evaluate)]
    pub fn evaluate(&self, t: f64) -> WasmPoint {
        let point = self.inner.value(t);
        WasmPoint { inner: point }
    }

    /// Get curve length
    #[wasm_bindgen(js_name = length)]
    pub fn length(&self) -> f64 {
        self.inner.length()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Curve".to_string()
    }
}

/// WebAssembly wrapper for SurfaceEnum
#[wasm_bindgen(js_name = Surface)]
#[derive(Debug, Clone)]
pub struct WasmSurface {
    pub(crate) inner: crate::geometry::SurfaceEnum,
}

#[wasm_bindgen(js_class = Surface)]
impl WasmSurface {
    /// Create a plane surface
    #[wasm_bindgen(js_name = plane)]
    pub fn plane(origin: &WasmPoint, normal: &WasmDirection) -> Self {
        let x_dir = if normal.inner.is_parallel(&crate::geometry::Direction::z_axis(), 0.001) {
            crate::geometry::Direction::x_axis()
        } else {
            normal.inner.cross(&crate::geometry::Direction::z_axis()).normalized()
        };
        let surface = crate::geometry::SurfaceEnum::Plane(
            crate::geometry::plane::Plane::new(origin.inner.clone(), normal.inner.clone(), x_dir)
        );
        Self { inner: surface }
    }

    /// Create a cylindrical surface
    #[wasm_bindgen(js_name = cylinder)]
    pub fn cylinder(location: &WasmAxis, radius: f64) -> Self {
        let surface = crate::geometry::SurfaceEnum::Cylinder(
            crate::geometry::cylinder::Cylinder::from_axis(&location.inner, radius)
        );
        Self { inner: surface }
    }

    /// Create a spherical surface
    #[wasm_bindgen(js_name = sphere)]
    pub fn sphere(center: &WasmPoint, radius: f64) -> Self {
        let surface = crate::geometry::SurfaceEnum::Sphere(
            crate::geometry::sphere::Sphere::new(center.inner.clone(), radius)
        );
        Self { inner: surface }
    }

    /// Create a conical surface
    #[wasm_bindgen(js_name = cone)]
    pub fn cone(location: &WasmAxis, radius: f64, semi_angle: f64) -> Self {
        let surface = crate::geometry::SurfaceEnum::Cone(
            crate::geometry::cone::Cone::from_axis(&location.inner, semi_angle, radius)
        );
        Self { inner: surface }
    }

    /// Create a toroidal surface
    #[wasm_bindgen(js_name = torus)]
    pub fn torus(location: &WasmAxis, major_radius: f64, minor_radius: f64) -> Self {
        let surface = crate::geometry::SurfaceEnum::Torus(
            crate::geometry::torus::Torus::new(
                location.inner.location().clone(),
                location.inner.direction().clone(),
                major_radius,
                minor_radius
            )
        );
        Self { inner: surface }
    }

    /// Evaluate surface at parameters u, v
    #[wasm_bindgen(js_name = evaluate)]
    pub fn evaluate(&self, u: f64, v: f64) -> WasmPoint {
        let point = self.inner.value(u, v);
        WasmPoint { inner: point }
    }

    /// Get surface normal at parameters u, v
    #[wasm_bindgen(js_name = normal)]
    pub fn normal(&self, u: f64, v: f64) -> WasmDirection {
        let vector = self.inner.normal(u, v);
        WasmDirection {
            inner: crate::geometry::Direction::from_vector(&vector),
        }
    }

    /// Get surface area
    #[wasm_bindgen(js_name = area)]
    pub fn area(&self) -> f64 {
        self.inner.area()
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        "Surface".to_string()
    }
}

/// WebAssembly wrapper for Transform
#[wasm_bindgen(js_name = Transformation)]
#[derive(Debug, Clone)]
pub struct WasmTransformation {
    pub(crate) inner: crate::geometry::Transform,
}

#[wasm_bindgen(js_class = Transformation)]
impl WasmTransformation {
    /// Create identity transformation
    #[wasm_bindgen(js_name = identity)]
    pub fn identity() -> Self {
        Self {
            inner: crate::geometry::Transform::identity(),
        }
    }

    /// Create translation transformation
    #[wasm_bindgen(js_name = translation)]
    pub fn translation(vector: &WasmVector) -> Self {
        Self {
            inner: crate::geometry::Transform::from_translation(&vector.inner),
        }
    }

    /// Create rotation transformation around X axis
    #[wasm_bindgen(js_name = rotationX)]
    pub fn rotation_x(angle: f64) -> Self {
        let axis = crate::geometry::Axis::new(
            crate::geometry::Point::origin(),
            crate::geometry::Direction::x_axis()
        );
        Self {
            inner: crate::geometry::Transform::from_rotation(&axis, angle),
        }
    }

    /// Create rotation transformation around Y axis
    #[wasm_bindgen(js_name = rotationY)]
    pub fn rotation_y(angle: f64) -> Self {
        let axis = crate::geometry::Axis::new(
            crate::geometry::Point::origin(),
            crate::geometry::Direction::y_axis()
        );
        Self {
            inner: crate::geometry::Transform::from_rotation(&axis, angle),
        }
    }

    /// Create rotation transformation around Z axis
    #[wasm_bindgen(js_name = rotationZ)]
    pub fn rotation_z(angle: f64) -> Self {
        let axis = crate::geometry::Axis::new(
            crate::geometry::Point::origin(),
            crate::geometry::Direction::z_axis()
        );
        Self {
            inner: crate::geometry::Transform::from_rotation(&axis, angle),
        }
    }

    /// Create rotation transformation around arbitrary axis
    #[wasm_bindgen(js_name = rotation)]
    pub fn rotation(axis: &WasmAxis, angle: f64) -> Self {
        Self {
            inner: crate::geometry::Transform::from_rotation(&axis.inner, angle),
        }
    }

    /// Create scaling transformation
    #[wasm_bindgen(js_name = scaling)]
    pub fn scaling(sx: f64, _sy: f64, _sz: f64) -> Self {
        Self {
            inner: crate::geometry::Transform::from_scale(sx),
        }
    }

    /// Create uniform scaling transformation
    #[wasm_bindgen(js_name = uniformScaling)]
    pub fn uniform_scaling(scale: f64) -> Self {
        Self {
            inner: crate::geometry::Transform::from_scale(scale),
        }
    }

    /// Multiply transformations
    #[wasm_bindgen(js_name = multiply)]
    pub fn multiply(&self, other: &WasmTransformation) -> WasmTransformation {
        WasmTransformation {
            inner: self.inner.multiply(&other.inner),
        }
    }

    /// Invert transformation
    #[wasm_bindgen(js_name = inverted)]
    pub fn inverted(&self) -> WasmTransformation {
        WasmTransformation {
            inner: self.inner.inverted(),
        }
    }

    /// Apply transformation to point
    #[wasm_bindgen(js_name = transformPoint)]
    pub fn transform_point(&self, point: &WasmPoint) -> WasmPoint {
        WasmPoint {
            inner: self.inner.transforms(&point.inner),
        }
    }

    /// Apply transformation to vector
    #[wasm_bindgen(js_name = transformVector)]
    pub fn transform_vector(&self, vector: &WasmVector) -> WasmVector {
        WasmVector {
            inner: self.inner.transforms_vec(&vector.inner),
        }
    }

    /// Get transformation determinant
    #[wasm_bindgen(js_name = determinant)]
    pub fn determinant(&self) -> f64 {
        self.inner.scale()
    }

    /// Check if transformation is singular
    #[wasm_bindgen(js_name = isSingular)]
    pub fn is_singular(&self) -> bool {
        self.inner.scale().abs() < 1e-10
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!("Transformation(determinant={})", self.determinant())
    }
}

impl Default for WasmTransformation {
    fn default() -> Self {
        Self::identity()
    }
}

/// WebAssembly wrapper for BoundingBox
#[wasm_bindgen(js_name = BoundingBox)]
#[derive(Debug, Clone)]
pub struct WasmBoundingBox {
    pub(crate) inner: crate::geometry::BoundingBox,
}

#[wasm_bindgen(js_class = BoundingBox)]
impl WasmBoundingBox {
    /// Create bounding box from min and max points
    #[wasm_bindgen(constructor)]
    pub fn new(min_point: &WasmPoint, max_point: &WasmPoint) -> Self {
        Self {
            inner: crate::geometry::BoundingBox::new(min_point.inner.clone(), max_point.inner.clone()),
        }
    }

    /// Get minimum point
    #[wasm_bindgen(getter, js_name = min)]
    pub fn min(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.min.clone(),
        }
    }

    /// Get maximum point
    #[wasm_bindgen(getter, js_name = max)]
    pub fn max(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.max.clone(),
        }
    }

    /// Get center point
    #[wasm_bindgen(js_name = center)]
    pub fn center(&self) -> WasmPoint {
        WasmPoint {
            inner: self.inner.center().clone(),
        }
    }

    /// Get size
    #[wasm_bindgen(js_name = size)]
    pub fn size(&self) -> WasmVector {
        let (sx, sy, sz) = self.inner.size();
        WasmVector {
            inner: crate::geometry::Vector::new(sx, sy, sz),
        }
    }

    /// Get volume
    #[wasm_bindgen(js_name = volume)]
    pub fn volume(&self) -> f64 {
        self.inner.volume()
    }

    /// Check if point is inside bounding box
    #[wasm_bindgen(js_name = containsPoint)]
    pub fn contains_point(&self, point: &WasmPoint) -> bool {
        self.inner.contains(&point.inner)
    }

    /// Expand bounding box
    #[wasm_bindgen(js_name = expand)]
    pub fn expand(&mut self, delta: f64) {
        self.inner.expand(delta);
    }

    /// Merge with another bounding box
    #[wasm_bindgen(js_name = merge)]
    pub fn merge(&self, other: &WasmBoundingBox) -> WasmBoundingBox {
        WasmBoundingBox {
            inner: self.inner.merge(&other.inner),
        }
    }

    /// Convert to string
    #[wasm_bindgen(js_name = toString)]
    pub fn to_string(&self) -> String {
        format!(
            "BoundingBox(min={}, max={}, volume={})",
            self.min().to_string(),
            self.max().to_string(),
            self.volume()
        )
    }
}

impl Default for WasmBoundingBox {
    fn default() -> Self {
        Self {
            inner: crate::geometry::BoundingBox::default(),
        }
    }
}
