//! Topological operations module
//! 
//! This module provides topological modification operations for solids,
//! including edge splitting, face merging, and other topological operations.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{TopoDsShape, topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_solid::TopoDsSolid, topods_vertex::TopoDsVertex, topods_wire::TopoDsWire};

/// Topological operations result
#[derive(Debug, Clone, PartialEq)]
pub struct TopoOperationResult {
    /// Whether the operation was successful
    pub success: bool,
    /// Newly created shapes (if any)
    pub new_shapes: Vec<Handle<TopoDsShape>>,
    /// Modified shapes (if any)
    pub modified_shapes: Vec<Handle<TopoDsShape>>,
}

/// Topological operations
pub struct TopoOperations {
    /// Tolerance for topological operations
    tolerance: f64,
}

impl TopoOperations {
    /// Create a new topological operations object with default tolerance
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
        }
    }

    /// Create a new topological operations object with custom tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            tolerance,
        }
    }

    /// Split an edge at a parameter value
    pub fn split_edge(&self, edge: &TopoDsEdge, parameter: f64) -> TopoOperationResult {
        // Get the edge's curve
        let curve = edge.curve();
        if curve.is_none() {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        let curve = curve.unwrap();
        
        // Check if the parameter is within the edge's parameter range
        let (u_min, u_max) = edge.parameter_range();
        if parameter < u_min - self.tolerance || parameter > u_max + self.tolerance {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        // Calculate the point at the split parameter
        let split_point = curve.value(parameter);
        
        // Create a new vertex at the split point
        let new_vertex = TopoDsVertex::new(split_point);
        
        // Create two new edges
        let edge1 = TopoDsEdge::new(curve.clone(), u_min, parameter);
        let edge2 = TopoDsEdge::new(curve, parameter, u_max);
        
        TopoOperationResult {
            success: true,
            new_shapes: vec![Handle::new(TopoDsShape::from_vertex(new_vertex)),
                           Handle::new(TopoDsShape::from_edge(edge1)),
                           Handle::new(TopoDsShape::from_edge(edge2))],
            modified_shapes: vec![Handle::new(TopoDsShape::from_edge(edge.clone()))],
        }
    }

    /// Split an edge at a point
    pub fn split_edge_at_point(&self, edge: &TopoDsEdge, point: &Point) -> TopoOperationResult {
        // Get the edge's curve
        let curve = edge.curve();
        if curve.is_none() {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        let curve = curve.unwrap();
        
        // Find the parameter corresponding to the point
        let parameter = self.find_parameter_on_edge(edge, point);
        if parameter.is_none() {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        self.split_edge(edge, parameter.unwrap())
    }

    /// Merge two adjacent faces
    pub fn merge_faces(&self, face1: &TopoDsFace, face2: &TopoDsFace) -> TopoOperationResult {
        // Check if the faces are adjacent
        if !self.are_faces_adjacent(face1, face2) {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        // Get the common edge
        let common_edge = self.find_common_edge(face1, face2);
        if common_edge.is_none() {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        // Create a new face by merging the two faces
        let new_face = self.create_merged_face(face1, face2, common_edge.unwrap());
        
        TopoOperationResult {
            success: true,
            new_shapes: vec![Handle::new(TopoDsShape::from_face(new_face))],
            modified_shapes: vec![Handle::new(TopoDsShape::from_face(face1.clone())),
                               Handle::new(TopoDsShape::from_face(face2.clone()))],
        }
    }

    /// Split a face by a line
    pub fn split_face_by_line(&self, face: &TopoDsFace, start_point: &Point, end_point: &Point) -> TopoOperationResult {
        // Check if both points are on the face boundary
        if !self.is_point_on_face_boundary(start_point, face) || 
           !self.is_point_on_face_boundary(end_point, face) {
            return TopoOperationResult {
                success: false,
                new_shapes: Vec::new(),
                modified_shapes: Vec::new(),
            };
        }
        
        // Create a new edge between the two points
        let new_edge = TopoDsEdge::new_line(start_point, end_point);
        
        // Create two new faces by splitting the original face
        let (face1, face2) = self.create_split_faces(face, &new_edge);
        
        TopoOperationResult {
            success: true,
            new_shapes: vec![Handle::new(TopoDsShape::from_edge(new_edge)),
                           Handle::new(TopoDsShape::from_face(face1)),
                           Handle::new(TopoDsShape::from_face(face2))],
            modified_shapes: vec![Handle::new(TopoDsShape::from_face(face.clone()))],
        }
    }

    /// Find the parameter of a point on an edge
    fn find_parameter_on_edge(&self, edge: &TopoDsEdge, point: &Point) -> Option<f64> {
        let curve = edge.curve();
        if curve.is_none() {
            return None;
        }
        
        let curve = curve.unwrap();
        let (u_min, u_max) = edge.parameter_range();
        
        // Use Newton-Raphson to find the parameter
        let mut u = (u_min + u_max) / 2.0;
        let max_iterations = 100;
        
        for _ in 0..max_iterations {
            let curve_point = curve.value(u);
            let distance = point.distance(&curve_point);
            
            if distance < self.tolerance {
                return Some(u);
            }
            
            let derivative = curve.derivative(u);
            if derivative.magnitude() < self.tolerance {
                break;
            }
            
            let delta = (point.x - curve_point.x) * derivative.x +
                       (point.y - curve_point.y) * derivative.y +
                       (point.z - curve_point.z) * derivative.z;
            
            let step = delta / derivative.magnitude().powi(2);
            u += step;
            
            if u < u_min || u > u_max {
                break;
            }
        }
        
        None
    }

    /// Check if two faces are adjacent
    fn are_faces_adjacent(&self, face1: &TopoDsFace, face2: &TopoDsFace) -> bool {
        let wires1 = face1.wires();
        let wires2 = face2.wires();
        
        for wire1 in wires1 {
            if let Some(wire1_ref) = wire1.as_ref() {
                let edges1 = wire1_ref.edges();
                
                for wire2 in wires2 {
                    if let Some(wire2_ref) = wire2.as_ref() {
                        let edges2 = wire2_ref.edges();
                        
                        for edge1 in &edges1 {
                            for edge2 in &edges2 {
                                if self.are_edges_coincident(edge1.as_ref().unwrap(), edge2.as_ref().unwrap()) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        false
    }

    /// Check if two edges are coincident
    fn are_edges_coincident(&self, edge1: &TopoDsEdge, edge2: &TopoDsEdge) -> bool {
        let v1_start = edge1.start_vertex();
        let v1_end = edge1.end_vertex();
        let v2_start = edge2.start_vertex();
        let v2_end = edge2.end_vertex();
        
        if let (Some(v1s), Some(v1e), Some(v2s), Some(v2e)) = 
            (v1_start.get(), v1_end.get(), v2_start.get(), v2_end.get()) {
            let p1s = v1s.point();
            let p1e = v1e.point();
            let p2s = v2s.point();
            let p2e = v2e.point();
            
            // Check if the edges share both vertices (in either order)
            ((p1s.distance(&p2s) < self.tolerance && p1e.distance(&p2e) < self.tolerance) ||
             (p1s.distance(&p2e) < self.tolerance && p1e.distance(&p2s) < self.tolerance))
        } else {
            false
        }
    }

    /// Find the common edge between two adjacent faces
    fn find_common_edge(&self, face1: &TopoDsFace, face2: &TopoDsFace) -> Option<Handle<TopoDsEdge>> {
        let wires1 = face1.wires();
        let wires2 = face2.wires();
        
        for wire1 in wires1 {
            if let Some(wire1_ref) = wire1.as_ref() {
                let edges1 = wire1_ref.edges();
                
                for wire2 in wires2 {
                    if let Some(wire2_ref) = wire2.as_ref() {
                        let edges2 = wire2_ref.edges();
                        
                        for edge1 in &edges1 {
                            for edge2 in &edges2 {
                                if self.are_edges_coincident(edge1.as_ref().unwrap(), edge2.as_ref().unwrap()) {
                                    return Some(edge1.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        None
    }

    /// Create a merged face from two adjacent faces
    fn create_merged_face(&self, face1: &TopoDsFace, face2: &TopoDsFace, common_edge: &Handle<TopoDsEdge>) -> TopoDsFace {
        // Get the wires of both faces
        let wires1 = face1.wires();
        let wires2 = face2.wires();
        
        // Create a new wire by combining the wires of both faces, excluding the common edge
        let mut new_edges = Vec::new();
        
        // Add edges from face1, excluding the common edge
        for wire1 in &wires1 {
            if let Some(wire1_ref) = wire1.as_ref() {
                let edges1 = wire1_ref.edges();
                for edge in &edges1 {
                    if !self.are_edges_coincident(edge.as_ref().unwrap(), common_edge.as_ref().unwrap()) {
                        new_edges.push(edge.clone());
                    }
                }
            }
        }
        
        // Add edges from face2, excluding the common edge
        for wire2 in &wires2 {
            if let Some(wire2_ref) = wire2.as_ref() {
                let edges2 = wire2_ref.edges();
                for edge in &edges2 {
                    if !self.are_edges_coincident(edge.as_ref().unwrap(), common_edge.as_ref().unwrap()) {
                        new_edges.push(edge.clone());
                    }
                }
            }
        }
        
        // Create a new wire from the edges
        let new_wire = TopoDsWire::from_edges(&new_edges);
        
        // Create a new face from the wire
        TopoDsFace::from_wire(&new_wire)
    }

    /// Check if a point is on the face boundary
    fn is_point_on_face_boundary(&self, point: &Point, face: &TopoDsFace) -> bool {
        let wires = face.wires();
        
        for wire in wires {
            if let Some(wire_ref) = wire.as_ref() {
                if self.is_point_on_wire(point, wire_ref) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Check if a point is on a wire
    fn is_point_on_wire(&self, point: &Point, wire: &TopoDsWire) -> bool {
        let edges = wire.edges();
        
        for edge in &edges {
            if let Some(edge_ref) = edge.as_ref() {
                if self.is_point_on_edge(point, edge_ref) {
                    return true;
                }
            }
        }
        
        false
    }

    /// Check if a point is on an edge
    fn is_point_on_edge(&self, point: &Point, edge: &TopoDsEdge) -> bool {
        let curve = edge.curve();
        if curve.is_none() {
            return false;
        }
        
        let parameter = self.find_parameter_on_edge(edge, point);
        parameter.is_some()
    }

    /// Create two new faces by splitting a face with an edge
    fn create_split_faces(&self, face: &TopoDsFace, edge: &TopoDsEdge) -> (TopoDsFace, TopoDsFace) {
        // Get the face's wires
        let wires = face.wires();
        let outer_wire = wires[0].clone();
        
        // Get the edge's vertices
        let v1 = edge.start_vertex();
        let v2 = edge.end_vertex();
        
        // Split the outer wire at the two vertices
        let (wire1, wire2) = self.split_wire_at_vertices(&outer_wire, &v1, &v2);
        
        // Create two new wires by combining the split wires with the new edge
        let new_wire1 = self.create_wire_from_segments(&wire1, edge, true);
        let new_wire2 = self.create_wire_from_segments(&wire2, edge, false);
        
        // Create two new faces
        let face1 = TopoDsFace::from_wire(&new_wire1);
        let face2 = TopoDsFace::from_wire(&new_wire2);
        
        (face1, face2)
    }

    /// Split a wire at two vertices
    fn split_wire_at_vertices(&self, wire: &TopoDsWire, v1: &Handle<TopoDsVertex>, v2: &Handle<TopoDsVertex>) -> (Vec<Handle<TopoDsEdge>>, Vec<Handle<TopoDsEdge>>) {
        let edges = wire.edges();
        let mut segment1 = Vec::new();
        let mut segment2 = Vec::new();
        let mut in_segment1 = false;
        
        for edge in &edges {
            if let Some(edge_ref) = edge.as_ref() {
                let edge_v1 = edge_ref.start_vertex();
                let edge_v2 = edge_ref.end_vertex();
                
                if edge_v1 == *v1 || edge_v2 == *v1 {
                    in_segment1 = true;
                }
                
                if in_segment1 {
                    segment1.push(edge.clone());
                } else {
                    segment2.push(edge.clone());
                }
                
                if edge_v1 == *v2 || edge_v2 == *v2 {
                    in_segment1 = false;
                }
            }
        }
        
        (segment1, segment2)
    }

    /// Create a wire from segments and a connecting edge
    fn create_wire_from_segments(&self, segments: &[Handle<TopoDsEdge>], edge: &TopoDsEdge, reverse_edge: bool) -> TopoDsWire {
        let mut new_edges = segments.to_vec();
        
        if reverse_edge {
            // Reverse the edge direction
            let reversed_edge = TopoDsEdge::new_reversed(edge);
            new_edges.push(Handle::new(reversed_edge));
        } else {
            new_edges.push(Handle::new(edge.clone()));
        }
        
        TopoDsWire::from_edges(&new_edges)
    }
}

impl Default for TopoOperations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_split_edge() {
        // Create a line edge
        let start = Point::new(0.0, 0.0, 0.0);
        let end = Point::new(1.0, 1.0, 1.0);
        let edge = TopoDsEdge::new_line(&start, &end);
        
        let operations = TopoOperations::new();
        let result = operations.split_edge(&edge, 0.5);
        
        assert!(result.success);
        assert_eq!(result.new_shapes.len(), 3); // New vertex and two new edges
    }

    #[test]
    fn test_are_faces_adjacent() {
        // Create two adjacent faces
        // This would require more complex setup, so we'll skip for now
        let face1 = TopoDsFace::new();
        let face2 = TopoDsFace::new();
        
        let operations = TopoOperations::new();
        let result = operations.are_faces_adjacent(&face1, &face2);
        
        assert!(!result);
    }
}
