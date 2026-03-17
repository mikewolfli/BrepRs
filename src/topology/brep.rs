//! BRep (Boundary Representation) implementation
//! 
//! This module provides a comprehensive boundary representation for solid models,
//! including topological structure, model management, and advanced operations.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, SurfaceEnum, Vector};
use crate::topology::{TopoDsShape, topods_edge::TopoDsEdge, topods_face::TopoDsFace, topods_shell::TopoDsShell, topods_solid::TopoDsSolid, topods_vertex::TopoDsVertex, topods_wire::TopoDsWire};
use std::collections::{HashMap, HashSet};

/// BRep model
#[derive(Debug, Clone)]
pub struct BrepModel {
    /// Solids in the model
    pub solids: Vec<Handle<TopoDsSolid>>,
    /// Shells in the model
    pub shells: Vec<Handle<TopoDsShell>>,
    /// Faces in the model
    pub faces: Vec<Handle<TopoDsFace>>,
    /// Wires in the model
    pub wires: Vec<Handle<TopoDsWire>>,
    /// Edges in the model
    pub edges: Vec<Handle<TopoDsEdge>>,
    /// Vertices in the model
    pub vertices: Vec<Handle<TopoDsVertex>>,
    /// Topological relationships
    pub topology: BrepTopology,
}

/// BRep topology
#[derive(Debug, Clone)]
pub struct BrepTopology {
    /// Vertex-edge relationships
    pub vertex_edges: HashMap<Handle<TopoDsVertex>, Vec<Handle<TopoDsEdge>>>,
    /// Edge-vertex relationships
    pub edge_vertices: HashMap<Handle<TopoDsEdge>, (Handle<TopoDsVertex>, Handle<TopoDsVertex>)>,
    /// Edge-face relationships
    pub edge_faces: HashMap<Handle<TopoDsEdge>, Vec<Handle<TopoDsFace>>>,
    /// Face-edge relationships
    pub face_edges: HashMap<Handle<TopoDsFace>, Vec<Handle<TopoDsEdge>>>,
    /// Face-wire relationships
    pub face_wires: HashMap<Handle<TopoDsFace>, Vec<Handle<TopoDsWire>>>,
    /// Wire-edge relationships
    pub wire_edges: HashMap<Handle<TopoDsWire>, Vec<Handle<TopoDsEdge>>>,
    /// Shell-face relationships
    pub shell_faces: HashMap<Handle<TopoDsShell>, Vec<Handle<TopoDsFace>>>,
    /// Solid-shell relationships
    pub solid_shells: HashMap<Handle<TopoDsSolid>, Vec<Handle<TopoDsShell>>>,
}

impl BrepTopology {
    /// Create a new BRep topology
    pub fn new() -> Self {
        Self {
            vertex_edges: HashMap::new(),
            edge_vertices: HashMap::new(),
            edge_faces: HashMap::new(),
            face_edges: HashMap::new(),
            face_wires: HashMap::new(),
            wire_edges: HashMap::new(),
            shell_faces: HashMap::new(),
            solid_shells: HashMap::new(),
        }
    }
}

impl BrepModel {
    /// Create a new BRep model
    pub fn new() -> Self {
        Self {
            solids: Vec::new(),
            shells: Vec::new(),
            faces: Vec::new(),
            wires: Vec::new(),
            edges: Vec::new(),
            vertices: Vec::new(),
            topology: BrepTopology::new(),
        }
    }

    /// Add a solid to the model
    pub fn add_solid(&mut self, solid: TopoDsSolid) {
        let solid_handle = Handle::new(solid);
        self.solids.push(solid_handle.clone());
        
        // Extract and add shells from the solid
        let shells = solid_handle.as_ref().unwrap().shells();
        for shell in shells {
            self.add_shell_internal(shell, Some(solid_handle.clone()));
        }
    }

    /// Add a shell to the model
    fn add_shell_internal(&mut self, shell: Handle<TopoDsShell>, parent_solid: Option<Handle<TopoDsSolid>>) {
        // Check if the shell is already in the model
        if !self.shells.contains(&shell) {
            self.shells.push(shell.clone());
            
            // Update solid-shell relationship
            if let Some(solid) = parent_solid {
                self.topology.solid_shells.entry(solid).or_insert(Vec::new()).push(shell.clone());
            }
            
            // Extract and add faces from the shell
            let faces = shell.as_ref().unwrap().faces();
            for face in faces {
                self.add_face_internal(face, Some(shell.clone()));
            }
        }
    }

    /// Add a face to the model
    fn add_face_internal(&mut self, face: Handle<TopoDsFace>, parent_shell: Option<Handle<TopoDsShell>>) {
        // Check if the face is already in the model
        if !self.faces.contains(&face) {
            self.faces.push(face.clone());
            
            // Update shell-face relationship
            if let Some(shell) = parent_shell {
                self.topology.shell_faces.entry(shell).or_insert(Vec::new()).push(face.clone());
            }
            
            // Extract and add wires from the face
            let wires = face.as_ref().unwrap().wires();
            for wire in wires {
                self.add_wire_internal(wire, Some(face.clone()));
            }
        }
    }

    /// Add a wire to the model
    fn add_wire_internal(&mut self, wire: Handle<TopoDsWire>, parent_face: Option<Handle<TopoDsFace>>) {
        // Check if the wire is already in the model
        if !self.wires.contains(&wire) {
            self.wires.push(wire.clone());
            
            // Update face-wire relationship
            if let Some(face) = parent_face {
                self.topology.face_wires.entry(face).or_insert(Vec::new()).push(wire.clone());
            }
            
            // Extract and add edges from the wire
            let edges = wire.as_ref().unwrap().edges();
            for edge in edges {
                self.add_edge_internal(edge, Some(wire.clone()));
            }
        }
    }

    /// Add an edge to the model
    fn add_edge_internal(&mut self, edge: Handle<TopoDsEdge>, parent_wire: Option<Handle<TopoDsWire>>) {
        // Check if the edge is already in the model
        if !self.edges.contains(&edge) {
            self.edges.push(edge.clone());
            
            // Update wire-edge relationship
            if let Some(wire) = parent_wire {
                self.topology.wire_edges.entry(wire).or_insert(Vec::new()).push(edge.clone());
            }
            
            // Extract and add vertices from the edge
            let v1 = edge.as_ref().unwrap().start_vertex();
            let v2 = edge.as_ref().unwrap().end_vertex();
            
            self.add_vertex_internal(v1, Some(edge.clone()));
            self.add_vertex_internal(v2, Some(edge.clone()));
            
            // Update edge-vertex relationship
            self.topology.edge_vertices.insert(edge.clone(), (v1, v2));
        }
    }

    /// Add a vertex to the model
    fn add_vertex_internal(&mut self, vertex: Handle<TopoDsVertex>, parent_edge: Option<Handle<TopoDsEdge>>) {
        // Check if the vertex is already in the model
        if !self.vertices.contains(&vertex) {
            self.vertices.push(vertex.clone());
        }
        
        // Update vertex-edge relationship
        if let Some(edge) = parent_edge {
            self.topology.vertex_edges.entry(vertex).or_insert(Vec::new()).push(edge);
        }
    }

    /// Build the topology for the model
    pub fn build_topology(&mut self) {
        // Clear existing topology
        self.topology = BrepTopology::new();
        
        // Rebuild topology from scratch
        for solid in &self.solids {
            let shells = solid.as_ref().unwrap().shells();
            for shell in &shells {
                self.topology.solid_shells.entry(solid.clone()).or_insert(Vec::new()).push(shell.clone());
                
                let faces = shell.as_ref().unwrap().faces();
                for face in &faces {
                    self.topology.shell_faces.entry(shell.clone()).or_insert(Vec::new()).push(face.clone());
                    
                    let wires = face.as_ref().unwrap().wires();
                    for wire in &wires {
                        self.topology.face_wires.entry(face.clone()).or_insert(Vec::new()).push(wire.clone());
                        
                        let edges = wire.as_ref().unwrap().edges();
                        for edge in &edges {
                            self.topology.wire_edges.entry(wire.clone()).or_insert(Vec::new()).push(edge.clone());
                            self.topology.face_edges.entry(face.clone()).or_insert(Vec::new()).push(edge.clone());
                            self.topology.edge_faces.entry(edge.clone()).or_insert(Vec::new()).push(face.clone());
                            
                            let v1 = edge.as_ref().unwrap().start_vertex();
                            let v2 = edge.as_ref().unwrap().end_vertex();
                            
                            self.topology.edge_vertices.insert(edge.clone(), (v1, v2));
                            self.topology.vertex_edges.entry(v1).or_insert(Vec::new()).push(edge.clone());
                            self.topology.vertex_edges.entry(v2).or_insert(Vec::new()).push(edge.clone());
                        }
                    }
                }
            }
        }
    }

    /// Get all edges connected to a vertex
    pub fn get_edges_from_vertex(&self, vertex: &Handle<TopoDsVertex>) -> Vec<Handle<TopoDsEdge>> {
        self.topology.vertex_edges.get(vertex).cloned().unwrap_or(Vec::new())
    }

    /// Get all faces connected to an edge
    pub fn get_faces_from_edge(&self, edge: &Handle<TopoDsEdge>) -> Vec<Handle<TopoDsFace>> {
        self.topology.edge_faces.get(edge).cloned().unwrap_or(Vec::new())
    }

    /// Get all edges of a face
    pub fn get_edges_from_face(&self, face: &Handle<TopoDsFace>) -> Vec<Handle<TopoDsEdge>> {
        self.topology.face_edges.get(face).cloned().unwrap_or(Vec::new())
    }

    /// Get all wires of a face
    pub fn get_wires_from_face(&self, face: &Handle<TopoDsFace>) -> Vec<Handle<TopoDsWire>> {
        self.topology.face_wires.get(face).cloned().unwrap_or(Vec::new())
    }

    /// Get all faces of a shell
    pub fn get_faces_from_shell(&self, shell: &Handle<TopoDsShell>) -> Vec<Handle<TopoDsFace>> {
        self.topology.shell_faces.get(shell).cloned().unwrap_or(Vec::new())
    }

    /// Get all shells of a solid
    pub fn get_shells_from_solid(&self, solid: &Handle<TopoDsSolid>) -> Vec<Handle<TopoDsShell>> {
        self.topology.solid_shells.get(solid).cloned().unwrap_or(Vec::new())
    }

    /// Check if the model is manifold
    pub fn is_manifold(&self) -> bool {
        // Check that every edge is shared by at most two faces
        for edge in &self.edges {
            let faces = self.get_faces_from_edge(edge);
            if faces.len() > 2 {
                return false;
            }
        }
        
        // Check that every vertex has a consistent neighborhood
        for vertex in &self.vertices {
            let edges = self.get_edges_from_vertex(vertex);
            if edges.is_empty() {
                continue;
            }
            
            // Check if edges form a closed loop around the vertex
            if !self.is_vertex_manifold(vertex) {
                return false;
            }
        }
        
        true
    }

    /// Check if a vertex is manifold
    fn is_vertex_manifold(&self, vertex: &Handle<TopoDsVertex>) -> bool {
        let edges = self.get_edges_from_vertex(vertex);
        if edges.len() < 2 {
            return true; // Boundary vertex
        }
        
        // Build a list of adjacent edges in order
        let mut ordered_edges = Vec::new();
        let mut current_edge = edges[0].clone();
        ordered_edges.push(current_edge.clone());
        
        for _ in 1..edges.len() {
            let next_edge = self.find_next_edge(vertex, &current_edge, &edges);
            if next_edge.is_none() {
                return false;
            }
            
            current_edge = next_edge.unwrap();
            if ordered_edges.contains(&current_edge) {
                break;
            }
            ordered_edges.push(current_edge.clone());
        }
        
        ordered_edges.len() == edges.len()
    }

    /// Find the next edge in the loop around a vertex
    fn find_next_edge(&self, vertex: &Handle<TopoDsVertex>, current_edge: &Handle<TopoDsEdge>, all_edges: &[Handle<TopoDsEdge>]) -> Option<Handle<TopoDsEdge>> {
        // Get the other vertex of the current edge
        let (v1, v2) = self.topology.edge_vertices.get(current_edge).unwrap();
        let other_vertex = if v1 == *vertex { v2 } else { v1 };
        
        // Find faces adjacent to the current edge
        let faces = self.get_faces_from_edge(current_edge);
        if faces.is_empty() {
            return None;
        }
        
        // For each face, find the next edge in the loop
        for face in &faces {
            let face_edges = self.get_edges_from_face(face);
            let edge_index = face_edges.iter().position(|e| e == current_edge);
            if let Some(index) = edge_index {
                let next_index = (index + 1) % face_edges.len();
                let next_edge = face_edges[next_index].clone();
                
                // Check if the next edge is connected to the original vertex
                let (nv1, nv2) = self.topology.edge_vertices.get(&next_edge).unwrap();
                if nv1 == *vertex || nv2 == *vertex {
                    return Some(next_edge);
                }
            }
        }
        
        None
    }

    /// Calculate the bounding box of the model
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
            if let Some(vertex_ref) = vertex.as_ref() {
                let point = vertex_ref.point();
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                min_z = min_z.min(point.z);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
                max_z = max_z.max(point.z);
            }
        }
        
        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z)
        ))
    }

    /// Export the model to a string representation
    pub fn to_string(&self) -> String {
        let mut result = format!("BRep Model\n");
        result.push_str(&format!("Solids: {}\n", self.solids.len()));
        result.push_str(&format!("Shells: {}\n", self.shells.len()));
        result.push_str(&format!("Faces: {}\n", self.faces.len()));
        result.push_str(&format!("Wires: {}\n", self.wires.len()));
        result.push_str(&format!("Edges: {}\n", self.edges.len()));
        result.push_str(&format!("Vertices: {}\n", self.vertices.len()));
        result.push_str(&format!("Manifold: {}\n", self.is_manifold()));
        
        if let Some((min, max)) = self.bounding_box() {
            result.push_str(&format!("Bounding Box: {:?} to {:?}\n", min, max));
        }
        
        result
    }
}

impl Default for BrepModel {
    fn default() -> Self {
        Self::new()
    }
}

/// BRep builder
pub struct BrepBuilder {
    /// Current model being built
    model: BrepModel,
}

impl BrepBuilder {
    /// Create a new BRep builder
    pub fn new() -> Self {
        Self {
            model: BrepModel::new(),
        }
    }

    /// Add a solid to the model
    pub fn add_solid(&mut self, solid: TopoDsSolid) -> &mut Self {
        self.model.add_solid(solid);
        self
    }

    /// Build the model
    pub fn build(&mut self) -> BrepModel {
        self.model.build_topology();
        std::mem::take(&mut self.model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_brep_model_creation() {
        let mut builder = BrepBuilder::new();
        
        // Create a simple solid
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        
        let model = builder.build();
        
        assert_eq!(model.solids.len(), 1);
        assert!(model.is_manifold());
    }

    #[test]
    fn test_bounding_box() {
        let mut builder = BrepBuilder::new();
        
        // Create a simple solid
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        
        let model = builder.build();
        
        assert!(model.bounding_box().is_some());
    }
}
