//! BRep (Boundary Representation) implementation
//!
//! This module provides a comprehensive boundary representation for solid models,
//! including topological structure, model management, advanced operations, and
//! integration with other geometric and modeling modules.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Transform};
use crate::topology::{
    topods_compound::TopoDsCompound, topods_edge::TopoDsEdge, topods_face::TopoDsFace,
    topods_shell::TopoDsShell, topods_solid::TopoDsSolid, topods_vertex::TopoDsVertex,
    topods_wire::TopoDsWire, TopoDsShape,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

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
        let solid_handle = Handle::new(std::sync::Arc::new(solid));
        self.solids.push(solid_handle.clone());

        let shells = solid_handle.as_ref().unwrap().shells();
        for shell in shells {
            self.add_shell_internal(shell.clone(), Some(solid_handle.clone()));
        }
    }

    /// Add a shell to the model
    pub fn add_shell(&mut self, shell: TopoDsShell) {
        let shell_handle = Handle::new(std::sync::Arc::new(shell));
        self.add_shell_internal(shell_handle, None);
    }

    /// Add a face to the model
    pub fn add_face(&mut self, face: TopoDsFace) {
        let face_handle = Handle::new(std::sync::Arc::new(face));
        self.add_face_internal(face_handle, None);
    }

    /// Add a wire to the model
    pub fn add_wire(&mut self, wire: TopoDsWire) {
        let wire_handle = Handle::new(std::sync::Arc::new(wire));
        self.add_wire_internal(wire_handle, None);
    }

    /// Add an edge to the model
    pub fn add_edge(&mut self, edge: TopoDsEdge) {
        let edge_handle = Handle::new(std::sync::Arc::new(edge));
        self.add_edge_internal(edge_handle, None);
    }

    /// Add a vertex to the model
    pub fn add_vertex(&mut self, vertex: TopoDsVertex) {
        let vertex_handle = Handle::new(std::sync::Arc::new(vertex));
        self.add_vertex_internal(vertex_handle, None);
    }

    /// Add a shell to the model (internal)
    fn add_shell_internal(
        &mut self,
        shell: Handle<TopoDsShell>,
        parent_solid: Option<Handle<TopoDsSolid>>,
    ) {
        if !self.shells.contains(&shell) {
            self.shells.push(shell.clone());

            if let Some(solid) = parent_solid {
                self.topology
                    .solid_shells
                    .entry(solid)
                    .or_insert(Vec::new())
                    .push(shell.clone());
            }

            let faces = shell.as_ref().unwrap().faces();
            for face in faces {
                self.add_face_internal(face.clone(), Some(shell.clone()));
            }
        }
    }

    /// Add a face to the model (internal)
    fn add_face_internal(
        &mut self,
        face: Handle<TopoDsFace>,
        parent_shell: Option<Handle<TopoDsShell>>,
    ) {
        // Check if the face is already in the model
        if !self.faces.contains(&face) {
            self.faces.push(face.clone());

            if let Some(shell) = parent_shell {
                self.topology
                    .shell_faces
                    .entry(shell)
                    .or_insert(Vec::new())
                    .push(face.clone());
            }

            let wires = face.as_ref().unwrap().wires();
            for wire in wires {
                self.add_wire_internal(wire.clone(), Some(face.clone()));
            }
        }
    }

    /// Add a wire to the model (internal)
    fn add_wire_internal(
        &mut self,
        wire: Handle<TopoDsWire>,
        parent_face: Option<Handle<TopoDsFace>>,
    ) {
        if !self.wires.contains(&wire) {
            self.wires.push(wire.clone());

            if let Some(face) = parent_face {
                self.topology
                    .face_wires
                    .entry(face)
                    .or_insert(Vec::new())
                    .push(wire.clone());
            }

            let edges = wire.as_ref().unwrap().edges();
            for edge in edges {
                self.add_edge_internal(edge.clone(), Some(wire.clone()));
            }
        }
    }

    /// Add an edge to the model (internal)
    fn add_edge_internal(
        &mut self,
        edge: Handle<TopoDsEdge>,
        parent_wire: Option<Handle<TopoDsWire>>,
    ) {
        if !self.edges.contains(&edge) {
            self.edges.push(edge.clone());

            if let Some(wire) = parent_wire {
                self.topology
                    .wire_edges
                    .entry(wire)
                    .or_insert(Vec::new())
                    .push(edge.clone());
            }

            let v1 = edge.as_ref().unwrap().start_vertex();
            let v2 = edge.as_ref().unwrap().end_vertex();

            self.add_vertex_internal(v1.clone(), Some(edge.clone()));
            self.add_vertex_internal(v2.clone(), Some(edge.clone()));

            self.topology
                .edge_vertices
                .insert(edge.clone(), (v1.clone(), v2.clone()));
        }
    }

    /// Add a vertex to the model (internal)
    fn add_vertex_internal(
        &mut self,
        vertex: Handle<TopoDsVertex>,
        parent_edge: Option<Handle<TopoDsEdge>>,
    ) {
        // Check if the vertex is already in the model
        if !self.vertices.contains(&vertex) {
            self.vertices.push(vertex.clone());
        }

        // Update vertex-edge relationship
        if let Some(edge) = parent_edge {
            self.topology
                .vertex_edges
                .entry(vertex)
                .or_insert(Vec::new())
                .push(edge);
        }
    }

    /// Build the topology for the model
    pub fn build_topology(&mut self) {
        self.topology = BrepTopology::new();

        for solid in &self.solids {
            let shells = solid.as_ref().unwrap().shells();
            for shell in shells {
                self.topology
                    .solid_shells
                    .entry(solid.clone())
                    .or_insert(Vec::new())
                    .push(shell.clone());

                let faces = shell.as_ref().unwrap().faces();
                for face in faces {
                    self.topology
                        .shell_faces
                        .entry(shell.clone())
                        .or_insert(Vec::new())
                        .push(face.clone());

                    let wires = face.as_ref().unwrap().wires();
                    for wire in wires {
                        self.topology
                            .face_wires
                            .entry(face.clone())
                            .or_insert(Vec::new())
                            .push(wire.clone());

                        let edges = wire.as_ref().unwrap().edges();
                        for edge in edges {
                            self.topology
                                .wire_edges
                                .entry(wire.clone())
                                .or_insert(Vec::new())
                                .push(edge.clone());
                            self.topology
                                .face_edges
                                .entry(face.clone())
                                .or_insert(Vec::new())
                                .push(edge.clone());
                            self.topology
                                .edge_faces
                                .entry(edge.clone())
                                .or_insert(Vec::new())
                                .push(face.clone());

                            let v1 = edge.as_ref().unwrap().start_vertex();
                            let v2 = edge.as_ref().unwrap().end_vertex();

                            self.topology
                                .edge_vertices
                                .insert(edge.clone(), (v1.clone(), v2.clone()));
                            self.topology
                                .vertex_edges
                                .entry(v1.clone())
                                .or_insert(Vec::new())
                                .push(edge.clone());
                            self.topology
                                .vertex_edges
                                .entry(v2.clone())
                                .or_insert(Vec::new())
                                .push(edge.clone());
                        }
                    }
                }
            }
        }

        // Build face-edge relationships for all faces
        for face in &self.faces {
            if !self.topology.face_edges.contains_key(face) {
                let wires = face.as_ref().unwrap().wires();
                let mut face_edges = Vec::new();
                for wire in wires {
                    let edges = wire.as_ref().unwrap().edges();
                    face_edges.extend(edges.iter().cloned());
                }
                self.topology.face_edges.insert(face.clone(), face_edges);
            }
        }
    }

    /// Get all edges connected to a vertex
    pub fn get_edges_from_vertex(&self, vertex: &Handle<TopoDsVertex>) -> Vec<Handle<TopoDsEdge>> {
        self.topology
            .vertex_edges
            .get(vertex)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all faces connected to an edge
    pub fn get_faces_from_edge(&self, edge: &Handle<TopoDsEdge>) -> Vec<Handle<TopoDsFace>> {
        self.topology
            .edge_faces
            .get(edge)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all edges of a face
    pub fn get_edges_from_face(&self, face: &Handle<TopoDsFace>) -> Vec<Handle<TopoDsEdge>> {
        self.topology
            .face_edges
            .get(face)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all wires of a face
    pub fn get_wires_from_face(&self, face: &Handle<TopoDsFace>) -> Vec<Handle<TopoDsWire>> {
        self.topology
            .face_wires
            .get(face)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all faces of a shell
    pub fn get_faces_from_shell(&self, shell: &Handle<TopoDsShell>) -> Vec<Handle<TopoDsFace>> {
        self.topology
            .shell_faces
            .get(shell)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all shells of a solid
    pub fn get_shells_from_solid(&self, solid: &Handle<TopoDsSolid>) -> Vec<Handle<TopoDsShell>> {
        self.topology
            .solid_shells
            .get(solid)
            .cloned()
            .unwrap_or(Vec::new())
    }

    /// Get all vertices of a face
    pub fn get_vertices_from_face(&self, face: &Handle<TopoDsFace>) -> Vec<Handle<TopoDsVertex>> {
        let edges = self.get_edges_from_face(face);
        let mut vertices = HashSet::new();

        for edge in edges {
            if let Some((v1, v2)) = self.topology.edge_vertices.get(&edge) {
                vertices.insert(v1.clone());
                vertices.insert(v2.clone());
            }
        }

        vertices.into_iter().collect()
    }

    /// Get all vertices of a wire
    pub fn get_vertices_from_wire(&self, wire: &Handle<TopoDsWire>) -> Vec<Handle<TopoDsVertex>> {
        if let Some(edges) = self.topology.wire_edges.get(wire) {
            let mut vertices = HashSet::new();
            for edge in edges {
                if let Some((v1, v2)) = self.topology.edge_vertices.get(edge) {
                    vertices.insert(v1.clone());
                    vertices.insert(v2.clone());
                }
            }
            vertices.into_iter().collect()
        } else {
            Vec::new()
        }
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
    fn find_next_edge(
        &self,
        vertex: &Handle<TopoDsVertex>,
        current_edge: &Handle<TopoDsEdge>,
        _all_edges: &[Handle<TopoDsEdge>],
    ) -> Option<Handle<TopoDsEdge>> {
        let (v1, v2) = self.topology.edge_vertices.get(current_edge).unwrap();
        let _other_vertex = if *v1 == *vertex { v2 } else { v1 };

        let faces = self.get_faces_from_edge(current_edge);
        if faces.is_empty() {
            return None;
        }

        for face in &faces {
            let face_edges = self.get_edges_from_face(face);
            let edge_index = face_edges.iter().position(|e| e == current_edge);
            if let Some(index) = edge_index {
                let next_index = (index + 1) % face_edges.len();
                let next_edge = face_edges[next_index].clone();

                let (nv1, nv2) = self.topology.edge_vertices.get(&next_edge).unwrap();
                if *nv1 == *vertex || *nv2 == *vertex {
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
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Transform the entire model
    pub fn transform(&mut self, transform: &Transform) {
        // Transform all vertices
        for vertex in &mut self.vertices {
            if let Some(vertex_ref) = vertex.as_mut() {
                let point = vertex_ref.point();
                let transformed_point = transform.transforms(&point);
                vertex_ref.set_point(transformed_point);
            }
        }

        // Edges, faces, wires, shells, and solids are implicitly transformed through their vertices
    }

    /// Merge another BRep model into this one
    pub fn merge(&mut self, other: &BrepModel) {
        // Add all solids
        for solid in &other.solids {
            if !self.solids.contains(solid) {
                self.solids.push(solid.clone());
            }
        }

        // Add all shells
        for shell in &other.shells {
            if !self.shells.contains(shell) {
                self.shells.push(shell.clone());
            }
        }

        // Add all faces
        for face in &other.faces {
            if !self.faces.contains(face) {
                self.faces.push(face.clone());
            }
        }

        // Add all wires
        for wire in &other.wires {
            if !self.wires.contains(wire) {
                self.wires.push(wire.clone());
            }
        }

        // Add all edges
        for edge in &other.edges {
            if !self.edges.contains(edge) {
                self.edges.push(edge.clone());
            }
        }

        // Add all vertices
        for vertex in &other.vertices {
            if !self.vertices.contains(vertex) {
                self.vertices.push(vertex.clone());
            }
        }

        // Rebuild topology
        self.build_topology();
    }

    /// Validate the model for correctness
    pub fn validate(&self) -> Result<(), String> {
        // Check for duplicate elements
        let mut vertex_set = HashSet::new();
        for vertex in &self.vertices {
            if !vertex_set.insert(vertex) {
                return Err("Duplicate vertex found".to_string());
            }
        }

        let mut edge_set = HashSet::new();
        for edge in &self.edges {
            if !edge_set.insert(edge) {
                return Err("Duplicate edge found".to_string());
            }
        }

        let mut face_set = HashSet::new();
        for face in &self.faces {
            if !face_set.insert(face) {
                return Err("Duplicate face found".to_string());
            }
        }

        // Check edge vertex consistency
        for edge in &self.edges {
            if let Some((v1, v2)) = self.topology.edge_vertices.get(edge) {
                if !self.vertices.contains(v1) || !self.vertices.contains(v2) {
                    return Err("Edge references non-existent vertices".to_string());
                }
            }
        }

        // Check face edge consistency
        for face in &self.faces {
            if let Some(edges) = self.topology.face_edges.get(face) {
                for edge in edges {
                    if !self.edges.contains(edge) {
                        return Err("Face references non-existent edge".to_string());
                    }
                }
            }
        }

        Ok(())
    }

    /// Repair common issues in the model
    pub fn repair(&mut self) {
        // Remove duplicate vertices
        let mut unique_vertices: Vec<Handle<TopoDsVertex>> = Vec::new();

        for vertex in &self.vertices {
            let point = vertex.as_ref().unwrap().point();
            let mut is_duplicate = false;

            // Check if vertex already exists in unique_vertices
            for existing_vertex in &unique_vertices {
                let existing_point = existing_vertex.as_ref().unwrap().point();
                // Compare with tolerance
                if (point.x - existing_point.x).abs() < 1e-6
                    && (point.y - existing_point.y).abs() < 1e-6
                    && (point.z - existing_point.z).abs() < 1e-6
                {
                    is_duplicate = true;
                    break;
                }
            }

            if !is_duplicate {
                unique_vertices.push(vertex.clone());
            }
        }

        self.vertices = unique_vertices;

        // Rebuild topology
        self.build_topology();
    }

    /// Convert the model to a compound shape
    pub fn to_compound(&self) -> TopoDsCompound {
        let mut compound = TopoDsCompound::new();

        for solid in &self.solids {
            compound.add_component(Handle::new(std::sync::Arc::new(
                solid.as_ref().unwrap().shape().clone(),
            )));
        }

        // Add shells not part of solids
        for shell in &self.shells {
            let mut is_in_solid = false;
            for solid in &self.solids {
                if let Some(shells) = self.topology.solid_shells.get(solid) {
                    if shells.contains(shell) {
                        is_in_solid = true;
                        break;
                    }
                }
            }
            if !is_in_solid {
                compound.add_component(Handle::new(std::sync::Arc::new(
                    shell.as_ref().unwrap().shape().clone(),
                )));
            }
        }

        compound
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

    /// Add a shell to the model
    pub fn add_shell(&mut self, shell: TopoDsShell) -> &mut Self {
        self.model.add_shell(shell);
        self
    }

    /// Add a face to the model
    pub fn add_face(&mut self, face: TopoDsFace) -> &mut Self {
        self.model.add_face(face);
        self
    }

    /// Add a wire to the model
    pub fn add_wire(&mut self, wire: TopoDsWire) -> &mut Self {
        self.model.add_wire(wire);
        self
    }

    /// Add an edge to the model
    pub fn add_edge(&mut self, edge: TopoDsEdge) -> &mut Self {
        self.model.add_edge(edge);
        self
    }

    /// Add a vertex to the model
    pub fn add_vertex(&mut self, vertex: TopoDsVertex) -> &mut Self {
        self.model.add_vertex(vertex);
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
    use crate::geometry::{Point, Vector};

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

    #[test]
    fn test_model_validation() {
        let mut builder = BrepBuilder::new();

        // Create a simple solid
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);

        let model = builder.build();

        // Validate the model
        let result = model.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_model_merge() {
        // Create first model
        let mut builder1 = BrepBuilder::new();
        let solid1 = TopoDsSolid::new();
        builder1.add_solid(solid1);
        let model1 = builder1.build();

        // Create second model
        let mut builder2 = BrepBuilder::new();
        let solid2 = TopoDsSolid::new();
        builder2.add_solid(solid2);
        let model2 = builder2.build();

        // Merge models
        let mut merged_model = model1;
        merged_model.merge(&model2);

        assert_eq!(merged_model.solids.len(), 2);
    }

    #[test]
    fn test_model_transform() {
        let mut builder = BrepBuilder::new();
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        let mut model = builder.build();

        // Create a translation transform
        let translation_vector = Vector::new(1.0, 1.0, 1.0);
        let transform = Transform::from_translation(&translation_vector);

        // Apply transform
        model.transform(&transform);

        // Validate the model after transform
        let result = model.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_model_repair() {
        let mut builder = BrepBuilder::new();
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        let mut model = builder.build();

        // Repair the model
        model.repair();

        // Validate the model after repair
        let result = model.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_to_compound() {
        let mut builder = BrepBuilder::new();
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        let model = builder.build();

        // Convert to compound
        let compound = model.to_compound();
        assert!(!compound.components().is_empty());
    }

    #[test]
    fn test_topology_queries() {
        let mut builder = BrepBuilder::new();
        let solid = TopoDsSolid::new();
        builder.add_solid(solid);
        let model = builder.build();

        // Test topology queries
        if !model.solids.is_empty() {
            let solid = &model.solids[0];
            let shells = model.get_shells_from_solid(solid);
            assert!(!shells.is_empty());

            for shell in &shells {
                let faces = model.get_faces_from_shell(shell);
                for face in &faces {
                    let edges = model.get_edges_from_face(face);
                    let wires = model.get_wires_from_face(face);
                    let vertices = model.get_vertices_from_face(face);

                    for edge in &edges {
                        let edge_faces = model.get_faces_from_edge(edge);
                        assert!(!edge_faces.is_empty());
                    }

                    for wire in &wires {
                        let wire_vertices = model.get_vertices_from_wire(wire);
                        assert!(!wire_vertices.is_empty());
                    }

                    for vertex in &vertices {
                        let vertex_edges = model.get_edges_from_vertex(vertex);
                        assert!(!vertex_edges.is_empty());
                    }
                }
            }
        }
    }
}
