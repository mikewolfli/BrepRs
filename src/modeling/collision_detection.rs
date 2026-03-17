//! Collision detection module
//! 
//! This module provides collision detection functionality for solids and other geometric objects.

use crate::foundation::handle::Handle;
use crate::geometry::{Point, Vector};
use crate::topology::{TopoDsShape, topods_face::TopoDsFace, topods_solid::TopoDsSolid};
use std::collections::VecDeque;

/// Collision detection result
#[derive(Debug, Clone, PartialEq)]
pub struct CollisionResult {
    /// Whether a collision was detected
    pub has_collision: bool,
    /// Collision points (if any)
    pub collision_points: Vec<Point>,
    /// Minimum distance between objects (if no collision)
    pub minimum_distance: f64,
}

/// Bounding volume hierarchy node
#[derive(Debug, Clone)]
pub struct BvhNode {
    /// Bounding box min point
    pub min: Point,
    /// Bounding box max point
    pub max: Point,
    /// Left child node
    pub left: Option<Box<BvhNode>>,
    /// Right child node
    pub right: Option<Box<BvhNode>>,
    /// Faces contained in this node (only for leaf nodes)
    pub faces: Vec<Handle<TopoDsFace>>,
}

impl BvhNode {
    /// Create a new leaf node
    pub fn new_leaf(faces: Vec<Handle<TopoDsFace>>, min: Point, max: Point) -> Self {
        Self {
            min,
            max,
            left: None,
            right: None,
            faces,
        }
    }

    /// Create a new internal node
    pub fn new_internal(left: Box<BvhNode>, right: Box<BvhNode>) -> Self {
        let min = Point::new(
            left.min.x.min(right.min.x),
            left.min.y.min(right.min.y),
            left.min.z.min(right.min.z),
        );
        let max = Point::new(
            left.max.x.max(right.max.x),
            left.max.y.max(right.max.y),
            left.max.z.max(right.max.z),
        );
        
        Self {
            min,
            max,
            left: Some(left),
            right: Some(right),
            faces: Vec::new(),
        }
    }

    /// Check if this node intersects with another node
    pub fn intersects(&self, other: &BvhNode) -> bool {
        !(self.max.x < other.min.x || self.min.x > other.max.x ||
          self.max.y < other.min.y || self.min.y > other.max.y ||
          self.max.z < other.min.z || self.min.z > other.max.z)
    }

    /// Check if this node contains a point
    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y &&
        point.z >= self.min.z && point.z <= self.max.z
    }
}

/// Bounding volume hierarchy
#[derive(Debug, Clone)]
pub struct BvhTree {
    /// Root node of the tree
    pub root: Option<Box<BvhNode>>,
}

impl BvhTree {
    /// Create a new BVH tree from a solid
    pub fn from_solid(solid: &TopoDsSolid) -> Self {
        let faces = solid.faces();
        let mut bvh = Self {
            root: None,
        };
        
        if !faces.is_empty() {
            bvh.root = Some(bvh.build_tree(faces));
        }
        
        bvh
    }

    /// Build the BVH tree recursively
    fn build_tree(&self, faces: Vec<Handle<TopoDsFace>>) -> Box<BvhNode> {
        if faces.len() == 1 {
            // Create leaf node
            let face = faces[0].as_ref().unwrap();
            let (min, max) = face.bounding_box().unwrap_or((Point::origin(), Point::origin()));
            return Box::new(BvhNode::new_leaf(faces, min, max));
        }
        
        // Find the axis with the largest extent
        let (min, max) = self.calculate_bbox(&faces);
        let extents = Vector::new(
            max.x - min.x,
            max.y - min.y,
            max.z - min.z,
        );
        
        let split_axis = if extents.x >= extents.y && extents.x >= extents.z {
            0 // x-axis
        } else if extents.y >= extents.x && extents.y >= extents.z {
            1 // y-axis
        } else {
            2 // z-axis
        };
        
        // Sort faces along the split axis
        let mut sorted_faces = faces.clone();
        sorted_faces.sort_by(|a, b| {
            let a_face = a.as_ref().unwrap();
            let b_face = b.as_ref().unwrap();
            let a_bbox = a_face.bounding_box().unwrap_or((Point::origin(), Point::origin()));
            let b_bbox = b_face.bounding_box().unwrap_or((Point::origin(), Point::origin()));
            
            let a_center = Point::new(
                (a_bbox.0.x + a_bbox.1.x) / 2.0,
                (a_bbox.0.y + a_bbox.1.y) / 2.0,
                (a_bbox.0.z + a_bbox.1.z) / 2.0,
            );
            let b_center = Point::new(
                (b_bbox.0.x + b_bbox.1.x) / 2.0,
                (b_bbox.0.y + b_bbox.1.y) / 2.0,
                (b_bbox.0.z + b_bbox.1.z) / 2.0,
            );
            
            match split_axis {
                0 => a_center.x.partial_cmp(&b_center.x).unwrap_or(std::cmp::Ordering::Equal),
                1 => a_center.y.partial_cmp(&b_center.y).unwrap_or(std::cmp::Ordering::Equal),
                _ => a_center.z.partial_cmp(&b_center.z).unwrap_or(std::cmp::Ordering::Equal),
            }
        });
        
        // Split faces into two groups
        let mid = sorted_faces.len() / 2;
        let left_faces = sorted_faces[..mid].to_vec();
        let right_faces = sorted_faces[mid..].to_vec();
        
        // Recursively build left and right subtrees
        let left_node = self.build_tree(left_faces);
        let right_node = self.build_tree(right_faces);
        
        // Create internal node
        Box::new(BvhNode::new_internal(left_node, right_node))
    }

    /// Calculate bounding box for a set of faces
    fn calculate_bbox(&self, faces: &[Handle<TopoDsFace>]) -> (Point, Point) {
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut min_z = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        let mut max_z = f64::MIN;
        
        for face in faces {
            if let Some(face_ref) = face.as_ref() {
                if let Some((face_min, face_max)) = face_ref.bounding_box() {
                    min_x = min_x.min(face_min.x);
                    min_y = min_y.min(face_min.y);
                    min_z = min_z.min(face_min.z);
                    max_x = max_x.max(face_max.x);
                    max_y = max_y.max(face_max.y);
                    max_z = max_z.max(face_max.z);
                }
            }
        }
        
        (Point::new(min_x, min_y, min_z), Point::new(max_x, max_y, max_z))
    }
}

/// Collision detector
pub struct CollisionDetector {
    /// Tolerance for collision detection
    tolerance: f64,
}

impl CollisionDetector {
    /// Create a new collision detector with default tolerance
    pub fn new() -> Self {
        Self {
            tolerance: 1e-6,
        }
    }

    /// Create a new collision detector with custom tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            tolerance,
        }
    }

    /// Detect collision between two solids
    pub fn detect_collision(&self, solid1: &TopoDsSolid, solid2: &TopoDsSolid) -> CollisionResult {
        // Build BVH trees for both solids
        let bvh1 = BvhTree::from_solid(solid1);
        let bvh2 = BvhTree::from_solid(solid2);
        
        // If either tree is empty, no collision
        if bvh1.root.is_none() || bvh2.root.is_none() {
            return CollisionResult {
                has_collision: false,
                collision_points: Vec::new(),
                minimum_distance: f64::MAX,
            };
        }
        
        // Perform BVH traversal to find collisions
        let mut collision_points = Vec::new();
        let mut minimum_distance = f64::MAX;
        
        self.traverse_bvh(
            bvh1.root.as_ref().unwrap(),
            bvh2.root.as_ref().unwrap(),
            &mut collision_points,
            &mut minimum_distance,
        );
        
        CollisionResult {
            has_collision: !collision_points.is_empty(),
            collision_points,
            minimum_distance,
        }
    }

    /// Traverse BVH trees to find collisions
    fn traverse_bvh(
        &self,
        node1: &BvhNode,
        node2: &BvhNode,
        collision_points: &mut Vec<Point>,
        minimum_distance: &mut f64,
    ) {
        // Check if nodes intersect
        if !node1.intersects(node2) {
            // Calculate distance between bounding boxes
            let distance = self.calculate_bbox_distance(node1, node2);
            if distance < *minimum_distance {
                *minimum_distance = distance;
            }
            return;
        }
        
        // If both nodes are leaves, check face-face collisions
        if node1.left.is_none() && node2.left.is_none() {
            for face1 in &node1.faces {
                for face2 in &node2.faces {
                    if let (Some(face1_ref), Some(face2_ref)) = (face1.as_ref(), face2.as_ref()) {
                        if let Some(points) = self.check_face_face_collision(face1_ref, face2_ref) {
                            collision_points.extend(points);
                        } else {
                            // Calculate distance between faces
                            let distance = self.calculate_face_face_distance(face1_ref, face2_ref);
                            if distance < *minimum_distance {
                                *minimum_distance = distance;
                            }
                        }
                    }
                }
            }
            return;
        }
        
        // If node1 is a leaf, traverse node2's children
        if node1.left.is_none() {
            if let Some(left) = &node2.left {
                self.traverse_bvh(node1, left, collision_points, minimum_distance);
            }
            if let Some(right) = &node2.right {
                self.traverse_bvh(node1, right, collision_points, minimum_distance);
            }
            return;
        }
        
        // If node2 is a leaf, traverse node1's children
        if node2.left.is_none() {
            if let Some(left) = &node1.left {
                self.traverse_bvh(left, node2, collision_points, minimum_distance);
            }
            if let Some(right) = &node1.right {
                self.traverse_bvh(right, node2, collision_points, minimum_distance);
            }
            return;
        }
        
        // Both nodes are internal, traverse all combinations
        if let (Some(left1), Some(right1), Some(left2), Some(right2)) = 
            (&node1.left, &node1.right, &node2.left, &node2.right) {
            self.traverse_bvh(left1, left2, collision_points, minimum_distance);
            self.traverse_bvh(left1, right2, collision_points, minimum_distance);
            self.traverse_bvh(right1, left2, collision_points, minimum_distance);
            self.traverse_bvh(right1, right2, collision_points, minimum_distance);
        }
    }

    /// Check collision between two faces
    fn check_face_face_collision(&self, face1: &TopoDsFace, face2: &TopoDsFace) -> Option<Vec<Point>> {
        // Get the surfaces of the faces
        let surface1 = face1.surface();
        let surface2 = face2.surface();
        
        if surface1.is_none() || surface2.is_none() {
            return None;
        }
        
        // Get the wires of both faces
        let wires1 = face1.wires();
        let wires2 = face2.wires();
        
        let mut collision_points = Vec::new();
        
        // Check edge-edge intersections
        for wire1 in wires1 {
            if let Some(wire1_ref) = wire1.as_ref() {
                let edges1 = wire1_ref.edges();
                
                for wire2 in wires2 {
                    if let Some(wire2_ref) = wire2.as_ref() {
                        let edges2 = wire2_ref.edges();
                        
                        for edge1 in &edges1 {
                            if let Some(edge1_ref) = edge1.as_ref() {
                                for edge2 in &edges2 {
                                    if let Some(edge2_ref) = edge2.as_ref() {
                                        if let Some(point) = self.check_edge_edge_intersection(edge1_ref, edge2_ref) {
                                            collision_points.push(point);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if !collision_points.is_empty() {
            Some(collision_points)
        } else {
            None
        }
    }

    /// Check intersection between two edges
    fn check_edge_edge_intersection(
        &self,
        edge1: &crate::topology::topods_edge::TopoDsEdge,
        edge2: &crate::topology::topods_edge::TopoDsEdge,
    ) -> Option<Point> {
        // Get the curves of the edges
        let curve1 = edge1.curve();
        let curve2 = edge2.curve();
        
        if curve1.is_none() || curve2.is_none() {
            return None;
        }
        
        // For simplicity, check if the edges share any vertices
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
            
            // Check if any vertices are the same
            if p1s.distance(&p2s) < self.tolerance {
                return Some(p1s.clone());
            }
            if p1s.distance(&p2e) < self.tolerance {
                return Some(p1s.clone());
            }
            if p1e.distance(&p2s) < self.tolerance {
                return Some(p1e.clone());
            }
            if p1e.distance(&p2e) < self.tolerance {
                return Some(p1e.clone());
            }
            
            // Check for actual edge-edge intersection
            if let Some(intersection) = self.calculate_line_segment_intersection(&p1s, &p1e, &p2s, &p2e) {
                return Some(intersection);
            }
        }
        
        None
    }

    /// Calculate intersection between two line segments
    fn calculate_line_segment_intersection(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> Option<Point> {
        let d1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let d2 = Vector::new(p4.x - p3.x, p4.y - p3.y, p4.z - p3.z);
        
        let cross = d1.cross(&d2);
        let cross_magnitude = cross.magnitude();
        
        if cross_magnitude < self.tolerance {
            return None; // Lines are parallel
        }
        
        let diff = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);
        
        let t = (diff.x * cross.y * d2.z - diff.x * cross.z * d2.y + 
                 diff.y * cross.z * d2.x - diff.y * cross.x * d2.z + 
                 diff.z * cross.x * d2.y - diff.z * cross.y * d2.x) / 
                (cross_magnitude * cross_magnitude);
        
        let s = (diff.x * cross.y * d1.z - diff.x * cross.z * d1.y + 
                 diff.y * cross.z * d1.x - diff.y * cross.x * d1.z + 
                 diff.z * cross.x * d1.y - diff.z * cross.y * d1.x) / 
                (cross_magnitude * cross_magnitude);
        
        if t >= -self.tolerance && t <= 1.0 + self.tolerance && 
           s >= -self.tolerance && s <= 1.0 + self.tolerance {
            let intersection = Point::new(
                p1.x + t * d1.x,
                p1.y + t * d1.y,
                p1.z + t * d1.z
            );
            Some(intersection)
        } else {
            None
        }
    }

    /// Calculate distance between two bounding boxes
    fn calculate_bbox_distance(&self, node1: &BvhNode, node2: &BvhNode) -> f64 {
        let mut distance = 0.0;
        
        // Calculate distance in x-direction
        if node1.max.x < node2.min.x {
            distance += (node2.min.x - node1.max.x) * (node2.min.x - node1.max.x);
        } else if node1.min.x > node2.max.x {
            distance += (node1.min.x - node2.max.x) * (node1.min.x - node2.max.x);
        }
        
        // Calculate distance in y-direction
        if node1.max.y < node2.min.y {
            distance += (node2.min.y - node1.max.y) * (node2.min.y - node1.max.y);
        } else if node1.min.y > node2.max.y {
            distance += (node1.min.y - node2.max.y) * (node1.min.y - node2.max.y);
        }
        
        // Calculate distance in z-direction
        if node1.max.z < node2.min.z {
            distance += (node2.min.z - node1.max.z) * (node2.min.z - node1.max.z);
        } else if node1.min.z > node2.max.z {
            distance += (node1.min.z - node2.max.z) * (node1.min.z - node2.max.z);
        }
        
        distance.sqrt()
    }

    /// Calculate distance between two faces
    fn calculate_face_face_distance(&self, face1: &TopoDsFace, face2: &TopoDsFace) -> f64 {
        // Get the wires of both faces
        let wires1 = face1.wires();
        let wires2 = face2.wires();
        
        let mut min_distance = f64::MAX;
        
        // Calculate edge-edge distances
        for wire1 in wires1 {
            if let Some(wire1_ref) = wire1.as_ref() {
                let edges1 = wire1_ref.edges();
                
                for wire2 in wires2 {
                    if let Some(wire2_ref) = wire2.as_ref() {
                        let edges2 = wire2_ref.edges();
                        
                        for edge1 in &edges1 {
                            if let Some(edge1_ref) = edge1.as_ref() {
                                for edge2 in &edges2 {
                                    if let Some(edge2_ref) = edge2.as_ref() {
                                        let distance = self.calculate_edge_edge_distance(edge1_ref, edge2_ref);
                                        if distance < min_distance {
                                            min_distance = distance;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        min_distance
    }

    /// Calculate distance between two edges
    fn calculate_edge_edge_distance(
        &self,
        edge1: &crate::topology::topods_edge::TopoDsEdge,
        edge2: &crate::topology::topods_edge::TopoDsEdge,
    ) -> f64 {
        // Get the vertices of the edges
        let v1_start = edge1.start_vertex();
        let v1_end = edge1.end_vertex();
        let v2_start = edge2.start_vertex();
        let v2_end = edge2.end_vertex();
        
        if let (Some(v1s), Some(v1e), Some(v2s), Some(v2e)) = 
            (v1_start.get(), v1_end.get(), v2_start.get(), v2_end.get()) {
            let p1 = v1s.point();
            let p2 = v1e.point();
            let p3 = v2s.point();
            let p4 = v2e.point();
            
            // Calculate line segment distance
            self.line_segment_distance(&p1, &p2, &p3, &p4)
        } else {
            f64::MAX
        }
    }

    /// Calculate distance between two line segments
    fn line_segment_distance(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
        let d1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let d2 = Vector::new(p4.x - p3.x, p4.y - p3.y, p4.z - p3.z);
        let r = Vector::new(p1.x - p3.x, p1.y - p3.y, p1.z - p3.z);
        
        let a = d1.dot(&d1);
        let e = d2.dot(&d2);
        let f = d2.dot(&r);
        
        let s; let t;
        
        if a <= self.tolerance && e <= self.tolerance {
            // Both segments are points
            return p1.distance(p3);
        }
        
        if a <= self.tolerance {
            // First segment is a point
            s = 0.0;
            t = f / e;
            t = t.max(0.0).min(1.0);
        } else {
            let c = d1.dot(&r);
            if e <= self.tolerance {
                // Second segment is a point
                t = 0.0;
                s = (-c) / a;
                s = s.max(0.0).min(1.0);
            } else {
                let b = d1.dot(&d2);
                let denom = a * e - b * b;
                
                if denom != 0.0 {
                    s = (b * f - c * e) / denom;
                    s = s.max(0.0).min(1.0);
                } else {
                    s = 0.0;
                }
                
                t = (b * s + f) / e;
                
                if t < 0.0 {
                    t = 0.0;
                    s = (-c) / a;
                    s = s.max(0.0).min(1.0);
                } else if t > 1.0 {
                    t = 1.0;
                    s = (b - c) / a;
                    s = s.max(0.0).min(1.0);
                }
            }
        }
        
        let p = Point::new(
            p1.x + s * d1.x,
            p1.y + s * d1.y,
            p1.z + s * d1.z
        );
        
        let q = Point::new(
            p3.x + t * d2.x,
            p3.y + t * d2.y,
            p3.z + t * d2.z
        );
        
        p.distance(&q)
    }
}

impl Default for CollisionDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;

    #[test]
    fn test_bvh_construction() {
        // Create a simple solid
        let solid = crate::topology::topods_solid::TopoDsSolid::new();
        let bvh = BvhTree::from_solid(&solid);
        
        // The BVH should be empty for an empty solid
        assert!(bvh.root.is_none());
    }

    #[test]
    fn test_collision_detection() {
        // Create two solids
        let solid1 = crate::topology::topods_solid::TopoDsSolid::new();
        let solid2 = crate::topology::topods_solid::TopoDsSolid::new();
        
        let detector = CollisionDetector::new();
        let result = detector.detect_collision(&solid1, &solid2);
        
        // Empty solids should not collide
        assert!(!result.has_collision);
        assert!(result.collision_points.is_empty());
    }
}
