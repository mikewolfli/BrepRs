//! Topology module
//!
//! This module provides the topological data structures for boundary representation (BRep) modeling.
//! It defines the hierarchical relationships between geometric elements such as vertices, edges, wires,
//! faces, shells, and solids.
//!
//! # Main Components
//!
//! - **TopoDsShape**: Base class for all topological shapes
//! - **TopoDsVertex**: Zero-dimensional topological element (point)
//! - **TopoDsEdge**: One-dimensional topological element (curve)
//! - **TopoDsWire**: Ordered set of edges forming a closed or open boundary
//! - **TopoDsFace**: Two-dimensional topological element (surface)
//! - **TopoDsShell**: Set of connected faces
//! - **TopoDsSolid**: Three-dimensional topological element (volume)
//! - **TopoDsCompound**: Collection of shapes of different types
//! - **TopoDsCompSolid**: Set of solids sharing common faces
//! - **ShapeType**: Enumeration of all shape types
//! - **TopExpExplorer**: Tool for exploring topological structure
//! - **TopExpTools**: Collection of topological tools
//! - **BrepModel**: Complete BRep model with all shapes and relationships
//!
//! # Topological Hierarchy
//!
//! ```text
//! Compound
//!   ├─ CompSolid
//!   │   └─ Solid
//!   │       └─ Shell
//!   │           └─ Face
//!   │               └─ Wire
//!   │                   └─ Edge
//!   │                       └─ Vertex
//!   └─ Face
//!       └─ Wire
//!           └─ Edge
//!               └─ Vertex
//! ```
//!
//! # Example
//!
//! ```rust
//! use breprs::topology::{TopoDsVertex, TopoDsEdge, TopoDsWire, TopoDsFace};
//!
//! // Create a simple face from vertices
//! let v1 = TopoDsVertex::new(point1, 1e-6);
//! let v2 = TopoDsVertex::new(point2, 1e-6);
//! let edge = TopoDsEdge::new(v1, v2, curve);
//! let wire = TopoDsWire::new(vec![edge]);
//! let face = TopoDsFace::new(wire);
//! ```

pub mod brep;
pub mod shape_enum;
pub mod top_exp_explorer;
pub mod top_exp_tools;
pub mod top_tools;
pub mod topods_compound;
pub mod topods_compsolid;
pub mod topods_edge;
pub mod topods_face;
pub mod topods_location;
pub mod topods_shape;
pub mod topods_shell;
pub mod topods_solid;
pub mod topods_vertex;
pub mod topods_wire;
pub mod validation;

pub use brep::{BrepModel, BrepTopology};
pub use shape_enum::ShapeType;
pub use top_exp_explorer::TopExpExplorer;
pub use top_exp_tools::{TopExpTools, TopToolsAnalyzer};
pub use topods_compound::{TopoDsCompSolid, TopoDsCompound};
pub use topods_edge::{Curve, TopoDsEdge};
pub use topods_face::{Surface, TopoDsFace};
pub use topods_location::TopoDsLocation;
pub use topods_shape::TopoDsShape;
pub use topods_shell::TopoDsShell;
pub use topods_solid::TopoDsSolid;
pub use topods_vertex::TopoDsVertex;
pub use topods_wire::TopoDsWire;
