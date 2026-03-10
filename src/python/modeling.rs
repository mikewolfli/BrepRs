/// Python wrapper for Compound
#[pyclass(name = "Compound")]
#[derive(Debug, Clone)]
pub struct PyCompound {
    pub(crate) inner: Handle<crate::topology::topods_compound::TopoDsCompound>,
}

#[pymethods]
impl PyCompound {
    /// String representation
    fn __repr__(&self) -> String {
        "Compound()".to_string()
    }
}
/// Python bindings for modeling operations
use super::geometry::PyPoint;
use super::topology::{PyEdge, PyFace, PyShell, PySolid, PyVertex, PyWire};
use crate::foundation::handle::Handle;
use crate::modeling::{
    boolean_operations::BooleanOperations, brep_builder::BrepBuilder,
    fillet_chamfer::FilletChamfer, offset_operations::OffsetOperations,
};
use pyo3::prelude::*;

/// Python wrapper for BRep Builder
#[pyclass(name = "BrepBuilder")]
#[derive(Debug, Clone)]
pub struct PyBrepBuilder {
    inner: BrepBuilder,
}

#[pymethods]
impl PyBrepBuilder {
    /// Create a new BRep builder
    #[new]
    fn new() -> Self {
        Self {
            inner: BrepBuilder::new(),
        }
    }

    /// Create a vertex
    fn make_vertex(&self, point: &PyPoint) -> PyVertex {
        PyVertex {
            inner: self.inner.make_vertex(point.inner.clone()),
        }
    }

    /// Create an edge from two vertices
    fn make_edge(&self, v1: &PyVertex, v2: &PyVertex) -> PyEdge {
        PyEdge {
            inner: self.inner.make_edge(v1.inner.clone(), v2.inner.clone()),
        }
    }

    /// Create a wire
    fn make_wire(&self) -> PyWire {
        PyWire {
            inner: self.inner.make_wire(),
        }
    }

    /// Create a face
    fn make_face(&self) -> PyFace {
        PyFace {
            inner: self.inner.make_face(),
        }
    }

    /// Create a shell
    fn make_shell(&self) -> PyShell {
        PyShell {
            inner: self.inner.make_shell(),
        }
    }

    /// Create a solid
    fn make_solid(&self) -> PySolid {
        PySolid {
            inner: self.inner.make_solid(),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        "BrepBuilder()".to_string()
    }
}

impl Default for PyBrepBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Python wrapper for Boolean Operations
#[pyclass(name = "BooleanOperations")]
#[derive(Debug, Clone)]
pub struct PyBooleanOperations {
    inner: BooleanOperations,
}

#[pymethods]
impl PyBooleanOperations {
    /// Create a new boolean operations instance
    #[new]
    fn new() -> Self {
        Self {
            inner: BooleanOperations::new(),
        }
    }

    /// Fuse (union) two solids
    fn fuse(&self, solid1: &PySolid, solid2: &PySolid) -> PyCompound {
        let shape1 = Handle::new(std::sync::Arc::new(
            solid1
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let shape2 = Handle::new(std::sync::Arc::new(
            solid2
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let result = self.inner.fuse(&shape1, &shape2);
        PyCompound {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// Cut (subtract) solid2 from solid1
    fn cut(&self, solid1: &PySolid, solid2: &PySolid) -> PyCompound {
        let shape1 = Handle::new(std::sync::Arc::new(
            solid1
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let shape2 = Handle::new(std::sync::Arc::new(
            solid2
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let result = self.inner.cut(&shape1, &shape2);
        PyCompound {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// Common (intersection) of two solids
    fn common(&self, solid1: &PySolid, solid2: &PySolid) -> PyCompound {
        let shape1 = Handle::new(std::sync::Arc::new(
            solid1
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let shape2 = Handle::new(std::sync::Arc::new(
            solid2
                .inner
                .as_ref()
                .expect("PySolid missing inner")
                .shape()
                .clone(),
        ));
        let result = self.inner.common(&shape1, &shape2);
        PyCompound {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        "BooleanOperations()".to_string()
    }
}

impl Default for PyBooleanOperations {
    fn default() -> Self {
        Self::new()
    }
}

/// Python wrapper for Fillet and Chamfer operations
#[pyclass(name = "FilletChamfer")]
#[derive(Debug, Clone)]
pub struct PyFilletChamfer {
    inner: FilletChamfer,
}

#[pymethods]
impl PyFilletChamfer {
    /// Create a new fillet/chamfer instance
    #[new]
    fn new() -> Self {
        Self {
            inner: FilletChamfer::new(),
        }
    }

    /// Create with specific radius
    #[staticmethod]
    fn with_radius(radius: f64) -> Self {
        Self {
            inner: FilletChamfer::with_radius(radius),
        }
    }

    /// Create with specific chamfer distance
    #[staticmethod]
    fn with_chamfer_distance(distance: f64) -> Self {
        Self {
            inner: FilletChamfer::with_chamfer_distance(distance),
        }
    }

    /// Set radius
    fn set_radius(&mut self, radius: f64) {
        self.inner.set_radius(radius);
    }

    /// Get radius
    #[getter]
    fn radius(&self) -> f64 {
        self.inner.radius()
    }

    /// Set chamfer distance
    fn set_chamfer_distance(&mut self, distance: f64) {
        self.inner.set_chamfer_distance(distance);
    }

    /// Get chamfer distance
    #[getter]
    fn chamfer_distance(&self) -> f64 {
        self.inner.chamfer_distance()
    }

    /// Apply fillet to a solid
    fn apply_fillet(&self, solid: &PySolid) -> PySolid {
        let result = self.inner.apply_fillet(&solid.inner);
        PySolid {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// Apply chamfer to a solid
    fn apply_chamfer(&self, solid: &PySolid) -> PySolid {
        let result = self.inner.apply_chamfer(&solid.inner);
        PySolid {
            inner: Handle::new(std::sync::Arc::new(result)),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "FilletChamfer(radius={}, chamfer_distance={})",
            self.radius(),
            self.chamfer_distance()
        )
    }
}

impl Default for PyFilletChamfer {
    fn default() -> Self {
        Self::new()
    }
}

/// Python wrapper for Offset Operations
#[pyclass(name = "OffsetOperations")]
#[derive(Debug, Clone)]
pub struct PyOffsetOperations {
    inner: OffsetOperations,
}

#[pymethods]
impl PyOffsetOperations {
    /// Create a new offset operations instance
    #[new]
    fn new() -> Self {
        Self {
            inner: OffsetOperations::new(),
        }
    }

    /// Create with specific offset distance
    #[staticmethod]
    fn with_offset_distance(distance: f64) -> Self {
        Self {
            inner: OffsetOperations::with_offset_distance(distance),
        }
    }

    /// Set offset distance
    fn set_offset_distance(&mut self, distance: f64) {
        self.inner.set_offset_distance(distance);
    }

    /// Get offset distance
    #[getter]
    fn offset_distance(&self) -> f64 {
        self.inner.offset_distance()
    }

    /// Set tolerance
    fn set_tolerance(&mut self, tolerance: f64) {
        self.inner.set_tolerance(tolerance);
    }

    /// Get tolerance
    #[getter]
    fn tolerance(&self) -> f64 {
        self.inner.tolerance()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "OffsetOperations(offset_distance={}, tolerance={})",
            self.offset_distance(),
            self.tolerance()
        )
    }
}

impl Default for PyOffsetOperations {
    fn default() -> Self {
        Self::new()
    }
}
