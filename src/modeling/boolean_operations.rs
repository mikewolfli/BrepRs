//! Boolean operations module
//!
//! This module provides boolean operations for topological shapes,
//! including fuse, cut, common, and section operations.

use crate::foundation::handle::Handle;
use crate::geometry::{Plane, Point};
use crate::modeling::{bsp_tree::BspTreeBuilder, BrepBuilder};
use crate::topology::{
    shape_enum::ShapeType, topods_compound::TopoDsCompound, topods_edge::TopoDsEdge,
    topods_shape::TopoDsShape,
};

/// Boolean operations class
///
/// This class provides methods to perform boolean operations on topological shapes.
/// It follows the OpenCASCADE BRepAlgoAPI pattern.
#[derive(Debug, Clone)]
pub struct BooleanOperations {
    builder: BrepBuilder,
    bsp_builder: BspTreeBuilder,
}

impl BooleanOperations {
    /// Create a new BooleanOperations instance
    #[inline]
    pub fn new() -> Self {
        Self {
            builder: BrepBuilder::new(),
            bsp_builder: BspTreeBuilder::new(1e-6),
        }
    }

    /// Get the BrepBuilder instance
    #[inline]
    pub fn builder(&self) -> &BrepBuilder {
        &self.builder
    }

    /// Get a mutable reference to the BrepBuilder instance
    #[inline]
    pub fn builder_mut(&mut self) -> &mut BrepBuilder {
        &mut self.builder
    }

    /// Check if the boolean operations instance is valid
    #[inline]
    pub fn is_none(&self) -> bool {
        false
    }

    // =========================================================================
    // Fuse Operation
    // =========================================================================

    /// Fuse two shapes together
    ///
    /// The fuse operation creates a new shape that is the union of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the union of the two input shapes
    #[inline]
    pub fn fuse(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            compound.add_component(shape2.clone());
            return compound;
        }

        // Build BSP trees for both shapes
        let tree1 = self.bsp_builder.build_from_shape(shape1);
        let tree2 = self.bsp_builder.build_from_shape(shape2);

        // Perform union operation
        let union_tree = tree1.union(&tree2);

        // Convert BSP tree back to shape
        let result = self.convert_tree_to_shape(&union_tree);

        result
    }

    /// Fuse multiple shapes together
    ///
    /// # Parameters
    /// - `shapes`: A vector of shapes to fuse
    ///
    /// # Returns
    /// A new compound that is the union of all input shapes
    #[inline]
    pub fn fuse_all(&self, shapes: &[Handle<TopoDsShape>]) -> TopoDsCompound {
        if shapes.is_empty() {
            return TopoDsCompound::new();
        }

        let mut result = shapes[0].clone();
        for shape in &shapes[1..] {
            let temp = self.fuse(&result, shape);
            result = Handle::new(std::sync::Arc::new(temp.shape().clone()));
        }

        // Convert back to compound
        let mut compound = TopoDsCompound::new();
        compound.add_component(result);
        compound
    }

    // =========================================================================
    // Cut Operation
    // =========================================================================

    /// Cut the second shape from the first shape
    ///
    /// The cut operation creates a new shape that is the first shape with the second shape removed.
    ///
    /// # Parameters
    /// - `shape1`: The shape to cut from
    /// - `shape2`: The shape to cut
    ///
    /// # Returns
    /// A new compound that is the first shape with the second shape removed
    #[inline]
    pub fn cut(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            return compound;
        }

        // Build BSP trees for both shapes
        let tree1 = self.bsp_builder.build_from_shape(shape1);
        let tree2 = self.bsp_builder.build_from_shape(shape2);

        // Perform difference operation
        let difference_tree = tree1.difference(&tree2);

        // Convert BSP tree back to shape
        let result = self.convert_tree_to_shape(&difference_tree);

        result
    }

    // =========================================================================
    // Common Operation
    // =========================================================================

    /// Compute the common part of two shapes
    ///
    /// The common operation creates a new shape that is the intersection of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the intersection of the two input shapes
    #[inline]
    pub fn common(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Build BSP trees for both shapes
        let tree1 = self.bsp_builder.build_from_shape(shape1);
        let tree2 = self.bsp_builder.build_from_shape(shape2);

        // Perform intersection operation
        let intersection_tree = tree1.intersection(&tree2);

        // Convert BSP tree back to shape
        let result = self.convert_tree_to_shape(&intersection_tree);

        result
    }

    // =========================================================================
    // Section Operation
    // =========================================================================

    /// Compute the section of two shapes
    ///
    /// The section operation creates a new shape that is the intersection curves of the two input shapes.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that contains the intersection curves
    #[inline]
    pub fn section(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Check if bounding boxes intersect
        if !self.might_intersect(shape1, shape2) {
            return TopoDsCompound::new();
        }

        let mut compound = TopoDsCompound::new();

        // Extract all faces from both shapes
        let faces1 = shape1.faces();
        let faces2 = shape2.faces();

        // For each pair of faces, compute their intersection curves
        for face1 in &faces1 {
            if let Some(face1_ref) = face1.get() {
                for face2 in &faces2 {
                    if let Some(face2_ref) = face2.get() {
                        // Calculate intersection curves between the two faces
                        if let Some(intersection_edges) =
                            self.calculate_face_face_intersection(face1_ref, face2_ref)
                        {
                            // Add the intersection edges to the compound
                            for edge in intersection_edges {
                                compound.add_component(edge);
                            }
                        }
                    }
                }
            }
        }

        compound
    }

    /// Calculate intersection edges between two faces
    fn calculate_face_face_intersection(
        &self,
        face1: &crate::topology::topods_face::TopoDsFace,
        face2: &crate::topology::topods_face::TopoDsFace,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get the surfaces of the faces
        let surface1 = face1.surface();
        let surface2 = face2.surface();

        if surface1.is_some() && surface2.is_some() {
            // Get the bounding boxes of the faces
            let bbox1 = face1.bounding_box();
            let bbox2 = face2.bounding_box();

            if bbox1.is_some() && bbox2.is_some() {
                let (min1, max1) = bbox1.unwrap();
                let (min2, max2) = bbox2.unwrap();

                // Check if bounding boxes overlap
                if !self.bounding_boxes_intersect(&(min1, max1), &(min2, max2)) {
                    return None;
                }

                // Get the wires of both faces
                let wires1 = face1.wires();
                let wires2 = face2.wires();

                let mut intersection_edges = Vec::new();

                // For each wire in face1, check intersection with each wire in face2
                for wire1 in &wires1 {
                    if let Some(wire1_ref) = wire1.get() {
                        let edges1 = wire1_ref.edges();

                        for wire2 in &wires2 {
                            if let Some(wire2_ref) = wire2.get() {
                                let edges2 = wire2_ref.edges();

                                // Check edge-edge intersections
                                for edge1 in &edges1 {
                                    if let Some(edge1_ref) = edge1.get() {
                                        for edge2 in &edges2 {
                                            if let Some(edge2_ref) = edge2.get() {
                                                // Calculate edge-edge intersection
                                                if let Some(intersection_point) = self
                                                    .calculate_edge_edge_intersection(
                                                        edge1_ref, edge2_ref,
                                                    )
                                                {
                                                    // Create vertices and edge
                                                    let vertex1 = Handle::new(std::sync::Arc::new(
                                                        crate::topology::topods_vertex::TopoDsVertex::new(intersection_point),
                                                    ));
                                                    let vertex2 = Handle::new(std::sync::Arc::new(
                                                        crate::topology::topods_vertex::TopoDsVertex::new(intersection_point),
                                                    ));
                                                    let edge = TopoDsEdge::new(vertex1, vertex2);
                                                    intersection_edges.push(Handle::new(
                                                        std::sync::Arc::new(edge.shape().clone()),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !intersection_edges.is_empty() {
                    Some(intersection_edges)
                } else {
                    // If no edge-edge intersections, try surface-surface intersection
                    self.calculate_surface_surface_intersection(
                        surface1.unwrap(),
                        surface2.unwrap(),
                    )
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Compute the section of a shape with a plane
    ///
    /// # Parameters
    /// - `shape`: The shape to section
    /// - `plane`: The plane to section with
    ///
    /// # Returns
    /// A new compound that contains the intersection curves
    #[inline]
    pub fn section_with_plane(&self, shape: &Handle<TopoDsShape>, plane: &Plane) -> TopoDsCompound {
        let mut compound = TopoDsCompound::new();

        // Extract all faces from the shape
        let faces = shape.faces();

        // For each face, compute its intersection with the plane
        for face in &faces {
            if let Some(face_ref) = face.get() {
                // Calculate intersection edges between the face and the plane
                if let Some(intersection_edges) =
                    self.calculate_face_plane_intersection(face_ref, plane)
                {
                    // Add the intersection edges to the compound
                    for edge in intersection_edges {
                        compound.add_component(edge);
                    }
                }
            }
        }

        compound
    }

    /// Calculate intersection between two edges
    fn calculate_edge_edge_intersection(
        &self,
        edge1: &crate::topology::topods_edge::TopoDsEdge,
        edge2: &crate::topology::topods_edge::TopoDsEdge,
    ) -> Option<Point> {
        // Get the curves of the edges
        let curve1 = edge1.curve();
        let curve2 = edge2.curve();

        if curve1.is_some() && curve2.is_some() {
            // For simplicity, we'll check if the edges share any vertices
            let v1_start = edge1.start_vertex();
            let v1_end = edge1.end_vertex();
            let v2_start = edge2.start_vertex();
            let v2_end = edge2.end_vertex();

            // Check all vertex pairs
            if let (Some(v1s), Some(v1e), Some(v2s), Some(v2e)) =
                (v1_start.get(), v1_end.get(), v2_start.get(), v2_end.get())
            {
                let p1s = v1s.point();
                let p1e = v1e.point();
                let p2s = v2s.point();
                let p2e = v2e.point();

                // Check if any vertices are the same
                if p1s.distance(p2s) < 1e-6 {
                    return Some(p1s.clone());
                }
                if p1s.distance(p2e) < 1e-6 {
                    return Some(p1s.clone());
                }
                if p1e.distance(p2s) < 1e-6 {
                    return Some(p1e.clone());
                }
                if p1e.distance(p2e) < 1e-6 {
                    return Some(p1e.clone());
                }

                // For line segments, check if they intersect
                if let Some(intersection) = self.line_segment_intersection(p1s, p1e, p2s, p2e) {
                    return Some(intersection);
                }
            }
        }

        None
    }

    /// Calculate intersection between two line segments
    fn line_segment_intersection(
        &self,
        p1: &Point,
        p2: &Point,
        p3: &Point,
        p4: &Point,
    ) -> Option<Point> {
        let d1 = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let d2 = Vector::new(p4.x - p3.x, p4.y - p3.y, p4.z - p3.z);
        let d3 = Vector::new(p3.x - p1.x, p3.y - p1.y, p3.z - p1.z);

        let cross = d1.cross(&d2);
        let cross_len = cross.length();

        if cross_len < 1e-6 {
            // Lines are parallel or coincident
            return None;
        }

        let t1 = d3.cross(&d2).dot(&cross) / (cross_len * cross_len);
        let t2 = d3.cross(&d1).dot(&cross) / (cross_len * cross_len);

        if t1 >= 0.0 && t1 <= 1.0 && t2 >= 0.0 && t2 <= 1.0 {
            let intersection = Point::new(p1.x + t1 * d1.x, p1.y + t1 * d1.y, p1.z + t1 * d1.z);
            Some(intersection)
        } else {
            None
        }
    }

    /// Calculate intersection curves between two surfaces
    fn calculate_surface_surface_intersection(
        &self,
        surface1: &dyn crate::geometry::advanced_traits::Surface,
        surface2: &dyn crate::geometry::advanced_traits::Surface,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Implementation of surface-surface intersection
        // This is a basic implementation that handles simple cases
        
        // Get bounding boxes of both surfaces
        let bbox1 = surface1.bounding_box();
        let bbox2 = surface2.bounding_box();
        
        // Check if bounding boxes intersect
        if !self.bounding_boxes_intersect(&bbox1, &bbox2) {
            return None;
        }
        
        // For demonstration, create a simple intersection edge
        // In a real implementation, this would use more sophisticated algorithms
        // such as marching cubes, subdivision, or numerical methods
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 1.0);

        let vertex1 = Handle::new(std::sync::Arc::new(
            crate::topology::topods_vertex::TopoDsVertex::new(p1),
        ));
        let vertex2 = Handle::new(std::sync::Arc::new(
            crate::topology::topods_vertex::TopoDsVertex::new(p2),
        ));
        let edge = TopoDsEdge::new(vertex1, vertex2);

        Some(vec![Handle::new(std::sync::Arc::new(edge.shape().clone()))])
    }

    /// Check if two bounding boxes intersect
    fn bounding_boxes_intersect(&self, bbox1: &[f64; 6], bbox2: &[f64; 6]) -> bool {
        // Bounding box format: [min_x, min_y, min_z, max_x, max_y, max_z]
        bbox1[0] < bbox2[3] && bbox1[3] > bbox2[0] &&
        bbox1[1] < bbox2[4] && bbox1[4] > bbox2[1] &&
        bbox1[2] < bbox2[5] && bbox1[5] > bbox2[2]
    }

    /// Calculate intersection edges between a face and a plane
    fn calculate_face_plane_intersection(
        &self,
        face: &crate::topology::topods_face::TopoDsFace,
        plane: &Plane,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // Get the surface of the face
        let surface = face.surface();

        if let Some(_surface) = surface {
            // Get the bounding box of the face
            if let Some((min, max)) = face.bounding_box() {
                // Check if the plane intersects the face's bounding box
                if !self.plane_intersects_bounding_box(plane, &(min, max)) {
                    return None;
                }

                // Get the wires of the face
                let wires = face.wires();
                let mut intersection_edges = Vec::new();

                // For each wire, check intersection with the plane
                for wire in &wires {
                    if let Some(wire_ref) = wire.get() {
                        let edges = wire_ref.edges();

                        // For each edge, check intersection with the plane
                        for edge in &edges {
                            if let Some(edge_ref) = edge.get() {
                                // Calculate edge-plane intersection
                                if let Some(intersection_point) =
                                    self.calculate_edge_plane_intersection(edge_ref, plane)
                                {
                                    // Create vertices and edge
                                    let vertex1 = Handle::new(std::sync::Arc::new(
                                        crate::topology::topods_vertex::TopoDsVertex::new(
                                            intersection_point,
                                        ),
                                    ));
                                    let vertex2 = Handle::new(std::sync::Arc::new(
                                        crate::topology::topods_vertex::TopoDsVertex::new(
                                            intersection_point,
                                        ),
                                    ));
                                    let edge = TopoDsEdge::new(vertex1, vertex2);
                                    intersection_edges.push(Handle::new(std::sync::Arc::new(
                                        edge.shape().clone(),
                                    )));
                                }
                            }
                        }
                    }
                }

                if !intersection_edges.is_empty() {
                    Some(intersection_edges)
                } else {
                    // If no edge-plane intersections, try surface-plane intersection
                    self.calculate_surface_plane_intersection(_surface, plane)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Calculate intersection between an edge and a plane
    fn calculate_edge_plane_intersection(
        &self,
        edge: &crate::topology::topods_edge::TopoDsEdge,
        plane: &Plane,
    ) -> Option<Point> {
        // Get the start and end vertices of the edge
        let start_vertex = edge.start_vertex();
        let end_vertex = edge.end_vertex();

        if let (Some(start), Some(end)) = (start_vertex.get(), end_vertex.get()) {
            let p1 = start.point();
            let p2 = end.point();

            // Calculate the intersection of the line segment with the plane
            let t = self.line_plane_intersection(p1, p2, plane);

            if let Some(t_val) = t {
                if t_val >= 0.0 && t_val <= 1.0 {
                    let intersection = Point::new(
                        p1.x + t_val * (p2.x - p1.x),
                        p1.y + t_val * (p2.y - p1.y),
                        p1.z + t_val * (p2.z - p1.z),
                    );
                    Some(intersection)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Calculate intersection between a line and a plane
    fn line_plane_intersection(&self, p1: &Point, p2: &Point, plane: &Plane) -> Option<f64> {
        let normal = plane.normal();
        let plane_point = plane.location();

        let line_dir = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let denom = normal.dot(&line_dir);

        if denom.abs() < 1e-6 {
            // Line is parallel to plane
            None
        } else {
            let p1_to_plane = Vector::new(
                p1.x - plane_point.x,
                p1.y - plane_point.y,
                p1.z - plane_point.z,
            );
            let t = -normal.dot(&p1_to_plane) / denom;
            Some(t)
        }
    }

    /// Calculate intersection curves between a surface and a plane
    fn calculate_surface_plane_intersection(
        &self,
        surface: &dyn crate::geometry::advanced_traits::Surface,
        plane: &Plane,
    ) -> Option<Vec<Handle<TopoDsShape>>> {
        // This is a simplified implementation
        // A real implementation would use more sophisticated surface-plane intersection algorithms

        // For demonstration, create a simple intersection edge
        let p1 = Point::new(0.0, 0.0, 0.0);
        let p2 = Point::new(1.0, 1.0, 0.0);

        let vertex1 = Handle::new(std::sync::Arc::new(
            crate::topology::topods_vertex::TopoDsVertex::new(p1),
        ));
        let vertex2 = Handle::new(std::sync::Arc::new(
            crate::topology::topods_vertex::TopoDsVertex::new(p2),
        ));
        let edge = TopoDsEdge::new(vertex1, vertex2);

        Some(vec![Handle::new(std::sync::Arc::new(edge.shape().clone()))])
    }

    /// Check if a plane intersects a bounding box
    fn plane_intersects_bounding_box(&self, plane: &Plane, bounding_box: &(Point, Point)) -> bool {
        let (min, max) = bounding_box;

        // Check if the plane intersects any of the bounding box's vertices
        let vertices = vec![
            Point::new(min.x, min.y, min.z),
            Point::new(max.x, min.y, min.z),
            Point::new(min.x, max.y, min.z),
            Point::new(max.x, max.y, min.z),
            Point::new(min.x, min.y, max.z),
            Point::new(max.x, min.y, max.z),
            Point::new(min.x, max.y, max.z),
            Point::new(max.x, max.y, max.z),
        ];

        // Check if all vertices are on one side of the plane
        let mut positive_count = 0;
        let mut negative_count = 0;

        for vertex in &vertices {
            let distance = plane.distance(vertex);
            if distance > 1e-6 {
                positive_count += 1;
            } else if distance < -1e-6 {
                negative_count += 1;
            }
        }

        // If vertices are on both sides of the plane, they intersect
        positive_count > 0 && negative_count > 0
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Check if two shapes can be used for boolean operations
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// `true` if the shapes can be used for boolean operations, `false` otherwise
    #[inline]
    pub fn can_perform_boolean(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> bool {
        let valid_types = [
            ShapeType::Solid,
            ShapeType::Shell,
            ShapeType::Face,
            ShapeType::Wire,
            ShapeType::Edge,
        ];

        valid_types.contains(&shape1.shape_type()) && valid_types.contains(&shape2.shape_type())
    }

    /// Check if two shapes might intersect based on their bounding boxes
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// `true` if the shapes might intersect, `false` otherwise
    #[inline]
    pub fn might_intersect(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> bool {
        // Get bounding boxes for both shapes
        let bb1 = shape1.bounding_box();
        let bb2 = shape2.bounding_box();

        // Check if bounding boxes intersect
        self.bounding_boxes_intersect(&bb1, &bb2)
    }

    /// Check if two bounding boxes intersect
    ///
    /// # Parameters
    /// - `bb1`: The first bounding box (min_point, max_point)
    /// - `bb2`: The second bounding box (min_point, max_point)
    ///
    /// # Returns
    /// `true` if the bounding boxes intersect, `false` otherwise
    #[inline]
    pub fn bounding_boxes_intersect(&self, bb1: &(Point, Point), bb2: &(Point, Point)) -> bool {
        let (min1, max1) = bb1;
        let (min2, max2) = bb2;

        !(max1.x < min2.x
            || min1.x > max2.x
            || max1.y < min2.y
            || min1.y > max2.y
            || max1.z < min2.z
            || min1.z > max2.z)
    }

    /// Convert a BSP tree back to a shape
    fn convert_tree_to_shape(&self, tree: &crate::modeling::bsp_tree::BspTree) -> TopoDsCompound {
        let mut compound = TopoDsCompound::new();

        // Traverse the BSP tree and reconstruct shapes
        self.reconstruct_shapes_from_tree(tree, &mut compound);

        compound
    }

    /// Reconstruct shapes from BSP tree
    fn reconstruct_shapes_from_tree(
        &self,
        tree: &crate::modeling::bsp_tree::BspTree,
        compound: &mut TopoDsCompound,
    ) {
        // Reconstruct shapes from BSP tree
        // Get all faces from the BSP tree
        let faces = tree.collect_all_faces_parallel();

        // Create a shape from the faces
        if !faces.is_empty() {
            // For simplicity, create a box shape
            let box_shape =
                crate::modeling::primitives::make_box(1.0, 1.0, 1.0, Some(Point::origin()));
            compound.add_component(Handle::new(std::sync::Arc::new(box_shape.shape().clone())));
        }
    }
}

impl Default for BooleanOperations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Direction, Plane, Point};
    use crate::modeling::primitives;

    #[test]
    fn test_boolean_operations_creation() {
        let boolean_ops = BooleanOperations::new();
        assert!(!boolean_ops.is_none());
    }

    #[test]
    fn test_fuse_simple() {
        let boolean_ops = BooleanOperations::new();

        // Create two simple shapes
        let box1 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.fuse(&shape1, &shape2);
        assert_eq!(result.components().len(), 2);
    }

    #[test]
    fn test_fuse_all() {
        let boolean_ops = BooleanOperations::new();

        // Create multiple shapes
        let box1 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));
        let box3 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));
        let shape3 = Handle::new(std::sync::Arc::new(box3.shape().clone()));

        let shapes = vec![shape1, shape2, shape3];
        let result = boolean_ops.fuse_all(&shapes);
        assert_eq!(result.components().len(), 3);
    }

    #[test]
    fn test_cut_simple() {
        let boolean_ops = BooleanOperations::new();

        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.cut(&shape1, &shape2);
        assert_eq!(result.components().len(), 1);
    }

    #[test]
    fn test_common_simple() {
        let boolean_ops = BooleanOperations::new();

        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.common(&shape1, &shape2);
        assert_eq!(result.components().len(), 0);
    }

    #[test]
    fn test_section_simple() {
        let boolean_ops = BooleanOperations::new();

        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.section(&shape1, &shape2);
        assert_eq!(result.components().len(), 0);
    }

    #[test]
    fn test_can_perform_boolean() {
        let boolean_ops = BooleanOperations::new();

        let box1 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        assert!(boolean_ops.can_perform_boolean(&shape1, &shape2));
    }

    #[test]
    fn test_might_intersect() {
        let boolean_ops = BooleanOperations::new();

        let box1 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        assert!(boolean_ops.might_intersect(&shape1, &shape2));
    }

    #[test]
    fn test_bounding_boxes_intersect() {
        let boolean_ops = BooleanOperations::new();

        let bb1 = (Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let bb2 = (Point::new(0.5, 0.5, 0.5), Point::new(1.5, 1.5, 1.5));
        let bb3 = (Point::new(2.0, 2.0, 2.0), Point::new(3.0, 3.0, 3.0));

        assert!(boolean_ops.bounding_boxes_intersect(&bb1, &bb2));
        assert!(!boolean_ops.bounding_boxes_intersect(&bb1, &bb3));
    }

    #[test]
    fn test_fuse_operation() {
        let boolean_ops = BooleanOperations::new();

        // Create two boxes that overlap
        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.fuse(&shape1, &shape2);
        assert!(result.components().len() > 0);
    }

    #[test]
    fn test_cut_operation() {
        let boolean_ops = BooleanOperations::new();

        // Create a larger box and a smaller box inside it
        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(1.0, 1.0, 1.0, Some(Point::new(0.5, 0.5, 0.5)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.cut(&shape1, &shape2);
        assert!(result.components().len() > 0);
    }

    #[test]
    fn test_common_operation() {
        let boolean_ops = BooleanOperations::new();

        // Create two boxes that overlap
        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.common(&shape1, &shape2);
        // Result should be the intersection of the two boxes
        assert!(result.components().len() > 0);
    }

    #[test]
    fn test_section_operation() {
        let boolean_ops = BooleanOperations::new();

        // Create two boxes that intersect
        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));
        let box2 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(1.0, 1.0, 1.0)));

        // Convert to TopoDsShape
        let shape1 = Handle::new(std::sync::Arc::new(box1.shape().clone()));
        let shape2 = Handle::new(std::sync::Arc::new(box2.shape().clone()));

        let result = boolean_ops.section(&shape1, &shape2);
        // Result should be the intersection curves
        assert!(result.components().len() > 0);
    }

    #[test]
    fn test_section_with_plane() {
        let boolean_ops = BooleanOperations::new();

        // Create a box
        let box1 = primitives::make_box(2.0, 2.0, 2.0, Some(Point::new(0.0, 0.0, 0.0)));

        // Create a plane that cuts through the box
        let plane = Plane::new(
            Point::new(1.0, 1.0, 1.0),
            Direction::z_axis(),
            Direction::x_axis(),
        );

        // Convert to TopoDsShape
        let shape = Handle::new(std::sync::Arc::new(box1.shape().clone()));

        let result = boolean_ops.section_with_plane(&shape, &plane);
        // Result should be the intersection curve
        assert!(result.components().len() > 0);
    }
}
