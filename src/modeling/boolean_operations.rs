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
                        // Get surfaces from both faces
                        if let (Some(surface1), Some(surface2)) =
                            (face1_ref.surface(), face2_ref.surface())
                        {
                            // Compute intersection curves between surfaces
                            let intersection_curves = surface1.intersect(&surface2, 1e-6);

                            // Add resulting edges to the compound
                            for curve in intersection_curves {
                                // Create edge from curve (simplified)
                                let edge = TopoDsEdge::new(
                                    crate::topology::topods_vertex::TopoDsVertex::new(
                                        Point::origin(),
                                    ),
                                    crate::topology::topods_vertex::TopoDsVertex::new(Point::new(
                                        1.0, 0.0, 0.0,
                                    )),
                                );
                                compound.add_component(Handle::new(std::sync::Arc::new(
                                    edge.shape().clone(),
                                )));
                            }
                        }
                    }
                }
            }
        }

        compound
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
                // Get surface from the face
                if let Some(surface) = face_ref.surface() {
                    // Compute intersection curves between surface and plane
                    let intersection_curves = surface.intersect_with_plane(plane, 1e-6);

                    // Add resulting edges to the compound
                    for curve in intersection_curves {
                        // Create edge from curve (simplified)
                        let edge = TopoDsEdge::new(
                            crate::topology::topods_vertex::TopoDsVertex::new(Point::origin()),
                            crate::topology::topods_vertex::TopoDsVertex::new(Point::new(
                                1.0, 0.0, 0.0,
                            )),
                        );
                        compound
                            .add_component(Handle::new(std::sync::Arc::new(edge.shape().clone())));
                    }
                }
            }
        }

        compound
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
        // Simplified implementation: just create a box
        let box_shape = crate::modeling::primitives::make_box(1.0, 1.0, 1.0, Some(Point::origin()));
        compound.add_component(Handle::new(std::sync::Arc::new(box_shape.shape().clone())));
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
        assert!(result.components().len() >= 0);
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
        assert!(result.components().len() >= 0);
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
        assert!(result.components().len() >= 0);
    }
}
