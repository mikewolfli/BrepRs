#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! OpenCASCADE Topology Compatibility Module
//!
//! Provides OpenCASCADE-compatible type aliases and wrappers
//! for topological entities.

// Re-export all topology types with OpenCASCADE naming
pub use crate::topology::{
    shape_enum::ShapeType, top_exp_explorer::TopExpExplorer,
    topods_compound::TopoDsCompSolid as TopoDS_CompSolid,
    topods_compound::TopoDsCompound as TopoDS_Compound, topods_edge::TopoDsEdge as TopoDS_Edge,
    topods_face::TopoDsFace as TopoDS_Face, topods_location::TopoDsLocation as TopoDS_Location,
    topods_shape::TopoDsShape as TopoDS_Shape, topods_shell::TopoDsShell as TopoDS_Shell,
    topods_solid::TopoDsSolid as TopoDS_Solid, topods_vertex::TopoDsVertex as TopoDS_Vertex,
    topods_wire::TopoDsWire as TopoDS_Wire,
};
