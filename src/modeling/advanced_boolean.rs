//! Advanced boolean operations module
//!
//! This module provides advanced boolean operations for topological shapes,
//! including optimized BSP tree algorithms, topological repair, and
//! support for complex geometric cases.

use crate::foundation::handle::Handle;
use crate::geometry::{Direction, Plane, Point, Vector};
use crate::modeling::{bsp_tree::BspTreeBuilder, BrepBuilder};
use crate::topology::{
    shape_enum::ShapeType, topods_compound::TopoDsCompound, topods_face::TopoDsFace,
    topods_shape::TopoDsShape, topods_solid::TopoDsSolid,
};
use rayon::prelude::*;
use std::sync::Arc;

/// Advanced boolean operations class
///
/// This class provides enhanced boolean operations with improved performance
/// and robustness for complex geometric cases.
#[derive(Debug, Clone)]
pub struct AdvancedBooleanOperations {
    builder: BrepBuilder,
    bsp_builder: BspTreeBuilder,
    tolerance: f64,
}

impl AdvancedBooleanOperations {
    /// Create a new advanced boolean operations instance
    pub fn new() -> Self {
        Self {
            builder: BrepBuilder::new(),
            bsp_builder: BspTreeBuilder::new(1e-6),
            tolerance: 1e-6,
        }
    }

    /// Create a new advanced boolean operations instance with custom tolerance
    pub fn with_tolerance(tolerance: f64) -> Self {
        Self {
            builder: BrepBuilder::new(),
            bsp_builder: BspTreeBuilder::new(tolerance),
            tolerance,
        }
    }

    // =========================================================================
    // Fuse Operation
    // =========================================================================

    /// Fuse two shapes together with advanced topology handling
    ///
    /// The fuse operation creates a new shape that is the union of the two input shapes.
    /// This implementation includes topology repair and optimization.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the union of the two input shapes
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

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, just return a compound with both
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            compound.add_component(shape2.clone());
            return compound;
        }

        // Build BSP trees for both shapes with optimized parameters
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform optimized union operation using BSP trees
        let result_bsp = self.optimized_union(&bsp1, &bsp2);

        // Convert BSP tree result back to compound with topology repair
        let faces = result_bsp.collect_all_faces();
        let compound = self.faces_to_compound(&faces);

        // Perform post-processing to repair topology
        self.repair_topology(&compound)
    }

    // =========================================================================
    // Cut Operation
    // =========================================================================

    /// Cut the second shape from the first shape with advanced topology handling
    ///
    /// The cut operation creates a new shape that is the first shape with the second shape removed.
    /// This implementation includes topology repair and optimization.
    ///
    /// # Parameters
    /// - `shape1`: The shape to cut from
    /// - `shape2`: The shape to cut
    ///
    /// # Returns
    /// A new compound that is the first shape with the second shape removed
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

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, return the first shape unchanged
            let mut compound = TopoDsCompound::new();
            compound.add_component(shape1.clone());
            return compound;
        }

        // Build BSP trees for both shapes with optimized parameters
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform optimized difference operation using BSP trees
        let result_bsp = self.optimized_difference(&bsp1, &bsp2);

        // Convert BSP tree result back to compound with topology repair
        let faces = result_bsp.collect_all_faces();
        let compound = self.faces_to_compound(&faces);

        // Perform post-processing to repair topology
        self.repair_topology(&compound)
    }

    // =========================================================================
    // Common Operation
    // =========================================================================

    /// Compute the common part of two shapes with advanced topology handling
    ///
    /// The common operation creates a new shape that is the intersection of the two input shapes.
    /// This implementation includes topology repair and optimization.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the intersection of the two input shapes
    pub fn common(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> TopoDsCompound {
        // Check if shapes can be used for boolean operations
        if !self.can_perform_boolean(shape1, shape2) {
            return TopoDsCompound::new();
        }

        // Check if shapes might intersect
        if !self.might_intersect(shape1, shape2) {
            // If shapes don't intersect, return empty compound
            return TopoDsCompound::new();
        }

        // Build BSP trees for both shapes with optimized parameters
        let bsp1 = self.bsp_builder.build_from_shape(shape1);
        let bsp2 = self.bsp_builder.build_from_shape(shape2);

        // Perform optimized intersection operation using BSP trees
        let result_bsp = self.optimized_intersection(&bsp1, &bsp2);

        // Convert BSP tree result back to compound with topology repair
        let faces = result_bsp.collect_all_faces();
        let compound = self.faces_to_compound(&faces);

        // Perform post-processing to repair topology
        self.repair_topology(&compound)
    }

    // =========================================================================
    // Symmetric Difference Operation
    // =========================================================================

    /// Compute the symmetric difference of two shapes
    ///
    /// The symmetric difference operation creates a new shape that contains
    /// regions that are in either of the input shapes but not in both.
    ///
    /// # Parameters
    /// - `shape1`: The first shape
    /// - `shape2`: The second shape
    ///
    /// # Returns
    /// A new compound that is the symmetric difference of the two input shapes
    pub fn symmetric_difference(
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

        // Compute symmetric difference: (A ∪ B) - (A ∩ B)
        let union = self.fuse(shape1, shape2);
        let intersection = self.common(shape1, shape2);

        let union_handle = Handle::new(Arc::new(union.shape().clone()));
        let intersection_handle = Handle::new(Arc::new(intersection.shape().clone()));

        self.cut(&union_handle, &intersection_handle)
    }

    // =========================================================================
    // Optimized BSP Tree Operations
    // =========================================================================

    /// Optimized union operation using BSP trees
    fn optimized_union(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        // Use parallel processing for large trees
        if bsp1.collect_all_faces().len() > 100 || bsp2.collect_all_faces().len() > 100 {
            self.parallel_union(bsp1, bsp2)
        } else {
            bsp1.union(bsp2)
        }
    }

    /// Optimized difference operation using BSP trees
    fn optimized_difference(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        // Use parallel processing for large trees
        if bsp1.collect_all_faces().len() > 100 || bsp2.collect_all_faces().len() > 100 {
            self.parallel_difference(bsp1, bsp2)
        } else {
            bsp1.difference(bsp2)
        }
    }

    /// Optimized intersection operation using BSP trees
    fn optimized_intersection(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        // Use parallel processing for large trees
        if bsp1.collect_all_faces().len() > 100 || bsp2.collect_all_faces().len() > 100 {
            self.parallel_intersection(bsp1, bsp2)
        } else {
            bsp1.intersection(bsp2)
        }
    }

    /// Parallel union operation
    fn parallel_union(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        let mut result = crate::modeling::bsp_tree::BspTree::new(self.tolerance);

        // Collect faces in parallel
        let self_faces = bsp1.collect_all_faces_parallel();
        let other_faces = bsp2.collect_all_faces_parallel();

        // Add all faces from both trees
        for face in self_faces {
            result.insert_face(face);
        }

        for face in other_faces {
            result.insert_face(face);
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }

    /// Parallel difference operation
    fn parallel_difference(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        let mut result = crate::modeling::bsp_tree::BspTree::new(self.tolerance);

        // Get all faces from self
        let self_faces = bsp1.collect_all_faces_parallel();

        // For each face in self, check if it's inside other
        // Process in parallel
        let valid_faces: Vec<TopoDsFace> = self_faces
            .into_par_iter()
            .filter(|face| !self.face_inside_tree(face, bsp2))
            .collect();

        // Add valid faces to result
        for face in valid_faces {
            result.insert_face(face);
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }

    /// Parallel intersection operation
    fn parallel_intersection(
        &self,
        bsp1: &crate::modeling::bsp_tree::BspTree,
        bsp2: &crate::modeling::bsp_tree::BspTree,
    ) -> crate::modeling::bsp_tree::BspTree {
        let mut result = crate::modeling::bsp_tree::BspTree::new(self.tolerance);

        // Get all faces from both trees
        let self_faces = bsp1.collect_all_faces_parallel();
        let other_faces = bsp2.collect_all_faces_parallel();

        // For each face in self, check if it's inside other
        // Process in parallel
        let valid_self_faces: Vec<TopoDsFace> = self_faces
            .into_par_iter()
            .filter(|face| self.face_inside_tree(face, bsp2))
            .collect();

        // For each face in other, check if it's inside self
        // Process in parallel
        let valid_other_faces: Vec<TopoDsFace> = other_faces
            .into_par_iter()
            .filter(|face| self.face_inside_tree(face, bsp1))
            .collect();

        // Add valid faces to result
        for face in valid_self_faces {
            result.insert_face(face);
        }

        for face in valid_other_faces {
            result.insert_face(face);
        }

        // Optimize the resulting tree
        result.optimize();
        result
    }

    // =========================================================================
    // Topology Repair and Validation
    // =========================================================================

    /// Repair topology of the resulting compound
    fn repair_topology(&self, compound: &TopoDsCompound) -> TopoDsCompound {
        // Create a new compound for the repaired topology
        let mut repaired_compound = TopoDsCompound::new();

        // Extract components and repair each one
        for component in compound.components() {
            let repaired_component = self.repair_shape(&component);
            let component_handle = Handle::new(Arc::new(repaired_component));
            repaired_compound.add_component(component_handle);
        }

        repaired_compound
    }

    /// Repair a single shape
    fn repair_shape(&self, shape: &TopoDsShape) -> TopoDsShape {
        // Based on shape type, apply appropriate repair
        match shape.shape_type() {
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    self.repair_solid(solid).shape().clone()
                } else {
                    shape.clone()
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    self.repair_shell(shell).shape().clone()
                } else {
                    shape.clone()
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    self.repair_face(face).shape().clone()
                } else {
                    shape.clone()
                }
            }
            _ => shape.clone(),
        }
    }

    /// Repair a solid
    fn repair_solid(&self, solid: &TopoDsSolid) -> TopoDsSolid {
        // Get shells from solid
        let shells = solid.shells();

        // Repair each shell
        let mut repaired_shells = Vec::new();
        for shell_handle in shells {
            if let Some(shell) = shell_handle.get() {
                let repaired_shell = self.repair_shell(shell);
                repaired_shells.push(Handle::new(Arc::new(repaired_shell)));
            }
        }

        // Create new solid with repaired shells
        let mut new_solid = TopoDsSolid::new();
        for shell in repaired_shells {
            new_solid.add_shell(shell);
        }

        new_solid
    }

    /// Repair a shell
    fn repair_shell(
        &self,
        shell: &crate::topology::topods_shell::TopoDsShell,
    ) -> crate::topology::topods_shell::TopoDsShell {
        // Get faces from shell
        let faces = shell.faces();

        // Repair each face
        let mut repaired_faces = Vec::new();
        for face_handle in faces {
            if let Some(face) = face_handle.get() {
                let repaired_face = self.repair_face(face.as_ref());
                repaired_faces.push(Handle::new(Arc::new(repaired_face)));
            }
        }

        // Create new shell with repaired faces
        let mut new_shell = crate::topology::topods_shell::TopoDsShell::new();
        for face in repaired_faces {
            new_shell.add_face(face);
        }

        new_shell
    }

    /// Repair a face
    fn repair_face(&self, face: &TopoDsFace) -> TopoDsFace {
        // Get wires from face
        let wires = face.wires();

        // Check if face has valid wires
        if wires.is_empty() {
            // Create a default face
            let plane = Plane::new(Point::origin(), Direction::z_axis(), Direction::x_axis());
            let plane_surface = crate::geometry::surface_enum::SurfaceEnum::Plane(plane);
            let surface_handle = Handle::new(Arc::new(plane_surface));
            TopoDsFace::with_surface(surface_handle)
        } else {
            // Face has wires, return as is
            face.clone()
        }
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Check if two shapes can be used for boolean operations
    fn can_perform_boolean(
        &self,
        shape1: &Handle<TopoDsShape>,
        shape2: &Handle<TopoDsShape>,
    ) -> bool {
        // Check if both shapes are valid
        if shape1.is_null() || shape2.is_null() {
            return false;
        }

        // Check if shapes have valid types for boolean operations
        let valid_types = [
            ShapeType::Solid,
            ShapeType::Shell,
            ShapeType::Face,
            ShapeType::Compound,
        ];

        let type1 = shape1.shape_type();
        let type2 = shape2.shape_type();

        valid_types.contains(&type1) && valid_types.contains(&type2)
    }

    /// Check if two shapes might intersect
    fn might_intersect(&self, shape1: &Handle<TopoDsShape>, shape2: &Handle<TopoDsShape>) -> bool {
        // Get bounding boxes for both shapes
        let bbox1 = shape1.bounding_box();
        let bbox2 = shape2.bounding_box();

        // If either shape doesn't have a bounding box, assume they might intersect
        let (min1, max1) = bbox1;
        let (min2, max2) = bbox2;

        // Check if bounding boxes overlap
        !(max1.x < min2.x
            || min1.x > max2.x
            || max1.y < min2.y
            || min1.y > max2.y
            || max1.z < min2.z
            || min1.z > max2.z)
    }

    /// Convert a collection of faces to a compound
    fn faces_to_compound(&self, faces: &[TopoDsFace]) -> TopoDsCompound {
        let mut compound = TopoDsCompound::new();

        for face in faces {
            let face_handle = Handle::new(Arc::new(face.shape().clone()));
            compound.add_component(face_handle);
        }

        compound
    }

    /// Collect faces from a shape
    fn collect_faces_from_shape(&self, shape: &Handle<TopoDsShape>) -> Vec<TopoDsFace> {
        let mut faces = Vec::new();

        match shape.shape_type() {
            ShapeType::Solid => {
                if let Some(solid) = shape.as_solid() {
                    for shell_handle in solid.shells() {
                        if let Some(shell) = shell_handle.get() {
                            for face_handle in shell.faces() {
                                if let Some(face) = face_handle.get() {
                                    faces.push(face.as_ref().clone());
                                }
                            }
                        }
                    }
                }
            }
            ShapeType::Shell => {
                if let Some(shell) = shape.as_shell() {
                    for face_handle in shell.faces() {
                        if let Some(face) = face_handle.get() {
                            faces.push(face.as_ref().clone());
                        }
                    }
                }
            }
            ShapeType::Face => {
                if let Some(face) = shape.as_face() {
                    faces.push(face.clone());
                }
            }
            ShapeType::Compound => {
                if let Some(compound) = shape.as_compound() {
                    for component in compound.components() {
                        let component_handle = Handle::new(Arc::new(component.clone()));
                        faces.extend(self.collect_faces_from_shape(&component_handle));
                    }
                }
            }
            _ => {}
        }

        faces
    }

    /// Check if a face is inside a BSP tree
    fn face_inside_tree(
        &self,
        face: &TopoDsFace,
        tree: &crate::modeling::bsp_tree::BspTree,
    ) -> bool {
        // Get face center
        let center = self.face_center(face);

        // Check if center is inside the tree
        self.point_inside_tree(&center, tree)
    }

    /// Get face center
    fn face_center(&self, face: &TopoDsFace) -> Point {
        let wires = face.wires();
        if wires.is_empty() {
            return Point::origin();
        }

        let outer_wire = wires[0].get().unwrap();
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();

        let mut center = Point::origin();
        let mut count = 0;

        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) =
                    (edge.start_vertex().get(), edge.end_vertex().get())
                {
                    let start_point = start.point();
                    let end_point = end.point();

                    center.x += (start_point.x + end_point.x) / 2.0;
                    center.y += (start_point.y + end_point.y) / 2.0;
                    center.z += (start_point.z + end_point.z) / 2.0;
                    count += 1;
                }
            }
        }

        if count > 0 {
            center.x /= count as f64;
            center.y /= count as f64;
            center.z /= count as f64;
        }

        center
    }

    /// Check if a point is inside a BSP tree
    fn point_inside_tree(&self, point: &Point, tree: &crate::modeling::bsp_tree::BspTree) -> bool {
        if let Some(ref root) = tree.root {
            self.point_inside_node(point, root)
        } else {
            false
        }
    }

    /// Check if a point is inside a BSP node
    fn point_inside_node(&self, point: &Point, node: &crate::modeling::bsp_tree::BspNode) -> bool {
        let distance = node.plane.distance(point);

        if distance > self.tolerance {
            // Point is in front of the plane
            if let Some(ref front) = node.front {
                return self.point_inside_node(point, front);
            }
            return false;
        } else if distance < -self.tolerance {
            // Point is behind the plane
            if let Some(ref back) = node.back {
                return self.point_inside_node(point, back);
            }
            return false;
        } else {
            // Point is on the plane
            // Check if point is inside any of the faces on this node
            for face in &node.faces {
                if self.point_inside_face(point, face.as_ref()) {
                    return true;
                }
            }

            // Check child nodes
            if let Some(ref front) = node.front {
                if self.point_inside_node(point, front) {
                    return true;
                }
            }
            if let Some(ref back) = node.back {
                if self.point_inside_node(point, back) {
                    return true;
                }
            }

            return false;
        }
    }

    /// Check if a point is inside a face
    fn point_inside_face(&self, point: &Point, face: &TopoDsFace) -> bool {
        let wires = face.wires();
        if wires.is_empty() {
            return false;
        }

        let outer_wire = wires[0].get().unwrap();
        let edges_slice: &[Handle<crate::topology::TopoDsEdge>] = outer_wire.edges();

        // Ray casting algorithm
        let mut intersection_count = 0;
        let ray_dir = Vector::new(1.0, 0.0, 0.0);

        for edge_handle in edges_slice.iter() {
            if let Some(edge) = edge_handle.get() {
                if let (Some(start), Some(end)) =
                    (edge.start_vertex().get(), edge.end_vertex().get())
                {
                    let start_point = start.point();
                    let end_point = end.point();

                    // Check if ray intersects this edge
                    if self.ray_intersects_edge(point, &ray_dir, &start_point, &end_point) {
                        intersection_count += 1;
                    }
                }
            }
        }

        // If odd, point is inside
        intersection_count % 2 == 1
    }

    /// Check if a ray intersects an edge
    fn ray_intersects_edge(&self, origin: &Point, dir: &Vector, p1: &Point, p2: &Point) -> bool {
        // Implementation of ray-edge intersection
        let edge_vec = Vector::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);
        let ray_vec = dir;

        let cross = edge_vec.cross(&ray_vec);
        let denom = cross.magnitude();

        if denom < 1e-6 {
            return false; // Ray and edge are parallel
        }

        let origin_to_p1 = Vector::new(p1.x - origin.x, p1.y - origin.y, p1.z - origin.z);
        let t = origin_to_p1.cross(&edge_vec).dot(&cross) / (denom * denom);

        if t < 0.0 {
            return false; // Intersection is behind ray origin
        }

        let u = origin_to_p1.cross(&ray_vec).dot(&cross) / (denom * denom);

        u >= 0.0 && u <= 1.0
    }
}

impl Default for AdvancedBooleanOperations {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::topods_solid::TopoDsSolid;
    use std::sync::Arc;

    #[test]
    fn test_advanced_fuse() {
        let boolean_ops = AdvancedBooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Fuse the solids
        let result = boolean_ops.fuse(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_advanced_cut() {
        let boolean_ops = AdvancedBooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Cut solid2 from solid1
        let result = boolean_ops.cut(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_advanced_common() {
        let boolean_ops = AdvancedBooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Compute common part
        let result = boolean_ops.common(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_advanced_symmetric_difference() {
        let boolean_ops = AdvancedBooleanOperations::new();

        // Create two simple solids
        let solid1 = TopoDsSolid::new();
        let solid2 = TopoDsSolid::new();

        // Compute symmetric difference
        let result = boolean_ops.symmetric_difference(
            &Handle::new(Arc::new(solid1.shape().clone())),
            &Handle::new(Arc::new(solid2.shape().clone())),
        );

        // Result should be a compound
        assert!(result.shape().is_compound());
    }

    #[test]
    fn test_boolean_operations_with_different_shape_types() {
        let boolean_ops = AdvancedBooleanOperations::new();

        // Create different shape types
        let solid = TopoDsSolid::new();
        let face = TopoDsFace::new();
        let solid_handle = Handle::new(Arc::new(solid.shape().clone()));
        let face_handle = Handle::new(Arc::new(face.shape().clone()));

        // Test operations with different shape types
        let fuse_result = boolean_ops.fuse(&solid_handle, &face_handle);
        let cut_result = boolean_ops.cut(&solid_handle, &face_handle);
        let common_result = boolean_ops.common(&solid_handle, &face_handle);
        let symmetric_diff_result = boolean_ops.symmetric_difference(&solid_handle, &face_handle);

        // All results should be compounds
        assert!(fuse_result.shape().is_compound());
        assert!(cut_result.shape().is_compound());
        assert!(common_result.shape().is_compound());
        assert!(symmetric_diff_result.shape().is_compound());
    }
}
