#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]
//! Topology Explorer Compatibility Module
//!
//! Provides OpenCASCADE-compatible TopExp API for topology exploration.

use crate::foundation::handle::Handle;
use crate::topology::{
    topods_compound::TopoDsCompound,
    topods_edge::TopoDsEdge,
    topods_face::TopoDsFace,
    topods_shape::TopoDsShape,
    topods_shell::TopoDsShell,
    topods_solid::TopoDsSolid,
    topods_vertex::TopoDsVertex,
    topods_wire::TopoDsWire,
    ShapeType,
    TopExpExplorer,
};

/// Topology Explorer for exploring sub-shapes (OpenCASCADE compatible)
pub struct TopExp_Explorer {
    inner: TopExpExplorer,
}

impl TopExp_Explorer {
    /// Create a new explorer for the given shape and shape type
    pub fn new(S: &TopoDsShape, ToFind: ShapeType) -> Self {
        Self {
            inner: TopExpExplorer::new(S, ToFind),
        }
    }
    
    /// Get the current shape
    pub fn Current(&self) -> Option<&TopoDsShape> {
        self.inner.current()
    }
    
    /// Move to the next shape
    pub fn Next(&mut self) {
        self.inner.next()
    }
    
    /// Check if there are more shapes
    pub fn More(&self) -> bool {
        self.inner.more()
    }
    
    /// Reset the explorer
    pub fn ReInit(&mut self, S: &TopoDsShape) {
        // Note: OpenCascade's ReInit only takes the shape, not the shape type
        // We'll use the same shape type as before
        if let Some(current_shape) = self.inner.current() {
            self.inner.init(S, current_shape.shape_type());
        } else {
            // If no current shape, default to vertex
            self.inner.init(S, ShapeType::Vertex);
        }
    }
    
    /// Get the number of shapes
    pub fn NbShapes(&self) -> usize {
        // Count the number of shapes by iterating through the explorer
        let mut count = 0;
        if let Some(current_shape) = self.inner.current() {
            let mut explorer = TopExpExplorer::new(current_shape, current_shape.shape_type());
            while explorer.more() {
                explorer.next();
                count += 1;
            }
        }
        count
    }
}

/// Topology Tools for shape analysis (OpenCASCADE compatible)
pub struct TopExp;

impl TopExp {
    /// Count the number of vertices in a shape
    pub fn NbVertices(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Vertex);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Vertex {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Count the number of edges in a shape
    pub fn NbEdges(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Edge);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Edge {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Count the number of wires in a shape
    pub fn NbWires(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Wire);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Wire {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Count the number of faces in a shape
    pub fn NbFaces(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Face);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Face {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Count the number of shells in a shape
    pub fn NbShells(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Shell);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Shell {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Count the number of solids in a shape
    pub fn NbSolids(S: &TopoDsShape) -> usize {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Solid);
        let mut count = 0;
        while explorer.more() {
            explorer.next();
            if let Some(current) = explorer.current() {
                if current.shape_type() == ShapeType::Solid {
                    count += 1;
                }
            }
        }
        count
    }
    
    /// Get the first vertex of a shape
    pub fn FirstVertex(S: &TopoDsShape) -> Option<Handle<TopoDsVertex>> {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Vertex);
        if explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_vertex() {
                    return Some(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsVertex)).clone())
                    }));
                }
            }
        }
        None
    }
    
    /// Get the last vertex of a shape
    pub fn LastVertex(S: &TopoDsShape) -> Option<Handle<TopoDsVertex>> {
        let mut explorer = TopExpExplorer::new(S, ShapeType::Vertex);
        let mut last_vertex = None;
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_vertex() {
                    last_vertex = Some(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsVertex)).clone())
                    }));
                }
            }
        }
        last_vertex
    }
    
    /// Get the vertices of a shape
    pub fn Vertices(S: &TopoDsShape) -> Vec<Handle<TopoDsVertex>> {
        let mut vertices = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Vertex);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_vertex() {
                    vertices.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsVertex)).clone())
                    }));
                }
            }
        }
        vertices
    }
    
    /// Get the edges of a shape
    pub fn Edges(S: &TopoDsShape) -> Vec<Handle<TopoDsEdge>> {
        let mut edges = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Edge);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_edge() {
                    edges.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsEdge)).clone())
                    }));
                }
            }
        }
        edges
    }
    
    /// Get the wires of a shape
    pub fn Wires(S: &TopoDsShape) -> Vec<Handle<TopoDsWire>> {
        let mut wires = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Wire);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_wire() {
                    wires.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsWire)).clone())
                    }));
                }
            }
        }
        wires
    }
    
    /// Get the faces of a shape
    pub fn Faces(S: &TopoDsShape) -> Vec<Handle<TopoDsFace>> {
        let mut faces = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Face);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_face() {
                    faces.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsFace)).clone())
                    }));
                }
            }
        }
        faces
    }
    
    /// Get the shells of a shape
    pub fn Shells(S: &TopoDsShape) -> Vec<Handle<TopoDsShell>> {
        let mut shells = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Shell);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_shell() {
                    shells.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsShell)).clone())
                    }));
                }
            }
        }
        shells
    }
    
    /// Get the solids of a shape
    pub fn Solids(S: &TopoDsShape) -> Vec<Handle<TopoDsSolid>> {
        let mut solids = Vec::new();
        let mut explorer = TopExpExplorer::new(S, ShapeType::Solid);
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.is_solid() {
                    solids.push(Handle::new(unsafe {
                        std::sync::Arc::new((*(shape as *const TopoDsShape as *const TopoDsSolid)).clone())
                    }));
                }
            }
        }
        solids
    }
    
    /// Check if a shape contains another shape
    pub fn Contains(S1: &TopoDsShape, S2: &TopoDsShape) -> bool {
        let mut explorer = TopExpExplorer::new(S1, S2.shape_type());
        while explorer.more() {
            explorer.next();
            if let Some(shape) = explorer.current() {
                if shape.shape_id() == S2.shape_id() {
                    return true;
                }
            }
        }
        false
    }
    
    /// Map vertices from one shape to another
    pub fn MapVertices(S1: &TopoDsShape, S2: &TopoDsShape) -> Vec<(Handle<TopoDsVertex>, Handle<TopoDsVertex>)> {
        let vertices1 = Self::Vertices(S1);
        let vertices2 = Self::Vertices(S2);
        let mut mappings = Vec::new();
        
        for v1 in &vertices1 {
            for v2 in &vertices2 {
                if (v1.point().x - v2.point().x).abs() < 1e-7 &&
                   (v1.point().y - v2.point().y).abs() < 1e-7 &&
                   (v1.point().z - v2.point().z).abs() < 1e-7 {
                    mappings.push((v1.clone(), v2.clone()));
                }
            }
        }
        
        mappings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::modeling::brep_builder::BrepBuilder;
    
    #[test]
    fn test_explorer_creation() {
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let v2 = builder.make_vertex(p2);
        let edge = builder.make_edge(v1, v2);
        let shape = edge.shape();
        
        let explorer = TopExp_Explorer::new(shape, ShapeType::Vertex);
        assert!(explorer.More());
    }
    
    #[test]
    fn test_topexp_nb_vertices() {
        let builder = BrepBuilder::new();
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 0.0, 0.0);
        let v1 = builder.make_vertex(p1);
        let v2 = builder.make_vertex(p2);
        let edge = builder.make_edge(v1, v2);
        let shape = edge.shape();
        
        assert_eq!(TopExp::NbVertices(shape), 2);
    }
}
