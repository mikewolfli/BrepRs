//! Python Bindings Module
//!
//! This module provides Python bindings for the BrepRs library using PyO3.
//! It enables Python developers to use the CAD kernel functionality.
//!
//! # Example Python Usage
//!
//! ```python
//! import breprs
//!
//! # Create a box
//! box = breprs.Box(10.0, 10.0, 10.0)
//!
//! # Apply fillet
//! filleted = box.fillet(1.0)
//!
//! # Export to STL
//! filleted.to_stl("output.stl")
//! ```

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

pub mod geometry;
pub mod modeling;
pub mod primitives;
pub mod topology;

use geometry::*;
use modeling::*;
use primitives::*;
use topology::PyCompound;
use topology::PyEdge;
use topology::PyFace;
use topology::PyShell;
use topology::PySolid;
use topology::PyVertex;
use topology::PyWire;

/// Python module initialization
#[pymodule]
fn breprs(_py: Python, m: &PyModule) -> PyResult<()> {
    // Version info
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    // Register geometry classes
    m.add_class::<PyPoint>()?;
    m.add_class::<PyVector>()?;
    m.add_class::<PyDirection>()?;
    m.add_class::<PyAxis>()?;
    m.add_class::<PyPlane>()?;

    // Register topology classes
    m.add_class::<PyVertex>()?;
    m.add_class::<PyEdge>()?;
    m.add_class::<PyWire>()?;
    m.add_class::<PyFace>()?;
    m.add_class::<PyShell>()?;
    m.add_class::<PySolid>()?;
    m.add_class::<PyCompound>()?;

    // Register primitive classes
    m.add_class::<PyBox>()?;
    m.add_class::<PySphere>()?;
    m.add_class::<PyCylinder>()?;
    m.add_class::<PyCone>()?;
    m.add_class::<PyTorus>()?;

    // Register modeling classes
    m.add_class::<PyBrepBuilder>()?;
    m.add_class::<PyBooleanOperations>()?;
    m.add_class::<PyFilletChamfer>()?;
    m.add_class::<PyOffsetOperations>()?;

    // Register utility functions
    m.add_wrapped(wrap_pyfunction!(version))?;
    m.add_wrapped(wrap_pyfunction!(set_tolerance))?;
    m.add_wrapped(wrap_pyfunction!(get_tolerance))?;

    Ok(())
}

/// Get the library version
#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Set global tolerance
#[pyfunction]
fn set_tolerance(tol: f64) {
    // In a real implementation, this would set a global tolerance
    let _ = tol;
}

/// Get global tolerance
#[pyfunction]
fn get_tolerance() -> f64 {
    // In a real implementation, this would return the global tolerance
    1e-6
}

/// Convert Python errors to PyErr
pub fn to_py_err<E: std::error::Error>(e: E) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

/// Trait for converting Rust types to Python types
pub trait ToPython {
    fn to_python(&self, py: Python) -> PyObject;
}

/// Trait for converting Python types to Rust types
pub trait FromPython: Sized {
    fn from_python(obj: &PyAny) -> PyResult<Self>;
}
