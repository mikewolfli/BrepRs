use crate::foundation::handle::Handle;
use crate::geometry::{Plane, Point};
use crate::topology::{TopoDsFace, TopoDsShape};
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BspNode {
    pub plane: Plane,
    pub front: Option<Box<BspNode>>,
    pub back: Option<Box<BspNode>>,
    pub faces: Vec<Arc<TopoDsFace>>,
}

impl BspNode {
    pub fn new(plane: Plane) -> Self {
        Self {
            plane,
            front: None,
            back: None,
            faces: Vec::new(),
        }
    }

    pub fn insert(&mut self, face: &TopoDsFace, tolerance: f64) {
        // Classify face relative to node plane
        let classification = self.classify_face(face, tolerance);

        match classification {
            FaceClassification::Front => {
                if let Some(ref mut front) = self.front {
                    front.insert(face, tolerance);
                } else {
                    let plane = self.create_splitting_plane(face);
                    let mut new_node = BspNode::new(plane);
                    new_node.faces.push(Arc::new(face.clone()));
                    self.front = Some(Box::new(new_node));
                }
            }
            FaceClassification::Back => {
                if let Some(ref mut back) = self.back {
                    back.insert(face, tolerance);
                } else {
                    let plane = self.create_splitting_plane(face);
                    let mut new_node = BspNode::new(plane);
                    new_node.faces.push(Arc::new(face.clone()));
                    self.back = Some(Box::new(new_node));
                }
            }
            FaceClassification::Coplanar => {
                self.faces.push(Arc::new(face.clone()));
            }
            FaceClassification::Spanning => {
                // Split face and insert both parts
                if let Some((front_face, back_face)) = self.split_face(face, tolerance) {
                    self.insert(&front_face, tolerance);
                    self.insert(&back_face, tolerance);
                }
            }
        }
    }

    fn classify_face(&self, face: &TopoDsFace, tolerance: f64) -> FaceClassification {
        // Get face vertices
        let vertices = face.vertices();
        if vertices.is_empty() {
            return FaceClassification::Coplanar;
        }

        let mut front_count = 0;
        let mut back_count = 0;

        for vertex in vertices {
            if let Some(vertex_ref) = vertex.as_ref() {
                let point = vertex_ref.point();
                let distance = self.plane.distance(&point);

                if distance > tolerance {
                    front_count += 1;
                } else if distance < -tolerance {
                    back_count += 1;
                }
            }
        }

        if front_count > 0 && back_count > 0 {
            FaceClassification::Spanning
        } else if front_count > 0 {
            FaceClassification::Front
        } else if back_count > 0 {
            FaceClassification::Back
        } else {
            FaceClassification::Coplanar
        }
    }

    fn split_face(&self, face: &TopoDsFace, tolerance: f64) -> Option<(TopoDsFace, TopoDsFace)> {
        // Get face wires and vertices
        let wires = face.wires();
        if wires.is_empty() {
            return None;
        }

        // Get the outer wire
        let outer_wire = wires[0].get()?;
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();
        if edges_slice.len() < 3 {
            return None;
        }

        // Classify each vertex of the face
        let mut front_count = 0;
        let mut back_count = 0;

        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let Some(start) = edge.start_vertex().get() {
                    let point = start.point();
                    let distance = self.plane.distance(&point);

                    if distance > tolerance {
                        front_count += 1;
                    } else if distance < -tolerance {
                        back_count += 1;
                    }
                }
            }
        }

        // If all vertices are on one side, no splitting needed
        if front_count == 0 || back_count == 0 {
            return None;
        }

        // Create front face with front vertices and on-plane vertices
        let front_face = face.clone();
        // Create back face with back vertices and on-plane vertices
        let back_face = face.clone();

        // For a proper implementation, we would need to:
        // 1. Find intersection points between face edges and the splitting plane
        // 2. Create new edges along the intersection
        // 3. Reconstruct the face polygons for both sides
        // 4. Create new faces with proper topology

        // Simplified approach: return the original face for both sides
        // This maintains the geometry while allowing the BSP tree to proceed
        Some((front_face, back_face))
    }

    fn create_splitting_plane(&self, face: &TopoDsFace) -> Plane {
        // Create a plane from the face's surface
        if let Some(surface) = face.surface() {
            if let Some(surface_ref) = surface.as_ref() {
                // Get surface parameters and create a plane
                let (u_range, v_range) = surface_ref.parameter_range();
                let u_mid = (u_range.0 + u_range.1) / 2.0;
                let v_mid = (v_range.0 + v_range.1) / 2.0;
                let point = surface_ref.value(u_mid, v_mid);
                let normal = surface_ref.normal(u_mid, v_mid);
                return Plane::new(point, crate::geometry::Direction::from_vector(&normal), crate::geometry::Direction::x_axis());
            }
        }

        // Fallback plane
        Plane::new(
            Point::origin(),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        )
    }

    pub fn collect_faces(&self) -> Vec<TopoDsFace> {
        let mut faces = Vec::new();

        // Add current node's faces
        for face in &self.faces {
            faces.push(face.as_ref().clone());
        }

        // Recursively collect from children
        if let Some(ref front) = self.front {
            faces.extend(front.collect_faces());
        }
        if let Some(ref back) = self.back {
            faces.extend(back.collect_faces());
        }

        faces
    }

    pub fn collect_faces_parallel(&self) -> Vec<TopoDsFace> {
        let mut faces: Vec<TopoDsFace> =
            self.faces.par_iter().map(|f| f.as_ref().clone()).collect();

        // Collect from front child
        if let Some(ref front) = self.front {
            let front_faces = front.collect_faces_parallel();
            faces.extend(front_faces);
        }

        // Collect from back child
        if let Some(ref back) = self.back {
            let back_faces = back.collect_faces_parallel();
            faces.extend(back_faces);
        }

        faces
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum FaceClassification {
    Front,
    Back,
    Coplanar,
    Spanning,
}

#[derive(Debug, Clone)]
pub struct BspTree {
    pub root: Option<Box<BspNode>>,
    pub tolerance: f64,
}

impl BspTree {
    pub fn new(tolerance: f64) -> Self {
        Self {
            root: None,
            tolerance,
        }
    }

    pub fn insert_face(&mut self, face: TopoDsFace) {
        if let Some(ref mut root) = self.root {
            root.insert(&face, self.tolerance);
        } else {
            // Create initial plane from face
            let plane = self.create_initial_plane(&face);
            let mut node = BspNode::new(plane);
            node.faces.push(Arc::new(face));
            self.root = Some(Box::new(node));
        }
    }

    fn create_initial_plane(&self, face: &TopoDsFace) -> Plane {
        if let Some(surface) = face.surface() {
            if let Some(surface_ref) = surface.as_ref() {
                let (u_range, v_range) = surface_ref.parameter_range();
                let u_mid = (u_range.0 + u_range.1) / 2.0;
                let v_mid = (v_range.0 + v_range.1) / 2.0;
                let point = surface_ref.value(u_mid, v_mid);
                let normal = surface_ref.normal(u_mid, v_mid);
                return Plane::new(point, crate::geometry::Direction::from_vector(&normal), crate::geometry::Direction::x_axis());
            }
        }

        // Fallback plane
        Plane::new(
            Point::origin(),
            crate::geometry::Direction::z_axis(),
            crate::geometry::Direction::x_axis(),
        )
    }

    pub fn build(&mut self, faces: &[TopoDsFace]) {
        for face in faces {
            self.insert_face(face.clone());
        }
    }

    pub fn build_parallel(&mut self, faces: &[TopoDsFace]) {
        // For small number of faces, build sequentially
        if faces.len() < 10 {
            for face in faces {
                self.insert_face(face.clone());
            }
            return;
        }

        // Build initial tree with first face
        if self.root.is_none() && !faces.is_empty() {
            let plane = self.create_initial_plane(&faces[0]);
            let mut node = BspNode::new(plane);
            node.faces.push(Arc::new(faces[0].clone()));
            self.root = Some(Box::new(node));
        }

        // Insert remaining faces in parallel
        if let Some(ref mut root) = self.root {
            faces[1..].par_iter().for_each(|face| {
                let mut node = root.clone();
                node.insert(face, self.tolerance);
            });
        }
    }

    pub fn union(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Add all faces from self
        let self_faces = self.collect_all_faces();
        for face in self_faces {
            result.insert_face(face);
        }

        // Add all faces from other that aren't already present
        let other_faces = other.collect_all_faces();
        let existing_shape_ids: std::collections::HashSet<i32> = result
            .collect_all_faces()
            .iter()
            .map(|f| f.shape_id())
            .collect();

        for face in other_faces {
            if !existing_shape_ids.contains(&face.shape_id()) {
                result.insert_face(face);
            }
        }

        result
    }

    pub fn difference(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Get all faces from self
        let self_faces = self.collect_all_faces();
        let other_faces = other.collect_all_faces();

        // Create set of other shape IDs
        let other_shape_ids: std::collections::HashSet<i32> =
            other_faces.iter().map(|f| f.shape_id()).collect();

        // Add faces from self that aren't in other
        for face in self_faces {
            if !other_shape_ids.contains(&face.shape_id()) {
                result.insert_face(face);
            }
        }

        result
    }

    pub fn intersection(&self, other: &BspTree) -> BspTree {
        let mut result = BspTree::new(self.tolerance);

        // Get all faces from both trees
        let self_faces = self.collect_all_faces();
        let other_faces = other.collect_all_faces();

        // Create set of self shape IDs
        let self_shape_ids: std::collections::HashSet<i32> =
            self_faces.iter().map(|f| f.shape_id()).collect();

        // Add faces from other that are also in self
        for face in other_faces {
            if self_shape_ids.contains(&face.shape_id()) {
                result.insert_face(face);
            }
        }

        result
    }

    pub fn collect_all_faces(&self) -> Vec<TopoDsFace> {
        if let Some(ref root) = self.root {
            root.collect_faces()
        } else {
            Vec::new()
        }
    }

    pub fn collect_all_faces_parallel(&self) -> Vec<TopoDsFace> {
        if let Some(ref root) = self.root {
            root.collect_faces_parallel()
        } else {
            Vec::new()
        }
    }

    /// Merge another BSP tree into this tree
    pub fn merge_tree(&mut self, other: &BspNode) {
        // Collect all faces from the other tree
        let faces = other.collect_faces();
        for face in faces {
            self.insert_face(face);
        }
    }

    pub fn difference_with_tolerance(&self, other: &BspTree, tolerance: f64) -> BspTree {
        let mut result = BspTree::new(tolerance);

        // Get all faces from self
        let self_faces = self.collect_all_faces();
        let other_faces = other.collect_all_faces();

        // Add faces from self that are not within tolerance of other faces
        for face in self_faces {
            let mut found = false;
            for other_face in &other_faces {
                if self.faces_are_close(&face, other_face, tolerance) {
                    found = true;
                    break;
                }
            }
            if !found {
                result.insert_face(face);
            }
        }

        result
    }

    fn faces_are_close(&self, face1: &TopoDsFace, face2: &TopoDsFace, tolerance: f64) -> bool {
        // Check if faces are close based on bounding boxes
        if let (Some((min1, _max1)), Some((min2, _max2))) = (face1.bounding_box(), face2.bounding_box()) {
            // Check if bounding boxes overlap within tolerance
            let dx = (min1.x - min2.x).abs();
            let dy = (min1.y - min2.y).abs();
            let dz = (min1.z - min2.z).abs();

            return dx <= tolerance && dy <= tolerance && dz <= tolerance;
        }
        
        false
    }
}

/// BSP tree builder
#[derive(Debug, Clone)]
pub struct BspTreeBuilder {
    tolerance: f64,
}

impl BspTreeBuilder {
    /// Create a new BSP tree builder
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    /// Build BSP tree from a shape
    pub fn build_from_shape(&self, shape: &Handle<TopoDsShape>) -> BspTree {
        let mut tree = BspTree::new(self.tolerance);

        // Extract faces from the shape based on its type
        match shape.shape_type() {
            crate::topology::ShapeType::Solid => {
                // For solids, get faces from the solid's shells
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            for face_handle in shell.faces() {
                                if let Some(face) = face_handle.get() {
                                    tree.insert_face(face.as_ref().clone());
                                }
                            }
                        }
                    }
                }
            }
            crate::topology::ShapeType::Shell => {
                // For shells, extract faces directly
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            tree.insert_face(face.as_ref().clone());
                        }
                    }
                }
            }
            crate::topology::ShapeType::Face => {
                // For faces, add the face directly
                if let Some(face) = shape.as_face() {
                    tree.insert_face(face.clone());
                }
            }
            crate::topology::ShapeType::Compound => {
                // For compounds, recursively extract faces from components
                if let Some(compound) = shape.as_compound() {
                    for component in compound.components() {
                        let component_handle = Handle::new(std::sync::Arc::new(component.clone()));
                        let component_tree = self.build_from_shape(&component_handle);
                        if let Some(root) = component_tree.root {
                            tree.merge_tree(&root);
                        }
                    }
                }
            }
            _ => {
                // For other types, no faces to extract
            }
        }

        tree
    }
}

pub trait BspCommand {
    fn apply(&mut self, tree: &mut BspTree);
    fn undo(&mut self, tree: &mut BspTree);
}
