//! BSP Tree implementation for boolean operations
//! 
//! This module provides a Binary Space Partitioning (BSP) tree implementation
//! for efficient boolean operations on 3D shapes.

use crate::geometry::{Plane, Point, Vector};
use crate::topology::TopoDsFace;

/// BSP Tree node
#[derive(Debug, Clone)]
pub struct BspNode {
    /// Splitting plane
    pub plane: Plane,
    /// Front child (positive side of the plane)
    pub front: Option<Box<BspNode>>,
    /// Back child (negative side of the plane)
    pub back: Option<Box<BspNode>>,
    /// Faces in this node
    pub faces: Vec<TopoDsFace>,
}

impl BspNode {
    /// Create a new BSP node with the given plane
    pub fn new(plane: Plane) -> Self {
        Self {
            plane,
            front: None,
            back: None,
            faces: Vec::new(),
        }
    }

    /// Insert a face into the BSP tree
    pub fn insert(&mut self, face: TopoDsFace, tolerance: f64) {
        // Check if the face is on the plane
        if self.is_face_on_plane(&face, tolerance) {
            self.faces.push(face);
            return;
        }

        // Determine which side of the plane the face is on
        let side = self.classify_face(&face, tolerance);

        match side {
            FaceClassification::Front => {
                if let Some(ref mut front) = self.front {
                    front.insert(face, tolerance);
                } else {
                    // Create a new front child with a plane from the face
                    let new_plane = self.create_plane_from_face(&face);
                    let mut new_node = BspNode::new(new_plane);
                    new_node.faces.push(face);
                    self.front = Some(Box::new(new_node));
                }
            }
            FaceClassification::Back => {
                if let Some(ref mut back) = self.back {
                    back.insert(face, tolerance);
                } else {
                    // Create a new back child with a plane from the face
                    let new_plane = self.create_plane_from_face(&face);
                    let mut new_node = BspNode::new(new_plane);
                    new_node.faces.push(face);
                    self.back = Some(Box::new(new_node));
                }
            }
            FaceClassification::Spanning => {
                // Face spans the plane, split it
                let (front_face, back_face) = self.split_face(&face, tolerance);
                if let Some(front_face) = front_face {
                    if let Some(ref mut front) = self.front {
                        front.insert(front_face, tolerance);
                    } else {
                        let new_plane = self.create_plane_from_face(&front_face);
                        let mut new_node = BspNode::new(new_plane);
                        new_node.faces.push(front_face);
                        self.front = Some(Box::new(new_node));
                    }
                }
                if let Some(back_face) = back_face {
                    if let Some(ref mut back) = self.back {
                        back.insert(back_face, tolerance);
                    } else {
                        let new_plane = self.create_plane_from_face(&back_face);
                        let mut new_node = BspNode::new(new_plane);
                        new_node.faces.push(back_face);
                        self.back = Some(Box::new(new_node));
                    }
                }
            }
        }
    }

    /// Classify a face with respect to the node's plane
    fn classify_face(&self, face: &TopoDsFace, tolerance: f64) -> FaceClassification {
        // Get the face's vertices
        let vertices = face.vertices();
        if vertices.is_empty() {
            return FaceClassification::Front;
        }

        let mut front_count = 0;
        let mut back_count = 0;

        for vertex in vertices {
            let point = vertex.point();
            let distance = self.plane.distance(&point);

            if distance > tolerance {
                front_count += 1;
            } else if distance < -tolerance {
                back_count += 1;
            }
        }

        if front_count > 0 && back_count > 0 {
            FaceClassification::Spanning
        } else if front_count > 0 {
            FaceClassification::Front
        } else {
            FaceClassification::Back
        }
    }

    /// Check if a face is on the plane
    fn is_face_on_plane(&self, face: &TopoDsFace, tolerance: f64) -> bool {
        let vertices = face.vertices();
        for vertex in vertices {
            let point = vertex.point();
            let distance = self.plane.distance(&point);
            if distance.abs() > tolerance {
                return false;
            }
        }
        true
    }

    /// Split a face by the node's plane
    fn split_face(&self, face: &TopoDsFace, tolerance: f64) -> (Option<TopoDsFace>, Option<TopoDsFace>) {
        // Find intersection points between the face and the splitting plane
        let mut front_vertices = Vec::new();
        let mut back_vertices = Vec::new();
        let mut intersection_vertices = Vec::new();
        let plane = &self.plane;
        let wire = match face.outer_wire() {
            Some(w) => w,
            None => return (None, None),
        };
        let vertices = wire.vertices();
        if vertices.len() < 3 {
            return (None, None);
        }
        for i in 0..vertices.len() {
            let v1 = &vertices[i];
            let v2 = &vertices[(i + 1) % vertices.len()];
            let p1 = v1.point();
            let p2 = v2.point();
            let d1 = plane.distance(&p1);
            let d2 = plane.distance(&p2);
            if d1 > tolerance {
                front_vertices.push(v1.clone());
            } else if d1 < -tolerance {
                back_vertices.push(v1.clone());
            }
            // Check for intersection
            if (d1 > tolerance && d2 < -tolerance) || (d1 < -tolerance && d2 > tolerance) {
                // Linear interpolation for intersection point
                let t = d1 / (d1 - d2);
                let ix = p1.x + t * (p2.x - p1.x);
                let iy = p1.y + t * (p2.y - p1.y);
                let iz = p1.z + t * (p2.z - p1.z);
                let intersection = Handle::new(std::sync::Arc::new(crate::topology::topods_vertex::TopoDsVertex::new(
                    crate::geometry::Point::new(ix, iy, iz)
                )));
                intersection_vertices.push(intersection.clone());
                front_vertices.push(intersection.clone());
                back_vertices.push(intersection.clone());
            }
        }
        // Build new wires for each side
        let mut front_wire = crate::topology::topods_wire::TopoDsWire::new();
        for v in &front_vertices {
            // Create edges between consecutive vertices
            if front_wire.num_vertices() > 0 {
                let prev = front_wire.vertices()[front_wire.num_vertices() - 1].clone();
                let edge = Handle::new(std::sync::Arc::new(crate::topology::topods_edge::TopoDsEdge::new(prev, v.clone())));
                front_wire.add_edge(edge);
            } else {
                front_wire.vertices().push(v.clone());
            }
        }
        front_wire.update_closed();
        let mut back_wire = crate::topology::topods_wire::TopoDsWire::new();
        for v in &back_vertices {
            if back_wire.num_vertices() > 0 {
                let prev = back_wire.vertices()[back_wire.num_vertices() - 1].clone();
                let edge = Handle::new(std::sync::Arc::new(crate::topology::topods_edge::TopoDsEdge::new(prev, v.clone())));
                back_wire.add_edge(edge);
            } else {
                back_wire.vertices().push(v.clone());
            }
        }
        back_wire.update_closed();
        // Create new faces
        let front_face = if front_wire.num_vertices() >= 3 {
            Some(crate::topology::topods_face::TopoDsFace::with_outer_wire(front_wire))
        } else {
            None
        };
        let back_face = if back_wire.num_vertices() >= 3 {
            Some(crate::topology::topods_face::TopoDsFace::with_outer_wire(back_wire))
        } else {
            None
        };
        (front_face, back_face)
    }

    /// Create a plane from a face
    fn create_plane_from_face(&self, face: &TopoDsFace) -> Plane {
        // Create a plane from the first three vertices of the outer wire
        if let Some(wire) = face.outer_wire() {
            let vertices = wire.vertices();
            if vertices.len() >= 3 {
                let p1 = vertices[0].point().clone();
                let p2 = vertices[1].point().clone();
                let p3 = vertices[2].point().clone();
                if let Some(plane) = crate::geometry::Plane::from_points(p1, p2, p3) {
                    return plane;
                }
            }
        }
        // Fallback: best-fit plane from all face points
        let points: Vec<_> = face.all_points();
        if points.len() >= 3 {
            if let Some(plane) = crate::geometry::Plane::best_fit(&points) {
                return plane;
            }
        }
        // Absolute fallback: return default plane
        Plane::new(Point::origin(), crate::geometry::Direction::z_axis(), crate::geometry::Direction::x_axis())
    }
}

/// Face classification with respect to a plane
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FaceClassification {
    /// Face is entirely on the front side of the plane
    Front,
    /// Face is entirely on the back side of the plane
    Back,
    /// Face spans the plane (intersects it)
    Spanning,
}

/// BSP Tree
#[derive(Debug, Clone)]
pub struct BspTree {
    /// Root node
    pub root: Option<Box<BspNode>>,
    /// Tolerance for geometric operations
    pub tolerance: f64,
}

impl BspTree {
    /// Create a new BSP tree with the given tolerance
    pub fn new(tolerance: f64) -> Self {
        Self {
            root: None,
            tolerance,
        }
    }

    /// Build the BSP tree from a list of faces
    pub fn build(&mut self, faces: &[TopoDsFace]) {
        for face in faces {
            self.insert_face(face.clone());
        }
    }

    /// Insert a face into the BSP tree
    pub fn insert_face(&mut self, face: TopoDsFace) {
        if let Some(ref mut root) = self.root {
            root.insert(face, self.tolerance);
        } else {
            // Create root node with a plane from the first face
            let plane = self.create_plane_from_face(&face);
            let mut root_node = BspNode::new(plane);
            root_node.faces.push(face);
            self.root = Some(Box::new(root_node));
        }
    }

    /// Create a plane from a face
    fn create_plane_from_face(&self, face: &TopoDsFace) -> Plane {
        // TODO: Implement plane creation from face
        // For now, return a default plane
        Plane::new(Point::origin(), crate::geometry::Direction::z_axis(), crate::geometry::Direction::x_axis())
    }

    /// Perform boolean union with another BSP tree
    pub fn union(&self, other: &BspTree) -> BspTree {
        // Boolean union: merge faces from both trees
        let mut result = BspTree::new(self.tolerance);
        if let Some(ref root) = self.root {
            for face in &root.faces {
                result.insert_face(face.clone());
            }
        }
        if let Some(ref other_root) = other.root {
            for face in &other_root.faces {
                result.insert_face(face.clone());
            }
        }
        result
    }

    /// Perform boolean difference with another BSP tree
    pub fn difference(&self, other: &BspTree) -> BspTree {
        // Boolean difference: keep faces in self not in other
        let mut result = BspTree::new(self.tolerance);
        if let Some(ref root) = self.root {
            for face in &root.faces {
                // Check if face exists in other
                let mut found = false;
                if let Some(ref other_root) = other.root {
                    for other_face in &other_root.faces {
                        if face.shape_id() == other_face.shape_id() {
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    result.insert_face(face.clone());
                }
            }
        }
        result
    }

    /// Perform boolean intersection with another BSP tree
    pub fn intersection(&self, other: &BspTree) -> BspTree {
        // Boolean intersection: keep faces present in both trees
        let mut result = BspTree::new(self.tolerance);
        if let Some(ref root) = self.root {
            if let Some(ref other_root) = other.root {
                for face in &root.faces {
                    for other_face in &other_root.faces {
                        if face.shape_id() == other_face.shape_id() {
                            result.insert_face(face.clone());
                        }
                    }
                }
            }
        }
        result
    }
}

/// BSP Tree builder
pub struct BspTreeBuilder {
    tolerance: f64,
}

impl BspTreeBuilder {
    /// Create a new BSP tree builder with the given tolerance
    pub fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
        }
    }

    /// Build a BSP tree from a shape
    pub fn build_from_shape(&self, shape: &crate::topology::TopoDsShape) -> BspTree {
        let mut tree = BspTree::new(self.tolerance);
        
        // Extract faces from the shape
        let faces = self.extract_faces(shape);
        tree.build(&faces);
        
        tree
    }

    /// Extract faces from a shape
    fn extract_faces(&self, shape: &crate::topology::TopoDsShape) -> Vec<TopoDsFace> {
        // Recursively extract faces from shape
        let mut faces = Vec::new();
        match shape.shape_type() {
            crate::topology::shape_enum::ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    faces.push(face.clone());
                }
            }
            crate::topology::shape_enum::ShapeType::Shell => {
                // Downcast to shell and collect faces
                if let Some(shell) = shape.as_shell() {
                    for f in shell.faces() {
                        faces.push(f.as_ref().clone());
                    }
                }
            }
            crate::topology::shape_enum::ShapeType::Solid => {
                // Downcast to solid and collect faces from shells
                if let Some(solid) = shape.as_solid() {
                    for shell in solid.shells() {
                        for f in shell.faces() {
                            faces.push(f.as_ref().clone());
                        }
                    }
                }
            }
            crate::topology::shape_enum::ShapeType::Compound => {
                // Downcast to compound and collect faces from components
                if let Some(compound) = shape.as_compound() {
                    for component in compound.components() {
                        faces.extend(self.extract_faces(component.as_ref()));
                    }
                }
            }
            _ => {}
        }
        faces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Direction, Point};
    use crate::topology::TopoDsFace;

    #[test]
    fn test_bsp_tree_creation() {
        let tree = BspTree::new(0.001);
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_bsp_node_creation() {
        let plane = Plane::new(Point::origin(), Direction::z_axis(), Direction::x_axis());
        let node = BspNode::new(plane);
        assert!(node.front.is_none());
        assert!(node.back.is_none());
        assert!(node.faces.is_empty());
    }
}
