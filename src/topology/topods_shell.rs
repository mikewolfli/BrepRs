use crate::foundation::handle::Handle;
use crate::geometry::Point;
use crate::topology::{
    topods_face::TopoDsFace, topods_location::TopoDsLocation, topods_shape::TopoDsShape,
};
use serde::{Deserialize, Serialize};

/// Represents a shell in topological structure
///
/// A shell is a set of faces connected by edges. Shells can be
/// open or closed. A closed shell can represent the boundary
/// of a solid.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TopoDsShell {
    shape: TopoDsShape,
    faces: Vec<Handle<TopoDsFace>>,
    closed: bool,
    tolerance: f64,
}

impl TopoDsShell {
    /// Create a new empty shell
    pub fn new() -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Shell),
            faces: Vec::new(),
            closed: false,
            tolerance: 0.001,
        }
    }

    /// Create a new shell with specified faces
    pub fn with_faces(faces: Vec<Handle<TopoDsFace>>) -> Self {
        let mut shell = Self::new();
        for face in faces {
            shell.add_face(face);
        }
        shell.update_closed();
        shell
    }

    /// Create a new shell with specified tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            shape: TopoDsShape::new(crate::topology::shape_enum::ShapeType::Shell),
            faces: Vec::new(),
            closed: false,
            tolerance,
        }
    }

    /// Add a face to the shell
    pub fn add_face(&mut self, face: Handle<TopoDsFace>) {
        self.faces.push(face);
        self.update_closed();
    }

    /// Get the faces of the shell
    pub fn faces(&self) -> &[Handle<TopoDsFace>] {
        &self.faces
    }

    /// Get mutable reference to the faces of the shell
    pub fn faces_mut(&mut self) -> &mut [Handle<TopoDsFace>] {
        &mut self.faces
    }

    /// Get the number of faces in the shell
    pub fn num_faces(&self) -> usize {
        self.faces.len()
    }

    /// Check if the shell is closed
    pub fn is_closed(&self) -> bool {
        self.closed
    }

    /// Update the closed status of the shell
    pub fn update_closed(&mut self) {
        if self.faces.is_empty() {
            self.closed = false;
            return;
        }

        self.closed = self.check_closed();
    }

    /// Check if the shell is closed by checking edge connectivity
    fn check_closed(&self) -> bool {
        use std::collections::HashMap;

        let mut edge_count: HashMap<i32, i32> = HashMap::new();

        for face in &self.faces {
            if let Some(outer_wire) = face.outer_wire() {
                for edge in outer_wire.edges() {
                    let edge_id = edge.shape_id();
                    *edge_count.entry(edge_id).or_insert(0) += 1;
                }
            }
        }

        edge_count.values().all(|&count| count == 2)
    }

    /// Get the tolerance of the shell
    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }

    /// Set the tolerance of the shell
    pub fn set_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }

    /// Get the shape base
    pub fn shape(&self) -> &TopoDsShape {
        &self.shape
    }

    /// Get mutable reference to shape base
    pub fn shape_mut(&mut self) -> &mut TopoDsShape {
        &mut self.shape
    }

    /// Get the location of the shell
    pub fn location(&self) -> Option<&TopoDsLocation> {
        self.shape.location()
    }

    /// Set the location of the shell
    pub fn set_location(&mut self, location: TopoDsLocation) {
        self.shape.set_location(location);
    }

    /// Check if the shell is empty
    pub fn is_empty(&self) -> bool {
        self.faces.is_empty()
    }

    /// Clear all faces from the shell
    pub fn clear(&mut self) {
        self.faces.clear();
        self.closed = false;
    }

    /// Get the total surface area of the shell
    pub fn area(&self) -> f64 {
        self.faces.iter().map(|f| f.area()).sum()
    }

    /// Get the first face of the shell
    pub fn first_face(&self) -> Option<&Handle<TopoDsFace>> {
        self.faces.first()
    }

    /// Get the last face of the shell
    pub fn last_face(&self) -> Option<&Handle<TopoDsFace>> {
        self.faces.last()
    }

    /// Get the unique identifier of the shell
    pub fn shape_id(&self) -> i32 {
        self.shape.shape_id()
    }

    /// Set the unique identifier of the shell
    pub fn set_shape_id(&mut self, id: i32) {
        self.shape.set_shape_id(id);
    }

    /// Check if this shell is mutable
    pub fn is_mutable(&self) -> bool {
        self.shape.is_mutable()
    }

    /// Set the mutability of the shell
    pub fn set_mutable(&mut self, mutable: bool) {
        self.shape.set_mutable(mutable);
    }

    /// Check if the shell contains a specific face
    pub fn contains_face(&self, face: &Handle<TopoDsFace>) -> bool {
        self.faces.contains(face)
    }

    /// Remove a face from the shell
    pub fn remove_face(&mut self, face: &Handle<TopoDsFace>) {
        self.faces.retain(|f| f != face);
        self.update_closed();
    }

    /// Get the bounding box of the shell
    pub fn bounding_box(&self) -> Option<(Point, Point)> {
        if self.faces.is_empty() {
            return None;
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;

        for face in &self.faces {
            if let Some((min, max)) = face.bounding_box() {
                min_x = min_x.min(min.x);
                min_y = min_y.min(min.y);
                min_z = min_z.min(min.z);
                max_x = max_x.max(max.x);
                max_y = max_y.max(max.y);
                max_z = max_z.max(max.z);
            }
        }

        Some((
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        ))
    }

    /// Get all edges in the shell
    pub fn edges(&self) -> Vec<Handle<crate::topology::topods_edge::TopoDsEdge>> {
        use std::collections::HashSet;

        let mut edge_set = HashSet::new();
        let mut edges = Vec::new();

        for face in &self.faces {
            if let Some(outer_wire) = face.outer_wire() {
                for edge in outer_wire.edges() {
                    if edge_set.insert(edge.shape_id()) {
                        edges.push(edge.clone());
                    }
                }
            }
        }

        edges
    }

    /// Get all vertices in the shell
    pub fn vertices(&self) -> Vec<Handle<crate::topology::topods_vertex::TopoDsVertex>> {
        use std::collections::HashSet;

        let mut vertex_set = HashSet::new();
        let mut vertices = Vec::new();

        for face in &self.faces {
            if let Some(outer_wire) = face.outer_wire() {
                for vertex in outer_wire.vertices() {
                    if vertex_set.insert(vertex.shape_id()) {
                        vertices.push(vertex.clone());
                    }
                }
            }
        }

        vertices
    }

    /// Check if the shell is manifold (no non-manifold edges or vertices)
    pub fn is_manifold(&self) -> bool {
        use std::collections::HashMap;

        let mut edge_count: HashMap<i32, i32> = HashMap::new();

        for face in &self.faces {
            if let Some(outer_wire) = face.outer_wire() {
                for edge in outer_wire.edges() {
                    let edge_id = edge.shape_id();
                    *edge_count.entry(edge_id).or_insert(0) += 1;
                }
            }
        }

        edge_count.values().all(|&count| count == 1 || count == 2)
    }

    /// Get the number of edges in the shell
    pub fn num_edges(&self) -> usize {
        self.edges().len()
    }

    /// Get the number of vertices in the shell
    pub fn num_vertices(&self) -> usize {
        self.vertices().len()
    }
}

impl Default for TopoDsShell {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TopoDsShell {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            faces: self.faces.clone(),
            closed: self.closed,
            tolerance: self.tolerance,
        }
    }
}

impl PartialEq for TopoDsShell {
    fn eq(&self, other: &Self) -> bool {
        self.shape_id() == other.shape_id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_creation() {
        let shell = TopoDsShell::new();
        assert!(shell.is_empty());
        assert_eq!(shell.num_faces(), 0);
    }

    #[test]
    fn test_shell_add_face() {
        let mut shell = TopoDsShell::new();
        let face = Handle::new(std::sync::Arc::new(TopoDsFace::new()));

        shell.add_face(face);
        assert_eq!(shell.num_faces(), 1);
    }

    #[test]
    fn test_shell_clear() {
        let face = Handle::new(std::sync::Arc::new(TopoDsFace::new()));
        let mut shell = TopoDsShell::with_faces(vec![face]);
        assert!(!shell.is_empty());

        shell.clear();
        assert!(shell.is_empty());
    }

    #[test]
    fn test_shell_shape_id() {
        let mut shell = TopoDsShell::new();
        // shape_id is now auto-generated, so it should not be 0
        let initial_id = shell.shape_id();
        assert!(initial_id > 0);

        shell.set_shape_id(42);
        assert_eq!(shell.shape_id(), 42);
    }

    #[test]
    fn test_shell_mutable() {
        let mut shell = TopoDsShell::new();
        assert!(!shell.is_mutable());

        shell.set_mutable(true);
        assert!(shell.is_mutable());
    }

    #[test]
    fn test_shell_clone() {
        let mut shell1 = TopoDsShell::new();
        shell1.set_shape_id(10);

        let shell2 = shell1.clone();
        assert_eq!(shell2.shape_id(), 10);
        assert_eq!(shell1, shell2);
    }

    #[test]
    fn test_shell_tolerance() {
        let shell = TopoDsShell::with_tolerance(0.01);
        assert_eq!(shell.tolerance(), 0.01);
    }
}
