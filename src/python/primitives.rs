//! Python bindings for primitive creation

use super::geometry::PyPoint;
use super::topology::PySolid;
use crate::foundation::handle::Handle;
use crate::modeling::primitives;
use crate::topology::topods_solid::TopoDsSolid;
use pyo3::prelude::*;

/// Python wrapper for Box primitive
#[pyclass(name = "Box")]
#[derive(Debug, Clone)]
pub struct PyBox {
    pub(crate) inner: Handle<TopoDsSolid>,
    width: f64,
    height: f64,
    depth: f64,
}

#[pymethods]
impl PyBox {
    /// Create a box
    #[new]
    fn new(width: f64, height: f64, depth: f64) -> Self {
        let solid = primitives::make_box(width, height, depth, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            width,
            height,
            depth,
        }
    }

    /// Create a box at a specific position
    #[staticmethod]
    fn at(width: f64, height: f64, depth: f64, position: &PyPoint) -> Self {
        let solid = primitives::make_box(width, height, depth, Some(position.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            width,
            height,
            depth,
        }
    }

    /// Get width
    #[getter]
    fn width(&self) -> f64 {
        self.width
    }

    /// Get height
    #[getter]
    fn height(&self) -> f64 {
        self.height
    }

    /// Get depth
    #[getter]
    fn depth(&self) -> f64 {
        self.depth
    }

    /// Get volume
    fn volume(&self) -> f64 {
        self.width * self.height * self.depth
    }

    /// Convert to solid
    fn to_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Box({}, {}, {})", self.width, self.height, self.depth)
    }
}

/// Python wrapper for Sphere primitive
#[pyclass(name = "Sphere")]
#[derive(Debug, Clone)]
pub struct PySphere {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius: f64,
}

#[pymethods]
impl PySphere {
    /// Create a sphere
    #[new]
    fn new(radius: f64) -> Self {
        let solid = primitives::make_sphere(radius, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
        }
    }

    /// Create a sphere at a specific position
    #[staticmethod]
    fn at(radius: f64, center: &PyPoint) -> Self {
        let solid = primitives::make_sphere(radius, Some(center.inner.clone()));
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
        }
    }

    /// Get radius
    #[getter]
    fn radius(&self) -> f64 {
        self.radius
    }

    /// Get volume
    fn volume(&self) -> f64 {
        (4.0 / 3.0) * std::f64::consts::PI * self.radius.powi(3)
    }

    /// Get surface area
    fn surface_area(&self) -> f64 {
        4.0 * std::f64::consts::PI * self.radius.powi(2)
    }

    /// Convert to solid
    fn to_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Sphere({})", self.radius)
    }
}

/// Python wrapper for Cylinder primitive
#[pyclass(name = "Cylinder")]
#[derive(Debug, Clone)]
pub struct PyCylinder {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius: f64,
    height: f64,
}

#[pymethods]
impl PyCylinder {
    /// Create a cylinder
    #[new]
    fn new(radius: f64, height: f64) -> Self {
        let solid = primitives::make_cylinder(radius, height, None, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
            height,
        }
    }

    /// Create a cylinder at a specific position
    #[staticmethod]
    fn at(radius: f64, height: f64, position: &PyPoint) -> Self {
        let solid = primitives::make_cylinder(radius, height, Some(position.inner.clone()), None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius,
            height,
        }
    }

    /// Get radius
    #[getter]
    fn radius(&self) -> f64 {
        self.radius
    }

    /// Get height
    #[getter]
    fn height(&self) -> f64 {
        self.height
    }

    /// Get volume
    fn volume(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2) * self.height
    }

    /// Convert to solid
    fn to_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Cylinder({}, {})", self.radius, self.height)
    }
}

/// Python wrapper for Cone primitive
#[pyclass(name = "Cone")]
#[derive(Debug, Clone)]
pub struct PyCone {
    pub(crate) inner: Handle<TopoDsSolid>,
    radius1: f64,
    radius2: f64,
    height: f64,
}

#[pymethods]
impl PyCone {
    /// Create a cone
    #[new]
    fn new(radius1: f64, radius2: f64, height: f64) -> Self {
        let solid = primitives::make_cone(radius1, radius2, height, None, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            radius1,
            radius2,
            height,
        }
    }

    /// Get radius1
    #[getter]
    fn radius1(&self) -> f64 {
        self.radius1
    }

    /// Get radius2
    #[getter]
    fn radius2(&self) -> f64 {
        self.radius2
    }

    /// Get height
    #[getter]
    fn height(&self) -> f64 {
        self.height
    }

    /// Convert to solid
    fn to_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Cone({}, {}, {})", self.radius1, self.radius2, self.height)
    }
}

/// Python wrapper for Torus primitive
#[pyclass(name = "Torus")]
#[derive(Debug, Clone)]
pub struct PyTorus {
    pub(crate) inner: Handle<TopoDsSolid>,
    major_radius: f64,
    minor_radius: f64,
}

#[pymethods]
impl PyTorus {
    /// Create a torus
    #[new]
    fn new(major_radius: f64, minor_radius: f64) -> Self {
        let solid = primitives::make_torus(major_radius, minor_radius, None);
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
            major_radius,
            minor_radius,
        }
    }

    /// Get major radius
    #[getter]
    fn major_radius(&self) -> f64 {
        self.major_radius
    }

    /// Get minor radius
    #[getter]
    fn minor_radius(&self) -> f64 {
        self.minor_radius
    }

    /// Convert to solid
    fn to_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.clone(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Torus({}, {})", self.major_radius, self.minor_radius)
    }
}
