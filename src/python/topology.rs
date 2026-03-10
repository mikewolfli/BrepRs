//! Python bindings for topology types

use super::geometry::PyPoint;
use crate::foundation::handle::Handle;
use crate::topology::{
    topods_compound::TopoDsCompound, topods_edge::TopoDsEdge, topods_face::TopoDsFace,
    topods_shape::TopoDsShape, topods_shell::TopoDsShell, topods_solid::TopoDsSolid,
    topods_vertex::TopoDsVertex, topods_wire::TopoDsWire,
};
use pyo3::prelude::*;

/// Python wrapper for Vertex
#[pyclass(name = "Vertex")]
#[derive(Debug, Clone)]
pub struct PyVertex {
    pub(crate) inner: Handle<TopoDsVertex>,
}

#[pymethods]
impl PyVertex {
    /// Create a vertex from a point
    #[new]
    fn new(point: PyPoint) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsVertex::new(point.inner.clone()))),
        }
    }

    /// Get the point of this vertex
    #[getter]
    fn point(&self) -> PyPoint {
        PyPoint {
            inner: self.inner.point().clone(),
        }
    }

    /// Set the point of this vertex
    #[setter]
    fn set_point(&mut self, point: PyPoint) {
        self.inner.set_point(point.inner.clone());
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Vertex({})", self.point().__repr__())
    }
}

/// Python wrapper for Edge
#[pyclass(name = "Edge")]
#[derive(Debug, Clone)]
pub struct PyEdge {
    pub(crate) inner: Handle<TopoDsEdge>,
}

#[pymethods]
impl PyEdge {
    /// Create an edge from two vertices
    #[new]
    fn new(v1: &PyVertex, v2: &PyVertex) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsEdge::new(
                v1.inner.clone(),
                v2.inner.clone(),
            ))),
        }
    }

    /// Get the vertices of this edge
    fn vertices(&self) -> Vec<PyVertex> {
        self.inner
            .vertices()
            .iter()
            .map(|v| PyVertex { inner: v.clone() })
            .collect()
    }

    /// Check if this edge is degenerate
    fn is_degenerate(&self) -> bool {
        self.inner.is_degenerate()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Edge({} vertices)", self.vertices().len())
    }
}

/// Python wrapper for Wire
#[pyclass(name = "Wire")]
#[derive(Debug, Clone)]
pub struct PyWire {
    pub(crate) inner: Handle<TopoDsWire>,
}

#[pymethods]
impl PyWire {
    /// Create an empty wire
    #[new]
    fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsWire::new())),
        }
    }

    /// Add an edge to this wire
    fn add_edge(&mut self, edge: &PyEdge) {
        self.inner.add_edge(edge.inner.clone());
    }

    /// Get the edges of this wire
    fn edges(&self) -> Vec<PyEdge> {
        self.inner
            .edges()
            .iter()
            .map(|e| PyEdge { inner: e.clone() })
            .collect()
    }

    /// Check if this wire is closed
    fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Wire({} edges)", self.edges().len())
    }
}

/// Python wrapper for Face
#[pyclass(name = "Face")]
#[derive(Debug, Clone)]
pub struct PyFace {
    pub(crate) inner: Handle<TopoDsFace>,
}

#[pymethods]
impl PyFace {
    /// Create an empty face
    #[new]
    fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsFace::new())),
        }
    }

    /// Create a face from a wire
    #[staticmethod]
    fn from_wire(wire: &PyWire) -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsFace::with_outer_wire(
                (*wire.inner).clone(),
            ))),
        }
    }

    /// Get the wires of this face
    fn wires(&self) -> Vec<PyWire> {
        let mut wires = Vec::new();
        if let Some(outer) = self.inner.outer_wire() {
            wires.push(PyWire {
                inner: outer.clone(),
            });
        }
        wires
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Face({} wires)", self.wires().len())
    }
}

/// Python wrapper for Shell
#[pyclass(name = "Shell")]
#[derive(Debug, Clone)]
pub struct PyShell {
    pub(crate) inner: Handle<TopoDsShell>,
}

#[pymethods]
impl PyShell {
    /// Create an empty shell
    #[new]
    fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsShell::new())),
        }
    }

    /// Add a face to this shell
    fn add_face(&mut self, face: &PyFace) {
        self.inner.add_face(face.inner.clone());
    }

    /// Get the faces of this shell
    fn faces(&self) -> Vec<PyFace> {
        self.inner
            .faces()
            .iter()
            .map(|f| PyFace { inner: f.clone() })
            .collect()
    }

    /// Check if this shell is closed
    fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Shell({} faces)", self.faces().len())
    }
}

/// Python wrapper for Solid
#[pyclass(name = "Solid")]
#[derive(Debug, Clone)]
pub struct PySolid {
    pub(crate) inner: Handle<TopoDsSolid>,
}

#[pymethods]
impl PySolid {
    /// Create an empty solid
    #[new]
    fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsSolid::new())),
        }
    }

    /// Create a solid from a shell
    #[staticmethod]
    fn from_shell(shell: &PyShell) -> Self {
        let mut solid = TopoDsSolid::new();
        solid.set_outer_shell(shell.inner.clone());
        Self {
            inner: Handle::new(std::sync::Arc::new(solid)),
        }
    }

    /// Get the shells of this solid
    fn shells(&self) -> Vec<PyShell> {
        self.inner
            .shells()
            .iter()
            .map(|s| PyShell { inner: s.clone() })
            .collect()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Solid({} shells)", self.shells().len())
    }
}

/// Python wrapper for Compound
#[pyclass(name = "Compound")]
#[derive(Debug, Clone)]
pub struct PyCompound {
    pub(crate) inner: Handle<TopoDsCompound>,
}

#[pymethods]
impl PyCompound {
    /// Create an empty compound
    #[new]
    fn new() -> Self {
        Self {
            inner: Handle::new(std::sync::Arc::new(TopoDsCompound::new())),
        }
    }

    /// Add a shape to this compound
    fn add_shape(&mut self, shape: &PySolid) {
        let shape_handle: Handle<TopoDsShape> =
            Handle::new(std::sync::Arc::new((*shape.inner).clone().shape().clone()));
        self.inner.add_component(shape_handle);
    }

    /// Get the components of this compound
    fn components(&self) -> Vec<PySolid> {
        // Simplified - in reality would need to handle different shape types
        Vec::new()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!("Compound({} components)", self.inner.components().len())
    }
}
