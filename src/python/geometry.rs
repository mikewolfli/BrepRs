// Removed unused import
use crate::geometry::{Axis, Direction, Plane, Point, Vector};
/// Python bindings for geometry types
use pyo3::prelude::*;

/// Python wrapper for Point
#[pyclass(name = "Point")]
#[derive(Debug, Clone)]
pub struct PyPoint {
    pub(crate) inner: Point,
}

#[pymethods]
impl PyPoint {
    /// Create a new point
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: Point::new(x, y, z),
        }
    }

    /// Get X coordinate
    #[getter]
    fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y coordinate
    #[getter]
    fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z coordinate
    #[getter]
    fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Set X coordinate
    #[setter]
    fn set_x(&mut self, x: f64) {
        self.inner.set_x(x);
    }

    /// Set Y coordinate
    #[setter]
    fn set_y(&mut self, y: f64) {
        self.inner.set_y(y);
    }

    /// Set Z coordinate
    #[setter]
    fn set_z(&mut self, z: f64) {
        self.inner.set_z(z);
    }

    /// Distance to another point
    fn distance_to(&self, other: &PyPoint) -> f64 {
        self.inner.distance(&other.inner)
    }

    /// Add a vector to this point
    fn add(&self, vec: &PyVector) -> PyPoint {
        PyPoint {
            inner: self.inner.add(&vec.inner),
        }
    }

    /// Subtract another point from this point
    fn sub(&self, other: &PyPoint) -> PyVector {
        PyVector {
            inner: self.inner - other.inner.clone(),
        }
    }

    /// String representation
    pub fn __repr__(&self) -> String {
        format!("Point({}, {}, {})", self.x(), self.y(), self.z())
    }

    /// String representation
    pub fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Python wrapper for Vector
#[pyclass(name = "Vector")]
#[derive(Debug, Clone)]
pub struct PyVector {
    pub(crate) inner: Vector,
}

#[pymethods]
impl PyVector {
    /// Create a new vector
    #[new]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            inner: Vector::new(x, y, z),
        }
    }

    /// Get X component
    #[getter]
    fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y component
    #[getter]
    fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z component
    #[getter]
    fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Get vector magnitude
    fn magnitude(&self) -> f64 {
        self.inner.magnitude()
    }

    /// Normalize the vector
    fn normalized(&self) -> PyVector {
        PyVector {
            inner: self.inner.normalized(),
        }
    }

    /// Dot product with another vector
    fn dot(&self, other: &PyVector) -> f64 {
        self.inner.dot(&other.inner)
    }

    /// Cross product with another vector
    fn cross(&self, other: &PyVector) -> PyVector {
        PyVector {
            inner: self.inner.cross(&other.inner),
        }
    }

    /// Add another vector
    fn add(&self, other: &PyVector) -> PyVector {
        PyVector {
            inner: self.inner.add(&other.inner),
        }
    }

    /// Scale the vector
    fn scale(&self, factor: f64) -> PyVector {
        PyVector {
            inner: self.inner.scaled(factor),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Vector({}, {}, {})", self.x(), self.y(), self.z())
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Python wrapper for Direction
#[pyclass(name = "Direction")]
#[derive(Debug, Clone)]
pub struct PyDirection {
    pub(crate) inner: Direction,
}

#[pymethods]
impl PyDirection {
    /// Create a new direction
    #[new]
    fn new(x: f64, y: f64, z: f64) -> PyResult<Self> {
        let dir = Direction::new(x, y, z);
        Ok(Self { inner: dir })
    }

    /// Get X component
    #[getter]
    fn x(&self) -> f64 {
        self.inner.x()
    }

    /// Get Y component
    #[getter]
    fn y(&self) -> f64 {
        self.inner.y()
    }

    /// Get Z component
    #[getter]
    fn z(&self) -> f64 {
        self.inner.z()
    }

    /// Reversed direction
    fn reversed(&self) -> PyDirection {
        PyDirection {
            inner: self.inner.reversed(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Direction({}, {}, {})", self.x(), self.y(), self.z())
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Python wrapper for Axis
#[pyclass(name = "Axis")]
#[derive(Debug, Clone)]
pub struct PyAxis {
    pub(crate) inner: Axis,
}

#[pymethods]
impl PyAxis {
    /// Create a new axis
    #[new]
    fn new(origin: &PyPoint, direction: &PyDirection) -> Self {
        Self {
            inner: Axis::new(origin.inner.clone(), direction.inner.clone()),
        }
    }

    /// Get origin point
    #[getter]
    fn origin(&self) -> PyPoint {
        PyPoint {
            inner: self.inner.location().clone(),
        }
    }

    /// Get direction
    #[getter]
    fn direction(&self) -> PyDirection {
        PyDirection {
            inner: self.inner.direction().clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Axis(origin={}, direction={})",
            self.origin().__repr__(),
            self.direction().__repr__()
        )
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Python wrapper for Plane
#[pyclass(name = "Plane")]
#[derive(Debug, Clone)]
pub struct PyPlane {
    pub(crate) inner: Plane,
}

#[pymethods]
impl PyPlane {
    /// Create a plane from point and normal
    #[new]
    fn from_point_normal(point: &PyPoint, normal: &PyDirection) -> Self {
        Self {
            inner: Plane::from_point_normal(point.inner.clone(), normal.inner.clone()),
        }
    }

    /// Create a plane from three points
    #[staticmethod]
    fn from_points(p1: &PyPoint, p2: &PyPoint, p3: &PyPoint) -> PyResult<Self> {
        match Plane::from_points(p1.inner.clone(), p2.inner.clone(), p3.inner.clone()) {
            Some(plane) => Ok(Self { inner: plane }),
            None => Err(pyo3::exceptions::PyValueError::new_err(
                "Cannot create plane from collinear points",
            )),
        }
    }

    /// Get origin point
    #[getter]
    fn origin(&self) -> PyPoint {
        PyPoint {
            inner: self.inner.origin().clone(),
        }
    }

    /// Get normal direction
    #[getter]
    fn normal(&self) -> PyDirection {
        PyDirection {
            inner: self.inner.normal().clone(),
        }
    }

    /// Distance from point to plane
    fn distance_to(&self, point: &PyPoint) -> f64 {
        self.inner.distance(&point.inner)
    }

    /// Project point onto plane
    fn project(&self, point: &PyPoint) -> PyPoint {
        // Project point onto plane: p_proj = p - n * distance
        let dist = self.inner.distance(&point.inner);
        let normal = self.inner.normal();
        let v = crate::geometry::Vector::new(normal.x, normal.y, normal.z);
        let proj_vec = v * -dist;
        let proj_point = point.inner.translated(&proj_vec);
        PyPoint { inner: proj_point }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Plane(origin={}, normal={})",
            self.origin().__repr__(),
            self.normal().__repr__()
        )
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}
