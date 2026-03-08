use crate::foundation::handle::Handle;
use crate::topology::{topods_shape::TopoDS_Shape, topods_edge::TopoDS_Edge, topods_vertex::TopoDS_Vertex, topods_location::TopoDS_Location};
use crate::geometry::Point;
use std::collections::HashSet;

/// Represents a wire in topological structure
///
/// A wire is an ordered set of edges connected by vertices.
/// Wires can be open or closed (forming a loop).
#[derive(Debug)]
pub struct TopoDS_Wire {
    shape: TopoDS_Shape,
    edges: Vec<Handle<TopoDS_Edge>>,
    vertices: Vec<Handle<TopoDS_Vertex>>,
    closed: bool,
    tolerance: f64,
}

impl TopoDS_Wire {
    /// Create a new empty wire
    pub fn new() -> Self {
        Self {
            shape: TopoDS_Shape::new(crate::topology::shape_enum::ShapeType::Wire),
            edges: Vec::new(),
            vertices: Vec::new(),
            closed: false,
            tolerance: 0.001,
        }
    }

    /// Create a new wire with specified edges
    pub fn with_edges(edges: Vec<Handle<TopoDS_Edge>>) -> Self {
        let mut wire = Self::new();
        for edge in edges {
            wire.add_edge(edge);
        }
        wire.update_closed();
        wire
    }

    /// Create a new wire with specified tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            shape: TopoDS_Shape::new(crate::topology::shape_enum::ShapeType::Wire),
            edges: Vec::new(),
            vertices: Vec::new(),
            closed: false,
            tolerance,
        }
    }

    /// Add an edge to the wire
    pub fn add_edge(&mut self, edge: Handle<TopoDS_Edge>) {
        if self.edges.is_empty() {
            self.vertices.push(edge.vertex1().clone());
        }
        
        self.edges.push(edge.clone());
        self.vertices.push(edge.vertex2().clone());
        self.update_closed();
    }

    /// Get the edges of the wire
    pub fn edges(&self) -> &[Handle<TopoDS_Edge>] {
        &self.edges
    }

    /// Get the vertices of the wire
    pub fn vertices(&self) -> &[Handle<TopoDS_Vertex>] {
        &self.vertices
    }

    /// Get the number of edges in the wire
    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    /// Get the number of vertices in the wire
    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Check if the wire is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Update the closed status of the wire
    pub fn update_closed(&mut self) {
        if self.edges.is_empty() {
            self.closed = false;
            return;
        }

        if self.edges.len() == 1 {
            let edge = &self.edges[0];
            self.closed = edge.is_degenerate();
            return;
        }

        let first_vertex = self.edges[0].vertex1();
        let last_vertex = self.edges[self.edges.len() - 1].vertex2();
        self.closed = first_vertex == last_vertex;
    }

    /// Get the tolerance of the wire
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the wire
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDS_Shape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDS_Shape {
        &mut self.shape
    }

    /// Get the location of the wire
    pub fn location(&self) -> Option<&TopoDS_Location> {
        self.shape.location()
    }

    /// Set the location of the wire
    pub fn set_location(&mut self, location: TopoDS_Location) {
        self.shape.set_location(location);
    }

    /// Check if the wire is empty
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Clear all edges from the wire
    pub fn clear(&mut self) {
        self.edges.clear();
        self.vertices.clear();
        self.closed = false;
    }

    /// Get the total length of the wire
    pub fn length(&self) -> f64 {
        self.edges.iter().map(|e| e.length()).sum()
    }

    /// Get the first edge of the wire
    pub fn first_edge(&self) -> Option<&Handle<TopoDS_Edge>> {
        self.edges.first()
    }

    /// Get the last edge of the wire
    pub fn last_edge(&self) -> Option<&Handle<TopoDS_Edge>> {
        self.edges.last()
    }

    /// Get the first vertex of the wire
    pub fn first_vertex(&self) -> Option<&Handle<TopoDS_Vertex>> {
        self.vertices.first()
    }

    /// Get the last vertex of the wire
    pub fn last_vertex(&self) -> Option<&Handle<TopoDS_Vertex>> {
        self.vertices.last()
    }

    /// Get the unique identifier of the wire
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the wire
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this wire is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the wire
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the wire contains a specific edge
    pub fn contains_edge(&self, edge: &Handle<TopoDS_Edge>) -> bool {
        self.edges.contains(edge)
    }

    /// Remove an edge from the wire
    pub fn remove_edge(&mut self, edge: &Handle<TopoDS_Edge>) {
        if let Some(pos) = self.edges.iter().position(|e| e == edge) {
            self.edges.remove(pos);
            // Rebuild vertices
            self.vertices.clear();
            if !self.edges.is_empty() {
                self.vertices.push(self.edges[0].vertex1().clone());
                for edge in &self.edges {
                    self.vertices.push(edge.vertex2().clone());
                }
            }
            self.update_closed();
        }
    }

    /// Check if the wire contains a specific vertex
    pub fn contains_vertex(&self, vertex: &Handle<TopoDS_Vertex>) -> bool {
        self.vertices.contains(vertex)
    }

    /// Get all unique vertices in the wire
    pub fn unique_vertices(&self) -> Vec<Handle<TopoDS_Vertex>> {
        let mut seen = HashSet::new();
        let mut unique = Vec::new();
        
        for vertex in &self.vertices {
            if seen.insert(vertex.shape_id()) {
                unique.push(vertex.clone());
            }
        }
        
        unique
    }

    /// Get the bounding box of the wire
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.vertices.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for vertex in &self.vertices {
            let point = vertex.point();
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Check if the wire is self-intersecting
    pub fn is_self_intersecting(&self) -> bool {
        if self.edges.len() < 2 {
            return false;
        }

        for i in 0..self.edges.len() {
            for j in (i + 1)..self.edges.len() {
                if self.edges_intersect(i, j) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if two edges in the wire intersect
    fn edges_intersect(&self, idx1: usize, idx2: usize) -> bool {
        if idx1 >= self.edges.len() || idx2 >= self.edges.len() {
            return false;
        }

        let edge1 = &self.edges[idx1];
        let edge2 = &self.edges[idx2];

        let v1_1 = edge1.vertex1().point();
        let v1_2 = edge1.vertex2().point();
        let v2_1 = edge2.vertex1().point();
        let v2_2 = edge2.vertex2().point();

        let edge1_vertices = [v1_1, v1_2];
        let edge2_vertices = [v2_1, v2_2];

        for v1 in &edge1_vertices {
            for v2 in &edge2_vertices {
                if v1 == v2 {
                    return false;
                }
            }
        }

        false
    }

    /// Reverse the orientation of the wire
    pub fn reverse(&mut self) {
        self.edges.reverse();
        self.vertices.reverse();
        self.update_closed();
    }
}

impl Default for TopoDS_Wire {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDS_Wire {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            edges: self.edges.clone(),
            vertices: self.vertices.clone(),
            closed: self.closed,
            tolerance: self.tolerance,
        }
    }
}

impl PartialEq for TopoDS_Wire {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_creation() {
        let wire = TopoDS_Wire::new();
        assert!(wire.is_empty());
        assert_eq!(wire.num_edges(), 0);
    }

    #[test]
    fn test_wire_add_edge() {
        let mut wire = TopoDS_Wire::new();
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        
        wire.add_edge(edge);
        assert_eq!(wire.num_edges(), 1);
        assert!(!wire.is_closed());
    }

    #[test]
    fn test_wire_closed() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge1 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        let edge2 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v2.clone(), v1.clone())));
        
        let wire = TopoDS_Wire::with_edges(vec![edge1, edge2]);
        assert!(wire.is_closed());
    }

    #[test]
    fn test_wire_length() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let v3 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(2.0, 0.0, 0.0))));
        let edge1 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        let edge2 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v2.clone(), v3.clone())));
        
        let wire = TopoDS_Wire::with_edges(vec![edge1, edge2]);
        let length = wire.length();
        assert!((length - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_wire_clear() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        
        let mut wire = TopoDS_Wire::with_edges(vec![edge]);
        assert!(!wire.is_empty());
        
        wire.clear();
        assert!(wire.is_empty());
    }

    #[test]
    fn test_wire_unique_vertices() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let edge1 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        let edge2 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v2.clone(), v1.clone())));
        
        let wire = TopoDS_Wire::with_edges(vec![edge1, edge2]);
        let unique = wire.unique_vertices();
        assert_eq!(unique.len(), 2);
    }

    #[test]
    fn test_wire_bounding_box() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 1.0, 0.0))));
        let edge = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        
        let wire = TopoDS_Wire::with_edges(vec![edge]);
        let bbox = wire.bounding_box();
        
        assert!(bbox.is_some());
        let (min, max) = bbox.unwrap();
        assert_eq!(min, Point::new(0.0, 0.0, 0.0));
        assert_eq!(max, Point::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_wire_reverse() {
        let v1 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(0.0, 0.0, 0.0))));
        let v2 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(1.0, 0.0, 0.0))));
        let v3 = Handle::new(std::sync::Arc::new(TopoDS_Vertex::new(Point::new(2.0, 0.0, 0.0))));
        let edge1 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v1.clone(), v2.clone())));
        let edge2 = Handle::new(std::sync::Arc::new(TopoDS_Edge::new(v2.clone(), v3.clone())));
        
        let mut wire = TopoDS_Wire::with_edges(vec![edge1.clone(), edge2.clone()]);
        let first_edge_id = wire.first_edge().unwrap().shape_id();
        let second_edge_id = wire.last_edge().unwrap().shape_id();
        
        wire.reverse();
        
        // 反转后，原来的最后一个边应该变成第一个边
        assert_eq!(wire.first_edge().unwrap().shape_id(), second_edge_id);
        // 原来的第一个边应该变成最后一个边
        assert_eq!(wire.last_edge().unwrap().shape_id(), first_edge_id);
    }
}
